## Futures and the Async Syntax

The key elements of asynchronous programming in Rust are _futures_ and Rust’s
`async` and `await` keywords.

A _future_ is a value that may not be ready now but will become ready at some
point in the future. (This same concept shows up in many languages, sometimes
under other names such as _task_ or _promise_.) Rust provides a `Future` trait
as a building block so that different async operations can be implemented with
different data structures but with a common interface. In Rust, futures are
types that implement the `Future` trait. Each future holds its own information
about the progress that has been made and what "ready" means.

You can apply the `async` keyword to blocks and functions to specify that they
can be interrupted and resumed. Within an async block or async function, you can
use the `await` keyword to _await a future_ (that is, wait for it to become
ready). Any point where you await a future within an async block or function is
a potential spot for that async block or function to pause and resume. The
process of checking with a future to see if its value is available yet is called
_polling_.

Some other languages, such as C# and JavaScript, also use `async` and `await`
keywords for async programming. If you’re familiar with those languages, you may
notice some significant differences in how Rust does things, including how it
handles the syntax. That’s for good reason, as we’ll see!

When writing async Rust, we use the `async` and `await` keywords most of the
time. Rust compiles them into equivalent code using the `Future` trait, much as
it compiles `for` loops into equivalent code using the `Iterator` trait. Because
Rust provides the `Future` trait, though, you can also implement it for your own
data types when you need to. Many of the functions we’ll see throughout this
chapter return types with their own implementations of `Future`. We’ll return to
the definition of the trait at the end of the chapter and dig into more of how
it works, but this is enough detail to keep us moving forward.

This may all feel a bit abstract, so let’s write our first async program: a
little web scraper. We’ll pass in two URLs from the command line, fetch both of
them concurrently, and return the result of whichever one finishes first. This
example will have a fair bit of new syntax, but don’t worry—we’ll explain
everything you need to know as we go.

## Our First Async Program

To keep the focus of this chapter on learning async rather than juggling parts
of the ecosystem, we’ve created the `trpl` crate (`trpl` is short for “The Rust
Programming Language”). It re-exports all the types, traits, and functions
you’ll need, primarily from the [`futures`][futures-crate]<!-- ignore --> and
[`tokio`][tokio]<!-- ignore --> crates. The `futures` crate is an official home
for Rust experimentation for async code, and it’s actually where the `Future`
trait was originally designed. Tokio is the most widely used async runtime in
Rust today, especially for web applications. There are other great runtimes out
there, and they may be more suitable for your purposes. We use the `tokio` crate
under the hood for `trpl` because it’s well tested and widely used.

In some cases, `trpl` also renames or wraps the original APIs to keep you
focused on the details relevant to this chapter. If you want to understand what
the crate does, we encourage you to check out [its source
code][crate-source]<!-- ignore -->. You’ll be able to see what crate each
re-export comes from, and we’ve left extensive comments explaining what the
crate does.

Create a new binary project named `hello-async` and add the `trpl` crate as a
dependency:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Now we can use the various pieces provided by `trpl` to write our first async
program. We’ll build a little command line tool that fetches two web pages,
pulls the `<title>` element from each, and prints out the title of whichever
page finishes that whole process first.

### Defining the page_title Function

Let’s start by writing a function that takes one page URL as a parameter, makes
a request to it, and returns the text of the title element (see Listing 17-1).

<Listing number="17-1" file-name="src/main.rs" caption="Defining an async function to get the title element from an HTML page">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

First, we define a function named `page_title` and mark it with the `async`
keyword. Then we use the `trpl::get` function to fetch whatever URL is passed in
and add the `await` keyword to await the response. To get the text of the
response, we call its `text` method, and once again await it with the `await`
keyword. Both of these steps are asynchronous. For the `get` function, we have
to wait for the server to send back the first part of its response, which will
include HTTP headers, cookies, and so on, and can be delivered separately from
the response body. Especially if the body is very large, it can take some time
for it all to arrive. Because we have to wait for the _entirety_ of the response
to arrive, the `text` method is also async.

We have to explicitly await both of these futures, because futures in Rust are
_lazy_: they don’t do anything until you ask them to with the `await` keyword.
(In fact, Rust will show a compiler warning if you don’t use a future.) This
might remind you of Chapter 13’s discussion of iterators in the section
[Processing a Series of Items With Iterators][iterators-lazy]<!-- ignore -->.
Iterators do nothing unless you call their `next` method—whether directly or by
using `for` loops or methods such as `map` that use `next` under the hood.
Likewise, futures do nothing unless you explicitly ask them to. This laziness
allows Rust to avoid running async code until it’s actually needed.

