# An I/O Project

We've learned a lot over the last few chapters. Let's take that new knowledge
and apply it by building a project together. Along the way, we'll learn a bit
more about Rust's standard library.

So what should we build? One that uses Rust's strengths. A great use of Rust is
for command line tools: Rust's speed, safety, 'single binary' output, and
cross-platform support make it a good language choice for this kind of task. So
we'll make our own version of a classic command line tool: `grep`. `grep` is
short for "*g*lobally search a *r*egular *e*xpression and *p*rint." In the
simplest use case, it does this:

- Takes a filename and a string as arguments.
- Reads the file.
- Finds lines in the file that contain the string argument.
- Prints out those lines.

In addition, we'll add one extra feature: an environment variable that will
allow us to search for the string argument in a case-insensitive way.

There's another great reason to use `grep` as an example project: a very
fully-featured version of `grep` has already been created in Rust by a
community member, Andrew Gallant. It's called `ripgrep`, and it's very,
very fast. While our version of `grep` will be fairly simple, you'll have
some of the background knowledge to understand that project if you want to see
something more real-world.

This project will bring together a number of things we learned previously:

- Organize code (using what we learned in modules, Chapter 7)
- Use vectors and strings (collections, Chapter 8)
- Handle errors (Chapter 9)
- Use traits and lifetimes where appropriate (Chapter 10)
- Have tests (Chapter 11)

Additionally, we'll briefly introduce closures, iterators, and trait objects,
which Chapters XX, YY, and ZZ respectively are about to cover in detail.

Let's create a new project with, as always, `cargo new`:

```text
$ cargo new --bin greprs
     Created binary (application) `greprs` project
$ cd greprs
```

We're calling our version of `grep` 'greprs', so that we don't confuse any of
our users into thinking that it's the more fully-featured version of `grep`
they may already have installed on their system.

## Accepting Command Line Arguments

Our first task is to have `greprs` accept its two command line arguments. There
are some existing libraries on crates.io that can help us do this, but since
we're learning, we'll implement this ourselves.

We'll need to call a function provided in Rust's standard library:
`std::env::args`. This function returns an *iterator* of the command line
arguments that were given to our program. We haven't discussed iterators yet;
Chapter 16 will cover them fully. For our purposes, though, we don't need to
understand much about how they work in order to use them. We only need to
understand two things:

1. Iterators produce a series of values by repeatedly calling a `next`
   function.
2. We can call the `collect` function on an iterator to turn it into a vector
   containing all of the elements the iterator produces.

Let's give it a try as shown in Listing 12-1:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

<figcaption>

Listing 12-1: Collect the command line arguments into a vector and print them out

</figcaption>
</figure>

<!-- Will add wingdings in libreoffice /Carol -->

First, we have a `use` statement to bring the `std::env` module into scope.
When using a function that's nested in more than one level of module, like
`std::env::args` is, it's conventional to use `use` to bring the parent module
into scope, rather than the function itself. `env::args` is less ambiguous than
a lone `args`. Also, if we end up using more than one function in `std::env`,
we only need a single `use`.

On the first line of `main`, we call `env::args`, and immediately use `collect`
to create a vector out of it. We're also explicitly annotating the type of
`args` here: `collect` can be used to create many kinds of collections. Rust
won't be able to infer what kind of type we want, so the annotation is
required. We very rarely need to annotate types in Rust, but `collect` is one
function where you often need to.

Finally, we print out the vector with the debug formatter, `:?`. Let's try
running our code with no arguments, and then with two arguments:

```text
$ cargo run
["target/debug/greprs"]

$ cargo run needle haystack
...snip...
["target/debug/greprs", "needle", "haystack"]
```

You'll notice one interesting thing: the name of the binary is the first
argument. The reasons for this are out of the scope of this chapter, but it's
something we'll have to remember to account for.

