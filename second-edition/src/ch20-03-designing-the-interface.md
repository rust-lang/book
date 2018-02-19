#### Code Structure if We Could Spawn a Thread for Each Request

First, let’s explore how our code might look if it did create a new thread for
every connection. As mentioned, this isn’t our final plan due to the problems
with potentially spawning an unlimited number of threads, but it’s a starting
point. Listing 20-11 shows the changes to make to `main` to spawn a new thread
to handle each stream within the `for` loop:

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
run the code in the closure in the new thread. If you run this code and load
`/sleep` in your browser, then `/` in two more browser tabs, you’ll indeed see
the requests to `/` don’t have to wait for `/sleep` to finish. But as we
mentioned, this will eventually overwhelm the system because we’re making new
threads without any limit.

#### Creating a Similar Interface for a Finite Number of Threads

We want our thread pool to work in a similar, familiar way so that switching
from threads to a thread pool doesn’t require large changes to the code using
our API. Listing 20-12 shows the hypothetical interface for a `ThreadPool`
struct we’d like to use instead of `thread::spawn`:

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

<span class="caption">Listing 20-12: Our ideal `ThreadPool` interface</span>

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` has a
similar interface as `thread::spawn`, in that it takes a closure of what code
the pool should run for each stream. We need to implement `pool.execute` such
that it takes the closure and gives it to a thread in the pool to run. This
code won't yet compile, but we're going to try so the compiler can guide us in
how to fix it.

<!-- Can you be more specific here about how pool.execute will work? -->
<!-- So clarified. I hope this helps with some of the future confusion as well
/Carol -->

#### Building the `ThreadPool` Struct Using Compiler Driven Development

Go ahead and make the changes in Listing 20-12 to *src/main.rs*, and let’s use
the compiler errors from `cargo check` to drive our development. Here’s the
first error we get:

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

Great, this is telling us we need a `ThreadPool` type or module, so we'll build
one now. Our `ThreadPool` implementation will be independent of the kind of
work our web server is doing, so let’s switch the `hello` crate from a binary
crate to a library crate to hold our `ThreadPool` implementation. This also
means we could use the separate thread pool library for whatever work we want
to do, not just for serving web requests.

Create a *src/lib.rs* that contains the following, which is simplest definition
of a `ThreadPool` struct that we can have for now:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;
```

Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. This will make the library crate the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run` though. After moving the *main.rs* file,
edit it to bring the library crate in and bring `ThreadPool` into scope by
adding the following code to the top of *src/bin/main.rs*:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;
```

This still won't work, but let's try checking it again in order to get the next
error that we need to address:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0599]: no function or associated item named `new` found for type
`hello::ThreadPool` in the current scope
 --> src/bin/main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ function or associated item not found in
   `hello::ThreadPool`
```

Cool, this tells us that next we need to create an associated function named
`new` for `ThreadPool`. We also know that `new` needs to have one parameter
that can accept `4` as an argument, and should return a `ThreadPool` instance.
Let’s implement the simplest `new` function that will have those
characteristics:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

We picked `usize` as the type of the `size` parameter, because we know that a
negative number of threads makes no sense. We also know we're going to use this
4 as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the "Integer Types" section of Chapter 3.

Let’s check the code again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_size` instead

error[E0599]: no method named `execute` found for type `hello::ThreadPool` in the current scope
  --> src/bin/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

<!--Can you say a few words on why we would need an execute method, what Rust
needs it for? Also why we need a closure/what indicated that we need a closure
here? -->
<!-- *Rust* doesn't need it, the thread pool functionality we're working on
implementing needs it. I've tried to clarify without getting too repetitive
with the "Creating a Similar Interface for a Finite Number of Threads" section
/Carol -->

Now we get a warning and an error. Ignoring the warning for a moment, the error
occurs because we don’t have an `execute` method on `ThreadPool`. Recall from
the "Creating a Similar Interface for a Finite Number of Threads" section that
we decided our thread pool should have an interface similar to that of
`thread::spawn`, and that we're going to implement the `execute` function to
take the closure that it's given and give it to an idle thread in the pool to
run.

We'll define the `execute` method on `ThreadPool` to take a closure as a
parameter. If you remember from the "Storing Closures Using Generic Parameters
and the `Fn` Traits" section in Chapter 13, we can take closures as parameters
with three different traits: `Fn`, `FnMut`, and `FnOnce`. We need to decide
which kind of closure to use here. We know we’re going to end up doing
something similar to the standard library `thread::spawn` implementation, so we
can look at what bounds the signature of `thread::spawn` has on its parameter.
The documentation tells us:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; `T` is related to the return value and
we’re not concerned with that. We can see that `spawn` uses `FnOnce` as the
trait bound on `F`. This is probably what we want as well, because we’ll
eventually be passing the argument we get in `execute` to `spawn`. We can be
further confident that `FnOnce` is the trait we want to use because the thread
for running a request is only going to execute that request’s closure one time,
which matches the `Once` in `FnOnce`.

<!-- Above -- why does that second reason mean FnOnce is the trait to use, can
you remind us? -->
<!-- Attempted, we're just pointing out that it's in the name Fn*Once* /Carol
-->

`F` also has the trait bound `Send` and the lifetime bound `'static`, which are
useful for our situation: we need `Send` to transfer the closure from one
thread to another, and `'static` because we don’t know how long the thread will
take to execute. Let’s create an `execute` method on `ThreadPool` that will
take a generic parameter `F` with these bounds:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

We still use the `()` after `FnOnce` because this `FnOnce` is representing a
closure that takes no parameters and doesn’t return a value. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.

Again, we’ll add the simplest implementation of the `execute` method, which
does nothing, just to get our code compiling. Let’s check it again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_size` instead

warning: unused variable: `f`
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
  |
  = note: to avoid this warning, consider using `_f` instead
```

We're receiving only warnings now! That means it compiles! Note, though, that
if you try `cargo run` and make a request in the browser, you’ll see the errors
in the browser that we saw in the beginning of the chapter. Our library isn’t
actually calling the closure passed to `execute` yet!

> A saying you might hear about languages with strict compilers like Haskell
> and Rust is “if the code compiles, it works.” This is a good time to remember
> that this is not actually universally true. Our project compiles, but it does
> absolutely nothing! If we were building a real, complete project, this would
> be a great time to start writing unit tests to check that the code compiles
> *and* has the behavior we want.