> Note: This is different from the behavior we saw in the previous chapter when
> using `thread::spawn` in [Creating a New Thread with
> spawn][thread-spawn]<!--ignore-->, where the closure we passed to another
> thread started running immediately. It’s also different from how many other
> languages approach async. But it’s important for Rust to be able to provide
> its performance guarantees, just as it is with iterators.

Once we have `response_text`, we can parse it into an instance of the `Html`
type using `Html::parse`. Instead of a raw string, we now have a data type we
can use to work with the HTML as a richer data structure. In particular, we can
use the `select_first` method to find the first instance of a given CSS
selector. By passing the string `"title"`, we’ll get the first `<title>` element
in the document, if there is one. Because there may not be any matching element,
`select_first` returns an `Option<ElementRef>`. Finally, we use the
`Option::map` method, which lets us work with the item in the `Option` if it’s
present, and do nothing if it isn’t. (We could also use a `match` expression
here, but `map` is more idiomatic.) In the body of the function we supply to
`map`, we call `inner_html` on the `title_element` to get its content, which is
a `String`. When all is said and done, we have an `Option<String>`.

Notice that Rust’s `await` keyword goes _after_ the expression you’re awaiting,
not before it. That is, it’s a _postfix_ keyword. This may differ from what
you’re used to if you’ve used `async` in other languages, but in Rust it makes
chains of methods much nicer to work with. As a result, we can change the body
of `page_title` to chain the `trpl::get` and `text` function calls together
with `await` between them, as shown in Listing 17-2.

<Listing number="17-2" file-name="src/main.rs" caption="Chaining with the `await` keyword">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

With that, we have successfully written our first async function! Before we add
some code in `main` to call it, let’s talk a little more about what we’ve
written and what it means.

When Rust sees a block marked with the `async` keyword, it compiles it into a
unique, anonymous data type that implements the `Future` trait. When Rust sees a
function marked with `async`, it compiles it into a non-async function whose
body is an async block. An async function’s return type is the type of the
anonymous data type the compiler creates for that async block.

Thus, writing `async fn` is equivalent to writing a function that returns a
_future_ of the return type. To the compiler, a function definition such as the
`async fn page_title` in Listing 17-1 is equivalent to a non-async function
defined like this:

```rust
# extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Let’s walk through each part of the transformed version:

- It uses the `impl Trait` syntax we discussed back in Chapter 10 in the
  [“Traits as Parameters”][impl-trait]<!-- ignore --> section.
- The returned trait is a `Future` with an associated type of `Output`. Notice
  that the `Output` type is `Option<String>`, which is the same as the original
  return type from the `async fn` version of `page_title`.
- All of the code called in the body of the original function is wrapped in an
  `async move` block. Remember that blocks are expressions. This whole block is
  the expression returned from the function.
- This async block produces a value with the type `Option<String>`, as just
  described. That value matches the `Output` type in the return type. This
  is just like other blocks you have seen.
- The new function body is an `async move` block because of how it uses the
  `url` parameter. (We’ll talk much more about `async` versus `async move` later
  in the chapter.)

Now we can call `page_title` in `main`.

## Determining a Single Page’s Title

To start, we’ll just get the title for a single page. In Listing 17-3, we follow
the same pattern we used in Chapter 12 to get command line arguments in the
[Accepting Command Line Arguments][cli-args]<!-- ignore --> section. Then we
pass the first URL `page_title` and await the result. Because the value
produced by the future is an `Option<String>`, we use a `match` expression to
print different messages to account for whether the page had a `<title>`.

<Listing number="17-3" file-name="src/main.rs" caption="Calling the `page_title` function from `main` with a user-supplied argument">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

Unfortunately, this code doesn’t compile. The only place we can use the `await`
keyword is in async functions or blocks, and Rust won’t let us mark the
special `main` function as `async`.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

The reason `main` can’t be marked `async` is that async code needs a _runtime_:
a Rust crate that manages the details of executing asynchronous code. A
program’s `main` function can _initialize_ a runtime, but it’s not a runtime
_itself_. (We’ll see more about why this is the case in a bit.) Every Rust
program that executes async code has at least one place where it sets up a
runtime and executes the futures.

Most languages that support async bundle a runtime, but Rust does not. Instead,
there are many different async runtimes available, each of which makes different
tradeoffs suitable to the use case it targets. For example, a high-throughput
web server with many CPU cores and a large amount of RAM has very different
needs than a microcontroller with a single core, a small amount of RAM, and no
heap allocation ability. The crates that provide those runtimes also often
supply async versions of common functionality such as file or network I/O.

Here, and throughout the rest of this chapter, we’ll use the `run` function from
the `trpl` crate, which takes a future as an argument and runs it to completion.
Behind the scenes, calling `run` sets up a runtime that’s used to run the future
passed in. Once the future completes, `run` returns whatever value the future
produced.

We could pass the future returned by `page_title` directly to `run`, and once it
completed, we could match on the resulting `Option<String>`, as
we tried to do in Listing 17-3. However, for most of the examples in the chapter
(and most async code in the real world), we’ll be doing more than just one
async function call, so instead we’ll pass an `async` block and explicitly
await the result of the `page_title` call, as in Listing 17-4.

<Listing number="17-4" caption="Awaiting an async block with `trpl::run`" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

When we run this code, we get the behavior we expected initially:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run https://www.rust-lang.org
# copy the output here
-->

```console
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

