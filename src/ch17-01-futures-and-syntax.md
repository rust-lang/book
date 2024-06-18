## Futures and the Async Syntax

Like other languages with async, Rust uses the `async` and `await`
keywords—though with some important differences from how other languages do
things, as we will see. Blocks and functions can be marked `async`, and you can
wait on the result of an `async` function or block to resolve using the `await`
keyword.

Let’s write our first async function, and call it:

<Listing number="17-1" file-name="src/main.rs" caption="Defining a very simple async function">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

If we compile and run this… nothing happens, and we get a compiler warning:

```console
{{#include ../listings/ch17-async-await/listing-17-01/output.txt}}
```

The warning tells us that just calling `hello_async()` was not enough: we also
need to `.await` or poll the future it returns. This raises two important
questions:

- Given there is no return type on the function, how is it returning a future?
- What is a future?

### Async functions

In Rust, `async fn` is equivalent to writing a function which returns a *future*
of the return type. That is, when the compiler sees a function like this:

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:async-fn}}
```

It is basically equivalent to a function defined like this instead:

```rust
fn hello_async() -> impl Future<Output = ()> {
    async {
        println!("Hello, async!");
    }
}
```

Let’s break that down and see what each part means:

* It uses the `impl Trait` syntax we discussed back in the [“Traits as
  Parameters”][impl-trait] section in Chapter 10 to return a `Future` with an
  associated type of `Output`. That tells us that `Future` is a trait, and the
  `Output` associated type of `()` matches the original return type from the
  async version of the function.

* It wraps the whole body of the `async fn` in an `async` block. Given that
  blocks like this are expressions, and that this is expression is the one which
  is returned from the function, we can infer that an `async` block like this
  produces some anonymous data type which implements the `Future` trait.

Combined, those explain that when we called `hello_async` in `main`, it returned
a `Future`. Then Rust warned us that we did not do anything with the future.
This is because futures are *lazy*: they don’t do anything until you ask them
to. This should remind you of our discussion of iterators [back in Chapter
13][iterators-lazy]. With iterators, you have to call `next` to get them to do
anything—whether by using a `for` loop or by using iterator methods like `.iter()` and `.map()` which ultimately call `next()` under the hood.

With futures, the same basic idea applies, although for different reasons, and
with different syntax and methods. This is different from the behavior we saw
when using `thread::spawn` in the previous chapter, and it is different from how
many other languages approach async. This allows Rust to avoid running async
code unless it is actually needed. We will see why that is later on. For now,
let’s start by awaiting the future returned by `hello_async` to actually have it
run.

> Note: Rust’s `await` keyword goes after the expression you are awaiting, not
> before it—that is, it is a _postfix keyword_. This is different from what you
> might be used to if you have used async in languages like JavaScript or C#.
> Rust chose this because it makes chains of async and non-async methods much
> nicer to work with. As of now, `await` is Rust’s only postfix keyword.

<Listing number="17-2" caption="Attempting to fix a compiler warning by awaiting a future" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:main}}
```

</Listing>

Oh no! We have gone from a compiler warning to an actual error:

```console
{{#include ../listings/ch17-async-await/listing-17-02/output.txt}}
```

This time, the compiler is informing us we cannot use `.await` in `main`,
because `main` is not an `async` function. That is because async code needs a
*runtime*: a Rust crate which manages the details of executing asynchronous
code.

Most languages which support async, including C#, JavaScript, Go, Kotlin,
Erlang, and Swift, bundle a runtime with the language. At least for now, Rust
does not. Instead, there are many different async runtimes available, each of
which makes different tradeoffs suitable to the use case they target. For
example, a high-throughput web server with dozens of CPU cores and terabytes of
RAM has very different different needs than a microcontroller with a single
core, one gigabyte of RAM, and no ability to do heap allocations.

To keep this chapter focused on learning async, rather than juggling parts of
the ecosystem, we have created the `trpl` crate (`trpl` is short for “The Rust
Programming Language”). It re-exports all the types, traits, and functions you
will need, and in a couple cases wires up a few things for you which are less
relevant to the subject of the book. There is no magic involved, though! If you
want to understand what the crate does, we encourage you to check out [its
source code][crate-source]. You will be able to see what crate each re-export
comes from, and we have left extensive comments explaining what the handful of
helper functions we supply are doing.

> ### The `futures` and `tokio` Crates
>
> Whenever you see code from the `trpl` crate throughout the rest of the
> chapter, it will be re-exporting code from the [`futures`][futures-crate] and
> [`tokio`][tokio] crates.
>
> - The `futures` crate is an official home for Rust experimentation for async
>   code, and is actually where the `Future` type was originally designed.
>
> - Tokio is the most widely used async runtime in Rust today, especially (but
>   not only!) for web applications. There are other great options out there,
>   too, and they may be more suitable for your purposes. We are using Tokio
>   because it is the most widely-used runtime—not as a judgment call on whether
>   it is the *best* runtime!

For now, go ahead and add the `trpl` crate to your `hello-async` project:

```console
$ cargo add trpl
```

Then, in our `main` function, let’s wrap the call to `hello_async` with the
`trpl::block_on` function, which takes in a `Future` and runs it until it
completes.

<Listing number="17-3" caption="Using the `block_on` helper function to wait on a future in non-async code" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

When we run this, we get the behavior we might have expected initially:

```console
{{#include ../listings/ch17-async-await/listing-17-03/output.txt}}
```

