## Futures, Tasks, and Threads

As we saw in the previous chapter, threads provide one approach to concurrency.
We’ve seen another approach to concurrency in this chapter, using async with
futures and streams. You might be wondering why you would choose one or the
other. The answer is: it depends! And in many cases, the choice isn’t threads
_or_ async but rather threads _and_ async.

Many operating systems have supplied threading-based concurrency models for
decades now, and many programming languages have support for them as a result.
However, they are not without their tradeoffs. On many operating systems, they
use a fair bit of memory for each thread, and they come with some overhead for
starting up and shutting down. Threads are also only an option when your
operating system and hardware support them! Unlike mainstream desktop and mobile
computers, some embedded systems don’t have an OS at all, so they also don’t
have threads!

The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations don’t require their own
threads. Instead, they can run on tasks, as when we used `trpl::spawn_task` to
kick off work from a synchronous function throughout the streams section. A task
is similar to a thread, but instead of being managed by the operating system,
it’s managed by library-level code: the runtime.

In the previous section, we saw that we could build a `Stream` by using an async
channel and spawning an async task which we could call from synchronous code. We
could do the exact same thing with a thread! In Listing 17-40, we used
`trpl::spawn_task` and `trpl::sleep`. In Listing 17-41, we replace those with
the `thread::spawn` and `thread::sleep` APIs from the standard library in the
`get_intervals` function.

<Listing number="17-41" caption="Using the `std::thread` APIs instead of the async `trpl` APIs for the `get_intervals` function" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-41/src/main.rs:threads}}
```

</Listing>

If you run this, the output is identical. And notice how little changes here
from the perspective of the calling code! What’s more, even though one of our
functions spawned an async task on the runtime and the other spawned an
OS thread, the resulting streams were unaffected by the differences.

Despite the similarities, these two approaches behave very differently, although
we might have a hard time measuring it in this very simple example. We could
spawn millions of async tasks on any modern personal computer. If we tried to do
that with threads, we would literally run out of memory!

However, there’s a reason these APIs are so similar. Threads act as a boundary
for sets of synchronous operations; concurrency is possible _between_ threads.
Tasks act as a boundary for sets of _asynchronous_ operations; concurrency is
possible both _between_ and _within_ tasks, because a task can switch between
futures in its body. Finally, futures are Rust’s most granular unit of
concurrency, and each future may represent a tree of other futures. The
runtime—specifically, its executor—manages tasks, and tasks manage futures. In
that regard, tasks are similar to lightweight, runtime-managed threads with
added capabilities that come from being managed by a runtime instead of by the
operating system.

This doesn’t mean that async tasks are always better than threads, any more than
that threads are always better than tasks.

Concurrency with threads is in some ways a simpler programming model than
concurrency with `async`. That can be a strength or a weakness. Threads are
somewhat “fire and forget,” they have no native equivalent to a future, so they
simply run to completion, without interruption except by the operating system
itself. That is, they have no built-in support for _intra-task concurrency_ the
way futures do. Threads in Rust also have no mechanisms for cancellation—a
subject we haven’t covered in depth in this chapter, but which is implicit in
the fact that whenever we ended a future, its state got cleaned up correctly.

These limitations also make threads harder to compose than futures. It’s much
more difficult, for example, to use threads to build helpers such as the
`timeout` we built in [“Building Our Own Async Abstractions”][combining-futures]
or the `throttle` method we used with streams in [“Composing Streams”][streams].
The fact that futures are richer data structures means they can be composed
together more naturally, as we have seen.

Tasks then give _additional_ control over futures, allowing you to choose where
and how to group the futures. And it turns out that threads and tasks often
work very well together, because tasks can (at least in some runtimes) be moved
around between threads. We haven’t mentioned it up until now, but under the
hood the `Runtime` we have been using, including the `spawn_blocking` and
`spawn_task` functions, is multithreaded by default! Many runtimes use an
approach called _work stealing_ to transparently move tasks around between
threads based on the current utilization of the threads, with the aim of
improving the overall performance of the system. To build that actually requires
threads _and_ tasks, and therefore futures.

As a default way of thinking about which to use when:

- If the work is _very parallelizable_, such as processing a bunch of data where
  each part can be processed separately, threads are a better choice.
- If the work is _very concurrent_, such as handling messages from a bunch of
  different sources which may come in a different intervals or different rates,
  async is a better choice.

And if you need some mix of parallelism and concurrency, you don’t have to
choose between threads and async. You can use them together freely, letting each
one serve the part it is best at. For example, Listing 17-42 shows a fairly
common example of this kind of mix in real-world Rust code.

<Listing number="17-42" caption="Sending messages with blocking code in a thread and awaiting the messages in an async block" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-42/src/main.rs:all}}
```

</Listing>

We begin by creating an async channel. Then we spawn a thread which takes
ownership of the sender side of the channel. Within the thread, we send the
numbers 1 through 10, and sleep for a second in between each. Finally, we run a
future created with an async block passed to `trpl::run` just as we have
throughout the chapter. In that future, we await those messages, just as in
the other message-passing examples we have seen.

To return to the examples we opened the chapter with: you could imagine running
a set of video encoding tasks using a dedicated thread, because video encoding
is compute bound, but notifying the UI that those operations are done with an
async channel. Examples of this kind of mix abound!

## Summary

This isn’t the last you’ll see of concurrency in this book: the project in
Chapter 21 will use the concepts in this chapter in a more realistic situation
than the smaller examples discussed here—and compare more directly what it looks
like to solve these kinds of problems with threading vs. with tasks and futures.

Whether with threads, with futures and tasks, or with the combination of them
all, Rust gives you the tools you need to write safe, fast, concurrent
code—whether for a high-throughput web server or an embedded operating system.

Next, we’ll talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger. In addition, we’ll discuss how Rust’s idioms
relate to those you might be familiar with from object-oriented programming.

[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#composing-streams
