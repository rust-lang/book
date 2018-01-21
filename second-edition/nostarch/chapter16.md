
[TOC]

# Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust’s
major goals. *Concurrent programming*, where different parts of a program
execute independently, and *parallel programming*, where different parts of a
program are executing at the same time, are becoming increasingly important as
more computers have multiple processors to take advantage of. Historically,
programming in these contexts has been difficult and error prone: Rust hopes to
change that.

Initially, the Rust team thought that ensuring memory safety and preventing
concurrency problems were two separate challenges to be solved with different
methods. Over time, they discovered that the ownership and type systems are a
powerful set of tools to help in dealing with both memory safety *and*
concurrency problems! By leveraging ownership and type checking, many
concurrency errors are *compile time* errors in Rust, rather than runtime
errors. Rather than spending lots of time trying to reproduce the exact
circumstances under which a runtime concurrency bug occurs, incorrect code will
refuse to compile with an error explaining the problem. This lets you fix your
code while you’re working on it, rather than potentially after it’s been
shipped to production. We’ve nicknamed this aspect of Rust *fearless
concurrency*. Fearless concurrency allows you to write code that’s free of
subtle bugs and is easy to refactor without introducing new bugs.

> Note: we’ll be referring to many of the problems here as *concurrent* rather
> than being more precise by saying *concurrent and/or parallel*, for
> simplicity’s sake. If this were a book specifically about concurrency and/or
> parallelism, we’d be sure to be more specific. For this chapter, please
> mentally substitute *concurrent and/or parallel* whenever we say *concurrent*.

Many languages are strongly opinionated about the solutions they offer for
dealing with concurrent problems. For example, Erlang has elegant functionality
for message passing concurrency, but only obscure ways to share state between
threads. Only supporting a subset of possible solutions is a reasonable
strategy for higher-level languages to take, because a higher-level language
promises benefits from giving up some control in order to gain abstractions.
However, lower-level languages are expected to provide the solution with the
best performance in any given situation, and have fewer abstractions over the
hardware. Rust, therefore, gives us a variety of tools for modeling your
problems in whatever way is appropriate for your situation and requirements.

Here’s what we’ll cover in this chapter:

* How to create threads to run multiple pieces of code at the same time
* *Message passing* concurrency, where channels are used to send messages
  between threads.
* *Shared state* concurrency, where multiple threads have access to some piece
  of data.
* The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to
  user-defined types as well as types provided by the standard library.

## Using Threads to Run Code Simultaneously

In most operating systems today, an executed program’s code is run in a
*process*, and the operating system manages multiple process at once. Within
your program, you can also have independent parts that run simultaneously. The
feature that runs these independent parts is called *threads*.

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

Filename: src/main.rs

```
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

Listing 16-1: Creating a new thread to print one thing while the main thread
prints something else

Note that with this function, the new thread will be stopped when the main
thread ends, whether it has finished running or not. The output from this
program might be a little different every time, but it will look similar to
this:

```
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

We can fix this by saving the return value of `thread::spawn` in a variable.
The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned
value that, when we call the `join` method on it, will wait for its thread to
finish. Listing 16-2 shows how to use the `JoinHandle` of the thread we created
in Listing 16-1 and call `join` in order to make sure the spawned thread
finishes before the `main` exits:

Filename: src/main.rs

```
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

Listing 16-2: Saving a `JoinHandle` from `thread::spawn` to guarantee the
thread is run to completion

Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. *Blocking* a thread means that
thread is prevented from performing work or exiting. Because we’ve put the call
to `join` after the main thread’s `for` loop, running this example should
produce output that looks something like this:

```
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

Filename: src/main.rs

```
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

```
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

Now we’re creating new threads, so let’s talk about capturing values in
closures!

Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. In order to do so, the spawned thread’s closure must capture the
values it needs. Listing 16-3 shows an attempt to create a vector in the main
thread and use it in the spawned thread. However, this won’t yet work, as
you’ll see in a moment:

Filename: src/main.rs

```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}
```

Listing 16-3: Attempting to use a vector created by the main thread in another
thread

The closure uses `v`, so will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread.

When we compile this example, however, we’ll get the following error:

```
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

Filename: src/main.rs

```
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

Listing 16-4: A thread with a closure that attempts to capture a reference to
`v` from a main thread that drops `v`

