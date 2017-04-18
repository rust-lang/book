## Sending Requests to Threads Via Channels

The next problem to tackle is that our closures do absolutely nothing. We've
been working around the problem that we get the actual closure we want to
execute in the `execute` method, but it feels like we need to know the actual
closures when we create the `ThreadPool`.

Let's think about what we really want to do though: we want the `Worker`
structs that we just created to fetch jobs from a queue that the `ThreadPool`
holds, and run those jobs in a thread.

In Chapter 16, we learned about channels. Channels are a great way to
communicate between two threads, and they're perfect for this use-case. The
channel will function as the queue of jobs, and `execute` will send a job from
the `ThreadPool` to the `Worker` instances that are checking for jobs in the
thread they've spawned. Here's the plan:

1. `ThreadPool` will create a channel and hold on to the sending side.
2. Each `Worker` will hold on to the receiving side of the channel.
3. A new `Job` struct will hold the closures we want to send down the channel.
4. The `execute` method of `ThreadPool` will send the job it wants
   to execute down the sending side of the channel.
5. In a thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.

Let's start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. `Job` is the type
of item we're going to be sending down the channel; it's a struct that doesn't
hold anything for now:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
// ...snip...
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers: workers,
            sender: job_sender,
        }
    }
    // ...snip...
}
#
# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
# impl Worker {
#     fn new(id: usize) -> Worker {
#         let thread = thread::spawn(|| { });
#
#         Worker {
#             id: id,
#             thread: thread,
#         }
#     }
# }
```

<span class="caption">Listing 20-16: Modifying `ThreadPool` to store the
sending end of a channel that sends `Job` instances</span>

In `ThreadPool::new`, we create our new channel, and then have the pool hang on
to the sending end. This will successfully compile, still with warnings.

Let's try passing a receiving end of the channel into each worker when the
thread pool creates them. We know we want to use the receiving end of the
channel in the thread that the workers spawn, so we're going to reference the
`job_receiver` parameter in the closure. The code shown here in Listing 20-17
won't quite compile yet:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, job_receiver));
        }

        ThreadPool {
            workers: workers,
            sender: job_sender,
        }
    }
    // ...snip...
}

// ...snip...

impl Worker {
    fn new(id: usize, job_receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            job_receiver;
        });

        Worker {
            id: id,
            thread: thread,
        }
    }
}
```

<span class="caption">Listing 20-17: Passing the receiving end of the channel
to the workers</span>

These are small and straightforward changes: we pass in the receiving end of
the channel into `Worker::new`, and then we use it inside of the closure.

If we try to check this, we get this error:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0382]: use of moved value: `job_receiver`
  --> src/lib.rs:27:42
   |
27 |             workers.push(Worker::new(id, job_receiver));
   |                                          ^^^^^^^^^^^^ value moved here in
   previous iteration of loop
   |
   = note: move occurs because `job_receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
```

The code as written won't quite work since it's trying to pass `job_receiver`
to multiple `Worker` instances. We instead need to share the single receiver
between all of our workers. If you remember Chapter 16, you'll know the answer:
`Arc<Mutex<T>>` to the rescue! The `Arc` will let multiple workers own the
receiver, and the `Mutex` will make sure that only one worker is getting a job
from the receiver at a time. Listing 20-18 shows the changes we need to make:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
// ...snip...
use std::sync::Arc;
use std::sync::Mutex;

// ...snip...

# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# struct Job;
#
impl ThreadPool {
    // ...snip...
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, job_receiver.clone()));
        }

        ThreadPool {
            workers: workers,
            sender: job_sender,
        }
    }

    // ...snip...
}
# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
impl Worker {
    fn new(id: usize, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // ...snip...
#         let thread = thread::spawn(|| {
#            job_receiver;
#         });
#
#         Worker {
#             id: id,
#             thread: thread,
#         }
    }
}
```

<span class="caption">Listing 20-18: Sharing the receiving end of the channel
between the workers by using `Arc` and `Mutex`</span>

In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.

With these changes, things compile! We're getting there!

Let's finally implement the `execute` method on `ThreadPool`. We're also going
to change the `Job` struct: instead of being a struct, `Job` is going to be a
type alias for a trait object that holds the type of closure that `execute`
receives. We discussed how type aliases can help make long types shorter, and
this is such a case! Take a look at Listing 20-19:

<span class="filename">Filename: src/lib.rs</span>

```rust
// ...snip...
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# use std::sync::mpsc;
# struct Worker {}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // ...snip...

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// ...snip...
```

<span class="caption">Listing 20-19: Creating a `Job` type alias for a `Box`
that holds each closure, then sending the job down the channel</span>

After creating a new `Job` instance using the closure we get in
`execute`, we send that job down the sending end of the channel. We're calling
`unwrap` on `send` since sending may fail if the receiving end has stopped
receiving new messages, which would happen if we stop all of our threads from
executing. This isn't possible right now, though, since our threads continue
executing as long as the pool exists. We use `unwrap` since we know the failure
case won't happen even though the compiler can't tell that, which is an
appropriate use of `unwrap` as we discussed in Chapter 9.

Are we done yet? Not quite! We've still got a closure that only references the
receiving end of the channel in the worker, and instead we need the closure to
loop forever, asking the receiving end of the channel for a job, and running
the job when it gets one. Let's make the change shown in Listing 20-20 to
`Worker::new`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
// ...snip...

impl Worker {
    fn new(id: usize, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = job_receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id: id,
            thread: thread,
        }
    }
}
```

