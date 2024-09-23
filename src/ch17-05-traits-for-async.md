## Digging Into the Traits for Async

Throughout the chapter, we have used the `Future`, `Pin`, `Unpin`, `Stream`, and
`StreamExt` traits in various ways. So far, though, we have avoided digging too
far into the details of how they work or how they fit together. Much of the time
when writing Rust day to day, this is fine. Sometimes, though, you will hit
situations where understanding a few more of these details matters. In this
section, we will dig down *enough* further to help with those situations—while
still leaving the *really* deep dive for other documentation!


### Future

Back in [Futures and the Async Syntax][futures-syntax], we noted that `Future`
is a trait. Let’s start by taking a closer look at how it works. Here is how
Rust defines a `Future`:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

That trait definition includes a bunch of new types and also some syntax we have
not seen before, so let’s walk through the definition piece by piece.

First, `Future`’s associated type `Output` says what the future resolves to.
This is analogous to the `Item` associated type for the `Iterator` trait.
Second, `Future` also has the `poll` method, which takes a special `Pin`
reference for its `self` parameter and a mutable reference to a `Context` type,
and returns a `Poll<Self::Output>`. We will talk a little more about `Pin` and
`Context` later in the section. For now, let’s focus on what the method returns,
the `Poll` type:

```rust
enum Poll<T> {
    Ready(T),
    Pending
}
```

This `Poll` type is a lot like an `Option`: it has one variant which has a value
(`Ready(T)`), and one which does not (`Pending`). It means something quite
different, though! The `Pending` variant indicates that the future still has
work to do, so the caller will need to check again later. The `Ready` variant
indicates that the `Future` has finished its work and the `T` value is
available.

> Note: With most futures, the caller should not call `poll` again after the
> future has returned `Ready`. Many futures will panic if polled again after
> becoming ready! Futures which are safe to poll again will say so explicitly in
> their documentation. This is similar to how `Iterator::next` behaves!

Under the hood, when you call `.await`, Rust compiles that to code which calls
`poll`, kind of (although not exactly <!-- TODO: describe `IntoFuture`? -->)
like this:

```rust,ignore
match hello("async").poll() {
    Ready(_) => {
        // We’re done!
    }
    Pending => {
        // But what goes here?
    }
}
```

What should we do when the `Future` is still `Pending`? We need some way to try
again… and again, and again, until the future is finally ready. In other words,
a loop:

```rust,ignore
let hello_fut = hello("async");
loop {
    match hello_fut.poll() {
        Ready(_) => {
            break;
        }
        Pending => {
            // continue
        }
    }
}
```

If Rust compiled it to exactly that code, though, every `.await` would be
blocking—exactly the opposite of what we were going for! Instead, Rust needs
makes sure that the loop can hand off control to something which can pause work
on this future and work on other futures and check this one again later. That
“something” is an async runtime, and this scheduling and coordination work is
one of the main jobs for a runtime.

Recall our description (in the [Counting][counting] section) of waiting on
`rx.recv`. The `recv` call returns a `Future`, and awaiting it polls it. In our
initial discussion, we noted that a runtime will pause the future until it is
ready with either `Some(message)` or `None` when the channel closes. With our
deeper understanding of `Future` in place, and specifically `Future::poll`, we
can see how that works. The runtime knows the future is not ready when it
returns `Poll::Pending`. Conversely, the runtime knows the future is ready and
advances it when `poll` returns `Poll::Ready(Some(message))` or
`Poll::Ready(None)`.

The exact details of how a runtime does that are more than we will cover in even
this deep dive section. The key here is to see the basic mechanic of futures: a
runtime *polls* each future it is responsible for, putting it back to sleep when
it is not yet ready.

### Pinning and the Pin and Unpin Traits

<!-- TODO: get a *very* careful technical review of this section! -->

When we introduced the idea of pinning, while working on Listing 17-17, we ran
into a very gnarly error message. Here is the relevant part of it again:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-18
cargo build
copy *only* the final `error` block from the errors
-->

```text
error[E0277]: `{async block@src/main.rs:8:23: 20:10}` cannot be unpinned
  --> src/main.rs:46:33
   |
46 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:8:23: 20:10}`, which is required by `Box<{async block@src/main.rs:8:23: 20:10}>: std::future::Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:8:23: 20:10}>` to implement `std::future::Future`
note: required by a bound in `JoinAll`
  --> /Users/chris/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

When we read this error message carefully, it not only tells us that we need to
pin the values but also us why pinning is required. The `trpl::join_all`
function returns a struct called `JoinAll`. That struct in turn is generic over
a type `F`, which is constrained to implement the `Future` trait. Finally,
directly awaiting a Future requires that the future in question implement the
`Unpin` trait. That’s a lot! But we can understand it, if we dive a little
further into how the `Future` type actually works, in particular around
*pinning*.

