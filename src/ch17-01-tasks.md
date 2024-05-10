## Futures and the Async Syntax

As we saw in the previous chapter, threads provide one approach to concurrency,
and they let us solve some of these issues. However, they also have some
tradeoffs. On many operating systems, they use a fair bit of memory for each
thread, and they come with some overhead for starting up and shutting down.
Threads are also only an option when your operating system and hardware support
them! While mainstream desktop and mobile operating systems have all had
threading for many years, many embedded operating systems, like those used on
some microcontrollers, do not.

The async model provides a different—and ultimately complementary—set of
tradeoffs. In

<!-- TODO: the following paragraph is not where it needs to be structurally. -->

In the async model, concurrent operations do not require their own threads.
Instead, they can run on *tasks*. A task is a bit like a thread, but instead of
being managed by the operating system, it is managed by a runtime.

<!-- TODO: connective tissue as it were. -->

Like other languages with async, Rust uses the `async` and `await`
keywords—though with some important differences, as we will see. Blocks and
functions can be marked `async`, and you can wait on the result of an `async`
function or block to resolve using the `await` keyword.

Let’s write our first async function, and call it:

```rust
fn main() {
    hello_async();
}

async fn hello_async() {
    println!("Hello, async!");
}
```

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

In Rust, `async fn` is equivalent to writing a function which returns a
future of the return type, using the `impl Trait` syntax we discussed back in
the [“Traits as Parameters”][impl-trait] section in Chapter 10. An `async`
block compiles to an anonymous struct which implements the `Future` trait.

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

<!-- TODO: check whether the "unused Result" thing has been covered. -->

That explains why we got the `unused_must_use` warning: writing `async fn` meant
we were actually returning an anonymous `Future`. Just like with `Result`, Rust
will let us know if we don’t use a `Future`. It is often a mistake to ignore an
error in an operation your program performed. With a `Future`, it is even more
significant. To see why, let’s look at the other part of the warning. It told us
that “futures do nothing unless you `.await` or poll them”. That is, futures are
*lazy*: they don’t do anything until you ask them to. The compiler is telling us
that ignoring a `Future` makes it completely useless! We will see why that is
later on. For now, let’s start by awaiting the future returned by `hello_async`
to actually have it run.

> Note: Rust’s `await` keyword goes *after* the expression you are awaiting—that
> is, it is a *postfix keyword*. This is different from what you might be used
> to if you have used async in languages like JavaScript or C#. Rust chose this
> because it makes chains of async and non-async methods much nicer to work
> with. As of now, `await` is the only postfix keyword in the language.

<!-- does not compile -->
```rust
fn main() {
    hello_async().await;
}
```

Oh no! We have gone from a compiler warning ton an actual error:

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:2:19
  |
1 | fn main() {
  |    ---- this is not `async`
2 |     hello_async().await;
  |                   ^^^^^ only allowed inside `async` functions and blocks
```

This time, the compiler is informing us we cannot actually use `.await` in
`main`, because `main` is not an `async` function. As of today, it cannot be
without some extra help: it needs a *runtime* to execute the asynchronous code.

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

Now we can get our code working by using the `trpl::block_on` function, which
takes in a `Future` and runs it until it completes.

```rust
fn main() {
    trpl::block_on(hello_async());
}

async fn hello_async() {
    println!("Hello, async!");
}
```

Now when we run this, we get the behavior we might have expected initially:

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
a runtime to make this work.

### Futures

Since `async fn` compiles to a return type with `impl Future<Output = …>`, we
know that `Future` is a trait, with an associated type `Output`. The other part
of the trait is its one method: `poll`. The `poll` method returns a fairly
simple type:

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
`poll` instead, kind of like this:

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

If we wrote that code, though, we would block the computer from doing anything
else, though—the opposite of what we were going for. It also wouldn’t compile
for three other reasons:

1. The `poll` method actually requires an argument: a `Context` that carries
   along a way to say when to call it again, to avoid exactly the problem of
   blocking other operations from making progress.

2. The `Future` returned by our `async fn` is not actually ready to use here,
   because `poll` requires that the future be “pinned”—guaranteed not to get
   moved around in memory, so that any references to the future can be checked
   for memory safety.

3. Even if we pinned the future, this code would move `hello_async_fut` in the
   first iteration through the `loop`. After the first time through the loop, we
   would not be able to call it again.

When we use the `async fn` form with `.await`, Rust compiles it to something
smarter than the `loop` above, in conjunction with a *runtime* responsible for
executing that loop.

> Note: Some languages, including Go, Kotlin, Erlang, and Swift, ship runtimes
> with the language. In Rust, there are many different runtimes, because a
> runtime might need to do very different things to support a high-throughput
> web server with dozens of CPU cores and terabytes of RAM than it would for a
> microcontroller with a single core and one gigabyte of RAM should do.

<!-- TODO: more text here -->

The other thing to notice here is that futures in Rust are *lazy*. They do not
do anything until you explicitly ask them to—whether by calling `poll` or by
using `.await` to do so. This is different from the behavior we saw when using
`thread::spawn` in the previous chapter, and it is different from how many other
languages approach async. This allows Rust to avoid running async code unless it
is actually needed, and supports some of the memory safety features Rust brings
to async. (The details are beyond the scope of this book, but are well worth
digging into. In particular, see [Chapter 4: Pinning][pinning] in the official
[_Asynchronous Programming in Rust_][async-book] book.)

### Running Async Code

<!-- TODO: runtime and executor -->

Going back to `main`, this explains why we cannot have an `async fn main`: what
would execute the async code? We need to pick a runtime and executor.

Now, that’s a lot of work to just print a string, but we have laid some key
foundations for working with async in Rust! Now that you know the basics of how
futures work, and the

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[async-book]: https://rust-lang.github.io/async-book/
