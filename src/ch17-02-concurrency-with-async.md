## Concurrency With Async

In this section, we will apply async to some of the same concurrency challenges
we tackled with threads in chapter 16. Since we already talked about a lot of
the key ideas there, in this section we will focus on what is different between
threads and futures.

In many cases, the APIs for working with concurrency using async are very
similar to those for using threads. In other cases, they end up being shaped
fairly differently. Even when the APIs look similar, they often have different
behavior and they nearly always have different performance characteristics.

### Counting

The first task we tackled in Chapter 16 was counting up on two separate threads.
Let’s do the same using async. The `trpl` crate supplies a `spawn_task` function
which looks very similar to the `thread::spawn` API, and a `sleep` function
which is an async version of the `thread::sleep` API. We can use these together
to implement the same counting example as with threads, in Listing 17-5.

<Listing number="17-5" caption="Using `spawn_task` to count with two" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

As our starting point, we set up our `main` function with `trpl::block_on`, so
that our top-level function can be async.

> Note: From this point forward in the chapter, every example will include this
> exact same wrapping code with `trpl::block_on` in `main`, so we will often
> skip it just like we do with `main`. Don’t forget to include it in your
> code!

Then we write two loops within that block, each with a `trpl::sleep` call in it,
which waits for half a second (500 milliseconds) before sending the next
message. We put one loop in the body of a `trpl::spawn_task` and the other in a
top-level `for` loop. We also add an `.await` after the `sleep` calls.

This does something similar to the thread-based implementation—including the
fact that you may see the messages appear in a different order in your own
terminal when you run it.

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

This version stops as soon as the for loop in the body of the main async block
finishes, because the task spawned by `spawn_task` is shut down when the main
function ends. If you want to run all the way to the completion of the task, you
will need to use a join handle to wait for the first task to complete.

<Listing number="17-6" caption="Using `.await` with a join handle to run a task to completion" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:handle}}
```

</Listing>

With threads, we used the `join` method to “block” until the thread was done
running. In Listing 17-6, we can use `await` to do the same thing, because the
task handle itself is a future. Its `Output` type is a `Result`, so we also
unwrap it after awaiting it. This updated version runs till *both* loops
finish.

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
with different syntax: using `.await` instead of calling `join` on the join
handle, and awaiting the `sleep` calls.

The bigger difference is that we did not need to spawn another operating system
thread to do this. In fact, we do not even a task here. Given that async blocks
compile to anonymous futures, we can put each loop in an async block and have
the runtime run them both to completion using `trpl::join`.

<!--
We were able to get concurrency for just the cost of a task.
Tasks have much faster startup time and use much less memory than an OS thread.
-->

<Listing number="17-7" caption="Using `trpl::join` to await two anonymous futures" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:join}}
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

Here, you will see the exact same order every time, which is very different from
what we saw with threads. That is because the `trpl::join` function is *fair*,
meaning it checks each future equally often, alternating between them, and never
lets one race ahead if the other is ready. With threads, the operating system
decides which thread to check and how long to let it run. With async Rust, the
runtime decides which future to check. (In practice, the details get complicated
because an async runtime might use operating system threads under the hood as
part of how it manages concurrency, so guaranteeing fairness can be more work
for a runtime—but it is still possible!) Runtimes do not have to guarantee
fairness for any given operation, and runtimes often offer different APIs to let
you choose whether you want fairness or not.

Try some of these different variations on awaiting the futures and see what they
do:

* Remove the async block from around either or both of the loops.
* Await each async block immediately after defining it.
* Wrap only the first loop in an async block, and await the resulting future
  after the body of second loop.

For an extra challenge, see if you can figure out what the output will be in
each case *before* running the code!

### Message Passing

Sharing data between futures will also be familiar: we will use message passing,
again, but this with async versions of the types and functions.

<Listing number="17-8" caption="Creating an async channel and assigning the two halves to `tx` and `rx`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:channel}}
```

</Listing>

We start by using `trpl::channel`, an async version of the multiple-producer,
single-consumer channel API we used with threads back in Chapter 16. The async
version of the API is only a little different: we have a mutable receiver `rx`.
With the channel in place, we can send messages from the sender to the receiver.
Again, the API is just a little different from the threaded version. Instead of
spawning a separate thread, we await the `rx.recv()` call.

The synchronous `Receiver::recv()` method in `std::mpsc::channel` blocks until
it receives a message. The `trpl::Receiver::recv()` method does not, because it
is async. Instead of blocking, it will return `Poll::Pending` until a message is
received or the send side of the channel closes. By contrast, we do not await
the `send` call, because it does not block. It does not need to, because the
channel we are sending it into is unbounded.

> Note: Since this is all wrapped in a `trpl::block_on`, this would effectively
> block anything happening outside that. That is the whole point of `block_on`,
> in fact: to allow you to *choose* where to block on some set of async code to
> transition between sync and async code. However, *within* this block, the
> `.await` does not block further operations—as we will see!

It is hard to see the effect of async in Listing 17-8, though, since the message
will arrive right away! Let’s go ahead and send a whole series of messages, and
sleep in between them, as shown in Listing 17-9:

<!-- We cannot test this one because it never stops! -->

