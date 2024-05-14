<!-- TODO: find the right home for this (maybe nowhere!) -->

As we saw in the previous chapter, threads provide one approach to concurrency,
and they let us solve some of these issues. However, they also have some
tradeoffs. On many operating systems, they use a fair bit of memory for each
thread, and they come with some overhead for starting up and shutting down.
Threads are also only an option when your operating system and hardware support
them! While mainstream desktop and mobile operating systems have all had
threading for many years, many embedded operating systems, like those used on
some microcontrollers, do not.

The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations do not require their own
threads. Instead, they can run on *tasks*. A task is a bit like a thread, but
instead of being managed by the operating system, it is managed by code that
lives at the level of libraries.

<!--
  TODO: connective tissue as it were. Also, an open question on whether we want
  to use “task” as the primary term here. Futures do not actually *require* a
  task primitive to run (and there are runtimes which do *not* use tasks!) so it
  might make more sense to find another common, appropriate term for it instead.
-->
