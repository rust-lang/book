[TOC]

## Async and Await

Many operations we ask the computer to do can take a while to finish. For
example, if you used a video editor to create a video of a family celebration,
exporting it could take anywhere from minutes to hours. Similarly, downloading a
video shared by someone in your family might take a long time. It would be nice
if we could do something else while we are waiting for those long-running
processes to complete.

The video export will use as much CPU and GPU power as it can. If you only had
one CPU core, and your operating system never paused that export until it
completed, you couldn’t do anything else on your computer while it was running.
That would be a pretty frustrating experience, though. Instead, your computer’s
operating system can—and does!—invisibly interrupt the export often enough to
let you get other work done along the way.

The file download is different. It does not take up very much CPU time. Instead,
the CPU needs to wait on data to arrive from the network. While you can start
reading the data once some of it is present, it might take a while for the rest
to show up. Even once the data is all present, a video can be quite large, so it
might take some time to load it all. Maybe it only takes a second or two—but
that’s a very long time for a modern processor, which can do billions of
operations every second. It would be nice to be able to put the CPU to use for
other work while waiting for the network call to finish—so, again, your
operating system will invisibly interrupt your program so other things can
happen while the network operation is still ongoing.

> Note: The video export is the kind of operation which is often described as
> “CPU-bound” or “compute-bound”. It’s limited by the speed of the computer’s
> ability to process data within the *CPU* or *GPU*, and how much of that speed
> it can use. The video download is the kind of operation which is often
> described as “IO-bound,” because it’s limited by the speed of the computer’s
> *input and output*. It can only go as fast as the data can be sent across the
> network.

In both of these examples, the operating system’s invisible interrupts provide a
form of concurrency. That concurrency only happens at the level of a whole
program, though: the operating system interrupts one program to let other
programs get work done. In many cases, because we understand our programs at a
much more granular level than the operating system does, we can spot lots of
opportunities for concurrency that the operating system cannot see.

For example, if we’re building a tool to manage file downloads, we should be
able to write our program in such a way that starting one download does not lock
up the UI, and users should be able to start multiple downloads at the same
time. Many operating system APIs for interacting with the network are
*blocking*, though. That is, these APIs block the program’s progress until the
data that they are processing is completely ready.

> Note: This is how *most* function calls work, if you think about it! However,
> we normally reserve the term “blocking” for function calls which interact with
> files, the network, or other resources on the computer, because those are the
> places where an individual program would benefit from the operation being
> *non*-blocking.

We could avoid blocking our main thread by spawning a dedicated thread to
download each file. However, we would eventually find that the overhead of those
threads was a problem. It would also be nicer if the call were not blocking in
the first place. Last but not least, it would be better if we could write in the
same direct style we use in blocking code. Something similar to this:

```
let data = fetch_data_from(url).await;
println!("{data}");
```

That is exactly what Rust’s async abstraction gives us. Before we see how this
works in practice, though, we need to take a short detour into the differences
between parallelism and concurrency.

### Parallelism and Concurrency

In the “Fearless Concurrency” chapter on page XX, we treated parallelism and
concurrency as mostly interchangeable. Now we need to distinguish between them
more precisely, because the differences will show up as we start working.

Consider the different ways a team could split up work on a software project. We
could assign a single individual multiple tasks, or we could assign one task per
team member, or we could do a mix of both approaches.

When an individual works on several different tasks before any of them is
complete, this is *concurrency*. Maybe you have two different projects checked
out on your computer, and when you get bored or stuck on one project, you switch
to the other. You’re just one person, so you can’t make progress on both tasks
at the exact same time—but you can multi-task, making progress on multiple
tasks by switching between them.

<img alt="Concurrent work flow" src="img/trpl17-01.svg" />

Figure 17-1: A concurrent workflow, switching between Task A and Task B.

When you agree to split up a group of tasks between the people on the team, with
each person taking one task and working on it alone, this is *parallelism*. Each
person on the team can make progress at the exact same time.

<img alt="Concurrent work flow" src="img/trpl17-02.svg" />

Figure 17-2: A parallel workflow, where work happens on Task A and Task B
independently.

With both of these situations, you might have to coordinate between different
tasks. Maybe you *thought* the task that one person was working on was totally
independent from everyone else’s work, but it actually needs something finished
by another person on the team. Some of the work could be done in parallel, but
some of it was actually *serial*: it could only happen in a series, one thing
after the other, as in Figure 17-3.

<img alt="Concurrent work flow" src="img/trpl17-03.svg" class="center" />

Figure 17-3: A partially parallel workflow, where work happens on Task A and Task B independently until task A3 is blocked on the results of task B3.

Likewise, you might realize that one of your own tasks depends on another of
your tasks. Now your concurrent work has also become serial.

Parallelism and concurrency can intersect with each other, too. If you learn
that a colleague is stuck until you finish one of your tasks, you’ll probably
focus all your efforts on that task to “unblock” your colleague. You and your
coworker are no longer able to work in parallel, and you’re also no longer able
to work concurrently on your own tasks.

The same basic dynamics come into play with software and hardware. On a machine
with a single CPU core, the CPU can only do one operation at a time, but it can
still work concurrently. Using tools such as threads, processes, and async, the
computer can pause one activity and switch to others before eventually cycling
back to that first activity again. On a machine with multiple CPU cores, it can
also do work in parallel. One core can be doing one thing while another core
does something completely unrelated, and those actually happen at the same
time.

When working with async in Rust, we’re always dealing with concurrency.
Depending on the hardware, the operating system, and the async runtime we are
using—more on async runtimes shortly!—that concurrency may also use parallelism
under the hood.

Now, let’s dive into how async programming in Rust actually works! In the rest
of this chapter, we will:

* see how to use Rust’s `async` and `await` syntax
* explore how to use the async model to solve some of the same challenges we
  looked at in Chapter 16
* look at how multithreading and async provide complementary solutions, which
  you can even use together in many cases

## Futures and the Async Syntax

The key elements of asynchronous programming in Rust are *futures* and Rust’s
`async` and `await` keywords.

A *future* is a value which may not be ready now, but will become ready at some
point in the future. (This same concept shows up in many languages, sometimes
under other names such as “task” or “promise”.) Rust provides a `Future` trait
as a building block so different async operations can be implemented with
different data structures, but with a common interface. In Rust, we say that
types which implement the `Future` trait are futures. Each type which
implements `Future` holds its own information about the progress that has been
made and what “ready” means.

The `async` keyword can be applied to blocks and functions to specify that they
can be interrupted and resumed. Within an async block or async function, you can
use the `await` keyword to wait for a future to become ready, called *awaiting a
future*. Each place you await a future within an async block or function is a
place that async block or function may get paused and resumed. The process of
checking with a future to see if its value is available yet is called *polling*.

Some other languages also use `async` and `await` keywords for async
programming. If you’re familiar with those languages, you may notice some
significant differences in how Rust does things, including how it handles the
syntax. That’s for good reason, as we’ll see!

Most of the time when writing async Rust, we use the `async` and `await`
keywords. Rust compiles them into equivalent code using the `Future` trait, much
as it compiles `for` loops into equivalent code using the `Iterator` trait.
Because Rust provides the `Future` trait, though, you can also implement it for
your own data types when you need to. Many of the functions we’ll see
throughout this chapter return types with their own implementations of `Future`.
We’ll return to the definition of the trait at the end of the chapter and dig
into more of how it works, but this is enough detail to keep us moving forward.

That may all feel a bit abstract. Let’s write our first async program: a little
web scraper. We’ll pass in two URLs from the command line, fetch both of them
concurrently, and return the result of whichever one finishes first. This
example will have a fair bit of new syntax, but don’t worry. We’ll explain
everything you need to know as we go.

### Our First Async Program

To keep this chapter focused on learning async, rather than juggling parts of
the ecosystem, we have created the `trpl` crate (`trpl` is short for “The Rust
Programming Language”). It re-exports all the types, traits, and functions
you’ll need, primarily from the `futures` and `tokio` crates, available on
*https://crates.io*.

* The `futures` crate is an official home for Rust experimentation for async
  code, and is actually where the `Future` type was originally designed.

* Tokio is the most widely used async runtime in Rust today, especially (but
  not only!) for web applications. There are other great runtimes out there,
  and they may be more suitable for your purposes. We use Tokio under the hood
  for `trpl` because it’s well-tested and widely used.

In some cases, `trpl` also renames or wraps the original APIs to let us stay
focused on the details relevant to this chapter. If you want to understand what
the crate does, we encourage you to check out its source code at
*https://github.com/rust-lang/book/tree/main/packages/trpl*.
You’ll be able to see what crate each re-export comes from, and we’ve left
extensive comments explaining what the crate does.

Create a new binary project named `hello-async` and add the `trpl` crate as a
dependency:

```
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Now we can use the various pieces provided by `trpl` to write our first async
program. We’ll build a little command line tool which fetches two web pages,
pulls the `<title>` element from each, and prints out the title of whichever
finishes that whole process first.

Let’s start by writing a function that takes one page URL as a parameter, makes
a request to it, and returns the text of the title element:

Filename: src/main.rs

```
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}
```

Listing 17-1: Defining an async function to get the title element from an HTML page

In Listing 17-1, we define a function named `page_title`, and we mark it with
the `async` keyword. Then we use the `trpl::get` function to fetch whatever URL
is passed in, and, and we await the response by using the `await` keyword. Then
we get the text of the response by calling its `text` method and once again
awaiting it with the `await` keyword. Both of these steps are asynchronous. For
`get`, we need to wait for the server to send back the first part of its
response, which will include HTTP headers, cookies, and so on. That part of the
response can be delivered separately from the body of the request. Especially if
the body is very large, it can take some time for it all to arrive. Thus, we
have to wait for the *entirety* of the response to arrive, so the `text` method
is also async.

We have to explicitly await both of these futures, because futures in Rust are
*lazy*: they don’t do anything until you ask them to with `await`. (In fact,
Rust will show a compiler warning if you don’t use a future.) This should
remind you of our discussion of iterators back in the “Processing a Series of
Items with Iterators” section of Chapter 13 on page XX. Iterators do nothing
unless you call their `next` method—whether directly, or using `for` loops or
methods such as `map` which use `next` under the hood. With futures, the same
basic idea applies: they do nothing unless you explicitly ask them to. This
laziness allows Rust to avoid running async code until it’s actually needed.

> Note: This is different from the behavior we saw when using `thread::spawn` in
> the “Creating a New Thread with `spawn`” section of Chapter 16 on page XX,
> where the closure we passed to another thread started running immediately.
> It’s also different from how many other languages approach async! But it’s
> important for Rust. We’ll see why that is later.

Once we have `response_text`, we can then parse it into an instance of the
`Html` type using `Html::parse`. Instead of a raw string, we now have a data
type we can use to work with the HTML as a richer data structure. In particular,
we can use the `select_first` method to find the first instance of a given CSS
selector. By passing the string `"title"`, we’ll get the first `<title>`
element in the document, if there is one. Because there may not be any matching
element, `select_first` returns an `Option<ElementRef>`. Finally, we use the
`Option::map` method, which lets us work with the item in the `Option` if it’s
present, and do nothing if it isn’t. (We could also use a `match` expression
here, but `map` is more idiomatic.) In the body of the function we supply to
`map`, we call `inner_html` on the `title_element` to get its content, which is
a `String`. When all is said and done, we have an `Option<String>`.

Notice that Rust’s `await` keyword goes after the expression you’re awaiting,
not before it. That is, it’s a *postfix keyword*. This may be different from
what you might be used to if you have used async in other languages. Rust chose
this because it makes chains of methods much nicer to work with. As a result, we
can change the body of `page_url_for` to chain the `trpl::get` and `text`
function calls together with `await` between them, as shown in Listing 17-2:

Filename: src/main.rs

```
    let response_text = trpl::get(url).await.text().await;
