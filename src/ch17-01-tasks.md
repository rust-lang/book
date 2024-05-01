## Futures and the Async Syntax

### Tasks

As we saw in the previous chapter, threads provide one approach to concurrency,
and they let us solve some of these issues. However, they also have some
tradeoffs. On many operating systems, they use a fair bit of memory for each
thread, and they come with some overhead for starting up and shutting down.
Threads are also only an option when your operating system and hardware support
multiple threads. While mainstream desktop and mobile operating systems have all
had threading for many years, many embedded operating systems used on
microcontrollers do not.

The async model provides a different, complementary set of tradeoffs. In

<!-- TODO: the following paragraph is not where it needs to be structurally. -->

In the async model, concurrent operations do not require their own threads.
Instead, they can run on *tasks*. A task is a bit like a thread, but instead of
being managed by the operating system, it is managed by a runtime.

<!-- TODO: connective tissue as it were. -->

###

Like many other languages with first-class support for the async programming
model, Rust uses the `async` and `await` keywords—though with some important
differences from other languages like C# or JavaScript. Blocks and functions can
be marked `async`, and you can wait on the result of an `async` function or
block to resolve using the `await` keyword.

Let’s write our first async function:

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

The warning tells us why nothing happened. Calling `hello_async()` itself was
not enough: we need to `.await`or poll the “future” it returns. That might be a
bit surprising: we did not write a return type on the function. However, we
*did* mark it as an `async fn`. In Rust, `async fn` is equivalent to writing a
function which returns a *future* of the return type, using the `impl Trait`
syntax we discussed back in the [“Traits as Parameters”][impl-trait] section in
Chapter 10. So these two are roughly equivalent:

<!-- no-compile -->
```rust
fn hello_async() -> impl Future<Output = ()> {
    println!("Hello, async!");
}
```

```rust
async fn hello_async()  {
    println!("Hello, async!");
}
```

That explains why we got the `unused_must_use` warning. The other part of the
warning was the note that we need to `.await` or poll the future. Rust's `await`
keyword is a postfix keyword, meaning it goes *after* the expression you are
awaiting. (As of now, `await` is the only postfix keyword in the language.)
Let’s try that here:

```rust
fn main() {
    hello_async().await;
}
```

Now we actually have a compiler error!

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:2:19
  |
1 | fn main() {
  |    ---- this is not `async`
2 |     hello_async().await;
  |                   ^^^^^ only allowed inside `async` functions and blocks
```

Okay, so we cannot actually use `.await` in `main`, because it is not an `async`
function itself—and it cannot be. To understand why, we need to pause to see
what a `Future` actually is and why it needs to be `.await`-ed or polled to do
anything.

### Understanding `Future`

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

You might have noticed that this `Poll` type is a lot like an `Option`. Having a
dedicated type lets Rust treat `Poll` differently from `Option`, though, which
is important since they have very different meanings! The `Pending` variant
indicates that the future still has work to do, so the caller will need to check
again later. The `Ready` variant indicates that the `Future` has finished its
work and the `T` value is available.

> Note: With most future, the caller should not call `poll()` again after the
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
for three other reasons, though:

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

Some languages, including Go, Kotlin, Erlang, and Swift, ship runtimes with the
language. In Rust, there are many different runtimes, because the things a
runtime for a high-throughput web server with dozens of CPU cores and terabytes
of RAM should do are very different from the things a runtime for a
microcontroller with a single core and one gigabyte of RAM should do.

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
would execute the async code? We need to pick a runtime and executor. We can get
started with that easily by using the simple one that comes bundled with the
`futures` crate, an official home for Rust experimentation for async code. Since
we will be using a bunch of tools from that crate for the rest of the chapter,
let’s go ahead and add it to the dependencies for our test project:

```
cargo add futures@0.3
```

Now we can use the executor which comes with `futures` to run the code. The
`futures::executor::block_on` function takes in a `Future` and runs it until it
completes.

```rust
use futures::executor;

fn main() {
    executor::block_on(hello_async());
}

async fn hello_async() {
    println!("Hello, async!");
}
```

Now when we run this, we get the behavior we might have expected initially:

```console
$ cargo run
   Compiling hello-async v0.1.0 (/Users/chris/dev/chriskrycho/async-trpl-fun/hello-async)
    Finished dev [unoptimized + debuginfo] target(s) in 4.89s
     Running `target/debug/hello-async`
Hello, async!
```

Now, that’s a lot of work to just print a string, but we have laid some key
foundations for working with async in Rust! Now that you know the basics of how
futures work, and the

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[async-book]: https://rust-lang.github.io/async-book/