If we run this code, there’s a possibility the spawned thread will be
immediately put in the background without getting a chance to run at all. The
spawned thread has a reference to `v` inside, but the main thread immediately
drops `v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!

To fix the problem in Listing 16-3, we can listen to the advice of the error
message:

```
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using, rather than allowing Rust to infer that it
should borrow. The modification to Listing 16-3 shown in Listing 16-5 will
compile and run as we intend:

Filename: src/main.rs

```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}
```

Listing 16-5: Using the `move` keyword to force a closure to take ownership of
the values it uses

What would happen to the code in Listing 16-4 where the main thread called
`drop` if we use a `move` closure? Would `move` fix that case? Nope, we get a
different error, because what Listing 16-4 is trying to do isn’t allowed for a
different reason! If we add `move` to the closure, we’d move `v` into the
closure’s environment, and we could no longer call `drop` on it in the main
thread. We would get this compiler error instead:

```
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

Now that we have a basic understanding of threads and the thread API, let’s
talk about what we can actually *do* with threads.

## Message Passing to Transfer Data Between Threads

One increasingly popular approach to ensuring safe concurrency is *message
passing*, where threads or actors communicate by sending each other messages
containing data. Here’s the idea in slogan form from the Go language
documentation:

> Do not communicate by sharing memory; instead, share memory by
> communicating.
>
> --Effective Go at *http://golang.org/doc/effective_go.html*

One major tool Rust has for accomplishing message sending concurrency is the
*channel*, a programming concept that Rust’s standard library provides an
implementation of. You can imagine a channel in programming like a channel of
water, such as a stream or a river. If you put something like a rubber duck or
a boat into a stream, it will travel downstream to the end of the river.

A channel in programming has two halves: a transmitter and a receiver. The
transmitter half is like the upstream location where we put rubber ducks into
the river, and the receiver half is the downstream place where the rubber duck
ends up. One part of our code calls methods on the transmitter with the data we
want to send, and another part checks the receiving end for arriving messages.

Here we’ll work up to a program that has one thread to generate values and send
them down a channel, and another thread that will receive the values and print
them out. We’re going to be sending simple values between threads using a
channel for the purposes of illustration. Once you’re familiar with the
technique, you could use channels to implement a chat system, or a system where
many threads perform parts of a calculation and send the parts to one thread
that aggregates the results.

First, we’ll create a channel but not do anything with it in Listing 16-6:

Filename: src/main.rs

```
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

Listing 16-6: Creating a channel and assigning the two halves to `tx` and `rx`

We create a new channel using the `mpsc::channel` function; `mpsc` stands for
*multiple producer, single consumer*. In short, the way Rust’s standard library
has implemented channels is such that a channel can have multiple *sending*
ends that produce values, but only one *receiving* end that consumes those
values. Imagine multiple rivers and streams flowing together into one big
river: everything sent down any of the streams will end up in one river at the
end. We’re going to start with a single producer for now, but we’ll add
multiple producers once we get this example working.

The `mpsc::channel` function returns a tuple, the first element of which is the
sending end and the second element the receiving end. The abbreviations `tx`
and `rx` are traditionally used in many fields for *transmitter* and *receiver*
respectively, so we give our variables those names to indicate each end. We’re
using a `let` statement with a pattern that destructures the tuples; we’ll be
discussing the use of patterns in `let` statements and destructuring in Chapter
18. Using a `let` statement in this way is a convenient way to extract the
pieces of the tuple returned by `mpsc::channel`.

Let’s move the transmitting end into a spawned thread and have it send one
string so that the spawned thread is communicating with the main thread, shown
in Listing 16-7. This is like putting a rubber duck in the river upstream or
sending a chat message from one thread to another:

Filename: src/main.rs

```
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

Listing 16-7: Moving `tx` to a spawned thread and sending “hi”

We’re again using `thread::spawn` to create a new thread, and then use `move`
to move `tx` into the closure so the spawned thread owns `tx`. The spawned
thread needs to own the transmitting end of the channel in order to be able to
send messages through the channel.

The transmitting end has a `send` method that takes the value we want to send.
The `send` method returns a `Result<T, E>` type, so that if the receiving end
has already been dropped and there’s nowhere to send a value, the send
operation will error. In this example, we’re simply calling `unwrap` to panic
in case of error, but for a real application, we’d handle it properly--return
to Chapter 9 to review strategies for proper error handling.

In Listing 16-8, we’ll get the value from the receiving end of the channel in
the main thread. This is like retrieving the rubber duck from the water at the
end of the river, or like getting a chat message:

Filename: src/main.rs

```
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

