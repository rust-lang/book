## Graceful Shutdown and Cleanup

The code in Listing 20-21 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about fields that
we’re not using in a direct way, which are a reminder that we’re not cleaning
anything up. When we use <span class="keystroke">ctrl-C</span> to halt the main
thread, all the other threads are stopped immediately as well, even if they’re
in the middle of serving a request.

We’re now going to implement the `Drop` trait for `ThreadPool` to call `join`
on each of the threads in the pool so that the threads will finish the requests
they’re working on. Then we’ll implement a way for the `ThreadPool` to tell the
threads they should stop accepting new requests and shut down. To see this code
in action, we’ll modify our server to only accept two requests before
gracefully shutting down its thread pool.

Let’s start with implementing `Drop` for our thread pool. When the pool is
dropped, we should join on all of our threads to make sure they finish their
work. Listing 20-22 shows a first attempt at a `Drop` implementation; this code
won’t quite work yet:

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
then we call `join` on that worker’s thread. If the call to `join` fails, we
`unwrap` the error to panic and go into an ungraceful shutdown.

Here’s the error we get if we compile this code:

```text
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

Because we only have a mutable borrow of each `worker`, we can’t call `join`:
`join` takes ownership of its argument. In order to solve this, we need a way
to move the `thread` out of the `Worker` instance that owns `thread` so that
`join` can consume the thread. We saw a way to do this in Listing 17-15: if the
`Worker` holds an `Option<thread::JoinHandle<()>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a worker,
we’ll replace `Some` with `None` so the worker doesn’t have a thread to run.

So we know we want to update the definition of `Worker` like this:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

Now let’s lean on the compiler to find the other places that need to change. We
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
89 |             thread,
   |             ^^^^^^ expected enum `std::option::Option`, found
   struct `std::thread::JoinHandle`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`
```

The second error is pointing to the code at the end of `Worker::new`; we need
to wrap the `thread` value in `Some` when we create a new `Worker`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // ...snip...

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

The first error is in our `Drop` implementation, and we mentioned that we’ll be
calling `take` on the `Option` value to move `thread` out of `worker`. Here’s
what that looks like:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

As we saw in Chapter 17, the `take` method on `Option` takes the `Some` variant
out and leaves `None` in its place. We’re using `if let` to destructure the
`Some` and get the thread, then call `join` on the thread. If a worker’s thread
is already `None`, then we know this worker has already had its thread cleaned
up so we don’t do anything in that case.

With this, our code compiles without any warnings. Bad news though, this code
doesn’t function the way we want it to yet. The key is the logic in the
closures that the spawned threads of the `Worker` instances run: calling `join`
won’t shut down the threads since they `loop` forever looking for jobs. If we
try to drop our `ThreadPool` with this implementation, the main thread will
block forever waiting for the first thread to finish.

To fix this, we’re going to modify the threads to listen for either a `Job` to
run or a signal that they should stop listening and exit the infinite loop. So
instead of `Job` instances, our channel will send one of these two enum
variants:

<span class="filename">Filename: src/lib.rs</span>

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

We need to adjust the channel to use values of type `Message` rather than type
`Job`, as shown in Listing 20-23:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// ...snip...

impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // ...snip...
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// ...snip...

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<span class="caption">Listing 20-23: Sending and receiving `Message` values and
exiting the loop if a `Worker` receives `Message::Terminate`</span>

We need to change `Job` to `Message` in the definition of `ThreadPool`, in
`ThreadPool::new` where we create the channel, and in the signature of
`Worker::new`. The `execute` method of `ThreadPool` needs to send jobs wrapped
in the `Message::NewJob` variant. Then, in `Worker::new` where we receive a
`Message` from the channel, we’ll process the job if we get the `NewJob`
variant and break out of the loop if we get the `Terminate` variant.

With these changes, the code will compile again and continue to function in the
same way as it has been. We’ll get a warning, though, because we aren’t using
the `Terminate` variant in any messages. Let’s change our `Drop` implementation
to look like Listing 20-24:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

