## Testing the Library's Functionality

Now that we've extracted the logic into *src/lib.rs* and left all the argument
parsing and error handling in *src/main.rs*, it's much easier for us to write
tests for the core functionality of our code. We can call our functions
directly with various arguments and check return values without having to call
our binary from the command line.

In this section, we're going to follow the Test Driven Development (TDD)
process. This is a software development technique that follows this set of
steps:

1. Write a test that fails, and run it to make sure it fails for the reason
   you expected.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed, and make sure the tests
   continue to pass.
4. Repeat!

This is just one of many ways to write software, but TDD can help drive the
design of code. Writing the test before writing the code that makes the test
pass helps to maintain high test coverage throughout the process.

We're going to test drive the implementation of the part of our `greprs`
program that will actually do the searching for the query string in the file
contents and produce a list of lines that match the query. We're going to add
this functionality in a function called `search`.

### Writing a Failing Test

First, since we don't really need them any more, let's remove the `println!`
statements from both *src/lib.rs* and *src/main.rs*. Then we'll add a `test`
module with a test function, like we did in Chapter 11. The test function
specifies the behavior we'd like the `search` function to have: it will take
a query and the text to search for the query in, and will return only the lines
from the text that contain the query. Listing 12-15 shows this test:

<span class="filename">Filename: src/lib.rs</span>

```rust
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```

<span class="caption">Listing 12-15: Creating a failing test for the `search`
function we wish we had</span>

We've chosen to use "duct" as the string we're looking for in this test. The
text we're searching in is three lines, only one of which contains "duct". We
assert that the value returned from the `search` function contains only the one
line we expect.

We aren't able to run this test and watch it fail though, since this test
doesn't even compile yet! We're going to add just enough code to get it to
compile: a definition of the `search` function that always returns an empty
vector, as shown in Listing 12-16. Once we have this, the test should compile
and fail because an empty vector doesn't match a vector containing the one
line `"safe, fast, productive."`.

<span class="filename">Filename: src/lib.rs</span>

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}
```

<span class="caption">Listing 12-16: Defining just enough of the `search`
function that our test will compile</span>

Notice that we need an explicit lifetime `'a` defined in the signature of
`search` and used with the `contents` argument and the return value. Remember
from Chapter 10 that the lifetime parameters specify which argument lifetime is
connected to the lifetime of the return value. In this case, we're indicating
that the returned vector should contain string slices that reference slices of
the argument `contents` (rather than the argument `query`).

In other words, we're telling Rust that the data returned by the `search`
function will live as long as the data passed into the `search` function in the
`contents` argument. This is important! The data referenced *by* a slice needs
to be valid in order for the reference to be valid; if the compiler assumed we
were making string slices of `query` rather than `contents`, it would do its
safety checking incorrectly.

If we tried to compile this function without lifetimes, we would get this error:

```text
error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:47
  |
5 | fn search(query: &str, contents: &str) -> Vec<&str> {
  |                                               ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
  signature does not say whether it is borrowed from `query` or `contents`
```

Rust can't possibly know which of the two arguments we need, so we need to tell
it. Because `contents` is the argument that contains all of our text and we
want to return the parts of that text that match, we know `contents` is the
argument that should be connected to the return value using the lifetime syntax.

Other programming languages don't require you to connect arguments to return
values in the signature, so this may still feel strange, but will get easier
over time. You may want to compare this example with the Lifetime Syntax
section in Chapter 10.

Now let's try running our test:

```text
$ cargo test
...warnings...
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
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

### Writing Code that Gets the Test to Pass

Currently, our test is failing because we always return an empty vector. To fix
that and implement `search`, our program needs to follow these steps:

1. Iterate through each line of the contents.
2. Check if the line contains our query string.
   * If it does, add it to the list of values we're returning.
   * If it doesn't, do nothing.
3. Return the list of results that match.

Let's take each step at a time, starting with iterating through lines.

#### Iterating Through Lines with the `lines` method

Rust has a helpful method to handle line-by-line iteration of strings,
conveniently named `lines`, that works as shown in Listing 12-17:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

<span class="caption">Listing 12-17: Iterating through each line in
`contents`</span>

The `lines` method returns an iterator. We'll be talking about iterators in
depth in Chapter 13, but we've already seen this way of using an iterator in
Listing 3-6, where we used a `for` loop with an iterator to run some code on
each item in a collection.

#### Searching Each Line for the Query

Next, we'll add functionality to check if the current line contains the query
string. Luckily, strings have another helpful method named `contains` that does
this for us! Add the `contains` method to the `search` function as shown in
Listing 12-18:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```

<span class="caption">Listing 12-18: Adding functionality to see if the line
contains the string in `query`</span>

#### Storing Matching Lines

Finally, we need a way to store the lines that contain our query string. For
that, we can make a mutable vector before the `for` loop and call the `push`
method to store a `line` in the vector. After the `for` loop, we return the
vector, as shown in Listing 12-19:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">Listing 12-19: Storing the lines that match so that we
can return them</span>

Now the `search` function should be returning only the lines that contain
`query`, and our test should pass. Let's run the tests:

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

Our test passed, great, it works!

Now that our test is passing, we could consider opportunities for refactoring
the implementation of the `search` function while keeping the tests passing in
order to maintain the same functionality while we do so. This code isn't bad,
but it isn't taking advantage of some useful features of iterators. We'll be
coming back to this example in Chapter 13 where we'll explore iterators in
detail and see how to improve it.

#### Using the `search` Function in the `run` Function

Now that we have the `search` function working and tested, we need to actually
call `search` from our `run` function. We need to pass the `config.query` value
and the `contents` that `run` read from the file to the `search` function. Then
`run` will print out each line returned from `search`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

We're again using a `for` loop to get each line returned from `search`, and
the code that we run for each line prints it out.

Now our whole program should be working! Let's try it out, first with a word
that should return exactly one line from the Emily Dickinson poem, "frog":

```text
$ cargo run frog poem.txt
   Compiling greprs v0.1.0 (file:///projects/greprs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38 secs
     Running `target/debug/greprs frog poem.txt`
How public, like a frog
```

Cool! Next, how about a word that will match multiple lines, like "the":

```text
$ cargo run the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs the poem.txt`
Then there's a pair of us — don't tell!
To tell your name the livelong day
```

And finally, let's make sure that we don't get any lines when we search for a
word that isn't anywhere in the poem, like "monomorphization":

```text
$ cargo run monomorphization poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/greprs monomorphization poem.txt`
```

Excellent! We've built our own version of a classic tool, and learned a lot
about how to structure applications. We've also learned a bit about file input
and output, lifetimes, testing, and command line parsing.

Feel free to move on to Chapter 13 if you'd like at this point. To round out
this project chapter, though, we're going to briefly demonstrate how to work
with environment variables and printing to standard error, both of which are
useful when writing command line programs.
