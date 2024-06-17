## More Ways of Combining Futures

Thus far, we have only used the `join` family of functions and macros. When we
“join” on some collection of futures, we require *all* of them to finish before
we move on. Sometimes, though, we only need *some* future from a set to finish
before we move on—kind of like racing one future against another. This operation
is often named `race` for exactly that reason.

In Listing 17-26, we use `race` to run two futures, `slow` and `fast`, against
each other. Each one prints a message when it starts running, pauses for some
amount of time by calling and awaiting `sleep`, and then prints another message
when it finishes. Then we pass both to `trpl::race` and wait for one of them to
finish. (The outcome here won’t be too surprising: `fast` wins!)

<Listing number="17-26" caption="Using `race` to get the result of whichever future finishes first" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

One other thing to notice: if you flip the order of the arguments to `race`, the
order of the start messages changes, even though the `fast` future always
completes first. That is because the implementation of this particular `race`
function is not *fair*. It always runs the futures passed as arguments in the
order they are passed. Other implementations *are* fair, and will randomly
choose which future to start first.

Regardless of whether the implementation of race we are using is fair, though,
*one* of the futures will run up to the first `.await` in its body before
another task can start.

To see why, recall from our discussion in [“What Are Futures?”][futures] that
Rust compiles async blocks in a way that hands control back to the async runtime
at each await point. That has an important corollary: async runtimes can only
switch which future they are executing at await points. Everything in between
await points is just normal synchronous Rust code. That means if you do a bunch
of really expensive work in an async function without an `.await`, that future
will block any other futures from making progress.

> Note: You may sometimes hear this referred to as one future *starving* other
> futures. The same thing applies to threads, too!

That has another important consequence for using `race`, `join`, and other such
helpers. *Some* future is going to run first, and everything up to the first
await point in that future will run before any part of any other future gets a
chance to run. For simple code, that may not be a big deal. However, if you are
doing some kind of expensive setup or long-running work, or if you have a future
which will keep doing some particular task indefinitely, you will need to think
about when and where to hand control back to the runtime.

### Yielding

Let’s consider a long-running operation. Here, we will simulate it using `sleep`
inside the function, but in the real world it could be any of operations which
might take a while, and which, critically, are *blocking*. Our `slow` helper
function “slow” will just take a number of milliseconds to run, and sleep the
thread for that long. This is intentionally not an async function, because the
idea is to represent work that is *not* async.

<Listing number="17-27" caption="Using `thread::sleep` to simulate slow operations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:slow}}
```

</Listing>

In Listing 17-28, we use `slow` to emulate doing this kind of CPU-bound work in
a pair of futures. To begin, each future only hands control back to the runtime
*after* carrying out a bunch of slow operations.

<Listing number="17-28" caption="Using `thread::sleep` to simulate slow operations" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:slow-futures}}
```

</Listing>

If you run this, you will see this output:

```console
{{#include ../listings/ch17-async-await/listing-17-28/output.txt}}
```

As with our earlier example, `race` still finishes when `a` finishes. There is
no interleaving between the two futures, though. The `a` future does all of its
work until the `trpl::sleep` call is awaited, then the `b` future does all of
its work until its own `trpl::sleep` call is awaited, and then the `a` future
completes. It would be better if both futures could make progress between their
slow tasks. We need some way to hand control back to the runtime there—and we
know that await points are the way to do that. However, that means we need
something we can await!

However, we can also see the handoff happening in this very example: if we
removed the `trpl::sleep` at the end of the `a` future, it would complete
without the `b` future running *at all*. Given that, maybe we could use the
`sleep` function as a starting point, as in Listing 17-29.

<Listing number="17-29" caption="Using `sleep` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:here}}
```

</Listing>

Now the two futures’ work is interleaved, as we can see if we run it.

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
'a' started.
'a' ran for 300ms
'b' started.
'b' ran for 750ms
'a' ran for 100ms
'b' ran for 100ms
'a' ran for 200ms
'b' ran for 150ms
'a' ran for 900ms
'b' ran for 350ms
'a' finished.
```


The `a` future still runs for a bit before handing off control to `b`, because
it has some expensive work to do up front, but after that they just swap back
and forth every time one of them hits an await point. In this case, we have done
that after every call to `slow`, but we could break up the work however makes
the most sense to us.

However, we do not actually need to sleep to accomplish this. We just need to
hand back control to the runtime. We can actually *yield* control back to the
runtime, using a function named `yield_now`. It does just what it says: hands
control back to the runtime, so that the runtime can check whether any other
tasks are ready to make progress.

<Listing number="17-30" caption="Using `yield_now` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-30/src/main.rs:here}}
```

</Listing>

This is both clearer about the actual intent and can be significantly faster
than using `sleep`, because timers like the one used by `sleep` often have
limits to how granular they can be. The version of `sleep` we are using, for
example, will always sleep for at least a millisecond, even if we pass it a
`Duration` of one nanosecond. Again, modern computers are *fast*: they can do a
lot in one millisecond!

You can see this for yourself by setting up a little benchmark, like the one in
Listing 17-31. (This is not an especially rigorous way to do performance
testing, but it suffices to show the difference here.) Here, we skip all the
status printing, pass a one-nanosecond `Duration` to `sleep`, let each future
run by itself so that they do not interfere with each other, and get rid of all
the status printing that we did to see the back-and-forth between tasks in
Listings 17-29 and 17-30. Then we run for 1,000 iterations and see how long
`sleep` takes vs. `yield_now`.

<Listing number="17-31" caption="Comparing the performance of `sleep` and `yield_now`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-31/src/main.rs:here}}
```

</Listing>

The version with `yield_now` is *way* faster!

> Note: This also means that async can be a useful tool even for CPU-bound
> tasks, depending on what else your program is doing, because it provides a
> useful tool for structuring the relationships between different parts of the
> program. This is a form of *cooperative multitasking*, where each future has
> both the power to determine when it hands over control via await points and
> therefore also the *responsibility* to avoid blocking for too long. This is
> how some Rust-based embedded operating systems work!

In real-world code, you will not usually be alternative regular function calls
with await points on every single line, of course. The underlying dynamic is an
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

When we saw `race` earlier in Listing 17-26, we ignored its return type,
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

Up next, let’s look at how we can work with *sequences* of futures.

[futures]: ch17-01-futures-and-syntax.html#what-are-futures
