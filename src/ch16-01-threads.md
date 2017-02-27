# Threads

When your program executes, the operating system runs it in the context of
something called a "process." This is, of course, operating system dependent,
but most operating systems today work this way. Your operating system runs
many processes, and this is what lets multiple programs execute at the same
time on your computer.

We can take this idea down one level of abstraction: what if your program could
also have independent bits of execution, and allow them to run simultaneously?
This feature is called *threads*.

There are a few different ways to implement threads, however. Many operating
systems provide an API for creating new threads. In addition, many programming
languages provide their own, special implementation of threads. These are
sometimes called "lightweight" or "green" threads. They then take multiple
green threads and execute them in the context of a different number of
operating system threads. For this reason, the operating system threads model
is sometimes called *1:1*, that is, one OS thread per thread. The green thread
model is called the *M:N* model, that is, `M` green threads per `N` OS threads.

Each model has their own advantages and tradeoffs. But there's one tradeoff
that's very important to Rust, and that's runtime support. The green threading
model requires a larger language runtime in order to manage the threads, and
often adds overhead when calling into C code. While many languages are okay
with this tradeoff, Rust needs to have nearly no runtime, and cannot compromise
on calling into C for performance reasons. As such, the Rust standard library
only provides an implementation of 1:1 threading. However, since Rust is such a
low-level language, you can add crates that give you M:N threading if you can
deal with these drawbacks.

Regardless, now that we've covered the basic concepts of threads, let's check
out the API that the standard library gives us.

## spawn

To create a new thread, we pass a closure to the `thread::spawn` function:

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    println!("Hello from the main thread!");
}
```

The code inside the closure will be executed in a new OS thread; when the
function ends, so does the thread.

### join handles

There's one trick though: if the main thread ends, the whole program ends. So
our example program above is not guaranteed to actually execute the new thread.
To do this, we can save the return value of `thread::spawn`:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    println!("Hello from the main thread!");

    handle.join();
}
```

By calling `join` on the handle, the current thread will block until the thread
that the handle represents terminates. So in this case, we might get the
messages in either order. But if we did this:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    handle.join();

    println!("Hello from the main thread!");
}
```

We would be guaranteed to see "Hello from a new thread!" before "Hello from the
main thread"; the call to `join` ensures that.

## move closures

There's a feature of closures that we didn't cover in Chapter 13 that's often
useful with `thread::spawn`: `move` closures. We said this back then:

> Creating closures that capture values from their environment is mostly used
> in the context of starting new threads.

Here we are!

If you notice, the closure that we pass to `thread::spawn` takes no arguments.
So how do we access data inside of our thread? The answer: capturing values.
Like this:

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}
```

Because we access `v` inside the closure, the closure will capture it into its
environment. And because `thread::spawn` runs this closure in a new thread, we
can access `v` inside that new thread.

There's a problem though: that code doesn't work! Here's the error:

```text
	error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> <anon>:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

When we capture something in a closure's environment, Rust will try to infer
how to capture it. In the case of `println!`, we only need a reference to `v`.
As such, it tries to borrow it, `&v`. But there's a problem: We don't know how
long the thread will run, and so we don't know if `&v` is always valid. That's
a problem. Consider this code:

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v);

    // oh no!

    handle.join();
}
```

Imagine that this code runs, and the thread gets put in the background. It has
a reference to `v` inside, but the main thread is still running: it immediately
drops `v`. Then, the thread starts to execute. `v` is now invalid, so a
reference to it is also invalid. Oh no!

In this case, we can listen to the advice of the error message:

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

By making this change, we tell the closure "I don't care what you infer, take
the entire environment by owner." And indeed, that works:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}
```

What about our bad code, with `drop`? If we have the `move`, well, we've moved
`v` into the closure's environment, and so we can't call `drop` on it. We get
this error:

```text
error[E0382]: use of moved value: `v`
  --> <anon>:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v);
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
```

Rust has saved us again!

Now that we have a basic understanding of threads and the thread API, let's
talk about what you can actually _do_ with threads.
