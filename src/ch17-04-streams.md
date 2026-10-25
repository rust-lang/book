<!-- Old headings. Do not remove or links may break. -->

<a id="streams"></a>

## Streams: Futures in Sequence

Recall how we used the receiver for our async channel earlier in this chapter
in the [“Message Passing”][17-02-messages]<!-- ignore --> section. The async
`recv` method produces a sequence of items over time. This is an instance of a
much more general pattern known as a _stream_. Many concepts are naturally
represented as streams: items becoming available in a queue, chunks of data
being pulled incrementally from the filesystem when the full data set is too
large for the computer’s memory, or data arriving over the network over time.
Because streams are futures, we can use them with any other kind of future and
combine them in interesting ways. For example, we can batch up events to avoid
triggering too many network calls, set timeouts on sequences of long-running
operations, or throttle user interface events to avoid doing needless work.

We saw a sequence of items back in Chapter 13, when we looked at the Iterator
trait in [“The Iterator Trait and the `next` Method”][iterator-trait]<!--
ignore --> section, but there are two differences between iterators and the
async channel receiver. The first difference is time: iterators are
synchronous, while the channel receiver is asynchronous. The second difference
is the API. When working directly with `Iterator`, we call its synchronous
`next` method. With the `trpl::Receiver` stream in particular, we called an
asynchronous `recv` method instead. Otherwise, these APIs feel very similar,
and that similarity isn’t a coincidence. A stream is like an asynchronous form
of iteration. Whereas the `trpl::Receiver` specifically waits to receive
messages, though, the general-purpose stream API is much broader: it provides
the next item the way `Iterator` does, but asynchronously.

The similarity between iterators and streams in Rust means we can actually
create a stream from any iterator. As with an iterator, we can work with a
stream by calling its `next` method and then awaiting the output, as in Listing
17-21, which won’t compile yet.

<Listing number="17-21" caption="Creating a stream from an iterator and printing its values" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

We start with an array of numbers, which we convert to an iterator and then
call `map` on to double all the values. Then we convert the iterator into a
stream using the `trpl::stream_from_iter` function. Next, we loop over the
items in the stream as they arrive with the `while let` loop.

Unfortunately, when we try to run the code, it doesn’t compile but instead
reports that there’s no `next` method available:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

As this output explains, the reason for the compiler error is that we need the
right trait in scope to be able to use the `next` method. Given our discussion
so far, you might reasonably expect that trait to be `Stream`, but it’s
actually `StreamExt`. Short for _extension_, `Ext` is a common pattern in the
Rust community for extending one trait with another.

The `Stream` trait defines a low-level interface that effectively combines the
`Iterator` and `Future` traits. `StreamExt` supplies a higher-level set of APIs
on top of `Stream`, including the `next` method as well as other utility
methods similar to those provided by the `Iterator` trait. `Stream` and
`StreamExt` are not yet part of Rust’s standard library, but most ecosystem
crates use similar definitions.

The fix to the compiler error is to add a `use` statement for
`trpl::StreamExt`, as in Listing 17-22.

<Listing number="17-22" caption="Successfully using an iterator as the basis for a stream" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

With all those pieces put together, this code works the way we want! What’s
more, now that we have `StreamExt` in scope, we can use all of its utility
methods, just as with iterators.

[17-02-messages]: ch17-02-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