Now that we have a way to access all of the arguments, let's find the ones we
care about and save them in variables as shown in Listing 12-2:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search = &args[1];
    let filename = &args[2];

    println!("Searching for {}", search);
    println!("In file {}", filename);
}
```

<figcaption>

Listing 12-2: Create variables to hold the search argument and filename argument

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Remember, the program's name is the first argument, so we don't need `args[0]`.
We've decided that the first argument will be the string we're searching for,
so we put a reference to the first argument in the variable `search`. The
second argument will be the filename, so we put a reference to the second
argument in the variable `filename`. Let's try running this program again:

```text
$ cargo run test sample.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe test sample.txt`
Searching for test
In file sample.txt
```

Great! There's one problem, though. Let's try giving it no arguments:

```text
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1', ../src/libcollections\vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Because our vector only has one element, the program's name, but we tried to
access the second element, our program panics with a message about the
out-of-bound access. While this error message is _accurate_, it's not
meaningful to users of our program at all. We could fix this problem right now,
but let's push forward: we'll improve this situation before we're finished.

## Reading a File

Now that we have some variables containing the information that we need, let's
try using them. The next step is to open the file that we want to search. To do
that, we need a file. Create one called `poem.txt` at the root level of your
project, and fill it up with some Emily Dickinson:

<span class="filename">Filename: poem.txt</span>

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<!-- Public domain Emily Dickinson poem. This will work best with something
short, but that has multiple lines and some repetition. We could search through
code; that gets a bit meta and possibly confusing... Changes to this are most
welcome. /Carol -->

With that in place, let's edit *src/main.rs* and add code to open the file as
shown in Listing 12-3:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search = &args[1];
    let filename = &args[2];

    println!("Searching for {}", search);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

<figcaption>

Listing 12-3: Read the contents of the file specified by the second argument

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've added a few things. First of all, we need some more `use` statements to
bring in the relevant parts of the standard library: we need `std::fs::File`
for dealing with files, and `std::io::prelude::*` contains various traits that
are useful when doing I/O, including file I/O. In the same way that Rust has a
general prelude that brings certain things into scope automatically, the
`std::io` module has its own prelude of common things you'll need when working
with I/O. Unlike the default prelude, we must explicitly `use` the prelude in
`std::io`.

In `main`, we've added three things: first, we get a handle to the file and
open it by using the `File::open` function and passing it the name of the file
specified in the second argument. Second, we create a mutable, empty `String`
in the variable `contents`, then call `read_to_string` on our file handle with
our `contents` string as the argument; `contents` is where `read_to_string`
will place the data it reads. Finally, we print out the entire file contents,
which is a way for us to be sure our program is working so far.

Let's try running this code, specifying any string for the first argument (since
we haven't implemented the searching part yet) and our *poem.txt* file as the
second argument:

```text
$ cargo run the poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Great! Our code is working. However, it's got a few flaws. Because our program
is still small, these flaws aren't a huge deal, but as our program grows, it
will be harder and harder to fix them in a clean way. Let's do the refactoring
now, instead of waiting. The refactoring will be much easier to do with only
this small amount of code.

So what are these problems? There are four. The first problem is where we open
the file: we've used `expect` to print out an error message if opening the file
fails, but the error message only says "file not found". There are a number of
ways that opening a file can fail, but we're always assuming that it's due to
the file being missing. For example, the file could exist, but we might not have
permission to open it: right now, we print an error message that says the wrong
thing!

Secondly, our use of `expect` over and over is similar to the earlier issue we
noted with the `panic!` on indexing if we don't pass any command line
arguments: while it _works_, it's a bit unprincipled, and we're doing it all
throughout our program. It would be nice to put our error handling in one spot.

The third problem is that our `main` function now does two things: it parses
arguments, and it opens up files. For such a small function, this isn't a huge
problem. However, as we keep growing our program inside of `main`, the number of
separate tasks in the `main` function will get larger and larger. As one
function gains many responsibilities, it gets harder to reason about, harder to
test, and harder to change without breaking one of its parts.

This also ties into our fourth problem: while `search` and `filename` are
configuration variables to our program, variables like `f` and `contents` are
used to perform our program's logic. The longer `main` gets, the more variables
we're going to bring into scope, and the more variables we have in scope, the
harder it is to keep track of which ones we need for which purpose.

