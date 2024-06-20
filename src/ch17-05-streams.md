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
  it is polling like a future and producing a sequence of items one after
  another, just like an iterator.

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
standard library, just the way `Future` was. In the meantime, it is part of the
toolkit of most runtimes, so you can rely on it, and everything we cover below
should generally apply!

### Working With Streams

We *could* work directly in terms of the `poll_next` API by hand-writing our own
`Stream` state machines. However, just as we do not generally work with futures
directly via their `poll` method, we generally also do not work directly with
the `poll_next` method for streams. Instead, we usually use a `next` method,
which is defined roughly like this:

```rust
trait Stream {
    async fn next(&mut self) -> Option<Self::Item>;
}
```

<!--
TODO: update this if/when tokio/etc. update their MSRV and switch to using AFIT,
since the lack thereof is the reason they do not yet have this.
-->

> Note: The actual definition we will use looks slightly different than this,
> because it supports versions of Rust which did not yet support using async
> functions in traits. As a result, it looks like this:
>
> ```rust
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> That `Next` type is just a simple `struct` which implements `Future`, so that
> `.await` can work with this!

Working with this API will be kind of like working with iterators without the
convenience of a `for` loop. In fact, it will look a lot like the way we used
`rx.recv` back in the [“Message Passing”][17-02-messages] section, using `while
let` loops.

Let’s start with a very simple example: using an iterator *as* a stream. Let’s
start by creating a range of numbers, including every integer from 1 to 100,
using the `..` range operator. Then we can double all of those values with the
`map` method, as Listing 17-36 shows:

<Listing number="17-36" caption="Creating an iterator ranging over the values from 1 to 100" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-36/src/main.rs:range}}
```

</Listing>

We can convert this iterator to a stream using the `trpl::stream_from_iter`
function.

<Listing number="17-37" caption="Converting an iterator to a stream with `trpl::stream_from_iter`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-37/src/main.rs:stream}}
```

</Listing>

This gives us the stream. Now, to work with it, we want to use the `next` method
with a `while let` loop as described above, as in Listing 17-38:

<Listing number="17-38" caption="Trying to use the `next` method on the newly-created `stream`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-38/src/main.rs:next}}
```

</Listing>

Unfortunately, this does not yet work. When we try to run the code, it does not
compile. Instead, as we can see in the output, it reports that there is no
`next` method available.

```console
{{#include ../listings/ch17-async-await/listing-17-38/output.txt}}
```

As the output suggests, the problem is that we need the right trait in scope to
be able to use it. In this case, that trait is `StreamExt`. (The `Ext` there is
for “extension”: this is a common pattern in the Rust community for extending
one trait with another.) `StreamExt` is automatically implemented for every type
which implements `Stream`, but they are separated out so that the community can
iterate on the foundational trait distinctly from the convenience APIs. All we
need to do, then, is add a `use` statement for `trpl::StreamExt`, as in Listing
17-39.

<Listing number="17-39" caption="Successfully using an iterator as the basis for a stream" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-39/src/main.rs:all}}
```

</Listing>

 With all those pieces put together, things work the way we want! There is a lot
 of output, though, since we told it to print all of the 100 numbers in the
 iterator. We can filter that down, to, say, multiples of three and five by using
 the `filter` method, which conveniently also comes from `StreamExt`.

 <Listing number="17-40" caption="Using" file-name="src/main.rs">

 ```rust,ignore,does_not_compile
 {{#rustdoc_include ../listings/ch17-async-await/listing-17-40/src/main.rs:filter}}
 ```

 </Listing>

Of course, in the real world, the only time we would be directly converting an
iterator to a stream like this is to help break up longer chunks of  work, like
we discussed in the previous section. There are more interesting things we can
do with streams, though!

For one thing, lots of things are naturally represented as streams—items
becoming available in a queue over time, for example, or working with more data
than can fit in a computer’s memory by only pulling chunks of it from the file
system at a time, or data arriving over the network over time. For another
thing, since streams are futures, we can use them with any other kind of
future, and we can combine them in interesting ways.

This is the kind of thing we might actually do to help break up longer chunks of
work, like we discussed in the previous section—though of course, presumably
with more interesting iterators than this one!

<!--
  - Maybe motivate with “doing some work” where that work is simple enough in
    the example but demonstrates how it could be the kind of thing that is *not*
    trivial and takes some time.
-->

[17-02-messages]: /ch17-02-concurrency-with-async.md#message-passing