<Listing number="17-9" caption="Sending and receiving multiple messages over the async channel and sleeping with an `.await` between each message" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:many-messages}}
```

</Listing>

In addition to sending the messages, we need to receive them. In this case, we
could do that manually, by just doing `rx.recv().await` four times, because we
know how many messages are coming in. In the real world, though, we will
generally be waiting on some *unknown* number of messages. In that case, we need
to keep waiting until we determine that there are no more messages.

That sounds like a good job for a loop! In synchronous code, we might use a
`for` loop to process a sequence of items, regardless of how many items are in
the loop. However, Rust does not yet have a way to write a `for` loop over an
*asynchronous* series of items. Instead, we need to use a new kind of loop we
haven’t seen before, the `while let` conditional loop. A `while let` loop is the
loop version of the `if let` construct we saw back in Chapter 6. It continues as
long as the condition it relies on is true.

<!-- TODO: update text in ch. 19 to account for our having introduced this. -->

The `rx.recv()` call produces a `Future`, which we await. The runtime will pause
the `Future` until it is ready. Once a message arrives, the future will resolve
to `Some(message)`, as many times as a message arrives. When the channel closes,
regardless of whether *any*  messages have arrived, the future will instead
resolve to `None` to indicate that there are no more values, and we should stop
polling—that is, stop awaiting.

The `while let` loop pulls all of this together. If the result of calling
`rx.recv().await` is `Some(message)`, we get access to the message and we can
use it in the loop body, just like we could with `if let`. If the result is
`None`, the loop ends. Every time the loop completes, it hits the await point
again, so the runtime pauses it again until another message arrives.

The code now successfully sends and receives all of the messages. Unfortunately,
there are still a couple problems. For one thing, the messages do not arrive at
half-second intervals. They arrive all at once, two seconds (2,000 milliseconds)
after we start the program. For another, this program also never stops! You will
need to shut it down using <span class="keystroke">ctrl-c</span>.

Let’s start by understanding why the messages all come in at once after the full
delay, rather than coming in with delays in between each one. Within a given
async block, the order that `.await` keywords appear in the code is also the
order they happen when running the program.

There is only one async block in Listing 17-9, so everything in it runs
linearly. All the `tx.send` calls happen, interspersed with all of the
`trpl::sleep` calls and their associated await points. Only then does the `while
let` loop get to go through any of the `.await` points on the `recv` calls.

To get the behavior we want, where the sleep delay happens between receiving
each message, we need to put the `tx` and `rx` operations in their own async
blocks. Then the runtime can execute each of them separately using `trpl::join`,
just like in the counting example.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="Separating `send` and `recv` into their own `async` blocks and awaiting the futures for those blocks" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:futures}}
```

</Listing>

With the updated code in Listing 17-10, the messages get printed at
500-millisecond intervals, rather than all in a rush after two seconds.

The program still never stops, because of the way `while let` loop interacts
with `trpl::join`:

* The future returned from `trpl::join` only completes once *both* futures
  passed to it have completed.
* The `tx` future completes once it finishes sleeping after sending the last
  message in `vals`.
* The `rx` future will not complete until the `while let` loop ends.
* The `while let` loop will not end until `rx.recv().await` produces `None`.
* The `rx.recv().await` will only return `None` once the other end of the
  channel is closed.
* The channel will only close if we call `rx.close()` or when the sender side,
  `tx`, is dropped.
* We do not call `rx.close()` anywhere, and `tx` will not be dropped until the
  async block ends.
* The block cannot end because it is blocked on `trpl::join` completing,
  which takes us back to the top of this list!

We could manually close `rx` by calling `rx.close()` somewhere, but that does
not make much sense. Stopping after handling some arbitrary number of messages
would make the program shut down, but we could miss messages. We need some other
way to make sure that `tx` gets dropped *before* the end of the function.

Right now, the async block where we send the messages only borrows `tx`, but if
we could move `tx` into that async block, it would be dropped once that block
ends. In Chapter 13, we learned how to use the `move` keyword with closures, and
in Chapter 16, we saw that we often need to use move data into closures when
working with threads. The same basic dynamics apply to async blocks, so the
`move` keyword works with async blocks just like it does with closures.

In Listing 17-11, we change the async block for sending messages an `async move`
block. When we run *this* version of the code, it shuts down gracefully after
the last message is sent.

<Listing number="17-11" caption="A working example of sending and receiving messages between futures which correctly shuts down when complete" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:with-move}}
```

</Listing>

This async channel is also a multiple-producer channel, so we can call `clone`
on `tx` if we want to send messages from multiple futures. In Listing 17-12, we
clone `tx`, creating `tx1` outside the first async block. We move `tx1` into
that block just as we did before with `tx`. Then, later, we move the original
`tx` into a *new* async block, where we send more messages on a slightly
slower delay. (We happen to put this new async block after the async block
for receiving messages, but it could go before it just as well.)

Both of the async blocks for sending messages need to be `async move` blocks, so
that both `tx` and `tx1` get dropped when those blocks finish. Otherwise we will
end up back in the same infinite loop we started out in. Finally, we switch from
`trpl::join` to `trpl::join3` to handle the additional future.

<Listing number="17-12" caption="Using multiple producers with async blocks" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:here}}
```

</Listing>

Now we see all the messages from both sending futures. Because the sending
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

[streams]: /ch17-05-streams.md
