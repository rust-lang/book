## Turning Our Single-Threaded Server into a Multithreaded Server

Right now, the server will process each request in turn, meaning it won’t
process a second connection until the first is finished processing. If the
server received more and more requests, this serial execution would be less and
less optimal. If the server receives a request that takes a long time to
process, subsequent requests will have to wait until the long request is
finished, even if the new requests can be processed quickly. We’ll need to fix
this, but first, we’ll look at the problem in action.

### Simulating a Slow Request in the Current Server Implementation

We’ll look at how a slow-processing request can affect other requests made to
our current server implementation. Listing 20-10 implements handling a request
to */sleep* with a simulated slow response that will cause the server to sleep
for 5 seconds before responding.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-10/src/main.rs:here}}
```

<span class="caption">Listing 20-10: Simulating a slow request by recognizing
*/sleep* and sleeping for 5 seconds</span>

This code is a bit messy, but it’s good enough for simulation purposes. We
created a second request `sleep`, whose data our server recognizes. We added an
`else if` after the `if` block to check for the request to */sleep*. When that
request is received, the server will sleep for 5 seconds before rendering the
successful HTML page.

You can see how primitive our server is: real libraries would handle the
recognition of multiple requests in a much less verbose way!

Start the server using `cargo run`. Then open two browser windows: one for
*http://127.0.0.1:7878/* and the other for *http://127.0.0.1:7878/sleep*. If
you enter the */* URI a few times, as before, you’ll see it respond quickly.
But if you enter */sleep* and then load */*, you’ll see that */* waits until
`sleep` has slept for its full 5 seconds before loading.

There are multiple ways we could change how our web server works to avoid
having more requests back up behind a slow request; the one we’ll implement is
a thread pool.

### Improving Throughput with a Thread Pool

A *thread pool* is a group of spawned threads that are waiting and ready to
handle a task. When the program receives a new task, it assigns one of the
threads in the pool to the task, and that thread will process the task. The
remaining threads in the pool are available to handle any other tasks that come
in while the first thread is processing. When the first thread is done
processing its task, it’s returned to the pool of idle threads, ready to handle
a new task. A thread pool allows you to process connections concurrently,
increasing the throughput of your server.

We’ll limit the number of threads in the pool to a small number to protect us
from Denial of Service (DoS) attacks; if we had our program create a new thread
for each request as it came in, someone making 10 million requests to our
server could create havoc by using up all our server’s resources and grinding
the processing of requests to a halt.

Rather than spawning unlimited threads, we’ll have a fixed number of threads
waiting in the pool. As requests come in, they’ll be sent to the pool for
processing. The pool will maintain a queue of incoming requests. Each of the
threads in the pool will pop off a request from this queue, handle the request,
and then ask the queue for another request. With this design, we can process
`N` requests concurrently, where `N` is the number of threads. If each thread
is responding to a long-running request, subsequent requests can still back up
in the queue, but we’ve increased the number of long-running requests we can
handle before reaching that point.

This technique is just one of many ways to improve the throughput of a web
server. Other options you might explore are the fork/join model and the
single-threaded async I/O model. If you’re interested in this topic, you can
read more about other solutions and try to implement them in Rust; with a
low-level language like Rust, all of these options are possible.

Before we begin implementing a thread pool, let’s talk about what using the
pool should look like. When you’re trying to design code, writing the client
interface first can help guide your design. Write the API of the code so it’s
structured in the way you want to call it; then implement the functionality
within that structure rather than implementing the functionality and then
designing the public API.

Similar to how we used test-driven development in the project in Chapter 12,
we’ll use compiler-driven development here. We’ll write the code that calls the
functions we want, and then we’ll look at errors from the compiler to determine
what we should change next to get the code to work.

#### Code Structure If We Could Spawn a Thread for Each Request

First, let’s explore how our code might look if it did create a new thread for
every connection. As mentioned earlier, this isn’t our final plan due to the
problems with potentially spawning an unlimited number of threads, but it is a
starting point. Listing 20-11 shows the changes to make to `main` to spawn a
new thread to handle each stream within the `for` loop.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-11/src/main.rs:here}}
```

<span class="caption">Listing 20-11: Spawning a new thread for each
stream</span>