Let’s look again at the definition of `Future`, focusing specifically on the
`poll` method’s `self` type:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

This is the first time we have seen a method where `self` has a type annotation
like this. When we specify the type of `self` like this, we are telling Rust
what type `self` must be to call this method. These kinds of type annotations
for `self` are similar to those for other function parameters, but with the
restriction that the type annotation has to be the type on which the method is
implemented, or a reference or smart pointer to that type, or a `Pin` wrapping a
reference to that type. We will see more on this syntax in Chapter 18. For now,
it is enough to know that if we want to poll a future (to check whether it is
`Pending` or `Ready(Output)`), we need a mutable reference to the type, which is
wrapped in a `Pin`.

`Pin` is a wrapper type. In some ways, it is like the `Box`, `Rc`, and other
smart pointer types we saw in Chapter 15, which also wrap other types. Unlike
those, however, `Pin` only works with *other pointer types* like reference (`&`
and `&mut`) and smart pointers (`Box`, `Rc`, and so on). To be precise, `Pin`
works with types which implement the `Deref` or `DerefMut` traits, which we
covered in Chapter 15. You can think of this restriction as equivalent to only
working with pointers, though, since implementing `Deref` or `DerefMut` means
your type behaves like a pointer type. `Pin` is also not a pointer itself, and
it does not have any behavior of its own like the ref counting of `Rc` or `Arc`.
It is purely a tool the compiler can use to uphold the relevant guarantees, by
wrapping pointers in the type.

Recalling that `.await` is implemented in terms of calls to `poll`, this
starts to explain the error message we saw above—but that was in terms of
`Unpin`, not `Pin`. So what exactly are `Pin` and `Unpin`, how do they relate,
and why does `Future` need `self` to be in a `Pin` type to call `poll`?

In [Our First Async Program][first-async], we described how a series of await
points in a future get compiled into a state machine—and noted how the compiler
helps make sure that state machine follows all of Rust’s normal rules around
safety, including borrowing and ownership. To make that work, Rust looks at what
data is needed between each await point and the next await point or the end of
the async block. It then creates a corresponding variant in the state machine it
creates. Each variant gets the access it needs to the data that will be used in
that section of the source code, whether by taking ownership of that data or by
getting a mutable or immutable reference to it.

So far so good: if we get anything wrong about the ownership or references in a
given async block, the borrow checker will tell us. When we want to move around
the future that corresponds to that block—like moving it into a `Vec` to pass to
`join_all`—things get trickier.

When we move a future—whether by pushing into a data structure to use as an
iterator with `join_all`, or returning them from a function—that actually means
moving the state machine Rust creates for us. And unlike most other types in
Rust, the futures Rust creates for async blocks can end up with references to
themselves in the fields of any given variant. Any object which has a reference
to itself is unsafe to move, though, because references always point to the
actual memory address of the thing they refer to. If you move the data structure
itself, you *have* to update any references to it, or they will be left pointing
to the old location.

In principle, you could make the Rust compiler try to update every reference to
an object every time it gets moved. That would potentially be a lot of
performance overhead, especially given there can be a whole web of references
that need updating. On the other hand, if we could make sure the data structure
in question *does not move in memory*, we do not have to update any references.
And this is exactly what Rust’s borrow checker already guarantees: you cannot
move an item which has any active references to it using safe code.

`Pin` builds on that to give us the exact guarantee we need. When we *pin* a
value by wrapping a pointer to it in `Pin`, it can no longer move. Thus, if you
have `Pin<Box<SomeType>>`, you actually pin the `SomeType` value, *not* the
`Box` pointer. In fact, the pinned box pointer can move around freely. Remember:
we care about making sure the data ultimately being referenced stays in its
place. If a pointer moves around, but the data it points to is in the same
place, there is no problem.

However, most types are perfectly safe to move around, even if they happen to be
behind a `Pin` pointer. We only need to think about pinning when items have
internal references. Primitive values like numbers and booleans do not have any
internal structure like that, so they are obviously safe. Neither do most types
you normally work with in Rust. A `Vec`, for example, does not have any internal
references it needs to keep up to date this way, so you can move it around
without worrying. If you have a `Pin<Vec<String>>`, you would have to do
everything via Pin’s safe but restrictive APIs, even though a `Vec<String>` is
always safe to move if there are no other references to it. We need a way to
tell the compiler that it is actually just fine to move items around in cases
like these. For that, we have `Unpin`.

`Unpin` is a marker trait, like `Send` and `Sync`, which we saw in Chapter 16.
Recall that marker traits have no functionality of their own. They exist only to
tell the compiler that it is safe to use the type which implements a given trait
in a particular context. `Unpin` informs the compiler that a given type does
*not* need to uphold any particular guarantees about whether the value in
question can be moved.

