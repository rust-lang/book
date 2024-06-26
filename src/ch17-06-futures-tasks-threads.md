# Futures, Tasks, and Threads

As we saw in the previous chapter, threads provide one approach to concurrency.
We have seen another approach to concurrency in this chapter, using async with
futures and streams. You might be wondering why you would choose one or the
other. The answer is: it depends! And in many cases, it is not threads *vs.*
async but rather threads *and* async.

Threads are an older and more common tool for concurrency. Many operating
systems have supplied threading-based concurrency models for decades now, and
many programming languages have support for them as a result. However, they are
not without their tradeoffs. On many operating systems, they use a fair bit of
memory for each thread, and they come with some overhead for starting up and
shutting down. Threads are also only an option when your operating system and
hardware support them! Unlike mainstream desktop and mobile operating systems,
many embedded operating systems, like those used on some microcontrollers, do
not have OS-level threads at all.

The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations do not require their own
threads. Instead, they can run on tasks, as when we used `trpl::spawn_task` to
kick off work from a synchronous function throughout the streams section. A task
is a lot like a thread—but instead of being managed by the operating system, it
is managed by library-level code: the runtime.

In the previous section, we saw that we could build a `Stream` by using a
channel and spawning an async task which we could call from synchronous code. We
could do the exact same thing with a thread! We’ll use a simpler version of the
streams example so we can focus on the differences. In Listing 17-50, we used
`trpl::spawn_task` and `trpl::sleep`. In Listing 17-51, we replace those with
the `thread::spawn` and `thread::sleep` APIs from the standard library, in just
the `get_intervals` function.

<Listing number="17-51" caption="TODO" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-51/src/main.rs:threads}}
```

</Listing>

If you run this, the output is identical. And notice how little changes here
from the perspective of the calling code! What is more, even though one of our
functions spawned an async task on the runtime and the other spawned an
operating system thread, they worked exactly the same way as far as processing
the stream was concerned.

This highlights that async tasks are kind of like lightweight, runtime-managed
threads. However, there is a meaningful difference in the way this system
behaves, although we might have a hard time measuring it in this very simple
example. We could spawn hundreds of thousands or even millions of async tasks on
any modern personal computer. If we tried to do that with threads, we would
literally run out of memory!

However, this does not mean that async tasks are always better than threads. In
fact, they often work very well *together*. We have not mentioned it up until
now, but under the hood the `Runtime` we have been using, including the
`spawn_blocking` and `spawn_task` functions, are multithreaded by default! Many
runtimes can transparently move tasks around between threads based on the
current utilization of the threads, to hopefully improve the overall performance
of the system.

<!-- TODO: keep going from here: differentiate them? -->

### Parallelism and Concurrency

<!-- TODO: phrasing makes less sense given new home here at end of chapter -->

First, though, we need to dig a little deeper into the differences between
parallelism and concurrency. In the previous chapter we treated them as mostly
interchangeable. Now we need to distinguish between the two a little more,
because the differences will show up as we start working:

* *Parallelism* is when operations can happen simultaneously.

* *Concurrency* is when operations can make progress without having to wait for
  all other operations to complete.

One common analogy for thinking about the difference between concurrency and
parallelism is cooking in a kitchen. Parallelism is like having two cooks: one
working on cooking eggs, and the other working on preparing fruit bowls. Those
can happen at the same time, without either affecting the other. Concurrency is
like having a single cook who can start cooking some eggs, start dicing up some
vegetables to use in the omelette, adding seasoning and whatever vegetables are
ready to the eggs at certain points, and switching back and forth between those
tasks.

(This analogy breaks down if you think about it too hard. The eggs keep cooking
while the cook is chopping up the vegetables, after all. That is parallelism,
not just concurrency! The focus of the analogy is the *cook*, not the food,
though, and as long as you keep that in mind, it mostly works.)

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
may not. (More about async runtimes later!)

A big difference between the cooking analogy and Rust’s async model for
concurrency is that in the cooking example, the cook makes the decision about
when to switch tasks. In Rust’s async model, the tasks are in control of that.
To see how, let’s look at how Rust actually uses async.
