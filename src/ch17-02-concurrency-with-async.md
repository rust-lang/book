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
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:block_on}}
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
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:task}}
```

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
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:handle}}
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
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:join}}
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

### Message Passing

Sharing data between futures will look familiar. We can again use async versions
of Rust’s types for message-passing. Instead of `std::sync:mpsc::channel`, we
will use a `tprl::channel`, for example.

The `Receiver::recv()` method in the `std` channel blocks until it receives a
message. The `trpl::Receiver::recv()` method, by contrast, is an `async`
function. Instead of blocking, it sleeps until a message is received or the send
side of the channel closes. One other difference with this particular `recv()`
implementation is that it returns an `Option` of the type sent over the channel
instead of a `Result`.

We can start by introducing an async version of the multiple-producer,
single-consumer channel channel API we used with threads back in Chapter 16:

<Listing number="17-TODO" caption="Creating an async channel and assigning the two halves to `tx` and `rx`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:add-channel}}
```

</Listing>

Now we can send messages from the sender to the receiver. Unlike in Chapter 16,
where we needed to spawn a separate thread to allow the message passing to
happen asynchronously, here we opt into async behavior on the receiver side by
using `.await` on the `rx.recv()` call.

<Listing number="17-TODO" caption='Sending `"hi"` from `tx` and receiving it in `rx`' file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:send-and-receive}}
```

</Listing>

The `send` call does not block, since the channel we are sending it into is
unbounded. That was true with our threading example back in Chapter 16, too,
though. The difference here is that the `rx.recv()` call here does not block the
rest of the program, whereas the one in Chapter 16 *did* block the main thread.
Instead, once the program hits the `.await` on the `rx.recv()` call, it hands
control back to the runtime, which can go on scheduling other operations until a
message arrives. It might be hard to see that from this code, though, since the
message will arrive right away!

> Note: Since this is all wrapped in a `trpl::block_on`, this would effectively
> block anything happening outside that. That is the whole point of `block_on`,
> in fact: to allow you to *choose* where to block on some set of async code to
> transition between sync and async code. However, *within* this block, the
> `.await` does not block further operations—as we will see!

Let’s go ahead and send a whole series of messages, and sleep in between them,
as shown in Listing 17-TODO:

<Listing number="17-TODO" caption="Sending multiple messages over the async channel and sleeping with an `.await` between each message" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:many-messages}}
```

</Listing>

Then we can wait on each of those messages in a loop. Here, we need to use a
`while let` loop rather than a `for` loop, because Rust does not yet have an
async version of `Iterator`, which is what the `for` loop does. In TODO: SECTION
TITLE, we will see more about two related traits the community has been working
on, `AsyncIterator` and `Stream`. For now, we can stick with `while let`, as in
Listing 17-TODO, and the loop will end when `rx.recv().await` produces a `None`.

<Listing number="17-TODO" caption="Using a `while let` loop with `.await` to receive messages asynchronously" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:loop}}
```

</Listing>

This code does not do what we want. It does successfully send and receive the
messages, but instead of seeing the messages received at one-second intervals,
we see them arrive all at once, four seconds after we start the program. It
also never stops! You will need to shut it down using
<span class="keystroke">ctrl-c</span>.

Let’s start by understanding why the messages all come in at once after the full
delay, rather than coming in with delays in between each one. This highlights an
important point about the way that async works in Rust. Within any given async
block, the await points are sequential: each one happens one after another. That
is, after all, one of the big motivations for using this syntax instead of
callbacks, event handlers, or chains of methods: the flow through the program is
much easier to follow, because having the order that `.await` keywords appear in
the *code* is also the order they happen when running the *program*.

With that in mind, we can see why this code behaves the way it does by looking
at the whole thing all together.

<Listing number="17-TODO" caption="An async block with multiple `.await` points in it" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

There is just one async block here, so everything here will proceed linearly.
Every one of the `.await` points for the `trpl::sleep` calls appears before the
`.await` points on the `rx.recv()`, so all the `tx.send` calls happen,
interspersed with all of the `trpl::sleep` calls. Only then does the `while let`
loop get to go through all of the `.await` points on the `recv` calls.

To get the behavior we actually want, where the delay happens in between
receiving each message, rather than before receiving any message, we need to
give put the `tx` and `rx` operations in their own async blocks, so the runtime
can execute each of them separately. We also need to tell the runtime to
actually run them using `trpl::join`, just like we did for the counting example
above. Listing 17-TODO shows how that looks.

<Listing number="17-TODO" caption="Separating `send` and `recv` into their own `async` blocks and awaiting the futures for those blocks" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:futures}}
```

</Listing>

With these changes made, the messages get printed at one-second intervals,
rather than all in a rush after four seconds.

The program still never stops running, though. That’s because of the combination
of the `while let` loop and the `trpl::join` call. Let’s consider the way this
loop works:

* The `trpl::join` future only completes once *both* futures passed to it
  have completed.
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

We need to make sure the channel gets closed so that `trpl::join` will complete.
We could manually close `rx` somewhere by calling `rx.close()`, but that does
not make much sense in this case. The idea is that `rx` should keep listening
until `tx` is done sending. Stopping after handling some arbitrary number of
messages would make the program shut down, but it would mean we could miss
messages if the sending side changed. Given that we cannot use `rx.close()`, we
need to make sure that `tx` gets dropped *before* the end of the function.

Right now, the async block only borrows `tx`. We can confirm this by adding
another async block which uses `tx`, and using `trpl::join3` to wait for all
three futures to complete:

<Listing number="17-TODO" caption="Adding another async block which borrows `tx`, to see that we can borrow it repeatedly" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:updated}}
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
ownership of the data they reference. We can do that by change the first async
block from an `async` block to an `async move` block, as in Listing 17-TODO:

<Listing number="17-TODO" caption="Fixing the async mpsc channel by using `move` to take ownership of the `Sender` (`tx`)" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:move}}
```

The result is Listing 17-TODO, and when we run *this* version of the code, it
shuts down gracefully after the last message is sent.

<Listing number="17-TODO" caption="A working example of sending and receiving messages between futures which correctly shuts down when complete" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:all}}
```

</Listing>

This async channel is also a multiple-producer channel, so we can call `clone`
on `tx` if we want to send messages from multiple futures. For example, we can
make the code from Listing 17-TODO work by cloning the `tx` before moving it
into the first async block, moving the original `tx` into the second async
block, and switching back to `join3`.

<Listing number="17-TODO" caption="Using multiple producers with async blocks" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:here}}
```

</Listing>

Both of these blocks need to be `async move` blocks, or else we will end up back
in the same infinite loop we started out in.

### Working with More Futures

When we switched from using two futures to three, we also had to switch from
using `join` to using `join3`. It would be annoying to do this every time we
changed our code.

<!-- TODO: explain how to use `join!` -->

However, both the function nor macro forms of `join` only work for cases where
we know the number of futures ahead of time. If instead we have a dynamic number
of futures, we need a function which works with a collection type which can grow
and shrink dynamically at runtime, such as a vector. In real-world Rust, pushing
futures into a collection and then waiting on some or all the futures in that
collection to complete is a very common pattern.

The `trpl::join_all` function accepts any type which implements the `Iterator`
trait, which we learned about back in Chapter 13, so it seems like just the
ticket. Let’s try putting our futures in a vector, so we can swap out our
`join3` call and replace it with `join_all`.

<Listing  number="17-TODO" caption="Storing anonymous futures in a vector and calling `join_all`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:here}}
```

</Listing>

Unfortunately, this does not compile. Instead, we get this error:

<!-- TODO: extract to output.txt -->

```text
error[E0308]: mismatched types
  --> src/main.rs:43:37
   |
8  |           let tx1_fut = async move {
   |  _______________________-
9  | |             let vals = vec![
10 | |                 String::from("hi"),
11 | |                 String::from("from"),
...  |
19 | |             }
20 | |         };
   | |_________- the expected `async` block
21 |
22 |           let rx_fut = async {
   |  ______________________-
23 | |             while let Some(value) = rx.recv().await {
24 | |                 println!("received '{value}'");
25 | |             }
26 | |         };
   | |_________- the found `async` block
...
43 |           let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                       ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:8:23: 20:10}`
              found `async` block `{async block@src/main.rs:22:22: 26:10}`
```

This error message is admittedly not the most helpful! It only tells us that it
expected one async block and found another—but why is it looking for the async
blocks that it names here, and why does it only reference them by where they
appear in the code?

One clue is the format of this message. Notice that it is exactly the same as if
we had tried to create a `Vec` with a a number and a string in it:

<!-- TODO: should this be a listing? -->

```rust
let a = 1;
let b = "Hello";
let vals = vec![a, b];
```

The output there would be:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:24
  |
4 |     let vals = vec![a, b];
  |                        ^ expected integer, found `&str`
```

