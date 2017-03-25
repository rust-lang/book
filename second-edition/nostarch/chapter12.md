
[TOC]

# An I/O Project

<!-- We might need a more descriptive title, something that captures the new
elements we're introducing -- are we going to cover things like environment
variables more in later chapters, or is this the only place we explain how to
use them? -->

This chapter is both a recap of the many skills you've learned so far and an
exploration of a few more standard library features. We're going to build an
input/output project to practice some of the Rust you now have under your belt.

Rust's speed, safety, 'single binary' output, and cross-platform support make
it a good language for creating command line tools, so for our project we'll
make our own version of the classic command line tool `grep`, an acronym for
"Globally search a Regular Expression and Print." In the simplest use case,
`grep` searches a specified file for a specified string using the following
steps:

- Take as arguments a filename and a string.
- Read the file.
- Find lines in the file that contain the string argument.
- Print out those lines.

We'll also add one extra feature to our function that `grep` doesn't have: an
environment variable that will allow us to search for the string argument in a
case-insensitive way.

One Rust community member, Andrew Gallant, has already created a
fully-featured, very fast version of `grep`, called `ripgrep`. By comparison,
our version of `grep` will be fairly simple, but this is a good real-world
example `grep` to use for reference or ideas.

This project will bring together a number of concepts you've learned so far:

- Organizing code (using what we learned in modules, Chapter 7)
- Using vectors and strings (collections, Chapter 8)
- Handling errors (Chapter 9)
- Using traits and lifetimes where appropriate (Chapter 10)
- Running tests (Chapter 11)

We'll also briefly introduce closures, iterators, and trait objects, which
Chapters XX, YY, and ZZ respectively will cover in detail.

Let's create a new project with, as always, `cargo new`:

```text
$ cargo new --bin greprs
     Created binary (application) `greprs` project
$ cd greprs
```

We're calling our version 'greprs'.

<!-- Unless I'm misunderstanding something, it seems like we start calling it
merely "greps" at the end of the project, maybe something to look out for -->

## Accepting Command Line Arguments

Our first task is to make `greprs` able to accept its two command line
arguments: the filename and a string to search for. There are some existing
libraries on crates.io that can help us do this, but since you're learning
let's implement this ourselves.

<!--Below -- I'm not clear what we need the args function for, yet, can you set
it out more concretely? Otherwise, will it make more sense in context of the
code later? Is this function needed to allow our function to accept arguments,
is that was "args" is for? -->

### Creating the Argument Placeholders

<!-- you'll see from my comments that I wasn't entirely sure what we were doing
at different point in this section, I've tried to make it clearer and added
some headings, but plesae do change if I'm misunderstanding. -->

We'll need to call a function provided in Rust's standard library:
`std::env::args`. This function returns an *iterator* of the command line
arguments that were given to our program. We haven't discussed iterators yet,
and we'll cover them fully in Chapter 16, but for our purposes now we only need
to know two things about iterators:

1. Iterators produce a series of values.
2. We can call the `collect` function on an iterator to turn it into a vector
   containing all of the elements the iterator produces.

Let's give it a try; use the code in Listing 12-1 to create the two command
line arguments our `grep` function needs and collect them into a vector.

<!-- Give what a try, here, what are we making? Can you lay that out? I've
tried above but I'm not sure it's complete -->

<span class="filename">Filename: src/main.rs</span>

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

Listing 12-1: Collect the command line arguments into a vector and print them
out

<!-- Will add wingdings in libreoffice /Carol -->

First, we bring the `std::env` module into scope with a `use` statement so that
we can use its `args` function. Notice we have two environments here: the
`std::env::args` function is nested in two levels of module. In cases where the
desired function is nested in multiple evironments it's conventional to bring
the parent module into scope, rather than the function itself, as that allows
you to easily use other functions from `std::env`, and is less ambiguous than
entering a lone `args`.

<!--what is it we're making into a vector here, the arguments we pass?-->

On the first line of `main`, we call `env::args`, and immediately use `collect`
make it into a vector. The `collect` function can be used to create many kinds
of collections so we explictly annotate the type of `args` to specify that we
want a string type. Though we very rarely need to annotate types in Rust,
`collect` is one function you do often need to annotate because Rust isn't able
to infer what kind of type you want.

Finally, we print out the vector with the debug formatter, `:?`. Let's try
running our code with no arguments, and then with two arguments:

```text
$ cargo run
["target/debug/greprs"]

$ cargo run needle haystack
...snip...
["target/debug/greprs", "needle", "haystack"]
```

<!--Below --- This initially confused me, do you mean that the argument at
index 0 is taken up by the name of the binary, so we start arguments at 1 when
setting them? It seems like it's something like that, reading on, and I've
edited as such, can you check? -->

You may notice that the first argument, 0, is taken up by the name of the
binary. We'll have to account for that when setting the other arguments and
begin them at `1`. The reasons for this are out of the scope of this chapter,
but it's something to remember.

### Setting the Arguments

Now that we have a way to access arguments, let's set the two we need for
`grep` and save them in variables as shown in Listing 12-2:

<!-- By 'find the ones we care about' did you mean set particular arguments so
the user knows what to enter? I'm a little confused about what we are doing,
I've tried to clarify above -->

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

Listing 12-2: Create variables to hold the search argument and filename argument

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Remember, the program's name takes up the first argument at `args[0]`, so we
start at `[1]`. The first argument `greprs` will take is the string we're
searching for, so we put a reference to the first argument in the variable
`search`. The second argument will be the filename, so we put a reference to
the second argument in the variable `filename`. We add descriptions to print to
the screen to let the user know what the program is doing. Let's try running
this program again with the arguments `test` and `sample.txt`:

```text
$ cargo run test sample.txt
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe test sample.txt`
Searching for test
In file sample.txt
```

<!-- What here indicates that it's working, can you point that out? -->

Great, it's working! Later we'll add some error handling to deal with
situations such as when the user provides no argmuents, but so now we'll add
file reading capabilities.

<!-- There's one problem, though, exposed when we try running it with no
arguments:

```text
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1', ../src/libcollections\vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

We get a panic error, as we might expect, because our vector only has one
element, the program's name, but we tried to access the second element.
However, while this error message is _accurate_, it's not meaningful to users
of our program at all. Not to worry, we'll fix this situation a little later in
the chapter.-->

<!-- This might be more distracting than useful at this point, ok to cut this
bit? I added a smaller summary line above -->

## Reading a File

Next we need to give our program the ability to open the file we specify in
order to search it. First, we need a sample file to test it with---the best
sample to use in this case is one with a small amount of text, over multiple
lines, and with some repeition. We've given you a sample poem to use in Listing
12-X. Create a file called `poem.txt` at the root level of your project, and
fill it up with some Emily Dickinson.

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
Listing 12-X

<!-- Public domain Emily Dickinson poem. This will work best with something
short, but that has multiple lines and some repetition. We could search through
code; that gets a bit meta and possibly confusing... Changes to this are most
welcome. /Carol -->
<!-- :D I like it! I'm all for keeping -->

With that in place, edit *src/main.rs* and add code to open the file as shown
in Listing 12-3:

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

Listing 12-3: Reading the contents of the file specified by the second argument

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

First, we add some more `use` statements to bring in relevant parts of the
standard library: we need `std::fs::File` for dealing with files, and
`std::io::prelude::*` contains various traits that are useful when doing I/O,
including file I/O. In the same way that Rust has a general prelude that brings
certain things into scope automatically, the `std::io` module has its own
prelude of common things you'll need when working with I/O. Unlike the default
prelude, we must explicitly `use` the prelude in `std::io`.

In `main`, we've added three things: first, we set a mutable handle to the file
and add functionality to open it using the `File::open` function, and give it
the filename argument so that it will open whatever we pass as the second
argument. Second, we create a variable to hold the data the program reads in
called `contents`, which we make a mutable, empty `String`. This will hold the
content of the file given as the second argument. Then we call `read_to_string`
on our file handle with our `contents` string as the argument.

Finally, we print out the contents of `contents` so we can check our program is
working so far.

