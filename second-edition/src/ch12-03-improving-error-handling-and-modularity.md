## Refactoring to Improve Modularity and Error Handling

There are four problems that we'd like to fix to improve our program, and they
have to do with the way the program is structured and how it's handling
potential errors.

First, our `main` function now performs two tasks: it parses arguments and
opens up files. For such a small function, this isn't a huge problem. However,
if we keep growing our program inside of `main`, the number of separate tasks
the `main` function handles will grow. As a function gains responsibilities, it
gets harder to reason about, harder to test, and harder to change without
breaking one of its parts. It's better to separate out functionality so that
each function is responsible for one task.

This also ties into our second problem: while `query` and `filename` are
configuration variables to our program, variables like `f` and `contents` are
used to perform our program's logic. The longer `main` gets, the more variables
we're going to need to bring into scope; the more variables we have in scope,
the harder it is to keep track of the purpose of each. It's better to group the
configuration variables into one structure to make their purpose clear.

The third problem is that we've used `expect` to print out an error message if
opening the file fails, but the error message only says `file not found`. There
are a number of ways that opening a file can fail besides a missing file: for
example, the file might exist, but we might not have permission to open it.
Right now, if we're in that situation, we'd print the `file not found` error
message that would give the user the wrong advice!

Fourth, we use `expect` repeatedly to deal with different errors, and if the
user runs our programs without specifying enough arguments, they'll get an
"index out of bounds" error from Rust that doesn't clearly explain the problem.
It would be better if all our error handling code was in one place so that
future maintainers only have one place to consult in the code if the error
handling logic needs to change. Having all the error handling code in one place
will also help us to ensure that we're printing messages that will be
meaningful to our end users.

Let's address these problems by refactoring our project.

### Separation of Concerns for Binary Projects

The organizational problem of having the `main` function responsible for
multiple tasks is common to many binary projects, so the Rust community has
developed a kind of guideline process for splitting up the separate concerns of
a binary program when `main` starts getting large. The process has the
following steps:

1. Split your program into both a *main.rs* and a *lib.rs* and move your
   program's logic into *lib.rs*.
2. While your command line parsing logic is small, it can remain in *main.rs*.
3. When the command line parsing logic starts getting complicated, extract it
   from *main.rs* into *lib.rs* as well.
4. The responsibilities that remain in the `main` function after this process
   should be:
   * Calling the command line parsing logic with the argument values
   * Setting up any other configuration
   * Calling a `run` function in *lib.rs*
   * If `run` returns an error, handling that error

This pattern is all about separating concerns: *main.rs* handles running the
program, and *lib.rs* handles all of the logic of the task at hand. Because we
can't test the `main` function directly, this structure lets us test all of our
program's logic by moving it into functions in *lib.rs*. The only code that
remains in *main.rs* will be small enough to verify its correctness by reading
it. Let's re-work our program by following this process.

### Extracting the Argument Parser

First, we'll extract the functionality for parsing arguments. Listing 12-5
shows the new start of `main` that calls a new function `parse_config`, which
we're still going to define in *src/main.rs* for the moment:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // ...snip...
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

<span class="caption">Listing 12-5: Extract a `parse_config` function from
`main`</span>

We're still collecting the command line arguments into a vector, but instead of
assigning the argument value at index 1 to the variable `query` and the
argument value at index 2 to the variable `filename` within the `main`
function, we pass the whole vector to the `parse_config` function. The
`parse_config` function then holds the logic that knows which argument goes in
which variable, and passes the values back to `main`. We still create the
`query` and `filename` variables in `main`, but `main` no longer has the
responsibility of knowing how the command line arguments and variables
correspond.

This may seem like overkill for our small program, but we're refactoring in
small, incremental steps. After making this change, run the program again to
verify that the argument parsing still works. It's good to check your progress
often, as that will help you identify the cause of problems when they occur.

#### Grouping Configuration Values

We can take another small step to improve this function further. At the moment,
we're returning a tuple, but then we immediately break that tuple up into
individual parts again. This is a sign that perhaps we don't have the right
abstraction yet.

Another indicator that there's room for improvement is the `config` part of
`parse_config`, which implies that the two values we return are related and are
both part of one configuration value. We're not currently conveying this
meaning in the structure of the data other than grouping the two values into a
tuple: we could put the two values into one struct and give each of the struct
fields a meaningful name. This will make it easier for future maintainers of
this code to understand how the different values relate to each other and what
their purpose is.

