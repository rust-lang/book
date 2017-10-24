## Using Threads to Run Code Simultaneously

In most operating systems today, an executed program’s code is run in a
*process*, and the operating system manages multiple process at once. Within
your program, you can also have independent parts that run simultaneously. The
feature that runs these independent parts is called *threads*.

<!-- I've tried to simplify the text above, can you check that I haven't
changed meaning? -->
<!-- Made some small tweaks, overall seems fine /Carol -->

Splitting the computation in your program up into multiple threads can improve
performance, since the program will be doing multiple things at the same time,
but it also adds complexity. Because threads may run simultaneously, there’s no
inherent guarantee about the order in which parts of your code on different
threads will run. This can lead to problems such as:

- Race conditions, where threads are accessing data or resources in an
  inconsistent order
- Deadlocks, where two threads are waiting for each other to finish using a
  resource the other thread has, which prevents both threads from continuing
- Bugs that only happen in certain situations and are hard to reproduce and
  fix reliably

<!-- How do threads prevent each other from continuing? Or is that something
we'll cover later?-->
<!-- We don't really get into that later, so I've expanded a bit here /Carol -->

Rust attempts to mitigate negative effects of using threads. Programming in a
multithreaded context still takes careful thought and requires a code structure
that’s different from programs that run in a single thread.

Programming languages implement threads in a few different ways. Many operating
systems provide an API for creating new threads. This model where a language
calls the operating system APIs to create threads is sometimes called *1:1*,
one OS thread per one language thread.

Many programming languages provide their own special implementation of threads.
Programming language-provided threads are known as *green* threads, and
languages that use these green threads will execute them in the context of a
different number of operating system threads. For this reason, the green
threaded model is called the *M:N* model, `M` green threads per `N` OS threads,
where `M` and `N` are not necessarily the same number.

Each model has its own advantages and tradeoffs, and the tradeoff most
important to Rust is runtime support. *Runtime* is a confusing term and can
have different meanings in different contexts.

<!-- Below - you mean this is the cause of runtime? Or "runtime" literally
means the code included by Rust in every binary? -->
<!-- Runtime literally means the code included by Rust in every binary.
Wikipedia calls this "runtime system":
https://en.wikipedia.org/wiki/Runtime_system but most people colloquially just
say "the runtime". I've tried to clarify. /Carol -->

In this context, by runtime we mean code that’s included by the language in
every binary. This code can be large or small depending on the language, but
every non-assembly language will have some amount of runtime code. For that
reason, colloquially when people say a language has “no runtime” they often
mean “small runtime.” Smaller runtimes have fewer features but have the
advantage of resulting in smaller binaries, which make it easier to combine the
language with other languages in more contexts. While many languages are okay
with increasing the runtime size in exchange for more features, Rust needs to
have nearly no runtime, and cannot compromise on being able to call into C in
order to maintain performance.

The green threading M:N model requires a larger language runtime to manage
threads. As such, the Rust standard library only provides an implementation of
1:1 threading. Because Rust is such a low-level language, there are crates that
implement M:N threading if you would rather trade overhead for aspects such as
more control over which threads run when and lower costs of context switching,
for example.

Now that we’ve defined threads in Rust, let’s explore how to use the
thread-related API provided by the standard library.

### Creating a New Thread with `spawn`

To create a new thread, we call the `thread::spawn` function, and pass it a
closure (we talked about closures in Chapter 13) containing the code we want to
run in the new thread. The example in Listing 16-1 prints some text from a main
thread and other text from a new thread:

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
while the main thread prints something else</span>

Note that with this function, the new thread will be stopped when the main
thread ends, whether it has finished running or not. The output from this
program might be a little different every time, but it will look similar to
this:

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

<!-- This seems interesting, how come the threads often take turns, but not
always? -->
<!-- I've added a bit of clarification /Carol -->

The threads will probably take turns, but that’s not guaranteed: it depends on
how your operating system schedules the threads. In this run, the main thread
printed first, even though the print statement from the spawned thread appears
first in the code. And even though we told the spawned thread to print until
`i` is 9, it only got to 5 before the main thread shut down.

If you run this code and only see one thread, or don’t see any overlap, try
increasing the numbers in the ranges to create more opportunities for a thread
to take a break and give the other thread a turn.

