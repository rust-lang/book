# An I/O Project

We've learned a lot over the last few chapters. Let's take that new knowledge
and apply it by building a project together. Along the way, we'll learn a bit
more about Rust's standard library.

So what should we build? One that uses Rust's strengths. A great use of Rust is
for command-line tools: Rust's speed, safety, and 'single binary' output make
it a good choice for this kind of task. So we'll make our own version of a
classic command line tool: `grep`. `grep` is short for "*g*lobally search a
*r*egular *e*xpression and *p*rint." In other words, it does this:


- Takes a filename and a string as arguments.
- Reads the file.
- Finds lines in the file that contain the string argument.
- Prints out those lines.

In addition, we'll add one extra feature: an environment variable that will
allow us to search for the text in a case-insensitive way.

There's another great reason to use `grep` as an example project: a very
fully-featured version of `grep` has already been created in Rust by a
community member, Andrew Gallant. It's called `ripgrep`, and it's very,
very fast. While our version of `grep` will be fairly simple, you'll have
some of the basic knowledge to check out that project if you want to see
something more real-world.

This project will bring together a number of things we learned previously:

- Organize code (using what we learned in modules, ch 7)
- Use vectors and strings (collections, ch 8)
- Handle errors (ch 9)
- Use traits and lifetimes where appropriate (ch 10)
- Have tests (ch 11)

Let's make our project with, as always, `cargo new`:

```text
$ cargo new --bin greprs
     Created binary (application) `greprs` project
$ cd greprs
```

We're calling our version of `grep` 'greprs', so that we don't confuse any of
our users that it's the more fully-featured version of `grep` you may already
have installed on your system.

## Command line arguments

Our first task is to have `greprs` accept its two command-line arguments. There
are some existing libraries on crates.io that can help us do this, but since
we're learning, we'll implement this ourselves.

To do this, we'll need to call a function provided in Rust's standard library:
`std::env::args`. This function returns an *iterator* of the command-line
arguments that were given to our program. We haven't discussed iterators yet;
chapter 16 will cover them fully. For our purposes, though, we don't need to
understand much about them. We only need to understand two things:

1. Iterators produce a series of values by repeatedly calling a `next()`
   function.
2. We can call the `collect` function on an iterator to turn it into a vector
   of the elements of the iterator.

Let's give it a try.

Filename: src/main.rs

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

You'll notice a few things about this code. First, we have a `use` statement to
bring the `std::env` module into scope. When using a function that's nested in
more than one level of module, like `std::env::args` is, it's conventional to
use `use` to bring the parent module into scope, rather than the function
itself. `env::args` is less ambiguous than a lone `args`. Also, if we end up
using more than one function in `std::env`, we only need a single `use`.

On the first line of `main`, we call `env::args`, and then immediately use
`collect` to create a vector out of it. We're also explicitly annotating `args`
here: `collect` can be used to create many kinds of collections, and so Rust
won't be able to infer what kind of type we want, so the annotation is
required. We very rarely need to annotate types in Rust, but `collect` is a
one function where you very often need to.

Finally, we print out the vector with the debug formatter, `:?`. Let's try
running our code with various arguments:

```text
$ cargo run
["target/debug/greprs"]

$ cargo run needle haystack
...snip...
["target/debug/greprs", "needle", "haystack"]
```

You'll notice one interesting thing: the name of the binary is the first
argument. The reasons for this are out of the scope of this chapter, but it's
something we'll have to remember.

Now that we have a way to access all of the arguments, let's find the ones we
care about, and pull them out into their own variables:

Filename: src/main.rs

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

Remember, the program's name is the first argument, so we don't need `args[0]`.
Let's try running this program again:

```text
$ cargo run test sample.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe test sample.txt`
Searching for test
In file sample.txt
```

Great! There's one problem, though. Let's try giving it no arguments:

```text
steve@becoming  ~/tmp/greprs (master)
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1', ../src/libcollections\vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Because our vector only has one element, but we tried to access the second
element, our program panics with a message about the out-of-bound access. While
this error message is _accurate_, it's not meaningful to users of our program
at all. We could fix this problem right now, but let's push forward: we'll
improve this situation before we're finished.

## Reading a file

Now that we have some variables containing the information that we need, let's
try using them. The next step is to open the file that we want to search. To do
that, we need a file. Create one called `poem.txt` at the root level of your
project, and fill it up with some Emily Dickinson:

Filename: poem.txt

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

With that in place, let's edit `main.rs` to open the file:

Filename: src/main.rs

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

We've added a few things. First of all, we need some more `use` statements to
bring in the relevant parts of the standard library: we need `std::fs::File`
for dealing with files, and `std::io::prelude::*` contains various traits that
are useful when doing I/O, including file I/O. In the same way that Rust has a
general prelude that brings certain things into scope automatically, the
`std::io` module has its own prelude of common things you'll need. Unlike the
default prelude, we must explicitly `use` the prelude in `std::io`.

