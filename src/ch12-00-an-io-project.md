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
- Have tests (ch 11)

Generics/traits/lifetimes?

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

1. Discard the binary name
2. Get the search string, which will be in the next argument
3. Get the filename we want to search in as the next argument

We'll ignore any arguments after that, and we'll error if we don't get enough
arguments.

Filename: src/main.rs

```rust,ignore
use std::env;

fn main() {
    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    // `next` returns an `Option`
    let search = args.next().expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.next().expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    println!("Searching for {}", search);
    println!("In file {}", filename);
}
```

Try it out with no arguments, one argument, and two arguments:

```text
$ cargo run
thread 'main' panicked at 'No search string or filename found. Usage: greprs <search> <file>', ../src/libcore/option.rs:705

$ cargo run needle
thread 'main' panicked at 'No filename found. Usage: greprs <search> <file>', ../src/libcore/option.rs:705

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

Filename: src/main.rs

```rust,ignore
use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    // `next` returns an `Option`
    let search = args.next().expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.next().expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    println!("Searching for {}", search);
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("Could not open file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    println!("With text:\n{}", contents);
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

Make a src/lib.rs and start some tests.

We want a function that will take a search term and contents, and return a
vector of lines from the contents that contain the search term.

<!-- Oh hey we can use lifetimes here /Carol -->

File: src/lib.rs

```rust
pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
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

Get the test passing by:

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

## Call the function from main

Filename: src/main.rs

```rust,ignore
extern crate greprs;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    // `next` returns an `Option`
    let search = args.next().expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.next().expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    let mut f = File::open(filename).expect("Could not open file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    let results = greprs::grep(&search, &contents);

    for line in results {
        println!("{}", line);
    }
}
```

```text
$ cargo run the poem.txt
Then there's a pair of us — don't tell!
To tell your name the livelong day
```

## Working with Environment Variables

### Implement a Case-Insensitive `grep` Function

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
    use super::*;

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

Filename: src/main.rs

```rust,ignore
extern crate greprs;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut case_sensitive = true;
    for (var, _) in env::vars() {
        if var == "CASE_INSENSITIVE" {
            case_sensitive = false;
        }
    }

    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    // `next` returns an `Option`
    let search = args.next().expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.next().expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    let results = if case_sensitive {
        greprs::grep(search, &contents)
    } else {
        greprs::grep_case_insensitive(search, &contents)
    };

    for line in results {
        println!("{}", line);
    }
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

Let's say we want to output statistics about how many lines have matched to `stderr`.

Filename: src/main.rs

```rust,ignore
extern crate greprs;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut stderr = std::io::stderr();

    let case_insensitive = env::vars().find(|&(ref var, _)| {
        var ==  "CASE_INSENSITIVE"
    }).is_some();

    let mut args = env::args();

    // Discard the name of the binary
    args.next();

    // `next` returns an `Option`
    let search = args.next().expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.next().expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    let mut f = File::open(filename).expect("Could not open file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    let results = if case_insensitive {
        greprs::grep_case_insensitive(&search, &contents)
    } else {
        greprs::grep(&search, &contents)
    };

    let matching_lines = results.len();
    writeln!(
        &mut stderr,
        "{} lines matched",
        matching_lines
    ).expect("Could not write to stderr");

    for line in results {
        println!("{}", line);
    }
}
```

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
    use super::*;

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

Output:

```text
$ cargo run to poem.txt
2/9 lines matched
Are you nobody, too?
How dreary to be somebody!
```

Redirecting stdout to a file, stderr still gets displayed:

```text
$ cargo run to poem.txt > new_poem.txt
2/9 lines matched
```