As you learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in the new thread. If you run this code and load
*/sleep* in your browser, then */* in two more browser tabs, you’ll indeed see
that the requests to */* don’t have to wait for */sleep* to finish. But as we
mentioned, this will eventually overwhelm the system because you’d be making
new threads without any limit.

#### Creating a Similar Interface for a Finite Number of Threads

We want our thread pool to work in a similar, familiar way so switching from
threads to a thread pool doesn’t require large changes to the code that uses
our API. Listing 20-12 shows the hypothetical interface for a `ThreadPool`
struct we want to use instead of `thread::spawn`.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-12/src/main.rs:here}}
```

<span class="caption">Listing 20-12: Our ideal `ThreadPool` interface</span>

We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` has a
similar interface as `thread::spawn` in that it takes a closure the pool should
run for each stream. We need to implement `pool.execute` so it takes the
closure and gives it to a thread in the pool to run. This code won’t yet
compile, but we’ll try so the compiler can guide us in how to fix it.

#### Building the `ThreadPool` Struct Using Compiler Driven Development

Make the changes in Listing 20-12 to *src/main.rs*, and then let’s use the
compiler errors from `cargo check` to drive our development. Here is the first
error we get:

```console
{{#include ../listings/ch20-web-server/listing-20-12/output.txt}}
```

Great! This error tells us we need a `ThreadPool` type or module, so we’ll
build one now. Our `ThreadPool` implementation will be independent of the kind
of work our web server is doing. So, let’s switch the `hello` crate from a
binary crate to a library crate to hold our `ThreadPool` implementation. After
we change to a library crate, we could also use the separate thread pool
library for any work we want to do using a thread pool, not just for serving
web requests.

Create a *src/lib.rs* that contains the following, which is the simplest
definition of a `ThreadPool` struct that we can have for now:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. Doing so will make the library crate the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run`. After moving the *main.rs* file, edit it
to bring the library crate in and bring `ThreadPool` into scope by adding the
following code to the top of *src/bin/main.rs*:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/bin/main.rs:here}}
```

This code still won’t work, but let’s check it again to get the next error that
we need to address:

```console
{{#include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

This error indicates that next we need to create an associated function named
`new` for `ThreadPool`. We also know that `new` needs to have one parameter
that can accept `4` as an argument and should return a `ThreadPool` instance.
Let’s implement the simplest `new` function that will have those
characteristics:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

We chose `usize` as the type of the `size` parameter, because we know that a
negative number of threads doesn’t make any sense. We also know we’ll use this
4 as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the [“Integer Types”][integer-types]<!--
ignore --> section of Chapter 3.

Let’s check the code again:

```console
{{#include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

Now the error occurs because we don’t have an `execute` method on `ThreadPool`.
Recall from the [“Creating a Similar Interface for a Finite Number of
Threads”](#creating-a-similar-interface-for-a-finite-number-of-threads)<!--
ignore --> section that we decided our thread pool should have an interface
similar to `thread::spawn`. In addition, we’ll implement the `execute` function
so it takes the closure it’s given and gives it to an idle thread in the pool
to run.

We’ll define the `execute` method on `ThreadPool` to take a closure as a
parameter. Recall from the [“Storing Closures Using Generic Parameters and the
`Fn` Traits”][storing-closures-using-generic-parameters-and-the-fn-traits]<!--
ignore --> section in Chapter 13 that we can take closures as parameters with
three different traits: `Fn`, `FnMut`, and `FnOnce`. We need to decide which
kind of closure to use here. We know we’ll end up doing something similar to
the standard library `thread::spawn` implementation, so we can look at what
bounds the signature of `thread::spawn` has on its parameter. The documentation
shows us the following:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

The `F` type parameter is the one we’re concerned with here; the `T` type
parameter is related to the return value, and we’re not concerned with that. We
can see that `spawn` uses `FnOnce` as the trait bound on `F`. This is probably
what we want as well, because we’ll eventually pass the argument we get in
`execute` to `spawn`. We can be further confident that `FnOnce` is the trait we
want to use because the thread for running a request will only execute that
request’s closure one time, which matches the `Once` in `FnOnce`.

The `F` type parameter also has the trait bound `Send` and the lifetime bound
`'static`, which are useful in our situation: we need `Send` to transfer the
closure from one thread to another and `'static` because we don’t know how long
the thread will take to execute. Let’s create an `execute` method on
`ThreadPool` that will take a generic parameter of type `F` with these bounds:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

We still use the `()` after `FnOnce` because this `FnOnce` represents a closure
that takes no parameters and returns the unit type `()`. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.

Again, this is the simplest implementation of the `execute` method: it does
nothing, but we’re trying only to make our code compile. Let’s check it again:

```console
{{#include ../listings/ch20-web-server/no-listing-03-define-execute/output.txt}}
```

It compiles! But note that if you try `cargo run` and make a request in the
browser, you’ll see the errors in the browser that we saw at the beginning of
the chapter. Our library isn’t actually calling the closure passed to `execute`
yet!

> Note: A saying you might hear about languages with strict compilers, such as
> Haskell and Rust, is “if the code compiles, it works.” But this saying is not
> universally true. Our project compiles, but it does absolutely nothing! If we
> were building a real, complete project, this would be a good time to start
> writing unit tests to check that the code compiles *and* has the behavior we
> want.

#### Validating the Number of Threads in `new`

We aren’t doing anything with the parameters to `new` and `execute`. Let’s
implement the bodies of these functions with the behavior we want. To start,
let’s think about `new`. Earlier we chose an unsigned type for the `size`
parameter, because a pool with a negative number of threads makes no sense.
However, a pool with zero threads also makes no sense, yet zero is a perfectly
valid `usize`. We’ll add code to check that `size` is greater than zero before
we return a `ThreadPool` instance and have the program panic if it receives a
zero by using the `assert!` macro, as shown in Listing 20-13.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-13/src/lib.rs:here}}
```

<span class="caption">Listing 20-13: Implementing `ThreadPool::new` to panic if
`size` is zero</span>

We’ve added some documentation for our `ThreadPool` with doc comments. Note
that we followed good documentation practices by adding a section that calls
out the situations in which our function can panic, as discussed in Chapter 14.
Try running `cargo doc --open` and clicking the `ThreadPool` struct to see what
the generated docs for `new` look like!

Instead of adding the `assert!` macro as we’ve done here, we could make `new`
return a `Result` like we did with `Config::new` in the I/O project in Listing
12-9. But we’ve decided in this case that trying to create a thread pool
without any threads should be an unrecoverable error. If you’re feeling
ambitious, try to write a version of `new` with the following signature to
compare both versions:

```rust,ignore
pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### Creating Space to Store the Threads

Now that we have a way to know we have a valid number of threads to store in
the pool, we can create those threads and store them in the `ThreadPool` struct
before returning it. But how do we “store” a thread? Let’s take another look at
the `thread::spawn` signature:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

The `spawn` function returns a `JoinHandle<T>`, where `T` is the type that the
closure returns. Let’s try using `JoinHandle` too and see what happens. In our
case, the closures we’re passing to the thread pool will handle the connection
and not return anything, so `T` will be the unit type `()`.

The code in Listing 20-14 will compile but doesn’t create any threads yet.
We’ve changed the definition of `ThreadPool` to hold a vector of
`thread::JoinHandle<()>` instances, initialized the vector with a capacity of
`size`, set up a `for` loop that will run some code to create the threads, and
returned a `ThreadPool` instance containing them.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-14/src/lib.rs:here}}
```

<span class="caption">Listing 20-14: Creating a vector for `ThreadPool` to hold
the threads</span>

We’ve brought `std::thread` into scope in the library crate, because we’re
using `thread::JoinHandle` as the type of the items in the vector in
`ThreadPool`.

Once a valid size is received, our `ThreadPool` creates a new vector that can
hold `size` items. We haven’t used the `with_capacity` function in this book
yet, which performs the same task as `Vec::new` but with an important
difference: it preallocates space in the vector. Because we know we need to
store `size` elements in the vector, doing this allocation up front is slightly
more efficient than using `Vec::new`, which resizes itself as elements are
inserted.

When you run `cargo check` again, you’ll get a few more warnings, but it should
succeed.

#### A `Worker` Struct Responsible for Sending Code from the `ThreadPool` to a Thread

We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. Here, we’ll look at how we actually create threads. The standard
library provides `thread::spawn` as a way to create threads, and
`thread::spawn` expects to get some code the thread should run as soon as the
thread is created. However, in our case, we want to create the threads and have
them *wait* for code that we’ll send later. The standard library’s
implementation of threads doesn’t include any way to do that; we have to
implement it manually.

We’ll implement this behavior by introducing a new data structure between the
`ThreadPool` and the threads that will manage this new behavior. We’ll call
this data structure `Worker`, which is a common term in pooling
implementations. Think of people working in the kitchen at a restaurant: the
workers wait until orders come in from customers, and then they’re responsible
for taking those orders and filling them.

Instead of storing a vector of `JoinHandle<()>` instances in the thread pool,
we’ll store instances of the `Worker` struct. Each `Worker` will store a single
`JoinHandle<()>` instance. Then we’ll implement a method on `Worker` that will
take a closure of code to run and send it to the already running thread for
execution. We’ll also give each worker an `id` so we can distinguish between
the different workers in the pool when logging or debugging.

Let’s make the following changes to what happens when we create a `ThreadPool`.
We’ll implement the code that sends the closure to the thread after we have
`Worker` set up in this way:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`.
2. Change `ThreadPool` to hold a vector of `Worker` instances.
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the `id` and a thread spawned with an empty
   closure.
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create
   a new `Worker` with that `id`, and store the worker in the vector.

