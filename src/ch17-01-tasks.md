## Futures and the Async Syntax

Like other languages with async, Rust uses the `async` and `await`
keywords—though with some important differences from how other languages do
things, as we will see. Blocks and functions can be marked `async`, and you can
wait on the result of an `async` function or block to resolve using the `await`
keyword.

Let’s write our first async function, and call it:

<Listing number="17-1" file-name="src/main.rs" caption="Defining a very simple async function">

```rust
fn main() {
    hello_async();
}

async fn hello_async() {
    println!("Hello, async!");
}
```

</Listing>

If we compile and run this… nothing happens, and we get a compiler warning:

```console
$ cargo run
warning: unused implementer of `Future` that must be used
 --> src/main.rs:2:5
  |
2 |     hello_async();
  |     ^^^^^^^^^^^^^
  |
  = note: futures do nothing unless you `.await` or poll them
  = note: `#[warn(unused_must_use)]` on by default

warning: `hello-async` (bin "hello-async") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/hello-async`
```

The warning tells us that just calling `hello_async()` was not enough: we also
need to `.await` or poll the future it returns. This raises two important
questions:

- Given there is no return type on the function, how is it returning a future?
- What is a future?

### Async functions

In Rust, `async fn` is equivalent to writing a function which returns a future
of the return type, using the `impl Trait` syntax we discussed back in the
[“Traits as Parameters”][impl-trait] section in Chapter 10. An `async` block
compiles to an anonymous struct which implements the `Future` trait.

That means these two are roughly equivalent:

```rust
async fn hello_async()  {
    println!("Hello, async!");
}
```

```rust
fn hello_async() -> impl Future<Output = ()> {
    async {
        println!("Hello, async!");
    }
}
```

That explains why we got the `unused_must_use` warning: writing `async fn` meant
we were actually returning an anonymous `Future`. The compiler will warn us that
“futures do nothing unless you `.await` or poll them”. That is, futures are
*lazy*: they don’t do anything until you ask them to.

The compiler is telling us that ignoring a `Future` makes it completely useless!
This is different from the behavior we saw when using `thread::spawn` in the
previous chapter, and it is different from how many other languages approach
async. This allows Rust to avoid running async code unless it is actually
needed. We will see why that is later on. For now, let’s start by awaiting the
future returned by `hello_async` to actually have it run.

> Note: Rust’s `await` keyword goes *after* the expression you are awaiting—that
> is, it is a _postfix keyword_. This is different from what you might be used
> to if you have used async in languages like JavaScript or C#. Rust chose this
> because it makes chains of async and non-async methods much nicer to work
> with. As of now, `await` is the only postfix keyword in the language.

<Listing number="17-2" caption="Attempting to fix a compiler warning by awaiting a future" file-name="src/main.rs">

<!-- does not compile -->

```rust
fn main() {
    hello_async().await;
}
```

</Listing>

Oh no! We have gone from a compiler warning to an actual error:

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:2:19
  |
1 | fn main() {
  |    ---- this is not `async`
2 |     hello_async().await;
  |                   ^^^^^ only allowed inside `async` functions and blocks
```

<!-- TODO: eliminate duplicate definition of runtime here and below -->

This time, the compiler is informing us we cannot use `.await` in `main`,
because `main` is not an `async` function. That is because async code needs a
*runtime*: a Rust crate which manages the details of executing the asynchronous
code, including whether or not to use threads for it, scheduling different async
operations, and so on.

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

For now, go ahead and add the `trpl` crate to your `hello-async` project:

```console
$ cargo add trpl
```

Then, in our `main` function, let’s wrap the call to `hello_async` with the
`trpl::block_on` function, which takes in a `Future` and runs it until it
completes.

<Listing number="17-3" caption="Using the `block_on` helper function to wait on a future in non-async code" file-name="src/main.rs">

```rust
fn main() {
    trpl::block_on(hello_async());
}

async fn hello_async() {
    println!("Hello, async!");
}
```

</Listing>

When we run this, we get the behavior we might have expected initially:

<!-- TODO: paths in the output here! -->

```console
$ cargo run
   Compiling hello-async v0.1.0 (/Users/chris/dev/chriskrycho/async-trpl-fun/hello-async)
    Finished dev [unoptimized + debuginfo] target(s) in 4.89s
     Running `target/debug/hello-async`
Hello, async!
```

Phew: we finally have some working async code! Now we can answer that second
question: what is a future anyway? That will also help us understand why we need
that `trpl::block_on` call to make this work.

### Futures

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

You may notice that this `Poll` type is a lot like an `Option`. Having a
dedicated type lets Rust treat `Poll` differently from `Option`, though, which
is important since they have very different meanings! The `Pending` variant
indicates that the future still has work to do, so the caller will need to check
again later. The `Ready` variant indicates that the `Future` has finished its
work and the `T` value is available.

> Note: With most futures, the caller should not call `poll()` again after the
> future has returned `Ready`. Many futures will panic if polled after becoming
> ready! Futures which are safe to poll again will say so explicitly in their
> documentation.

Under the hood, when you call `.await`, Rust compiles that to code which calls
`poll`, kind of like this:

<!-- TODO: does not compile -->

```rust
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

<!-- TODO: does not compile -->

```rust
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

When we use `.await`, Rust actually does compile it to something very similar to
that loop. If Rust compiled it to *exactly* that code, though, every `.await`
would block the computer from doing anything else—the opposite of what we were
going for! Instead, Rust internally makes sure that the loop can hand back
control to the the context of the code where which is awaiting this little bit
of code.

When we follow that chain far enough, eventually we end up back in some
non-async function. At that point, something needs to “translate” between the
async and sync worlds. That “something” is a *runtime*, a crate which handles
the top-level `poll()` call, scheduling and handing off between the different
async operations which may be in flight, and often providing async versions of
functionality like file I/O.

Now we can understand why the compiler was blocking us in Listing 17-2 (before
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
> with the `Pin` type and we did not pass along a `Context` argument.
>
> More details here are beyond the scope of this book, but are well worth
> digging into if you want to understand how things work “under the hood.” In
> particular, see [Chapter 2: Under the Hood: Executing Futures and
> Tasks][under-the-hood] and [Chapter 4: Pinning][pinning] in the official
> [_Asynchronous Programming in Rust_][async-book] book.

Now, that’s a lot of work to just print a string, but we have laid some key
foundations for working with async in Rust! Now that you know the basics of how
futures and runtimes work, we can see some of the things we can *do* with async.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[async-book]: https://rust-lang.github.io/async-book/
[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl

<!-- TODO: map source link version to version of Rust? -->