These organizational problems are common to many similar kinds of projects, so
the Rust community has developed a pattern for organizing the separate
concerns. This pattern is useful for organizing any binary project you'll build
in Rust, so we can justify doing this refactoring a bit earlier, since we know
that our project fits the pattern. The pattern looks like this:

1. Split your program into both a *main.rs* and a *lib.rs*.
2. Place your command line parsing logic into *main.rs*.
3. Place your program's logic into *lib.rs*.
4. The job of the `main` function is:
   * parse arguments
   * set up any other configuration
   * call a `run` function in *lib.rs*
   * if `run` returns an error, handle that error

Whew! The pattern sounds more complicated than it is, honestly. It's all about
separating concerns: *main.rs* handles actually running the program, and
*lib.rs* handles all of the actual logic of the task at hand. Let's re-work our
program into this pattern. First, let's extract a function whose purpose is
only to parse arguments. Listing 12-4 shows the new start of `main` that calls
a new function `parse_config`, which we're still going to define in
*src/main.rs*:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let (search, filename) = parse_config(&args);

    // ...snip...
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let search = &args[1];
    let filename = &args[2];

    (search, filename)
}
```

<figcaption>

Listing 12-4: Extract a `parse_config` function from `main`

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

This may seem like overkill, but we're working in small steps. After making
this change, run the program again to verify that the argument parsing still
works. It's good to check your progress often, so that you have a better idea
of which change caused a problem, should you encounter one.

<!-- steve: cargo check is going to be in stable rust soon, so we should
include it here i think. Thoughts? -->
<!-- I haven't been keeping up with what cargo check does-- it just checks
syntax? If it seems worthwhile to check that but not the functionality, I'd
be into it! /Carol -->

Now that we have a function, let's improve it. Our code still has an indication
that there's a better design possible: we return a tuple, but then immediately
break that tuple up into individual parts again. This code isn't bad on its
own, but there's one other sign we have room for improvement: we called our
function `parse_config`. The `config` part of the name is saying the two values
we return should really be bound together, since they're both part of one
configuration value.

> Note: some people call this antipattern of using primitive values when a
> complex type would be more appropriate *primitive obsession*.

Let's introduce a struct to hold all of our configuration. Listing 12-5 shows
the addition of the `Config` struct definition, the refacting of `parse_config`,
and updates to `main`:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    // ...snip...
}

struct Config {
    search: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let search = args[1].clone();
    let filename = args[2].clone();

    Config {
        search: search,
        filename: filename,
    }
}
```

<figcaption>

Listing 12-5: Refactoring `parse_config` to return an instance of a `Config`
struct

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

The signature of `parse_config` now indicates that it returns a `Config` value.
In the body of `parse_config`, we used to be returning string slices that were
references to `String` values in `args`, but we've defined `Config` to contain
owend `String` values. Because the argument to `parse_config` is a slice of
`String` values, the `Config` instance can't take ownership of the `String`
values: that violates Rust's borrowing rules, since the `args` variable in
`main` owns the `String` values and is only letting the `parse_config` function
borrow them.

There are a number of different ways we could manage the `String` data; for
now, we'll take the easy but less efficient route, and call the `clone` method
on the string slices. The call to `clone` will make a full copy of the string's
data for the `Config` instance to own, which does take more time and memory
than storing a reference to the string data, but cloning the data makes our
code very straightforward.

There's a tendency amongst many Rustaceans to prefer not to use `clone` to fix
ownership problems, due to its runtime cost. In Chapter XX on iterators, we'll
learn how to make this situation more efficient. For now, it's okay to copy a
few strings to keep making progress. We're only going to be making these copies
once, and our filename and search string are both very small. It's better to
have a working program that's a bit inefficient than try to hyper-optimize code
on your first pass. As you get more experienced with Rust, it'll be easier to
skip this step, but for now, it's perfectly acceptable to call `clone`.

We've updated `main` to put the instance of `Config` that `parse_config`
returns in a variable named `config`, and we've updated the code that was using
the separate `search` and `filename` variables to use the fields on the
`Config` struct instead.

This is getting pretty good! Give your program another run to make sure it's
still working. We have two more refactorings to do here, though! Let's think
about the purpose of `parse_config`: it's a function that creates a `Config`
instance. We've already seen a convention for functions that create instances:
a `new` function, like `String::new`. Listing 12-6 shows the result of
transforming `parse_config` into a `new` function associated with our `Config`
struct:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // ...snip...
}

// ...snip...

impl Config {
    fn new(args: &[String]) -> Config {
        let search = args[1].clone();
        let filename = args[2].clone();

        Config {
            search: search,
            filename: filename,
        }
    }
}
```

<figcaption>

Listing 12-6: Changing `parse_config` into `Config::new`

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've changed the name of `parse_config` to `new` and moved it within  an `impl`
block. We've also updated the callsite in `main`. Try compiling this again to
make sure it works.

Here's our last refactoring of this method: remember how accessing the vector
with indices 1 and 2 panics when it contains fewer than 3 items and gives a bad
error message? Let's fix that! Listing 12-7 shows how we can check that our
slice is long enough before accessing those locations, and panic with a better
error message:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// ...snip...
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // ...snip...
```

<figcaption>

Listing 12-7: Adding a check for the number of arguments

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

With these extra few lines of code in `new`, let's try running our program
without any arguments:

```bash
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'not enough arguments', src\main.rs:29
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This is a bit better! We at least have a reasonable error message here.
However, we also have a bunch of extra information that we don't want to give
to our users. We can do better by changing the type signature of `new`. Right
now, it returns only a `Config`, so there's no way to indicate that an error
happened while creating our `Config`. Instead, we can return a `Result`, as
shown in Listing 12-8:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}
```

<figcaption>

Listing 12-8: Return a `Result` from `Config::new`

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Our `new` function now returns a `Result`, with a `Config` instance in the
success case and a `&'static str` when an error happens. Recall from "The
Static Lifetime" section in Chapter 10 `&'static str` is the type of string
literals, which is what our error message is for now.

We've made two changes in the body of the `new` function: instead of calling
`panic!` if there aren't enough arguments, we now return an `Err` value. We
wrapped the `Config` return value in an `Ok`. These changes make the function
conform to its new type signature.

Additionally, we're going to add a new `use` line:

```rust,ignore
use std::process;
```

And in the `main` function itself, we'll handle the `Result` value returned
from the `new` function and exit the process in a cleaner way if `Config::new`
returns an `Err` value:

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We're using a method we haven't covered before that's defined on `Result<T, E>`
by the standard library: `unwrap_or_else`. This method has similar behavior as
`unwrap` if the `Result` is an `Ok` value: it returns the inner value `Ok` is
wrapping. Unlike `unwrap`, if the value is an `Err` value, this method calls a
*closure* which is an anonymous function that we define and pass as an argument
to `unwrap_or_else`. We'll be covering closures in more detail in Chapter XX;
the important part to understand in this case is that `unwrap_or_else` will
pass the inner value of the `Err` to our closure in the argument `err` that
appears between the vertical pipes. Using `unwrap_or_else` lets us do some
custom, non-`panic!` error handling.

Said error handling is only two lines: we print out the error, then call
`std::process::exit`. That function will stop our program's execution
immediately and return the number passed to it as a return code. By convention,
a zero means success and any other value means failure. In the end, this has
similar characteristics to our `panic!`-based handling we had in Listing 12-7,
but we no longer get all the extra output. Let's try it:

```bash
$ cargo run
   Compiling greprs v0.1.0 (file:///project/greprs)
    Finished debug [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target\debug\greprs.exe`
Problem parsing arguments: not enough arguments
```

Great! This output is much friendlier for our users. Now that we're done
refactoring our configuration parsing, let's improve our program's logic.
Listing 12-9 shows the code after extracting a function named `run` that we'll
call from `main`. The `run` function contains the code that was in `main`:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // ...snip...

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// ...snip...
```

<figcaption>

Listing 12-9: Extracting a `run` functionality for the rest of the program logic

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

The contents of `run` are the previous lines that were in `main`, and the `run`
function takes a `Config` as an argument. Now that we have a separate function,
we can make a similar improvement to the one we made to `Config::new` in
Listing 12-8: let's return a `Result<T, E>` instead of calling `panic!` via
`expect`. Listing 12-10 shows the addition of a `use` statement to bring
`std::error::Error` struct into scope and the changes to the `run` function
to return a `Result`:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::error::Error;

// ...snip...

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

<figcaption>

Listing 12-10: Changing the `run` function to return `Result`

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've made three big changes here. The first is the return type of the `run`
function is now `Result<(), Box<Error>>`. Previously, our function returned the
unit type, `()`, so that's still the value returned in the `Ok` case. For our
error type, we're going to use `Box<Error>`. This is called a *trait object*,
which we'll be covering in Chapter XX. For now, think of it like this:
`Box<Error>` means the function will return some kind of type that implements
the `Error` trait, but we're not specifying what particular type the return
value will be. This gives us flexibility because... `Box` means...

Secondly, we've removed our calls to `expect` in favor of `?`, like we talked
about in chapter XX. Rather than `panic!` on an error, this will instead return
the result from the function we're in, as we talked about in that chapter.

Finally, we have to return an `Ok` value from this function, and since we have
declared its type as `()` in our function signature, we need `Ok(())`. This
looks a bit strange at first, but using `()` in this way is the idiomatic way
to say "we are calling `run` for its side effects only, it doesn't return a
value of anything interesting."

This will compile, but with a warning:

```text
warning: unused result which must be used, #[warn(unused_must_use)] on by default
  --> src\main.rs:39:5
   |
39 |     run(config);
   |     ^^^^^^^^^^^^
```

Rust is trying to tell us that we're ignoring our result, which may fail. Let's
handle that now. It similar to the way we handled failure with `Config::new`,
but slightly different:

```rust,ignore
if let Err(e) = run(config) {
    println!("Application error: {}", e);

    process::exit(1);
}
```

Instead of `unwrap_or_else`, we use `if let` to see if `run` returns an `Err`
value. Why? The distinction is a bit subtle. With `Config::new`, we cared about
two things:

1. Detecting any errors that happen
2. Getting a `Config` if no errors happened

In this case, because `run` returns a `()` in the success case, the only thing
we care about is the first case: detecting an error. If we used
`unwrap_or_else`, we'd have to save its return value, which would be `()`.
That's not very useful.

The bodies are the same in both cases though: we print out an error and exit.

This is looking pretty good! There's one more thing we haven't done yet: split
things up into a `lib.rs` as well. Let's do that now. Move `run` from `main.rs`
to a new file, `src/lib.rs`. You'll also need to move the relevant `use`
statements, as well as `Config`:

```rust,ignore
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub search: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            search: search,
            filename: filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

Notice we also made liberal use of `pub`: on `Config`, its `new` function,
and its elements. In addition, on `run`.

Now, for `main.rs`. We need to add in our `lib.rs` through `extern crate`,
use `Config`, and prefix the `run` function with our crate name:

```rust,ignore
extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

With that, everything should work again. Give it a few `cargo run`s and make
sure you haven't broken anything. Whew! That all was a lot of work, but we've
set ourselves up for success in the future. Almost all of our work will be
done in the library from here on out, but we've set up a way to handle errors
in a much nicer fashion, and we've made our code slightly more modular.

Let's take advantage of this newfound modularity by doing something that would
have been hard with our old code, but is easy with our new code: write some
tests!

## Tests

We need to write a function, `grep`, that takes our search term and the text to
search, and procudes a list of search results. Let's remove that `println!` from
`run` (and probably from `main.rs` as well, as we don't really need those anymore
either), and call this function with the options we've collected. We'll add a
dummy implementation of the function, and some tests. Here's our new `run`
function:

File: src/lib.rs

```rust
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    grep(&config.search, &contents);

    Ok(())
}
```

And then our new definitions of `grep` and a test:

File: src/lib.rs

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use grep;

    #[test]
    fn one_result() {
        let search = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            grep(search, contents)
        );
    }
}
```

Being able to do this test is enabled by all that modularization work we did
in the previous sections. By separating the code that relies on dealing with
the environment from our core logic, we can write very straightforward tests
that don't need to load files or deal with command line arguments. Nice!

Before we run the test, let's talk about this type signature:

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
```