#### Waiting for All Threads to Finish Using `join` Handles

The code in Listing 16-1 not only stops the spawned thread prematurely most of
the time, because the main thread ends before the spawned thread is done,
there’s actually no guarantee that the spawned thread will get to run at all,
because there’s no guarantee on the order in which threads run!

<!-- Above -- why is this the case, because there are no guarantees over which
order the threads run in? -->
<!-- Yep! /Carol -->

We can fix this by saving the return value of `thread::spawn` in a variable.
The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned
value that, when we call the `join` method on it, will wait for its thread to
finish. Listing 16-2 shows how to use the `JoinHandle` of the thread we created
in Listing 16-1 and call `join` in order to make sure the spawned thread
finishes before the `main` exits:

<!-- Saving the return value where? I think this explanation of join handle
needs expanding, this feels cut short -->
<!-- In a variable. I've expanded a bit, but I'm not sure what information
seems missing, so I'm not sure if this is sufficient /Carol -->

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

Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. *Blocking* a thread means that
thread is prevented from performing work or exiting. Because we’ve put the call
to `join` after the main thread’s `for` loop, running this example should
produce output that looks something like this:

<!-- Liz: I've added a definition of "block" in the context of threads here,
which is the first time we used the term-- it seemed to cause some confusion
later on. /Carol -->

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

The main thread will wait for the spawned thread to finish and then run its
`for` loop, so the output won’t be interleaved anymore:

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

The `move` closure, which we didn’t cover in Chapter 13, is often used
alongside `thread::spawn`, as it allows us to use data from one thread in
another thread.

In Chapter 13, we said that “Creating closures that capture values from their
environment is mostly used in the context of starting new threads.”

<!-- PROD: DE to check this quote, see if it has changed -->

Now we’re creating new threads, so let’s talk about capturing values in
closures!

Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. In order to do so, the spawned thread’s closure must capture the
values it needs. Listing 16-3 shows an attempt to create a vector in the main
thread and use it in the spawned thread. However, this won’t yet work, as
you’ll see in a moment:

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
main thread in another thread</span>

The closure uses `v`, so will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread.

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

Rust *infers* how to capture `v`, and since `println!` only needs a reference
to `v`, the closure tries to borrow `v`. There’s a problem, though: Rust can’t
tell how long the spawned thread will run, so doesn’t know if the reference to
`v` will always be valid.

Let’s look at a scenario that’s more likely to have a reference to `v` that
won’t be valid, shown Listing 16-4:

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

If we run this code, there’s a possibility the spawned thread will be
immediately put in the background without getting a chance to run at all. The
spawned thread has a reference to `v` inside, but the main thread immediately
drops `v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!

To fix the problem in Listing 16-3, we can listen to the advice of the error
message:

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using, rather than allowing Rust to infer that it
should borrow. The modification to Listing 16-3 shown in Listing 16-5 will
compile and run as we intend:

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

<!-- Can you be more specific about the question we're asking about 16-4?-->
<!-- Done /Carol -->

What would happen to the code in Listing 16-4 where the main thread called
`drop` if we use a `move` closure? Would `move` fix that case? Nope, we get a
different error, because what Listing 16-4 is trying to do isn’t allowed for a
different reason! If we add `move` to the closure, we’d move `v` into the
closure’s environment, and we could no longer call `drop` on it in the main
thread. We would get this compiler error instead:

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

Rust’s ownership rules have saved us again! We got an error from the code in
Listing 16-3 because Rust was being conservative and only borrowing `v` for the
thread, which meant the main thread could theoretically invalidate the spawned
thread’s reference. By telling Rust to move ownership of `v` to the spawned
thread, we’re guaranteeing to Rust that the main thread won’t use `v` anymore.
If we change Listing 16-4 in the same way, we’re then violating the ownership
rules when we try to use `v` in the main thread. The `move` keyword overrides
Rust’s conservative default of borrowing; it doesn’t let us violate the
ownership rules.

<!-- Uh oh, I'm lost again, I thought we were trying to fix 16-4 with move, but
we don't want it to work, is that right? Can you talk about this a little?-->
<!-- I've tried to clarify a bit in the paragraph before this error and a bit
after the error /Carol -->

Now that we have a basic understanding of threads and the thread API, let’s
talk about what we can actually *do* with threads.
