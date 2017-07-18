## Using Threads to Run Code Simultaneously

In most operating systems in use today, when your program executes, the context
in which the operating system runs your code is called a *process*. The
operating system runs many processes, and the operating system managing these
processes is what lets multiple programs execute at the same time on your
computer.

We can take the idea of processes each running a program down one level of
abstraction: your program can also have independent parts that run
simultaneously within the context of your program. The feature that enables
this functionality is called *threads*.

Splitting up the computation your program needs to do into multiple threads can
improve performance, since the program will be doing multiple things at the
same time. Programming with threads can add complexity, however. Since threads
run simultaneously, there’s no inherent guarantee about the order in which the
parts of your code on different threads will run. This can lead to race
conditions where threads are accessing data or resources in an inconsistent
order, deadlocks where two threads both prevent each other from continuing, or
bugs that only happen in certain situations that are hard to reproduce
reliably. Rust lessens the effect of these and other downsides of using
threads, but programming in a multithreaded context still takes thought and
code structured differently than for programs only expected to run in a single
thread.

There are a few different ways that programming languages implement threads.
Many operating systems provide an API for creating new threads. In addition,
many programming languages provide their own special implementation of threads.
Programming language provided threads are sometimes called *lightweight* or
*green* threads. These languages take a number of green threads and execute
them in the context of a different number of operating system threads. For this
reason, the model where a language calls the operating system APIs to create
threads is sometimes called *1:1*, one OS thread per one language thread. The
green threaded model is called the *M:N* model, `M` green threads per `N` OS
threads, where `M` and `N` are not necessarily the same number.

Each model has its own advantages and tradeoffs. The tradeoff that’s most
important to Rust is runtime support. *Runtime* is a confusing term; it can
have different meaning in different contexts. Here, we mean some code included
by the language in every binary. For some languages, this code is large, and
for others, this code is small. Colloquially, “no runtime” is often what people
will say when they mean “small runtime”, since every non-assembly language has
some amount of runtime. Smaller runtimes have fewer features but have the
advantage of resulting in smaller binaries. Smaller binaries make it easier to
combine the language with other languages in more contexts. While many
languages are okay with increasing the runtime in exchange for more features,
Rust needs to have nearly no runtime, and cannot compromise on being able to
call into C in order to maintain performance.

The green threading model is a feature that requires a larger language runtime
in order to manage the threads. As such, the Rust standard library only
provides an implementation of 1:1 threading. Because Rust is such a low-level
language, there are crates that implement M:N threading if you would rather
trade overhead for aspects such as more control over which threads run when and
lower costs of context switching, for example.

Now that we’ve defined what threads are in Rust, let’s explore how to use the
thread-related API that the standard library provides for us.

### Creating a New Thread with `spawn`

To create a new thread, we call the `thread::spawn` function and pass it a
closure (we talked about closures in Chapter 13), containing the code we want
to run in the new thread. The example in Listing 16-1 prints some text from a
new thread and other text from the main thread:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

<span class="caption">Listing 16-1: Creating a new thread to print one thing
while the main thread is printing something else</span>

Note that the way this function is written, when the main thread ends, it will
stop the new thread too. The output from this program might be a little
different every time, but it will look similar to this:

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

The threads will probably take turns, but that’s not guaranteed. In this run,
the main thread printed first, even though the print statement from the spawned
thread appears first in the code we wrote. And even though we told the spawned
thread to print until `i` is 9, it only got to 5 before the main thread shut
down. If you always only see one thread, or if you don’t see any overlap, try
increasing the numbers in the ranges to create more opportunities for a thread
to take a break and give the other thread a turn.

#### Waiting for All Threads to Finish Using `join` Handles

Not only does the code in Listing 16-1 not allow the spawned thread to finish
most of the time since the main thread ends before the spawned thread is done,
there’s actually no guarantee that the spawned thread will get to run at all! We
can fix this by saving the return value of `thread::spawn`, which is a
`JoinHandle`. That looks like Listing 16-2:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join();
}
```

<span class="caption">Listing 16-2: Saving a `JoinHandle` from `thread::spawn`
to guarantee the thread is run to completion</span>

A `JoinHandle` is an owned value that can wait for a thread to finish, which is
what the `join` method does. By calling `join` on the handle, the current
thread will block until the thread that the handle represents terminates. Since
we’ve put the call to `join` after the main thread’s `for` loop, running this
example should produce output that looks something like this:

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

The two threads are still alternating, but the main thread waits because of the
call to `handle.join()` and does not end until the spawned thread is finished.

If we instead move `handle.join()` before the `for` loop in main, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    handle.join();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

The main thread will wait for the spawned thread to finish before the main
thread starts running its `for` loop, so the output won’t be interleaved
anymore:

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

Thinking about a small thing such as where to call `join` can affect whether
your threads are actually running at the same time or not.

### Using `move` Closures with Threads

There’s a feature of closures that we didn’t cover in Chapter 13 that’s often
useful with `thread::spawn`: `move` closures. We said this in Chapter 13:

> Creating closures that capture values from their environment is mostly used
> in the context of starting new threads.

Now we’re creating new threads, so let’s talk about capturing values in
closures!

Notice the closure that we pass to `thread::spawn` in Listing 16-1 takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. In order to use data in the spawned thread that comes from the
main thread, we need the spawned thread’s closure to capture the values it
needs. Listing 16-3 shows an attempt to create a vector in the main thread and
use it in the spawned thread, which won’t work the way this example is written:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 16-3: Attempting to use a vector created by the
main thread from another thread</span>

The closure uses `v`, so the closure will capture `v` and make `v` part of the
closure’s environment. Because `thread::spawn` runs this closure in a new
thread, we can access `v` inside that new thread.

When we compile this example, however, we’ll get the following error:

```text
error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
 -->
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

When we capture something in a closure’s environment, Rust will try to infer
how to capture it. `println!` only needs a reference to `v`, so the closure
tries to borrow `v`. There’s a problem, though: we don’t know how long the
spawned thread will run, so we don’t know if the reference to `v` will always
be valid.

Consider the code in Listing 16-4 that shows a scenario where it’s more likely
that the reference to `v` won’t be valid:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join();
}
```

<span class="caption">Listing 16-4: A thread with a closure that attempts to
capture a reference to `v` from a main thread that drops `v`</span>

This code could be run, and the spawned thread could immediately get put in the
background without getting a chance to run at all. The spawned thread has a
reference to `v` inside, but the main thread is still running: it immediately
drops `v`, using the `drop` function that we discussed in Chapter 15 that
explicitly drops its argument. Then, the spawned thread starts to execute. `v`
is now invalid, so a reference to it is also invalid. Oh no!

To fix this problem, we can listen to the advice of the error message:

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using, rather than inferring borrowing. This
modification to the code from Listing 16-3 shown in Listing 16-5 will compile
and run as we intend:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 16-5: Using the `move` keyword to force a closure
to take ownership of the values it uses</span>

What about the code in Listing 16-4 where the main thread called `drop`? If we
add `move` to the closure, we’ve moved `v` into the closure’s environment, and
we can no longer call `drop` on it. We get this compiler error instead:

```text
error[E0382]: use of moved value: `v`
  -->
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
```

Rust’s ownership rules have saved us again!

Now that we have a basic understanding of threads and the thread API, let’s
talk about what we can actually *do* with threads.
