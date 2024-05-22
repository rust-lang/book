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
to implement the same counting example as with threads.

To start, we will set up our `main` function with `trpl::block_on`:

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-01/src/main.rs:block_on}}
```

> Note: From this point forward in the chapter, every example will include this
> exact same code, so we will often skip it just like we do with `main`. Don’t
> forget to include it in your own code!

Then we can write two loops within that block, each with a `trpl::sleep` call in
them. Similar to the threading example, we put one loop in the body of a
`trpl::spawn_task`, the same way we did with `thread::spawn`, and the other in a
top-level `for` loop. Notice that we also need to add a `.await` after the
`sleep` calls.

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-01/src/main.rs:task}}
```

Putting that all together, we end up with the code in Listing 17-TODO:

<Listing number="17-TODO" caption="Showing how we might implement two counters with `async` instead of threads" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-01/src/main.rs:all}}
```

</Listing>

This does something very similar to what the thread-based implementation did, as
we can see from the output when we run it. (As with the threading example, you
may see a different order in your own terminal output when you run this.)

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

This stops as soon as the for loop in the body of the main async block finishes,
because the task spawned by `spawn_task` is shut down when the main function
ends—just like threads are. Thus, if you want to run all the way to the
completion of the task, you will need to use a join handle to wait for the first
task to complete. With threads, we used the `join` method to “block” until the
thread was done running. Here, we can use `await` to do the same thing:

<Listing number="17-TODO" caption="Using `.await` with a join handle to run a task to completion" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-02/src/main.rs:handle}}
```

</Listing>

Now the output again looks like what we saw in the threading example.

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

So far, it looks like async and threads basically give us the same basic
behavior. However, there are a few important differences already. One was using
`.await` instead of calling `join` on the join handle. Another is that we needed
to await both `sleep` calls. Most importantly, though, we did not need to spawn
another operating system thread to do this. We were able to get concurrency for
just the cost of a task, which has much faster startup time and uses much less
memory than an OS thread.

What is more, we actually do not need the `spawn_task` call at all to get
concurrency here. Remember that each async block compiles to an anonymous
future. That means we can put each of these two loops in an async block and then
ask the runtime to run them both to completion using `trpl::join`:

<Listing number="17-TODO" caption="Using `trpl::join` to await two anonymous futures" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-03/src/main.rs:join}}
```

</Listing>

When we run this, we see both futures run to completion:

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
meaning it checks both futures equally, rather than letting one race ahead. With
threads, the operating system decides which thread to check, and that is
ultimately out of our control. With an async runtime, the runtime itself decides
which future to check, so it has the final say. In practice, the details get
complicated because an async runtime might use operating system threads under
the hood as part of how it manages concurrency, but a runtime can still choose
to guarantee fairness even so. However, runtimes do not have to guarantee
fairness for any given operation, and even within a given runtime, different
APIs sometimes exist to let you choose whether fairness is something you care
about as a caller.

Try some of these different variations on awaiting the futures and see what they
do:

* Remove the async block from around either or both of the loops.
* Await each async block immediately after defining it.
* Wrap only the first loop in an async block, and await the resulting future
  after the body of second loop.

For an extra challenge, see if you can figure out what the output will be in
each case *before* running the code!

### Futures, Tasks, and Threads

<!-- TODO: discuss tasks vs. threads more *here*? -->

### Message Passing

<!-- NOTE: mentions blocking again, need to define it somewhere before this. -->

Sharing data between futures will look familiar. We can again use async versions
of Rust’s types for message-passing. Instead of `std::sync:mpsc::channel`, we
will use a `tprl::channel`, for example.

The `Receiver::recv()` method in the `std` channel blocks until it receives a
message. The `trpl::Receiver::recv()` method, by contrast, is an `async`
function. Instead of blocking, it sleeps until a message is received or the send
side of the channel closes. One other difference with this particular `recv()`
implementation is that it returns an `Option` of the type sent over the channel
instead of a `Result`.

<!-- TODO: build up to this, rather than dumping the whole code all at once -->

<Listing number="17-TODO" caption="Using an async mpsc channel" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-04/src/main.rs:all}}
```

</Listing>