> Note: some people call this anti-pattern of using primitive values when a
> complex type would be more appropriate *primitive obsession*.

Listing 12-6 shows the addition of a struct named `Config` defined to have
fields named `query` and `filename`. We've also changed the `parse_config`
function to return an instance of the `Config` struct, and updated `main` to
use the struct fields rather than having separate variables:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
# use std::env;
# use std::fs::File;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // ...snip...
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

Listing 12-6: Refactoring `parse_config` to return an instance of a `Config`
struct

The signature of `parse_config` now indicates that it returns a `Config` value.
In the body of `parse_config`, where we used to return string slices that
reference `String` values in `args`, we've now chosen to define `Config` to
contain owned `String` values. The `args` variable in `main` is the owner of
the argument values and is only letting the `parse_config` function borrow
them, though, which means we'd violate Rust's borrowing rules if `Config` tried
to take ownership of the values in `args`.

There are a number of different ways we could manage the `String` data, and the
easiest, though somewhat inefficient, route is to call the `clone` method on
the values. This will make a full copy of the data for the `Config` instance to
own, which does take more time and memory than storing a reference to the
string data. However, cloning the data also makes our code very straightforward
since we don't have to manage the lifetimes of the references, so in this
circumstance giving up a little performance to gain simplicity is a worthwhile
trade-off.

> #### The Tradeoffs of Using `clone`
>
> There's a tendency among many Rustaceans to avoid using `clone` to fix
> ownership problems because of its runtime cost. In Chapter 13 on iterators,
> you'll learn how to use more efficient methods in this kind of situation, but
> for now, it's okay to copy a few strings to keep making progress since we'll
> only make these copies once, and our filename and query string are both very
> small. It's better to have a working program that's a bit inefficient than
> try to hyper-optimize code on your first pass. As you get more experienced
> with Rust, it'll be easier to go straight to the desirable method, but for
> now it's perfectly acceptable to call `clone`.

We've updated `main` so that it places the instance of `Config` that
`parse_config` returns into a variable named `config`, and updated the code
that previously used the separate `query` and `filename` variables so that is
now uses the fields on the `Config` struct instead.

Our code now more clearly conveys our intent that `query` and `filename` are
related and their purpose is to configure how the program will work. Any code
that uses these values knows to find them in the `config` instance in the
fields named for their purpose.

#### Creating a Constructor for `Config`

So far, we've extracted the logic responsible for parsing the command line
arguments from `main` into the `parse_config` function, which helped us to see
that the `query` and `filename` values were related and that relationship
should be conveyed in our code. We then added a `Config` struct to name the
related purpose of `query` and `filename`, and to be able to return the values'
names as struct field names from the `parse_config` function.

So now that the purpose of the `parse_config` function is to create a `Config`
instance, we can change `parse_config` from being a plain function into a
function named `new` that is associated with the `Config` struct. Making this
change will make our code more idiomatic: we can create instances of types in
the standard library like `String` by calling `String::new`, and by changing
`parse_config` to be a `new` function associated with `Config`, we'll be able
to create instances of `Config` by calling `Config::new`. Listing 12-7 shows
the changes we'll need to make:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
# use std::env;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // ...snip...
}

# struct Config {
#     query: String,
#     filename: String,
# }
#
// ...snip...

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

<span class="caption">Listing 12-7: Changing `parse_config` into
`Config::new`</span>

We've updated `main` where we were calling `parse_config` to instead call
`Config::new`. We've changed the name of `parse_config` to `new` and moved it
within an `impl` block, which makes the `new` function associated with
`Config`. Try compiling this again to make sure it works.

### Fixing the Error Handling

Now we'll work on fixing our error handling. Recall that we mentioned
attempting to access the values in the `args` vector at index 1 or index 2 will
cause the program to panic if the vector contains fewer than 3 items. Try
running the program without any arguments; it will look like this:

```text
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1',  /stable-dist-rustc/build/src/libcollections/vec.rs:1307
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

`index out of bounds: the len is 1 but the index is 1` is an error message that
is intended for programmers, and won't really help our end users understand
what happened and what they should do instead. Let's fix that now.

#### Improving the Error Message

In Listing 12-8, we're adding a check in the `new` function to check that the
slice is long enough before accessing index 1 and 2. If the slice isn't long
enough, we panic with a better error message than the `index out of bounds`
message:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
// ...snip...
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // ...snip...
```

