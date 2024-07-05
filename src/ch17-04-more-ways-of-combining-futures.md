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

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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
swap back and forth eaach time one of them hits an await point. In this case, we
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

> Note: This means that async can be a useful tool even for CPU-bound tasks,
> depending on what else your program is doing, because it provides a useful
> tool for structuring the relationships between different parts of the program.
> This is a form of *cooperative multitasking*, where each future has both the
> power to determine when it hands over control via await points and therefore
> also the *responsibility* to avoid blocking for too long. This is how some
> Rust-based embedded operating systems work!

In real-world code, you will not usually be alternating function calls with
await points on every single line, of course. The underlying dynamic is an
important one to keep in mind, though!

### Building Our Own Async Abstractions

Many of these patterns are common enough to warrant abstracting over. For
example, the `trpl::timeout` function takes a `Duration` for the maximum time to
run, but also takes a future to run, and produces a new future you can await,
whose `Output` type is a `Result`. Listing 17-32 shows how we can use it. If
the passed-in future finishes first, the output result will be `Ok`, with the
result of that passed-in future. If the duration elapses before the passed-in
future finishes, the result will be `Err` with the duration that elapsed.

<Listing number="17-32" caption="Using `timeout` to run a slow operation with a time limit" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-32/src/main.rs:here}}
```

</Listing>

Here we were using the `timeout` supplied by `trpl`, but we do not have to. We
can implement it ourselves using `race` and `sleep`! To begin, let’s think about
the API of `timeout`:

- Its first parameter is a `std::time::Duration` which specifies the maximum
  time to wait.
- Its second parameter is the future to run.
- It returns a `Result`. If the future completes successfully, the `Result` will
  be `Ok` with the value produced by the future. If the timeout happens, the
  `Result` will be `Err` with the duration that the timeout waited for.

We can write the same signature ourselves, as in Listing 17-33.

<Listing number="17-33" caption="Defining the signature of `timeout`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-33/src/main.rs:declaration}}
```

</Listing>

Then, in the body of the function, we can `race` whatever future the caller
passes with a `sleep` future.

When we saw `race` earlier in Listing 17-20, we ignored its return type,
because we were just interested in seeing the behavior of `fast` and `slow` when
we ran the program. Here, though, its return value tells us whether the future
or the sleep finished first. With `race`, both futures passed as arguments can
legitimately “win,” so it does not make sense to use a `Result` to represent the
return type. Instead, it returns a similar type called `Either`. Like `Result`,
`Either` can be one of two types, but unlike `Result`, there is no notion of
success or failure baked into the type. Instead, it uses `Left` and `Right` to
indicate “one or the other”. Its implementation looks like this:

```rust
enum Either<A, B> {
    Left(A),
    Right(B)
}
```

In the case of `race` specifically, it returns `Left` if the first argument
finishes first, with that future’s output, and `Right` with the second future
argument’s output if *that* one finishes first.

```rust,ignore
match trpl::race(future_a, future_b).await {
    Either::Left(output_from_future_a) => /* ... */,
    Either::Right(output_from_future_b) => /* ... */,
}
```

That gives us enough to be able to implement `timeout` ourselves using `race`
and `sleep`.

<Listing number="17-34" caption="Defining `timeout` with `race` and `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-34/src/main.rs:timeout}}
```

</Listing>

Let’s walk through the details. Since we know from earlier that `race` is not
fair, and will prefer the first argument to the second, we pass it the future
first so it gets a chance to complete even if the caller passes in a very short
value for `max_time`. Then we match on the result of awaiting the `race`. If the
future passed in by the caller finished first, we will have `Left(output)`,
which we can return as a success with `Ok`. If the sleep finished first, we will
have `Right(())` instead, since `timeout` returns the unit type `()` if it
succeeds. We can ignore that `()` by using `_` and return `Err` with the
duration the user passed in instead. And that’s it!

Back in `main`, we can call this new `timeout` function exactly like we called
`trpl::timeout` before, but without the `trpl::` namespace:

<Listing number="17-35" caption="Using the `timeout` function we defined ourselves" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-35/src/main.rs:main}}
```

</Listing>

This pattern is quite common and useful. Futures compose with other futures, so
you can build really powerful tools using smaller async building blocks. For
example, you can use this same approach to combine timeouts with retries, and
in turn use those with things like network calls—the exact example we started
out with at the beginning of the chapter!

Over the last two sections, we have seen how to work with multiple futures at
the same time. Up next, let’s look at how we can work with multiple futures in a
sequence over time, with *streams*.

[futures]: ch17-01-futures-and-syntax.html#what-are-futures
