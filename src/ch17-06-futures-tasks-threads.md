## Putting It All Together: Futures, Tasks, and Threads

As we saw in [Chapter 16][ch16]<!-- ignore -->, threads provide one approach to
concurrency. We’ve seen another approach in this chapter: using async with
futures and streams. If you’re wondering when to choose one method over the other,
the answer is: it depends! And in many cases, the choice isn’t threads _or_
async but rather threads _and_ async.

Many operating systems have supplied threading-based concurrency models for
decades now, and many programming languages support them as a result. However,
these models are not without their tradeoffs. On many operating systems, they
use a fair bit of memory for each thread. Threads are also only an option when
your operating system and hardware support them. Unlike mainstream desktop and
mobile computers, some embedded systems don’t have an OS at all, so they also
don’t have threads.

The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations don’t require their own
threads. Instead, they can run on tasks, as when we used `trpl::spawn_task` to
kick off work from a synchronous function in the streams section. A task is
similar to a thread, but instead of being managed by the operating system, it’s
managed by library-level code: the runtime.

There’s a reason the APIs for spawning threads and spawning tasks are so
similar. Threads act as a boundary for sets of synchronous operations;
concurrency is possible _between_ threads. Tasks act as a boundary for sets of
_asynchronous_ operations; concurrency is possible both _between_ and _within_
tasks, because a task can switch between futures in its body. Finally, futures
are Rust’s most granular unit of concurrency, and each future may represent a
tree of other futures. The runtime—specifically, its executor—manages tasks,
and tasks manage futures. In that regard, tasks are similar to lightweight,
runtime-managed threads with added capabilities that come from being managed by
a runtime instead of by the operating system.

This doesn’t mean that async tasks are always better than threads (or vice
versa). Concurrency with threads is in some ways a simpler programming model
than concurrency with `async`. That can be a strength or a weakness. Threads are
somewhat “fire and forget”; they have no native equivalent to a future, so they
simply run to completion without being interrupted except by the operating
system itself.

And it turns out that threads and tasks often work
very well together, because tasks can (at least in some runtimes) be moved
around between threads. In fact, under the hood, the runtime we’ve been
using—including the `spawn_blocking` and `spawn_task` functions—is multithreaded
by default! Many runtimes use an approach called _work stealing_ to
transparently move tasks around between threads, based on how the threads are
currently being utilized, to improve the system’s overall performance. That
approach actually requires threads _and_ tasks, and therefore futures.

When thinking about which method to use when, consider these rules of thumb:

- If the work is _very parallelizable_ (that is, CPU-bound), such as processing
  a bunch of data where each part can be processed separately, threads are a
  better choice.
- If the work is _very concurrent_ (that is, I/O-bound), such as handling
  messages from a bunch of different sources that may come in at different
  intervals or different rates, async is a better choice.

And if you need both parallelism and concurrency, you don’t have to choose
between threads and async. You can use them together freely, letting each
play the part it’s best at. For example, Listing 17-25 shows a fairly common
example of this kind of mix in real-world Rust code.

<Listing number="17-25" caption="Sending messages with blocking code in a thread and awaiting the messages in an async block" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

We begin by creating an async channel, then spawning a thread that takes
ownership of the sender side of the channel using the `move` keyword. Within
the thread, we send the numbers 1 through 10, sleeping for a second between
each. Finally, we run a future created with an async block passed to
`trpl::block_on` just as we have throughout the chapter. In that future, we
await those messages, just as in the other message-passing examples we have
seen.

To return to the scenario we opened the chapter with, imagine running a set of
video encoding tasks using a dedicated thread (because video encoding is
compute-bound) but notifying the UI that those operations are done with an
async channel. There are countless examples of these kinds of combinations in
real-world use cases.

## Summary

This isn’t the last you’ll see of concurrency in this book. The project in
[Chapter 21][ch21]<!-- ignore --> will apply these concepts in a more realistic
situation than the simpler examples discussed here and compare problem-solving
with threading versus tasks and futures more directly.

No matter which of these approaches you choose, Rust gives you the tools you
need to write safe, fast, concurrent code—whether for a high-throughput web
server or an embedded operating system.

Next, we’ll talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger. In addition, we’ll discuss how Rust’s idioms
relate to those you might be familiar with from object-oriented programming.

[ch16]: ch16-00-concurrency.html
[ch21]: ch21-00-final-project-a-web-server.html