In `main`, we've added three things: first, we open our file with `File::open`.
Second, we create a `String`, and then call `read_to_string` on our file,
placing the contents of the file into the string. Finally, we print out the
contents.

Let's try running it:

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
is still small, they're not a huge deal, but as our program grows, it will be
harder and harder to fix them in a clean way. Let's do the refactoring now,
instead of waiting. The refactoring will be much easier to do with only this
small amount of code.

So what are these problems? There are four. The first is opening the file:
we've used `expect` to print out an error message if opening the file fails,
but it only says "file not found". There are a number of ways that opening a
file can fail, but we're always assuming that it's due to the file being
missing. For example, the file could exist, but we don't have permission to
open it: right now, we print an error message that says the wrong thing!
Secondly, our use of `expect` over and over recalls our earlier issue with the
`panic!`s on indexing: while it _works_, it's a bit unprincipled, and we're
doing it all throughout our program. It would be nice to put our error handling
in one spot. Finally, our `main` function now does two things: it parses
arguments, and opens up files. For such a small function, this isn't a problem,
but as we keep growing our program inside of `main`, it will get larger and
larger. This also ties into our fourth problem: while `search` and `filename`
are configuration variables to our program, things like `f` and `contents` are
used to do our program's logic. The longer `main` gets, the more variables
we're going to bring into scope, and the more variables we have in scope, the
harder it is to keep track of which ones we need and for which purpose.

These problems are common to many similar kinds of projects, and so the Rust
community has developed a pattern for dealing with them. This pattern is useful
for organizing any binary project you'll build in Rust, and so we can do this
refactoring a bit earlier, since we know that our project fits the pattern.
It looks like this:

1. Split your program into both a `main.rs` and a `lib.rs`.
2. Place your command-line parsing logic into `main.rs`.
3. Place your program's logic into `lib.rs`.
4. The job of the `main` function is:
   * parse arguments
   * set up any other configuration
   * call a `run` function in `lib.rs`
   * if `run` returns an error, handle that error


Whew! It sounds like a lot more than it is, honestly. It's all about separating
concerns: `main.rs` handles actually running the program, and `lib.rs` handles
all of the actual logic of the task at hand. This pattern has additional
benefits that we won't talk about just yet. Let's re-work our program into this
pattern. First, let's split out a function to parse arguments. Here's the new
start of `main`:

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let (search, filename) = parse_config(&args);

    println!("Searching for {}", search);
    println!("In file {}", filename);
```

And the definition of `parse_config`:

```rust,ignore
fn parse_config(args: &[String]) -> (&str, &str) {
    let search = &args[1];
    let filename = &args[2];
    
    (search, filename)
}
```

This may seem like overkill, but we're working in small steps. After making
this change, run the program again to verify that things still work. It's good
to do this often.

<!-- steve: cargo check is going to be in stable rust soon, so we should
include it here i think. Thoughts? -->

Now that we have a function, let's improve it. Our code has a smell: we return
a tuple, but then immediately break that tuple up into individual parts again.
This isn't bad on its own, but there's one other thing: we called our function
`parse_config`. That is, these two variables should really be bound together,
as they're both part of a configuration.

> Note: some people call this smell "primitive obsession."

Let's introduce a struct to hold all of our configuration. It will look like
this:

```rust
struct Config {
    search: String,
    filename: String,
}
```

And we can refactor `parse_string` like this:

```rust,ignore
fn parse_config(args: &[String]) -> Config {
    let search = args[1].clone();
    let filename = args[2].clone();
    
    Config {
        search: search,
        filename: filename,
    }
}
```

We're now returning a `Config`, but there's one other change. We used to be
returning string slices, but our `Config` contains `String`s. And since we have
a slice of `String`s, we can't take ownership of them, as that violates Rust's
borrowing rules. There are a number of different things we could do in this
situation, but for now, we'll take the "easy but less efficient" route, and
call `clone`. This will make a full copy of the string's data, but it makes our
code very straightforward. There's a tendency amongst many Rustaceans to really
dislike calls to `clone` to fix ownership problems, due to that lack of
efficiency. In chapter XX on iterators, we'll learn the tricks we need to make
this more efficient. But for now, it's okay to copy a few strings to keep
making progress. We're only going to be making this copy one time, and our
filename and search string are both very small. It's better to have a working
program and have it be a bit inefficient then to try to hyper-optimize code on
your first pass. As you get more experienced with Rust, it'll be easier to skip
this step, but for now, just call `clone`. It's fine.

Whew, sorry about that little digression! Before our program works, we need to
update `main` as well:

```rust,ignore
let config = parse_config(&args);

println!("Searching for {}", config.search);
println!("In file {}", config.filename);