Saying “expected *something*, found *something else*” is Rust’s standard format
for telling us about a type mismatch. As we saw with vectors in [Using an Enum
to Store Multiple Types][collections] back in Chapter 8, we need the type of
each item in a collection to be the same—and `tx1_fut`, `rx_fut`, and `tx_fut`
do not have the same type.

The underlying issue here is what we learned in the previous section: async
blocks compile to anonymous futures. Under the hood, there is a data structure
corresponding to each of these blocks, and it has its own unique type. This
might be surprising. After all, none of them returns anything, so the `Future`
type in each case is `Future<Output = ()>`. However, `Future` is a trait, not a
concrete type. The actual types here are invisible from our point of view as the
person writing the code.

In Chapter 8, we discussed one way to include multiple types in a single vector:
using an enum to represent each of the different types which can appear in the
vector. We cannot do that here, though. For one thing, we do not even have a way
to name the different types, because they are anonymous. For another, the reason
we reached for a vector and `join_all` in the first place was to be able to work
with a dynamic collection of futures where we do not know what they will all be
until runtime.

To make this work, we need to use *trait objects*, just as we did for returning
different kinds of errors from the same function in [Returning Errors from the
run function][dyn] back in Chapter 12. Again, we will cover trait objects in
detail in Chapter 17. Here, it lets us treat each of the anonymous futures
produced by these types as interchangeable, since all of them by definition
implement the `Future` trait.

We can start by wrapping each of the futures in the `vec!` in a `Box::new()`.
Unfortunately, the initial way we might try this, as shown in Listing 17-TODO,
still does not compile.

<Listing number="17-TODO" caption="" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:here}}
```

</Listing>

In fact, we have the same basic error we did before, but we get one for both the
second and third `Box::new` calls, and we also get new errors referring to the
`Unpin` trait.

We can start by fixing the type error around the `Box::new` calls, by telling
the compiler explicitly that we want to use these types as trait objects. The
clearest way to do that here is by adding a type annotation to the declaration
of `futures`, as we see in Listing 17-TODO. The type we have to write here is a
little involved, so let’s walk through each part of it.

- The innermost type is the future itself. We note explicitly that it the output
  of the future is the unit type `()` by writing `Future<Output = ()>`.
- Then we annotate the trait with `dyn` to mark it as dynamic.
- The entire trait is wrapped in a `Box`.
- Finally, we state explicitly that `futures` is a `Vec` containing these items.

<Listing number="17-TODO" caption="" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:here}}
```

</Listing>

That already made a big difference. Now when we run the compiler, we only have
the errors mentioning `Unpin`, each of which is a variation on this same output:

<!-- TODO: compiler output listing for the listing -->

