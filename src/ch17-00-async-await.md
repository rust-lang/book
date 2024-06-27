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
exporting it could take anywhere from minutes to hours. Similarly, when you
download a video shared by someone in your family, that download process might
take a long time. It would be nice if we could do something else while we are
waiting for those long-running processes to complete.

The video export will use as much CPU and GPU power as it can. If you only had
one CPU core, and your operating system never paused that export until it
completed, you could not do anything else on your computer while it was running.
That would be a pretty frustrating experience, though, so instead your computer
can (and does!) invisibly interrupt the export often enough to let you get other
small amounts of work done along the way.

The file download is different. It does not take up very much CPU time. Instead,
you are mostly waiting on data to transfer across the network. You can start
reading from a network socket, but it might take a while for all the data to
arrive and be fed into the socket by the network controller. Moreover, even once
the data has all arrived, videos can be quite large, so it might take some time
to load all the data from the socket. Even if “some time” here is just a second
or two, that is a very long time for a modern processor, which can do billions
of operations every second. You could choose to wait for all of that to finish,
but you might be waiting for a while… with your CPU doing not much! Thus, even
if your specific program cannot do anything until it finishes reading data from
a network socket, your computer will once again invisibly interrupt your
program so other things can happen at the same time as the network operation.

> Note: The video export is the kind of operation which is often described as
> “CPU-bound”. It is limited by the speed of the computer’s CPU (and GPU), and
> how much of that power it can use. The video download is the kind of operation
> which is often described as “IO-bound,” because it is limited by the speed of
> the computer’s *input and output*. It can only go as fast as the data can be
> sent across the network, which means that it can only go as fast as the data
> can be written to the socket by the network controller.

In both of these examples, the concurrency happens at the level of the whole
program. The operating system decides to interrupt the program to let other
programs get work done. In many cases, since we understand our programs at a
much more granular level than the operating system does, we can lots of
opportunities for concurrency that the operating system cannot see. For example,
if we are building a tool to manage file downloads, it is important that the
user interface stay responsive while a download is happening. In fact, we should
even be able to start multiple downloads at the same time.

However, many operating system APIs for interacting with network sockets are
*blocking*. That is, the function calls block further progress in the program
when they are called until they return. This is how *most* function calls work,
if you think about it! However, we normally reserve the term “blocking” for
function calls which interact with files, network sockets, or other resources on
the computer, because those are the places where an individual program would
benefit from the operation being *non*-blocking.

When doing file downloads, we could use threads to work around the fact that the
call to write to a network socket is blocking. If we move the data over to a
dedicated thread which handles the write operation, it will *not* block the rest
of the program. But in many ways, it would be nicer if the call were not
blocking in the first place.

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

Each of these can make the control flow for the program more complicated,
though. You can end up with event handler callbacks scattered across the code
base, or groups of deeply nested callbacks, or long chains of `and_then` calls.
Understanding the flow of data through the program can become more difficult as
a result, and dealing with callbacks can also complicate ownership.

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

The callback passed to `and_then` needs a mutable reference to `data`, but the
`load` function tries to return `data` to the caller. Rust would helpfully tell
us that we cannot borrow `data` immutably to print it because it is still
borrowed mutably for the `and_then` callback. This is not just Rust being fussy,
either: the result of this would normally always just print the `None` value and
exit, but if the read *happened* to go fast enough, it is possible it could
sometimes print some string data instead. That is *definitely* not what we
want!

We also could not cancel `read_non_blocking`: once it has started, it will run
till it finishes unless the whole program stops.

What we really want to be able to write is something much more direct, like we
would write in blocking code, but with the benefits of getting the data when it
is available and *not* blocking the rest of the program while waiting for the
data to arrive—something like this:

```rust,ignore,does_not_compile
let data = network_socket.read(&path).await;
println!("{data}");
```

That is exactly what Rust’s async abstraction gives us. It is designed to help
us solve all of these issues. In the next section, we will see how this works in
practice.

### Parallelism and Concurrency

First, though, we need to dig a little deeper into the differences between
parallelism and concurrency. In the previous chapter we treated them as mostly
interchangeable. Now we need to distinguish between the two a little more,
because the differences will show up as we start working:

* *Parallelism* is when operations can happen simultaneously.

* *Concurrency* is when operations can make progress without having to wait for
  all other operations to complete.

One way to think about the difference between parallelism and concurrency is to
think about working on a software project as a team. When you agree to split up
a group of tasks between a group of people, with each person working on one task
and delivering them separately, this is *parallelism*. Each person on the team
can be making progress at the exact same time.

On the other hand, when an individual works on several different tasks before
any of them is complete, this is *concurrency*. Maybe you have two different
projects checked out on your computer, and when you get bored or stuck on one
project, you switch to the other. You are just one person, and you cannot make
progress on both tasks at the exact same time.

With both of these situations, you might have to coordinate between different
tasks. Maybe you *thought* the task that one person was working on was totally
independent from everyone else’s work, but it actually needs something finished
by another person on the team. *Some* of the work could be done in parallel, but
some of it was actually *serial*: it could only happen in a series, one thing
after the other. Likewise, maybe with the two projects you were
switching between yourself, you realize that one of them needs the result from
the other, so now your concurrent work has also become *serial*.

Parallelism and concurrency can intersect with each other, too. For example, if
it turns out your coworker is waiting on one of your projects to finish, then
you might need to focus on that project and not give any time to the other one
until it is done, so your own work stops being concurrent.

On a machine with multiple CPU cores, we can actually do work in parallel. One
core can be doing one thing while another core does something completely
unrelated, and those actually happen at the same time. On a machine with a
single CPU core, the CPU can only do one operation at a time, but we can still
have concurrency. Using tools like threads, processes, and async, the computer
can pause one activity and switch to others before eventually cycling back to
that first activity again. So all parallel operations are also concurrent, but
not all concurrent operations happen in parallel!

When working with async in Rust, we are always dealing with concurrency.
Depending on the hardware, the operating system, and the async runtime we are
using, that concurrency may use some degree of parallelism under the hood, or it
may not. Let’s dive into how async programming in Rust actually works!
