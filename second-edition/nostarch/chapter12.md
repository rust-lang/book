
[TOC]

# An I/O Project

We've learned a lot over the last few chapters. Let's take that new knowledge
and apply it by building a project together. Along the way, we'll learn a bit
more about Rust's standard library.

So what should we build? One that uses Rust's strengths. A great use of Rust is
for command line tools: Rust's speed, safety, 'single binary' output, and
cross-platform support make it a good language choice for this kind of task. So
we'll make our own version of a classic command line tool: `grep`. `grep` is
short for "Globally search a Regular Expression and Print." In the
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

1. Iterators produce a series of values.
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

## Improving Error Handling and Modularity

There are four problems that we'd like to fix to improve our program, and they
all have to do with potential errors and the way the program is structured. The
first problem is where we open the file: we've used `expect` to print out an
error message if opening the file fails, but the error message only says "file
not found". There are a number of ways that opening a file can fail, but we're
always assuming that it's due to the file being missing. For example, the file
could exist, but we might not have permission to open it: right now, we print
an error message that says the wrong thing!

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
harder it is to keep track of which ones we need for which purpose. It would be
better if we grouped the configuration variables into one structure to make
their purpose clear.

Let's address these problems by restructuring our project.

### Separation of Concerns for Binary Projects

These kinds of organizational problems are common to many similar kinds of
projects, so the Rust community has developed a pattern for organizing the
separate concerns. This pattern is useful for organizing any binary project
you'll build in Rust, so we can justify doing this refactoring a bit earlier,
since we know that our project fits the pattern. The pattern looks like this:

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

### Grouping Configuration Values

Now that we have a function, let's improve it. Our code still has an indication
that there's a better design possible: we return a tuple, but then immediately
break that tuple up into individual parts again. This code isn't bad on its
own, but there's one other sign we have room for improvement: we called our
function `parse_config`. The `config` part of the name is saying the two values
we return should really be bound together, since they're both part of one
configuration value.

> Note: some people call this anti-pattern of using primitive values when a
> complex type would be more appropriate *primitive obsession*.

Let's introduce a struct to hold all of our configuration. Listing 12-5 shows
the addition of the `Config` struct definition, the refactoring of
`parse_config`, and updates to `main`:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
# use std::env;
#
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
owned `String` values. Because the argument to `parse_config` is a slice of
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

<!-- PROD: START BOX -->

> #### The Tradeoffs of Using `clone`
>
> There's a tendency amongst many Rustaceans to prefer not to use `clone` to fix
> ownership problems due to its runtime cost. In Chapter XX on iterators, we'll
> learn how to make this situation more efficient. For now, it's okay to copy a
> few strings to keep making progress. We're only going to be making these
> copies once, and our filename and search string are both very small. It's
> better to have a working program that's a bit inefficient than try to
> hyper-optimize code on your first pass. As you get more experienced with Rust,
> it'll be easier to skip this step, but for now, it's perfectly acceptable to
> call `clone`.

<!-- PROD: END BOX -->

We've updated `main` to put the instance of `Config` that `parse_config`
returns in a variable named `config`, and we've updated the code that was using
the separate `search` and `filename` variables to use the fields on the
`Config` struct instead.

### Creating a Constructor for `Config`

Let's now think about the purpose of `parse_config`: it's a function that
creates a `Config` instance. We've already seen a convention for functions that
create instances: a `new` function, like `String::new`. Listing 12-6 shows the
result of transforming `parse_config` into a `new` function associated with our
`Config` struct:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust

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

### Returning a `Result` from the Constructor

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

### Calling `Config::new` and Handling Errors

Now we need to make some changes to `main` as shown in Listing 12-9:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
```

<figcaption>

Listing 12-9: Exiting with an error code if creating a new `Config` fails

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've added a new `use` line to import `process` from the standard library.
In the `main` function itself, we'll handle the `Result` value returned
from the `new` function and exit the process in a cleaner way if `Config::new`
returns an `Err` value.

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
   Compiling greprs v0.1.0 (file:///projects/greprs)
    Finished debug [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target\debug\greprs.exe`
