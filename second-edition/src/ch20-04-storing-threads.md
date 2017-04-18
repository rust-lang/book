## Storing Threads in the Pool

Let's write some more code. Here's the sketch for the changes we need to make:

* create `size` new threads
* store these new threads inside the `ThreadPool` and return it.

This raises a question: how do we "store" a thread? Let's turn again to the
signature of `spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`spawn` returns a `JoinHandle<T>`, where `T` is the type that's returned from
the closure. In our case, we're handling our connection and not returning
anything, so `T` will be `()`, unit, here.

This won't compile yet, but let's start here:

```rust,ignore
struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: u32) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in threads
        }

        ThreadPool {
            threads: threads,
        }
    }
```

We get an error:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0308]: mismatched types
  --> src\main.rs:70:46
   |
70 |         let mut threads = Vec::with_capacity(size);
   |                                              ^^^^ expected usize, found u32

error: aborting due to previous error
```

`size` is a `u32`, but `Vec::with_capacity` needs a `usize`. We haven't used
`with_capacity` in this book yet; it does the same thing as `Vec::new`, but
with an important difference: it pre-allocates space in the vector. Since we
know that we need to store `size` elements in the vector, doing this allocation
up-front is slightly more efficient than only writing `Vec::new`, and it's not
harder to write. Well, until we get an error like this!

We have two options here: we can change our function's signature, or we can
cast. If you remember when we defined `new`, we didn't think too hard about
what number made sense, we just chose one. Let's give it some more thought now.
Given that `size` is the length of a vector, `usize` makes a lot of sense. They
even almost share a name! Let's change the signature, and this will now compile:

```rust,ignore
fn new(size: usize) -> ThreadPool {
```

If you check this out with `cargo check`, you'll get a few more warnings, but
it should succeed. We left a little comment above regarding the creation of
threads. This is a tough question though... what should go in these threads
we've created? We don't know what work they need to do, because the `execute`
method takes the closure and gives it to the pool.

Let's refactor slightly: instead of storing a vector of `JoinHandle<()>`s,
let's create a new `struct` to represent each of these 'workers'. We can also
then give each worker an `id` so we can tell them apart when logging or
debugging.

This won't work yet, but let's start here:

```rust,ignore
struct ThreadPool {
    threads: Vec<Worker>,
}

struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        let thread = thread::spawn(||{ });

        Worker {
            id: id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i as u32));
        }

        ThreadPool {
            threads: threads,
        }
    }

```

Here we've made a few changes:

1. `ThreadPool` now has a vector of `Worker`s.
2. `Worker`s have a `new` method that takes an `id` number
3. Currently the closure a `Worker` has does nothing; we'll fix this soon.
4. In `ThreadPool::new`, we use the loop counter to generate an `id`, and
   then create a new worker for each iteration. We use `as` to convert between
   `u32` and `usize` here.

This compiles, though we still get a number of warnings. Let's keep going!
