# Testing

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
>
> Edsger W. Dijkstra, "The Humble Programmer" (1972)

Rust is a programming language that cares a lot about correctness. But
correctness is a complex topic, and isn't exactly easy to get right. Rust
places a lot of weight on its type system to help ensure that our programs do
what we intend, but it cannot help with everything. As such, Rust also includes
support for writing software tests in the language itself.

Testing is a skill, and we cannot hope to learn everything about how to write
good tests in one chapter of a book. What we can learn, however, are the
mechanics of Rust's testing facilities. That's what we'll focus on in this
chapter.

## The `test` attribute

At its simplest, a test in Rust is a function that's annotated with the `test`
attribute. Let's make a new project with Cargo called `adder`:

```bash
$ cargo new adder
     Created library `adder` project
$ cd adder
```

Cargo will automatically generate a simple test when you make a new project.
Here's the contents of `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

For now, let's remove the `mod` bit, and focus on just the function:

```rust
#[test]
fn it_works() {
}
```

Note the `#[test]`. This attribute indicates that this is a test function. It
currently has no body. That's good enough to pass! We can run the tests with
`cargo test`:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Cargo compiled and ran our tests. There are two sets of output here: one
for the test we wrote, and another for documentation tests. We'll talk about
documentation tests later. For now, see this line:

```text
test it_works ... ok
```

Note the `it_works`. This comes from the name of our function:

```rust
fn it_works() {
# }
```

We also get a summary line:

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## The `assert!` macro

So why does our do-nothing test pass? Any test which doesn't `panic!` passes,
and any test that does `panic!` fails. Let's make our test fail:

```rust
#[test]
fn it_works() {
    assert!(false);
}
```

`assert!` is a macro provided by Rust which takes one argument: if the argument
is `true`, nothing happens. If the argument is `false`, it will `panic!`. Let's
run our tests again:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

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

And that's reflected in the summary line:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

## Inverting failure with `should_panic`

We can invert our test's failure with another attribute: `should_panic`:

```rust
#[test]
#[should_panic]
fn it_works() {
    assert!(false);
}
```

This test will now succeed if we `panic!` and fail if we complete.

`should_panic` tests can be fragile, as it's hard to guarantee that the test
didn't fail for an unexpected reason. To help with this, an optional `expected`
parameter can be added to the `should_panic` attribute. The test harness will
make sure that the failure message contains the provided text. A safer version
of the example above would be:

```rust
#[test]
#[should_panic(expected = "assertion failed")]
fn it_works() {
    assert!(false);
}
```

## Testing equality

Rust provides a pair of macros, `assert_eq!` and `assert_ne!`, that compares
two arguments for equality:

```rust
#[test]
fn it_works() {
    assert_eq!("Hello", "Hello");

    assert_ne!("Hello", "world");
}
```

These macros expand to something like this:

```rust,ignore
// assert_eq
if left_val == right_val {
    panic!("message goes here")
}

// assert_ne
if left_val =! right_val {
    panic!("message goes here")
}
```

But they're a bit more convenient than writing this out by hand.  These macros
are often used to call some function with some known arguments and compare it
to the expected output, like this:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}
```

## The `ignore` attribute

Sometimes a few specific tests can be very time-consuming to execute. These
can be disabled by default by using the `ignore` attribute:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[test]
fn it_works() {
    assert_eq!(4, add_two(2));
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

Now we run our tests and see that `it_works` is run, but `expensive_test` is
not:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

The expensive tests can be run explicitly using `cargo test -- --ignored`:

```bash
$ cargo test -- --ignored
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

The `--ignored` argument is an argument to the test binary, and not to Cargo,
which is why the command is `cargo test -- --ignored`.
