## TODO: Title This Section!

### Race

Thus far, we have only used the `join` family of functions and macros. When we
“join” on some collection of futures, we require *all* of them to finish before
we move on. Sometimes, though, we only need *some* future from a set to finish
before we move on—kind of like racing one future against another. This operation
is often named `race` for exactly that reason.

In Listing 17-TODO, we use `race` to run two futures, `slow` and `fast` against
each other. First, we introduce the two futures. Each one prints a message when
it starts running, pauses for some amount of time by calling and awaiting
`sleep`, and then prints another message when it finishes. Then we pass both to
`trpl::race` and wait for one of them to finish. (The outcome here won’t be too
surprising: `fast` wins!)

<Listing number="17-TODO" caption="Using `race` to get the result of whichever future finishes first" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-race/src/main.rs:here}}
```

</Listing>

One other thing to notice: if you flip the order of the arguments to `race`, the
order of the start messages changes, but the `fast` future always completes
first. That is because the implementation of this particular `race` function is
not *fair*. It always runs the futures passed as arguments in the order they are
passed. That means everything up to the first `.await` in a given future will
run before *any* of the other future gets a chance to run. Other implementations
*are* fair, and will randomly choose which future to start first.

This dynamic is important to keep in mind! An async runtime can only switch
which future it is executing at await points. That means if you load up a bunch
of really expensive work in an async function, it will block any other futures
from making progress. (You may sometimes hear this referred to as one future
*starving* other futures. The same thing applies to threads, too!) We can work
around this using the `sleep` function, as in Listing 17-TODO.

<!--
    TODO: maybe tweak that example so that it actually shows the difference
    between doing all of the work in a single chunk vs. breaking it into
    separate chunks which can make progress.
-->

<Listing number="17-TODO" caption="Using `sleep` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-yield-a-sleep/src/main.rs:here}}
```

</Listing>

However, we do not actually need to sleep to accomplish this. We just need to
hand back control to the runtime. We can actually *yield* control back to the
runtime, using a function named `yield_now`. It does just what it says: hands
control back to the runtime, so that the runtime can check whether any other
tasks are ready to make progress.

<Listing number="17-TODO" caption="Using `yield_now` to let operations switch off making progress" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-yield-b-yield/src/main.rs:here}}
```

</Listing>

This is both clearer about the actual intent and can be significantly faster
than using `sleep`, because timers like the one used by `sleep` often have
limits to how granular they can be. The version of `sleep` we are using, for
example, will always sleep for at least a millisecond, even if we pass it a
`Duration` of one nanosecond. Again, modern computers are *fast*: they can do a
lot in one millisecond!

You can see this for yourself by comparing what happens if you change Listings
17-TODO and 17-TODO to both do 100 or 1,000 iterations instead of just 5. The
version with `yield_now` is *way* faster!

<!-- TODO: make this its own listing? -->

### Building Our Own Async Abstractions

Many of these patterns are common enough to warrant abstracting over. For
example, the `trpl::timeout` function takes a `Duration` for the maximum time to
run, but also takes a future to run, and produces a new future you can await,
whose `Output` type is a `Result`. Listing 17-TODO shows how we can use it. If
the passed-in future finishes first, the output result will be `Ok`, with the
result of that passed-in future. If the duration elapses before the passed-in
future finishes, the result will be `Err` with the duration that elapsed.

<Listing number="17-TODO" caption="Using `timeout` to run a slow operation with a time limit" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-timeout-a/src/main.rs:here}}
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

We can write the same signature ourselves, as in Listing 17-TODO.

<Listing number="17-TODO" caption="Defining the signature of `timeout`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-timeout-final/src/main.rs:declaration}}
```

</Listing>

What about the body of the function? Here, we can `race` whatever future the
caller passes with a `sleep` future.

When we saw `race` earlier in Listing 17-TODO, we ignored its return type,
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
match race(future_a, future_b).await {
    Either::Left(output_from_future_a) => /* ... */,
    Either::Right(output_from_future_b) => /* ... */,
}
```

That gives us enough to be able to implement `timeout` ourselves using `race`
and `sleep`.

<Listing number="17-TODO" caption="Defining `timeout` with `race` and `sleep`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-timeout-final/src/main.rs:timeout}}
```

</Listing>

Let’s walk through the details. Since we know from earlier that `race` is not
fair, and will prefer the first argument to the second, we pass it the future
first so it gets a chance to complete even if the caller passes in a very short
value for `max_time`. Then we match on the result of awaiting the `race`. If the
future passed in by the caller finished first, we will have
`Either::Left(output)`, which we can return as a success with `Ok`. If the sleep
finished first, we will have `Either::Right(())` instead, since `timeout`
returns the unit type `()` if it succeeds. We can ignore that `()` by using `_`
and return `Err` with the duration the user passed in instead. And that’s it!

Back in `main`, we can call this new `timeout` function exactly like we called
`trpl::timeout` before.

[collections]: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types
[dyn]: https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
[futures]: /ch17-01-futures-and-syntax.html#futures