We need an explicit lifetime on `grep`, which is what the `<'a>` business is
all about. But why? And what does it do?

Remember, the `'a`s are used to connect the lifetimes of different arguments to
our function. So in this case, we're saying that the vector we're returning is
going to have `&str`s that reference `contents`, not `search`. This is
important! Given that slices and references need to always be valid, if Rust
thought that we were making string slices of `search`, rather than `contents`,
it would do its safety checking incorrectly. If we tried to compile this
function without lifetimes, Rust would fail to compile it:

```text
error[E0106]: missing lifetime specifier
  --> src\lib.rs:37:46
   |
37 | fn grep(search: &str, contents: &str) -> Vec<&str> {
   |                                              ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
	   signature does not say whether it is borrowed from `search` or
           `contents`
```

Rust can't possibly know which of the two arguments we need, so it needs us to
tell it. Since `contents` is the string that contains all of our text, we know
that's the one that our results will refer to, not the search string. And so,
we use the lifetime syntax to connect the two.

The above situation is a little tricky at first, but it gets easier over time!
Practice makes perfect. Don't feel bad if you need to re-read the above
section, and maybe go back and compare with chapter XX's lifetimes section.

Now that we understand that, let's try running our test:

```text
$ cargo test
...warnings...
    Finished debug [unoptimized + debuginfo] target(s) in 0.43 secs
     Running target/debug/deps/greprs-917c5edfc3cf199a

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
	thread 'test::one_result' panicked at 'assertion failed: `(left == right)` (left: `["safe, fast, productive."]`, right: `[]`)', src/lib.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

Great, our test fails, exactly as we expected. Let's get the test to pass! It's
failing because we always return an empty vector. Here's what we have to do
to implement `grep`:

1. Iterate through each line of the contents.
2. Check if the line contains our search string.
   * If it does, add it to the list of values we're returning.
   * If not, do nothing
3. Return our list of results.

Let's take each step at a time. First, iterating through lines. Strings have
a helpful method to handle this, conveniently named `lines`:

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

We can use a `for` loop along with the `lines` method to get each line in turn.
Next, let's see if our line contains the search string. Luckily, strings have a
helpful method named `contains` that does this for us! It looks like this:

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(search) {
            // do something with line
        }
    }
}
```

Finally, we need a way to store these lines that contian our search string.
For that, we can make a vector:

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
}
```

Let's give it a try:

```text
$ cargo test
running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/greprs-2f55ee8cd1721808

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests greprs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Great! It works. However, we can do better. This code isn't bad, but it's very
focused on the fiddly details of each step. We can do better, but to do so, we
need to learn more about iterators and how they work. We'll come back to this
example in that chapter and see how to improve it.

