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

```rust
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let (search, filename) = parse_config(&args);

    println!("Searching for {}", search);
    println!("In file {}", filename);

    // ...snip...
#
#     let mut f = File::open(filename).expect("file not found");
#
#     let mut contents = String::new();
#     f.read_to_string(&mut contents).expect("something went wrong reading the file");
#
#     println!("With text:\n{}", contents);
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
# use std::fs::File;
# use std::io::prelude::*;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // ...snip...
#     let mut contents = String::new();
#     f.read_to_string(&mut contents).expect("something went wrong reading the file");
#
#    println!("With text:\n{}", contents);
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
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
#
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    // ...snip...

#     let mut f = File::open(config.filename).expect("file not found");
#
#     let mut contents = String::new();
#     f.read_to_string(&mut contents).expect("something went wrong reading the file");
#
#    println!("With text:\n{}", contents);

}

# struct Config {
#     search: String,
#     filename: String,
# }
#
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

```rust
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
#
# fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let config = Config::new(&args);
#
#     println!("Searching for {}", config.search);
#     println!("In file {}", config.filename);
#
#     let mut f = File::open(config.filename).expect("file not found");
#
#     let mut contents = String::new();
#     f.read_to_string(&mut contents).expect("something went wrong reading the file");
#
#     println!("With text:\n{}", contents);
# }
#
# struct Config {
#     search: String,
#     filename: String,
# }
#
# impl Config {
// ...snip...
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }

    let search = args[1].clone();
    // ...snip...
#     let filename = args[2].clone();
#
#     Config {
#         search: search,
#         filename: filename,
#     }
}
# }
```

<figcaption>

Listing 12-7: Adding a check for the number of arguments

</figcaption>
</figure>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

With these extra few lines of code in `new`, let's try running our program
without any arguments:

```text
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

```rust
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
# use std::process;
#
# fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let config = Config::new(&args).unwrap_or_else(|err| {
#         println!("Problem parsing arguments: {}", err);
#         process::exit(1);
#     });
#
#     println!("Searching for {}", config.search);
#     println!("In file {}", config.filename);
#
#     let mut f = File::open(config.filename).expect("file not found");
#
#     let mut contents = String::new();
#     f.read_to_string(&mut contents).expect("something went wrong reading the file");
#
#     println!("With text:\n{}", contents);
# }
# struct Config {
#     search: String,
#     filename: String,
# }
#
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

```rust
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
// ...snip...
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.search);
    println!("In file {}", config.filename);

    // ...snip...
#
#     let mut f = File::open(config.filename).expect("file not found");
#
#     let mut contents = String::new();
#     f.read_to_string(&mut contents).expect("something went wrong reading the file");
#
#     println!("With text:\n{}", contents);
# }
#
# struct Config {
#     search: String,
#     filename: String,
# }
#
# impl Config {
#     fn new(args: &[String]) -> Result<Config, &'static str> {
#         if args.len() < 3 {
#             return Err("not enough arguments");
#         }
#
#         let search = args[1].clone();
#         let filename = args[2].clone();
#
#         Ok(Config {
#             search: search,
#             filename: filename,
#         })
#     }
# }
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
pass the inner value of the `Err` to our closure in the parameter `err` that
appears between the vertical pipes. Using `unwrap_or_else` lets us do some
custom, non-`panic!` error handling.

Said error handling is only two lines: we print out the error, then call
`std::process::exit`. That function will stop our program's execution
immediately and return the number passed to it as a return code. By convention,
a zero means success and any other value means failure. In the end, this has
similar characteristics to our `panic!`-based handling we had in Listing 12-7,
but we no longer get all the extra output. Let's try it:

```text
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

```rust
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
# use std::process;
#
fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let config = Config::new(&args).unwrap_or_else(|err| {
#         println!("Problem parsing arguments: {}", err);
#         process::exit(1);
#     });
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
#
# struct Config {
#     search: String,
#     filename: String,
# }
#
# impl Config {
#     fn new(args: &[String]) -> Result<Config, &'static str> {
#         if args.len() < 3 {
#             return Err("not enough arguments");
#         }
#
#         let search = args[1].clone();
#         let filename = args[2].clone();
#
#         Ok(Config {
#             search: search,
#             filename: filename,
#         })
#     }
# }
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

```rust
use std::error::Error;
# use std::env;
# use std::fs::File;
# use std::io::prelude::*;
# use std::process;

// ...snip...
# fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let config = Config::new(&args).unwrap_or_else(|err| {
#         println!("Problem parsing arguments: {}", err);
#         process::exit(1);
#     });
#
#     println!("Searching for {}", config.search);
#     println!("In file {}", config.filename);
#
#     run(config);
#
# }

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
#
# struct Config {
#     search: String,
#     filename: String,
# }
#
# impl Config {
#     fn new(args: &[String]) -> Result<Config, &'static str> {
#         if args.len() < 3 {
#             return Err("not enough arguments");
#         }
#
#         let search = args[1].clone();
#         let filename = args[2].clone();
#
#         Ok(Config {
#             search: search,
#             filename: filename,
#         })
#     }
# }
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

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
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

```rust
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