Let's try running this code with any string passed for the first argument
(since we haven't implemented the searching part yet) and our *poem.txt* file
as the second argument:

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

Great! Our code read in and printed out the content of the file. There are
still a few flaws:

<!-- what kind of flaws, so far? -->

While our program is still small, these flaws aren't a big problem, but as our
program grows, it will be harder to fix them cleanly. It's good practice to
begin refactoring early on when developing a program, as it's much easier to do
with only this small amount of code, so we'll do that now.

## Refactoring to Improve Error Handling and Modularity

There are four problems that we'd like to fix to improve our program, and they
all have to do with potential errors and the way the program is structured.

The first problem is that we've used `expect` to print out an error message if
opening the file fails, but our blanket error message only says "file not
found". There are a number of ways that opening a file can fail besides a
missing file, for example, the file might exist, but we might not have
permission to open it: right now, we print a blanket error message that may
give the user the wrong advice!

Secondly, we use `expect` repeatedly to deal with different errors, which
result in index errors when no arguments are provided in the command line.
While it _works_, it's a bit unprincipled, and we're doing it all throughout
our program. It would be much more user friendly to put our error handling in
one spot.

<!-- I'm not sure I understand what we mean here, can you give a line or two of
recap on the indexing issue? Why is it unprincipled? -->

Thirdly, our `main` function now performs two tasks: it parses arguments, and
opens up files. For such a small function, this isn't a huge problem. However,
if we keep growing our program inside of `main` alone, the number of separate
tasks the `main` function handles will grow, and as a function gains
responsibilities it gets harder to reason about, harder to test, and harder to
change without breaking one of its parts. It's better to separate out
functionality.

This also ties into our fourth problem: while `search` and `filename` are
configuration variables to our program, variables like `f` and `contents` are
used to perform our program's logic. The longer `main` gets, the more variables
we're going to need to bring into scope; the more variables we have in scope,
the harder it is to keep track of the purpose of each. It's better to group the
configuration variables into one structure to make their purpose clear.

Let's address these problems by restructuring our project.

<!--Looks like we perform the tasks in a different order to how they're listed
--- minor issue, but I think it'd make more sense to do them in order, I'm not
sure if it would be better to rearrange the paragraphs above or the following
sections. Probably the ones above --- could you do that to match? -->

### Separation of Concerns for Binary Projects

Organizational problems are common to many similar kinds of projects, so the
Rust community has developed a kind of guideline pattern for organizing the
separate concerns of a program. The pattern looks like this:

1. Split your program into both a *main.rs* and a *lib.rs*.
2. Place your command line parsing logic into *main.rs*.
3. Place your program's logic into *lib.rs*.
4. The job of the `main` function should be to:
   * parse arguments
   * set up any other configuration
   * call a `run` function in *lib.rs*
   * if `run` returns an error, handle that error

<!-- it actually seems pretty simple to me! Okay to sell it as that? -->

The pattern is simple! It's all about separating concerns: *main.rs* handles
running the program, and *lib.rs* handles all of the logic of the task at hand.
Let's re-work our program into this pattern.

<!--Since main is already handling the parsing of arguments, why do we need to
add a new function for it, can you say how that improves things? -->

#### Extracting the Argument Parser

First, we'll extract the functionality for parsing arguments. Listing 12-4
shows the new start of `main` that calls a new function `parse_config`, which
we're still going to define in *src/main.rs*:

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

Listing 12-4: Extract a `parse_config` function from `main`

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

<!-- Can you talk this through, point out how the code is different to our
original argument parsing code?-->

This may seem like overkill for our small program, but we're working in small
steps. After making this change, run the program again to verify that the
argument parsing still works. It's good to check your progress often, as that
will help you identify the cause of problems when they occur.

#### Grouping Configuration Values

We can improve on this function. At the moment, we return a tuple, but then
immediately break that tuple up into individual parts again, which seems a
clear indicator that we can make it more efficient.

Another indicator that there's room for improvement is the `config` part of
`parse_config`, which implies that the two values we return should be bound
together, since they're both part of one configuration value.

<!-- above -- I'm not sure why this is a problem --- because they aren't
currently bound together? And why does it imply that -->

> Note: some people call this anti-pattern of using primitive values when a
> complex type would be more appropriate *primitive obsession*.

<!-- Ah, I see, so the problems here stem from using simple types to do tasks
inefficiently, when a more complex task could handle it in ways that improve...
behavior? Readability? Can you say as much? -->

We'll introduce a struct to hold all of our configuration so that the
information is held in one place. Listing 12-5 shows the addition of the
`Config` struct definition, the refactoring of `parse_config`, as well as a few
updates to `main` we'll talk through:

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

Listing 12-5: Refactoring `parse_config` to return an instance of a `Config`
struct

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

The signature of `parse_config` now indicates that it returns a `Config` value.
In the body of `parse_config`, where we used to return string slices as
references to `String` values in `args`, we've now defined `Config` to contain
owned `String` values. This is because the `args` variable in `main` owns the
`String` values and is only letting the `parse_config` function borrow them,
meaning it would violate Rust's borrowing rules to let `Config take ownership
of the `String` values, since the argument to `parse_config` is a slice of
`String` values.

<!-- This paragraph above somewhat bamboozled me, you can see I've attempted a
rewrite, rearranging the sentences so that the subject is clear at any one
point, but anything you could add to clarify or perhaps split the sentences up
to simplify might help -->

There are a number of different ways we could manage the `String` data, and the
easiest, though somewhat inefficient, route is to call the `clone` method on
the string slices. This will make a full copy of the string's data for the
`Config` instance to own, which does take more time and memory than storing a
reference to the string data, but also makes our code very straightforward, so
in this simple circumstance is a worthwhile trade-off.

We've updated `main` so that it places the instance of `Config` that
`parse_config` returns into a variable named `config`, and updated the code
that previously used the separate `search` and `filename` variables so that is
now uses the fields on the `Config` struct instead.

<!-- can you give a quick summary of what this has improved here, what the
effect on the final program will be? "The configuration values are now all held
in ...."?-->

<!-- PROD: START BOX -->

> #### The Tradeoffs of Using `clone`
>
> There's a tendency amongst many Rustaceans to avoid using `clone` to fix
> ownership problems because of its runtime cost. In Chapter XX on iterators,
> you'll learn how to use more efficient methods in this kind of situation, but
> for now, it's okay to copy a few strings to keep making progress since we'll
> only make these copies once, and our filename and search string are both very
> small. It's better to have a working program that's a bit inefficient than
> try to hyper-optimize code on your first pass. As you get more experienced
> with Rust, it'll be easier to go straight to the desirable method, but for
> now it's perfectly acceptable to call `clone`.

<!-- PROD: END BOX -->

### Creating a Constructor for `Config`

<!-- Can you lay out what we intend to do in this section? I wasn't sure even
at the end what we did and why --- why did we create it as parse_config to then
change it to new? -->

The purpose of the `parse_config function is to create a `Config` instance, but
as we know there are other ways to create new instanes, such as the `new`
function, like `String::new` to create a new string. We'll transform our
`parse_config` into a `new` function associated with our `Config` struct so
that :

<!--Can you say why we do this here? -->

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

Listing 12-6: Changing `parse_config` into `Config::new`

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've changed the name of `parse_config` to `new` and moved it within an `impl`
block. We've also updated the callsite in `main`. Try compiling this again to
make sure it works.

<!-- why do we move it inside an impl block, and what what updates do we make
to the callsite? (I also don't think we've spoken about the callsite in the
chapter at all so far, I'm not sure what it is) -->

### Fixing the Error Handling

Now we'll do the last refactoring of this method, and fix our error handling:
remember that an attempt as accessing the vector with indices 1 and 2 causes
the program to panic if it contains fewer than 3 items, and currently it gives
a bad error message. We'll fix that now.

#### Improving the Error Message

In Listing 12-7 we change it so the program checks that the slice is long
enough before accessing those locations, and if it isn't the panic returns a
better error message:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// ...snip...
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // ...snip...
```

Listing 12-7: Adding a check for the number of arguments

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

<!--Can you talk about the code a little -->

With these extra few lines of code in `new`, let's try running our program
without any arguments and see what error occurs:

```bash
$ cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target\debug\greprs.exe`
thread 'main' panicked at 'not enough arguments', src\main.rs:29
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This output is better, we now have a reasonable error message. However, we also
have a bunch of extra information we don't want to give to our users.

<!-- Below -- how does using new fix this, can you lay that our up front? -->

#### Returning a Result from the Constructor

We can resolve this by changing the type signature of `new`. Right now, it
returns only a `Config`, so there's no way to indicate that an error happened
while creating our `Config`. Instead, we can return a `Result`, as shown in
Listing 12-8:

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

Listing 12-8: Return a `Result` from `Config::new`

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

<!-- what does returning a Result rather than a Config do? -->

Our `new` function now returns a `Result`, with a `Config` instance in the
success case and a `&'static str` in the error case. Recall from "The Static
Lifetime" section in Chapter 10 that `&'static str` is the type for string
literals, which is our error message type for now.

We've made two changes in the body of the `new` function: instead of calling
`panic!` when the user doesn't pass enough arguments, we now return an `Err`
value, and we've wrapped the `Config` return value in an `Ok`. These changes
make the function conform to its new type signature.

By having `Config::new` return an `Err` value, it allows the `main` functionto
handle the `Result` value returned from the `new` function and exit the process
more cleanly.

### Calling `Config::new` and Handling Errors

<!-- why, what changes are required, and what for? -->

Now we need to make some changes to `main` as shown in Listing 12-9:

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

Listing 12-9: Exiting with an error code if creating a new `Config` fails

<!-- What is `process` used for here? -->
<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've added a new `use` line to import `process` from the standard library.
<!-- In the `main` function itself, we'll handle the `Result` value returned
from the `new` function and exit the process in a cleaner way if `Config::new`
returns an `Err` value.-->
<!-- I moved this line above to the previous section, it seems to at least
partially answer some of my earlier confusions, though I'm not following this
as well as I'd like so not sure if I have this right, can you confirm either
way whether that move makes sense? -->

In this listing we're using a method we haven't covered before:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us some custom, non-`panic!` error handling. If
the `Result` is an `Ok` value, this method's behavior is similar to `unwrap`:
it returns the inner value `Ok` is wrapping. However, if the value is an `Err`
value, this method calls a *closure*, which is an anonymous function we define
and pass as an argument to `unwrap_or_else`. We'll be covering closures in more
detail in Chapter XX; what you need to know in this case is that
`unwrap_or_else` will pass the inner value of the `Err` to our closure in the
argument `err` that appears between the vertical pipes.

<!--Can you give a high-level idea of what the closure does with it? -->

The error handling here is only two lines: we print out the error, then call
`std::process::exit`, which will execute the program immediately and return the
number that was passed as a return code. By convention, a zero indicates
success and any other value means failure. This isn't dissimilar to the
`panic!`-based handling we used in Listing 12-7, with the exception that we no
longer get all the extra output. Let's try it:

```text
$ cargo run
   Compiling greprs v0.1.0 (file:///projects/greprs)
    Finished debug [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target\debug\greprs.exe`
Problem parsing arguments: not enough arguments
```

Great! This output is much friendlier for our users.

### Handling Errors from the `run` Function

Now we're done refactoring our configuration parsing; let's improve our
program's logic. We'll extract a function named `run` that we'll call from
`main` and add to to our code, as shown in Listing 12-10. The `run` function
contains the code that was in `main`:

<!-- it contains ALL the function from main? Can you say why we're doing this,
hw this improves it? What is the run function doing? I'm afraid I feel a bit in
the dark here-->

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

Listing 12-10: Extracting the `run` functionality for the rest of the program
logic

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

The `run` function now holds the lines that were previously in `main`, and
takes a `Config` as an argument.

<!--- Below, a separate function for what? For running the logic? -->

#### Heading?

With this function separated, we'll improve the error handling like we did with
`Config::new` in Listing 12-8: rather than allowing the program to panic, it
will return a `Result<T, E>` when something goes wrong. In Listing 12-11 we add
a `use` statement to bring the `std::error::Error` struct into scope and we
change the `run` function to return a `Result`:

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

Listing 12-11: Changing the `run` function to return `Result`

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've made three big changes here. First, we changed the return type of the
`run` function to `Result<(), Box<Error>>`. Our function previously returned
the unit type, `()`, and we keep that as the value returned in the `Ok` case.

<!-- is just the `Box` bit the trait object, or the whole `Box<Error>`
syntax?-->

For our error type we use the *trait object* `Box<Error>`---we'll be covering
trait objects in Chapter XX, but for now just know that `Box<Error>` here means
the function will return a type that implements the `Error` trait, but that we
don't have to specify what particular type the return value will be. This gives
us flexibility to return error values that may be of different types in
different error cases. We'll be go into detail about `Box` in Chapter YY.

The second change we make is to remove the calls to `expect` in favor of `?`,
like we talked about in Chapter 9. Rather than `panic!` on an error, this will
return the error value from the current function for the caller to handle.

Thirdly, this function now returns an `Ok` value in the success case. We've
declared the `run` function's success type as `()` in the signature, which
means we need to wrap the unit type value in the `Ok` value. This `Ok(())`
syntax may look a bit strange at first, but using `()` like is the idiomatic
way to indicate that we're calling `run` for its side effects only; it doesn't
return anything we actually need.

When you run this, it will compile, but with a warning:

```text
warning: unused result which must be used, #[warn(unused_must_use)] on by default
  --> src\main.rs:39:5
   |
39 |     run(config);
   |     ^^^^^^^^^^^^
```

Rust is telling us that our code ignores our `Result`, which might be an error
value.

<!-- I'm not sure what you mean above, can you expand? -->

Let's rectify that now.

#### Heading

We'll use a similar technique to the way we handled failure with `Config::new`
in Listing 12-9, but with a slight difference:

<!--- what are we fixing here, I'm still not clear? -->

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

We use `if let` to check whether `run` returns an `Err` value, rather than
`unwrap_or_else`, and call `process::exit(1)` if it does.

<!-- Why is `if let` better suited, what's it actually doing here? Simply
closing the program in the case of an error? -->

There are a couple of reasons we use a different method here. With
`Config::new`, we cared about two things: detecting errors and getting a
`Config` there are no errors.

In this case, though, because `run` returns a `()` in the success case, we only
care about detecting an error, so we don't need `unwrap_or_else` to return its
value as it would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases though: we print out an error and exit.

### Split Code into a Library Crate

This is looking pretty good so far! Now we need to split the *src/main.rs* file
up and put some code into *src/lib.rs*

<!-- can you quickly remind them why we want to do this? -->

Move the `run` function from *src/main.rs* to a new file, *src/lib.rs*. You'll
also need to move the relevant `use` statements and the definition of `Config`
and its `new` method in the XXX file as well. Your *src/lib.rs* should now look
like Listing 12-12:

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

Listing 12-12: Moving `Config` and `run` into *src/lib.rs*

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We've made liberal use of `pub` here, on `Config`, its fields and its `new`
method, and on the `run` function.

<!-- Why, can you make it clear why we wanted to do this? What changes has this
made? -->

#### Calling the Library Code

Now we need to call the code we moved to *src/lib.rs* in our *src/main.rs*, by
using `extern crate greprs`. Then we'll add a `use greprs::Config` line to
bring `Config` into scope, and prefix the `run` function with our crate name as
shown in Listing 12-13:

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

Listing 12-13: Bringing the `greprs` crate into the scope of *src/main.rs*

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

With that, all the functionality should be connected and should work. Give it a
few `cargo run`s and make sure you haven't broken anything.

<!-- any tips for if they do find something is broken, main places to check? Or
just "diff your file against the XXX file in the book's resources to check
where it went wrong"? -->

Whew! That was a lot of work, but we've set ourselves up for success in the
future. Now it's much easier to handle errors, and we've made our code more
modular. Almost all of our work will be done in *src/lib.rs* from here on out.

Let's take advantage of this newfound modularity by doing something that would
have been hard with our old code, but is easy with our new code: write some
tests!

## Testing the Library's Functionality

Now we've extracted the logic into *src/lib.rs* and left all the argument
parsing and error handling in *src/main.rs*, it's much easier for us to write
tests for the core functionality of our code.

Now we can call our code directly with various arguments and check return
values without having to call our binary from the command line.

<!-- So is `grep` different from `greprs`? It seems like we're building the
function here, how is this different? -->

For our test we'll write a function named `grep` that takes our search term and
the text to search and produces a list of search results.

### Testing to Fail

<!-- or some, much more suitable, heading! -->

First, since we don't really need them any more, let's remove `println!` from
`run` from both lib.rs and *src/main.rs*. Then from lib.rs we'll call the new
`grep` function with the options we've collected.

<!--- How come we're calling grep, when we haven't written it yet? Also, which
options are we talking aboout hree, the search term and search text?-->

We'll add a placeholder implementation of the `grep` function for now, and add
a test that specifies the behavior we'd like the `grep` function to have. The
test will fail with our placeholder implementation, of course, but with this we
can make sure the code compiles and that we get the failure message we expect.
Listing 12-14 shows these modifications:

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

Listing 12-14: Creating a function to hold our test logic and a failing test
for that function

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Notice that we need an explicit lifetime `'a` declared in the signature of
`grep` and used with the `contents` argument and the return value. Remember, we
use lifetime parameters to specify which argument lifetime is connected to the
lifetime of the return value. In this case, we're indicating that the returned
vector should contain string slices that reference slices of the argument
`contents` (rather than the argument `search`).

In other words, we're telling Rust that the data returned by the `grep`
function will live as long as the data passed into the `grep` function in the
`contents` argument. This is important! The data referenced *by* a slice needs
to be valid in order for the reference to be valid; if the compiler assumed we
were making string slices of `search` rather than `contents`, it would do its
safety checking incorrectly. If we tried to compile this function without
lifetimes, we would get this error:

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

Rust can't possibly know which of the two arguments we need, so we need to tell
it. Because `contents` is the argument that contains all of our text and we
want to return the parts of that text that match, we know `contents` is the
argument that should be connected to the return value using the lifetime syntax.

Other programming languages don't require you to connect arguments to return
values in the signature, so this may feel strange at first, but will get easier
over time. You may want to compare this example with the Lifetime Syntax
section in Chapter 10 to help you get your head around it.

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

Great, our test fails, exactly as we expected. Let's get the test to pass!

### Testing to Succeed

<!-- or a heading of your creation that is more suitable!-->

Currently, our test is failing because we always return an empty vector. To fix
that and implement `grep`, our program needs to follow these steps:

1. Iterate through each line of the contents.
2. Check if the line contains our search string.
   * If it does, add it to the list of values we're returning.
   * If it doesn't, do nothing.
3. Return the list of results that match.

Let's take each step at a time, starting with iterating through lines.

#### Iterating Through Lines with lines

Rust has a helpful method to handle line-by-line iteration of strings,
conveniently named `lines`, that works like this:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

Listing 12-X: Iterating through lines

<!-- Will add wingdings in libreoffice /Carol -->

We use a `for` loop along with the `lines` method to get each line in turn.

<!-- so what does `lines` do on its own, if we need to use it in a for loop to
work? -->

#### Searching the Line for a String

Next, we'll add functionality to check if the current line contains the search
string. Luckily, strings have another helpful method named `contains` that does
this for us! Add the `contains` method to our program so far like this:

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

Listing 12-X: Adding functionality to search for a string

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

#### Storing Matching Lines

Finally, we need a way to store the lines that do contain our search string.
For that, we can make a mutable vector before the `for` loop and call the
`push` method to store a `line` in the vector. After the `for` loop, we return
the vector:

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

Listing 12-X: Storing the results

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

We push the lines with the matching content to the `line` vector. Let's run
this and give it a try:

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

Our reults passed and nothing failes, great, it works!

<!-- Now that our test is passing, we could consider opportunities
for refactoring the implementation of `grep` and be certain we maintain the
same functionality while we do so. This code isn't bad, but it isn't taking
advantage of some useful features of iterators. We'll be coming back to this
example in Chapter 13 where we'll explore iterators in detail and see how to
improve it. -->

<!-- If we aren't going into this here, maybe just keep it focused, there's a
lot going on here as is -->

#### Returning Matching Lines

With the functionality working, now we just need to print out the results.
We'll do that by adding a `for` loop that prints each line returned from the
`grep` function:

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

<!-- do you want to add a couple of lines about the code here? -->

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

<!-- Maybe this second one should be a more specific example, a whole word
match, like "nobody"? These results are not immediately clear -->

Excellent! We've built our own version of a classic tool, and learned a lot
about how to structure applications. We've also learned a bit about file input
and output, lifetimes, testing, and command line parsing.

<!-- If we'er going into environment variable more thoroughly in a later
chapter, I might suggest we cut this next bit --- I'm not sure it will be
useful to readers before they've covered environement variables, and this
chapter is already very full and quite intricate. What do you think? That would
simplify the content, and cut out a fair few pages of a long chapter. -->

## Working with Environment Variables

We'll improve our tool with one extra feature: an option for case insensitive
searching. We could make this a command line option and require they enter it
each time they want it to apply, but instead we're going to use an environment
variable. This allows our users to set the environment variable once and have
all their searches case insensitive in that terminal session.

### Implement and Test a Case-Insensitive `grep` Function

First, let's add a new function that we will call when the environment variable
is on.

<!-- You mean, to turn the environment variable on? I'm not sure what we're
doing here-->

Let's start by adding a new test and re-naming our existing one:

<!-- Can you say why? -->

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

We're going to define a new function named `grep_case_insensitive`, shown in
Listing 12-X. It will be almost the same as the `grep` function, but with some
minor changes. We'll lowercase the `search` function and `line` so that,
whatever the case of the input arguments, they will be the same case when
matched.

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

Listing 12-X: The grep_case_insensitive_ function

<!-- Will add ghosting and wingdings in libreoffice /Carol -->
<!-- why do we lowercase the search string? and why does it need to be a string
rather than a slice? -->

First, we lowercase the `search` string, and store it in a shadowed variable
with the same name. Note that `search` is now a `String` rather than a string
slice which means that, because it contains takes a string slice, we need to
reference `search` by adding an ampersand when we pass `search` to `contains`.

Second, we add a call to `to_lowercase` each `line` before we check if it
contains `search`. Now we've converted both `line` and `search` to all
lowercase, so we'll find matches no matter what case the user input in the file
and the command line arguments, respectively.

Let's see if this passes the tests:

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

Great! Now, let's actually use the new `grep_case_insensitive` function. First,
we need to add a configuration option for it to the `Config` struct:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

<!-- Will add ghosting in libreoffice /Carol -->

We add the case_sensitive function that takes a bool. Then we need our `run`
function to be able to check for the case_sensitive function, and for it decide
which function to call based on the value of the `case_sensitive` function:

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
the `env` module from the standard library into our project, we add a `use`
line at the top of *src/lib.rs*:

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

Here, we call `env::vars`, which returns an iterator of environment variables
in the same way `env::args` returns an iterator of command line arguments.
Instead of using `collect` to create a vector of all of the environment
variables, we're using a `for` loop. `env::vars` returns two tuples: the name
of the environment variable and its value.

<!-- why do we use a loop rather than collect? what benefit does it have, and
why use it here but not previously?-->

In this case we don't need to know the value of the XXX, only whether the
variable is set, so we use the `_` placeholder instead of a name to let Rust
know that it shouldn't warn us about an unused variable.

Finally, we have a `case_sensitive` variable, which is set to true by default.
If a `CASE_INSENSITIVE` environment variable is found, the `case_sensitive`
variable is set to false instead. Then we return the value as part of the
`Config`.

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

Excellent! Our `greprs` program can now do case insensitive searching,
controlled by an environment variable. Now you know how to manage options set
using either command line arguments or environment variables!

Some programs allow both arguments _and_ environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case
insensitivity through a command line argument as well as through the
environement variabble, and decide which should take precedence the program is
run with contradictory values.

The `std::env` module contains many more useful features for dealing with
environment variables; check out its documentation to see what's available.

<!-- And this section, too, might be too much. We might be wearing the reader
out at this point -->

## Write to `stderr` Instead of `stdout`

Right now, we're writing all of our output to the terminal with `println!`.
Most terminals provide two kinds of output: "standard out" for general
information, and "standard error" for error messages. This distinction makes it
easier to direct text; for example we could print error messages to the
terminal, but write other output to a file.

<!-- are we saying that using println makes it only capable of this kind of
output? I'm not sure what connection the two kinds of output has with that
first line about println -->

Let's send some output to a file rather than to the terminal by redirecting the
output using `>` and specifying the file. We run this on the command line with
our program, without any arguments, and if it causes an error:

```text
$ cargo run > output.txt
```

<!-- why do we get an error here? Was that intentional? Does that mean it can't
print stdout to a file? -->

The `>` syntax tells the shell to write the contents of standard out to
*output.txt* instead of the screen. If we open *output.txt* after running we'll
see our error message:

```text
Application error: No search string or filename found
```

<!-- I don't understand why we send this output to a file to then just say we
want it to the screen, won't it do that by default? And what has this got to do
with our use of println? I'm finding the motives here hard to follow -->

It's much more useful for error messaages like this to be printed to the screen
instead, and only have the output from a successful run end up in the file.
Let's change how error messages are printed as shown in Listing 12-15:

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

Listing 12-15: Writing error messages to `stderr` instead of `stdout`

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Rust does not have a convenient function like `println!` for writing to
standard error. Instead, we use the `writeln!` macro, which is like `println!`
but takes an extra argument. The first thing we pass to it is what to write to.
We can acquire a handle to standard error through the `std::io::stderr`
function. We give a mutable reference to `stderr` to `writeln!`; we need it to
be mutable so we can write to it! The second and third arguments to `writeln!`
are like the first and second arguments to `println!`: a format string and any
variables we're interpolating.

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

We'll see no output to our terminal, but `output.txt` will contain our results:

<span class="filename">Filename: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

## Summary

In this chapter, we've recapped on some of the major concepts so far and
covered how to do common I/O operations in a Rust context. By using command
line arguments, files, environment variables, and the `stderr` tool, you're now
prepared to write command line applications. By using the concepts from
previous chapters, your code will be well-organized, be able to store data
effectively in the appropriate data structures, handle errors nicely, and be
well tested.

Next, let's explore some functional-language influenced Rust features: closures
and iterators.
