# An I/O Project

`cargo new --bin greprs`

Project requirements:

- Take a search string and a filename as command line arguments
- Read the file
- Find lines in the file that contain the search string
- Print out those lines
- Have an environment variable setting to search without regards to case

In order to do a good job, we will:

- Organize code (using what we learned in modules, ch 7)
- Use vectors and strings (collections, ch 8)
- Handle errors (ch 9)
- Use traits and lifetimes where appropriate (ch 10)
- Have tests (ch 11)

## Command line arguments

To get the arguments passed to the binary, use `std::env::args()`. This
function returns an *iterator*. We're going to use a bunch of iterators and
useful functions on them in this chapter, and chapter 16 will go into depth on
how iterators work.

We can see all the items in the iterator by collecting them into a vector and
printing it out using debug formatting:

Filename: src/main.rs

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

Run it with various arguments. The binary is the first argument:

```text
$ cargo run
["target/debug/greprs"]

$ cargo run needle haystack
...snip...
["target/debug/greprs", "needle", "haystack"]
```

What we want to do is:

1. Discard the binary name that will always be the first argument
2. Collect the rest of the arguments and store them in a configuration struct
3. Pass the configuration to a `run` function that returns a `Result`
4. If there were errors, print information about them and exit with 1.

The reason we want to have a `run` function in `src/lib.rs` is that it will
make it easier to write tests and pass in different configurations, without
having to actually run our binary from the command line.

Filename: src/main.rs

```rust,ignore
extern crate greprs;

use greprs::Config;

use std::env;
use std::process;

fn main() {
    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    let config = Config {
        arguments: args.collect(),
    };

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

The code in `src/lib.rs` now needs to define a `Config` struct and a `run`
function. The `run` function in `src/lib.rs` will:

1. Get the search string, which will be in the next argument
2. Get the filename we want to search in as the next argument

This function will ignore any arguments after that, and will return an error if
it doesn't get enough arguments.

Filename: src/lib.rs

```rust
pub struct Config {
    pub arguments: Vec<String>,
}

pub fn run(config: Config) -> Result<(), String> {
    let mut args = config.arguments.iter();

    let search = args.next().ok_or("No search string or filename found")?;
    let filename = args.next().ok_or("No filename found")?;

    println!("Searching for {}", search);
    println!("In file {}", filename);
    Ok(())
}
```

Try it out with no arguments, one argument, and two arguments:

```text
$ cargo run
Application error: No search string or filename found

$ cargo run needle
Application error: No filename found

$ cargo run needle haystack
Searching for needle
In file haystack
```

## Reading a file

File to search in:

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

Filename: src/lib.rs

```rust
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub arguments: Vec<String>,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut args = config.arguments.iter();

    let search = args.next().ok_or("No search string or filename found")?;
    let filename = args.next().ok_or("No filename found")?;

    println!("Searching for {}", search);
    println!("In file {}", filename);

    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);
    Ok(())
}
```

```text
$ cargo run test poem.txt
Searching for test
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

## Tests

Start some tests in *src/lib.rs*.

We want a function that will take a search term and contents, and return a
vector of lines from the contents that contain the search term. Here's a
failing test:

<!-- Oh hey we can use lifetimes here /Carol -->

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

We're going to get the test passing by:

1. Getting an iterator over each line of the contents with the `lines` function
2. Use the `filter` method and specify the condition a line should meet in
   order to pass through the filter.
3. The condition is that the line contains the search string
4. Collect the results of the filtered iterator into a vector and return it

File: src/lib.rs

```rust
pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(search)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

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

## Call the `grep` function from the `run` function

Filename: src/lib.rs

```rust
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub arguments: Vec<String>,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut args = config.arguments.iter();

    let search = args.next().ok_or("No search string or filename found")?;
    let filename = args.next().ok_or("No filename found")?;

    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = grep(&search, &contents);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(search)).collect()
}
```

```text
$ cargo run the poem.txt
Then there's a pair of us — don't tell!
To tell your name the livelong day
```

## Working with Environment Variables

Enhancement we want to add: allow the user of our program to set an environment
variable in their terminal session to do searches that match on case exactly or
not.

### Implement and Test a Case-Insensitive `grep` Function

Filename: src/lib.rs

```rust
pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(search)).collect()
}

pub fn grep_case_insensitive<'a>(search: &str, contents: &'a str)
       -> Vec<&'a str> {
    let search = search.to_lowercase();
    contents.lines().filter(|line| {
        line.to_lowercase().contains(&search)
    }).collect()
}

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

### Have `main` Check the Environment Variable

And put the result in the configuration that gets passed to `run`.

Filename: src/main.rs

```rust,ignore
extern crate greprs;

use greprs::Config;

use std::env;
use std::process;

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
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

Modify `run` to decide which `grep` function to call based on the configuration.

Filename: src/lib.rs

```rust
pub struct Config {
    pub arguments: Vec<String>,
    pub case_insensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut args = config.arguments.iter();

    let search = args.next().ok_or("No search string or filename found")?;
    let filename = args.next().ok_or("No filename found")?;

    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_insensitive {
        grep_case_insensitive(&search, &contents)
    } else {
        grep(&search, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

```text
$ cargo run to poem.txt
Are you nobody, too?
How dreary to be somebody!
```

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

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