Listing 16-8: Receiving the value “hi” in the main thread and printing it out

The receiving end of a channel has two useful methods: `recv` and `try_recv`.
We’re using `recv`, short for *receive*, which will block the main thread’s
execution and wait until a value is sent down the channel. Once a value is
sent, `recv` will return it in a `Result<T, E>`. When the sending end of the
channel closes, `recv` will return an error to signal that no more values will
be coming.

The `try_recv` method doesn’t block, but will instead return a `Result<T, E>`
immediately: an `Ok` value holding a message if one is available, and an `Err`
value if there aren’t any messages this time. Using `try_recv` is useful if
this thread has other work to do while waiting for messages: we could write a
loop that calls `try_recv` every so often, handles a message if one is
available, and otherwise does other work for a little while until checking
again.

We’ve chosen to use `recv` in this example for simplicity; we don’t have any
other work for the main thread to do other than wait for messages, so blocking
the main thread is appropriate.

If we run the code in Listing 16-8, we’ll see the value printed out from the
main thread:

```
Got: hi
```

Perfect!

### Channels and Ownership Transference

The ownership rules play a vital role in message sending as far as helping us
write safe, concurrent code. Preventing errors in concurrent programming is the
advantage we get by making the tradeoff of having to think about ownership
throughout our Rust programs. Let’s do an experiment to show how channels and
ownership work together to prevent problems: we’ll try to use a `val` value in
the spawned thread *after* we’ve sent it down the channel. Try compiling the
code in Listing 16-9:

Filename: src/main.rs

```
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

Listing 16-9: Attempting to use `val` after we have sent it down the channel

Here, we try to print out `val` after we’ve sent it down the channel via
`tx.send`. Allowing this would be a bad idea: once the value has been sent to
another thread, that thread could modify or drop it before we try to use the
value again, which would potentially cause errors or unexpected results due to
inconsistent or nonexistent data.

However, Rust gives us an error if we try to compile this code:

```
error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
   not implement the `Copy` trait
```

Our concurrency mistake has caused a compile-time error! The `send` function
takes ownership of its parameter, and when the value is moved the receiver
takes ownership of it. This stops us from accidentally use the value again
after sending it; the ownership system checks that everything is okay.

### Sending Multiple Values and Seeing the Receiver Waiting

The code in Listing 16-8 compiled and ran, but doesn’t show us very clearly
that two separate threads are talking to each other over the channel. In
Listing 16-10 we’ve made some modifications that will prove this code is
running concurrently: the spawned thread will now send multiple messages and
pause for a second between each message.

Filename: src/main.rs

```
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

Listing 16-10: Sending multiple messages and pausing between each one

This time, the spawned thread has a vector of strings that we want to send to
the main thread. We iterate over them, sending each individually, and pause
between each by calling the `thread::sleep` function with a `Duration` value of
one second.

In the main thread, we’re not calling the `recv` function explicitly anymore:
instead we’re treating `rx` as an iterator. For each value received, we’re
printing it out. When the channel is closed, iteration will end.

When running the code in Listing 16-10, you should see the following output,
with a one second pause in between each line:

```
Got: hi
Got: from
Got: the
Got: thread
```

Because we don’t have any code that pauses or delays in the `for` loop in the
main thread, we can tell that the main thread is waiting to receive values from
the spawned thread.

### Creating Multiple Producers by Cloning the Transmitter

Near the start of this section, we mentioned that `mpsc` stood for *multiple
producer, single consumer*. Let’s put that ability to use and expand the code
from Listing 16-10 to create multiple threads that all send values to the same
receiver. We can do that by cloning the transmitting half of the channel, as
shown in Listing 16-11:

Filename: src/main.rs

```
// --snip--
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
// --snip--
```

Listing 16-11: Sending multiple messages and pausing between each one

This time, before we create the first spawned thread, we call `clone` on the
sending end of the channel. This will give us a new sending handle we can pass
to the first spawned thread. We pass the original sending end of the channel to
a second spawned thread. This gives us two threads, each sending different
messages to the receiving end of the channel.

If you run this, you’ll *probably* see output like this:

```
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

You might see the values in a different order, it depends on your system! This
is what makes concurrency interesting as well as difficult. If you play around
with `thread::sleep`, giving it different values in the different threads, each
run will be more non-deterministic and create different output each time.

Now that we’ve seen how channels work, let’s look at a different method of
concurrency.

## Shared State Concurrency

Message passing is a fine way of dealing with concurrency, but it’s not the
only one. Consider this slogan again:

> Do not communicate by sharing memory; instead, share memory by
> communicating.

What would “communicate by sharing memory” look like? And moreover, why would
message passing enthusiasts choose not to use it and do the opposite instead?

In a way, channels in any programming language are sort of like single
ownership, because once you transfer a value down a channel, you shouldn’t use
that value any longer. Shared memory concurrency is sort of like multiple
ownership: multiple threads can access the same memory location at the same
time. As we saw in Chapter 15 where multiple ownership was made possible by
smart pointers, multiple ownership can add additional complexity because these
different owners need managing.

Rust’s type system and ownership rules assist a lot in getting this management
correct, though. For an example, let’s look at one of the more common
concurrency primitives for shared memory: mutexes.

### Mutexes Allow Access to Data from One Thread at a Time

A *mutex* is a concurrency primitive for sharing memory. It’s short for “mutual
exclusion”, as in, it only allows one thread to access some data at any given
time. In order to access the data in a mutex, a thread must first signal that
it wants access by asking to acquire the mutex’s *lock*. The lock is a data
structure that is part of the mutex that keeps track of who currently has
exclusive access to the data. We therefore describe the mutex as *guarding* the
data it holds via the locking system.

Mutexes have a reputation for being hard to use because there are some
rules you have to remember:

1. You must attempt to acquire the lock before using the data.
2. Once you’re done with the data that’s guarded by the mutex, you must unlock
   the data so other threads can acquire the lock.

For a real-world metaphor of a mutex, imagine a panel discussion at a
conference with only one microphone. Before a panelist may speak, they have to
ask or signal that they would like to use the microphone. Once they get the
microphone, they may talk for as long as they would like, then hand the
microphone to the next panelist who requests to speak. If a panelist forgets to
hand the microphone off when they’re finished with it, no one else is able to
speak. If management of the shared microphone goes wrong, the panel would not
work as planned!

Management of mutexes can be incredibly tricky to get right, and that’s why so
many people are enthusiastic about channels. However, thanks to Rust’s type
system and ownership rules, we can’t get locking and unlocking wrong.

#### The API of `Mutex<T>`

Let’s start simply with an example of using a mutex in a single-threaded
context, shown in Listing 16-12:

Filename: src/main.rs

```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

Listing 16-12: Exploring the API of `Mutex<T>` in a single threaded context for
simplicity

As with many types, we create a `Mutex<T>` using the associated function `new`.
To access the data inside the mutex, we use the `lock` method to acquire the
lock. This call will block the current thread so that it can’t do any work
until it’s our turn to have the lock.

The call to `lock` would fail if another thread holding the lock panicked. In
that case, no one would ever be able to get the lock, so we’ve chosen to
`unwrap` and have this thread panic if we’re in that situation.

Once we’ve acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system ensures
that we acquire a lock before using this value: `Mutex<i32>` is not an `i32`,
so we *must* acquire the lock in order to be able to use the `i32` value. We
can’t forget; the type system won’t let us do it otherwise.

As you may suspect, `Mutex<T>` is a smart pointer. More accurately, the call to
`lock` *returns* a smart pointer called `MutexGuard`. This smart pointer
implements `Deref` to point at our inner data, and also has a `Drop`
implementation that releases the lock automatically when `MutexGuard` goes out
of scope, which happens at the end of the inner scope in Listing 16-12. This
way, we don’t risk forgetting to release the lock and blocking it from use by
other threads, because it happens automatically.

After dropping the lock, we can print out the mutex value and see that we were
able to change the inner `i32` to 6.

#### Sharing a `Mutex<T>` Between Multiple Threads

Let’s now try to share a value between multiple threads using `Mutex<T>`. We’ll
spin up ten threads, and have them each increment a counter value by 1 so that
the counter goes from 0 to 10. Note that the next few examples will have
compiler errors, and we’re going to use those errors to learn more about using
`Mutex<T>` and how Rust helps us use it correctly. Listing 16-13 has our
starting example:

Filename: src/main.rs