If you’re up for a challenge, try implementing these changes on your own before
looking at the code in Listing 20-15.

Ready? Here is Listing 20-15 with one way to make the preceding modifications.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-15/src/lib.rs:here}}
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
instance that is created by spawning a new thread using an empty closure.

This code will compile and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`. But we’re *still* not processing
the closure that we get in `execute`. Let’s look at how to do that next.

#### Sending Requests to Threads via Channels

Now we’ll tackle the problem that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method. But we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.

We want the `Worker` structs that we just created to fetch code to run from a
queue held in the `ThreadPool` and send that code to its thread to run.

In Chapter 16, you learned about *channels*—a simple way to communicate between
two threads—that would be perfect for this use case. We’ll use a channel to
function as the queue of jobs, and `execute` will send a job from the
`ThreadPool` to the `Worker` instances, which will send the job to its thread.
Here is the plan:

1. The `ThreadPool` will create a channel and hold on to the sending side of
   the channel.
2. Each `Worker` will hold on to the receiving side of the channel.
3. We’ll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute down the sending
   side of the channel.
5. In its thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.

Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. The `Job` struct
doesn’t hold anything for now but will be the type of item we’re sending down
the channel.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-16/src/lib.rs:here}}
```

<span class="caption">Listing 20-16: Modifying `ThreadPool` to store the
sending end of a channel that sends `Job` instances</span>

In `ThreadPool::new`, we create our new channel and have the pool hold the
sending end. This will successfully compile, still with warnings.

Let’s try passing a receiving end of the channel into each worker as the thread
pool creates the channel. We know we want to use the receiving end in the
thread that the workers spawn, so we’ll reference the `receiver` parameter in
the closure. The code in Listing 20-17 won’t quite compile yet.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-17/src/lib.rs:here}}
```

<span class="caption">Listing 20-17: Passing the receiving end of the channel
to the workers</span>

We’ve made some small and straightforward changes: we pass the receiving end of
the channel into `Worker::new`, and then we use it inside the closure.

When we try to check this code, we get this error:

```console
{{#include ../listings/ch20-web-server/listing-20-17/output.txt}}
```

The code is trying to pass `receiver` to multiple `Worker` instances. This
won’t work, as you’ll recall from Chapter 16: the channel implementation that
Rust provides is multiple *producer*, single *consumer*. This means we can’t
just clone the consuming end of the channel to fix this code. Even if we could,
that is not the technique we would want to use; instead, we want to distribute
the jobs across threads by sharing the single `receiver` among all the workers.

Additionally, taking a job off the channel queue involves mutating the
`receiver`, so the threads need a safe way to share and modify `receiver`;
otherwise, we might get race conditions (as covered in Chapter 16).

Recall the thread-safe smart pointers discussed in Chapter 16: to share
ownership across multiple threads and allow the threads to mutate the value, we
need to use `Arc<Mutex<T>>`. The `Arc` type will let multiple workers own the
receiver, and `Mutex` will ensure that only one worker gets a job from the
receiver at a time. Listing 20-18 shows the changes we need to make.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-18/src/lib.rs:here}}
```

<span class="caption">Listing 20-18: Sharing the receiving end of the channel
among the workers using `Arc` and `Mutex`</span>

In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.

With these changes, the code compiles! We’re getting there!

#### Implementing the `execute` Method

Let’s finally implement the `execute` method on `ThreadPool`. We’ll also change
`Job` from a struct to a type alias for a trait object that holds the type of
closure that `execute` receives. As discussed in the [“Creating Type Synonyms
with Type Aliases”][creating-type-synonyms-with-type-aliases]<!-- ignore -->
section of Chapter 19, type aliases allow us to make long types shorter. Look
at Listing 20-19.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-19/src/lib.rs:here}}
```

<span class="caption">Listing 20-19: Creating a `Job` type alias for a `Box`
that holds each closure and then sending the job down the channel</span>

After creating a new `Job` instance using the closure we get in `execute`, we
send that job down the sending end of the channel. We’re calling `unwrap` on
`send` for the case that sending fails. This might happen if, for example, we
stop all our threads from executing, meaning the receiving end has stopped
receiving new messages. At the moment, we can’t stop our threads from
executing: our threads continue executing as long as the pool exists. The
reason we use `unwrap` is that we know the failure case won’t happen, but the
compiler doesn’t know that.

But we’re not quite done yet! In the worker, our closure being passed to
`thread::spawn` still only *references* the receiving end of the channel.
Instead, we need the closure to loop forever, asking the receiving end of the
channel for a job and running the job when it gets one. Let’s make the change
shown in Listing 20-20 to `Worker::new`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-20/src/lib.rs:here}}
```