```

Listing 17-2: Chaining with the `await` keyword

With that, we have successfully written our first async function! Before we add
some code in `main` to call it, let’s talk a little more about what we’ve
written and what it means.

When Rust sees a block marked with the `async` keyword, it compiles it into a
unique, anonymous data type which implements the `Future` trait. When Rust sees
a function marked with `async`, it compiles it into a non-async function whose
body is an async block. An async function’s return type is the type of the of
the anonymous data type the compiler creates for that async block.

Thus, writing `async fn` is equivalent to writing a function which returns a
*future* of the return type. When the compiler sees a function definition such
as the `async fn page_title` in Listing 17-1, it’s equivalent to a non-async
function defined like this:

```
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Let’s walk through each part of the transformed version:

* It uses the `impl Trait` syntax we discussed back in the “Traits as
  Parameters” section in Chapter 10 on page XX.
* The returned trait is a `Future`, with an associated type of `Output`. Notice
  that the `Output` type is `Option<String>`, which is the same as the the
  original return type from the `async fn` version of `page_title`.
* All of the code called in the body of the original function is wrapped in an
  `async move` block. Remember that blocks are expressions. This whole block is
  the expression returned from the function.
* This async block produces a value with the type `Option<String>`, as described
  above. That value matches the `Output` type in the return type. This is just
  like other blocks you have seen.
* The new function body is an `async move` block because of how it uses the
  `url` parameter. (We’ll talk about `async` vs. `async move` much more later
  in the chapter.)
* The new version of the function has a kind of lifetime we haven’t seen before
  in the output type: `'_`. Because the function returns a `Future` which refers
  to a reference—in this case, the reference from the `url` parameter—we need to
  tell Rust that we mean for that reference to be included. We don’t have to
  name the lifetime here, because Rust is smart enough to know there is only one
  reference which could be involved, but we *do* have to be explicit that the
  resulting `Future` is bound by that lifetime.

Now we can call `page_title` in `main`. To start, we’ll just get the title for
a single page. In Listing 17-3, we follow the same pattern we used for getting
command line arguments back in the “Accepting Command Line Arguments” section
of Chapter 12 on page XX. Then we pass the first URL `page_title`, and await
the result. Because the value produced by the future is an `Option<String>`, we
use a `match` expression to print different messages to account for whether the
page had a `<title>`.

Filename: src/main.rs

```
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
}
```

Listing 17-3: Calling the `page_title` function from `main` with a
user-supplied argument

Unfortunately, this doesn’t compile. The only place we can use the `await`
keyword is in async functions or blocks, and Rust won’t let us mark the
special `main` function as `async`.

```
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

The reason `main` can’t be marked `async` is that async code needs a *runtime*:
a Rust crate which manages the details of executing asynchronous code. A
program’s `main` function can *initialize* a runtime, but it’s not a runtime
*itself*. (We’ll see more about why this is a bit later.) Every Rust program
that executes async code has at least one place where it sets up a runtime and
executes the futures.

Most languages which support async bundle a runtime with the language. Rust does
not. Instead, there are many different async runtimes available, each of which
makes different tradeoffs suitable to the use case they target. For example, a
high-throughput web server with many CPU cores and a large amount of RAM has
very different different needs than a microcontroller with a single core, a
small amount of RAM, and no ability to do heap allocations. The crates which
provide those runtimes also often supply async versions of common functionality
such as file or network I/O.

Here, and throughout the rest of this chapter, we’ll use the `run` function
from the `trpl` crate, which takes a future as an argument and runs it to
completion. Behind the scenes, calling `run` sets up a runtime to use to run the
future passed in. Once the future completes, `run` returns whatever value the
future produced.

We could pass the future returned by `page_title` directly to `run`. Once it
completed, we would be able to match on the resulting `Option<String>`, the way
we tried to do in Listing 17-3. However, for most of the examples in the chapter
(and most async code in the real world!), we’ll be doing more than just one
async function call, so instead we’ll pass an `async` block and explicitly
await the result of calling `page_title`, as in Listing 17-4.

Filename: src/main.rs

```
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
```

Listing 17-4: Awaiting an async block with `trpl::run`

When we run this, we get the behavior we might have expected initially:

```
$ cargo run "http://www.rust-lang.org"
The title for http://www.rust-lang.org was
            Rust Programming Language
```

Phew: we finally have some working async code! This now compiles, and we can run
it. Before we add code to race two sites against each other, let’s briefly turn
our attention back to how futures work.

Each *await point*—that is, every place where the code uses the `await`
keyword—represents a place where control gets handed back to the runtime. To
make that work, Rust needs to keep track of the state involved in the async
block, so that the runtime can kick off some other work and then come back when
it’s ready to try advancing this one again. This is an invisible state machine,
as if you wrote an enum in this way to save the current state at each `await`
point:

```
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}
```

Writing the code to transition between each state by hand would be tedious and
error-prone, especially when adding more functionality and more states to the
code later. Instead, the Rust compiler creates and manages the state machine
data structures for async code automatically. If you’re wondering: yep, the
normal borrowing and ownership rules around data structures all apply. Happily,
the compiler also handles checking those for us, and has good error messages.
We’ll work through a few of those later in the chapter!

Ultimately, something has to execute that state machine. That something is a
runtime. (This is why you may sometimes come across references to *executors*
when looking into runtimes: an executor is the part of a runtime responsible for
executing the async code.)

Now we can understand why the compiler stopped us from making `main` itself an
async function back in Listing 17-3. If `main` were an async function, something
else would need to manage the state machine for whatever future `main` returned,
but `main` is the starting point for the program! Instead, we call the
`trpl::run` function in `main`, which sets up a runtime and runs the future
returned by the `async` block until it returns `Ready`.

> Note: some runtimes provide macros to make it so you *can* write an async
> main function. Those macros rewrite `async fn main() { ... }` to be a normal
> `fn main` which does the same thing we did by hand in Listing 17-5: call a
> function which runs a future to completion the way `trpl::run` does.

Let’s put these pieces together and see how we can write concurrent code, by
calling `page_title` with two different URLs passed in from the command line
and racing them.

Filename: src/main.rs

```
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
```

Listing 17-5: Calling `page_title` for two URLs to see which returns first

In Listing 17-5, we begin by calling `page_title` for each of the user-supplied
URLs. We save the futures produced by calling `page_title` as `title_fut_1` and
`title_fut_2`. Remember, these don’t do anything yet, because futures are lazy,
and we haven’t yet awaited them. Then we pass the futures to `trpl::race`,
which returns a value to indicate which of the futures passed to it finishes
first.

> Note: Under the hood, `race` is built on a more general function, `select`,
> which you will encounter more often in real-world Rust code. A `select`
> function can do a lot of things that `trpl::race` function can’t, but it also
> has some additional complexity that we can skip over for now.

Either future can legitimately “win,” so it doesn’t make sense to return a
`Result`. Instead, `race` returns a type we haven’t seen before,
`trpl::Either`. The `Either` type is somewhat similar to a `Result`, in that it
has two cases. Unlike `Result`, though, there is no notion of success or
failure baked into `Either`. Instead, it uses `Left` and `Right` to indicate
“one or the other”.

```
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

The `race` function returns `Left` if the first argument finishes first, with
that future’s output, and `Right` with the second future argument’s output if
*that* one finishes first. This matches the order the arguments appear when
calling the function: the first argument is to the left of the second argument.

We also update `page_title` to return the same URL passed in. That way, if
the page which returns first does not have a `<title>` we can resolve, we can
still print a meaningful message. With that information available, we wrap up by
updating our `println!` output to indicate both which URL finished first and
what the `<title>` was for the web page at that URL, if any.

You have built a small working web scraper now! Pick a couple URLs and run the
command line tool. You may discover that some sites are reliably faster than
others, while in other cases which site “wins” varies from run to run. More
importantly, you’ve learned the basics of working with futures, so we can now
dig into even more of the things we can do with async.

## Concurrency With Async

In this section, we’ll apply async to some of the same concurrency challenges
we tackled with threads in Chapter 16. Because we already talked about a lot of
the key ideas there, in this section we’ll focus on what’s different between
threads and futures.

In many cases, the APIs for working with concurrency using async are very
similar to those for using threads. In other cases, they end up being shaped
quite differently. Even when the APIs *look* similar between threads and async,
they often have different behavior—and they nearly always have different
performance characteristics.

### Counting

The first task we tackled in the “Creating a New Thread with spawn” section of
Chapter 16 on page XX was counting up on two separate threads. Let’s do the
same using async. The `trpl` crate supplies a `spawn_task` function which looks
very similar to the `thread::spawn` API, and a `sleep` function which is an
async version of the `thread::sleep` API. We can use these together to
implement the same counting example as with threads, in Listing 17-6.

Filename: src/main.rs

```
use std::time::Duration;

fn main() {
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

Listing 17-6: Using `spawn_task` to count with two

As our starting point, we set up our `main` function with `trpl::run`, so
that our top-level function can be async.

> Note: From this point forward in the chapter, every example will include this
> exact same wrapping code with `trpl::run` in `main`, so we’ll often skip it
> just as we do with `main`. Don’t forget to include it in your code!

Then we write two loops within that block, each with a `trpl::sleep` call in it,
which waits for half a second (500 milliseconds) before sending the next
message. We put one loop in the body of a `trpl::spawn_task` and the other in a
top-level `for` loop. We also add an `await` after the `sleep` calls.

This does something similar to the thread-based implementation—including the
fact that you may see the messages appear in a different order in your own
terminal when you run it.

```
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

This version stops as soon as the for loop in the body of the main async block
finishes, because the task spawned by `spawn_task` is shut down when the main
function ends. If you want to run all the way to the completion of the task, you
will need to use a join handle to wait for the first task to complete. With
threads, we used the `join` method to “block” until the thread was done running.
In Listing 17-7, we can use `await` to do the same thing, because the task
handle itself is a future. Its `Output` type is a `Result`, so we also unwrap it
after awaiting it.

Filename: src/main.rs

```
let handle = trpl::spawn_task(async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
});

for i in 1..5 {
    println!("hi number {i} from the second task!");
    trpl::sleep(Duration::from_millis(500)).await;
}

handle.await.unwrap();
```

Listing 17-7: Using `await` with a join handle to run a task to completion

This updated version runs till *both* loops finish.

