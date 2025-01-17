## Combining Concurrency with Async

<!-- Old headings. Do not remove or links may break. -->
<a id="concurrency-with-async"></a>

In this section, we’ll apply async to some of the same concurrency challenges
we tackled with threads in chapter 16. Because we already talked about a lot of
the key ideas there, in this section we’ll focus on what’s different between
threads and futures.

In many cases, the APIs for working with concurrency using async are very
similar to those for using threads. In other cases, they end up being quite
different. Even when the APIs _look_ similar between threads and async, they
often have different behavior—and they nearly always have different performance
characteristics.

<!-- Old headings. Do not remove or links may break. -->
<a id="counting"></a>

### Creating a New Task with `spawn_task`

The first operation we tackled in [Creating a New Thread with
Spawn][thread-spawn]<!-- ignore --> was counting up on two separate threads.
Let’s do the same using async. The `trpl` crate supplies a `spawn_task` function
that looks very similar to the `thread::spawn` API, and a `sleep` function
that is an async version of the `thread::sleep` API. We can use these together
to implement the counting example, as shown in Listing 17-6.

<Listing number="17-6" caption="Creating a new task to print one thing while the main task prints something else" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

As our starting point, we set up our `main` function with `trpl::run` so that
our top-level function can be async.

> Note: From this point forward in the chapter, every example will include this
> exact same wrapping code with `trpl::run` in `main`, so we’ll often skip it
> just as we do with `main`. Don’t forget to include it in your code!

Then we write two loops within that block, each containing a `trpl::sleep` call,
which waits for half a second (500 milliseconds) before sending the next
message. We put one loop in the body of a `trpl::spawn_task` and the other in a
top-level `for` loop. We also add an `await` after the `sleep` calls.

This code behaves similarly to the thread-based implementation—including the
fact that you may see the messages appear in a different order in your own
terminal when you run it:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

This version stops as soon as the `for` loop in the body of the main async block
finishes, because the task spawned by `spawn_task` is shut down when the `main`
function ends. If you want it to run all the way to the task’s completion, you
will need to use a join handle to wait for the first task to complete. With
threads, we used the `join` method to “block” until the thread was done running.
In Listing 17-7, we can use `await` to do the same thing, because the task
handle itself is a future. Its `Output` type is a `Result`, so we also unwrap it
after awaiting it.

<Listing number="17-7" caption="Using `await` with a join handle to run a task to completion" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

This updated version runs until _both_ loops finish.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

So far, it looks like async and threads give us the same basic outcomes, just
with different syntax: using `await` instead of calling `join` on the join
handle, and awaiting the `sleep` calls.

The bigger difference is that we didn’t need to spawn another operating system
thread to do this. In fact, we don’t even need to spawn a task here. Because
async blocks compile to anonymous futures, we can put each loop in an async
block and have the runtime run them both to completion using the `trpl::join`
function.

In the section [Waiting for All Threads to Finishing Using `join`
Handles][join-handles]<!-- ignore -->, we showed how to use the `join` method on
the `JoinHandle` type returned when you call `std::thread::spawn`. The
`trpl::join` function is similar, but for futures. When you give it two futures,
it produces a single new future whose output is a tuple containing the output of
each future you passed in once they _both_ complete. Thus, in Listing 17-8, we
use `trpl::join` to wait for both `fut1` and `fut2` to finish. We do _not_ await
`fut1` and `fut2` but instead the new future produced by `trpl::join`. We ignore
the output, because it’s just a tuple containing two unit values.

<Listing number="17-8" caption="Using `trpl::join` to await two anonymous futures" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

When we run this, we see both futures run to completion:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Now, you’ll see the exact same order every time, which is very different from
what we saw with threads. That is because the `trpl::join` function is _fair_,
meaning it checks each future equally often, alternating between them, and never
lets one race ahead if the other is ready. With threads, the operating system
decides which thread to check and how long to let it run. With async Rust, the
runtime decides which task to check. (In practice, the details get complicated
because an async runtime might use operating system threads under the hood as
part of how it manages concurrency, so guaranteeing fairness can be more work
for a runtime—but it’s still possible!) Runtimes don’t have to guarantee
fairness for any given operation, and they often offer different APIs to let you
choose whether or not you want fairness.

