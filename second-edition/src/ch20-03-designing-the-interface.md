## Designing the Thread Pool Interface

Let’s talk about what using the pool should look like. The authors often find
that when trying to design some code, writing the client interface first can
really help guide your design. Write the API of the code to be structured in
the way you’d want to call it, then implement the functionality within that
structure rather than implementing the functionality then designing the public
API.

Similar to how we used Test Driven Development in the project in Chapter 12,
we’re going to use Compiler Driven Development here. We’re going to write the
code that calls the functions we wish we had, then we’ll lean on the compiler
to tell us what we should change next. The compiler error messages will guide
our implementation.

### Code Structure if We Could Use `thread::spawn`

First, let’s explore what the code to create a new thread for every connection
could look like. This isn’t our final plan due to the problems with potentially
spawning an unlimited number of threads that we talked about earlier, but it’s
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
then `/` in two browser tabs, you’ll indeed see the request to `/` doesn’t have
to wait for `/sleep` to finish. But as we mentioned, this will eventually
overwhelm the system since we’re making new threads without any limit.

### Creating a Similar Interface for `ThreadPool`

We want our thread pool to work in a similar, familiar way so that switching
from threads to a thread pool doesn’t require large changes to the code we want
to run in the pool. Listing 20-12 shows the hypothetical interface for a
`ThreadPool` struct we’d like to use instead of `thread::spawn`:

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
`ThreadPool` we’re going to implement</span>

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` will
work in a similar way to `thread::spawn`.

### Compiler Driven Development to Get the API Compiling

Go ahead and make the changes in Listing 20-12 to *src/main.rs*, and let’s use
the compiler errors to drive our development. Here’s the first error we get:

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

Great, we need a `ThreadPool`. Let’s switch the `hello` crate from a binary
crate to a library crate to hold our `ThreadPool` implementation, since the
thread pool implementation will be independent of the particular kind of work
that we’re doing in our web server. Once we’ve got the thread pool library
written, we could use that functionality to do whatever work we want to do, not
just serve web requests.

So create *src/lib.rs* that contains the simplest definition of a `ThreadPool`
struct that we can have for now:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;
```

Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. This will make the library crate be the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run` though. After moving the *main.rs* file,
edit it to bring the library crate in and bring `ThreadPool` into scope by
adding this at the top of *src/bin/main.rs*:

<span class="filename">Filename: src/bin/main.rs</span>

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
Let’s implement the simplest `new` function that will have those
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
actually implement `new` for real, we’ll reconsider whether this is the right
choice for what the implementation needs, but for now, we’re just working
through compiler errors.

Let’s check the code again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

error: no method named `execute` found for type `hello::ThreadPool` in the
current scope
  --> src/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

Okay, a warning and an error. Ignoring the warning for a moment, the error is
because we don’t have an `execute` method on `ThreadPool`. Let’s define one,
and we need it to take a closure. If you remember from Chapter 13, we can take
closures as arguments with three different traits: `Fn`, `FnMut`, and `FnOnce`.
What kind of closure should we use? Well, we know we’re going to end up doing
something similar to `thread::spawn`; what bounds does the signature of
`thread::spawn` have on its argument? Let’s look at the documentation, which
says:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; `T` is related to the return value and
we’re not concerned with that. Given that `spawn` uses `FnOnce` as the trait
bound on `F`, it’s probably what we want as well, since we’ll eventually be
passing the argument we get in `execute` to `spawn`. We can be further
confident that `FnOnce` is the trait that we want to use since the thread for
running a request is only going to execute that request’s closure one time.

`F` also has the trait bound `Send` and the lifetime bound `'static`, which
also make sense for our situation: we need `Send` to transfer the closure from
one thread to another, and `'static` because we don’t know how long the thread
will execute. Let’s create an `execute` method on `ThreadPool` that will take a
generic parameter `F` with these bounds:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    // ...snip...

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

The `FnOnce` trait still needs the `()` after it since this `FnOnce` is
representing a closure that takes no parameters and doesn’t return a value.
Just like function definitions, the return type can be omitted from the
signature, but even if we have no parameters, we still need the parentheses.

Again, since we’re working on getting the interface compiling, we’re adding the
simplest implementation of the `execute` method, which does nothing. Let’s
check again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

warning: unused variable: `f`, #[warn(unused_variables)] on by default
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
```

Only warnings now! It compiles! Note that if you try `cargo run` and making a
request in the browser, though, you’ll see the errors in the browser again that
we saw in the beginning of the chapter. Our library isn’t actually calling the
closure passed to `execute` yet!

> A saying you might hear about languages with strict compilers like Haskell
> and Rust is “if the code compiles, it works.” This is a good time to remember
> that this is just a phrase and a feeling people sometimes have, it’s not
> actually universally true. Our project compiles, but it does absolutely
> nothing! If we were building a real, complete project, this would be a great
> time to start writing unit tests to check that the code compiles *and* has
> the behavior we want.
