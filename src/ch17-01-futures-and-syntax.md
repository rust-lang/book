## Futures and the Async Syntax

Like other languages with async, Rust uses the `async` and `await` keywords for
async programming. (If you are familiar with other languages’ approach to async,
you may notice some significant differences, though.) In Rust, blocks and
functions can be marked `async`, and you can wait on the result of an `async`
function or block to resolve using the `await` keyword.

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

The warning tells us that just calling `hello` was not enough: we also need to
`.await` or poll the future it returns. This raises some important questions:

- Given there is no return type on the function, how is it returning a future?
- What exactly is a future?
- Why do we need to `.await` or poll futures to make them do something?
- How do `.await` and polling relate to each other?

We will work through each of these in turn. We can answer the first question by
learning what the syntax means, so let’s start there.

### Async functions

In Rust, writing `async fn` is equivalent to writing a function which returns a
*future* of the return type. That is, when the compiler sees a function like
`async fn hello` in Listing 17-1, it is basically equivalent to a function
defined like this instead:

```rust
use std::future::Future;

fn hello<'a>(name: &'a str) -> impl Future<Output = ()> + 'a {
    async move {
        let greeting = format!("Hello, {name}!");
        println!("{greeting}");
    }
}
```

Let’s walk through each part of the transformed version:

* It uses the `impl Trait` syntax we discussed back in the [“Traits as
  Parameters”][impl-trait] section in Chapter 10.
* The returned trait is a `Future`, with an associated type of `Output`. Notice
  that the `Output` type is `()`, which is the same as the the original return
  type from the `async fn` version of `hello`.
* All of the code called in the body of the original function is wrapped in an
  `async move` block. Remember that blocks are expressions. This whole block is
  the expression returned from the function.
* The new function body is an `async move` block because of how it uses the
  `name` argument.
* The new version of the function makes the lifetime of the `name` parameter
  explicit so that it can reference it in the output type.
* The async block itself has the “unit” value `()`, since it ends with a
  `println!` statement. That value matches the `Output` type in the return type.

An `async` block corresponds to a data type which implements the `Future` trait,
and the result of the async block will be the `Output` of the `Future`. Thus, an
`async fn`’s return type is an anonymous data type the compiler creates for us,
which implements `Future`. The associated `Output` type for the `Future` is the
return type of the original `async fn`. Thus, calling `hello` in Listing 17-1
returned a `Future<Output = ()>`.

Then Rust warned us that we did not do anything with the future. This is because
futures are *lazy*: they don’t do anything until you ask them to with `await`.
This should remind you of our discussion of iterators [back in Chapter
13][iterators-lazy]. Iterators do nothing unless you call their `.next()`
method—whether directly, or using `for` loops or methods like `.map()` which use
`.next()` under the hood.

With futures, the same basic idea applies: they do nothing unless you explicitly
ask them to. This laziness allows Rust to avoid running async code until it is
actually needed.

> Note: This is different from the behavior we saw when using `thread::spawn` in
> the previous chapter, where the closure we passed to another thread started
> running immediately. It is also different from how many other languages
> approach async! But it is important for Rust. We will see why that is later.
<!-- TODO: we need to pay off that promise later in the chapter! -->

For now, let’s start by awaiting the future returned by `hello` to actually have
it run. Rust’s `await` keyword goes after the expression you are awaiting, not
before it. That is, it is a *postfix keyword*. (This is different from what you
might be used to if you have used async in languages like JavaScript or C#. Rust
chose this because it makes chains of methods much nicer to work with.) In
Listing 17-2, we add `.await` to the `hello` call in `main`.

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
because `main` is not an `async` function. Your first thought might be to make
`main` an async function then, as in Listing 17-3.

<Listing number="17-3" caption="Attempting to make `main` an `async fn`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

However, we get another compiler error here:

```console
{{#include ../listings/ch17-async-await/listing-17-03/output.txt}}
```

The problem is that async code needs a *runtime*: a Rust crate which manages the
details of executing asynchronous code.

Most languages which support async bundle a runtime with the language. Rust does
not. Instead, there are many different async runtimes available, each of which
makes different tradeoffs suitable to the use case they target. For example, a
high-throughput web server with many CPU cores and a large amount of RAM has
very different different needs than a microcontroller with a single core, a
small amount of RAM, and no ability to do heap allocations.

Every async program in Rust has at least one place where it sets up a runtime
and executes the futures. Those runtimes also often supply async versions of
common functionality like file or network I/O.