Try some of these variations on awaiting the futures and see what they do:

- Remove the async block from around either or both of the loops.
- Await each async block immediately after defining it.
- Wrap only the first loop in an async block, and await the resulting future
  after the body of second loop.

For an extra challenge, see if you can figure out what the output will be in
each case _before_ running the code!

<!-- Old headings. Do not remove or links may break. -->
<a id="message-passing"></a>

### Counting Up on Two Tasks Using Message Passing

Sharing data between futures will also be familiar: we’ll use message passing
again, but this time with async versions of the types and functions. We’ll take
a slightly different path than we did in [Using Message Passing to Transfer Data
Between Threads][message-passing-threads]<!-- ignore --> to illustrate some of
the key differences between thread-based and futures-based concurrency. In
Listing 17-9, we’ll begin with just a single async block—_not_ spawning a
separate task as we spawned a separate thread.

<Listing number="17-9" caption="Creating an async channel and assigning the two halves to `tx` and `rx`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Here, we use `trpl::channel`, an async version of the multiple-producer,
single-consumer channel API we used with threads back in Chapter 16. The async
version of the API is only a little different from the thread-based version: it
uses a mutable rather than an immutable receiver `rx`, and its `recv` method
produces a future we need to await rather than producing the value directly. Now
we can send messages from the sender to the receiver. Notice that we don’t have
to spawn a separate thread or even a task; we merely need to await the `rx.recv`
call.

The synchronous `Receiver::recv` method in `std::mpsc::channel` blocks until
it receives a message. The `trpl::Receiver::recv` method does not, because it
is async. Instead of blocking, it hands control back to the runtime until either
a message is received or the send side of the channel closes. By contrast, we
don’t await the `send` call, because it doesn’t block. It doesn’t need to,
because the channel we’re sending it into is unbounded.

> Note: Because all of this async code runs in an async block in a `trpl::run`
> call, everything within it can avoid blocking. However, the code _outside_ it
> will block on the `run` function returning. That’s the whole point of the
> `trpl::run` function: it lets you _choose_ where to block on some set of async
> code, and thus where to transition between sync and async code. In most async
> runtimes, `run` is actually named `block_on` for exactly this reason.

Notice two things about this example. First, the message will arrive right away.
Second, although we use a future here, there’s no concurrency yet. Everything
in the listing happens in sequence, just as it would if there were no futures
involved.

Let’s address the first part by sending a series of messages and sleeping in
between them, as shown in Listing 17-10.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="Sending and receiving multiple messages over the async channel and sleeping with an `await` between each message" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

In addition to sending the messages, we need to receive them. In this case,
because we know how many messages are coming in, we could do that manually by
calling `rx.recv().await` four times. In the real world, though, we’ll generally
be waiting on some _unknown_ number of messages, so we need to keep waiting
until we determine that there are no more messages.

In Listing 16-10, we used a `for` loop to process all the items received from a
synchronous channel. Rust doesn’t yet have a way to write a `for` loop over an
_asynchronous_ series of items, however, so we need to use a loop we haven’t
seen before: the `while let` conditional loop. This is the loop version of the
`if let` construct we saw back in the section [Concise Control Flow with `if
let` and `let else`][if-let]<!-- ignore -->. The loop will continue executing as
long as the pattern it specifies continues to match the value.

The `rx.recv` call produces a future, which we await. The runtime will pause the
future until it is ready. Once a message arrives, the future will resolve to
`Some(message)` as many times as a message arrives. When the channel closes,
regardless of whether _any_ messages have arrived, the future will instead
resolve to `None` to indicate that there are no more values and thus we should
stop polling—that is, stop awaiting.

The `while let` loop pulls all of this together. If the result of calling
`rx.recv().await` is `Some(message)`, we get access to the message and we can
use it in the loop body, just as we could with `if let`. If the result is
`None`, the loop ends. Every time the loop completes, it hits the await point
again, so the runtime pauses it again until another message arrives.

The code now successfully sends and receives all of the messages. Unfortunately,
there are still a couple of problems. For one thing, the messages do not arrive
at half-second intervals. They arrive all at once, 2 (2,000 milliseconds) after
we start the program. For another, this program also never exits! Instead, it
waits forever for new messages. You will need to shut it down using <span
class="keystroke">ctrl-c</span>.