Just like `Send` and `Sync`, the compiler implements `Unpin` automatically for
all types where it can prove it is safe. Implementing `Unpin` manually is unsafe
because it requires *you* to uphold all the guarantees which make `Pin` and
`Unpin` safe yourself for a type with internal references. In practice, this is
a very rare thing to implement yourself!

Now we know enough to understand the errors reported for that `join_all` call.
We originally tried to move the futures produced by an async blocks into a
`Vec<Box<dyn Future<Output = ()>>>`, but as we have seen, those futures may have
internal references, so they do not implement `Unpin`. They need to be pinned,
and then we can pass the `Pin` type into the `Vec`, confident that the
underlying data in the futures will *not* be moved.

`Pin` and `Unpin` are mostly important for building lower-level libraries, or
when you are building a runtime itself, rather than for day to day Rust code.
When you see them, though, now you will know what to do!

> Note: This combination of `Pin` and `Unpin` allows a whole class of complex
> types to be safe in Rust which are otherwise difficult to implement because
> they are self-referential. Types which require `Pin` show up *most* commonly
> in async Rust today, but you might—very rarely!—see it in other contexts, too.
>
> The specific mechanics for how `Pin` and `Unpin` work under the hood are
> covered extensively in the API documentation for `std::pin`, so if you would
> like to understand them more deeply, that is a great place to start.
>
> If you want to understand how things work “under the hood” in even more
> detail, the official [_Asynchronous Programming in Rust_][async-book] book has
> you covered:
>
> - [Chapter 2: Under the Hood: Executing Futures and Tasks][under-the-hood]
> - [Chapter 4: Pinning][pinning].

### The Stream Trait

Now that we have a deeper grasp on the `Future`, `Pin`, and `Unpin` traits, we
can turn our attention to the `Stream` trait. As described in the section
introducing streams, streams are like asynchronous iterators. Unlike `Iterator`
and `Future`, there is no definition of a `Stream` trait in the standard library
as of the time of writing,<!-- TODO: verify before press time! --> but there
*is* a very common definition used throughout the ecosystem.

Let’s review the definitions of the `Iterator` and `Future` traits, so we can
build up to how a `Stream` trait that merges them together might look. From
`Iterator`, we have the idea of a sequence: its `next` method provides an
`Option<Self::Item>`. From `Future`, we have the idea of readiness over time:
its `poll` method provides a `Poll<Self::Output>`. To represent a sequence of
items which become ready over time, we define a `Stream` trait which puts those
features together:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

The `Stream` trait defines an associated type `Item` for the type of the items
produced by the stream. This is like `Iterator`: there may be zero to many of
these, and unlike `Future`, where there is always a single `Output` (even if it
the unit type `()`).

`Stream` also defines a method to get those items. We call it `poll_next`, to
make it clear that it polls like `Future::poll` and produces a sequence of items
like `Iterator::next`. Its return type combines `Poll`with `Option`. The outer
type is `Poll`, since it has to be checked for readiness, just like a future.
The inner type is `Option`, since it needs to signal whether there are more
messages, just like an iterator.

Something very similar to this will likely end up standardized as part of Rust’s
standard library. In the meantime, it is part of the toolkit of most runtimes,
so you can rely on it, and everything we cover below should generally apply!

In the example we saw in the section on streaming, though, we did not use
`poll_next` *or* `Stream`, but instead used `next` and `StreamExt`. We *could*
work directly in terms of the `poll_next` API by hand-writing our own `Stream`
state machines, of course, just as we *could* work with futures directly via
their `poll` method. Using `await` is much nicer, though, so the `StreamExt`
trait supplies the `next` method so we can do just that.

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: update this if/when tokio/etc. update their MSRV and switch to using async functions
in traits, since the lack thereof is the reason they do not yet have this.
-->

> Note: The actual definition we will use looks slightly different than this,
> because it supports versions of Rust which did not yet support using async
> functions in traits. As a result, it looks like this:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> That `Next` type is just a simple `struct` which implements `Future` and gives
> a way to name the lifetime of the reference to `self` with `Next<'_, Self>`,
> so that `.await` can work with this!

The `StreamExt` trait is also the home of all the interesting methods available
to use with streams. `StreamExt` is automatically implemented for every type
which implements `Stream`, but they are separated out so that the community can
iterate on the foundational trait distinctly from the convenience APIs.

In the version of `StreamExt` used in the `trpl` crate, the trait not only
defines the `next` method, it also supplies an implementation of `next`, which
correctly handles the details of calling `Stream::poll_next`. This means that
even when you need to write your own streaming data type, you *only* have to
implement `Stream`, and then anyone who uses your data type can use `StreamExt`
and its methods with it automatically.

[futures-syntax]: ch17-01-futures-and-syntax.html
[counting]: ch17-02-concurrency-with-async.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-and-syntax.html#our-first-async-program

That’s all we’re going to cover for the lower-level details on these traits. To
wrap up, let’s consider how futures (including streams), tasks, and threads all
fit together!
