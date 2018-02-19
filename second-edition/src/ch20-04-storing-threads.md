#### Validating the Number of Threads in `new`

We're still getting warnings because we aren’t doing anything with the
parameters to `new` and `execute`. Let’s implement the bodies of these
functions with the behavior we want. To start, let’s think about `new`.

Earlier we chose an unsigned type for the `size` parameter, because a pool with
a negative number of threads makes no sense. However, a pool with zero threads
also makes no sense, yet zero is a perfectly valid `usize`. Let’s add code to
check that `size` is greater than zero before we return a `ThreadPool`
instance, and have the program panic if a zero is received by using the
`assert!` macro as shown in Listing 20-13:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct ThreadPool;
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

        ThreadPool
    }

    // --snip--
}
```

<span class="caption">Listing 20-13: Implementing `ThreadPool::new` to panic if
`size` is zero</span>

We’ve taken this opportunity to add some documentation for our `ThreadPool`
with doc comments. Note that we followed good documentation practices by adding
a section that calls out the situations in which our function can panic as we
discussed in Chapter 14. Try running `cargo doc --open` and clicking on the
`ThreadPool` struct to see what the generate docs for `new` look like!

Instead of adding the `assert!` macro as we’ve done here, we could make `new`
return a `Result` like we did with `Config::new` in the I/O project in Listing
12-9, but we’ve decided in this case that trying to create a thread pool
without any threads should be an unrecoverable error. If you’re feeling
ambitious, try to write a version of `new` with this signature to see how you
feel about both versions:

```rust,ignore
fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### Creating Space to Store the Threads

Now that we have a way to know we have a valid number of threads to store in
the pool, we can actually create those threads and store them in the
`ThreadPool` struct before returning it.

This raises a question: how do we “store” a thread? Let’s take another look at
the signature of `thread::spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`spawn` returns a `JoinHandle<T>`, where `T` is the type that’s returned from
the closure. Let’s try using `JoinHandle` too and see what happens. In our
case, the closures we’re passing to the thread pool will handle the connection
and not return anything, so `T` will be the unit type `()`.

The code in Listing 20-14 will compile, but isn't actually creating any threads
yet. We’ve changed the definition of `ThreadPool` to hold a vector of
`thread::JoinHandle<()>` instances, initialized the vector with a capacity of
`size`, set up a `for` loop that will run some code to create the threads, and
returned a `ThreadPool` instance containing them:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool {
            threads
        }
    }

    // --snip--
}
```

<span class="caption">Listing 20-14: Creating a vector for `ThreadPool` to hold
the threads</span>

We’ve brought `std::thread` into scope in the library crate, because we’re using
`thread::JoinHandle` as the type of the items in the vector in `ThreadPool`.

Once a valid size is recevied, our `ThreadPool` creates a new vector that can
hold `size` items. We haven’t used the `with_capacity` function in this book
yet, which does the same thing as `Vec::new`, but with an important difference:
it pre-allocates space in the vector. Because we know that we need to store
`size` elements in the vector, doing this allocation up-front is slightly more
efficient than using `Vec::new`, which resizes itself as elements get inserted.

If you run `cargo check` again, you’ll get a few more warnings, but it should
succeed.

#### A `Worker` Struct Responsible for Sending Code from the `ThreadPool` to a Thread

<!-- I wasn't sure what this next paagraph was relevant to, can you connect it
up more clearly?-->
<!-- This is where we're actually getting into the meat of the implementation,
I've tried to make it clearer :( /Carol-->

We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. How do we actually create threads? This is a tough question. The way
to create a thread provided by the standard library, `thread::spawn`, expects
to get some code that the thread should run as soon as the thread is created.
However, we want to start up the threads and have them wait for code that we
will send them later. The standard library's implementation of threads doesn't
include any way to do that; we have to implement it.

<!-- Can you say how doing this refactoring will improve the code -- why don't
we want the pool to store threads directly? (I got that from the listing
caption because I wasn't sure what the end game was) -->
<!-- I hope the end game is now clearer in the previous paragraph: we *can't*
store the threads directly and get the behavior we want. /Carol -->

The way we're going to implement the behavior of creating threads and sending
code later is to introduce a new data structure between the `ThreadPool` and
the threads that will manage this new behavior. We're going to call this data
structure `Worker`; this is a common term in pooling implementations. Think of
people working in the kitchen at a restaurant: the workers wait until orders
come in from customers, then they're responsible for taking those orders and
fulfilling them.

<!-- I was unclear on what a worker actually is here -- is this a
programming/Rust term, or just what we're calling the struct? Can you make it
clearer what the worker is and its responsibilities? -->
<!-- I've tried in the previous paragraph; it's a common term in job
queue/pooling implementations in programming in general but I think should make
sense in plain English with the real-life metaphor I've added /Carol -->

Instead of storing a vector of `JoinHandle<()>` instances in the thread pool,
we'll store instances of the `Worker` struct. Each `Worker` will store a single
`JoinHandle<()>` instance. Then we'll implement a method on `Worker` that will
take a closure of code to run and send it to the already-running thread for
execution. We'll also give each worker an `id` so we can tell the different
workers in the pool apart when logging or debugging.

First, let's make these changes to what happens when we create a `ThreadPool`.
We'll implement the code that sends the closure to the thread after we have
`Worker` set up in this way:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`
2. Change `ThreadPool` to hold a vector of `Worker` instances
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the allocated `id` and a thread spawned with an
   empty closure
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create
   a new `Worker` with that `id`, and store the worker in the vector

If you’re up for a challenge, try implementing these changes on your own before
taking a look at the code in Listing 20-15.

Ready? Here’s Listing 20-15 with one way to make these modifications:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }
    // --snip--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-15: Modifying `ThreadPool` to hold `Worker`
instances instead of holding threads directly</span>

We’ve changed the name of the field on `ThreadPool` from `threads` to `workers`
because it's now holding `Worker` instances instead of `JoinHandle<()>`
instances. We use the counter in the `for` loop as an argument to
`Worker::new`, and we store each new `Worker` in the vector named `workers`.

External code (like our server in *src/bin/main.rs*) doesn’t need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` function private. The
`Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>`
instance that's created by spawning a new thread using an empty closure.

This code will compile and and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`, but we’re *still* not processing
the closure that we get in `execute`. Let’s talk about how to do that next.