Phew—we finally have some working async code! But before we add the code to race
the two sites against each other, let’s briefly turn our attention back to how
futures work.

Each _await point_—that is, every place where the code uses the `await`
keyword—represents a place where control is handed back to the runtime. To
make that work, Rust needs to keep track of the state involved in the async
block so that the runtime can kick off some other work and then come back when
it’s ready to try advancing the first one again. This is an invisible state machine,
as if you’d written an enum like this to save the current state at each await
point:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Writing the code to transition between each state by hand would be tedious and
error-prone, however, especially when you need to add more functionality and
more states to the code later. Fortunately, the Rust compiler creates and
manages the state machine data structures for async code automatically. The
normal borrowing and ownership rules around data structures all still apply, and
happily, the compiler also handles checking those for us and provides useful
error messages. We’ll work through a few of those later in the chapter.

Ultimately, something has to execute this state machine, and that something is a
runtime. (This is why you may come across references to _executors_
when looking into runtimes: an executor is the part of a runtime responsible for
executing the async code.)

Now you can see why the compiler stopped us from making `main` itself an async
function back in Listing 17-3. If `main` were an async function, something else
would need to manage the state machine for whatever future `main` returned, but
`main` is the starting point for the program! Instead, we called the `trpl::run`
function in `main` to set up a runtime and run the future returned by the
`async` block until it is done.

> Note: Some runtimes provide macros so you _can_ write an async `main`
> function. Those macros rewrite `async fn main() { ... }` to be a normal `fn
> main`, which does the same thing we did by hand in Listing 17-4: call a
> function that runs a future to completion the way `trpl::run` does.

Now let’s put these pieces together and see how we can write concurrent code.

### Racing Our Two URLs Against Each Other

In Listing 17-5, we call `page_title` with two different URLs passed in from the
command line and race them.

<Listing number="17-5" caption="" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

We begin by calling `page_title` for each of the user-supplied URLs. We save the
resulting futures as `title_fut_1` and `title_fut_2`. Remember, these don’t do
anything yet, because futures are lazy and we haven’t yet awaited them. Then we
pass the futures to `trpl::race`, which returns a value to indicate which of the
futures passed to it finishes first.

> Note: Under the hood, `race` is built on a more general function, `select`,
> which you will encounter more often in real-world Rust code. A `select`
> function can do a lot of things that the `trpl::race` function can’t, but it
> also has some additional complexity that we can skip over for now.

Either future can legitimately “win,” so it doesn’t make sense to return a
`Result`. Instead, `race` returns a type we haven’t seen before,
`trpl::Either`. The `Either` type is somewhat similar to a `Result` in that it
has two cases. Unlike `Result`, though, there is no notion of success or
failure baked into `Either`. Instead, it uses `Left` and `Right` to indicate
“one or the other”:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

The `race` function returns `Left` with the output from the first future
argument it finishes first, or `Right` with the output of the second future
argument if that one finishes first. This matches the order the arguments appear
in when calling the function: the first argument is to the left of the second
argument.

We also update `page_title` to return the same URL passed in. That way, if
the page that returns first does not have a `<title>` we can resolve, we can
still print a meaningful message. With that information available, we wrap up by
updating our `println!` output to indicate both which URL finished first and
what, if any, the `<title>` is for the web page at that URL.

You have built a small working web scraper now! Pick a couple URLs and run the
command line tool. You may discover that some sites are consistently faster than
others, while in other cases the faster site varies from run to run. More
importantly, you’ve learned the basics of working with futures, so now we can
dig deeper into what we can do with async.

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[cli-args]: ch12-01-accepting-command-line-arguments.html

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