Now that `grep` is working, we need to do one last thing inside of `run`: we never
printed out the results! Try this:

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in grep(&config.search, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

We've added a `for` loop to print out each of the lines we get back from
`grep`. Now it all should be working! Let's try it out:

```text
$ cargo run the poem.txt
   Compiling greprs v0.1.0 (file:///C:/Users/steve/tmp/greprs)
    Finished debug [unoptimized + debuginfo] target(s) in 0.38 secs
     Running `target\debug\greprs.exe the poem.txt`
Then there's a pair of us - don't tell!
To tell your name the livelong day

$ cargo run a poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe a poem.txt`
I'm nobody! Who are you?
Then there's a pair of us - don't tell!
They'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Excellent! We've built our own version of a classic tool, and learned a lot about
how to structure applications. We've also learned a bit about file input and output,
and command line parsing.

## Working with Environment Variables

Let's add one more feature: case insensitive searching. In addtion, this setting won't
be a command line option: it'll be an environment variable instead. But first, let's
build out the functionality. Let's add a new test, and re-name our existing one:

```rust,ignore
#[cfg(test)]
mod test {
    use {grep, grep_case_insensitive};

    #[test]
    fn case_sensitive() {
        let search = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            grep(search, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let search = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            grep_case_insensitive(search, contents)
        );
    }
}
```


### Implement and Test a Case-Insensitive `grep` Function

We also need to define a new function, `grep_case_insensitive`. We
can do this easily: it's almost the same as `grep`, but with one
minor change:

Filename: src/lib.rs

```rust
fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(search) {
            results.push(line);
        }
    }

    results
}
```

All we need to change is add a call to `to_lowercase`. Now, we'll ignore the case
of our contents when searching. Let's see if it works:

```text
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target\debug\deps\greprs-e58e9b12d35dc861.exe

running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

     Running target\debug\greprs-8a7faa2662b5030a.exe

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests greprs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Great! Now, we have to actually use it inside of our library. First, let's
add a configuration option for it:

```rust
pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

And then check for that option inside of `run`:

```rust,ignore
    let results = if config.case_sensitive {
        grep(&config.search, &contents)
    } else {
        grep_case_insensitive(&config.search, &contents)
    };

    for line in results {
        println!("{}", line);
    }
```

We check our configuration, and call the correct function for its setting.
We save the result to a variable, and then do the printing.

Finally, we need to actually check the environment for the variable. This means
two things. First, we add a `use` line at the top of `lib.rs`:

```rust
use std::env;
```

And then using the `vars` method from it inside of `new`:

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = true;

        for (name, _) in env::vars() {
            if name ==  "CASE_INSENSITIVE" {
                case_sensitive = false;
            }
        }

        Ok(Config {
            search: search,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}
```

Here, we call `env::vars`, which is kind of like `env::args`. The difference?
It returns an iterator of environment variables, rather than command line
arguments. Instead of using `collect` to create a vector of all of the
environment variables, we instead use a `for` loop. `env::vars` returns tuples:
the name of the environment varaible, and then its value. We never care about
the values, so we use a `_` to let Rust know that. Finally, we have a
`case_sensitive` variable, which is set to true by default. If we ever find a
`CASE_INSENSITIVE` environment variable, we set it to false instead.

Let's give it a try!

```text
$ cargo run to poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent! Some programs are configurable through environement variables, and some
allow both arguments _and_ environment variables, and decide that one or the other takes
precedence. But now you know how to handle either strategy.

There's a lot more stuff in `std::env` for dealing with environment variables; check out
its documentation for more goodies.

## Write to `stderr` Instead of `stdout`

Right now, we're writing all of our output to the terminal with `println!`.
This works, but most terminals provide two kinds of output: "standard out" is
used for most things, but "standard error" is used for error messages. This
makes it easier to do things like "Print error messages to my terminal, but
write other output to a file."

We can try this behavior with `>`:

```text
$ cargo run > output.txt
```

The `>` syntax says, "please write the contents of standard out to
`output.txt`." If we open *output.txt* we'll see:

```text
Application error: No search string or filename found
```

We'd like this to be printed to the screen instead. Let's make a change!


Filename: src/main.rs

```rust,ignore
extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    if let Err(e) = greprs::run(config) {
        let mut stderr = std::io::stderr();

        writeln!(
            &mut stderr,
            "Application error: {}",
            e
        ).expect("Could not write to stderr");

        process::exit(1);
    }
}
```

Rust does not have a convenient function like `println!` for writing to
standard error. Instead, we use the `writeln!` macro, which is sort of like
`println!`, but it takes an extra argument: the first thing we pass to it is
what to write to. We can aquire a handle to standard error through the
`std::io::stderr` function, and we give a mutable reference to it to
`wrintln!`; we need it to be mutable so we can write to it! The second and
third arguments to `writeln!` are like the first and second arguments to
`println!`: a format string, and then any variables we're interpolating.

Let's try running it again with `>`:

```text
$ cargo run > output.txt
Application error: No search string or filename found
```

Now we see our error on the screen, but `output.txt` contains nothing.
If we try it again with good arguments:

```text
$ cargo run to poem.txt > output.txt
```

We'll see no output to our terminal, but `output.txt` will contain
our results:

```text
Are you nobody, too?
How dreary to be somebody!
```
