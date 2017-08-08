## Working with Environment Variables

We’re going to improve our tool with an extra feature: an option for case
insensitive searching that the user can turn on via an environment variable. We
could make this a command line option and require that users enter it each time
they want it to apply, but instead we’re going to use an environment variable.
This allows our users to set the environment variable once and have all their
searches be case insensitive in that terminal session.

### Writing a Failing Test for the Case-Insensitive `search` Function

We want to add a new `search_case_insensitive` function that we will call when
the environment variable is on.

We’re going to continue following the TDD process, so the first step is again
to write a failing test. We’ll add a new test for the new case-insensitive
search function, and rename our old test from `one_result` to `case_sensitive`
to be clearer about the differences between the two tests, as shown in Listing
12-20:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

<span class="caption">Listing 12-20: Adding a new failing test for the case
insensitive function we’re about to add</span>

Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text “Duct tape”, with a capital D, that shouldn’t match the query
“duct” when we’re searching in a case sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case sensitive
search functionality that we’ve already implemented; this test should pass now
and should continue to pass as we work on the case insensitive search.

The new test for the case *insensitive* search uses “rUsT” as its query. In the
`search_case_insensitive` function we’re going to add, the query “rUsT” should
match both the line containing “Rust:” with a capital R and also the line
“Trust me.” even though both of those have different casing than the query.
This is our failing test, and it will fail to compile because we haven’t yet
defined the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector in the same way that we did
for the `search` function in Listing 12-16 in order to see the test compile and
fail.

### Implementing the `search_case_insensitive` Function

The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we’ll lowercase
the `query` and each `line` so that whatever the case of the input arguments,
they will be the same case when we check whether the line contains the query.

<span class="filename">Filename: src/lib.rs</span>

```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase both the query and the line before comparing them</span>

First, we lowercase the `query` string, and store it in a shadowed variable
with the same name. Calling `to_lowercase` on the query is necessary so that no
matter if the user’s query is “rust”, “RUST”, “Rust”, or “rUsT”, we’ll treat
the query as if it was “rust” and be insensitive to the case.

Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is “rUsT”, as an example: that string slice does not contain a lowercase
“u” or “t” for us to use, so we have to allocate a new `String` containing
“rust”. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.

Next, we add a call to `to_lowercase` on each `line` before we check if it
contains `query` to lowercase all characters. Now that we’ve converted both
`line` and `query` to lowercase, we’ll find matches no matter what the case of
the query.

Let’s see if this implementation passes the tests:

```text
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Great! Now, let’s actually call the new `search_case_insensitive` function from
the `run` function. First, we’re going to add a configuration option for
switching between case sensitive and case insensitive search to the `Config`
struct:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

We add the `case_sensitive` field that holds a boolean. Then we need our `run`
function to check the `case_sensitive` field’s value and use that to decide
whether to call the `search` function or the `search_case_insensitive` function
as shown in Listing 12-22:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::error::Error;
# use std::fs::File;
# use std::io::prelude::*;
#
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.case_sensitive`</span>

Finally, we need to actually check for the environment variable. The functions
for working with environment variables are in the `env` module in the standard
library, so we want to bring that module into scope with a `use std::env;` line
at the top of *src/lib.rs*. Then we’re going to use the `var` method from the
`env` module to check for an environment variable named `CASE_INSENSITIVE`, as
shown in Listing 12-23:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::env;
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }

// ...snip...

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

<span class="caption">Listing 12-23: Checking for an environment variable named
`CASE_INSENSITIVE`</span>

Here, we create a new variable `case_sensitive`. In order to set its value, we
call the `env::var` function and pass it the name of the `CASE_INSENSITIVE`
environment variable. The `env::var` method returns a `Result` that will be the
successful `Ok` variant that contains the value of the environment variable if
the environment variable is set. It will return the `Err` variant if the
environment variable is not set.

We’re using the `is_err` method on the `Result` to check to see if it’s an
error, and therefore unset, which means it *should* do a case sensitive search.
If the `CASE_INSENSITIVE` environment variable is set to anything, `is_err`
will return false and it will perform a case insensitive search. We don’t care
about the *value* of the environment variable, just whether it’s set or unset,
so we’re checking `is_err` rather than `unwrap`, `expect`, or any of the other
methods we’ve seen on `Result`.

We pass the value in the `case_sensitive` variable to the `Config` instance so
that the `run` function can read that value and decide whether to call `search`
or `search_case_insensitive` as we implemented in Listing 12-22.

Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query “to”, which should match any line that contains
the word “to” in all lowercase:

```text
$ cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

Looks like that still works! Now, let’s run the program with `CASE_INSENSITIVE`
set to 1 but with the same query “to”, and we should get lines that contain
“to” that might have uppercase letters:

```text
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent, we also got lines containing “To”! Our `minigrep` program can now do
case insensitive searching, controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables!

Some programs allow both arguments *and* environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case
insensitivity through either a command line argument or an environment
variable. Decide whether the command line argument or the environment variable
should take precedence if the program is run with one set to case sensitive and
one set to case insensitive.

The `std::env` module contains many more useful features for dealing with
environment variables; check out its documentation to see what’s available.