<span class="caption">Listing 12-8: Adding a check for the number of
arguments</span>

This is similar to the `Guess::new` function we wrote in Listing 9-8, where we
called `panic!` if the `value` argument was out of the range of valid values.
Instead of checking for a range of values, we're checking that the length of
`args` is at least 3, and the rest of the function can operate under the
assumption that this condition has been met. If `args` has fewer than 3 items,
this condition will be true, and we call the `panic!` macro to end the program
immediately.

With these extra few lines of code in `new`, let's try running our program
without any arguments again and see what the error looks like now:

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:29
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This output is better, we now have a reasonable error message. However, we also
have a bunch of extra information we don't want to give to our users. So
perhaps using the technique we used in Listing 9-8 isn't the best to use here;
a call to `panic!` is more appropriate for a programming problem rather than a
usage problem anyway, as we discussed in Chapter 9. Instead, we can use the
other technique we learned about in that chapter: returning a `Result` that can
indicate either success or an error.

#### Returning a `Result` from `new` Instead of Calling `panic!`

We can choose to instead return a `Result` value that will contain a `Config`
instance in the successful case, and will describe the problem in the error
case. When `Config::new` is communicating to `main`, we can use Rust's way of
signaling that there was a problem using the `Result` type. Then we can change
`main` to convert an `Err` variant into a nicer error for our users, without
the surrounding text about `thread 'main'` and `RUST_BACKTRACE` that a call to
`panic!` causes.

Listing 12-9 shows the changes to the return value of `Config::new` and the
body of the function needed to return a `Result`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

<span class="caption">Listing 12-9: Return a `Result` from `Config::new`</span>

Our `new` function now returns a `Result`, with a `Config` instance in the
success case and a `&'static str` in the error case. Recall from "The Static
Lifetime" section in Chapter 10 that `&'static str` is the type of string
literals, which is our error message type for now.

We've made two changes in the body of the `new` function: instead of calling
`panic!` when the user doesn't pass enough arguments, we now return an `Err`
value, and we've wrapped the `Config` return value in an `Ok`. These changes
make the function conform to its new type signature.

By having `Config::new` return an `Err` value, it allows the `main` function to
handle the `Result` value returned from the `new` function and exit the process
more cleanly in the error case.

#### Calling `Config::new` and Handling Errors

In order to handle the error case and print a user-friendly message, we need to
update `main` to handle the `Result` that `Config::new` is now returning as
shown in Listing 12-10. We're also going to implement by hand something that
`panic!` handled for us: exiting the command line tool with an error code of 1.
A nonzero exit status is a convention to signal to the process that called our
program that our program ended with an error state.

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

<span class="caption">Listing 12-10: Exiting with an error code if creating a
new `Config` fails</span>

In this listing, we're using a method we haven't covered before:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method's behavior is similar
to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value
is an `Err` value, this method calls the code in the *closure*, which is an
anonymous function we define and pass as an argument to `unwrap_or_else`. We'll
be covering closures in more detail in Chapter 13. What you need to know for
now is that `unwrap_or_else` will pass the inner value of the `Err`, which in
this case is the static string `not enough arguments` that we added in Listing
12-9, to our closure in the argument `err` that appears between the vertical
pipes. The code in the closure can then use the `err` value when it runs.

We've added a new `use` line to import `process` from the standard library. The
code in the closure that will get run in the error case is only two lines: we
print out the `err` value, then call `std::process::exit` (we've added a new
`use` line at the top to import `process` from the standard library).
`process::exit` will stop the program immediately and return the number that
was passed as the exit status code. This is similar to the `panic!`-based
handling we used in Listing 12-8, with the exception that we no longer get all
the extra output. Let's try it:

```text
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

Great! This output is much friendlier for our users.

### Extracting a `run` Function

Now we're done refactoring our configuration parsing; let's turn to our
program's logic. As we laid out in the process we discussed in the "Separation
of Concerns for Binary Projects" section, we're going to extract a function
named `run` that will hold all of the logic currently in the `main` function
that isn't setting up configuration or handling errors. Once we're done, `main`
will be concise and easy to verify by inspection, and we'll be able to write
tests for all of the other logic.

Listing 12-11 shows the extracted `run` function. For now, we're making only
the small, incremental improvement of extracting the function and still
defining the function in *src/main.rs*:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // ...snip...

    println!("Searching for {}", config.query);
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

<span class="caption">Listing 12-11: Extracting a `run` function containing the
rest of the program logic</span>

The `run` function now contains all the remaining logic from `main` starting
from reading the file. The `run` function takes the `Config` instance as an
argument.

#### Returning Errors from the `run` Function

With the remaining program logic separated into the `run` function rather than
being in `main`, we can improve the error handling like we did with
`Config::new` in Listing 12-9. Instead of allowing the program to panic by
calling `expect`, the `run` function will return a `Result<T, E>` when
something goes wrong. This will let us further consolidate the logic around
handling errors in a user-friendly way into `main`. Listing 12-12 shows the
changes to the signature and body of `run`:

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

<span class="caption">Listing 12-12: Changing the `run` function to return
`Result`</span>

We've made three big changes here. First, we're changing the return type of the
`run` function to `Result<(), Box<Error>>`. This function previously returned
the unit type, `()`, and we keep that as the value returned in the `Ok` case.

For our error type, we're using the *trait object* `Box<Error>` (and we've
brought `std::error::Error` into scope with a `use` statement at the top).
We'll be covering trait objects in Chapter 17. For now, just know that
`Box<Error>` means the function will return a type that implements the `Error`
trait, but we don't have to specify what particular type the return value will
be. This gives us flexibility to return error values that may be of different
types in different error cases.

The second change we're making is removing the calls to `expect` in favor of
`?`, like we talked about in Chapter 9. Rather than `panic!` on an error, this
will return the error value from the current function for the caller to handle.

Thirdly, this function now returns an `Ok` value in the success case. We've
declared the `run` function's success type as `()` in the signature, which
means we need to wrap the unit type value in the `Ok` value. This `Ok(())`
syntax may look a bit strange at first, but using `()` like this is the
idiomatic way to indicate that we're calling `run` for its side effects only;
it doesn't return a value we need.

When you run this, it will compile, but with a warning:

```text
warning: unused result which must be used, #[warn(unused_must_use)] on by default
  --> src/main.rs:39:5
   |
39 |     run(config);
   |     ^^^^^^^^^^^^
```

Rust is telling us that our code ignores the `Result` value, which might be
indicating that there was an error. We're not checking to see if there was an
error or not, though, and the compiler is reminding us that we probably meant
to have some error handling code here! Let's rectify that now.

#### Handling Errors Returned from `run` in `main`

We'll check for errors and handle them nicely using a similar technique to the
way we handled errors with `Config::new` in Listing 12-10, but with a slight
difference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    // ...snip...

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

We use `if let` to check whether `run` returns an `Err` value, rather than
`unwrap_or_else`, and call `process::exit(1)` if it does. `run` doesn't return
a value that we want to `unwrap` like `Config::new` returns the `Config`
instance. Because `run` returns a `()` in the success case, we only care about
detecting an error, so we don't need `unwrap_or_else` to return the unwrapped
value as it would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases though: we print out the error and exit.

### Split Code into a Library Crate

This is looking pretty good so far! Now we're going to split the *src/main.rs*
file up and put some code into *src/lib.rs* so that we can test it and have a
small `main` function.

Let's move the following pieces of code from *src/main.rs* to a new file,
*src/lib.rs*:

- The `run` function definition
- The relevant `use` statements
- The definition of `Config`
- The `Config::new` function definition

The contents of *src/lib.rs* should now look like Listing 12-13:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
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

<span class="caption">Listing 12-13: Moving `Config` and `run` into
*src/lib.rs*</span>

We've made liberal use of `pub` here: on `Config`, its fields and its `new`
method, and on the `run` function. We now have a library crate that has a
public API that we can test.

#### Calling the Library Crate from the Binary Crate

Now we need to bring the code we moved to *src/lib.rs* into the scope of the
binary crate in *src/main.rs* by using `extern crate minigrep`. Then we'll add a
`use minigrep::Config` line to bring the `Config` type into scope, and prefix the
`run` function with our crate name as shown in Listing 12-14:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

<span class="caption">Listing 12-14: Bringing the `minigrep` crate into the scope
of *src/main.rs*</span>

With that, all the functionality should be connected and should work. Give it a
`cargo run` and make sure everything is wired up correctly.

Whew! That was a lot of work, but we've set ourselves up for success in the
future. Now it's much easier to handle errors, and we've made our code more
modular. Almost all of our work will be done in *src/lib.rs* from here on out.

Let's take advantage of this newfound modularity by doing something that would
have been hard with our old code, but is easy with our new code: write some
tests!
