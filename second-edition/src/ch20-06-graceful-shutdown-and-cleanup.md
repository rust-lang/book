## Graceful Shutdown and Cleanup

The first thing we want to do is to implement `Drop` for our thread pool. When
the pool is dropped, we should join on all of our threads, to make sure they
finish their work. Here's a first attempt at it; it won't quite work yet:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

Here's the idea: we loop through each of our `threads`, using `&mut` because
`self` is itself a mutable reference. If we tried to iterate over the threads
directly, we'd get an error about moving. Anyway, we print out a message saying
that that particular worker is shutting down, and then we call `join` on that
worker's thread. An `unwrap` disregards the errors.

Here's the error we get:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0507]: cannot move out of borrowed content
   --> src\main.rs:129:13
    |
129 |             worker.thread.join();
    |             ^^^^^^ cannot move out of borrowed content

error: aborting due to previous error
```

Because we only have a `&mut` in `drop`, we cannot actually call `join`, as
`join` takes its argument by value. What to do? Well, we already have a way to
represent "something or nothing", and that's `Option<T>`. Let's update the
definition of `Worker`:

```rust,ignore
struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}
```

And then let the compiler tell us about anything we need to fix:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0308]: mismatched types
  --> src\main.rs:87:21
   |
87 |             thread: thread,
   |                     ^^^^^^ expected enum `std::option::Option`, found
   struct `std::thread::JoinHandle`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`

error: no method named `join` found for type
`std::option::Option<std::thread::JoinHandle<()>>` in the current scope
   --> src\main.rs:129:27
    |
129 |             worker.thread.join();
    |                           ^^^^

```

The first error is easy to fix; we need to add a `Some` at the end of
`ThreadPool::new`:

```rust,ignore
Worker {
        id: id,
        thread: Some(thread),
    }
```

The second one is in our `Drop` implementation. Here's one that works:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

The `take` method on `Option`, well, takes something out of it. That is, if the
`Option` is `Some(T)`, it will set the original option to `None`, and then
return that `Some(T)`. If the option is `None`, it will return `None`.

We use `if let` to check if the return value of `take` is `Some`, and if it is,
we call `join` on that thread.

With this, our code compiles without any warnings, and still works!

... or does it? There's one last issue we haven't handled yet: this `Drop`
implementation doesn't actually work. The key is the logic of our `Worker`s.
There's no way to shut them down; they only loop forever looking for jobs. If
we try to drop our `ThreadPool` with this implementation, it will block forever
on the first thread.

So what do we do? We need to modify our channel to take a `Message` instead of
a `Job`. Like this:

```rust,ignore
enum Message {
    NewJob(Job),
    Terminate,
}
```

First, we have a new `Message` enum. We have two kinds of messages: "here's a
new `Job`" and "please terminate execution."

```rust,ignore
struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
```

We need to adjust the `ThreadPool` to send `Message`s rather than `Job`s.

```rust,ignore
impl Worker {
    fn new(id: u32, job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = job_receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
```

Inside of our `Worker`, instead of receiving a `Job`, we get a `Message`. We
then execute the job if it's a `NewJob`, and break out of our `loop` if it's
`Terminate`.

```rust,ignore
impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Message>();

        // no other changes here
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Job {
            job: Box::new(f),
        };

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
```

`ThreadPool` has two changes: first, we need our channel to be of `Message`s
instead of `Job`s. Then, in `execute`, we need to send a `NewJob` rather than
just a `Job`.

With these changes, things compile again. But we haven't sent any `Terminate`
messages. Let's change our `Drop` implementation:

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

We need two loops here. Why? Well, if we send a message and then try to join,
it's not guaranteed that that worker will be the one that gets that message.
We'd then deadlock. Imagine this scenario: we have two worker threads. We send
a terminate message down the channel, and then join thread one. But thread one
is busy processing a request; thread two is idle. This means thread two would
get the terminate message and shut down; but we're waiting for thread one to
shut down. Since `join` blocks until shut down, we're now blocking forever, and
will never send the second message to terminate. Deadlock!

To prevent this, we first put all of our `Terminate` messages on the channel,
and then we join on all the threads.

Let's give it a try: modify `main` to only accept a small number of requests
before shutting the server down:

```rust,ignore
    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
            println!("Shutting down.");
            break;
        }

        counter += 1;
```

And then run it with `cargo run`. Load up the pages a few times, and then check
your terminal. You'll see something like this:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target\debug\hello.exe`
Worker 0 got a job; executing.
Worker 1 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down worker 0
Worker 2 was told to terminate.
Worker 3 was told to terminate.
Worker 0 was told to terminate.
Worker 1 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

You may get a different ordering of course. We can see how this works from the
messages though; workers zero and one get the two page loads, and then, we stop
accepting connections. When the `Pool` goes out of scope at the end of `main`,
its `Drop` implementation kicks in, and tells all workers to terminate. They
then each print the message that they have seen the terminate message, and then
they all get shut down. One interesting thing about this particular execution:
you'll notice that we told every worker to terminate, and then immediately
tried to join worker zero. Since it had not yet gotten the terminate message,
it waited, and the threads each acknowledged their termination.

Let's bump that request count up to five:

```rust,ignore
    if counter == 5 {
```

And try hitting `/sleep` and `/` at the same time, as we did before. You
should see the request for `/` complete before the request for `/sleep`;
we're doing our processing in the background, and not processing requests
sequentially any more!

Congrats! We now have completed our project. Here's the full code, for
reference:

```rust,no_run
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(4);

    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 5 {
            println!("Shutting down.");
            break;
        }

        counter += 1;

        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let get_start = &buffer[..get.len()];
    let sleep_start = &buffer[..sleep.len()];

    let (header, filename) = if get_start == get {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if sleep_start == sleep {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

struct Job {
    job: Box<FnBox + Send + 'static>,
}

struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    fn new(id: u32, job_receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = job_receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Message>();

        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, job_receiver.clone()));
        }

        ThreadPool {
            threads: threads,
            sender: job_sender,
        }
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Job {
            job: Box::new(f),
        };

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

There is still more we could do here; for example, our `ThreadPool` is not
inherently tied to HTTP handling, so we could extract it into its own
submodule, or maybe even its own crate! Extracting the code would make the
`ThreadPool` code more easily reusable in another context.

## Summary

TODO