```text
error[E0277]: `{async block@src/main.rs:8:23: 20:10}` cannot be unpinned
  --> src/main.rs:46:33
   |
46 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:8:23: 20:10}`, which is required by `Box<{async block@src/main.rs:8:23: 20:10}>: Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:8:23: 20:10}>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> /Users/chris/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

That is a *lot* to digest, so let’s pull it apart. The first part of the message
tell us that the first async block (`src/main.rs:8:23: 20:10`) does not
implement the `Unpin` trait, and suggests using `pin!` or `Box::pin` to resolve
it. The rest of the message tells us *why* that is required: the `JoinAll`
struct, which is itself a `Future`, is also generic over a `Future`, and
`Future` itself requires the `Unpin` trait. Understanding this error means we
need to dive into a little more of how the `Future` type actually works, in
particular the idea of *pinning*.

### Pinning and the Pin and Unpin Traits

When we introduced the `Future` trait in the previous chapter, we saw that the
definition of its `poll` method has an unusual way of specifying the `self`
parameter. To review, here is the full definition of `Future`:

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

We have not seen a method definition like this before, where `self` has a type
annotation rather than simply being named like `self`, `mut self`, `&self`, or
`&mut self`. This syntax means that the method can only be called when the
instance of the type which implements `Future` is behind a `Pin` pointer type.
This syntax is not specific to `Pin`; it also works with `Box` and other smart
pointer types, and we will see it again in Chapter 18.

Here, the signature tells us that if we want to poll a future to check whether
it is `Pending` or `Ready(Output)`, the type which implements `Future` has to be
behind a `Pin` smart pointer type. Recalling that `.await` is implemented in
terms of calls to `poll()`, this starts to explain the error message we saw
above—but that was in terms of `Unpin`, not `Pin`. So what exactly are `Pin` and
`Unpin`, how do they relate, and why does `Future` need `self` to be in a `Pin`
type to call `poll`?

<!-- TODO: keep going here: define `Pin`. -->

Remember that any time you write a future, a runtime is ultimately responsible
for executing it. That means that an async block might outlive the function
where you write it, the same way a closure can. <!-- TODO: connect this to the
need for pinning. -->

`Unpin` is a marker trait, like `Send` and `Sync`, which we saw in Chapter 16.
Recall that marker traits have no functionality of their own. They exist only to
tell the compiler that it is safe to use the type which implements a given trait
in certain context. Just like `Send` and `Sync`, the compiler implements `Unpin`
automatically for most types.

`Unpin`’s job is to tell the compiler that a given type does *not* need to
uphold any particular guarantees about whether the value in question can be
moved. For example, if a future 

<!-- TODO: discussion of `Pin` -->

<!-- 
    The reason it gets weird to talk about is:
    
    - Nearly everything gets `Unpin` automatically because it is an auto trait.
    - Things which do not have to `impl !Unpin for TheType`.
    - But `!Unpin` actually means “must be pinned to be able to be used”.
    - So the actual situation is that the `Future` produced by an `async` block
      implements `!Unpin`, “not unpin”,

    My head hurts.
 -->

Now we know enough to understand the error message from above. The problem is
that the futures produced by an async block are *not* pinned by default.
Strictly: they implement `!Unpin` to opt out of being copyable by default the
way most types are. We need to pin them explicitly.

Now that we have an idea what that error message was telling us, we can finally
get our `join_all` call to compile! First, we need to explicitly annotate
`futures` as referring to a pinned `Box` of futures. Second, we actually need to
pin the futures, which we can do using the handy `Box::pin` API, which exists
for exactly this. Putting that together, we end up with the code in Listing
17-TODO.

<Listing number="17-TODO" caption="Using `Pin` and `Box::pin` to make the `Vec` type check" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

If we compile and run this, we finally get the output we hoped for:

<!-- TODO: listing for output.txt -->

```text
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

Phew!

There is a bit more we can explore here. For one thing, using `Pin<Box<T>>`
comes with a small amount of extra overhead from putting these futures on the
heap with `Box`—and we are only doing that to get the types to line up. We don’t
actually *need* the heap allocation, after all: these futures are local to this
particular function. As noted above, `Pin` is itself a smart pointer, so we can
get the benefit of having a single type in the `Vec`—the original reason we
reached for `Box`—without doing a heap allocation. We can use `Pin` directly
instead.

The `std::pin::pin` macro exists to do just that for values. However, we must
still be explicit about the type of the pinned reference; otherwise Rust will
still not know to interpret these as dynamic trait objects, which is what we
need them to be in the `Vec`. We therefore `pin!` each future when we define it,
and define `futures` as a `Vec` containing pinned mutable references to the
dynamic `Future` type.

<Listing number="17-TODO" caption="Using `Pin` directly with the `pin!` macro to avoid unnecessary heap allocations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

This keeps everything on the stack, and that is a nice little performance win,
but it is still a lot of explicit types, which is quite unusual for Rust! There
is another problem, too. We got this far by ignoring the fact that we might have
different `Output` types. For example, in Listing 17-TODO, the anonymous future
type for `a` implements `Future<Output = u32>` and the anonymous future type for
`b` implements `Future<Output = &str>`.

<Listing number="17-TODO" caption="" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

We can use `trpl::join!` to await them together, since it accepts two different
future types, but we cannot use `trpl::join_all` with these futures, because we
will never be able to make them have the same type. (This is the same as working
with any other type in Rust, though: futures are not special, even though we
have some nice syntax for working with them, and that is a good thing!) We have
a basic tradeoff here: we can either deal with a dynamic number of futures with
`join_all`, as long as they all have the same type, or we can deal with a
static number of futures with `join!`, and so on, 

<!--
    TODO: validate that this is, you know, true. It matches my own experience,
    but it is a fairly strong claim.
-->
In practice, you will usually work directly with `async` and `.await`, and only
as a secondary tool reach for the functions like `join` or `join_all`, or their
corresponding macro equivalents. These kinds of tools are really handy for
building frameworks, or especially when you are building a runtime itself.

### Select

Thus far, we have only used the `join` family of functions and macros. When we
“join” on some collection of futures, we require *all* of them to finish before
we move on. Sometimes, though, we only need *some* future from a set to finish
before we move on.

<!-- TODO: timeout! retries! etc. -->

[collections]: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types
[dyn]: https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
[futures]: /ch17-01-futures-and-syntax.html#futures
