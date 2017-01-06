## Writing Tests

Tests are Rust functions that use particular features and are written in such a
way as to verify that non-test code is functioning in the expected manner.
Everything we've discussed about Rust code applies to Rust tests as well! Let's
look at the features Rust provides specifically for writing tests: the `test`
attribute, a few macros, and the `should_panic` attribute.

### The `test` attribute

At its simplest, a test in Rust is a function that's annotated with the `test`
attribute. Let's make a new library project with Cargo called `adder`:

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

Cargo will automatically generate a simple test when you make a new library
project. Here's the contents of `src/lib.rs`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

For now, let's ignore the `tests` module and the `#[cfg(test)]` annotation in
order to focus on just the function. Note the `#[test]` before it: this
attribute indicates this is a test function. The function currently has no
body; that's good enough to pass! We can run the tests with `cargo test`:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Cargo compiled and ran our tests. There are two sets of output here; we're
going to focus on the first set in this chapter. The second set of output is
for documentation tests, which we'll talk about in Chapter 14. For now, note
this line:

```text
test it_works ... ok
```

The `it_works` text comes from the name of our function.

We also get a summary line that tells us the aggregate results of all the
tests that we have:

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

### The `assert!` macro

The empty test function passes because any test which doesn't `panic!` passes,
and any test that does `panic!` fails. Let's make the test fail by using the
`assert!` macro:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert!(false);
}
```

The `assert!` macro is provided by the standard library, and it takes one
argument. If the argument is `true`, nothing happens. If the argument is
`false`, the macro will `panic!`. Let's run our tests again:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test it_works ... FAILED

failures:

---- it_works stdout ----
	thread 'it_works' panicked at 'assertion failed: false', src/lib.rs:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

Rust indicates that our test failed:

```text
test it_works ... FAILED
```

And shows that the test failed because the `assert!` macro in `src/lib.rs` on
line 5 got a `false` value:

```text
thread 'it_works' panicked at 'assertion failed: false', src/lib.rs:5
```

The test failure is also reflected in the summary line:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

### Testing equality with the `assert_eq!` and `assert_ne!` macros

A common way to test functionality is to compare the result of the code under
test to the value you expect it to be, and check that they're equal. You can do
this using the `assert!` macro by passing it an expression using the `==`
macro. This is so common, though, that the standard library provides a pair of
macros to do this for convenience: `assert_eq!` and `assert_ne!`. These macros
compare two arguments for equality or inequality, respectively. The other
advantage of using these macros is they will print out what the two values
actually are if the assertion fails so that it's easier to see *why* the test
failed, whereas the `assert!` macro would just print out that it got a `false`
value for the `==` expression.

Here's an example test that uses each of these macros and will pass:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert_eq!("Hello", "Hello");

    assert_ne!("Hello", "world");
}
```

You can also specify an optional third argument to each of these macros, which
is a custom message that you'd like to be added to the failure message. The
macros expand to logic similar to this:

```rust,ignore
// assert_eq! - panic if the values aren't equal
if left_val != right_val {
    panic!(
        "assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`): {}"
        left_val,
        right_val,
        optional_custom_message
    )
}

// assert_ne! - panic if the values are equal
if left_val == right_val {
    panic!(
        "assertion failed: `(left != right)` (left: `{:?}`, right: `{:?}`): {}"
        left_val,
        right_val,
        optional_custom_message
    )
}
```

Let's take a look at a test that will fail because `hello` is not equal to
`world`. We've also added a custom error message, `greeting operation failed`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn a_simple_case() {
    let result = "hello"; // this value would come from running your code
    assert_eq!(result, "world", "greeting operation failed");
}
```

Running this indeed fails, and the output we get explains why the test failed
and includes the custom error message we specified:

```text
---- a_simple_case stdout ----
	thread 'a_simple_case' panicked at 'assertion failed: `(left == right)`
    (left: `"hello"`, right: `"world"`): greeting operation failed',
    src/main.rs:4
```

The two parameters to `assert_eq!` are named "left" and "right" rather than
"expected" and "actual"; the order of the value that comes from your code and
the value hardcoded into your test isn't important.

Since these macros use the operators `==` and `!=` and print the values using
debug formatting, the values being compared must implement the `PartialEq` and
`Debug` traits. Types provided by Rust implement these traits, but for structs
and enums that you define, you'll need to add `PartialEq` in order to be able
to assert that values of those types are equal or not equal and `Debug` in
order to be able to print out the values in the case that the assertion fails.
Because both of these traits are derivable traits that we mentioned in Chapter
5, usually this is as straightforward as adding the `#[derive(PartialEq,
Debug)]` annotation to your struct or enum definition. See Appendix C for more
details about these and other derivable traits.

## Test for failure with `should_panic`

We can invert our test's failure with another attribute: `should_panic`. This
is useful when we want to test that calling a particular function will cause an
error. For example, let's test something that we know will panic from Chapter
8: attempting to create a slice using range syntax with byte indices that
aren't on character boundaries. Add the `#[should_panic]` attribute before the
function like the `#[test]` attribute, as shown in Listing 11-1:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
#[should_panic]
fn slice_not_on_char_boundaries() {
    let s = "Здравствуйте";
    &s[0..1];
}
```

<figcaption>

Listing 11-1: A test expecting a `panic!`

</figcaption>
</figure>

This test will succeed, since the code panics and we said that it should. If
this code happened to run and did not cause a `panic!`, this test would fail.

`should_panic` tests can be fragile, as it's hard to guarantee that the test
didn't fail for a different reason than the one you were expecting. To help
with this, an optional `expected` parameter can be added to the `should_panic`
attribute. The test harness will make sure that the failure message contains
the provided text. A more robust version of Listing 11-1 would be the
following, in Listing 11-2:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
#[should_panic(expected = "do not lie on character boundary")]
fn slice_not_on_char_boundaries() {
    let s = "Здравствуйте";
    &s[0..1];
}
```

<!-- I will add ghosting in libreoffice /Carol -->

<figcaption>

Listing 11-2: A test expecting a `panic!` with a particular message

</figcaption>
</figure>

Try on your own to see what happens when a `should_panic` test panics but
doesn't match the expected message: cause a `panic!` that happens for a
different reason in this test, or change the expected panic message to
something that doesn't match the character boundary panic message.
