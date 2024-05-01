## Async and Await

In Chapter 16, we saw one of Rust’s approaches to concurrency: using threads.
Since Rust 1.39, there has been another option for concurrency: asynchronous
programming, or *async*.

In the rest of chapter, we will:

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
upload that video to some service to share it with your family, that upload
process might take a long time. It would be nice if we could do something else
while we are waiting for those long-running processes to complete.

In the previous chapter we treated parallelism and concurrency as
interchangeable. Now we need to distinguish between the two a little more:

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

> Note: When working with async in Rust, we need to think in terms of
> *concurrency*. Depending on the hardware, the operating system, and the async
> runtime we are using, that concurrency may use some degree of parallelism
> under the hood, or it may not. More about async runtimes in a later section!

Consider again the examples of exporting a video file and waiting on the video
file to finish uploading. The video export will use as much CPU and GPU power as
it can. If you only had one CPU core, and your operating system never paused
that export until it completed, you could not do anything else on your computer
while it was running. That would be a pretty frustrating experience, though, so
instead your computer can (and does!) invisibly interrupt the export often
enough to let you get other small amounts of work done along the way.

The file upload is different. It does not take up very much CPU time. Instead,
you are mostly waiting on data to transfer across the network. If you only have
a single CPU core, you might write a bunch of data to a network socket and then
wait for it to finish getting sent by the network controller. You could choose
to wait for all the data to get “flushed” from the socket and actually sent over
the network, but if there is a busy network connection, you might be waiting for
a while… with your CPU doing not much! Thus, even if you make a blocking call to
write to a socket, your computer probably does other things while the network
operation is happening.

In both of these cases, it might be useful for *your program* to participate in
the same kind of concurrency the computer is providing for the rest of the
system. One way to do this is the approach we saw last chapter: using threads,
which are provided and managed by the operating system. Another way to get
access to concurrency is using language-specific capabilities—like async.

A big difference between the cooking analogy and Rust’s async model for
concurrency is that in the cooking example, the cook makes the decision about
when to switch tasks. In Rust’s async model, the tasks are in control of that.
To see how, let’s look at how Rust actually uses async.
