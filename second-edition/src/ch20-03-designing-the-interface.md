## Designing the Thread Pool Interface

Let's talk about what using the pool should look like. The authors often find
that when trying to design some code, writing the client interface first can
really help guide your design. Write the code you'd want to use, then implement
it, rather than the other way around.

To do this, first, let's examine what the "create a new thread for every
connection" would look like. It's not our final plan due to the problems we
talked about earlier, but it's a start. Here's the changes to `main.rs`:

```rust,ignore
// add this import at the top:
use std::thread;

// and then this change to main:
for stream in listener.incoming() {
    let stream = stream.unwrap();

    thread::spawn(|| {
        handle_connection(stream);
    });
}
```

As we learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in it. We'd want our thread pool to work in a
similar way. Something like this:

```rust,ignore
// create a pool with four threads
let pool = ThreadPool::new(4);

for stream in listener.incoming() {
    let stream = stream.unwrap();

    // run this closure in the pool
    pool.execute(|| {
        handle_connection(stream);
    });
}
```

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, and then `pool.execute` in a similar way to `thread::spawn`. Go
ahead and make those changes to `main.rs`, and then let's use the compiler
errors to drive our development. Here's the first error we get:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module `ThreadPool`

error: aborting due to previous error
```

Great, we need a `ThreadPool`. Let's define one:

```rust
struct ThreadPool;
```

And try again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error: no associated item named `new` found for type `ThreadPool` in the current scope
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope; the following traits define an item `new`, perhaps you need to implement one of them:
   = help: candidate #1: `std::sys_common::thread_info::NewThread`
   = help: candidate #2: `std::iter::ZipImpl`

error: aborting due to previous error
```

The helpful messages aren't super helpful here; we need to define our own `new`
function, not implement a trait. Here it is:

```rust,ignore
impl ThreadPool {
    fn new() {
    }
}
```

Let's check it again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
  --> src\main.rs:10:32
   |
10 |       let pool = ThreadPool::new(4);
   |                                  ^ expected 0 parameters
...
49 |       fn new() {
   |  _____- starting here...
50 | |
51 | |     }
   | |_____- ...ending here: defined here

error: no method named `execute` found for type `()` in the current scope
  --> src\main.rs:15:14
   |
15 |         pool.execute(|| {
   |              ^^^^^^^

error: aborting due to 2 previous errors
```

Two errors: we need a parameter for `new`, and a type error. Let's focus on the
first error for now:

```rust,ignore
impl ThreadPool {
    fn new(size: u32) {

    }
}
```

Why a `u32`? We know that a negative number of threads makes no sense, so an
unsigned value makes sense. `u32` is a solid default. Once we actually
implement `new` for real, we'll reconsider it, but for now, we're just working
through compiler errors.

And check again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error: no method named `execute` found for type `()` in the current scope
  --> src\main.rs:15:14
   |
15 |         pool.execute(|| {
   |              ^^^^^^^

error: aborting due to previous error
```

Okay, now we only have the second error. It's slightly obtuse: because `new`
doesn't return anything, `pool` has the type unit. And unit doesn't have an
`execute` method. What we actually intended was for `new` to return a
`ThreadPool`, so let's fix that, and then also add the `execute` method:

```rust,ignore
impl ThreadPool {
    fn new(size: u32) -> ThreadPool {
        ThreadPool
    }

    fn execute(&self) {

    }
}
```

Let's check again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
error[E0061]: this function takes 0 parameters but 1 parameter was supplied
  --> src\main.rs:15:22
   |
15 |           pool.execute(|| {
   |  ______________________^ starting here...
16 | |             handle_connection(stream);
17 | |         });
   | |_________^ ...ending here: expected 0 parameters
...
53 |       fn execute(&self) {
   |  _____- starting here...
54 | |
55 | |     }
   | |_____- ...ending here: defined here

error: aborting due to previous error
```

We need `execute` to take a closure parameter. If you remember from Chapter 13,
we can take closures as arguments with three different traits: `Fn`, `FnMut`,
and `FnOnce`. What kind of closure should we use? Well, we know we're going to
end up doing something similar to `thread::spawn`; what bounds does it have?

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; not `T`. Given that `spawn` uses
`FnOnce`, it's probably what we want as well, given that we're eventually
passing something to `spawn`. In addition, we have a `Send` and `'static`
bound, which also makes sense: we need `Send` to transfer something from one
thread to another, and `'static` because we don't know how long the thread will
execute. Let's modify `execute` to have these bounds:

```rust,ignore
fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
{

}
```

Let's check again:

```text
> cargo check
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
warning: unused import: `std::thread`, #[warn(unused_imports)] on by default
 --> src\main.rs:5:5
  |
5 | use std::thread;
  |     ^^^^^^^^^^^

warning: unused variable: `size`, #[warn(unused_variables)] on by default
  --> src\main.rs:49:12
   |
49 |     fn new(size: usize) -> ThreadPool {
   |            ^^^^

warning: unused variable: `f`, #[warn(unused_variables)] on by default
  --> src\main.rs:53:26
   |
53 |     fn execute<F>(&self, f: F)
   |                          ^
```

It compiles!

> This is a good time to remember that while "if it compiles, it works" is
> often true of Rust code, it's not universal. Our project compiles, but does
> absolutely nothing! If we were building something real, this would be a great
> time to start writing unit tests.

We do have some warnings; we're no longer using `std::thread`, and we aren't
doing anything with our arguments. Let's implement both of these methods on our
`ThreadPool`.

To start, let's think about `new`. The first thing that matters is something we
said above: a pool with a negative number of threads makes no sense. However, a
pool with zero threads also makes no sense, yet zero is a perfectly valid
`u32`. Let's check that our number is greater than zero:

```rust,ignore
/// Create a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
fn new(size: u32) -> ThreadPool {
    assert!(size > 0);

    ThreadPool
}
```

We've added some documentation for our `ThreadPool` with doc comments. Careful
observers will note we called out the situations in which our function can
panic as well; see Chapter 14 for more details on writing good documentation.

We've also added in an `assert!` to check the validity of `Size`. We could also
make `new` return a `Result` instead, but it involves a bunch of more code, and
arguably, passing in a zero is incoherent, and therefore deserves to be an
unrecoverable error rather than a recoverable one. If you're feeling ambitious,
try to write a version of `new` with this signature:

```rust,ignore
fn new(size: u32) -> Result<ThreadPool, PoolCreationError> {
```

See how you feel about both versions.
