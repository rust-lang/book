
[TOC]

# Fearless Concurrency

Ensuring memory safety isn’t Rust’s only goal: being a language that is better
equipped to handle concurrent and parallel programming has always been another
major goal of Rust. *Concurrent programming*, where different parts of a
program execute independently, and *parallel programming*, where different
parts of a program are executing at the same time, are becoming more important
as more computers have multiple processors for our programs to take advantage
of. Historically, programming in these contexts has been difficult and error
prone: Rust hopes to change that.

Originally, we thought that memory safety and preventing concurrency problems
were two separate challenges to be solved with different methods. However, over
time, we discovered that ownership and the type system are a powerful set of
tools that help in dealing with both memory safety *and* concurrency problems!
By leveraging ownership and type checking, many concurrency errors are *compile
time* errors in Rust, rather than runtime errors. We’ve nicknamed this aspect
of Rust *fearless concurrency*. Fearless concurrency means Rust not only allows
you to have confidence that your code is free of subtle bugs, but also lets you
refactor this kind of code easily without worrying about introducing new bugs.

> Note: given that Rust’s slogan is *fearless concurrency*, we’ll be referring
> to many of the problems here as *concurrent* rather than being more precise
> by saying *concurrent and/or parallel*, for simplicity’s sake. If this were a
> book specifically about concurrency and/or parallelism, we’d be sure to be
> more specific. For this chapter, please mentally substitute
> *concurrent and/or parallel* whenever we say *concurrent*.

Many languages are strongly opinionated about the solutions they offer you to
deal with concurrent problems. That’s a very reasonable strategy, especially
for higher-level languages, but lower-level languages don’t have that luxury.
Lower-level languages are expected to enable whichever solution would provide
the best performance in a given situation, and they have fewer abstractions
over the hardware. Rust, therefore, gives us a variety of tools for modeling
our problems in whatever way is appropriate for our situation and requirements.

Here’s what we’ll cover in this chapter:

* How to create threads to run multiple pieces of code at the same time
* *Message passing* concurrency, where channels are used to send messages
  between threads.
* *Shared state* concurrency, where multiple threads have access to some piece
  of data.
* The `Sync` and `Send` traits, which allow Rust’s concurrency guarantees to be
  extended to user-defined types as well as types provided by the standard
  library.

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

Listing 16-1: Creating a new thread to print one thing while the main thread is
printing something else

Note that the way this function is written, when the main thread ends, it will
stop the new thread too. The output from this program might be a little
different every time, but it will look similar to this:

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
there’s actualy no guarantee that the spawned thread will get to run at all! We
can fix this by saving the return value of `thread::spawn`, which is a
`JoinHandle`. That looks like Listing 16-2:

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

A `JoinHandle` is an owned value that can wait for a thread to finish, which is
what the `join` method does. By calling `join` on the handle, the current
thread will block until the thread that the handle represents terminates. Since
we’ve put the call to `join` after the main thread’s `for` loop, running this
example should produce output that looks something like this:

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

The main thread will wait for the spawned thread to finish before the main
thread starts running its `for` loop, so the output won’t be interleaved
anymore:

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

Listing 16-3: Attempting to use a vector created by the main thread from
another thread

The closure uses `v`, so the closure will capture `v` and make `v` part of the
closure’s environment. Because `thread::spawn` runs this closure in a new
thread, we can access `v` inside that new thread.

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

When we capture something in a closure’s environment, Rust will try to infer
how to capture it. `println!` only needs a reference to `v`, so the closure
tries to borrow `v`. There’s a problem, though: we don’t know how long the
spawned thread will run, so we don’t know if the reference to `v` will always
be valid.

Consider the code in Listing 16-4 that shows a scenario where it’s more likely
that the reference to `v` won’t be valid:

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

This code could be run, and the spawned thread could immediately get put in the
background without getting a chance to run at all. The spawned thread has a
reference to `v` inside, but the main thread is still running: it immediately
drops `v`, using the `drop` function that we discussed in Chapter 15 that
explicitly drops its argument. Then, the spawned thread starts to execute. `v`
is now invalid, so a reference to it is also invalid. Oh no!

To fix this problem, we can listen to the advice of the error message:

```
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using, rather than inferring borrowing. This
modification to the code from Listing 16-3 shown in Listing 16-5 will compile
and run as we intend:

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

What about the code in Listing 16-4 where the main thread called `drop`? If we
add `move` to the closure, we’ve moved `v` into the closure’s environment, and
we can no longer call `drop` on it. We get this compiler error instead:

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

Rust’s ownership rules have saved us again!

Now that we have a basic understanding of threads and the thread API, let’s
talk about what we can actually *do* with threads.

## Message Passing to Transfer Data Between Threads

One approach to concurrency that’s seen a rise in popularity as of late is
*message passing*, where threads or actors communicate by sending each other
messages containing data. Here’s the idea in slogan form:

> Do not communicate by sharing memory; instead, share memory by
> communicating.
>
> --Effective Go at *http://golang.org/doc/effective_go.html*

A major tool to accomplish this goal is the *channel*. A channel has two
halves, a transmitter and a receiver. One part of our code can call methods on
the transmitter with the data we want to send, and another part can check the
receiving end for arriving messages.

We’re going to work up to an example where we have one thread that will
generate values and send them down a channel. The main thread will receive the
values and print them out.

First, though, let’s start by creating a channel but not doing anything with it
in Listing 16-6:

Filename: src/main.rs

```
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

Listing 16-6: Creating a channel and assigning the two halves to `tx` and `rx`

The `mpsc::channel` function crates a new channel. `mpsc` stands for *multiple
producer, single consumer*. In short, we can have multiple *sending* ends of a
channel that produce values, but only one *receiving* end that consumes those
values. We’re going to start with a single producer for now, but we’ll add
multiple producers once we get this example working.

`mpsc::channel` returns a tuple: the first element is the sending end, and the
second element is the receiving end. For historical reasons, many people use
`tx` and `rx` to abbreviate *transmitter* and *receiver*, so those are the
names we’re using for the variables bound to each end. We’re using a `let`
statement with a pattern that destructures the tuples; we’ll be discussing the
use of patterns in `let` statements and destructuring in Chapter 18.

Let’s move the transmitting end into a spawned thread and have it send one
string, shown in Listing 16-7:

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

We’re using `thread::spawn` to create a new thread, just as we did in the
previous section. We use a `move` closure to make `tx` move into the closure so
that the thread owns it.

The transmitting end of a channel has the `send` method that takes the value we
want to send down the channel. The `send` method returns a `Result<T, E>` type,
because if the receiving end has already been dropped, there’s nowhere to send
a value to, so the send operation would error. In this example, we’re simply
calling `unwrap` to ignore this error, but for a real application, we’d want to
handle it properly. Chapter 9 is where you’d go to review strategies for proper
error handling.

In Listing 16-8, let’s get the value from the receiving end of the channel in
the main thread:

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
Here, we’re using `recv`, which is short for *receive*. This method will block
execution until a value is sent down the channel. Once a value is sent, `recv`
will return it in a `Result<T, E>`. When the sending end of the channel closes,
`recv` will return an error. The `try_recv` method will not block; it instead
returns a `Result<T, E>` immediately.

If we run the code in Listing 16-8, we’ll see the value printed out from the
main thread:

```
Got: hi
```

### How Channels Interact with Ownership

Let’s do an experiment at this point to see how channels and ownership work
together: we’ll try to use `val` in the spawned thread after we’ve sent it down
the channel. Try compiling the code in Listing 16-9:

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
`tx.send`. This is a bad idea: once we’ve sent the value to another thread,
that thread could modify it or drop it before we try to use the value again.
This could cause errors or unexpected results due to inconsistent or
nonexistent data.

If we try to compile this code, Rust will error:

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

Our concurrency mistake has caused a compile-time error! `send` takes ownership
of its parameter and moves the value so that the value is owned by the
receiver. This means we can’t accidentally use the value again after sending
it; the ownership system checks that everything is okay.

In this regard, message passing is very similar to single ownership in Rust.
Message passing enthusiasts enjoy message passing for similar reasons that
Rustaceans enjoy Rust’s ownership: single ownership means certain classes of
problems go away. If only one thread at a time can use some memory, there’s no
chance of a data race.

### Sending Multiple Values and Seeing the Receiver Waiting