<span class="caption">Listing 20-24: Sending `Message::Terminate` to the
workers before calling `join` on each worker thread</span>

We’re now iterating over the workers twice, once to send one `Terminate`
message for each worker, and once to call `join` on each worker’s thread. If we
tried to send a message and join immediately in the same loop, it’s not
guaranteed that the worker in the current iteration will be the one that gets
the message from the channel.

To understand better why we need two separate loops, imagine a scenario with
two workers. If we iterated through each worker in one loop, on the first
iteration where `worker` is the first worker, we’d send a terminate message
down the channel and call `join` on the first worker’s thread. If the first
worker was busy processing a request at that moment, the second worker would
pick up the terminate message from the channel and shut down. We’re waiting on
the first worker to shut down, but it never will since the second thread picked
up the terminate message. We’re now blocking forever waiting for the first
worker to shut down, and we’ll never send the second message to terminate.
Deadlock!

To prevent this, we first put all of our `Terminate` messages on the channel,
and then we join on all the threads. Because each worker will stop receiving
requests on the channel once it gets a terminate message, we can be sure that
if we send the same number of terminate messages as there are workers, each
worker will receive a terminate message before we call `join` on its thread.

In order to see this code in action, let’s modify `main` to only accept two
requests before gracefully shutting the server down as shown in Listing 20-25:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
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
```

<span class="caption">Listing 20-25: Shut down the server after serving two
requests by exiting the loop</span>

Only serving two requests isn’t behavior you’d like a production web server to
have, but this will let us see the graceful shutdown and cleanup working since
we won’t be stopping the server with <span class="keystroke">ctrl-C</span>.

We’ve added a `counter` variable that we’ll increment every time we receive an
incoming TCP stream. If that counter reaches 2, we’ll stop serving requests and
instead break out of the `for` loop. The `ThreadPool` will go out of scope at
the end of `main`, and we’ll see the `drop` implementation run.

Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output that looks like:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

You may get a different ordering, of course. We can see how this works from the
messages: workers zero and three got the first two requests, and then on the
third request, we stop accepting connections. When the `ThreadPool` goes out of
scope at the end of `main`, its `Drop` implementation kicks in, and the pool
tells all workers to terminate. The workers each print a message when they see
the terminate message, and then the thread pool calls `join` to shut down each
worker thread.

One interesting aspect of this particular execution: notice that we sent the
terminate messages down the channel, and before any worker received the
messages, we tried to join worker zero. Worker zero had not yet gotten the
terminate message, so the main thread blocked waiting for worker zero to
finish. In the meantime, each of the workers received the termination messages.
Once worker zero finished, the main thread waited for the rest of the workers
to finish, and they had all received the termination message and were able to
shut down at that point.

Congrats! We now have completed our project, and we have a basic web server
that uses a thread pool to respond asynchronously. We’re able to perform a
graceful shutdown of the server, which cleans up all the threads in the pool.
Here’s the full code for reference:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    let mut counter = 0;

    for stream in listener.incoming() {
        if counter == 2 {
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

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

     let mut file = File::open(filename).unwrap();
     let mut contents = String::new();

     file.read_to_string(&mut contents).unwrap();

     let response = format!("{}{}", status_line, contents);

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap();
}
```

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
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

type Job = Box<FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

There’s more we could do here! If you’d like to continue enhancing this
project, here are some ideas:

- Add more documentation to `ThreadPool` and its public methods
- Add tests of the library’s functionality
- Change calls to `unwrap` to more robust error handling
- Use `ThreadPool` to perform some other task rather than serving web requests
- Find a thread pool crate on crates.io and implement a similar web server
  using the crate instead and compare its API and robustness to the thread pool
  we implemented

## Summary

Well done! You’ve made it to the end of the book! We’d like to thank you for
joining us on this tour of Rust. You’re now ready to go out and implement your
own Rust projects or help with other people’s. Remember there’s a community of
other Rustaceans who would love to help you with any challenges you encounter
on your Rust journey.
