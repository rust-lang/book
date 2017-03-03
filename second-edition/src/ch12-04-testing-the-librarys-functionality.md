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
# use std::error::Error;
# use std::fs::File;
# use std::io::prelude::*;
#
# pub struct Config {
#     pub search: String,
#     pub filename: String,
# }
#
// ...snip...

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
`grep` and used with the `contents` parameter and the return value. Remember,
lifetime parameters are used to specify which function parameters' lifetimes
connect to the lifetime of the return value. In this case, we're indicating that
the vector we're returning is going to contain string slices that reference
slices of the parameter `contents`, as opposed to referencing slices of the
parameter `search`. Another way to think about what we're telling Rust is that
the data returned by the `grep` function will live as long as the data passed
into this function in the `contents` parameter. This is important! Given that
the data a slice references needs to be valid in order for the reference to be
valid, if the compiler thought that we were making string slices of `search`
rather than `contents`, it would do its safety checking incorrectly. If we tried
to compile this function without lifetimes, we would get this error:

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

Rust can't possibly know which of the two parameters we need, so it needs us to
tell it. Because `contents` is the parameter that contains all of our text and
we want to return the parts of that text that match, we know `contents` is the
parameter that should be connected to the return value using the lifetime
syntax.

Connecting parameters to return values in the signature is something that other
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
vector. Listing 12-15 has the full implementation:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
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

<figcaption>

Listing 12-15: Fully functioning implementation of the `grep` function

</figcaption>
</figure>

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
