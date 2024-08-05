## Async and Await

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

The file download is different. It does not take up very much CPU time. The CPU
needs to wait on data to arrive from the network. While you can start reading
the data once some of it arrives, it might take a while for the rest to arrive.
Even once the data has all arrived, videos can be quite large, so it might take
some time to load all the data from the network. Maybe it only takes a second or
two—but that is a very long time for a modern processor, which can do billions
of operations every second. It would be nice to be able to put the CPU to use
for other work while waiting for the network call to finish—so, again, your
computer will once again invisibly interrupt your program so other things can
happen while the network operation is still ongoing.

> Note: The video export is the kind of operation which is often described as
> “CPU-bound” or “compute-bound”. It is limited by the speed of the computer’s
> ability to process data within the *CPU* or *GPU*, and how much of that speed
> it can use. The video download is the kind of operation which is often
> described as “IO-bound,” because it is limited by the speed of the computer’s
> *input and output*. It can only go as fast as the data can be sent across the
> network.

In both of these examples, the concurrency only happens at the level of a whole
program. The operating system interrupts one program to let other
programs get work done. In many cases, since we understand our programs at a
much more granular level than the operating system does, we can spot lots of
opportunities for concurrency that the operating system cannot see.

For example, if we are building a tool to manage file downloads, we should be
able to write our program in such a way that starting one download does not lock
up the UI, and users should be able to start multiple downloads at the same
time. Many operating system APIs for interacting with the network are
*blocking*, though. That is, these APIs block the program’s progress until the
data that they are processing is completely ready.

> Note: This is how *most* function calls work, if you think about it! However,
> we normally reserve the term “blocking” for function calls which interact with
> files, the network, or other resources on the computer, because those are the
> places where an individual program would benefit from the operation being
> *non*-blocking.

We could avoid blocking our main thread by spawning a dedicated thread to
download each file. But it would be nicer if the call were not blocking in the
first place. It would also be nice if we could write in the same direct style
we use in blocking code. Something like this:

```rust,ignore,does_not_compile
let data = fetch_data_from(url).await;
println!("{data}");
```

That is exactly what Rust’s async abstraction gives us. Before we see how this
works in practice, though, we need to take a short detour into the differences
between parallelism and concurrency.

### Parallelism and Concurrency

In the previous chapter we treated parallelism and concurrency as mostly
interchangeable. Now we need to distinguish between them more precisely, because
the differences will show up as we start working.

Think about working on a software project as a team.

When an individual works on several different tasks before any of them is
complete, this is *concurrency*. Maybe you have two different projects checked
out on your computer, and when you get bored or stuck on one project, you switch
to the other. You are just one person, and you cannot make progress on both
tasks at the exact same time—but you can multi-task, making progress on multiple
tasks by switching between them.

<figure>

<img alt="Concurrent work flow" src="img/trpl17-01.svg" class="center" />

<figcaption>Figure 17-1: A concurrent workflow, switching between Task A and Task B</figcaption>

</figure>

When you agree to split up a group of tasks between the people on the team, with
each person taking one task and working on it alone, this is *parallelism*. Each
person on the team can make progress at the exact same time.

<figure>

<img alt="Concurrent work flow" src="img/trpl17-02.svg" class="center" />

<figcaption>Figure 17-2: A parallel workflow, where work happens on Task A and Task B independently</figcaption>

</figure>

With both of these situations, you might have to coordinate between different
tasks. Maybe you *thought* the task that one person was working on was totally
independent from everyone else’s work, but it actually needs something finished
by another person on the team. Some of the work could be done in parallel, but
some of it was actually *serial*: it could only happen in a series, one thing
after the other. Likewise, you might realize that one of the tasks you were
working on needs the result from another of your tasks. Now your concurrent work
has also become serial.

Parallelism and concurrency can intersect with each other, too. For example, if
it turns out your coworker is waiting on one of your projects to finish, you
might need to focus on that project and not give any time to your other task
until it is done. In that case, you and your coworker are no longer able to work
in parallel *and* you are no longer able to work concurrently.

The same basic dynamics come into play with software and hardware. On a machine
with a single CPU core, the CPU can only do one operation at a time, but it can
still work concurrently. Using tools like threads, processes, and async, the
computer can pause one activity and switch to others before eventually cycling
back to that first activity again. On a machine with multiple CPU cores, it can
also do work in parallel. One core can be doing one thing while another core
does something completely unrelated, and those actually happen at the same
time.

When working with async in Rust, we are always dealing with concurrency.
Depending on the hardware, the operating system, and the async runtime we are
using—more on async runtimes shortly!—that concurrency may or may not also use
parallelism under the hood.

Now, let’s dive into how async programming in Rust actually works! In the rest
of this chapter, we will:

* see how to use Rust’s `async` and `.await` syntax
* explore how to use the async model to solve some of the same challenges we
  looked at in Chapter 16
* look at how multithreading and async provide complementary solutions, which
  you can even use together in many cases
