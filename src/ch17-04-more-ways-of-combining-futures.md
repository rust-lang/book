## More Ways of Combining Futures

When we “join” futures with the `join` family of functions and macros, we
require *all* of them to finish before we move on. Sometimes, though, we only
need *some* future from a set to finish before we move on—kind of like racing
one future against another. This operation is often named `race` for exactly
that reason.

In Listing 17-20, we use `race` to run two futures, `slow` and `fast`, against
each other. Each one prints a message when it starts running, pauses for some
amount of time by calling and awaiting `sleep`, and then prints another message
when it finishes. Then we pass both to `trpl::race` and wait for one of them to
finish. (The outcome here won’t be too surprising: `fast` wins!)

<Listing number="17-20" caption="Using `race` to get the result of whichever future finishes first" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

Notice that if you flip the order of the arguments to `race`, the order of the
“started” messages changes, even though the `fast` future always completes
first. That is because the implementation of this particular `race` function is
not fair. It always runs the futures passed as arguments in the order they are
passed. Other implementations *are* fair, and will randomly choose which future
to poll first. Regardless of whether the implementation of race we are using is
fair, though, *one* of the futures will run up to the first `.await` in its body
before another task can start.

Recall from [“What Are Futures?”][futures] that at each await point, Rust pauses
the async block and hands control back to a runtime. The inverse is also true:
Rust *only* pauses async blocks and hands control back to a runtime at an await
point. Everything between await points is synchronous.

That means if you do a bunch of work in an async block without an await point,
that future will block any other futures from making progress. (You may
sometimes hear this referred to as one future *starving* other futures. And this
applies to threads, too!) In many cases, that may not be a big deal. However, if
you are doing some kind of expensive setup or long-running work, or if you have
a future which will keep doing some particular task indefinitely, you will need
to think about when and where to hand control back to the runtime.

But *how* would you hand control back to the runtime in those cases?

### Yielding

Let’s simulate a long-running operation. Listing 17-21 introduces a `slow`
function which uses `std::thread::sleep` to block the current thread for some
number of milliseconds. We can use `slow` to stand in for real-world operations
which are both long-running and blocking.

<Listing number="17-21" caption="Using `thread::sleep` to simulate slow operations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:slow}}
```

</Listing>

In Listing 17-22, we use `slow` to emulate doing this kind of CPU-bound work in
a pair of futures. To begin, each future only hands control back to the runtime
*after* carrying out a bunch of slow operations.

<Listing number="17-22" caption="Using `thread::sleep` to simulate slow operations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow-futures}}
```

</Listing>

If you run this, you will see this output:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-22/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

As with our earlier example, `race` still finishes as soon as `a` is done. There
is no interleaving between the two futures, though. The `a` future does all of
its work until the `trpl::sleep` call is awaited, then the `b` future does all
of its work until its own `trpl::sleep` call is awaited, and then the `a` future
completes. To allow both futures to make progress between their slow tasks, we
need await points so we can hand control back to the runtime. That means we need
something we can await!

We can already see this kind of handoff happening in Listing 17-22: if we
removed the `trpl::sleep` at the end of the `a` future, it would complete
without the `b` future running *at all*. Maybe we could use the `sleep` function
as a starting point?

<Listing number="17-23" caption="Using `sleep` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:here}}
```

</Listing>

In Listing 17-23, we add `trpl::sleep` calls with await points between each call
to `slow`. Now the two futures’ work is interleaved:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-23/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

The `a` future still runs for a bit before handing off control to `b`, because
it calls `slow` before ever calling `trpl::sleep`, but after that the futures
swap back and forth each time one of them hits an await point. In this case, we
have done that after every call to `slow`, but we could break up the work
however makes the most sense to us.

We do not really want to *sleep* here, though: we want to make progress as fast
as we can. We just need to hand back control to the runtime. We can do that
directly, using the `yield_now` function. In Listing 17-24, we replace all those
`sleep` calls with `yield_now`.

<Listing number="17-24" caption="Using `yield_now` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:yields}}
```

</Listing>

This is both clearer about the actual intent and can be significantly faster
than using `sleep`, because timers like the one used by `sleep` often have
limits to how granular they can be. The version of `sleep` we are using, for
example, will always sleep for at least a millisecond, even if we pass it a
`Duration` of one nanosecond. Again, modern computers are *fast*: they can do a
lot in one millisecond!

