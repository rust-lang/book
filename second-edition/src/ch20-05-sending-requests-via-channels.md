#### Sending Requests to Threads Via Channels

The next problem to tackle is that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method, but we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.

We want the `Worker` structs that we just created to fetch code to run from a
queue held in the `ThreadPool`, and send that code to its thread to run.

In Chapter 16, we learned about *channels*---a simple way to communicate
between two threads---that would be perfect for this use-case. We'll use a
channel to function as the queue of jobs, and `execute` will send a job from
the `ThreadPool` to the `Worker` instances, which will send the job to its
thread. Here’s the plan:

1. `ThreadPool` will create a channel and hold on to the sending side of the
   channel.
2. Each `Worker` will hold on to the receiving side of the channel.
3. We'll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute down the sending
   side of the channel.
5. In its thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.

Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. `Job` is a struct
that doesn't hold anything for now, but will be the type of item we’re sending
down the channel:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
// --snip--
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // --snip--
}
#
# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
# impl Worker {
#     fn new(id: usize) -> Worker {
#         let thread = thread::spawn(|| {});
#
#         Worker {
#             id,
#             thread,
#         }
#     }
# }
```

<span class="caption">Listing 20-16: Modifying `ThreadPool` to store the
sending end of a channel that sends `Job` instances</span>

In `ThreadPool::new`, we create our new channel, and have the pool hold the
sending end. This will successfully compile, still with warnings.

Let’s try passing a receiving end of the channel into each worker as the thread
pool creates them. We know we want to use the receiving end in the thread that
the workers spawn, so we’re going to reference the `receiver` parameter in the
closure. The code shown here in Listing 20-17 won’t quite compile yet:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // --snip--
}

// --snip--

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-17: Passing the receiving end of the channel
to the workers</span>

These are small and straightforward changes: we pass the receiving end of the
channel into `Worker::new`, and then we use it inside of the closure.

If we try to check this, we get this error:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0382]: use of moved value: `receiver`
  --> src/lib.rs:27:42
   |
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here in
   previous iteration of loop
   |
   = note: move occurs because `receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
```

The code is trying to pass `receiver` to multiple `Worker` instances. This
won't work, as we recall from Chapter 16: the channel implementation provided
by Rust is multiple *producer*, single *consumer*. This means we can’t just
clone the consuming end of the channel to fix this. Even if we could, that's
not the technique we'd want to use; we want to distribute the jobs across
threads by sharing the single `receiver` between all of the workers.

<!-- Above - you may be able to tell I struggled to follow this explanation,
can you double check my edits and correct here? -->
<!-- Yep, the text we had here was nonsensical. The edits are fine! /Carol -->

Additionally, taking a job off the channel queue involves mutating the
`receiver`, so the threads need a safe way to share and modify `receiver`,
otherwise we might get race conditions (as covered in Chapter 16).

Remembering the thread-safe smart pointers that we discussed in Chapter 16, in
order to share ownership across multiple threads and allow the threads to
mutate the value, we need to use `Arc<Mutex<T>>`. `Arc` will let multiple
workers own the receiver, and `Mutex` will make sure that only one worker is
getting a job from the receiver at a time. Listing 20-18 shows the changes we
need to make:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// --snip--

# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# struct Job;
#
impl ThreadPool {
    // --snip--
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

    // --snip--
}

# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
#         let thread = thread::spawn(|| {
#            receiver;
#         });
#
#         Worker {
#             id,
#             thread,
#         }
    }
}
```

<span class="caption">Listing 20-18: Sharing the receiving end of the channel
between the workers using `Arc` and `Mutex`</span>

In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.

With these changes, the code compiles! We’re getting there!

#### Implementing the `execute` Method

Let’s finally implement the `execute` method on `ThreadPool`. We’re also going
to change `Job` from a struct to a type alias for a trait object that holds the
type of closure that `execute` receives. As we discussed in the "Type Aliases
Create Type Synonyms" section of Chapter 19, type aliases allow us to make long
types shorter. Take a look at Listing 20-19:

<span class="filename">Filename: src/lib.rs</span>

```rust
// --snip--
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# use std::sync::mpsc;
# struct Worker {}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// --snip--
```

<span class="caption">Listing 20-19: Creating a `Job` type alias for a `Box`
that holds each closure, then sending the job down the channel</span>

After creating a new `Job` instance using the closure we get in `execute`, we
send that job down the sending end of the channel. We’re calling `unwrap` on
`send` for the case that sending fails, which might happen if, for example, we
stop all of our threads from executing, meaning the receiving end has stopped
receiving new messages. At the moment, though, we can't stop our threads
executing; our threads continue executing as long as the pool exists. The
reason we use `unwrap`, then, is that we we know the failure case won’t happen
but the compiler can’t tell that.

But we're not quite done yet! In the worker, our closure being passed to
`thread::spawn` still only *references* the receiving end of the channel.
Instead, we need the closure to loop forever, asking the receiving end of the
channel for a job, and running the job when it gets one. Let’s make the change
shown in Listing 20-20 to `Worker::new`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-20: Receiving and executing the jobs in the
worker’s thread</span>

Here, we first call `lock` on the `receiver` to acquire the mutex, then
`unwrap` to panic on any errors. Acquiring a lock might fail if the mutex is in
a *poisoned* state, which can happen if some other thread panicked while
holding the lock, rather than releasing the lock. In this situtation, calling
`unwrap` to have this thread panic is the correct action to take. Feel free to
change this `unwrap` to an `expect` with an error message that is meaningful to
you if you’d like.