<span class="caption">Listing 20-20: Receiving and executing the jobs in the
worker's thread</span>

TODO: CAROL EDITED UP TO HERE

Here, we first call `lock` on the `job_receiver` to acquire the mutex, then
`unwrap` to panic on any errors, then `recv` to receive a `Job` from the
channel. A final `unwrap` moves past those errors as well. What kinds of errors
are we ignoring here? Well, a mutex can be "poisoned", that is, if a thread is
holding the mutex and panics, it enters a "poisoned" state. Almost all of the
time, propagating this panic with `unwrap` is correct. As for `recv`, it will
return `Err` if the sending side has shut down, similar to how the `send`
method returns `Err` if the receiving side shuts down.

The call to `recv` blocks; that is, if there's no job yet, it will sit here
until one becomes available. The `Mutex<T>` makes sure that only one Worker at
a time tries to request a job.

Here's the error we'll get if we try to compile the above code:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error: no method named `job` found for type `Job` in the current scope
  --> src\main.rs:69:21
   |
69 |                 job.job();
   |                     ^^^
   |
note: use `(job.job)(...)` if you meant to call the function stored in the
`job` field
  --> src\main.rs:69:21
   |
69 |                 job.job();
   |                     ^^^

error: aborting due to previous error
```

Rust helpfully informs us that this is ambiguous: We're trying to invoke the
closure that `job.job` holds, not call a method `job`. In order to fix this, we
have to change that line:

```rust,ignore
(job.job)();
```

It looks a little funky, but it works. Well, almost. Now we get a different
error:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0161]: cannot move a value of type std::ops::FnOnce() +
std::marker::Send + 'static: the size of std::ops::FnOnce() + std::marker::Send
+ 'static cannot be statically determined
  --> src\main.rs:69:17
   |
69 |                 (job.job)();
   |                 ^^^^^^^^^

error: aborting due to previous error
```

This error is fairly cryptic, and that's because the problem is fairly cryptic.
Basically, in order to call a boxed `FnOnce`, the `FnOnce` needs to be able to
move itself out of the box. But the compiler doesn't understand that this is
okay to do.

In the future, this code should work just fine. Rust is still a work in
progress with places that the compiler could be improved. There are people just
like you working to fix this and other issues! Once you've finished the book,
we would love for you to join in.

But for now, let's work around this problem. Luckily, there's a trick! It looks
like this:

```rust,ignore
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

// we use this instead of (job.job)();
job.job.call_box();
```

Here's how the trick works: Rust *does* understand that when `self` is a
`Box<T>`, it can be moved out of. As such, we do four things:

First, we create a new trait, `FnBox`. This trait has one method, `call_box`,
similar to the `call` methods on the other `Fn*` traits. This method takes
`Box<Self>`.

Next, we implement `FnBox` for all things that implement `FnOnce()`:

```rust,ignore
impl<F: FnOnce()> FnBox for F {
```

That's what this line says: for any type `F` that implements `FnOnce()`, we are
going to implement `FnBox` for that type. Effectively, this means that any
`FnOnce()` closures can use our `call_box` method. Tricky!

Here's the implementation of `call_box`:

```rust,ignore
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
```

We do the same thing with `()()`s as we did above, only now instead of
`job.job`, it's `self`. And the dereference of self is what moves the contents
out of the box.

Finally, we use `call_box` instead of invoking the closure directly.

This is a very sneaky, complicated trick. Don't worry too much if it doesn't
make perfect sense; someday, it will be completely unnecessary.

With this trick, our thread pool is in a working state! Give it a `cargo run`,
and make some requests:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never used: `threads`, #[warn(dead_code)] on by default
  --> src\main.rs:50:5
   |
50 |     threads: Vec<Worker>,
   |     ^^^^^^^^^^^^^^^^^^^^

warning: field is never used: `id`, #[warn(dead_code)] on by default
  --> src\main.rs:69:5
   |
69 |     id: u32,
   |     ^^^^^^^

warning: field is never used: `thread`, #[warn(dead_code)] on by default
  --> src\main.rs:70:5
   |
70 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running `target\debug\hello.exe`
     Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

Success! We now have a thread pool executing connections asynchronously.

What about those warnings, though? Don't we use all those things? Well, here's
the thing: right now, we are using all three of these things to hold onto some
data, but we don't actually *do* anything with them. That is, we set up a ton
of interesting stuff, but then it just sits there.

So are these warnings wrong? In one sense yes, but in another sense, no. We
never do anything to clean up our thread pool once it's done being used. Let's
implement that now.
