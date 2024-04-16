## Async and Await

In Chapter 16, we saw one of Rust’s approaches to concurrency: using threads.
Since Rust 1.39, there has been another option for concurrency: the async-await
model.

In the async-await model, concurrent operations do not require their own
threads. Instead, they can run on *tasks*. A task is a bit like a thread, but
instead of being managed by the operating system, it is managed by a runtime.
The job of a runtime is to schedule Some languages, including Go, Kotlin,
Erlang, and Swift, ship runtimes with the language. In Rust, there are many
different runtimes, because the things a runtime for a high-throughput web
server should do are very different from the things a runtime for a
microcontroller should do.

In the rest of chapter, we will:

* see how to use Rust’s `async` and `.await` syntax
* explore how to use the async-await model to solve some of the same challenges
  we looked at in Chapter 16
* look at how multithreading and async provide complementary solutions, which
  you can even use together in many cases

First, though, let’s explore what async-await gives us.

### Why async-await

Many operations we ask the computer to do can take a while to finish. For
example, if you used a video editor to create a video of a family celebration,
exporting it could take anywhere from minutes to hours. Similarly, when you
upload that video to some service to share it with your family, that upload
process might take a long time.

It would be nice if we could do something else while we are waiting for those
long-running processes to complete.

As we saw in the previous chapter, threads provide one approach to concurrency,
and they let us solve some of these issues. However, they also have some
tradeoffs. On many operating systems, they use a fair bit of memory for each
thread, and they come with some overhead for starting up and shutting down.
Threads are also only an option when your operating system and hardware support
multiple threads. While mainstream desktop and mobile operating systems have all
had threading for many years, many embedded operating systems used on
microcontrollers do not.



The async-await model provides a different, complementary set of tradeoffs.