The code in Listing 16-8 compiled and ran, but it wasn’t very interesting: it’s
hard to see that we have two separate threads talking to each other over a
channel. Listing 16-10 has some modifications that will prove to us that this
code is running concurrently: the spawned thread will now send multiple
messages and pause for a second between each message.

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
            thread::sleep(Duration::new(1, 0));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

Listing 16-10: Sending multiple messages and pausing between each one

This time, we have a vector of strings in the spawned thread that we want to
send to the main thread. We iterate over them, sending each individually and
then pausing by calling the `thread::sleep` function with a `Duration` value of
one second.

In the main thread, we’re not calling the `recv` function explicitly anymore:
instead we’re treating `rx` as an iterator. For each value received, we’re
printing it out. When the channel is closed, iteration will end.

When running the code in Listing 16-10, we’ll see this output, with a one second
pause in between each line:

```
Got: hi
Got: from
Got: the
Got: thread
```

We don’t have any pausing or code that would take a while in the `for` loop in
the main thread, so we can tell that the main thread is waiting to receive
values from the spawned thread.

### Create Multiple Producers by Cloning the Transmitter

Near the start of this section, we mentioned that `mpsc` stood for *multiple
producer, single consumer*. We can expand the code from Listing 16-10 to create
multiple threads that all send values to the same receiver. We do that by
cloning the transmitting half of the channel, as shown in Listing 16-11:

Filename: src/main.rs

```
// ...snip...
let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::new(1, 0));
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
        thread::sleep(Duration::new(1, 0));
    }
});
// ...snip...
```

Listing 16-11: Sending multiple messages and pausing between each one

This time, before we create the first spawned thread, we call `clone` on the
sending end of the channel. This will give us a new sending handle that we can
pass to the first spawned thread. We’ll pass the original sending end of the
channel to a second spawned thread, and each thread is sending different
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

You might see the values in a different order, though. It depends on your
system! This is what makes concurrency interesting as well as difficult. If you
play around with `thread::sleep`, giving it different values in the different
threads, you can make the runs more non-deterministic and create different
output each time.

Now that we’ve seen how channels work, let’s look at shared-memory concurrency.

## Shared State Concurrency

While message passing is a fine way of dealing with concurrency, it’s not the
only one. Consider this slogan again:

> Do not communicate by sharing memory; instead, share memory by
> communicating.

What would “communicate by sharing memory” look like? And moreover, why would
message passing enthusiasts dislike it, and dislike it enough to invert it
entirely?

Remember how channels are sort of like single ownership? Shared memory
concurrency is sort of like multiple ownership: multiple threads can access the
same memory location at the same time. As we saw with multiple ownership made
possible by smart pointers in Chapter 15, multiple ownership can add additional
complexity, since we need to manage these different owners somehow.

Rust’s type system and ownership can help a lot here in getting this management
correct, though. For an example, let’s look at one of the more common
concurrency primitives for shared memory: mutexes.

### Mutexes Allow Access to Data from One Thread at a Time

A *mutex* is a concurrency primitive for sharing memory. It’s short for “mutual
exclusion”, that is, it only allows one thread to access some data at any given
time. Mutexes have a reputation for being hard to use, since there’s a lot you
have to remember:

1. You have to remember to attempt to acquire the lock before using the data.
2. One you’re done with the data that’s being guarded by the mutex, you have
   to remember to unlock the data so other threads can acquire the lock.

For a real-world example of a mutex, imagine a panel discussion at a conference
where there is only one microphone. Before a panelist may speak, they have to
ask or signal that they would like to use the microphone. Once they get the
microphone, they may talk for as long as they would like, then hand the
microphone to the next panelist who would like to speak. It would be rude for a
panelist to start shouting without having the microphone or to steal the
microphone before another panelist was finished. No one else would be able to
speak if a panelist forgot to hand the microphone to the next person when they
finished using it. If the management of the shared microphone went wrong in any
of these ways, the panel would not work as planned!

Management of mutexes can be incredibly tricky to get right, and that’s why so
many people are enthusiastic about channels. However, in Rust, we can’t get
locking and unlocking wrong, thanks to the type system and ownership.

#### The API of `Mutex<T>`

