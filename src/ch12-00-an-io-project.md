# An I/O Project

`cargo new --bin greprs`

Project requirements:

- Take a search string and a filename as command line arguments
- Read the file
- Find lines in the file that contain the search string
- Print out those lines

In order to do a good job, we will:

- Organize code (using what we learned in modules, ch 7)
- Use vectors and strings (collections, ch 8)
- Handle errors (ch 9)
- Have tests (ch 11)

Generics/traits/lifetimes?

## Command line arguments

* Use `std::env::args()`

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

Discard the binary name, next argument is the search string, next argument is
the filename we want to search in:

<!-- I'd probably use `next` here instead of collecting, but we haven't really
covered iterators yet and we have covered vectors.

Should we do a match so that when someone gives

/Carol -->

Filename: src/main.rs

```rust,ignore
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let search = args.get(1).expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.get(2).expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    println!("Searching for {}", search);
    println!("In file {}", filename);
}
```

Trying it out:

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
    let args: Vec<String> = env::args().collect();

    let search = args.get(1).expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.get(2).expect(
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

Get the test passing:

File: src/lib.rs

```rust
pub fn grep<'a>(search: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
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
    let args: Vec<String> = env::args().collect();

    let search = args.get(1).expect(
        "No search string or filename found. Usage: greprs <search> <file>"
    );
    let filename = args.get(2).expect(
        "No filename found. Usage: greprs <search> <file>"
    );

    let mut f = File::open(filename).expect("Could not open file.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    let results = greprs::grep(search, &contents);

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
