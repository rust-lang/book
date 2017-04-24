## Graceful Shutdown and Cleanup

The code in Listing 20-21 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about fields that
we're not using in a direct way, which are a reminder that we're not cleaning
anything up. When we use `CTRL-C` to halt the main thread, all the other
threads are stopped immediately as well, even if they're in the middle of
serving a request.

We're now going to implement the `Drop` trait for `ThreadPool` to call `join`
on each of the threads in the pool so that the threads will finish the requests
they're working on. Then we'll implement a way for the `ThreadPool` to tell the
threads they should stop accepting new requests and shut down. To see this code
in action, we'll modify our server to only accept two requests before
gracefully shutting down its thread pool.

Let's start with implementing `Drop` for our thread pool. When the pool is
dropped, we should join on all of our threads to make sure they finish their
work. Listing 20-22 shows a first attempt at a `Drop` implementation; this code
won't quite work yet:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

<span class="caption">Listing 20-22: Joining each thread when the thread pool
goes out of scope</span>

We loop through each of the thread pool `workers`, using `&mut` because `self`
is itself a mutable reference and we also need to be able to mutate `worker`.
We print out a message saying that this particular worker is shutting down, and
then we call `join` on that worker's thread. If the call to `join` fails, we
`unwrap` the error to panic and go into an ungraceful shutdown.

Here's the error we get if we compile this code:

```text
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

Because we only have a mutable borrow of each `worker`, we can't call `join`:
`join` takes ownership of its argument. In order to solve this, we need a way
to move the `thread` out of the `Worker` instance that owns `thread` so that
`join` can consume the thread. We saw a way to do this in Listing 17-15: if the
`Worker` holds an `Option<thread::JoinHandle<()>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a worker,
we'll replace `Some` with `None` so the worker doesn't have a thread to run.

So we know we want to update the definition of `Worker` like this:

```rust
# use std::thread;
struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}
```

Now let's lean on the compiler to find the other places that need to change. We
get two errors:

```text
error: no method named `join` found for type
`std::option::Option<std::thread::JoinHandle<()>>` in the current scope
  --> src/lib.rs:65:27
   |
65 |             worker.thread.join().unwrap();
   |                           ^^^^

error[E0308]: mismatched types
  --> src/lib.rs:89:21
   |
89 |             thread: thread,
   |                     ^^^^^^ expected enum `std::option::Option`, found
   struct `std::thread::JoinHandle`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`
```

The second error is pointing to the code at the end of `Worker::new`; we need
to wrap the `thread` value in `Some` when we create a new `Worker`:

```rust,ignore
impl Worker {
    fn new(id: usize, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // ...snip...

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
```

The first error is in our `Drop` implementation, and we mentioned that we'll be
calling `take` on the `Option` value to move `thread` out of `worker`. Here's
what that looks like:

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

As we saw in Chapter 17, the `take` method on `Option` takes the `Some` variant
out and leaves `None` in its place. We're using `if let` to destructure the
`Some` and get the thread, then call `join` on the thread. If a worker's thread
is already `None`, then we know this worker has already had its thread cleaned
up so we don't do anything in that case.

With this, our code compiles without any warnings. Bad news though, this code
doesn't function the way we want it to yet. The key is the logic in the
closures that the spawned threads of the `Worker` instances run: calling `join`
won't shut down the threads since they `loop` forever looking for jobs. If we
try to drop our `ThreadPool` with this implementation, the main thread will
block forever waiting for the first thread to finish.

To fix this, we're going to modify the threads to listen for either a `Job` to
run or a signal that they should stop listening and exit the infinite loop. So
instead of `Job` instances, our channel will send one of these two enum
variants:

```rust
# struct Job;
enum Message {
    NewJob(Job),
    Terminate,
}
```

This `Message` enum will either be a `NewJob` variant that holds the `Job` the
thread should run, or it will be a `Terminate` variant that will cause the
thread to exit its loop and stop.

TODO CAROL STOPPED EDITING HERE

We need to adjust the

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

Also more docs, better error handling

## Summary

TODO
