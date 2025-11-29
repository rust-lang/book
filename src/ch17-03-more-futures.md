<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>

### Yielding Control to the Runtime

Recall from the [“Our First Async Program”][async-program]<!-- ignore -->
section that at each await point, Rust gives a runtime a chance to pause the
task and switch to another one if the future being awaited isn’t ready. The
inverse is also true: Rust _only_ pauses async blocks and hands control back to
a runtime at an await point. Everything between await points is synchronous.

That means if you do a bunch of work in an async block without an await point,
that future will block any other futures from making progress. You may sometimes
hear this referred to as one future _starving_ other futures. In some cases,
that may not be a big deal. However, if you are doing some kind of expensive
setup or long-running work, or if you have a future that will keep doing some
particular task indefinitely, you’ll need to think about when and where to hand
control back to the runtime.

Let’s simulate a long-running operation to illustrate the starvation problem,
then explore how to solve it. Listing 17-14 introduces a `slow` function.

<Listing number="17-14" caption="Using `thread::sleep` to simulate slow operations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

This code uses `std::thread::sleep` instead of `trpl::sleep` so that calling
`slow` will block the current thread for some number of milliseconds. We can
use `slow` to stand in for real-world operations that are both long-running and
blocking.

In Listing 17-15, we use `slow` to emulate doing this kind of CPU-bound work in
a pair of futures.

<Listing number="17-15" caption="Calling the `slow` function to simulate slow operations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

Each future hands control back to the runtime only _after_ carrying out a bunch
of slow operations. If you run this code, you will see this output:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
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

As with Listing 17-5 where we used `trpl::select` to race futures fetching two
URLs, `select` still finishes as soon as `a` is done. There’s no interleaving
between the calls to `slow` in the two futures, though. The `a` future does all
of its work until the `trpl::sleep` call is awaited, then the `b` future does
all of its work until its own `trpl::sleep` call is awaited, and finally the
`a` future completes. To allow both futures to make progress between their slow
tasks, we need await points so we can hand control back to the runtime. That
means we need something we can await!

We can already see this kind of handoff happening in Listing 17-15: if we
removed the `trpl::sleep` at the end of the `a` future, it would complete
without the `b` future running _at all_. Let’s try using the `trpl::sleep`
function as a starting point for letting operations switch off making progress,
as shown in Listing 17-16.

<Listing number="17-16" caption="Using `trpl::sleep` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

We’ve added `trpl::sleep` calls with await points between each call to `slow`.
Now the two futures’ work is interleaved:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
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
have done that after every call to `slow`, but we could break up the work in
whatever way makes the most sense to us.

We don’t really want to _sleep_ here, though: we want to make progress as fast
as we can. We just need to hand back control to the runtime. We can do that
directly, using the `trpl::yield_now` function. In Listing 17-17, we replace
all those `trpl::sleep` calls with `trpl::yield_now`.

<Listing number="17-17" caption="Using `yield_now` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

This code is both clearer about the actual intent and can be significantly
faster than using `sleep`, because timers such as the one used by `sleep` often
have limits on how granular they can be. The version of `sleep` we are using,
for example, will always sleep for at least a millisecond, even if we pass it a
`Duration` of one nanosecond. Again, modern computers are _fast_: they can do a
lot in one millisecond!

This means that async can be useful even for compute-bound tasks, depending on
what else your program is doing, because it provides a useful tool for
structuring the relationships between different parts of the program (but at a
cost of the overhead of the async state machine). This is a form of
_cooperative multitasking_, where each future has the power to determine when
it hands over control via await points. Each future therefore also has the
responsibility to avoid blocking for too long. In some Rust-based embedded
operating systems, this is the _only_ kind of multitasking!

In real-world code, you won’t usually be alternating function calls with await
points on every single line, of course. While yielding control in this way is
relatively inexpensive, it’s not free. In many cases, trying to break up a
compute-bound task might make it significantly slower, so sometimes it’s better
for _overall_ performance to let an operation block briefly. Always
measure to see what your code’s actual performance bottlenecks are. The
underlying dynamic is important to keep in mind, though, if you _are_ seeing a
lot of work happening in serial that you expected to happen concurrently!

### Building Our Own Async Abstractions

We can also compose futures together to create new patterns. For example, we can
build a `timeout` function with async building blocks we already have. When
we’re done, the result will be another building block we could use to create
still more async abstractions.

Listing 17-18 shows how we would expect this `timeout` to work with a slow
future.

<Listing number="17-18" caption="Using our imagined `timeout` to run a slow operation with a time limit" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
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

Listing 17-19 shows this declaration.

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-19" caption="Defining the signature of `timeout`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

That satisfies our goals for the types. Now let’s think about the _behavior_ we
need: we want to race the future passed in against the duration. We can use
`trpl::sleep` to make a timer future from the duration, and use `trpl::select`
to run that timer with the future the caller passes in.

In Listing 17-20, we implement `timeout` by matching on the result of awaiting
`trpl::select`.

<Listing number="17-20" caption="Defining `timeout` with `select` and `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

The implementation of `trpl::select` is not fair: it always polls arguments in
the order in which they are passed (other `select` implementations will
randomly choose which argument to poll first). Thus, we pass `future_to_try` to
`select` first so it gets a chance to complete even if `max_time` is a very
short duration. If `future_to_try` finishes first, `select` will return `Left`
with the output from `future_to_try`. If `timer` finishes first, `select` will
return `Right` with the timer’s output of `()`.

If the `future_to_try` succeeds and we get a `Left(output)`, we return
`Ok(output)`. If the sleep timer elapses instead and we get a `Right(())`, we
ignore the `()` with `_` and return `Err(max_time)` instead.

With that, we have a working `timeout` built out of two other async helpers. If
we run our code, it will print the failure mode after the timeout:

```text
Failed after 2 seconds
```

Because futures compose with other futures, you can build really powerful tools
using smaller async building blocks. For example, you can use this same
approach to combine timeouts with retries, and in turn use those with
operations such as network calls (such as those in Listing 17-5).

In practice, you’ll usually work directly with `async` and `await`, and
secondarily with functions such as `select` and macros such as the `join!`
macro to control how the outermost futures are executed.

We’ve now seen a number of ways to work with multiple futures at the same time.
Up next, we’ll look at how we can work with multiple futures in a sequence over
time with _streams_.

[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
