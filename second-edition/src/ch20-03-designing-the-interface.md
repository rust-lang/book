## Designing the Thread Pool Interface

Let's talk about what using the pool should look like. The authors often find
that when trying to design some code, writing the client interface first can
really help guide your design. Write the API of the code to be structured in
the way you'd want to call it, then implement the functionality within that
structure rather than implementing the functionality then desiging the public
API.

First, let's explore what the code to create a new thread for every connection
could look like. This isn't our final plan due to the problems with potentially
spawning an unlimited number of threads that we talked about earlier, but it's
a start. Listing 20-11 shows the changes to `main` to spawn a new thread to
handle each stream within the `for` loop:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">Listing 20-11: Spawning a new thread for each
stream</span>

As we learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in it. If you run this code and load `/sleep` and
then `/` in two browser tabs, you'll indeed see the request to `/` doesn't have
to wait for `/sleep` to finish. But as we mentioned, this will eventually
overwhelm the system since we're making new threads without any limit.

We'd want our thread pool to work in a similar, familiar way so that switching
from threads to a thread pool doesn't require large changes to the code we want
to run in the pool. Listing 20-12 shows the hypothetical interface for a
`ThreadPool` struct we'd like to use instead of `thread::spawn`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
# struct ThreadPool;
# impl ThreadPool {
#    fn new(size: u32) -> ThreadPool { ThreadPool }
#    fn execute<F>(&self, f: F)
#        where F: FnOnce() + Send + 'static {}
# }
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">Listing 20-12: How we want to be able to use the
`ThreadPool` we're going to implement</span>

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` will
work in a similar way to `thread::spawn`. Go ahead and make the changes in
Listing 20-12 to *src/main.rs*, and let's use the compiler errors to drive our
development. Here's the first error we get:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module
   `ThreadPool`

error: aborting due to previous error
```

Great, we need a `ThreadPool`. Let's start a library crate alongside our binary
crate to hold our `ThreadPool` implementation, since the thread pool
implementation will be independent of the particular kind of work that we're
doing in our web server. Once we've got the thread pool library written, we
could use that functionality to do whatever work we want to do, not just serve
web requests.

So create *src/lib.rs* that contains the simplest definition of a `ThreadPool`
struct that we can have for now:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;
```

Then bring the library crate into the binary crate and bring `ThreadPool` into
scope by adding this at the top of *src/main.rs*:

```rust,ignore
extern crate hello;
use hello::ThreadPool;
```

And try again in order to get the next error that we need to address:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error: no associated item named `new` found for type `hello::ThreadPool` in the
current scope
  --> src\main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^
   |
```

Cool, the next thing is to create an associated function named `new` for
`ThreadPool`. We also know that `new` needs to have one parameter that can
accept `4` as an argument, and `new` should return a `ThreadPool` instance.
Let's implement the simplest `new` function that will have those
characteristics:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        ThreadPool
    }
}
```

We picked `u32` as the type of the `size` parameter, since we know that a
negative number of threads makes no sense. `u32` is a solid default. Once we
actually implement `new` for real, we'll reconsider whether this is the right
choice for what the implementation needs, but for now, we're just working
through compiler errors.

Let's check the code again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

error: no method named `execute` found for type `hello::ThreadPool` in the current scope
  --> src/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^

error: aborting due to previous error


$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
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


And check again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
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
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
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
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
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
