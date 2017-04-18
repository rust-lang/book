## Sending Requests to Threads Via Channels

The next problem to tackle is that our closures do absolutely nothing. This
raises the question: what should they do? We get the actual closure we want to
execute in the `execute` method, but we need to know it here.

Or do we? This closure is the behavior of the *worker*, not of the work it
does. And as we said above, our workers are going to attempt to fetch jobs off
of a queue that the `ThreadPool` holds. We have none of that infrastructure yet.

In Chapter 16, we learned about channels. Channels are a great way to
communicate between two threads, and they're perfect with our use-case. Here's
the plan of attack:

1. `ThreadPool` will hold on to a sending side of a channel.
2. Each `Worker` will hold on to a receiving side.
3. The `execute` method of `ThreadPool` will then send the closure it wants
   to execute down the sending side of the channel.
4. The `Worker` will loop over its receiving side, and when it gets a job,
   execute it.

Once we get all of this working, we should be in a good place!

Let's start by adding the sending side to `ThreadPool`:

```rust,ignore
// add this import at the top:
use std::sync::mpsc;

// and then modify this code below:
struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32));
        }

        ThreadPool {
            threads: threads,
            sender: job_sender,
        }
    }
```

We've introduced a new structure, `Job`, to represent each job we want to
execute. We have our `ThreadPool` hold onto an `mpsc::Sender`, which if you
recall is the type of a sending end of a channel. In `ThreadPool::new`, we
create our new channel, and then have the pool hang on to the sending end.

If you compile this, it will successfully compile, but still have warnings.
This code doesn't do the right thing yet, but it gets past the compiler. Let's
try passing the receiving end into our workers. This won't compile yet:

```rust,ignore
impl Worker {
    fn new(id: u32, job_receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(||{
            // we want to use receiver in the closure, let's just
            // reference it for now
            job_receiver;
        });

        Worker {
            id: id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, job_receiver));
        }
```

These are small and straightforward changes: we pass in the receiving end of
the channel into `Worker::new`, and then we use it inside of the closure.

If we try to compile this, we get this error:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0382]: use of moved value: `job_receiver`
  --> src\main.rs:82:48
   |
82 |             threads.push(Worker::new(i as u32, job_receiver));
   |                                                ^^^^^^^^^^^^ value moved
   here in previous iteration of loop
   |
   = note: move occurs because `job_receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait

error: aborting due to previous error
```

This won't quite work: we are trying to pass `job_receiver` to multiple
`Worker`s, but that won't work. We instead need to share the single receiver
between all of our workers. If you remember Chapter 16, you'll know the answer:
`Arc<Mutex<T>>` to the rescue! Here's the changes:

```rust,ignore
// add these imports to the top
use std::sync::Arc;
use std::sync::Mutex;

// and then change this code
impl Worker {
    fn new(id: u32, job_receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(||{
            // we want to use the receiver in the closure
            job_receiver;
        });

        Worker {
            id: id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (job_sender, job_receiver) = mpsc::channel::<Job>();

        let job_receiver = Arc::new(Mutex::new(job_receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32, job_receiver.clone()));
        }
```

We now accept an `Arc<Mutex<Receiver>>` in `Worker::new`, and we create one in
`ThreadPool::new`. Finally, when we call `Worker::new`, we use the `clone`
method of the `Arc<T>` to bump the reference count for each new `Worker`.

With these changes, things compile! We're getting there!

Let's finally implement the `execute` method. It looks like this:

```rust,ignore
struct Job {
    job: Box<FnOnce() + Send + 'static>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        // no changes here
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Job {
            job: Box::new(f),
        };

        self.sender.send(job).unwrap();
    }
}
```

Here, `Job` is now holding a trait object; specifically, a boxed closure. We
then send that `job` down the sending end of the channel. Sending may fail if
the receiving end has stopped receiving new messages, which would happen happen
if we stop all of our threads from executing. Our threads continue executing as
long as the pool exists, so we use `unwrap` to panic if we get an error here
for now. As we discussed in Chapter 9, using `unwrap` is perfectly fine while
prototyping to get the successful case to work, and more appropriate error
handling can be added in later.

Now that we've got the sending side working, let's write the logic of the
worker. Here's a first attempt, but it won't quite work:

```rust,ignore
let thread = thread::spawn(move ||{
    loop {
        let job = job_receiver.lock().unwrap().recv().unwrap();

        println!("Worker {} got a job; executing.", id);

        job.job();
    }
});
```

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
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
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
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
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
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
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
