## Async and Await

The threading-based concurrency model is one of the oldest concurrency models in
computing, and it was present and well-supported in Rust since 1.0. In the past
few decades, though, many programming languages have been experimenting with
other approaches to concurrency, especially asynchronous programming, or
*async*.

It took a few years to work out the right design for async in Rust. After a
bunch of experimentation and design work in the library ecosystem, Rust added
language-level support for async in Rust 1.39, in 2019, and there is a thriving
ecosystem of crates supporting all sorts of interesting capabilities offered by
those language primitives.

In the rest of this chapter, we will:

* see how to use Rust’s `async` and `.await` syntax
* explore how to use the async model to solve some of the same challenges we
  looked at in Chapter 16
* look at how multithreading and async provide complementary solutions, which
  you can even use together in many cases

First, though, let’s explore what async gives us.

### Why Async?

Many operations we ask the computer to do can take a while to finish. For
example, if you used a video editor to create a video of a family celebration,
exporting it could take anywhere from minutes to hours. Similarly, downloading a
video shared by someone in your family might take a long time. It would be nice
if we could do something else while we are waiting for those long-running
processes to complete.

The video export will use as much CPU and GPU power as it can. If you only had
one CPU core, and your operating system never paused that export until it
completed, you could not do anything else on your computer while it was running.
That would be a pretty frustrating experience, though. Instead, your computer
can (and does!) invisibly interrupt the export often enough to let you get other
work done along the way.

The file download is different. It does not take up very much CPU time. You are
mostly waiting on data to transfer across the network. You can start reading
from a network socket, but it might take a while for all the data to arrive and
be fed into the socket by the network controller. Even once the data has all
arrived, videos can be quite large, so it might take some time to load all the
data from the socket. Maybe it only takes a second or two—but that is a very
long time for a modern processor, which can do billions of operations every
second. It would be nice to be able to put the CPU to use for other work while
waiting for the network call to finish—so, again, your computer will once again
invisibly interrupt your program so other things can happen while the network
operation is still ongoing.

> Note: The video export is the kind of operation which is often described as
> “CPU-bound”. It is limited by the speed of the computer’s *CPU and GPU*, and
> how much of that speed it can use. The video download is the kind of operation
> which is often described as “IO-bound,” because it is limited by the speed of
> the computer’s *input and output*. It can only go as fast as the data can be
> sent across the network, which means that it can only go as fast as the data
> can be written to the socket by the network controller.

In both of these examples, the concurrency only happens at the level of a whole
program. The operating system interrupts one program to let other
programs get work done. In many cases, since we understand our programs at a
much more granular level than the operating system does, we can spot lots of
opportunities for concurrency that the operating system cannot see.

For example, if we are building a tool to manage file downloads, we should be
able to write our program in such a way that starting one download does not lock
up the UI, and users should be able to start multiple downloads at the same
time. Many operating system APIs for interacting with network sockets are
*blocking*, though. That is, the function calls block further progress in the
program when they are called until they return.

> Note: This is how *most* function calls work, if you think about it! However,
we normally reserve the term “blocking” for function calls which interact with
files, network sockets, or other resources on the computer, because those are
the places where an individual program would benefit from the operation being
*non*-blocking.

We could use threads to avoid blocking while downloading files, by using a
dedicated thread. But it would be nicer if the call were not blocking in the
first place.

One way to accomplish that would be to use an API built around callbacks. For
each blocking operation, we could pass in a function to call once the operation
completes:

```rust,ignore
network_socket.read_non_blocking(|result| {
    // ...
});
```

Or we could register callbacks to run when events happen:

```rust,ignore
network_socket.add_listener(Event::ReadFinished, |event| {
    // ...
});
```

Or we could have our functions return a type with `and_then` method, which in
turn accepts a callback which can do more work of the same sort (Historically,
this was the way that Rust did async):

```rust,ignore
network_socket.read_non_blocking().and_then(|result| {
    /* another non_blocking operation */
});
```