Let’s start by examining why the messages come in all at once after the full
delay, rather than coming in with delays between each one. Within a given async
block, the order in which `await` keywords appear in the code is also the order
in which they’re executed when the program runs.

There’s only one async block in Listing 17-10, so everything in it runs
linearly. There’s still no concurrency. All the `tx.send` calls happen,
interspersed with all of the `trpl::sleep` calls and their associated await
points. Only then does the `while let` loop get to go through any of the `await`
points on the `recv` calls.

To get the behavior we want, where the sleep delay happens between each message,
we need to put the `tx` and `rx` operations in their own async blocks, as shown
in Listing 17-11. Then the runtime can execute each of them separately using
`trpl::join`, just as in the counting example. Once again, we await the result
of calling `trpl::join`, not the individual futures. If we awaited the
individual futures in sequence, we would just end up back in a sequential
flow—exactly what we’re trying _not_ to do.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-11" caption="Separating `send` and `recv` into their own `async` blocks and awaiting the futures for those blocks" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

With the updated code in Listing 17-11, the messages get printed at
500-millisecond intervals, rather than all in a rush after 2 seconds.

The program still never exits, though, because of the way `while let` loop
interacts with `trpl::join`:

- The future returned from `trpl::join` completes only once _both_ futures
  passed to it have completed.
- The `tx` future completes once it finishes sleeping after sending the last
  message in `vals`.
- The `rx` future won’t complete until the `while let` loop ends.
- The `while let` loop won’t end until awaiting `rx.recv` produces `None`.
- Awaiting `rx.recv` will return `None` only once the other end of the channel
  is closed.
- The channel will close only if we call `rx.close` or when the sender side,
  `tx`, is dropped.
- We don’t call `rx.close` anywhere, and `tx` won’t be dropped until the
  outermost async block passed to `trpl::run` ends.
- The block can’t end because it is blocked on `trpl::join` completing, which
  takes us back to the top of this list.

We could manually close `rx` by calling `rx.close` somewhere, but that doesn’t
make much sense. Stopping after handling some arbitrary number of messages would
make the program shut down, but we could miss messages. We need some other way
to make sure that `tx` gets dropped _before_ the end of the function.

Right now, the async block where we send the messages only borrows `tx` because
sending a message doesn’t require ownership, but if we could move `tx` into that
async block, it would be dropped once that block ends. In the Chapter 13 section
[Capturing References or Moving Ownership][capture-or-move]<!-- ignore -->, you
learned how to use the `move` keyword with closures, and, as discussed in the
Chapter 16 section [Using `move` Closures with Threads][move-threads]<!-- ignore
-->, we often need to move data into closures when working with threads. The
same basic dynamics apply to async blocks, so the `move` keyword works with
async blocks just as it does with closures.

In Listing 17-12, we change the block used to send messages from `async` to
`async move`. When we run _this_ version of the code, it shuts down gracefully
after the last message is sent and received.

<Listing number="17-12" caption="A  revision of the code from Listing 17-11 that correctly shuts down when complete" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

This async channel is also a multiple-producer channel, so we can call `clone`
on `tx` if we want to send messages from multiple futures, as shown in Listing
17-13.

<Listing number="17-13" caption="Using multiple producers with async blocks" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

First, we clone `tx`, creating `tx1` outside the first async block. We move
`tx1` into that block just as we did before with `tx`. Then, later, we move the
original `tx` into a _new_ async block, where we send more messages on a
slightly slower delay. We happen to put this new async block after the async
block for receiving messages, but it could go before it just as well. The key is
the order in which the futures are awaited, not in which they’re created.

Both of the async blocks for sending messages need to be `async move` blocks so
that both `tx` and `tx1` get dropped when those blocks finish. Otherwise, we’ll
end up back in the same infinite loop we started out in. Finally, we switch from
`trpl::join` to `trpl::join3` to handle the additional future.

Now we see all the messages from both sending futures, and because the sending
futures use slightly different delays after sending, the messages are also
received at those different intervals.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

This is a good start, but it limits us to just a handful of futures: two with
`join`, or three with `join3`. Let’s see how we might work with more futures.

[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[join-handles]: ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#capturing-references-or-moving-ownership
[move-threads]: ch16-01-threads.html#using-move-closures-with-threads