Let’s look at an example of using a mutex in Listing 16-12, without involving
multiple threads for the moment:

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

Like many types, we create a `Mutex<T>` through an associated function named
`new`. To access the data inside the mutex, we use the `lock` method to acquire
the lock. This call will block until it’s our turn to have the lock. This call
can fail if another thread was holding the lock and then that thread panicked.
In a similar way as we did in Listing 16-6 in the last section, we’re using
`unwrap()` for now, rather than better error handling. See Chapter 9 for better
tools.

Once we have acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system is how
Rust ensures that we acquire a lock before using this value: `Mutex<i32>` is
not an `i32`, so we *must* acquire the lock in order to be able to use the
`i32` value. We can’t forget; the type system won’t let us do otherwise.

As you may have suspected, `Mutex<T>` is a smart pointer. Well, more
accurately, the call to `lock` returns a smart pointer called `MutexGuard`.
This smart pointer implements `Deref` to point at our inner data, similar to
the other smart pointers we saw in Chapter 15. In addition, `MutexGuard` has a
`Drop` implementation that releases the lock. This way, we can’t forget to
release the lock. It happens for us automatically when the `MutexGuard` goes
out of scope, which it does at the end of the inner scope in Listing 16-12. We
can print out the mutex value and see that we were able to change the inner
`i32` to 6.

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
        let handle = thread::spawn(|| {
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

Listing 16-13: The start of a program having 10 threads each increment a
counter guarded by a `Mutex<T>`

We’re creating a `counter` variable to hold an `i32` inside a `Mutex<T>`, like
we did in Listing 16-12. Next, we’re creating 10 threads by mapping over a
range of numbers. We use `thread::spawn` and give all the threads the same
closure: they’re each going to acquire a lock on the `Mutex<T>` by calling the
`lock` method and then add 1 to the value in the mutex. When a thread finishes
running its closure, `num` will go out of scope and release the lock so that
another thread can acquire it.

In the main thread, we’re collecting all the join handles like we did in
Listing 16-2, and then calling `join` on each of them to make sure all the
threads finish. At that point, the main thread will acquire the lock and print
out the result of this program.

We hinted that this example won’t compile, let’s find out why!

```
error[E0373]: closure may outlive the current function, but it borrows
`counter`, which is owned by the current function
  -->
   |
9  |         let handle = thread::spawn(|| {
   |                                    ^^ may outlive borrowed value `counter`
10 |             let mut num = counter.lock().unwrap();
   |                           ------- `counter` is borrowed here
   |
help: to force the closure to take ownership of `counter` (and any other
referenced variables), use the `move` keyword, as shown:
   |         let handle = thread::spawn(move || {
```

This is similar to the problem we solved in Listing 16-5. Given that we spin up
multiple threads, Rust can’t know how long the threads will run and whether
`counter` will still be valid when each thread tries to borrow it. The help
message has a reminder for how to solve this: we can use `move` to give
ownership to each thread. Let’s try it by making this change to the closure:

```
thread::spawn(move || {
```

And trying to compile again. We’ll get different errors this time!

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

`move` didn’t fix this program like it fixed Listing 16-5. Why not? This error
message is a little confusing to read, because it’s saying that the `counter`
value is moved into the closure, then is captured when we call `lock`. That
sounds like what we wanted, but it’s not allowed.

Let’s reason this out. Instead of making 10 threads in a `for` loop, let’s just
make two threads without a loop and see what happens then. Replace the first
`for` loop in Listing 16-13 with this code instead:

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

Here we’re making 2 threads, and we changed the variable names used with the
second thread to `handle2` and `num2`. We’re simplifying our example for the
moment to see if we can understand the error message we’re getting. This time,
compiling gives us:

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

Aha! In the first error message, Rust is showing us that `counter` is moved
into the closure for the thread that goes with `handle`. That move is
preventing us from capturing `counter` when we try to call `lock` on it and
store the result in `num2`, which is in the second thread! So Rust is telling
us that we can’t move ownership of `counter` into multiple threads. This was
hard to see before since we were creating multiple threads in a loop, and Rust
can’t point to different threads in different iterations of the loop.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we were able to have multiple ownership of a value by using the
smart pointer `Rc<T>` to create a reference-counted value. We mentioned in
Chapter 15 that `Rc<T>` was only for single-threaded contexts, but let’s try
using `Rc<T>` in this case anyway and see what happens. We’ll wrap the
`Mutex<T>` in `Rc<T>` in Listing 16-14, and clone the `Rc<T>` before moving
ownership to the thread. We’ll switch back to the `for` loop for creating the
threads, and keep the `move` keyword with the closure:

Filename: src/main.rs

```
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = counter.clone();
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

Wow, that’s quite wordy! Some important parts to pick out: the first note says
`Rc<Mutex<i32>> cannot be sent between threads safely`. The reason for this is
in the error message, which, once distilled, says `the trait bound Send is not
satisfied`. We’re going to talk about `Send` in the next section; it’s one of
the traits that ensures the types we use with threads are meant for use in
concurrent situations.

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it has to add to the count for each call to
`clone` and subtract from the count when each clone is dropped. `Rc<T>` doesn’t
use any concurrency primitives to make sure that changes to the count happen in
an operation that couldn’t be interrupted by another thread. This could lead to
subtle bugs where the counts are wrong, which could lead to memory leaks or
dropping a value before we’re done with it. So what if we had a type that was
exactly like `Rc<T>`, but made changes to the reference count in a thread-safe
way?

#### Atomic Reference Counting with `Arc<T>`

If you thought that question sounded like a leading one, you’d be right. There
is a type like `Rc<T>` that’s safe to use in concurrent situations: `Arc<T>`.
The ‘a’ stands for *atomic*, so it’s an *atomically reference counted* type.
Atomics are an additional kind of concurrency primitive that we won’t cover
here; see the standard library documentation for `std::sync::atomic` for more
details. The gist of it is this: atomics work like primitive types, but are
safe to share across threads.

Why aren’t all primitive types atomic, and why aren’t all standard library
types implemented to use `Arc<T>` by default? Thread safety comes with a
performance penalty that we only want to pay when we need it. If we’re only
doing operations on values within a single thread, our code can run faster
since it doesn’t need the guarantees that atomics give us.

Back to our example: `Arc<T>` and `Rc<T>` are identical except for the atomic
internals of `Arc<T>`. Their API is the same, so we can change the `use` line
and the call to `new`. The code in Listing 16-15 will finally compile and run:

```
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = counter.clone();
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

We did it! We counted from 0 to 10, which may not seem very impressive, but we
learned a lot about `Mutex<T>` and thread safety along the way! The structure
that we’ve built in this example could be used to do more complicated
operations than just incrementing a counter. Calculations that can be divided
up into independent parts could be split across threads in this way, and we can
use a `Mutex<T>` to allow each thread to update the final result with its part.

You may have noticed that, since `counter` is immutable but we could get a
mutable reference to the value inside it, this means `Mutex<T>` provides
interior mutability, like the `Cell` family does. In the same way that we used
`RefCell<T>` in Chapter 15 to be able to mutate contents inside an `Rc<T>`, we
use `Mutex<T>` to be able to mutate contents inside of an `Arc<T>`.

Recall that `Rc<T>` did not prevent every possible problem: we also talked
about the possibility of creating reference cycles where two `Rc<T>` values
refer to each other, which would cause a memory leak. We have a similar problem
with `Mutex<T>` that Rust also doesn’t prevent: deadlocks. A *deadlock* is a
situation in which an operation needs to lock two resources, and two threads
have each acquired one of the locks and will now wait for each other forever.
If you’re interested in this topic, try creating a Rust program that has a
deadlock, then research deadlock mitigation strategies that apply to the use of
mutexes in any language and try implementing them in Rust. The standard library
API documentation for `Mutex<T>` and `MutexGuard` will have useful information.

Rust’s type system and ownership has made sure that our threads have exclusive
access to the shared value when they’re updating it, so the threads won’t
overwrite each other’s answers in unpredictable ways. It took us a while to
work with the compiler to get everything right, but we’ve saved future time
that might be spent trying to reproduce subtly incorrect scenarios that only
happen when the threads run in a particular order.

Let’s round out this chapter by talking about the `Send` and `Sync` traits and
how we could use them with custom types.

## Extensible Concurrency with the `Sync` and `Send` Traits

One interesting aspect of Rust’s concurrency model is that the language knows
*very* little about concurrency. Almost everything we’ve been talking about so
far has been part of the standard library, not the language itself. Because we
don’t need the language to provide everything we need to program in a
concurrent context, we’re not limited to the concurrency options that the
standard library or language provide: we can write our own or use ones others
have written.

We said *almost* everything wasn’t in the language, so what is? There are two
traits, both in `std::marker`: `Sync` and `Send`.

### `Send` for Indicating Ownership May Be Transferred to Another Thread

The `Send` marker trait indicates that ownership of that type may be
transferred between threads. Almost every Rust type is `Send`, but there are
some exceptions. One type provided by the standard library that is not `Send`
is `Rc<T>`: if we clone an `Rc<T>` value and try to transfer ownership of the
clone to another thread, both threads might update the reference count at the
same time. As we mentioned in the previous section, `Rc<T>` is implemented for
use in single-threaded situations where you don’t want to pay the performance
penalty of having a threadsafe reference count.

Because `Rc<T>` is not marked `Send`, Rust’s type system and trait bounds
ensure that we can never forget and accidentally send an `Rc<T>` value across
threads unsafely. We tried to do this in Listing 16-14, and we got an error
that said `the trait Send is not implemented for Rc<Mutex<i32>>`. When we
switched to `Arc<T>`, which is `Send`, the code compiled.

Any type that is composed entirely of `Send` types is automatically marked as
`Send` as well. Almost all primitive types are `Send`, aside from raw pointers,
which we’ll discuss in Chapter 19. Most standard library types are `Send`,
aside from `Rc<T>`.

### `Sync` for Indicating Access from Multiple Threads is Safe

The `Sync` marker trait indicates that a type is safe to have references to a
value from multiple threads. Another way to say this is for any type `T`, `T`
is `Sync` if `&T` (a reference to `T`) is `Send` so that the reference can be
sent safely to another thread. In a similar manner as `Send`, primitive types
are `Sync` and types composed entirely of types that are `Sync` are also `Sync`.

`Rc<T>` is also not `Sync`, for the same reasons that it’s not `Send`.
`RefCell<T>` (which we talked about in Chapter 15) and the family of related
`Cell<T>` types are not `Sync`. The implementation of the borrow checking at
runtime that `RefCell<T>` does is not threadsafe. `Mutex<T>` is `Sync`, and can
be used to share access with multiple threads as we saw in the previous section.

### Implementing `Send` and `Sync` Manually is Unsafe

Usually, we don’t need to implement the `Send` and `Sync` traits, since types
that are made up of `Send` and `Sync` traits are automatically also `Send` and
`Sync`. Because they’re marker traits, they don’t even have any methods to
implement. They’re just useful for enforcing concurrency-related invariants.

Implementing the guarantees that these traits are markers for involves
implementing unsafe Rust code. We’re going to be talking about using unsafe
Rust code in Chapter 19; for now, the important information is that building
new concurrent types that aren’t made up of `Send` and `Sync` parts requires
careful thought to make sure the safety guarantees are upheld. The Nomicon at *https://doc.rust-lang.org/stable/nomicon/vec.html*
has more information about these guarantees and how to uphold them.

## Summary

This isn’t the last we’ll see of concurrency in this book; the project in
Chapter 20 will use these concepts in a more realistic situation than the
smaller examples we discussed in this chapter.

As we mentioned, since very little of how Rust deals with concurrency has to be
part of the language, there are many concurrency solutions implemnted as
crates. These evolve more quickly than the standard library; search online for
the current state-of-the-art crates for use in multithreaded situations.

Rust provides channels for message passing and smart pointer types like
`Mutex<T>` and `Arc<T>` that are safe to use in concurrent contexts. The type
system and the borrow checker will make sure the code we write using these
solutions won’t have data races or invalid references. Once we get our code
compiling, we can rest assured that our code will happily run on multiple
threads without the kinds of hard-to-track-down bugs common in other
programming languages. Concurrent programming is no longer something to be
afraid of: go forth and make your programs concurrent, fearlessly!

Next, let’s talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger, and how Rust’s idioms relate to those you
might be familiar with from Object Oriented Programming.
