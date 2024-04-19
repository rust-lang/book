## Tasks

As we saw in the previous chapter, threads provide one approach to concurrency,
and they let us solve some of these issues. However, they also have some
tradeoffs. On many operating systems, they use a fair bit of memory for each
thread, and they come with some overhead for starting up and shutting down.
Threads are also only an option when your operating system and hardware support
multiple threads. While mainstream desktop and mobile operating systems have all
had threading for many years, many embedded operating systems used on
microcontrollers do not.

The async-await model provides a different, complementary set of tradeoffs.

In the async-await model, concurrent operations do not require their own
threads. Instead, they can run on *tasks*. A task is a bit like a thread, but
instead of being managed by the operating system, it is managed by a runtime.
The job of a runtime is to schedule Some languages, including Go, Kotlin,
Erlang, and Swift, ship runtimes with the language. In Rust, there are many
different runtimes, because the things a runtime for a high-throughput web
server should do are very different from the things a runtime for a
microcontroller should do.