You can see this for yourself by setting up a little benchmark, like the one in
Listing 17-25. (This is not an especially rigorous way to do performance
testing, but it suffices to show the difference here.) Here, we skip all the
status printing, pass a one-nanosecond `Duration` to `sleep`, let each future
run by itself so that they do not interfere with each other, and get rid of all
the status printing that we did to see the back-and-forth between tasks in
Listings 17-23 and 17-24. Then we run for 1,000 iterations and see how long
`sleep` takes vs. `yield_now`.

<Listing number="17-25" caption="Comparing the performance of `sleep` and `yield_now`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:here}}
```

</Listing>

The version with `yield_now` is *way* faster!

This means that async can be useful even for CPU-bound tasks, depending on what
else your program is doing, because it provides a useful tool for structuring
the relationships between different parts of the program. This is a form of
*cooperative multitasking*, where each future has both the power to determine
when it hands over control via await points. Each future therefore also has the
*responsibility* to avoid blocking for too long. In some Rust-based embedded
operating systems, this is the *only* kind of multitasking!

In real-world code, you will not usually be alternating function calls with
await points on every single line, of course. The underlying dynamic is an
important one to keep in mind, though!

### Building Our Own Async Abstractions

We can also compose futures together to create new patterns. For example, we can
build a `timeout` function with async building blocks we already have. When we
are done, the result will be another building block we could use to build up yet
further async abstractions.

Listing 17-26 shows how we would expect this `timeout` to work with a slow
future.

<Listing number="17-26" caption="Using our imagined `timeout` to run a slow operation with a time limit" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

Let’s implement this! To begin, let’s think about the API for `timeout`:

- It needs to be an async function itself so we can await it.
- Its first parameter should be a future to run. We can make it generic to allow
  it to work with any future.
- Its second parameter will be the maximum time to wait. If we use a `Duration`,
  that will make it easy to pass along to `trpl::sleep`.
- It should return a `Result`. If the future completes successfully, the
  `Result` will be `Ok` with the value produced by the future. If the timeout
  elapses first, the `Result` will be `Err` with the duration that the timeout
  waited for.

Listing 17-27 shows this declaration.

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-27" caption="Defining the signature of `timeout`" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:declaration}}
```

</Listing>

The types line up now, so let’s think about the *behavior* we need. We want to
race the future passed in against the duration. We can use `trpl::sleep` to make
a timer future from the duration, and use `trpl::race` to run the future and the
timer against each other.

When we saw `race` earlier in Listing 17-20, we ignored its return type, because
we were just interested in seeing the behavior of `fast` and `slow` when we ran
the program. Here, though, its return value tells us whether the future or the
sleep finished first. With `race`, both futures passed as arguments can
legitimately “win,” so it does not make sense to use a `Result` to represent the
return type. Instead, it returns a similar type called `Either`. Unlike
`Result`, there is no notion of success or failure baked into `Either`. Instead,
it uses `Left` and `Right` to indicate “one or the other”:

```rust
enum Either<A, B> {
    Left(A),
    Right(B)
}
```

The `race` function returns `Left` if the first argument finishes first, with
that future’s output, and `Right` with the second future argument’s output if
*that* one finishes first. We also know that `race` is not fair, and polls
arguments in the order they are passed. For `timeout`, we pass the future to
`race` first so it gets a chance to complete even if `max_time` is a very short
duration. If `future` finishes first, `race` will return `Left` with the output
from `future`. If `timer` finishes first, `race` will return `Right` with the
timer’s output of `()`.

In Listing 17-28, we match on the result of awaiting `trpl::race`. If the
future succeeded and we get a `Left(output)`, we return `Ok(output)`. If the
sleep timer elapsed instead and we get a `Right(())`, we ignore the `()` with
`_` and return `Err(duration)` instead.

<Listing number="17-28" caption="Defining `timeout` with `race` and `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:implementation}}
```

</Listing>

With that, we have a working `timeout`, built out of two other async helpers. If
we run our code, it will print the failure mode after the timeout:

```text
Failed after 2 seconds
```

Because futures compose with other futures, you can build really powerful tools
using smaller async building blocks. For example, you can use this same approach
to combine timeouts with retries, and in turn use those with things like network
calls—one of the examples from the beginning of the chapter!

Over the last two sections, we have seen how to work with multiple futures at
the same time. Up next, let’s look at how we can work with multiple futures in a
sequence over time, with *streams*.

[futures]: ch17-01-futures-and-syntax.html#what-are-futures