> ### The `trpl` Crate
>
> To keep this chapter focused on learning async, rather than juggling parts of
> the ecosystem, we have created the `trpl` crate (`trpl` is short for “The Rust
> Programming Language”). It re-exports all the types, traits, and functions you
> will need, primarily from the [`futures`][futures-crate] and [`tokio`][tokio]
> crates.
>
> - The `futures` crate is an official home for Rust experimentation for async
>   code, and is actually where the `Future` type was originally designed.
>
> - Tokio is the most widely used async runtime in Rust today, especially (but
>   not only!) for web applications. There are other great runtimes out there,
>   and they may be more suitable for your purposes. We use Tokio under the hood
>   for `trpl` because it is good and widely used.
>
> In some cases, `trpl` also renames or wraps the original APIs to let us stay
> focused on the details relevant to chapter. If you want to understand what the
> crate does, we encourage you to check out [its source code][crate-source]. You
> will be able to see what crate each re-export comes from, and we have left
> extensive comments explaining what the crate does.

Go ahead and add the `trpl` crate to your `hello-async` project:

```console
$ cargo add trpl
```

Then, in our `main` function, let’s wrap the call to `hello` with the
`trpl::block_on` function, which takes in a `Future` and runs it until it
completes. Since `hello` returns a `Future`, we could simply wrap it directly in
`trpl::block_on`. However, for most of the examples in the chapter, we will be
doing more than just one async function call, so instead we will pass an `async`
block and explicitly await the result of calling `hello`.

<Listing number="17-4" caption="Using the `block_on` helper function to wait on a future in non-async code" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:main}}
```

</Listing>

When we run this, we get the behavior we might have expected initially:

```console
{{#include ../listings/ch17-async-await/listing-17-04/output.txt}}
```

Phew: we finally have some working async code! Let’s briefly turn our attention
to how the `Future` trait works.

A *future* is a data structure which manages the state of some async operation.
It is called a “future” because it represents work which may not be ready now,
but will become ready at some point in the future. (This same concept shows up
in many languages, sometimes under other names like “task” or “promise”.) Rust
provides a `Future` trait as a building block so different async operations can
be implemented with different data structures, but with a common interface.

Most of the time when writing async Rust, we don’t work directly with the
`Future` trait. Instead, we use the `async` and `await` keywords we saw above.
Rust does the work of compiling them into the appropriate calls to the `Future`
trait, much like it does with `for` loops and the `Iterator` trait.

We most often interact with the futures created via async blocks, but you can
also implement `Future` on your own data types. Indeed, many of the functions we
will see throughout this chapter return types with their own implementations of
`Future`. We will return to the definition of the trait at the end of the
chapter and dig into more of how it works, but this is enough detail to keep us
moving forward.

<!-- TODO: need to introduce/transition with this next paragraph. -->

Every *await point*—that is, every place where the code explicitly applies the
`await` keyword—represents a place where control gets handed back to the
runtime. To make that work, Rust needs to keep track of the state involved in
the async block, so that the runtime can kick off some other work and then come
back when it is ready to try advancing this one again. This is an invisible
state machine, as if you wrote something like this:

```rust
enum MyAsyncStateMachine {
    FirstAwaitPoint {
        // the state used up to the first await point...
    },
    SecondAwaitPoint {
        // the state used up to the second await point...
    },
    // etc. for each `.await` point...
}
```

Writing that out by hand would be tedious and error-prone, especially when
making changes to code later. Instead, the Rust compiler creates and manages the
state machine data structures for async code automatically.

If you’re wondering: yep, the normal borrowing and ownership rules around data
structures all apply. Happily, the compiler also handles checking those for us,
and has good error messages. We will work through a few of those later in the
chapter!

<!--
  TODO: this part needs to be rewritten to account for moving the content out to
  a later part of the book.
-->

Now we can understand why the compiler stopped us from making `main` itself an
async function in Listing 17-3. If `main` were an async function, something else
would need to call `poll()` on whatever `main` returned, but main is the
starting point for the program! Instead, we use the `trpl::block_on` function,
which sets up a runtime and polls the `Future` returned by `hello` until it
returns `Ready`.

> Note: We have skipped over some interesting implementation details in this
> discussion, because you should not have to think about them when writing Rust.
>
> If you want to understand how things work “under the hood,” though, the
> official [_Asynchronous Programming in Rust_][async-book] book covers them:
>
> - [Chapter 2: Under the Hood: Executing Futures and Tasks][under-the-hood]
> - [Chapter 4: Pinning][pinning].

Now that you know the basics of how futures and runtimes work, we can see some
of the things we can *do* with async.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[async-book]: https://rust-lang.github.io/async-book/
<!-- TODO: map source link version to version of Rust? -->
[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