let mut f = File::open(config.filename).expect("file not found");
```

We now have a single `config` variable, and so we need to update everything to
use it instead of the old variables.

This is getting pretty good! Give your program another run to make sure it's
still working. We have two more refactorings to do here, though! Let's think
about `parse_config`. It's a function that creates a `Config`. We already know
a convention for this: a `new` method. Let's transform `parse_config` into one:

```rust,ignore
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

The only changes here are putting it into an `impl` block and changing the name
to `new`. Easy! We do need to update the callsite in `main` too, though:

```rust,ignore
let config = Config::new(&args);
```

Try compiling this again to make sure it works.

Here's our last refactoring: remember how accessing the vector with incorrect
indices panics and gives a bad error message? Let's fix that! To do that, we
can check that our slice is long enough:

```rust,ignore
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
```

With these extra few lines of code in `new`, let's try running our program with
incorrect arguments:

```bash
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'not enough arguments', src\main.rs:29
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This is a bit better! We at least have a reasonable panic message here.
However, we also have a bunch of extra information here that we don't want to
give to our users. We can do better. To do that, we need to change the type
signature of `new`. Right now, because it returns only a `Config`, there's no
way to indicate that an error happened while creating our `Config`. So instead,
we can use `Result`, like this:

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

Our `new` function now gives us a `Result`, with a `&'static str` as our error
message. Remember, this is the type of string literals, which is what our error
message is right now.

We've made two changes in the body: instead of `panic!`ing, we now return an
`Err`, and we wrapped our return value in an `Ok`. This makes it conform to the
new type signature.

Finally, we make some changes in `main`. Up top, we need a new `use` line:

```rust,ignore
use std::process;
```

And then in the main function itself:

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
```

Here, we see a new method on `Result<T, E>`: `unwrap_or_else`. This method does
something similar to `unwrap`; that is, if the `Result<T, E>` is `Ok`, it
returns the inner value `Ok` is wrapping. But unlike `unwrap`, if the value is
an `Err`, it instead calls a closure, passing the inner value of the `Err` to
it. This lets us do some custom, non-`panic!` error handling.

Said error handling is only two lines: we print out the error, and then call
`std::process::exit`. That function will stop our program's execution
immediately, and return the number passed to it as a return code. By convention,
a zero means success and any other value means failure. So in the end, this has
similar characteristics to our `panic!`-based handling from before, but we no
longer get all the extra output. Let's try it:

```bash
$ cargo run
   Compiling greprs v0.1.0 (file:///C:/Users/steve/tmp/greprs)
    Finished debug [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target\debug\greprs.exe`
Problem parsing arguments: not enough arguments
```

Great! This is much nicer. Now that we're done with fixing up our configuration
parsing, let's do some work on our program's logic. Let's create a new function,
and then call it from `main`:

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
```

The contents of `run` are the previous lines that were in `main`, and we take a
`Config` as an argument. Now that we have a separate function, we can do
something very similar to what we did with `Config::new`: let's have it return
a `Result<T, E>` instead of `panic!`ing with `expect`. First, we need to add a
line at the top:

```rust
use std::error::Error;
```

And then change our `run()` function to look like this:

```rust,ignore
fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

We have three big changes here. The first is the return type: `Result<(),
Box<Error>>`. Previously, our function returned nothing, aka `()`, so that's
still our `Ok` case. For our error type, we're going to use `Box<Error>`. This
is called a "trait object," which we'll be covering in chapter XX. For now,
think of it like this: a `Box<Error>` means "I just want some kind of error,
I don't care what specific kind." We could do something more specialized, but
this is the most straightforward thing for now.

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
that don't need to load files or deal with command-line arguments. Nice!

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
and command-line parsing.

## Working with Environment Variables

Let's add one more feature: case insensitive searching. In addtion, this setting won't
be a command-line option: it'll be an environment variable instead. But first, let's
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
It returns an iterator of environment variables, rather than command-line
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

Let's say we want to output any errors to `stderr` instead of `stdout`. Right now, if we run:

```text
$ cargo run > output.txt
```

The contents of *output.txt* will be:

```text
Application error: No search string or filename found
```

Even if we're saving the output to a file, we want to see errors on the screen.

Filename: src/main.rs

```rust,ignore
extern crate greprs;

use greprs::Config;

use std::env;
use std::process;
use std::io::prelude::*;

fn main() {
    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    let case_insensitive = env::vars().find(|&(ref var, _)| {
        var ==  "CASE_INSENSITIVE"
    }).is_some();

    let config = Config {
        arguments: args.collect(),
        case_insensitive: case_insensitive,
    };

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

Now the output when we don't pass any arguments but redirect stdout to a file
is:

```text
$ cargo run > output.txt
Application error: No search string or filename found
```

and the file is empty.