If we get the lock on the mutex, then we call `recv` to receive a `Job` from
the channel. A final `unwrap` moves past any errors here as well, which might
occur if the thread holding the sending side of the channel has shut down,
similar to how the `send` method returns `Err` if the receiving side shuts down.

The call to `recv` *blocks*, so if there’s no job yet, the current thread will
sit until a job becomes available. The `Mutex<T>` makes sure that only one
`Worker` thread at a time is trying to request a job.

Theoretically, this code should compile. Unfortunately, the Rust compiler isn’t
perfect yet, and we get this error:

```text
error[E0161]: cannot move a value of type std::ops::FnOnce() +
std::marker::Send: the size of std::ops::FnOnce() + std::marker::Send cannot be
statically determined
  --> src/lib.rs:63:17
   |
63 |                 (*job)();
   |                 ^^^^^^
```

This error is fairly cryptic, and that’s because the problem is fairly cryptic.
In order to call a `FnOnce` closure that is stored in a `Box<T>` (which is what
our `Job` type alias is), the closure needs to be able to move itself *out* of
the `Box<T>` because the closure takes ownership of `self` when we call it. In
general, Rust doesn't allow us to move value out of a `Box<T>` because Rust
doesn’t know how big the value inside the `Box<T>` is going to be; recall in
Chapter 15 that we used `Box<T>` precisely because we had something of an
unknown size that we wanted to store in a `Box<T>` to get a value of a known
size.

We saw in Chapter 17, Listing 17-15 that we can write methods that use the
syntax `self: Box<Self>`, which allows the method to take ownership of a `Self`
value stored in a `Box<T>`. That’s exactly what we want to do here, but
unfortunately Rust won't let us: the part of Rust that implements behavior when
a closure is called isn’t implemented using `self: Box<Self>`. So Rust doesn’t
yet understand that it could use `self: Box<Self>` in this situation in order
to take ownership of the closure and move the closure out of the `Box<T>`.

Rust is still a work in progress with places that the compiler could be
improved, but in the future, the code in Listing 20-20 should work just fine.
There are people just like you working to fix this and other issues! Once
you’ve finished the book, we would love for you to join in.

But for now, let’s work around this problem with a handy trick. We can tell
Rust explicitly that in this case we can take ownership of the value inside the
`Box<T>` using `self: Box<Self>`, and once we have ownership of the closure, we
can call it. This involves defining a new trait `FnBox` with the method
`call_box` that will use `self: Box<Self>` in its signature, defining `FnBox`
for any type that implements `FnOnce()`, changing our type alias to use the new
trait, and changing `Worker` to use the `call_box` method. These changes are
shown in Listing 20-21:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-21: Adding a new trait `FnBox` to work around
the current limitations of `Box<FnOnce()>`</span>

First, we create a new trait named `FnBox`. This trait has the one method
`call_box`, which is similar to the `call` methods on the other `Fn*` traits
except that it takes `self: Box<Self>` in order to take ownership of `self` and
move the value out of the `Box<T>`.

Next, we implement the `FnBox` trait for any type `F` that implements the
`FnOnce()` trait. Effectively, this means that any `FnOnce()` closures can use
our `call_box` method. The implementation of `call_box` uses `(*self)()` to
move the closure out of the `Box<T>` and call the closure.

We now need our `Job` type alias to be a `Box` of anything that implements our
new trait `FnBox`. This will allow us to use `call_box` in `Worker` when we get
a `Job` value. Implementing the `FnBox` trait for any `FnOnce()` closure means
we don’t have to change anything about the actual values we’re sending down the
channel.

Finally, in the closure run in the thread in `Worker::new`, we use `call_box`
instead of invoking the closure directly. Now Rust is able to understand that
what we want to do is fine.

This is a very sneaky, complicated trick. Don’t worry too much if it doesn’t
make perfect sense; someday, it will be completely unnecessary.

With this trick, our thread pool is in a working state! Give it a `cargo run`,
and make some requests:

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never used: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `id`
  --> src/lib.rs:61:5
   |
61 |     id: usize,
   |     ^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: field is never used: `thread`
  --> src/lib.rs:62:5
   |
62 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running `target/debug/hello`
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

Success! We now have a thread pool executing connections asynchronously. There
are never more than four threads created, so our system won’t get overloaded if
the server receives a lot of requests. If we make a request to `/sleep`, the
server will be able to serve other requests by having another thread run them.

After learning about the `while let` loop in Chapter 18, you might be
wondering why we didn't write the worker thread like this:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-22: An alternative implementation of
`Worker::new` using `while let`</span>

This code compiles and runs, but doesn't result in the desired threading
behavior: a slow request will still cause other requests to wait to be
processed. The reason why is somewhat subtle: the `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. This allows the borrow checker to enforce at compile time that
we never access a resource guarded by a `Mutex` without holding the lock, but
it can also result in holding the lock longer than intended if we don't think
carefully about the lifetime of the `MutexGuard<T>`. Because the values in the
the `while` expression remain in scope for the duration of the block, the lock
remains held for the duration of the call to `job.call_box()`, meaning other
workers cannot receive jobs.

By using `loop` instead and acquiring the lock and a job within the block
rather than outside it, the `MutexGuard` returned from the `lock` method is
dropped as soon as the `let job` statement ends. This ensures that the lock is
held during the call to `recv`, but it is released before the call to
`job.call_box()`, allowing multiple requests to be serviced concurrently.