Phew: we finally have some working async code! Now we can answer that second
question: what is a future anyway? That will also help us understand why we need
that `trpl::block_on` call to make this work.

### What Are Futures?

A *future* is a data structure which represents the state of some async
operation. More precisely, a Rust `Future` is a trait; it allows many different
data structures to represent different async operations in different ways, but
with a common interface. Here is the definition of the trait:

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Notice that this is a normal trait. While we often interact with futures via
async blocks, you can also implement this yourself on your own data types when
you need to. Many of the functions we will see throughout this chapter return
types which have their own implementations of `Future`. Those implementations
can compose together nicely

`Future` has an associated type, `Output`, which says what the result of the
future will be when it resolves. (This is analogous to the `Item` associated
type for the `Iterator` trait, which we saw back in Chapter 13.) Beyond that,
`Future` has only one method: `poll`, which takes a special `Pin` reference for
its `self` parameter and a mutable reference to some `Context` type, and returns
a `Poll<Self::Output>`. We will talk a little more about `Pin` and `Context`
later in the chapter. For now, let’s focus on what the method returns, the
`Poll` type:

```rust
enum Poll<T> {
    Ready(T),
    Pending
}
```

You may notice that this `Poll` type is a lot like an `Option`: it has one
variant which has a value (`Ready(T)` and `Some(T)`), and one which does not
(`Pending` and `None`). Having a dedicated type lets Rust treat `Poll`
differently from `Option`, though, which is important since they have very
different meanings! The `Pending` variant indicates that the future still has
work to do, so the caller will need to check again later. The `Ready` variant
indicates that the `Future` has finished its work and the `T` value is
available.

> Note: With most futures, the caller should not call `poll()` again after the
> future has returned `Ready`. Many futures will panic if polled after becoming
> ready! Futures which are safe to poll again will say so explicitly in their
> documentation.

Under the hood, when you call `.await`, Rust compiles that to code which calls
`poll`, kind of (although not exactly <!-- TODO: describe `IntoFuture`? -->)
like this:

```rust,ignore
match hello_async().poll() {
    Ready(_) => {
        // We’re done!
    }
    Pending => {
        // But what goes here?
    }
}
```

As you can see from this sample, though, there is a question: what happens when
the `Future` is still `Pending`? We need some way to try again. We would need to
have something like this instead:

```rust,ignore
let hello_async_fut = hello_async();
loop {
    match hello_async_fut.poll() {
        Ready(_) => {
            break;
        }
        Pending => {
            // continue
        }
    }
}
```

When we use `.await`, Rust compiles it to something fairly similar to that loop.
If Rust compiled it to *exactly* that code, though, every `.await` would block
the computer from doing anything else—the opposite of what we were going for!
Instead, Rust needs makes sure that the loop can hand off control to something
which can pause work on this future and work on other futures and check this one
again later. That “something” is an async runtime, and this scheduling and
coordination work is one of the main jobs for a runtime.

Every *await point*—that is, every place where the code explicitly calls
`.await`—represents one of those places where control gets handed back to the
runtime. To make that work, Rust needs to keep track of the state involved in
the async block, so that the runtime can kick off some other work and then come
back when it is ready to try advancing this one again. This is an invisible
state machine, as if you wrote something like this:

```rust
enum MyAsyncStateMachine {
    FirstAwaitPoint(/* the state used after the first await point */),
    SecondAwaitPoint(/* the state used after the second await point */),
    // etc. for each `.await` point...
}
```

Writing that out by hand would be tedious and error-prone—especially when making
changes to code later. Async Rust creates that state machine for us, and it
really is an `enum` like this, just an anonymous one you don’t have to name. As
a result, the normal rules around data structures all apply, including for
borrowing and ownership. Happily, the compiler also handles checking that for
us, and has good error messages. We will work through a few of those later in
the chapter!

Once all of that compilation work is done, though, we need a runtime to actually
poll the futures, coordinate between different futures as they hand off control
at await points, and even provide async versions of common functionality like
file or network I/O.

Now we can understand why the compiler was stopping us in Listing 17-2 (before
we added the `trpl::block_on` function). The `main` function is not `async`—and
it really cannot be: if it were, something would need to call `poll()` on
whatever `main` returned! Instead, we use the `trpl::block_on` function, which
polls the `Future` returned by `hello_async` until it returns `Ready`. Every
async program in Rust has at least one place where it sets up an executor and
executes code.

> Note: Under the hood, Rust uses *generators* so that it can hand off control
> between different functions. These are an implementation detail, though, and
> you never have to think about it when writing Rust.
>
> The loop as written also wouldn’t compile, because it doesn’t actually satisfy
> the contract for a `Future`. In particular, `hello_async_fut` is not *pinned*
> with the `Pin` type and we did not pass along a `Context` argument. We will
> see a little more about `Pin` later in the chapter, but we will not dig into
> `Context` because you will not normally need them for working with futures in
> day-to-day Rust code.
>
> If you want to understand how things work “under the hood,” though, the
> official [_Asynchronous Programming in Rust_][async-book] book covers them:
>
> - [Chapter 2: Under the Hood: Executing Futures and Tasks][under-the-hood]
> - [Chapter 4: Pinning][pinning].

Now, that’s a lot of work to just print a string, but we have laid some key
foundations for working with async in Rust! Now that you know the basics of how
futures and runtimes work, we can see some of the things we can *do* with async.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[async-book]: https://rust-lang.github.io/async-book/
<!-- TODO: map source link version to version of Rust? -->
[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs


