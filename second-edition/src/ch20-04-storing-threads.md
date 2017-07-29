## Creating the Thread Pool and Storing Threads

The warnings are because we aren’t doing anything with the parameters to `new`
and `execute`. Let’s implement the bodies of both of these with the actual
behavior we want.

### Validating the Number of Threads in the Pool

To start, let’s think about `new`. We mentioned before that we picked an
unsigned type for the `size` parameter since a pool with a negative number of
threads makes no sense. However, a pool with zero threads also makes no sense,
yet zero is a perfectly valid `u32`. Let’s check that `size` is greater than
zero before we return a `ThreadPool` instance and panic if we get zero by using
the `assert!` macro as shown in Listing 20-13:

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
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // ...snip...
}
```

<span class="caption">Listing 20-13: Implementing `ThreadPool::new` to panic if
`size` is zero</span>

We’ve taken this opportunity to add some documentation for our `ThreadPool`
with doc comments. Note that we followed good documentation practices and added
a section that calls out the situations in which our function can panic as we
discussed in Chapter 14. Try running `cargo doc --open` and clicking on the
`ThreadPool` struct to see what the generate docs for `new` look like!

Instead of adding the use of the `assert!` macro as we’ve done here, we could
make `new` return a `Result` instead like we did with `Config::new` in the I/O
project in Listing 12-9, but we’ve decided in this case that trying to create a
thread pool without any threads should be an unrecoverable error. If you’re
feeling ambitious, try to write a version of `new` with this signature to see
how you feel about both versions:

```rust,ignore
fn new(size: u32) -> Result<ThreadPool, PoolCreationError> {
```

### Storing Threads in the Pool

Now that we know we have a valid number of threads to store in the pool, we can
actually create that many threads and store them in the `ThreadPool` struct
before returning it.

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

This won’t compile yet, but let’s consider the code shown in Listing 20-14.
We’ve changed the definition of `ThreadPool` to hold a vector of
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
    // ...snip...
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool {
            threads
        }
    }

    // ...snip...
}
```

<span class="caption">Listing 20-14: Creating a vector for `ThreadPool` to hold
the threads</span>

We’ve brought `std::thread` into scope in the library crate, since we’re using
`thread::JoinHandle` as the type of the items in the vector in `ThreadPool`.

After we have a valid size, we’re creating a new vector that can hold `size`
items. We haven’t used `with_capacity` in this book yet; it does the same thing
as `Vec::new`, but with an important difference: it pre-allocates space in the
vector. Since we know that we need to store `size` elements in the vector,
doing this allocation up-front is slightly more efficient than only writing
`Vec::new`, since `Vec::new` resizes itself as elements get inserted. Since
we’ve created a vector the exact size that we need up front, no resizing of the
underlying vector will happen while we populate the items.

That is, if this code works, which it doesn’t quite yet! If we check this code,
we get an error:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0308]: mismatched types
  --> src\main.rs:70:46
   |
70 |         let mut threads = Vec::with_capacity(size);
   |                                              ^^^^ expected usize, found u32

error: aborting due to previous error
```

`size` is a `u32`, but `Vec::with_capacity` needs a `usize`. We have two
options here: we can change our function’s signature, or we can cast the `u32`
as a `usize`. If you remember when we defined `new`, we didn’t think too hard
about what number type made sense, we just chose one. Let’s give it some more
thought now. Given that `size` is the length of a vector, `usize` makes a lot
of sense. They even almost share a name! Let’s change the signature of `new`,
which will get the code in Listing 20-14 to compile:

```rust,ignore
fn new(size: usize) -> ThreadPool {
```

If run `cargo check` again, you’ll get a few more warnings, but it should
succeed.

We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. How do we actually create threads? This is a tough question. What
should go in these threads? We don’t know what work they need to do at this
point, since the `execute` method takes the closure and gives it to the pool.

Let’s refactor slightly: instead of storing a vector of `JoinHandle<()>`
instances, let’s create a new struct to represent the concept of a *worker*. A
worker will be what receives a closure in the `execute` method, and it will
take care of actually calling the closure. In addition to letting us store a
fixed `size` number of `Worker` instances that don’t yet know about the
closures they’re going to be executing, we can also give each worker an `id` so
we can tell the different workers in the pool apart when logging or debugging.

Let’s make these changes:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`
2. Change `ThreadPool` to hold a vector of `Worker` instances
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance with that `id` and a thread spawned with an empty closure,
   which we’ll fix soon
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
    // ...snip...
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
    // ...snip...
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
instances instead of threads directly</span>

We’ve chosen to change the name of the field on `ThreadPool` from `threads` to
`workers` since we’ve changed what we’re holding, which is now `Worker`
instances instead of `JoinHandle<()>` instances. We use the counter in the
`for` loop as an argument to `Worker::new`, and we store each new `Worker` in
the vector named `workers`.

The `Worker` struct and its `new` function are private since external code
(like our server in *src/bin/main.rs*) doesn’t need to know the implementation
detail that we’re using a `Worker` struct within `ThreadPool`. The
`Worker::new` function uses the given `id` and stores a `JoinHandle<()>`
created by spawning a new thread using an empty closure.

This code compiles and is storing the number of `Worker` instances that we
specified as an argument to `ThreadPool::new`, but we’re *still* not processing
the closure that we get in `execute`. Let’s talk about how to do that next.
