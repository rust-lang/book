## Streams

In Chapter 13, we looked at the `Iterator` trait, and we saw how we could work
with a sequence of items in turn. So far in this chapter, we have mostly stuck
with individual futures. The one big exception was the async channel we used.
Recall how we used the receiver for our async channel in the [“Message
Passing”][17-02-messages] earlier in the chapter:

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:loop}}
```

This is because the `rx` receiver actually represents a *stream* of messages: a
sequence over time.

Unlike `Iterator` and `Future`, there is no definition of a `Stream` type in the
standard library yet <!-- TODO: verify before press time! --> but there *is* a
very common definition used throughout the ecosystem. If we start with the
definition of the `Iterator` and `Trait` types, we can figure out what a trait
that merges them together might look like.

The `Iterator` trait defines an associated type `Item` and a function `next`,
which produces `Some(Item)` until the underlying iterator is empty, and then
produces `None`.

<!-- TODO: support for no-listing listings in Listing? -->

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

As we saw earlier in this chapter, the `Future` trait defines an associated item
`Output` and a function `poll`, which produces `Poll::Pending` while waiting and
then `Poll::Ready(Output)` once the future is ready.

```rust
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

From `Iterator`, we have the idea of a sequence; its `next` method provides an
`Option<Self::Item>`. From `Future`, we have the idea of readiness; its `poll`
method provides a `Poll<Self::Output>`. To get a stream, a sequence of items
which become ready over time, we can define a `Stream` as a trait which has all
of those features put together:

* An associated type `Item` for the type of the items, just like in `Iterator`.
  Unlike in `Future`, where there was a single `Output`, we use `Item` here to
  indicate that it is more like `Iterator`: there may be zero to many of these.

* A method to get those items. We can call it `poll_next`, to make it clear that
  it is polling like a future and producing a sequence of items one after another
  like an iterator.

* A return type from `poll_next` which uses both `Poll` and `Option`. The outer
  type is `Poll`, since it has to be checked for readiness as a kind of future.
  The inner type is `Option`, since it needs to signal whether there are more
  messages, just like an iterator.

When we put those all together, we end up with the same definition for a
`Stream` trait as the one used by the Rust ecosystem:

```rust
trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

Something very similar to this will likely end up standardized as part of Rust’s
standard library, just the way `Future` was.

### Working With Streams

Now that we have seen the API, we can see that `rx.recv` is a good model for how
we will use streams.

> Note: As mentioned in the [“Message Passing”][17-02-messages] section, there
> is not yet an async version of `for` loops. There may well be in the future,
> though, and if so it will be built on something a lot like `Stream`.

<!--

- Motivation: you can do a lot with `while let` but it would be nice to be able
  to use `for` loops, even `async for`. (But we don’t get those from the
  ecosystem traits… so maybe only call them out in a `Note: …` context?)

- The basic API, with `poll_next()`, plus the “surface” syntax, `.next().await`.

- How to use it, with a worked example from .

-->

[17-02-messages]: /ch17-02-concurrency-with-async.md#message-passing