Problem parsing arguments: not enough arguments
```

Great! This output is much friendlier for our users.

### Handling Errors from the `run` Function

Now that we're done refactoring our configuration parsing, let's improve our
program's logic. Listing 12-10 shows the code after extracting a function named
`run` that we'll call from `main`. The `run` function contains the code that
was in `main`:

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

Listing 12-10: Extracting a `run` functionality for the rest of the program logic

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

The contents of `run` are the previous lines that were in `main`, and the `run`
function takes a `Config` as an argument. Now that we have a separate function,
we can make a similar improvement to the one we made to `Config::new` in
Listing 12-8: let's return a `Result<T, E>` instead of calling `panic!` via
`expect`. Listing 12-11 shows the addition of a `use` statement to bring
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

Listing 12-11: Changing the `run` function to return `Result`

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
value will be. This gives us flexibility to return error values that may be of
different types in different error cases. `Box` is a smart pointer to heap
data, and we'll be going into detail about `Box` in Chapter YY.

The second change is that we've removed our calls to `expect` in favor of `?`,
like we talked about in Chapter 9. Rather than `panic!` on an error, this will
return the error value from the function we're in for the caller to handle.

The third change is that we're now returning an `Ok` value from this function
in the success case. Because we've declared the `run` function's success type
as `()` in the signature, we need to wrap the unit type value in the `Ok`
value. `Ok(())` looks a bit strange at first, but using `()` in this way is the
idiomatic way to indicate that we're calling `run` for its side effects only;
it doesn't return anything interesting.

This will compile, but with a warning:

```text
warning: unused result which must be used, #[warn(unused_must_use)] on by default
  --> src\main.rs:39:5
   |
39 |     run(config);
   |     ^^^^^^^^^^^^
```

Rust is trying to tell us that we're ignoring our `Result`, which might be an
error value. Let's handle that now. We'll use a similar technique as the way we
handled failure with `Config::new` in Listing 12-9, but with a slight
difference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // ...snip...

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Instead of `unwrap_or_else`, we use `if let` to see if `run` returns an `Err`
value and call `process::exit(1)` if so. Why? The distinction between this case
and the `Config::new` case is a bit subtle. With `Config::new`, we cared about
two things:

1. Detecting any errors that happen
2. Getting a `Config` if no errors happened

In this case, because `run` returns a `()` in the success case, the only thing
we care about is the first case: detecting an error. If we used
`unwrap_or_else`, we'd get its return value, which would be `()`. That's not
very useful.

The bodies of the `if let` and of the `unwrap_or_else` are the same in both
cases though: we print out an error and exit.

### Split Code into a Library Crate

This is looking pretty good! There's one more thing we haven't done yet: split
the *src/main.rs* up and put some code into *src/lib.rs* Let's do that now:
move the `run` function from *src/main.rs* to a new file, *src/lib.rs*. You'll
also need to move the relevant `use` statements and the definition of `Config`
and its `new` method as well. Your *src/lib.rs* should now look like Listing
12-12:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 12-12: Moving `Config` and `run` into *src/lib.rs*

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Notice we also made liberal use of `pub`: on `Config`, its fields and its `new`
method, and on the `run` function.

Now in *src/main.rs*, we need to bring in the code that's now in *src/lib.rs*
through `extern crate greprs`. Then we need to add a `use greprs::Config` line
to bring `Config` into scope, and prefix the `run` function with our crate name
as shown in Listing 12-13:

<figure>
<span class="filename">Filename: src/main.rs</span>

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

<figcaption>

Listing 12-13: Bringing the `greprs` crate into the scope of *src/main.rs*

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

With that, everything should work again. Give it a few `cargo run`s and make
sure you haven't broken anything. Whew! That all was a lot of work, but we've
set ourselves up for success in the future. We've set up a way to handle errors
in a much nicer fashion, and we've made our code slightly more modular. Almost
all of our work will be done in *src/lib.rs* from here on out.

Let's take advantage of this newfound modularity by doing something that would
have been hard with our old code, but is easy with our new code: write some
tests!

## Testing the Library's Functionality

Writing tests for the core functionality of our code is now easier since we
extracted the logic into *src/lib.rs* and left all the argument parsing and
error handling in *src/main.rs*. We can now call our code directly with various
arguments and check return values without having to call our binary from the
command line.

We're going to write a function named `grep` that takes our search term and the
text to search and produces a list of search results. Let's remove that
`println!` from `run` (and from *src/main.rs* as well, as we don't really need
those anymore either), and call the new `grep` function with the options we've
collected. We'll add a placeholder implementation of the function for now, and
a test that specifies the behavior we'd like the `grep` function to have. The
test will fail with our placeholder implementation, of course, but we can make
sure the code compiles and that we get the failure message we expect. Listing
12-14 shows these modifications:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}

pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    grep(&config.search, &contents);

    Ok(())
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

<figcaption>

Listing 12-14: Creating a function where our logic will go and a failing test
for that function

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Notice that we need an explicit lifetime `'a` declared in the signature of
`grep` and used with the `contents` argument and the return value. Remember,
lifetime parameters are used to specify which arguments' lifetimes connect to
the lifetime of the return value. In this case, we're indicating that the
vector we're returning is going to contain string slices that reference slices
of the argument `contents`, as opposed to referencing slices of the argument
`search`. Another way to think about what we're telling Rust is that the data
returned by the `grep` function will live as long as the data passed into this
function in the `contents` argument. This is important! Given that the data a
slice references needs to be valid in order for the reference to be valid, if
the compiler thought that we were making string slices of `search` rather than
`contents`, it would do its safety checking incorrectly. If we tried to compile
this function without lifetimes, we would get this error:

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
tell it. Because `contents` is the argument that contains all of our text and
we want to return the parts of that text that match, we know `contents` is the
argument that should be connected to the return value using the lifetime syntax.