<span class="caption">Listing 20-20: Receiving and executing the jobs in the
worker’s thread</span>

Here, we first call `lock` on the `receiver` to acquire the mutex, and then we
call `unwrap` to panic on any errors. Acquiring a lock might fail if the mutex
is in a *poisoned* state, which can happen if some other thread panicked while
holding the lock rather than releasing the lock. In this situation, calling
`unwrap` to have this thread panic is the correct action to take. Feel free to
change this `unwrap` to an `expect` with an error message that is meaningful to
you.

If we get the lock on the mutex, we call `recv` to receive a `Job` from the
channel. A final `unwrap` moves past any errors here as well, which might occur
if the thread holding the sending side of the channel has shut down, similar to
how the `send` method returns `Err` if the receiving side shuts down.

The call to `recv` blocks, so if there is no job yet, the current thread will
wait until a job becomes available. The `Mutex<T>` ensures that only one
`Worker` thread at a time is trying to request a job.

With the implementation of this trick, our thread pool is in a working state!
Give it a `cargo run` and make some requests:

<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never read: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: field is never read: `id`
  --> src/lib.rs:48:5
   |
48 |     id: usize,
   |     ^^^^^^^^^

warning: field is never read: `thread`
  --> src/lib.rs:49:5
   |
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/main`
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

Success! We now have a thread pool that executes connections asynchronously.
There are never more than four threads created, so our system won’t get
overloaded if the server receives a lot of requests. If we make a request to
*/sleep*, the server will be able to serve other requests by having another
thread run them.

> Note: if you open */sleep* in multiple browser windows simultaneously, they
> might load one at a time in 5 second intervals. Some web browsers execute
> multiple instances of the same request sequentially for caching reasons. This
> limitation is not caused by our web server.

After learning about the `while let` loop in Chapter 18, you might be wondering
why we didn’t write the worker thread code as shown in Listing 20-21.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-21/src/lib.rs:here}}
```

<span class="caption">Listing 20-21: An alternative implementation of
`Worker::new` using `while let`</span>

This code compiles and runs but doesn’t result in the desired threading
behavior: a slow request will still cause other requests to wait to be
processed. The reason is somewhat subtle: the `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. At compile time, the borrow checker can then enforce the rule
that a resource guarded by a `Mutex` cannot be accessed unless we hold the
lock. But this implementation can also result in the lock being held longer
than intended if we don’t think carefully about the lifetime of the
`MutexGuard<T>`.

The code in Listing 20-20 that uses `let job =
receiver.lock().unwrap().recv().unwrap();` works because with `let`, any
temporary values used in the expression on the right hand side of the equals
sign are immediately dropped when the `let` statement ends. However, `while
let` (and `if let` and `match`) does not drop temporary values until the end of
the associated block. In Listing 20-21, the lock remains held for the duration
of the call to `job()`, meaning other workers cannot receive jobs.

[creating-type-synonyms-with-type-aliases]:
ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[storing-closures-using-generic-parameters-and-the-fn-traits]:
ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits
