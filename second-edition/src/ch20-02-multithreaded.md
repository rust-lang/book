## Turning our Single Threaded Server into a Multithreaded Server

<!-- Reading ahead, the original heading didn't seem to fit all of the sub
headings -- this might not be totally right either, so feel free to replace
with something more appropriate -->
<!-- This is fine! /Carol -->

Right now, the server will process each request in turn, meaning it won’t
process a second connection until the first is finished processing. If this
server were to receive more and more requests, this sort of serial execution
would prove to be less and less optimal. If the server receives a request that
takes a long time to process, subsequent requests will have to wait until the
long request is finished, even if the new requests can be processed quickly.
We’ll need to fix this, but first, we’ll look at the problem in action.

### Simulating a Slow Request in the Current Server Implementation

Let’s see how a slow-processing request can affect other requests made to our
current server implementation. Listing 20-10 implements handling a request to
`/sleep` with a simulated slow response that will cause the server to sleep for
five seconds before responding.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --snip--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
    // --snip--

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // --snip--
}
```

<span class="caption">Listing 20-10: Simulating a slow request by recognizing
`/sleep` and sleeping for 5 seconds</span>

This code is a bit messy, but it’s good enough for our simulation purposes! We
created a second request `sleep`, whose data our server recognizes. We added an
`else if` after the `if` block to check for the request to `/sleep`, and when
that request is received, our server will sleep for five seconds before
rendering the successful HTML page.

You can really see how primitive our server is here; real libraries would
handle the recognition of multiple requests in a much less verbose way!

Start the server with `cargo run`, and then open up two browser windows: one
for `http://localhost:7878/` and one for `http://localhost:7878/sleep`. If you
enter the `/` URI a few times, as before, you’ll see it respond quickly. But if
you enter `/sleep`, and then load up `/`, you’ll see that `/` waits until
`sleep` has slept for its full five seconds before loading.

There are multiple ways we could change how our web server works in order to
avoid having all requests back up behind a slow request; the one we’re going to
implement is a thread pool.

### Improving Throughput with a Thread Pool

<!--There seems to be some repetition throughout these thread pool sections, is
there any way to condense it? I've edited with this in mind, but am wary of
changing too much -->
<!-- Your edits that removed repetition are fine! /Carol -->

A *thread pool* is a group of spawned threads that are waiting and ready to
handle some task. When the program receives a new task, it will assign one of
the threads in the pool to the task, and that thread will go off and process
the task. The remaining threads in the pool are available to handle any other
tasks that come in while the first thread is processing. When the first thread
is done processing its task, it’s returned to the pool of idle threads ready to
handle a new task. A thread pool will allow us to process connections
concurrently, increasing the throughput of our server.

We’ll limit the number of threads in the pool to a small number to protect us
from Denial of Service (DoS) attacks; if we had our program create a new thread
for each request as it comes in, someone making ten million requests to our
server could create havoc by using up all of our server’s resources and
grinding the processing of all requests to a halt.

Rather than spawning unlimited threads, then, we’ll have a fixed number of
threads waiting in the pool. As requests come in, they’ll be sent to the pool
for processing. The pool will maintain a queue of incoming requests. Each of
the threads in the pool will pop a request off of this queue, handle the
request, and then ask the queue for another request. With this design, we can
process `N` requests concurrently, where `N` is the number of threads. If each
thread is responding to a long-running request, subsequent requests can still
back up in the queue, but we’ve increased the number of long-running requests
we can handle before that point.

This is just one of many ways to improve the throughput of our web server.
Other options you might explore are the fork/join model and the single threaded
async I/O model. If you’re interested in this topic, you may want to read more
about other solutions and try to implement them in Rust; with a low-level
language like Rust, all of these options are possible.

Before we begin, let’s talk about what using the pool should look like. When
trying to design code, writing the client interface first can really help guide
your design. Write the API of the code so that it’s structured in the way you’d
want to call it, then implement the functionality within that structure, rather
than implementing the functionality then designing the public API.

Similar to how we used Test Driven Development in the project in Chapter 12,
we’re going to use Compiler Driven Development here. We’ll write the code that
calls the functions we wish we had, then we’ll look at errors from the compiler
to tell us what we should change next to get things working.

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
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
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
code won’t yet compile, but we’re going to try so the compiler can guide us in
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

Great, this is telling us we need a `ThreadPool` type or module, so we’ll build
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

This still won’t work, but let’s try checking it again in order to get the next
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
negative number of threads makes no sense. We also know we’re going to use this
4 as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the “Integer Types” section of Chapter 3.

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
the “Creating a Similar Interface for a Finite Number of Threads” section that
we decided our thread pool should have an interface similar to that of
`thread::spawn`, and that we’re going to implement the `execute` function to
take the closure that it’s given and give it to an idle thread in the pool to
run.

We’ll define the `execute` method on `ThreadPool` to take a closure as a
parameter. If you remember from the “Storing Closures Using Generic Parameters
and the `Fn` Traits” section in Chapter 13, we can take closures as parameters
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

We’re receiving only warnings now! That means it compiles! Note, though, that
if you try `cargo run` and make a request in the browser, you’ll see the errors
in the browser that we saw in the beginning of the chapter. Our library isn’t
actually calling the closure passed to `execute` yet!

> A saying you might hear about languages with strict compilers like Haskell
> and Rust is “if the code compiles, it works.” This is a good time to remember
> that this is not actually universally true. Our project compiles, but it does
> absolutely nothing! If we were building a real, complete project, this would
> be a great time to start writing unit tests to check that the code compiles
> *and* has the behavior we want.

#### Validating the Number of Threads in `new`

We’re still getting warnings because we aren’t doing anything with the
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

The code in Listing 20-14 will compile, but isn’t actually creating any threads
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

Once a valid size is received, our `ThreadPool` creates a new vector that can
hold `size` items. We haven’t used the `with_capacity` function in this book
yet, which does the same thing as `Vec::new`, but with an important difference:
it pre-allocates space in the vector. Because we know that we need to store
`size` elements in the vector, doing this allocation up-front is slightly more
efficient than using `Vec::new`, which resizes itself as elements get inserted.

If you run `cargo check` again, you’ll get a few more warnings, but it should
succeed.

#### A `Worker` Struct Responsible for Sending Code from the `ThreadPool` to a Thread

<!-- I wasn't sure what this next paragraph was relevant to, can you connect it
up more clearly?-->
<!-- This is where we're actually getting into the meat of the implementation,
I've tried to make it clearer :( /Carol-->

We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. How do we actually create threads? This is a tough question. The way
to create a thread provided by the standard library, `thread::spawn`, expects
to get some code that the thread should run as soon as the thread is created.
However, we want to start up the threads and have them wait for code that we
will send them later. The standard library’s implementation of threads doesn’t
include any way to do that; we have to implement it.

<!-- Can you say how doing this refactoring will improve the code -- why don't
we want the pool to store threads directly? (I got that from the listing
caption because I wasn't sure what the end game was) -->
<!-- I hope the end game is now clearer in the previous paragraph: we *can't*
store the threads directly and get the behavior we want. /Carol -->

The way we’re going to implement the behavior of creating threads and sending
code later is to introduce a new data structure between the `ThreadPool` and
the threads that will manage this new behavior. We’re going to call this data
structure `Worker`; this is a common term in pooling implementations. Think of
people working in the kitchen at a restaurant: the workers wait until orders
come in from customers, then they’re responsible for taking those orders and
fulfilling them.

<!-- I was unclear on what a worker actually is here -- is this a
programming/Rust term, or just what we're calling the struct? Can you make it
clearer what the worker is and its responsibilities? -->
<!-- I've tried in the previous paragraph; it's a common term in job
queue/pooling implementations in programming in general but I think should make
sense in plain English with the real-life metaphor I've added /Carol -->

Instead of storing a vector of `JoinHandle<()>` instances in the thread pool,
we’ll store instances of the `Worker` struct. Each `Worker` will store a single
`JoinHandle<()>` instance. Then we’ll implement a method on `Worker` that will
take a closure of code to run and send it to the already-running thread for
execution. We’ll also give each worker an `id` so we can tell the different
workers in the pool apart when logging or debugging.

First, let’s make these changes to what happens when we create a `ThreadPool`.
We’ll implement the code that sends the closure to the thread after we have
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
because it’s now holding `Worker` instances instead of `JoinHandle<()>`
instances. We use the counter in the `for` loop as an argument to
`Worker::new`, and we store each new `Worker` in the vector named `workers`.

External code (like our server in *src/bin/main.rs*) doesn’t need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` function private. The
`Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>`
instance that’s created by spawning a new thread using an empty closure.

This code will compile and and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`, but we’re *still* not processing
the closure that we get in `execute`. Let’s talk about how to do that next.

#### Sending Requests to Threads Via Channels

The next problem to tackle is that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method, but we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.

We want the `Worker` structs that we just created to fetch code to run from a
queue held in the `ThreadPool`, and send that code to its thread to run.

In Chapter 16, we learned about *channels*---a simple way to communicate
between two threads---that would be perfect for this use-case. We’ll use a
channel to function as the queue of jobs, and `execute` will send a job from
the `ThreadPool` to the `Worker` instances, which will send the job to its
thread. Here’s the plan:

1. `ThreadPool` will create a channel and hold on to the sending side of the
   channel.
2. Each `Worker` will hold on to the receiving side of the channel.
3. We’ll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute down the sending
   side of the channel.
5. In its thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.

Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. `Job` is a struct
that doesn’t hold anything for now, but will be the type of item we’re sending
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
won’t work, as we recall from Chapter 16: the channel implementation provided
by Rust is multiple *producer*, single *consumer*. This means we can’t just
clone the consuming end of the channel to fix this. Even if we could, that’s
not the technique we’d want to use; we want to distribute the jobs across
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
type of closure that `execute` receives. As we discussed in the “Type Aliases
Create Type Synonyms” section of Chapter 19, type aliases allow us to make long
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
receiving new messages. At the moment, though, we can’t stop our threads
executing; our threads continue executing as long as the pool exists. The
reason we use `unwrap`, then, is that we we know the failure case won’t happen
but the compiler can’t tell that.

But we’re not quite done yet! In the worker, our closure being passed to
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
holding the lock, rather than releasing the lock. In this situation, calling
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
general, Rust doesn’t allow us to move value out of a `Box<T>` because Rust
doesn’t know how big the value inside the `Box<T>` is going to be; recall in
Chapter 15 that we used `Box<T>` precisely because we had something of an
unknown size that we wanted to store in a `Box<T>` to get a value of a known
size.

We saw in Chapter 17, Listing 17-15 that we can write methods that use the
syntax `self: Box<Self>`, which allows the method to take ownership of a `Self`
value stored in a `Box<T>`. That’s exactly what we want to do here, but
unfortunately Rust won’t let us: the part of Rust that implements behavior when
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
wondering why we didn’t write the worker thread like this:

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

This code compiles and runs, but doesn’t result in the desired threading
behavior: a slow request will still cause other requests to wait to be
processed. The reason why is somewhat subtle: the `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. This allows the borrow checker to enforce at compile time that
we never access a resource guarded by a `Mutex` without holding the lock, but
it can also result in holding the lock longer than intended if we don’t think
carefully about the lifetime of the `MutexGuard<T>`. Because the values in the
the `while` expression remain in scope for the duration of the block, the lock
remains held for the duration of the call to `job.call_box()`, meaning other
workers cannot receive jobs.

By using `loop` instead and acquiring the lock and a job within the block
rather than outside it, the `MutexGuard` returned from the `lock` method is
dropped as soon as the `let job` statement ends. This ensures that the lock is
held during the call to `recv`, but it is released before the call to
`job.call_box()`, allowing multiple requests to be serviced concurrently.