```
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Listing 16-13: Ten threads each increment a counter guarded by a `Mutex<T>`

We’re creating a `counter` variable to hold an `i32` inside a `Mutex<T>`, like
we did in Listing 16-12. Next, we’re creating 10 threads by mapping over a
range of numbers. We use `thread::spawn` and give all the threads the same
closure, one that moves the counter into the thread, acquires a lock on the
`Mutex<T>` by calling the `lock` method, and then adds 1 to the value in the
mutex. When a thread finishes running its closure, `num` will go out of scope
and release the lock so another thread can acquire it.

In the main thread, we collect all the join handles like we did in Listing
16-2, and then call `join` on each to make sure all the threads finish. At that
point, the main thread will acquire the lock and print out the result of this
program.

We hinted that this example won’t compile, now let’s find out why!

```
error[E0382]: capture of moved value: `counter`
  -->
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  -->
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

The error message is saying that the `counter` value is moved into the closure,
then is captured when we call `lock`. That sounds like what we wanted, but it’s
not allowed!

Let’s reason this out by simplifying the program. Instead of making 10 threads
in a `for` loop, let’s just make two threads without a loop and see what
happens then. Replace the first `for` loop in Listing 16-13 with this code
instead:

```
let handle = thread::spawn(move || {
    let mut num = counter.lock().unwrap();

    *num += 1;
});
handles.push(handle);

let handle2 = thread::spawn(move || {
    let mut num2 = counter.lock().unwrap();

    *num2 += 1;
});
handles.push(handle2);
```

We make two threads and change the variable names used with the second thread
to `handle2` and `num2`. When we run this time, compiling gives us:

```
error[E0382]: capture of moved value: `counter`
  -->
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  -->
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

Aha! The first error message tells us that `counter` is moved into the closure
for the thread associated with `handle`. That move is preventing us from
capturing `counter` when we try to call `lock` on it and store the result in
`num2` in the second thread! So Rust is telling us that we can’t move ownership
of `counter` into multiple threads. This was hard to see before because our
threads were in a loop, and Rust can’t point to different threads in different
iterations of the loop. Let’s try to fix this with a multiple-ownership method
we saw in Chapter 15.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we were able to give a value multiple owners by using the smart
pointer `Rc<T>` to create a reference-counted value. Let’s try to do the same
here and see what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing
16-14, and clone the `Rc<T>` before moving ownership to the thread. Now we’ve
seen the errors, we’ll also switch back to using the `for` loop, and we’ll keep
the `move` keyword with the closure:

Filename: src/main.rs

```
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Listing 16-14: Attempting to use `Rc<T>` to allow multiple threads to own the
`Mutex<T>`

Once again, we compile and get... different errors! The compiler is teaching us
a lot!