```
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

So far, it looks like async and threads give us the same basic outcomes, just
with different syntax: using `await` instead of calling `join` on the join
handle, and awaiting the `sleep` calls.

The bigger difference is that we didn’t need to spawn another operating system
thread to do this. In fact, we don’t even need to spawn a task here. Because
async blocks compile to anonymous futures, we can put each loop in an async
block and have the runtime run them both to completion using the `trpl::join`
function.

In the “Waiting for All Threads to Finish Using `join` Handles” section of
Chapter 16 on page XX, we showed how to use the `join` method on the
`JoinHandle` type returned when you call `std::thread::spawn`. The `trpl::join`
function is similar, but for futures. When you give it two futures, it produces
a single new future whose output is a tuple with the output of each of the
futures you passed in once *both* complete. Thus, in Listing 17-8, we use
`trpl::join` to wait for both `fut1` and `fut2` to finish. We do *not* await
`fut1` and `fut2`, but instead the new future produced by `trpl::join`. We
ignore the output, because it’s just a tuple with two unit values in it.

Filename: src/main.rs

```
let fut1 = async {
    for i in 1..10 {
        println!("hi number {i} from the first task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let fut2 = async {
    for i in 1..5 {
        println!("hi number {i} from the second task!");
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

trpl::join(fut1, fut2).await;
```

Listing 17-8: Using `trpl::join` to await two anonymous futures

When we run this, we see both futures run to completion:

```
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Here, you’ll see the exact same order every time, which is very different from
what we saw with threads. That is because the `trpl::join` function is *fair*,
meaning it checks each future equally often, alternating between them, and never
lets one race ahead if the other is ready. With threads, the operating system
decides which thread to check and how long to let it run. With async Rust, the
runtime decides which task to check. (In practice, the details get complicated
because an async runtime might use operating system threads under the hood as
part of how it manages concurrency, so guaranteeing fairness can be more work
for a runtime—but it’s still possible!) Runtimes don’t have to guarantee
fairness for any given operation, and runtimes often offer different APIs to let
you choose whether you want fairness or not.

Try some of these different variations on awaiting the futures and see what they
do:

* Remove the async block from around either or both of the loops.
* Await each async block immediately after defining it.
* Wrap only the first loop in an async block, and await the resulting future
  after the body of second loop.

For an extra challenge, see if you can figure out what the output will be in
each case *before* running the code!

### Message Passing

Sharing data between futures will also be familiar: we’ll use message passing
again, but this with async versions of the types and functions. We’ll take a
slightly different path than we did in the “Using Message Passing to Transfer
Data Between Threads” section of Chapter 16 on page XX, to illustrate some of
the key differences between thread-based and futures-based concurrency. In
Listing 17-9, we’ll begin with just a single async block—*not* spawning a
separate task as we spawned a separate thread.

Filename: src/main.rs

```
let (tx, mut rx) = trpl::channel();

let val = String::from("hi");
tx.send(val).unwrap();

let received = rx.recv().await.unwrap();
println!("Got: {received}");
```

Listing 17-9: Creating an async channel and assigning the two halves to `tx`
and `rx`

Here, we use `trpl::channel`, an async version of the multiple-producer,
single-consumer channel API we used with threads back in the “Using Message
Passing to Transfer Data Between Threads” section of Chapter 16 on page XX. The
async version of the API is only a little different from the thread-based
version: it uses a mutable rather than an immutable receiver `rx`, and its
`recv` method produces a future we need to await rather than producing the
value directly. Now we can send messages from the sender to the receiver.
Notice that we don’t have to spawn a separate thread or even a task; we merely
need to await the `rx.recv` call.

The synchronous `Receiver::recv` method in `std::mpsc::channel` blocks until
it receives a message. The `trpl::Receiver::recv` method does not, because it
is async. Instead of blocking, it hands control back to the runtime until either
a message is received or the send side of the channel closes. By contrast, we
don’t await the `send` call, because it doesn’t block. It doesn’t need to,
because the channel we’re sending it into is unbounded.

> Note: Because all of this async code runs in an async block in a `trpl::run`
> call, everything within it can avoid blocking. However, the code *outside* it
> will block on the `run` function returning. That is the whole point of the
> `trpl::run` function: it lets you *choose* where to block on some set of async
> code, and thus where to transition between sync and async code. In most async
> runtimes, `run` is actually named `block_on` for exactly this reason.

Notice two things about this example: First, the message will arrive right away!
Second, although we use a future here, there’s no concurrency yet. Everything
in the listing happens in sequence, just as it would if there were no futures
involved.

Let’s address the first part by sending a series of messages, and sleep in
between them, as shown in Listing 17-10:

Filename: src/main.rs

```
let (tx, mut rx) = trpl::channel();

let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("future"),
];

for val in vals {
    tx.send(val).unwrap();
    trpl::sleep(Duration::from_millis(500)).await;
}

while let Some(value) = rx.recv().await {
    println!("received '{value}'");
}
```

Listing 17-10: Sending and receiving multiple messages over the async channel
and sleeping with an `await` between each message

In addition to sending the messages, we need to receive them. In this case, we
could do that manually, by just doing `rx.recv().await` four times, because we
know how many messages are coming in. In the real world, though, we’ll
generally be waiting on some *unknown* number of messages. In that case, we need
to keep waiting until we determine that there are no more messages.

In Listing 16-10, we used a `for` loop to process all the items received from a
synchronous channel. However, Rust doesn’t yet have a way to write a `for` loop
over an *asynchronous* series of items. Instead, we need to use a new kind of
loop we haven’t seen before, the `while let` conditional loop. A `while let`
loop is the loop version of the `if let` construct we saw back in the “Concise
Control Flow with `if let`” section in Chapter 6 on page XX. The loop will
continue executing as long as the pattern it specifies continues to match the
value.

The `rx.recv` call produces a `Future`, which we await. The runtime will pause
the `Future` until it is ready. Once a message arrives, the future will resolve
to `Some(message)`, as many times as a message arrives. When the channel closes,
regardless of whether *any*  messages have arrived, the future will instead
resolve to `None` to indicate that there are no more values, and we should stop
polling—that is, stop awaiting.

The `while let` loop pulls all of this together. If the result of calling
`rx.recv().await` is `Some(message)`, we get access to the message and we can
use it in the loop body, just as we could with `if let`. If the result is
`None`, the loop ends. Every time the loop completes, it hits the await point
again, so the runtime pauses it again until another message arrives.

The code now successfully sends and receives all of the messages. Unfortunately,
there are still a couple problems. For one thing, the messages do not arrive at
half-second intervals. They arrive all at once, two seconds (2,000 milliseconds)
after we start the program. For another, this program also never exits! Instead,
it waits forever for new messages. You will need to shut it down using <span
class="keystroke">ctrl-c</span>.

Let’s start by understanding why the messages all come in at once after the full
delay, rather than coming in with delays in between each one. Within a given
async block, the order that `await` keywords appear in the code is also the
order they happen when running the program.

There’s only one async block in Listing 17-10, so everything in it runs
linearly. There’s still no concurrency. All the `tx.send` calls happen,
interspersed with all of the `trpl::sleep` calls and their associated await
points. Only then does the `while let` loop get to go through any of the `await`
points on the `recv` calls.

To get the behavior we want, where the sleep delay happens between receiving
each message, we need to put the `tx` and `rx` operations in their own async
blocks. Then the runtime can execute each of them separately using `trpl::join`,
just as in the counting example. Once again, we await the result of calling
`trpl::join`, not the individual futures. If we awaited the individual futures
in sequence, we would just end up back in a sequential flow—exactly what we’re
trying *not* to do.

Filename: src/main.rs

```
let tx_fut = async {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let rx_fut = async {
    while let Some(value) = rx.recv().await {
        println!("received '{value}'");
    }
};

trpl::join(tx_fut, rx_fut).await;
```

Listing 17-11: Separating `send` and `recv` into their own `async` blocks and
awaiting the futures for those blocks

With the updated code in Listing 17-11, the messages get printed at
500-millisecond intervals, rather than all in a rush after two seconds.

The program still never exits, though, because of the way `while let` loop
interacts with `trpl::join`:

* The future returned from `trpl::join` only completes once *both* futures
  passed to it have completed.
* The `tx` future completes once it finishes sleeping after sending the last
  message in `vals`.
* The `rx` future won’t complete until the `while let` loop ends.
* The `while let` loop won’t end until awaiting `rx.recv` produces `None`.
* Awaiting `rx.recv` will only return `None` once the other end of the channel
  is closed.
* The channel will only close if we call `rx.close` or when the sender side,
  `tx`, is dropped.
* We don’t call `rx.close` anywhere, and `tx` won’t be dropped until the
  outermost async block passed to `trpl::run` ends.
* The block can’t end because it is blocked on `trpl::join` completing, which
  takes us back to the top of this list!

We could manually close `rx` by calling `rx.close` somewhere, but that doesn’t
make much sense. Stopping after handling some arbitrary number of messages would
make the program shut down, but we could miss messages. We need some other way
to make sure that `tx` gets dropped *before* the end of the function.

Right now, the async block where we send the messages only borrows `tx` because
sending a message doesn’t require ownership, but if we could move `tx` into
that async block, it would be dropped once that block ends. In the “Capturing
References or Moving Ownership” section of Chapter 13 on page XX, we learned
how to use the `move` keyword with closures, and in the “Using `move` Closures
with Threads” section of Chapter 16 on page XX, we saw that we often need to
move data into closures when working with threads. The same basic dynamics
apply to async blocks, so the `move` keyword works with async blocks just as it
does with closures.

In Listing 17-12, we change the async block for sending messages from a plain
`async` block to an `async move` block. When we run *this* version of the code,
it shuts down gracefully after the last message is sent and received.

Filename: src/main.rs

```
let (tx, mut rx) = trpl::channel();

let tx_fut = async move {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let rx_fut = async {
    while let Some(value) = rx.recv().await {
        eprintln!("received '{value}'");
    }
};

trpl::join(tx_fut, rx_fut).await;
```

Listing 17-12: A working example of sending and receiving messages between
futures which correctly shuts down when complete

This async channel is also a multiple-producer channel, so we can call `clone`
on `tx` if we want to send messages from multiple futures. In Listing 17-13, we
clone `tx`, creating `tx1` outside the first async block. We move `tx1` into
that block just as we did before with `tx`. Then, later, we move the original
`tx` into a *new* async block, where we send more messages on a slightly slower
delay. We happen to put this new async block after the async block for receiving
messages, but it could go before it just as well. The key is the order of the
futures are awaited in, not the order they are created in.

Both of the async blocks for sending messages need to be `async move` blocks, so
that both `tx` and `tx1` get dropped when those blocks finish. Otherwise we’ll
end up back in the same infinite loop we started out in. Finally, we switch from
`trpl::join` to `trpl::join3` to handle the additional future.

Filename: src/main.rs

```
let (tx, mut rx) = trpl::channel();

let tx1 = tx.clone();
let tx1_fut = async move {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    }
};

let rx_fut = async {
    while let Some(value) = rx.recv().await {
        println!("received '{value}'");
    }
};

let tx_fut = async move {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(1500)).await;
    }
};

trpl::join3(tx1_fut, tx_fut, rx_fut).await;
```

Listing 17-13: Using multiple producers with async blocks

Now we see all the messages from both sending futures. Because the sending
futures use slightly different delays after sending, the messages are also
received at those different intervals.

```
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

This is a good start, but it limits us to just a handful of futures: two with
`join`, or three with `join3`. Let’s see how we might work with more futures.

## Working With Any Number of Futures

When we switched from using two futures to three in the previous section, we
also had to switch from using `join` to using `join3`. It would be annoying to
have to call a different function every time we changed the number of futures we
wanted to join. Happily, we have a macro form of `join` to which we can pass an
arbitrary number of arguments. It also handles awaiting the futures itself.
Thus, we could rewrite the code from Listing 17-13 to use `join!` instead of
`join3`, as in Listing 17-14:

Filename: src/main.rs

```
trpl::join!(tx1_fut, tx_fut, rx_fut);
```

Listing 17-14: Using `join!` to wait for multiple futures

This is definitely a nice improvement over needing to swap between `join` and
`join3` and `join4` and so on! However, even this macro form only works when we
know the number of futures ahead of time. In real-world Rust, though, pushing
futures into a collection and then waiting on some or all the futures in that
collection to complete is a common pattern.

To check all the futures in some collection, we’ll need to iterate over and
join on *all* of them. The `trpl::join_all` function accepts any type which
implements the `Iterator` trait, which we learned about back in “The Iterator
Trait and the next Method” section of Chapter 13 on page XX, so it seems like
just the ticket. Let’s try putting our futures in a vector, and replace `join!`
with `join_all`.

```
let futures = vec![tx1_fut, rx_fut, tx_fut];

trpl::join_all(futures).await;
```

Listing 17-15: Storing anonymous futures in a vector and calling `join_all`

Unfortunately, this doesn’t compile. Instead, we get this error:

```
error[E0308]: mismatched types
  --> src/main.rs:43:37
   |
8  |           let tx1_fut = async move {
   |  _______________________-
9  | |             let vals = vec![
10 | |                 String::from("hi"),
11 | |                 String::from("from"),
...  |
19 | |             }
20 | |         };
   | |_________- the expected `async` block
21 |
22 |           let rx_fut = async {
   |  ______________________-
23 | |             while let Some(value) = rx.recv().await {
24 | |                 println!("received '{value}'");
25 | |             }
26 | |         };
   | |_________- the found `async` block
...
43 |           let futures = vec![tx1_fut, rx_fut, tx_fut];
   |                                       ^^^^^^ expected `async` block, found a different `async` block
   |
   = note: expected `async` block `{async block@src/main.rs:8:23: 20:10}`
              found `async` block `{async block@src/main.rs:22:22: 26:10}`
   = note: no two async blocks, even if identical, have the same type
   = help: consider pinning your async block and and casting it to a trait object
```

This might be surprising. After all, none of them return anything, so each
block produces a `Future<Output = ()>`. However, `Future` is a trait, not a
concrete type. The concrete types are the individual data structures generated
by the compiler for async blocks. You can’t put two different hand-written
structs in a `Vec`, and the same thing applies to the different structs
generated by the compiler.

To make this work, we need to use *trait objects*, just as we did in the
“Returning Errors from the run function” section in Chapter 12 on page XX.
(We’ll cover trait objects in detail in Chapter 18.) Using trait objects lets
us treat each of the anonymous futures produced by these types as the same
type, because all of them implement the `Future` trait.

> Note: In the “Using an Enum to Store Multiple Types” section of Chapter 8 on
> page XX, we discussed another way to include multiple types in a `Vec`: using
> an enum to represent each of the different types which can appear in the
> vector. We can’t do that here, though. For one thing, we have no way to name
> the different types, because they are anonymous. For another, the reason we
> reached for a vector and `join_all` in the first place was to be able to work
> with a dynamic collection of futures where we don’t know what they will all
> be until runtime.

We start by wrapping each of the futures in the `vec!` in a `Box::new`, as shown
in Listing 17-16.

Filename: src/main.rs

```
let futures =
    vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

trpl::join_all(futures).await;
```

Listing 17-16: Trying to use `Box::new` to align the types of the futures in a
`Vec`

Unfortunately, this still doesn’t compile. In fact, we have the same basic
error we did before, but we get one for both the second and third `Box::new`
calls, and we also get new errors referring to the `Unpin` trait. We will come
back to the `Unpin` errors in a moment. First, let’s fix the type errors on the
`Box::new` calls, by explicitly annotating the type of the `futures` variable:

Filename: src/main.rs

```
let futures: Vec<Box<dyn Future<Output = ()>>> =
    vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
```

Listing 17-17: Fixing the rest of the type mismatch errors by using an explicit
type declaration

The type we had to write here is a little involved, so let’s walk through it:

* The innermost type is the future itself. We note explicitly that the output of
  the future is the unit type `()` by writing `Future<Output = ()>`.
* Then we annotate the trait with `dyn` to mark it as dynamic.
* The entire trait reference is wrapped in a `Box`.
* Finally, we state explicitly that `futures` is a `Vec` containing these items.

That already made a big difference. Now when we run the compiler, we only have
the errors mentioning `Unpin`. Although there are three of them, notice that
each is very similar in its contents.

```
error[E0277]: `{async block@src/main.rs:8:23: 20:10}` cannot be unpinned
   --> src/main.rs:46:24
    |
46  |         trpl::join_all(futures).await;
    |         -------------- ^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:8:23: 20:10}`, which is required by `Box<{async block@src/main.rs:8:23: 20:10}>: std::future::Future`
    |         |
    |         required by a bound introduced by this call
    |
    = note: consider using the `pin!` macro
            consider using `Box::pin` if you need to access the pinned value outside of the current scope
    = note: required for `Box<{async block@src/main.rs:8:23: 20:10}>` to implement `std::future::Future`
note: required by a bound in `join_all`
   --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:105:14
    |
102 | pub fn join_all<I>(iter: I) -> JoinAll<I::Item>
    |        -------- required by a bound in this function
...
105 |     I::Item: Future,
    |              ^^^^^^ required by this bound in `join_all`

error[E0277]: `{async block@src/main.rs:8:23: 20:10}` cannot be unpinned
  --> src/main.rs:46:9
   |
46 |         trpl::join_all(futures).await;
   |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:8:23: 20:10}`, which is required by `Box<{async block@src/main.rs:8:23: 20:10}>: std::future::Future`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<{async block@src/main.rs:8:23: 20:10}>` to implement `std::future::Future`
note: required by a bound in `JoinAll`
  --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

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
  --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

That is a *lot* to digest, so let’s pull it apart. The first part of the message
tell us that the first async block (`src/main.rs:8:23: 20:10`) does not
implement the `Unpin` trait, and suggests using `pin!` or `Box::pin` to resolve
it. Later in the chapter, we’ll dig into a few more details about `Pin` and
`Unpin`. For the moment, though, we can just follow the compiler’s advice to get
unstuck! In Listing 17-18, we start by updating the type annotation for
`futures`, with a `Pin` wrapping each `Box`. Second, we use `Box::pin` to pin
the futures themselves.

Filename: src/main.rs

```
let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
    vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
```

Listing 17-18: Using `Pin` and `Box::pin` to make the `Vec` type check

If we compile and run this, we finally get the output we hoped for:

```
received 'hi'
received 'more'
received 'from'
received 'messages'
received 'the'
received 'for'
received 'future'
received 'you'
```

Phew!

There’s a bit more we can explore here. For one thing, using `Pin<Box<T>>`
comes with a small amount of extra overhead from putting these futures on the
heap with `Box`—and we’re only doing that to get the types to line up. We don’t
actually *need* the heap allocation, after all: these futures are local to this
particular function. As noted above, `Pin` is itself a wrapper type, so we can
get the benefit of having a single type in the `Vec`—the original reason we
reached for `Box`—without doing a heap allocation. We can use `Pin` directly
with each future, using the `std::pin::pin` macro.

However, we must still be explicit about the type of the pinned reference;
otherwise Rust will still not know to interpret these as dynamic trait objects,
which is what we need them to be in the `Vec`. We therefore `pin!` each future
when we define it, and define `futures` as a `Vec` containing pinned mutable
references to the dynamic `Future` type, as in Listing 17-19.

Filename: src/main.rs

```
let tx1_fut = pin!(async move {
    // --snip--
});

let rx_fut = pin!(async {
    // --snip--
});

let tx_fut = pin!(async move {
    // --snip--
});

let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
    vec![tx1_fut, rx_fut, tx_fut];
```

Listing 17-19: Using `Pin` directly with the `pin!` macro to avoid unnecessary
heap allocations

We got this far by ignoring the fact that we might have different `Output`
types. For example, in Listing 17-20, the anonymous future for `a` implements
`Future<Output = u32>`, the anonymous future for `b` implements
`Future<Output = &str>`, and the anonymous future for `c` implements
`Future<Output = bool>`.

Filename: src/main.rs

```
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
```

Listing 17-20: Three futures with distinct types

We can use `trpl::join!` to await them, because it allows you to pass in
multiple future types and produces a tuple of those types. We *cannot* use
`trpl::join_all`, because it requires the futures passed in all to have the same
type. Remember, that error is what got us started on this adventure with `Pin`!

This is a fundamental tradeoff: we can either deal with a dynamic number of
futures with `join_all`, as long as they all have the same type, or we can deal
with a set number of futures with the `join` functions or the `join!` macro,
even if they have different types. This is the same as working with any other
types in Rust, though. Futures are not special, even though we have some nice
syntax for working with them, and that is a good thing.

### Racing futures

When we “join” futures with the `join` family of functions and macros, we
require *all* of them to finish before we move on. Sometimes, though, we only
need *some* future from a set to finish before we move on—kind of similar to
racing one future against another.

In Listing 17-21, we once again use `trpl::race` to run two futures, `slow` and
`fast`, against each other. Each one prints a message when it starts running,
pauses for some amount of time by calling and awaiting `sleep`, and then prints
another message when it finishes. Then we pass both to `trpl::race` and wait for
one of them to finish. (The outcome here won’t be too surprising: `fast` wins!)
Unlike when we used `race` back in the “Our First Async Program” section of this
chapter on page XX, we just ignore the `Either` instance it returns here,
because all of the interesting behavior happens in the body of the async blocks.

Filename: src/main.rs

```
let slow = async {
    println!("'slow' started.");
    trpl::sleep(Duration::from_millis(100)).await;
    println!("'slow' finished.");
};

let fast = async {
    println!("'fast' started.");
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'fast' finished.");
};

trpl::race(slow, fast).await;
```

Listing 17-21: Using `race` to get the result of whichever future finishes first

Notice that if you flip the order of the arguments to `race`, the order of the
“started” messages changes, even though the `fast` future always completes
first. That’s because the implementation of this particular `race` function is
not fair. It always runs the futures passed as arguments in the order they’re
passed. Other implementations *are* fair, and will randomly choose which future
to poll first. Regardless of whether the implementation of race we’re using is
fair, though, *one* of the futures will run up to the first `await` in its body
before another task can start.

Recall from the “Our First Async Program” section of this chapter on page XX
that at each await point, Rust gives a runtime a chance to pause the task and
switch to another one if the future being awaited isn’t ready. The inverse is
also true: Rust *only* pauses async blocks and hands control back to a runtime
at an await point. Everything between await points is synchronous.

That means if you do a bunch of work in an async block without an await point,
that future will block any other futures from making progress. You may sometimes
hear this referred to as one future *starving* other futures. In some cases,
that may not be a big deal. However, if you are doing some kind of expensive
setup or long-running work, or if you have a future which will keep doing some
particular task indefinitely, you’ll need to think about when and where to
hand control back to the runtime.

By the same token, if you have long-running blocking operations, async can be a
useful tool for providing ways for different parts of the program to relate to
each other.

But *how* would you hand control back to the runtime in those cases?

### Yielding

Let’s simulate a long-running operation. Listing 17-22 introduces a `slow`
function. It uses `std::thread::sleep` instead of `trpl::sleep` so that calling
`slow` will block the current thread for some number of milliseconds. We can use
`slow` to stand in for real-world operations which are both long-running and
blocking.

Filename: src/main.rs

```
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```

Listing 17-22: Using `thread::sleep` to simulate slow operations

In Listing 17-23, we use `slow` to emulate doing this kind of CPU-bound work in
a pair of futures. To begin, each future only hands control back to the runtime
*after* carrying out a bunch of slow operations.

Filename: src/main.rs

```
let a = async {
    println!("'a' started.");
    slow("a", 30);
    slow("a", 10);
    slow("a", 20);
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'a' finished.");
};

let b = async {
    println!("'b' started.");
    slow("b", 75);
    slow("b", 10);
    slow("b", 15);
    slow("b", 350);
    trpl::sleep(Duration::from_millis(50)).await;
    println!("'b' finished.");
};

trpl::race(a, b).await;
```

Listing 17-23: Using `thread::sleep` to simulate slow operations

If you run this, you will see this output:

```
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

As with our earlier example, `race` still finishes as soon as `a` is done.
There’s no interleaving between the two futures, though. The `a` future does all
of its work until the `trpl::sleep` call is awaited, then the `b` future does
all of its work until its own `trpl::sleep` call is awaited, and then the `a`
future completes. To allow both futures to make progress between their slow
tasks, we need await points so we can hand control back to the runtime. That
means we need something we can await!

We can already see this kind of handoff happening in Listing 17-23: if we
removed the `trpl::sleep` at the end of the `a` future, it would complete
without the `b` future running *at all*. Maybe we could use the `sleep` function
as a starting point?

Filename: src/main.rs

```
let one_ms = Duration::from_millis(1);

let a = async {
    println!("'a' started.");
    slow("a", 30);
    trpl::sleep(one_ms).await;
    slow("a", 10);
    trpl::sleep(one_ms).await;
    slow("a", 20);
    trpl::sleep(one_ms).await;
    println!("'a' finished.");
};

let b = async {
    println!("'b' started.");
    slow("b", 75);
    trpl::sleep(one_ms).await;
    slow("b", 10);
    trpl::sleep(one_ms).await;
    slow("b", 15);
    trpl::sleep(one_ms).await;
    slow("b", 35);
    trpl::sleep(one_ms).await;
    println!("'b' finished.");
};
```

Listing 17-24: Using `sleep` to let operations switch off making progress

In Listing 17-24, we add `trpl::sleep` calls with await points between each call
to `slow`. Now the two futures’ work is interleaved:

```
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

The `a` future still runs for a bit before handing off control to `b`, because
it calls `slow` before ever calling `trpl::sleep`, but after that the futures
swap back and forth each time one of them hits an await point. In this case, we
have done that after every call to `slow`, but we could break up the work
however makes the most sense to us.

We don’t really want to *sleep* here, though: we want to make progress as fast
as we can. We just need to hand back control to the runtime. We can do that
directly, using the `yield_now` function. In Listing 17-25, we replace all those
`sleep` calls with `yield_now`.

Filename: src/main.rs

```
let a = async {
    println!("'a' started.");
    slow("a", 30);
    trpl::yield_now().await;
    slow("a", 10);
    trpl::yield_now().await;
    slow("a", 20);
    trpl::yield_now().await;
    println!("'a' finished.");
};

let b = async {
    println!("'b' started.");
    slow("b", 75);
    trpl::yield_now().await;
    slow("b", 10);
    trpl::yield_now().await;
    slow("b", 15);
    trpl::yield_now().await;
    slow("b", 35);
    trpl::yield_now().await;
    println!("'b' finished.");
};
```

Listing 17-25: Using `yield_now` to let operations switch off making progress

This is both clearer about the actual intent and can be significantly faster
than using `sleep`, because timers such as the one used by `sleep` often have
limits to how granular they can be. The version of `sleep` we are using, for
example, will always sleep for at least a millisecond, even if we pass it a
`Duration` of one nanosecond. Again, modern computers are *fast*: they can do a
lot in one millisecond!

You can see this for yourself by setting up a little benchmark, such as the one
in Listing 17-26. (This isn’t an especially rigorous way to do performance
testing, but it suffices to show the difference here.) Here, we skip all the
status printing, pass a one-nanosecond `Duration` to `trpl::sleep`, and let
each future run by itself, with no switching between the futures. Then we run
for 1,000 iterations and see how long the future using `trpl::sleep` takes
compared to the future using `trpl::yield_now`.

Filename: src/main.rs

```
let one_ns = Duration::from_nanos(1);
let start = Instant::now();
async {
    for _ in 1..1000 {
        trpl::sleep(one_ns).await;
    }
}
.await;
let time = Instant::now() - start;
println!(
    "'sleep' version finished after {} seconds.",
    time.as_secs_f32()
);

let start = Instant::now();
async {
    for _ in 1..1000 {
        trpl::yield_now().await;
    }
}
.await;
let time = Instant::now() - start;
println!(
    "'yield' version finished after {} seconds.",
    time.as_secs_f32()
);
```

Listing 17-26: Comparing the performance of `sleep` and `yield_now`

The version with `yield_now` is *way* faster!

This means that async can be useful even for compute-bound tasks, depending on
what else your program is doing, because it provides a useful tool for
structuring the relationships between different parts of the program. This is a
form of *cooperative multitasking*, where each future has the power to determine
when it hands over control via await points. Each future therefore also has the
responsibility to avoid blocking for too long. In some Rust-based embedded
operating systems, this is the *only* kind of multitasking!

In real-world code, you won’t usually be alternating function calls with await
points on every single line, of course. While yielding control in this way is
relatively inexpensive, it’s not free! In many cases, trying to break up a
compute-bound task might make it significantly slower, so sometimes it’s better
for *overall* performance to let an operation block briefly. You should always
measure to see what your code’s actual performance bottlenecks are. The
underlying dynamic is an important one to keep in mind if you *are* seeing a
lot of work happening in serial that you expected to happen concurrently,
though!

### Building Our Own Async Abstractions

We can also compose futures together to create new patterns. For example, we can
build a `timeout` function with async building blocks we already have. When
we’re done, the result will be another building block we could use to build up
yet further async abstractions.

Listing 17-27 shows how we would expect this `timeout` to work with a slow
future.

Filename: src/main.rs

```
let slow = async {
    trpl::sleep(Duration::from_millis(100)).await;
    "I finished!"
};

match timeout(slow, Duration::from_millis(10)).await {
    Ok(message) => println!("Succeeded with '{message}'"),
    Err(duration) => {
        println!("Failed after {} seconds", duration.as_secs())
    }
}
```

Listing 17-27: Using our imagined `timeout` to run a slow operation with a time
limit

Let’s implement this! To begin, let’s think about the API for `timeout`:

* It needs to be an async function itself so we can await it.
* Its first parameter should be a future to run. We can make it generic to allow
  it to work with any future.
* Its second parameter will be the maximum time to wait. If we use a `Duration`,
  that will make it easy to pass along to `trpl::sleep`.
* It should return a `Result`. If the future completes successfully, the
  `Result` will be `Ok` with the value produced by the future. If the timeout
  elapses first, the `Result` will be `Err` with the duration that the timeout
  waited for.

Listing 17-28 shows this declaration.

Filename: src/main.rs

```
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go!
}
```

Listing 17-28: Defining the signature of `timeout`

That satisfies our goals for the types. Now let’s think about the *behavior* we
need: we want to race the future passed in against the duration. We can use
`trpl::sleep` to make a timer future from the duration, and use `trpl::race` to
run that timer with the future the caller passes in.

We also know that `race` is not fair, and polls arguments in the order they are
passed. Thus, we pass `future_to_try` to `race` first so it gets a chance to
complete even if `max_time` is a very short duration. If `future_to_try`
finishes first, `race` will return `Left` with the output from `future`. If
`timer` finishes first, `race` will return `Right` with the timer’s output of
`()`.

In Listing 17-29, we match on the result of awaiting `trpl::race`. If the
`future_to_try` succeeded and we get a `Left(output)`, we return `Ok(output)`.
If the sleep timer elapsed instead and we get a `Right(())`, we ignore the `()`
with `_` and return `Err(max_time)` instead.

Filename: src/main.rs

```
use trpl::Either;

// --snip--

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
```

Listing 17-29: Defining `timeout` with `race` and `sleep`

With that, we have a working `timeout`, built out of two other async helpers. If
we run our code, it will print the failure mode after the timeout:

```
Failed after 2 seconds
```

Because futures compose with other futures, you can build really powerful tools
using smaller async building blocks. For example, you can use this same
approach to combine timeouts with retries, and in turn use those with things
such as network calls—one of the examples from the beginning of the chapter!

In practice, you will usually work directly with `async` and `await`, and
secondarily with functions and macros such as `join`, `join_all`, `race`, and
so on. You’ll only need to reach for `pin` now and again to use them with those
APIs.

We’ve now seen a number of ways to work with multiple futures at the same
time. Up next, we’ll look at how we can work with multiple futures in a
sequence over time, with *streams*. Here are a couple more things you might want
to consider first, though:

* We used a `Vec` with `join_all` to wait for all of the futures in some group
  to finish. How could you use a `Vec` to process a group of futures in
  sequence, instead? What are the tradeoffs of doing that?

* Take a look at the `futures::stream::FuturesUnordered` type from the `futures`
  crate. How would using it be different from using a `Vec`? (Don’t worry about
  the fact that it is from the `stream` part of the crate; it works just fine
  with any collection of futures.)

## Streams

So far in this chapter, we have mostly stuck to individual futures. The one big
exception was the async channel we used. Recall how we used the receiver for
our async channel in the “Message Passing” section of this chapter on page XX.
The async `recv` method produces a sequence of items over time. This is an
instance of a much more general pattern, often called a *stream*.

A sequence of items is something we’ve seen before, when we looked at the
`Iterator` trait in “The `Iterator` Trait and the `next` Method” section of
Chapter 13 on page XX, but there are two differences between iterators and the
async channel receiver. The first difference is the element of time: iterators
are synchronous, while the channel receiver is asynchronous. The second
difference is the API. When working directly with an `Iterator`, we call its
synchronous `next` method. With the `trpl::Receiver` stream in particular, we
called an asynchronous `recv` method instead, but these APIs otherwise feel
very similar.

That similarity isn’t a coincidence. A stream is similar to an asynchronous
form of iteration. Whereas the `trpl::Receiver` specifically waits to receive
messages, though, the general-purpose stream API is much more general: it
provides the next item the way `Iterator` does, but asynchronously. The
similarity between iterators and streams in Rust means we can actually create a
stream from any iterator. As with an iterator, we can work with a stream by
calling its `next` method and then awaiting the output, as in Listing 17-30.

Filename: src/main.rs

```
let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let iter = values.iter().map(|n| n * 2);
let mut stream = trpl::stream_from_iter(iter);

while let Some(value) = stream.next().await {
    println!("The value was: {value}");
}
```

Listing 17-30: Creating a stream from an iterator and printing its values

We start with an array of numbers, which we convert to an iterator and then call
`map` on to double all the values. Then we convert the iterator into a stream
using the `trpl::stream_from_iter` function. Then we loop over the items in the
stream as they arrive with the `while let` loop.

Unfortunately, when we try to run the code, it doesn’t compile. Instead, as we
can see in the output, it reports that there is no `next` method available.

```
error[E0599]: no method named `next` found for struct `Iter` in the current scope
 --> src/main.rs:8:40
  |
8 |         while let Some(value) = stream.next().await {
  |                                        ^^^^
  |
  = note: the full type name has been written to '~/projects/hello-async/target/debug/deps/async_await-bbd5bb8f6851cb5f.long-type-18426562901668632191.txt'
  = note: consider using `--verbose` to print the full type name to the console
  = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
  |
1 + use futures_util::stream::stream::StreamExt;
  |
1 + use std::iter::Iterator;
  |
1 + use std::str::pattern::Searcher;
  |
1 + use trpl::StreamExt;
  |
help: there is a method `try_next` with a similar name
  |
8 |         while let Some(value) = stream.try_next().await {
  |                                        ~~~~~~~~

For more information about this error, try `rustc --explain E0599`.
```

As the output suggests, the reason for the compiler error is that we need the
right trait in scope to be able to use the `next` method. Given our discussion
so far, you might reasonably expect that to be `Stream`, but the trait we need
here is actually `StreamExt`. The `Ext` there is for “extension”: this is a
common pattern in the Rust community for extending one trait with another.

Why do we need `StreamExt` instead of `Stream`, and what does the `Stream` trait
itself do? Briefly, the answer is that throughout the Rust ecosystem, the
`Stream` trait defines a low-level interface which effectively combines the
`Iterator` and `Future` traits. The `StreamExt` trait supplies a higher-level
set of APIs on top of `Stream`, including the `next` method as well as other
utility methods similar to those provided by the `Iterator` trait. We’ll return
to the `Stream` and `StreamExt` traits in a bit more detail at the end of the
chapter. For now, this is enough to let us keep moving.

The fix to the compiler error is to add a `use` statement for `trpl::StreamExt`,
as in Listing 17-31.

Filename: src/main.rs

```
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

Listing 17-31: Successfully using an iterator as the basis for a stream

With all those pieces put together, this code works the way we want! What’s
more, now that we have `StreamExt` in scope, we can use all of its utility
methods, just as with iterators. For example, in Listing 17-32, we use the
`filter` method to filter out everything but multiples of three and five.

Filename: src/main.rs

```
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}
```

Listing 17-32: Filtering a `Stream` with the `StreamExt::filter` method

Of course, this isn’t very interesting. We could do that with normal iterators
and without any async at all. So let’s look at some of the other things we can
do which are unique to streams.

### Composing Streams

Many concepts are naturally represented as streams: items becoming available in
a queue, or working with more data than can fit in a computer’s memory by only
pulling chunks of it from the file system at a time, or data arriving over the
network over time. Because streams are futures, we can use them with any other
kind of future, too, and we can combine them in interesting ways. For example,
we can batch up events to avoid triggering too many network calls, set timeouts
on sequences of long-running operations, or throttle user interface events to
avoid doing needless work.

Let’s start by building a little stream of messages, as a stand-in for a stream
of data we might see from a WebSocket or another real-time communication
protocol. In Listing 17-33, we create a function `get_messages` which returns
`impl Stream<Item = String>`. For its implementation, we create an async
channel, loop over the first ten letters of the English alphabet, and send them
across the channel.

We also use a new type: `ReceiverStream`, which converts the `rx` receiver from
the `trpl::channel` into a `Stream` with a `next` method. Back in `main`, we use
a `while let` loop to print all the messages from the stream.

Filename: src/main.rs

```
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}
```

Listing 17-33: Using the `rx` receiver as a `ReceiverStream`

When we run this code, we get exactly the results we would expect:

```
Message: 'a'
Message: 'b'
Message: 'c'
Message: 'd'
Message: 'e'
Message: 'f'
Message: 'g'
Message: 'h'
Message: 'i'
Message: 'j'
```

We could do this with the regular `Receiver` API, or even the regular `Iterator`
API, though. Let’s add something that requires streams: adding a timeout
which applies to every item in the stream, and a delay on the items we emit.

In Listing 17-34, we start by adding a timeout to the stream with the `timeout`
method, which comes from the `StreamExt` trait. Then we update the body of the
`while let` loop, because the stream now returns a `Result`. The `Ok` variant
indicates a message arrived in time; the `Err` variant indicates that the
timeout elapsed before any message arrived. We `match` on that result and either
print the message when we receive it successfully, or print a notice about the
timeout. Finally, notice that we pin the messages after applying the timeout to
them, because the timeout helper produces a stream which needs to be pinned to
be polled.

Filename: src/main.rs

```
use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages =
            pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}
```

Listing 17-34: Using the `StreamExt::timeout` method to set a time limit on the
items in a stream

However, because there are no delays between messages, this timeout does not
change the behavior of the program. Let’s add a variable delay to the messages
we send. In `get_messages`, we use the `enumerate` iterator method with the
`messages` array so that we can get the index of each item we are sending along
with the item itself. Then we apply a 100 millisecond delay to even-index items
and a 300 millisecond delay to odd-index items, to simulate the different delays
we might see from a stream of messages in the real world. Because our timeout is
for 200 milliseconds, this should affect half of the messages.

Filename: src/main.rs

```
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-35: Sending messages through `tx` with an async delay without making
`get_messages` an async function

To sleep between messages in the `get_messages` function without blocking, we
need to use async. However, we can’t make `get_messages` itself into an async
function, because then we’d return a `Future<Output = Stream<Item = String>>`
instead of a `Stream<Item = String>>`. The caller would have to await
`get_messages` itself to get access to the stream. But remember: everything in a
given future happens linearly; concurrency happens *between* futures. Awaiting
`get_messages` would require it to send all the messages, including sleeping
between sending each message, before returning the receiver stream. As a result,
the timeout would end up useless. There would be no delays in the stream itself:
the delays would all happen before the stream was even available.

Instead, we leave `get_messages` as a regular function which returns a stream,
and spawn a task to handle the async `sleep` calls.

> Note: calling `spawn_task` in this way works because we already set up our
> runtime. Calling this particular implementation of `spawn_task` *without*
> first setting up a runtime will cause a panic. Other implementations choose
> different tradeoffs: they might spawn a new runtime and so avoid the panic but
> end up with a bit of extra overhead, or simply not provide a standalone way to
> spawn tasks without reference to a runtime. You should make sure you know what
> tradeoff your runtime has chosen and write your code accordingly!

Now our code has a much more interesting result! Between every other pair of
messages, we see an error reported: `Problem: Elapsed(())`.

```
Message: 'a'
Problem: Elapsed(())
Message: 'b'
Message: 'c'
Problem: Elapsed(())
Message: 'd'
Message: 'e'
Problem: Elapsed(())
Message: 'f'
Message: 'g'
Problem: Elapsed(())
Message: 'h'
Message: 'i'
Problem: Elapsed(())
Message: 'j'
```

The timeout doesn’t prevent the messages from arriving in the end—we still get
all of the original messages. This is because our channel is unbounded: it can
hold as many messages as we can fit in memory. If the message doesn’t arrive
before the timeout, our stream handler will account for that, but when it polls
the stream again, the message may now have arrived.

You can get different behavior if needed by using other kinds of channels, or
other kinds of streams more generally. Let’s see one of those in practice in our
final example for this section, by combining a stream of time intervals with
this stream of messages.

### Merging Streams

First, let’s create another stream, which will emit an item every millisecond if
we let it run directly. For simplicity, we can use the `sleep` function to send
a message on a delay, and combine it with the same approach of creating a stream
from a channel we used in `get_messages`. The difference is that this time,
we’re going to send back the count of intervals which has elapsed, so the return
type will be `impl Stream<Item = u32>`, and we can call the function
`get_intervals`.

In Listing 17-36, we start by defining a `count` in the task. (We could define
it outside the task, too, but it is clearer to limit the scope of any given
variable.) Then we create an infinite loop. Each iteration of the loop
asynchronously sleeps for one millisecond, increments the count, and then sends
it over the channel. Because this is all wrapped in the task created by
`spawn_task`, all of it will get cleaned up along with the runtime, including
the infinite loop.

Filename: src/main.rs

```
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-36: Creating a stream with a counter that will be emitted once every
millisecond

This kind of infinite loop, which only ends when the whole runtime gets torn
down, is fairly common in async Rust: many programs need to keep running
indefinitely. With async, this doesn’t block anything else, as long as there is
at least one await point in each iteration through the loop.

Back in our main function’s async block, we start by calling `get_intervals`.
Then we merge the `messages` and `intervals` streams with the `merge` method,
which combines multiple streams into one stream that produces items from any of
the source streams as soon as the items are available, without imposing any
particular ordering. Finally, we loop over that combined stream instead of over
`messages` (Listing 17-37).

Filename: src/main.rs

```
let messages = get_messages().timeout(Duration::from_millis(200));
let intervals = get_intervals();
let merged = messages.merge(intervals);
```

Listing 17-37: Attempting to merge streams of messages and intervals

At this point, neither `messages` nor `intervals` needs to be pinned or
mutable, because both will be combined into the single `merged` stream.
However, this call to `merge` does not compile! (Neither does the `next` call
in the `while let` loop, but we’ll come back to that after fixing this.) The
two streams have different types. The `messages` stream has the type
`Timeout<impl Stream<Item = String>>`, where `Timeout` is the type which
implements `Stream` for a `timeout` call. Meanwhile, the `intervals` stream has
the type `impl Stream<Item = u32>`. To merge these two streams, we need to
transform one of them to match the other.

In Listing 17-38, we rework the `intervals` stream, because `messages` is
already in the basic format we want and has to handle timeout errors. First, we
can use the `map` helper method to transform the `intervals` into a string.
Second, we need to match the `Timeout` from `messages`. Because we don’t
actually *want* a timeout for `intervals`, though, we can just create a timeout
which is longer than the other durations we are using. Here, we create a
10-second timeout with `Duration::from_secs(10)`. Finally, we need to make
`stream` mutable, so that the `while let` loop’s `next` calls can iterate
through the stream, and pin it so that it’s safe to do so.

Filename: src/main.rs

```
let messages = get_messages().timeout(Duration::from_millis(200));
let intervals = get_intervals()
    .map(|count| format!("Interval: {count}"))
    .timeout(Duration::from_secs(10));
let merged = messages.merge(intervals);
let mut stream = pin!(merged);
```

Listing 17-38: Aligning the types of the the `intervals` stream with the type
of the `messages` stream

That gets us *almost* to where we need to be. Everything type checks. If you run
this, though, there will be two problems. First, it will never stop! You’ll
need to stop it with <span class="keystroke">ctrl-c</span>. Second, the
messages from the English alphabet will be buried in the midst of all the
interval counter messages:

```
--snip--
Interval: 38
Interval: 39
Interval: 40
Message: 'a'
Interval: 41
Interval: 42
Interval: 43
--snip--
```

Listing 17-39 shows one way to solve these last two problems. First, we use the
`throttle` method on the `intervals` stream, so that it doesn’t overwhelm the
`messages` stream. Throttling is a way of limiting the rate at which a function
will be called—or, in this case, how often the stream will be polled. Once every
hundred milliseconds should do, because that is in the same ballpark as how
often our messages arrive.

To limit the number of items we will accept from a stream, we can use the `take`
method. We apply it to the *merged* stream, because we want to limit the final
output, not just one stream or the other.

Filename: src/main.rs

```
let messages = get_messages().timeout(Duration::from_millis(200));
let intervals = get_intervals()
    .map(|count| format!("Interval: {count}"))
    .throttle(Duration::from_millis(100))
    .timeout(Duration::from_secs(10));
let merged = messages.merge(intervals).take(20);
let mut stream = pin!(merged);
```

Listing 17-39: Using `throttle` and `take` to manage the merged streams

Now when we run the program, it stops after pulling twenty items from the
stream, and the intervals don’t overwhelm the messages. We also don’t get
`Interval: 100` or `Interval: 200` or so on, but instead get `Interval: 1`,
`Interval: 2`, and so on—even though we have a source stream which *can*
produce an event every millisecond. That’s because the `throttle` call
produces a new stream, wrapping the original stream, so that the original
stream only gets polled at the throttle rate, not its own “native” rate. We
don’t have a bunch of unhandled interval messages we’re choosing to
ignore. Instead, we never produce those interval messages in the first place!
This is the inherent “laziness” of Rust’s futures at work again, allowing us to
choose our performance characteristics.

```
Interval: 1
Message: 'a'
Interval: 2
Interval: 3
Problem: Elapsed(())
Interval: 4
Message: 'b'
Interval: 5
Message: 'c'
Interval: 6
Interval: 7
Problem: Elapsed(())
Interval: 8
Message: 'd'
Interval: 9
Message: 'e'
Interval: 10
Interval: 11
Problem: Elapsed(())
Interval: 12
```

There’s one last thing we need to handle: errors! With both of these
channel-based streams, the `send` calls could fail when the other side of the
channel closes—and that’s just a matter of how the runtime executes the futures
which make up the stream. Up until now we have ignored this by calling `unwrap`,
but in a well-behaved app, we should explicitly handle the error, at minimum by
ending the loop so we don’t try to send any more messages!  Listing 17-40 shows
a simple error strategy: print the issue and then `break` from the loops. As
usual, the correct way to handle a message send error will vary—just make sure
you have a strategy.

```
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-40: Handling errors and shutting down the loops

Now that we’ve seen a bunch of async in practice, let’s take a step back and
dig into a few of the details of how `Future`, `Stream`, and the other key
traits which Rust uses to make async work.

## Digging Into the Traits for Async

Throughout the chapter, we’ve used the `Future`, `Pin`, `Unpin`, `Stream`, and
`StreamExt` traits in various ways. So far, though, we’ve avoided digging too
far into the details of how they work or how they fit together. Much of the time
when writing Rust day to day, this is fine. Sometimes, though, you’ll hit
situations where understanding a few more of these details matters. In this
section, we’ll dig down *enough* further to help with those situations—while
still leaving the *really* deep dive for other documentation!

### Future

Back in the “Futures and the Async Syntax” section of this chapter on page XX,
we noted that `Future` is a trait. Let’s start by taking a closer look at how
it works. Here is how Rust defines a `Future`:

```
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

That trait definition includes a bunch of new types and also some syntax we
haven’t seen before, so let’s walk through the definition piece by piece.

First, `Future`’s associated type `Output` says what the future resolves to.
This is analogous to the `Item` associated type for the `Iterator` trait.
Second, `Future` also has the `poll` method, which takes a special `Pin`
reference for its `self` parameter and a mutable reference to a `Context` type,
and returns a `Poll<Self::Output>`. We’ll talk a little more about `Pin` and
`Context` later in the section. For now, let’s focus on what the method returns,
the `Poll` type:

```
enum Poll<T> {
    Ready(T),
    Pending,
}
```

This `Poll` type is similar to an `Option`: it has one variant which has a value
(`Ready(T)`), and one which does not (`Pending`). It means something quite
different, though! The `Pending` variant indicates that the future still has
work to do, so the caller will need to check again later. The `Ready` variant
indicates that the `Future` has finished its work and the `T` value is
available.

> Note: With most futures, the caller should not call `poll` again after the
> future has returned `Ready`. Many futures will panic if polled again after
> becoming ready! Futures which are safe to poll again will say so explicitly in
> their documentation. This is similar to how `Iterator::next` behaves!

Under the hood, when you see code which uses `await`, Rust compiles that to code
which calls `poll`. If you look back at Listing 17-4, where we printed out the
page title for a single URL once it resolved, Rust compiles it into something
kind of (although not exactly) like this:

```
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what goes here?
    }
}
```

What should we do when the `Future` is still `Pending`? We need some way to try
again… and again, and again, until the future is finally ready. In other words,
a loop:

```
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(value) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // continue
        }
    }
}
```

If Rust compiled it to exactly that code, though, every `await` would be
blocking—exactly the opposite of what we were going for! Instead, Rust needs
makes sure that the loop can hand off control to something which can pause work
on this future and work on other futures and check this one again later. That
“something” is an async runtime, and this scheduling and coordination work is
one of the main jobs for a runtime.

Recall our description (in the “Counting” section of this chapter on page XX)
of waiting on `rx.recv`. The `recv` call returns a `Future`, and awaiting it
polls it. In our initial discussion, we noted that a runtime will pause the
future until it’s ready with either `Some(message)` or `None` when the channel
closes. With our deeper understanding of `Future` in place, and specifically
`Future::poll`, we can see how that works. The runtime knows the future isn’t
ready when it returns `Poll::Pending`. Conversely, the runtime knows the future
is ready and advances it when `poll` returns `Poll::Ready(Some(message))` or
`Poll::Ready(None)`.

The exact details of how a runtime does that are more than we will cover in even
this deep dive section. The key here is to see the basic mechanic of futures: a
runtime *polls* each future it is responsible for, putting it back to sleep when
it is not yet ready.

### Pinning and the Pin and Unpin Traits

When we introduced the idea of pinning while working on Listing 17-17, we ran
into a very gnarly error message. Here is the relevant part of it again:

```
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
  --> ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.30/src/future/join_all.rs:29:8
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
pin the values, but also tells us why pinning is required. The `trpl::join_all`
function returns a struct called `JoinAll`. That struct is generic over a type
`F`, which is constrained to implement the `Future` trait. Directly awaiting a
future with `await` pins the future implicitly. That’s why we don’t need to use
`pin!` everywhere we want to await futures.

However, we’re not directly awaiting a future here. Instead, we construct a new
future, `JoinAll`, by passing a collection of futures to the `join_all`
function. The signature for `join_all` produces requires that the type of the
items in the collection all implement the `Future` trait, and `Box<T>` only
implements `Future` if the `T` that it wraps is a future which implements the
`Unpin` trait.

That’s a lot! But we can understand it, if we dive a little further into how the
`Future` type actually works, in particular around *pinning*.

Let’s look again at the definition of `Future`:

```
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

The `cx` parameter and its `Context` type is the key to how a runtime actually
knows when to check any given future, while still being lazy. The details of how
that works are beyond the scope of this chapter, though: you generally only need
to worry about it when writing a custom `Future` implementation.

Instead, we’ll focus on the type for `self`. This is the first time we’ve seen
a method where `self` has a type annotation. A type annotation for `self` is
similar to type annotations for other function parameters, with two key
differences. First, when we specify the type of `self` in this way, we’re
telling Rust what type `self` must be to call this method. Second, a type
annotation on `self` can’t be just any type. It’s only allowed to be the type
on which the method is implemented, a reference or smart pointer to that type,
or a `Pin` wrapping a reference to that type. We’ll see more on this syntax in
Chapter 18. For now, it’s enough to know that if we want to poll a future (to
check whether it is `Pending` or `Ready(Output)`), we need a mutable reference
to the type, which is wrapped in a `Pin`.

`Pin` is a wrapper type. In some ways, it’s similar to the `Box`, `Rc`, and
other smart pointer types we saw in Chapter 15, which also wrap other types.
Unlike those, however, `Pin` only works with *pointer types* such as references
(`&` and `&mut`) and smart pointers (`Box`, `Rc`, and so on). To be precise,
`Pin` works with types which implement the `Deref` or `DerefMut` traits, which
we covered in the “Treating Smart Pointers Like Regular References with the
Deref Trait” section of Chapter 15 on page XX. You can think of this restriction
as equivalent to only working with pointers, though, because implementing
`Deref` or `DerefMut` means your type behaves similarly to a pointer type. `Pin`
is also not a pointer itself, and it doesn’t have any behavior of its own the
way `Rc` and `Arc` do with ref counting. It’s purely a tool the compiler can use
to uphold the relevant guarantees, by wrapping pointers in the type.

Recalling that `await` is implemented in terms of calls to `poll`, this starts
to explain the error message we saw above—but that was in terms of `Unpin`, not
`Pin`. So what exactly are `Pin` and `Unpin`, how do they relate, and why does
`Future` need `self` to be in a `Pin` type to call `poll`?

In the “”ur First Async Program” section of this chapter on page XX, we
described how a series of await points in a future get compiled into a state
machine—and noted how the compiler helps make sure that state machine follows
all of Rust’s normal rules around safety, including borrowing and ownership. To
make that work, Rust looks at what data is needed between each await point and
the next await point or the end of the async block. It then creates a
corresponding variant in the state machine it creates. Each variant gets the
access it needs to the data that will be used in that section of the source
code, whether by taking ownership of that data or by getting a mutable or
immutable reference to it.

So far so good: if we get anything wrong about the ownership or references in a
given async block, the borrow checker will tell us. When we want to move around
the future that corresponds to that block—like moving it into a `Vec` to pass to
`join_all`, the way we did back in—things get trickier.

When we move a future—whether by pushing into a data structure to use as an
iterator with `join_all`, or returning them from a function—that actually means
moving the state machine Rust creates for us. And unlike most other types in
Rust, the futures Rust creates for async blocks can end up with references to
themselves in the fields of any given variant, as in Figure 17-4 (a simplified
illustration to help you get a feel for the idea, rather than digging into what
are often fairly complicated details).

<img alt="Concurrent work flow" src="img/trpl17-04.svg" />

Figure 17-4: A self-referential data type.

By default, though, any object which has a reference to itself is unsafe to
move, because references always point to the actual memory address of the thing
they refer to. If you move the data structure itself, those internal references
will be left pointing to the old location. However, that memory location is now
invalid. For one thing, its value will not be updated when you make changes to
the data structure. For another—and more importantly!—the computer is now free
to reuse that memory for other things! You could end up reading completely
unrelated data later.

<img alt="Concurrent work flow" src="img/trpl17-05.svg" />

Figure 17-5: The unsafe result of moving a self-referential data type.

In principle, the Rust compiler could try to update every reference to an object
every time it gets moved. That would potentially be a lot of performance
overhead, especially given there can be a whole web of references that need
updating. On the other hand, if we could make sure the data structure in
question *doesn’t move in memory*, we don’t have to update any references.
This is exactly what Rust’s borrow checker requires: you can’t move an item
which has any active references to it using safe code.

`Pin` builds on that to give us the exact guarantee we need. When we *pin* a
value by wrapping a pointer to that value in `Pin`, it can no longer move. Thus,
if you have `Pin<Box<SomeType>>`, you actually pin the `SomeType` value, *not*
the `Box` pointer. Figure 17-6 illustrates this:

<img alt="Concurrent work flow" src="img/trpl17-06.svg" />

Figure 17-6: Pinning a `Box` which points to a self-referential future type.

In fact, the `Box` pointer can still move around freely. Remember: we care about
making sure the data ultimately being referenced stays in its place. If a
pointer moves around, but the data it points to is in the same place, as in
Figure 17-7, there’s no potential problem. (How you would do this with a `Pin`
wrapping a `Box` is more than we’ll get into in this particular discussion,
but it would make for a good exercise! If you look at the docs for the types as
well as the `std::pin` module, you might be able to work out how you would do
that.) The key is that the self-referential type itself cannot move, because it
is still pinned.

<img alt="Concurrent work flow" src="img/trpl17-07.svg" />

Figure 17-7: Moving a `Box` which points to a self-referential future type.

However, most types are perfectly safe to move around, even if they happen to
be behind a `Pin` pointer. We only need to think about pinning when items have
internal references. Primitive values such as numbers and booleans don’t have
any internal references, so they’re obviously safe. Neither do most types you
normally work with in Rust. A `Vec`, for example, doesn’t have any internal
references it needs to keep up to date this way, so you can move it around
without worrying. If you have a `Pin<Vec<String>>`, you’d have to do everything
via the safe but restrictive APIs provided by `Pin`, even though a
`Vec<String>` is always safe to move if there are no other references to it. We
need a way to tell the compiler that it’s actually just fine to move items
around in cases such as these. For that, we have `Unpin`.

`Unpin` is a marker trait, similar to the `Send` and `Sync` traits we saw in the
“Extensible Concurrency with the `Sync` and `Send` Traits” section of Chapter
16 on page XX. Recall that marker traits have no functionality of their own.
They exist only to tell the compiler that it’s safe to use the type which
implements a given trait in a particular context. `Unpin` informs the compiler
that a given type does *not* need to uphold any particular guarantees about
whether the value in question can be moved.

Just as with `Send` and `Sync`, the compiler implements `Unpin` automatically
for all types where it can prove it is safe. The special case, again similar to
`Send` and `Sync`, is the case where `Unpin` is *not* implemented for a type.
The notation for this is `impl !Unpin for SomeType`, where `SomeType` is the
name of a type which *does* need to uphold those guarantees to be safe whenever
a pointer to that type it is used in a `Pin`.

In other words, there are two things to keep in mind about the relationship
between `Pin` and `Unpin`. First, `Unpin` is the “normal” case, and `!Unpin` is
the special case. Second, whether a type implements `Unpin` or `!Unpin` *only*
matters when using a pinned pointer to that type like `Pin<&mut SomeType>`.

To make that concrete, think about a `String`: it has a length and the Unicode
characters which make it up. We can wrap a `String` in `Pin`, as seen in Figure
17-8. However, `String` automatically implements `Unpin`, the same as most other
types in Rust.

<img alt="Concurrent work flow" src="img/trpl17-08.svg" />

Figure 17-8: Pinning a String, with a dotted line indicating that the String
implements the `Unpin` trait, so it is not pinned.

As a result, we can do things which would be illegal if `String` implemented
`!Unpin` instead, such as replace one string with another at the exact same
location in memory as in Figure 17-9. This doesn’t violate the `Pin` contract,
because `String` has no internal references that make it unsafe to move around!
That is precisely why it implements `Unpin` rather than `!Unpin`.

<img alt="Concurrent work flow" src="img/trpl17-09.svg" />

Figure 17-9: Replacing the String with an entirely different String in memory.

Now we know enough to understand the errors reported for that `join_all` call
from back in Listing 17-17. We originally tried to move the futures produced by
async blocks into a `Vec<Box<dyn Future<Output = ()>>>`, but as we’ve seen,
those futures may have internal references, so they don’t automatically
implement `Unpin`. Once we pin them, we can pass the resulting `Pin` type into
the `Vec`, confident that the underlying data in the futures will *not* be
moved.

`Pin` and `Unpin` are mostly important for building lower-level libraries, or
when you’re building a runtime itself, rather than for day to day Rust code.
When you see these traits in error messages, though, now you’ll have a better
idea of how to fix the code!

> Note: This combination of `Pin` and `Unpin` allows a whole class of complex
> types to be safe in Rust which are otherwise difficult to implement because
> they’re self-referential. Types which require `Pin` show up *most* commonly
> in async Rust today, but you might—very rarely!—see it in other contexts, too.
>
> The specifics of how `Pin` and `Unpin` work, and the rules they’re required
> to uphold, are covered extensively in the API documentation for `std::pin`, so
> if you’d like to understand them more deeply, that’s a great place to start.
>
> If you want to understand how things work “under the hood” in even more
> detail, the official *Asynchronous Programming in Rust* book available at
> *https://rust-lang.github.io/async-book/* has you covered:
>
> * Chapter 2: Under the Hood: Executing Futures and Tasks
> * Chapter 4: Pinning

### The Stream Trait

Now that we have a deeper grasp on the `Future`, `Pin`, and `Unpin` traits, we
can turn our attention to the `Stream` trait. As described in the section
introducing streams, streams are similar to asynchronous iterators. Unlike
`Iterator` and `Future`, there is no definition of a `Stream` trait in the
standard library as of the time of writing, but there *is* a very common
definition from the `futures` crate used throughout the ecosystem.

Let’s review the definitions of the `Iterator` and `Future` traits, so we can
build up to how a `Stream` trait that merges them together might look. From
`Iterator`, we have the idea of a sequence: its `next` method provides an
`Option<Self::Item>`. From `Future`, we have the idea of readiness over time:
its `poll` method provides a `Poll<Self::Output>`. To represent a sequence of
items which become ready over time, we define a `Stream` trait which puts those
features together:

```
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
produced by the stream. This is similar to `Iterator`: there may be zero to
many of these, and unlike `Future`, where there is always a single `Output`
(even if it’s the unit type `()`).

`Stream` also defines a method to get those items. We call it `poll_next`, to
make it clear that it polls in the same way `Future::poll` does and produces a
sequence of items in the same way `Iterator::next` does. Its return type
combines `Poll` with `Option`. The outer type is `Poll`, because it has to be
checked for readiness, just as a future does. The inner type is `Option`,
because it needs to signal whether there are more messages, just as an iterator
does.

Something very similar to this will likely end up standardized as part of Rust’s
standard library. In the meantime, it’s part of the toolkit of most runtimes,
so you can rely on it, and everything we cover below should generally apply!

In the example we saw in the section on streaming, though, we didn’t use
`poll_next` *or* `Stream`, but instead used `next` and `StreamExt`. We *could*
work directly in terms of the `poll_next` API by hand-writing our own `Stream`
state machines, of course, just as we *could* work with futures directly via
their `poll` method. Using `await` is much nicer, though, so the `StreamExt`
trait supplies the `next` method so we can do just that.

```
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}
```

> Note: The actual definition we used earlier in the chapter looks slightly
> different than this, because it supports versions of Rust which did not yet
> support using async functions in traits. As a result, it looks like this:
>
> ```
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> That `Next` type is a `struct` which implements `Future` and gives a way to
> name the lifetime of the reference to `self` with `Next<'_, Self>`, so that
> `await` can work with this method!

The `StreamExt` trait is also the home of all the interesting methods available
to use with streams. `StreamExt` is automatically implemented for every type
which implements `Stream`, but these traits are defined separately so that the
community can iterate on the foundational trait distinctly from the convenience
APIs.

In the version of `StreamExt` used in the `trpl` crate, the trait not only
defines the `next` method, it also supplies an implementation of `next`, which
correctly handles the details of calling `Stream::poll_next`. This means that
even when you need to write your own streaming data type, you *only* have to
implement `Stream`, and then anyone who uses your data type can use `StreamExt`
and its methods with it automatically.

That’s all we’re going to cover for the lower-level details on these traits. To
wrap up, let’s consider how futures (including streams), tasks, and threads all
fit together!

## Futures, Tasks, and Threads

As we saw in the “Using Threads to Run Code Simultaneously” section of Chapter
16 on page XX, threads provide one approach to concurrency. We’ve seen another
approach to concurrency in this chapter, using async with futures and streams.
You might be wondering why you would choose one or the other. The answer is: it
depends! And in many cases, the choice isn’t threads *or* async but rather
threads *and* async.

Many operating systems have supplied threading-based concurrency models for
decades now, and many programming languages have support for them as a result.
However, they are not without their tradeoffs. On many operating systems, they
use a fair bit of memory for each thread, and they come with some overhead for
starting up and shutting down. Threads are also only an option when your
operating system and hardware support them! Unlike mainstream desktop and mobile
computers, some embedded systems don’t have an OS at all, so they also don’t
have threads!

The async model provides a different—and ultimately complementary—set of
tradeoffs. In the async model, concurrent operations don’t require their own
threads. Instead, they can run on tasks, as when we used `trpl::spawn_task` to
kick off work from a synchronous function throughout the streams section. A task
is similar to a thread, but instead of being managed by the operating system,
it’s managed by library-level code: the runtime.

In the previous section, we saw that we could build a `Stream` by using an async
channel and spawning an async task which we could call from synchronous code. We
could do the exact same thing with a thread! In Listing 17-40, we used
`trpl::spawn_task` and `trpl::sleep`. In Listing 17-41, we replace those with
the `thread::spawn` and `thread::sleep` APIs from the standard library in the
`get_intervals` function.

Filename: src/main.rs

```
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    // This is *not* `trpl::spawn` but `std::thread::spawn`!
    thread::spawn(move || {
        let mut count = 0;
        loop {
            // Likewise, this is *not* `trpl::sleep` but `std::thread::sleep`!
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
```

Listing 17-41: Using the `std::thread` APIs instead of the async `trpl` APIs
for the `get_intervals` function

If you run this, the output is identical. And notice how little changes here
from the perspective of the calling code! What’s more, even though one of our
functions spawned an async task on the runtime and the other spawned an
OS thread, the resulting streams were unaffected by the differences.

Despite the similarities, these two approaches behave very differently, although
we might have a hard time measuring it in this very simple example. We could
spawn millions of async tasks on any modern personal computer. If we tried to do
that with threads, we would literally run out of memory!

However, there’s a reason these APIs are so similar. Threads act as a boundary
for sets of synchronous operations; concurrency is possible *between* threads.
Tasks act as a boundary for sets of *asynchronous* operations; concurrency is
possible both *between* and *within* tasks, because a task can switch between
futures in its body. Finally, futures are Rust’s most granular unit of
concurrency, and each future may represent a tree of other futures. The
runtime—specifically, its executor—manages tasks, and tasks manage futures. In
that regard, tasks are similar to lightweight, runtime-managed threads with
added capabilities that come from being managed by a runtime instead of by the
operating system.

This doesn’t mean that async tasks are always better than threads, any more than
that threads are always better than tasks.

Concurrency with threads is in some ways a simpler programming model than
concurrency with `async`. That can be a strength or a weakness. Threads are
somewhat “fire and forget,” they have no native equivalent to a future, so they
simply run to completion, without interruption except by the operating system
itself. That is, they have no built-in support for *intra-task concurrency* the
way futures do. Threads in Rust also have no mechanisms for cancellation—a
subject we haven’t covered in depth in this chapter, but which is implicit in
the fact that whenever we ended a future, its state got cleaned up correctly.

These limitations also make threads harder to compose than futures. It’s much
more difficult, for example, to use threads to build helpers such as the
`timeout` we built in the “Building Our Own Async Abstractions” section of this
chapter on page XX or the `throttle` method we used with streams in the
“Composing Streams” section of this chapter on page XX. The fact that futures
are richer data structures means they can be composed together more naturally,
as we have seen.

Tasks then give *additional* control over futures, allowing you to choose where
and how to group the futures. And it turns out that threads and tasks often
work very well together, because tasks can (at least in some runtimes) be moved
around between threads. We haven’t mentioned it up until now, but under the
hood the `Runtime` we have been using, including the `spawn_blocking` and
`spawn_task` functions, is multithreaded by default! Many runtimes use an
approach called *work stealing* to transparently move tasks around between
threads based on the current utilization of the threads, with the aim of
improving the overall performance of the system. To build that actually requires
threads *and* tasks, and therefore futures.

As a default way of thinking about which to use when:

* If the work is *very parallelizable*, such as processing a bunch of data where
  each part can be processed separately, threads are a better choice.
* If the work is *very concurrent*, such as handling messages from a bunch of
  different sources which may come in a different intervals or different rates,
  async is a better choice.

And if you need some mix of parallelism and concurrency, you don’t have to
choose between threads and async. You can use them together freely, letting each
one serve the part it is best at. For example, Listing 17-42 shows a fairly
common example of this kind of mix in real-world Rust code.

Filename: src/main.rs

```
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
```

Listing 17-42: Sending messages with blocking code in a thread and awaiting the
messages in an async block

We begin by creating an async channel. Then we spawn a thread which takes
ownership of the sender side of the channel. Within the thread, we send the
numbers 1 through 10, and sleep for a second in between each. Finally, we run a
future created with an async block passed to `trpl::run` just as we have
throughout the chapter. In that future, we await those messages, just as in
the other message-passing examples we have seen.

To return to the examples we opened the chapter with: you could imagine running
a set of video encoding tasks using a dedicated thread, because video encoding
is compute bound, but notifying the UI that those operations are done with an
async channel. Examples of this kind of mix abound!

## Summary

This isn’t the last you’ll see of concurrency in this book: the project in
Chapter 21 will use the concepts in this chapter in a more realistic situation
than the smaller examples discussed here—and compare more directly what it looks
like to solve these kinds of problems with threading vs. with tasks and futures.

Whether with threads, with futures and tasks, or with the combination of them
all, Rust gives you the tools you need to write safe, fast, concurrent
code—whether for a high-throughput web server or an embedded operating system.

Next, we’ll talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger. In addition, we’ll discuss how Rust’s idioms
relate to those you might be familiar with from object-oriented programming.