Each of these makes it harder to understand both the control flow and the flow
of data through the program. You can end up with event handler callbacks
scattered across the code base, or groups of deeply nested callbacks, or long
chains of `and_then` calls.

There are also no particularly good ways to get data out of those callbacks.
With other common types in Rust, we often use pattern-matching in scenarios like
this. When we are using callbacks we do not yet have the data at the time we
call `read_non_blocking`—and we will not have it until the callback gets called.
That means that there is no way to match on the data it will return: it is not
here yet!

As an alternative, we might try something like this, imagining a
`read_non_blocking` which has exactly the kind of `and_then` method described
above. If we were to try to do that, though, with code kind of like this, it
would not even compile:

```rust,ignore,does_not_compile
let mut data = None;
network_socket.read_non_blocking().and_then(|result| {
    data = Some(result);
});
println!("{data:?}");
```

In this very broken code, the callback passed to `and_then` needs a mutable
reference to `data`, but the `println` macro needs to borrow a reference to
`data` to be able to print it. Rust would helpfully tell us that we cannot
borrow `data` immutably to print it because it is still borrowed mutably for the
`and_then` callback. This is not just Rust being fussy, either: the result of
this would normally always just print the `None` value, but if the read
*happened* to go fast enough, it is possible it could sometimes print some
string data instead. That is *definitely* not what we want!

We also could not cancel `read_non_blocking`: once it has started, it will run
till it finishes unless the whole program stops. <!-- TODO: check whether we
pick back up this thread anywhere! Cancellation is important. -->

What we really want to be able to write is something much more direct, like we
would write in blocking code, but with the benefits of getting the data when it
is available and *not* blocking the rest of the program while waiting for the
data to arrive—something like this:

```rust,ignore,does_not_compile
let data = network_socket.read(&path).await;
println!("{data}");
```

That is exactly what Rust’s async abstraction gives us. It is designed to help
us solve all of these issues. Before we will see how this works in practice,
though, we need to dig a little deeper into the differences between parallelism
and concurrency.

### Parallelism and Concurrency

In the previous chapter we treated parallelism and concurrency as mostly
interchangeable. Now we need to distinguish between them more precisely, because
the differences will show up as we start working:

* *Parallelism* is when operations can happen simultaneously.

* *Concurrency* is when operations can make progress without having to wait for
  all other operations to complete.

Think about working on a software project as a team. When you agree to split up
a group of tasks between a group of people, with each person working on one task
and delivering them separately, this is *parallelism*. Each person on the team
can be making progress at the exact same time. On the other hand, when an
individual works on several different tasks before any of them is complete, this
is *concurrency*. Maybe you have two different projects checked out on your
computer, and when you get bored or stuck on one project, you switch to the
other. You are just one person, and you cannot make progress on both tasks at
the exact same time—but you can multi-task, making progress on multiple tasks by
switching between them. Work on one does not necessarily *block* working on the
other.

With both of these situations, you might have to coordinate between different
tasks. Maybe you *thought* the task that one person was working on was totally
independent from everyone else’s work, but it actually needs something finished
by another person on the team. Some of the work could be done in parallel, but
some of it was actually *serial*: it could only happen in a series, one thing
after the other. Likewise, you might realize that one of the tasks you were
working on needs the result from another of your tasks. Now your concurrent work
has also become serial.

Parallelism and concurrency can intersect with each other, too. For example, if
it turns out your coworker is waiting on one of your projects to finish, then
you might need to focus on that project and not give any time to your other task
until it is done, so your own work stops being concurrent.

On a machine with a single CPU core, the CPU can only do one operation at a
time, but we can still have concurrency. Using tools like threads, processes,
and async, the computer can pause one activity and switch to others before
eventually cycling back to that first activity again. On a machine with multiple
CPU cores, we can actually do work in parallel. One core can be doing one thing
while another core does something completely unrelated, and those actually
happen at the same time.

When working with async in Rust, we are always dealing with concurrency.
Depending on the hardware, the operating system, and the async runtime we are
using—more on async runtimes shortly!—that concurrency may or may not also use
parallelism under the hood. Now, let’s dive into how async programming in Rust
actually works!