```
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied
  -->
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ the trait `std::marker::Send` is not
   implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   |
   = note: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads
   safely
   = note: required because it appears within the type
   `[closure@src/main.rs:11:36: 15:10
   counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
```

Wow, that’s quite wordy! Here are some important parts to pick out: the first
note says `Rc<Mutex<i32>> cannot be sent between threads safely`. The reason
for this is in the error message, which, once distilled, says `the trait bound
Send is not satisfied`. We’re going to talk about `Send` in the next section;
it’s one of the traits that ensures the types we use with threads are meant for
use in concurrent situations.

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped, but it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts: subtle bugs
that could in turn lead to memory leaks or a value being dropped before we’re
done with it. What we need is a type exactly like `Rc<T>`, but that makes
changes to the reference count in a thread-safe way.

#### Atomic Reference Counting with `Arc<T>`

Luckily for us, there *is* a type like `Rc<T>` that’s safe to use in concurrent
situations: `Arc<T>`. The ‘a’ stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here; see the standard library
documentation for `std::sync::atomic` for more details. What you need to know
here is that atomics work like primitive types, but are safe to share across
threads.

You might then wonder why all primitive types aren’t atomic, and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re only doing operations on values within a single
thread, your code can run faster if it doesn’t have to enforce the guarantees
atomics provide.

Back to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix our
program by changing the `use` line and the call to `new`. The code in Listing
16-15 will finally compile and run:

Filename: src/main.rs

```
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>` to be able to share
ownership across multiple threads

This will print:

```
Result: 10
```

We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety! This structure could
also be used to do more complicated operations than just incrementing a
counter: these methods allow us to divide calculations up into independent
parts, which we could split across threads, and then we can use a `Mutex<T>` to
have each thread update the final result with its part.

### Similarities between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

You may have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, like the `Cell` family does. In the same way we used `RefCell<T>`
in Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use
`Mutex<T>` to mutate contents inside of an `Arc<T>`.

Another thing to note is that Rust can’t prevent us from all kinds of logic
errors when using `Mutex<T>`. Recall from Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes the risk of
*deadlocks*. These occur when an operation needs to lock two resources and two
threads have each acquired one of the locks, causing them to wait for each
other forever. If you’re interested in this topic, try creating a Rust program
that has a deadlock, then research deadlock mitigation strategies for mutexes
in any language, and have a go at implementing them in Rust. The standard
library API documentation for `Mutex<T>` and `MutexGuard` will have useful
information.

Let’s round out this chapter by talking about the `Send` and `Sync` traits and
how we could use them with custom types.

## Extensible Concurrency with the `Sync` and `Send` Traits

Interestingly, the Rust language itself knows *very* little about concurrency.
Almost everything we’ve talked about so far in this chapter has been part of
the standard library, not the language. Our concurrency options are not limited
to the language or the standard library, meaning we can write our own
concurrency options or use ones others have written.

There *are* two concurrency concepts embedded in the language, however: the
`std::marker` traits `Sync` and `Send`.

### Allowing Transference of Ownership Between Threads with `Send`

The `Send` marker trait indicates that ownership of the type implementing
`Send` may be transferred between threads. Almost every Rust type is `Send`,
but there are some exceptions, including `Rc<T>`: this cannot be `Send` because
if we cloned an `Rc<T>` value and tried to transfer ownership of the clone to
another thread, both threads might update the reference count at the same time.
For this reason, `Rc<T>` is implemented for use in single-threaded situations
where you don’t want to pay the threadsafe performance penalty.

In this way Rust’s type system and trait bounds ensure we can never
accidentally send an `Rc<T>` value across threads unsafely. When we tried to do
this in Listing 16-14, we got an error that said `the trait Send is not
implemented for Rc<Mutex<i32>>`. When we switched to `Arc<T>`, which is `Send`,
the code compiled.

Any type composed entirely of `Send` types is automatically marked as `Send` as
well. Almost all primitive types are `Send`, aside from raw pointers, which
we’ll discuss in Chapter 19.

### Allowing Access from Multiple Threads with `Sync`

The `Sync` marker trait indicates that it is safe for the type implementing
`Sync` to be referenced from multiple threads. Another way to say this is that
any type `T` is `Sync` if `&T` (a reference to `T`) is `Send`, meaning the
reference can be sent safely to another thread. In a similar manner as `Send`,
primitive types are `Sync` and types composed entirely of types that are `Sync`
are also `Sync`.

`Rc<T>` is also not `Sync`, for the same reasons that it’s not `Send`.
`RefCell<T>` (which we talked about in Chapter 15) and the family of related
`Cell<T>` types are not `Sync`. The implementation of borrow checking that
`RefCell<T>` does at runtime is not threadsafe. `Mutex<T>` is `Sync`, and can
be used to share access with multiple threads as we saw in the previous section.

### Implementing `Send` and `Sync` Manually is Unsafe

Because types that are made up of `Send` and `Sync` traits are automatically
also `Send` and `Sync`, we don’t have to implement those traits ourselves. As
marker traits, they don’t even have any methods to implement. They’re just
useful for enforcing concurrency-related invariants.

Manually implementing these traits involves implementing unsafe Rust code.
We’re going to be talking about using unsafe Rust code in Chapter 19; for now,
the important information is that building new concurrent types not made up of
`Send` and `Sync` parts requires careful thought, in order to uphold the safety
guarantees. The Nomicon at *https://doc.rust-lang.org/stable/nomicon/*
has more information about these guarantees and how to uphold them.

## Summary

This isn’t the last we’ll see of concurrency in this book; the project in
Chapter 20 will use these concepts in a more realistic situation than the
smaller examples discussed here.

As we mentioned, since very little of how Rust deals with concurrency is part
of the language, many concurrency solutions are implemented as crates. These
evolve more quickly than the standard library; search online for the current
state-of-the-art crates to use in multithreaded situations.

Rust provides channels for message passing and smart pointer types like
`Mutex<T>` and `Arc<T>` that are safe to use in concurrent contexts. The type
system and the borrow checker will make sure the code using these solutions
won’t end up with data races or invalid references. Once we get our code
compiling, we can rest assured that it will happily run on multiple threads
without the kinds of hard-to-track-down bugs common in other languages.
Concurrent programming is no longer something to be afraid of: go forth and
make your programs concurrent, fearlessly!

Next, let’s talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger, and how Rust’s idioms relate to those you
might be familiar with from Object Oriented Programming.