If we run this, though, it never stops! You will need to shut it down using
<span class="keystroke">ctrl-c</span>. We can see that `tx` sends all the
messages,and `rx` receives and prints them, but we never see the “Done!”
message, and the program never stops running. That’s because of the combination
of the `while let` loop and the `trpl::join` call:

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-04/src/main.rs:loop}}
```

Let’s consider the way this loop works:

* The `trpl::join` future only completes once *both* futures passed to it
  have completed.
* The `tx` future completes after sending the second message.
* The `rx` future will not complete until the `while let` loop ends, though.
* The `while let` loop will not end until `rx.recv().await` produces `None`.
* The `rx.recv().await` will only return `None` once the other end of the
  channel is closed.
* The channel will only close if we call `rx.close()` or when the sender side,
  `tx`, is dropped.
* We do not call `rx.close()` anywhere, and `tx` will not be dropped until the
  function exits.
* The function cannot exit because it is blocked on `trpl::join` completing,
  which takes us back to the top of the list!

To solve this, then, we need to make sure the channel gets closed so that
`trpl::join` will complete. We could manually close `rx` somewhere by calling
`rx.close()`, but that does not make much sense in this case. The idea is that
`rx` should keep listening until `tx` is done sending. Stopping after handling
some arbitrary number of messages would make the program shut down, but it would
mean we could miss messages if the sending side changed. Given that we cannot
use `rx.close()`, we need to make sure that `tx` gets dropped *before* the end
of the function.

Right now, the async block only borrows `tx`. We can confirm this by adding
another async block which uses `tx`, and using `trpl::join3` to wait for all
three futures to complete:

<Listing number="17-TODO" caption="Adding another async block which borrows `tx`, to see that we can borrow it repeatedly" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-04b/src/main.rs:updated}}
```

</Listing>

Now both blocks borrow `tx`, so they are both able to use it to send messages,
which `rx` can then receive. When we run that code, we see the extra output from
the new `async` block, and the message it sends being received by the
`rx.recv()`:

```text
Got: hi
Got: more
Got: from
Got: messages
Got: the
Got: for
Got: future
Got: you
```

As before, we also see that the program does not shut down on its own and
requires a <span class="keystroke">ctrl-c</span>. Now that we have seen how
`async` blocks borrow the items they reference from their outer scope, we can go
ahead and remove the extra block we just added, and switch back to using
`trpl::join` instead of `trpl::join3`.

This little exploration makes the original issue much clearer: it is ultimately
about *ownership*. We need to move `tx` into the async block so that once that
block ends, `tx` will be dropped.

In Chapter 13, we learned how to use the `move` keyword with closures, and in
Chapter 16, we saw that we often need to use closures marked with `move` when
working with threads. As we have discovered, the same dynamics apply to async
blocks—so the `move` keyword also works with async blocks, allowing them to take
ownership of the data they reference.

<!-- TODO: scrap this or find it a home! -->
Remember, any time you write a future, a runtime is ultimately responsible for
executing it. That means that an async block might outlive the function where
you write it, the same way a closure can.
<!-- TODO: Through here -->

We can do that by making the first async block an `async move` block.

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-05/src/main.rs:move}}
```

The result is Listing 17-TODO, and when we run *this* version of the code, it
shuts down gracefully after the last message is sent.

<Listing number="17-TODO" caption="Fixing the async mpsc channel by using `move` to take ownership of the `Sender` (`tx`)" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-TODO-05/src/main.rs:move}}
```

</Listing>

<!--
  TODO: maybe explore `tx.clone()`, picking up the thread from the 3-futures
  version of the example? That would let us see how shared ownership can work,
  and give a place to emphasize that we still need to make sure *one* of the
  async blocks
-->

<!-- TODO: bridge into a discussion of `Pin` by showing `join_all`? -->

<!-- TODO: find this a home or scrap it -->
The `async` keyword does not yet work with closures directly. That is, there is
no direct equivalent to `async fn` for anonymous functions. As a result, you
cannot write code like these function calls:

```rust,ignore
example_1(async || { ... });
example_2(async move || { ... });
```

However, since async blocks themselves can be marked with `move`, this ends up
not being a problem. Remember that `async` blocks compile to anonymous futures.
That means you can write calls like this instead:

```rust,ignore
example_1(|| async { ... });
example_2(|| async move { ... });
```

These closures now return anonymous futures, meaning they work basically the
same way that an async function does.
<!-- TODO: through here -->