Connecting arguments to return values in the signature is something that other
programming languages don't make you do, so don't worry if this still feels
strange! Knowing how to specify lifetimes gets easier over time, and practice
makes perfect. You may want to re-read the above section or go back and compare
this example with the Lifetime Syntax section in Chapter 10.

Now let's try running our test:

```text
$ cargo test
...warnings...
    Finished debug [unoptimized + debuginfo] target(s) in 0.43 secs
     Running target/debug/deps/greprs-abcabcabc

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
	thread 'test::one_result' panicked at 'assertion failed: `(left == right)`
(left: `["safe, fast, productive."]`, right: `[]`)', src/lib.rs:16
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

Great, our test fails, exactly as we expected. Let's get the test to pass! It's
failing because we always return an empty vector. Here's what we're going to do
to implement `grep`:

1. Iterate through each line of the contents.
2. Check if the line contains our search string.
   * If it does, add it to the list of values we're returning.
   * If not, do nothing.
3. Return the list of results that match.

Let's take each step at a time, starting with iterating through lines. Strings
have a helpful method to handle this, conveniently named `lines`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

<!-- Will add wingdings in libreoffice /Carol -->

We're using a `for` loop along with the `lines` method to get each line in turn.
Next, let's see if our line contains the search string. Luckily, strings have a
helpful method named `contains` that does this for us! Using the `contains`
method looks like this:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(search) {
            // do something with line
        }
    }
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Finally, we need a way to store the lines that contain our search string. For
that, we can make a mutable vector before the `for` loop and call the `push`
method to store a `line` in the vector. After the `for` loop, we return the
vector:

<span class="filename">Filename: src/lib.rs</span>

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

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

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

Great! It works. Now that our test is passing, we could consider opportunities
for refactoring the implementation of `grep` and be certain we maintain the
same functionality while we do so. This code isn't bad, but it isn't taking
advantage of some useful features of iterators. We'll be coming back to this
example in Chapter 13 where we'll explore iterators in detail and see how to
improve it.

Now that the `grep` function is working, we need to do one last thing inside of
the `run` function: we never printed out the results! We'll do that by adding
a `for` loop that prints each line returned from the `grep` function:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in grep(&config.search, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Now our whole program should be working! Let's try it out:

```text
$ cargo run the poem.txt
   Compiling greprs v0.1.0 (file:///projects/greprs)
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

Excellent! We've built our own version of a classic tool, and learned a lot
about how to structure applications. We've also learned a bit about file input
and output, lifetimes, testing, and command line parsing.

## Working with Environment Variables

Let's add one more feature: case insensitive searching. In addition, this
setting won't be a command line option: it'll be an environment variable
instead. We could choose to make case insensitivity a command line option, but
our users have requested an environment variable that they could set once and
make all their searches case insensitive in that terminal session.

### Implement and Test a Case-Insensitive `grep` Function

First, let's add a new function that we will call when the environment variable
is on. Let's start by adding a new test and re-naming our existing one:

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

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We're going to define a new function named `grep_case_insensitive`. Its
implementation will be almost the same as the `grep` function, but with some
minor changes:

<span class="filename">Filename: src/lib.rs</span>

```rust
fn grep_case_insensitive<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(line);
        }
    }

    results
}
```

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

First, we lowercase the `search` string, and store it in a shadowed variable
with the same name. Note that `search` is now a `String` rather than a string
slice, so we need to add an ampersand when we pass `search` to `contains` since
`contains` takes a string slice.

Second, we add a call to `to_lowercase` each `line` before we check if it
contains `search`. Since we've converted both `line` and `search` into all
lowercase, we'll find matches no matter what case they used in the file and the
command line arguments, respectively. Let's see if this passes the tests:

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

Great! Now, we have to actually use the new `grep_case_insensitive` function.
First, let's add a configuration option for it to the `Config` struct:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

<!-- Will add ghosting in libreoffice /Carol -->

And then check for that option inside of the `run` function, and decide which
function to call based on the value of the `case_sensitive` function:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        grep(&config.search, &contents)
    } else {
        grep_case_insensitive(&config.search, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<!-- Will add ghosting in libreoffice /Carol -->

Finally, we need to actually check the environment for the variable. To bring
the `env` module from the standard library into our project, we add a `use` line
at the top of *src/lib.rs*:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::env;
```

And then use the `vars` method from the `env` module inside of `Config::new`:

<span class="filename">Filename: src/lib.rs</span>

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
            if name == "CASE_INSENSITIVE" {
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

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Here, we call `env::vars`, which works in a similar way as `env::args`. The
difference is `env::vars` returns an iterator of environment variables rather
than command line arguments. Instead of using `collect` to create a vector of
all of the environment variables, we're using a `for` loop. `env::vars` returns
tuples: the name of the environment variable and its value. We never care about
the values, only if the variable is set at all, so we use the `_` placeholder
instead of a name to let Rust know that it shouldn't warn us about an unused
variable. Finally, we have a `case_sensitive` variable, which is set to true by
default. If we ever find a `CASE_INSENSITIVE` environment variable, we set the
`case_sensitive` variable to false instead. Then we return the value as part of
the `Config`.

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

Excellent! Our `greprs` program can now do case insensitive searching controlled
by an environment variable. Now you know how to manage options set using
either command line arguments or environment variables!

Some programs allow both arguments _and_ environment variables for the same
configuration. In those cases, the programs decide that one or the other of
arguments or environment variables take precedence. For another exercise on
your own, try controlling case insensitivity through a command line argument as
well, and decide which should take precedence if you run the program with
contradictory values.

The `std::env` module contains many more useful features for dealing with
environment variables; check out its documentation to see what's available.

## Write to `stderr` Instead of `stdout`

Right now, we're writing all of our output to the terminal with `println!`.
This works, but most terminals provide two kinds of output: "standard out" is
used for most information, but "standard error" is used for error messages. This
makes it easier to do things like "Print error messages to my terminal, but
write other output to a file."

We can see that our program is only capable of printing to `stdout` by
redirecting it to a file using `>` on the command line, and running our program
without any arguments, which causes an error:

```text
$ cargo run > output.txt
```

The `>` syntax tells the shell to write the contents of standard out to
*output.txt* instead of the screen. However, if we open *output.txt* after
running we'll see our error message:

```text
Application error: No search string or filename found
```

We'd like this to be printed to the screen instead, and only have the output
from a successful run end up in the file if we run our program this way. Let's
change how error messages are printed as shown in Listing 12-15:

<figure>
<span class="filename">Filename: src/main.rs</span>

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

<figcaption>

Listing 12-15: Writing error messages to `stderr` instead of `stdout`

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Rust does not have a convenient function like `println!` for writing to
standard error. Instead, we use the `writeln!` macro, which is sort of like
`println!`, but it takes an extra argument. The first thing we pass to it is
what to write to. We can acquire a handle to standard error through the
`std::io::stderr` function. We give a mutable reference to `stderr` to
`writeln!`; we need it to be mutable so we can write to it! The second and
third arguments to `writeln!` are like the first and second arguments to
`println!`: a format string and any variables we're interpolating.

Let's try running the program again in the same way, without any arguments and
redirecting `stdout` with `>`:

```text
$ cargo run > output.txt
Application error: No search string or filename found
```

Now we see our error on the screen, but `output.txt` contains nothing. If we
try it again with arguments that work:

```text
$ cargo run to poem.txt > output.txt
```

We'll see no output to our terminal, but `output.txt` will contain
our results:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

## Summary

In this chapter, we've covered how to do common I/O operations in a Rust
context. By using command line arguments, files, environment variables, and the
ability to write to `stderr`, you're now prepared to write command line
applications. By using the concepts from previous chapters, your code will be
well-organized, be able to store data effectively in the appropriate data
structures, handle errors nicely, and be well tested. We also saw a real-world
scenario where lifetime annotations are needed to ensure references are
always valid.

Next, let's explore how to make use of some features of Rust that were
influenced by functional languages: closures and iterators.
