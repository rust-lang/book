
[TOC]

# Testing

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
>
> Edsger W. Dijkstra, “The Humble Programmer” (1972)

Rust is a programming language that cares a lot about correctness, but
correctness is a complex topic and isn’t easy to prove. Rust places a lot of
weight on its type system to help ensure that our programs do what we intend,
but it cannot help with everything. As such, Rust also includes support for
writing software tests in the language itself.

For example, we can write a function called `add_two` with a signature that
accepts an integer as an argument and returns an integer as a result. We can
implement and compile that function, and Rust can do all the type checking and
borrow checking that we’ve seen it’s capable of doing. What Rust *can’t* check
for us is that we’ve implemented this function to return the argument plus two
and not the argument plus 10 or the argument minus 50! That’s where tests come
in. We can write tests that, for example, pass `3` to the `add_two` function
and check that we get `5` back. We can run the tests whenever we make changes
to our code to make sure we didn’t change any existing behavior from what the
tests specify it should be.

Testing is a skill, and we cannot hope to cover everything about how to write
good tests in one chapter of a book. What we can discuss, however, are the
mechanics of Rust’s testing facilities. We’ll talk about the annotations and
macros available to you when writing your tests, the default behavior and
options provided for running your tests, and how to organize tests into unit
tests and integration tests.

## Writing Tests

Tests are Rust functions that use particular features and are written in such a
way as to verify that non-test code is functioning in the expected manner.
Everything we’ve discussed about Rust code applies to Rust tests as well! Let’s
look at the features Rust provides specifically for writing tests: the `test`
attribute, a few macros, and the `should_panic` attribute.

### The `test` attribute

At its simplest, a test in Rust is a function that’s annotated with the `test`
attribute. Let’s make a new library project with Cargo called `adder`:

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

Cargo will automatically generate a simple test when you make a new library
project. Here’s the contents of `src/lib.rs`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

For now, let’s ignore the `tests` module and the `#[cfg(test)]` annotation in
order to focus on just the function. Note the `#[test]` before it: this
attribute indicates this is a test function. The function currently has no
body; that’s good enough to pass! We can run the tests with `cargo test`:

```text
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

Cargo compiled and ran our tests. There are two sets of output here; we’re
going to focus on the first set in this chapter. The second set of output is
for documentation tests, which we’ll talk about in Chapter 14. For now, note
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

The empty test function passes because any test which doesn’t `panic!` passes,
and any test that does `panic!` fails. Let’s make the test fail by using the
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
`false`, the macro will `panic!`. Let’s run our tests again:

```text
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
test to the value you expect it to be, and check that they’re equal. You can do
this using the `assert!` macro by passing it an expression using the `==`
macro. This is so common, though, that the standard library provides a pair of
macros to do this for convenience: `assert_eq!` and `assert_ne!`. These macros
compare two arguments for equality or inequality, respectively. The other
advantage of using these macros is they will print out what the two values
actually are if the assertion fails so that it’s easier to see *why* the test
failed, whereas the `assert!` macro would just print out that it got a `false`
value for the `==` expression.

Here’s an example test that uses each of these macros and will pass:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert_eq!("Hello", "Hello");

    assert_ne!("Hello", "world");
}
```

You can also specify an optional third argument to each of these macros, which
is a custom message that you’d like to be added to the failure message. The
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

Let’s take a look at a test that will fail becasue `hello` is not equal to
`world`. We’ve also added a custom error message, `greeting operation failed`:

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

The two arguments to `assert_eq!` are named “left” and “right” rather than
“expected” and “actual”; the order of the value that comes from your code and
the value hardcoded into your test isn’t important.

Since these macros use the operators `==` and `!=` and print the values using
debug formatting, the values being compared must implement the `PartialEq` and
`Debug` traits. Types provided by Rust implement these traits, but for structs
and enums that you define, you’ll need to add `PartialEq` in order to be able
to assert that values of those types are equal or not equal and `Debug` in
order to be able to print out the values in the case that the assertion fails.
Because both of these traits are derivable traits that we mentioned in Chapter
5, usually this is as straightforward as adding the `#[derive(PartialEq,
Debug)]` annotation to your struct or enum definition. See Appendix C for more
details about these and other derivable traits.

## Test for failure with `should_panic`

We can invert our test’s failure with another attribute: `should_panic`. This
is useful when we want to test that calling a particular function will cause an
error. For example, let’s test something that we know will panic from Chapter
8: attempting to create a slice using range syntax with byte indices that
aren’t on character boundaries. Add the `#[should_panic]` attribute before the
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

`should_panic` tests can be fragile, as it’s hard to guarantee that the test
didn’t fail for a different reason than the one you were expecting. To help
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
doesn’t match the expected message: cause a `panic!` that happens for a
different reason in this test, or change the expected panic message to
something that doesn’t match the character boundary panic message.

## Running tests

Just like `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. The default behavior of the binary that `cargo test` produces is to run
all the tests in parallel and to capture output generated during test runs so
that it’s easier to read the output about the test results.

The default behavior of running tests can be changed by specifying command line
options. Some of these options can be passed to `cargo test`, and some need to
be passed instead to the resulting test binary. The way to separate these
arguments is with `--`: after `cargo test`, list the arguments that go to
`cargo test`, then the separator `--`, and then the arguments that go to the
test binary.

### Tests Run in Parallel

Tests are run in parallel using threads. For this reason, you should take care
that your tests are written in such a way as to not depend on each other or on
any shared state. Shared state can also include the environment, such as the
current working directory or environment variables.

If you don’t want this behavior, or if you want more fine-grained control over
the number of threads used, you can send the `--test-threads` flag and the
number of threads to the test binary. Setting the number of test threads to 1
means to not use any parallelism:

```text
$ cargo test -- --test-threads=1
```

### Tests Capture Output

By default, Rust’s test library captures and discards output to standard out
and standard error, unless the test fails. For example, if you call `println!`
in a test and the test passes, you won’t see the `println!` output in your
terminal. This behavior can be disabled by sending the `--nocapture` flag to
the test binary:

```text
$ cargo test -- --nocapture
```

### Running a Subset of Tests by Name

Sometimes, running a full test suite can take a long time. If you’re only
working on code in a particular area, you might want to only run the tests
having to do with that code. `cargo test` takes an argument that allows you to
only run certain tests, specified by name.

Let’s create three tests with the following names as shown in Listing 11-3:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn add_two_and_two() {
    assert_eq!(4, 2 + 2);
}

#[test]
fn add_three_and_two() {
    assert_eq!(5, 3 + 2);
}

#[test]
fn one_hundred() {
    assert_eq!(102, 100 + 2);
}
```

<figcaption>

Listing 11-3: Three tests with a variety of names

</figcaption>
</figure>

Running with different arguments will run different subsets of the tests. No
arguments, as we’ve already seen, runs all the tests:

```text
$ cargo test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 3 tests
test add_three_and_two ... ok
test one_hundred ... ok
test add_two_and_two ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

We can pass the name of any test function to run only that test:

```text
$ cargo test one_hundred
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

We can also pass part of a name, and `cargo test` will run all tests that match:

```text
$ cargo test add
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test add_three_and_two ... ok
test add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Module names become part of the test name, so module names can be used in a
similar way to run just the tests for a particular module. For example, if our
code was organized into a module named `adding` and a module named
`subtracting` with tests in each, as in Listing 11-4:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
mod adding {
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, 3 + 2);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, 100 + 2);
    }
}

mod subtracting {
    #[test]
    fn subtract_three_and_two() {
        assert_eq!(1, 3 - 2);
    }
}
```

<figcaption>

Listing 11-4: Tests in two modules named `adding` and `subtracting`

</figcaption>
</figure>

Running `cargo test` will run all of the tests, and the module names will
appear in the test names in the output:

```text
$ cargo test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-d84f1c6cb24adeb4

running 4 tests
test adding::add_two_and_two ... ok
test adding::add_three_and_two ... ok
test subtracting::subtract_three_and_two ... ok
test adding::one_hundred ... ok
```

Running `cargo test adding` would run just the tests in that module and not any
of the tests in the subtracting module:

```text
$ cargo test adding
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-d84f1c6cb24adeb4

running 3 tests
test adding::add_three_and_two ... ok
test adding::one_hundred ... ok
test adding::add_two_and_two ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

### Ignore Some Tests Unless Specifically Requested

Sometimes a few specific tests can be very time-consuming to execute, so during
most runs of `cargo test`, we’d like to exclude them. Instead of having to
construct an argument to `cargo test` to run all tests except these and
remember to use that argument every time, we can annotate these tests with the
`ignore` attribute:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert!(true);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

Now if we run our tests, we’ll see `it_works` is run, but `expensive_test` is
not:

```text
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

We can run only the expensive tests by explicitly asking to run them using
`cargo test -- --ignored`:

```text
$ cargo test -- --ignored
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

This way, most of the time that you run `cargo test` the results would be fast.
When you’re at a point that it makes sense to check the results of the
`ignored` tests and you have time to wait for the results, you can choose to
run `cargo test -- --ignored` instead.

## Test Organization

As mentioned before, testing is a large discipline, and different people
sometimes use different terminology and organization. The Rust community tends
to think about tests in terms of two main categories: *unit tests* and
*integration tests*. Unit tests tend to be smaller and more focused, testing
one module in isolation at a time. They can also test private interfaces.
Integration tests are entirely external to your library. They use your code in
the same way any other code would, using only the public interface and
exercising multiple modules per test. Both kinds of tests are important to
ensure that the pieces of your library are doing what you expect them to
separately and together.

### Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the
rest of the code, in order to be able to quickly pinpoint where code is working
as expected or not. Unit tests live in the *src* directory, in the same files
as the code they are testing. They are separated into their own `tests` module
in each file.

#### The Tests Module and `cfg(test)`

By placing tests in their own module and using the `cfg` annotation on the
module, we can tell Rust to only compile and run the test code when we run
`cargo test`. This saves compile time when we only want to build the library
code with `cargo build`, and saves space in the resulting compiled artifact
since the tests are not included.

Remember when we generated the new `adder` project in the last section? Cargo
generated this code for us:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

We ignored the module stuff so we could concentrate on the mechanics of the
test code inside the module, but now let’s focus on the code surrounding our
tests.

First of all, there’s a new attribute, `cfg`. The `cfg` attribute lets us
declare that something should only be included given a certain *configuration*.
Rust provides the `test` configuration for compiling and running tests. By
using this attribute, Cargo only compiles our test code if we’re currently
trying to run the tests.

Next, the `tests` module holds all of our test functions, while our code is
outside of the `tests` module. The name of the `tests` module is a convention;
otherwise this is a regular module that follows the usual visibility rules we
covered in Chapter 7. Because we’re in an inner module, we need to bring the
code under test into scope. This can be annoying if you have a large module, so
this is a common use of globs.

Up until now in this chapter, we’ve been writing tests in our `adder` project
that don’t actually call any code we’ve written. Let’s change that now! In
*src/lib.rs*, place this `add_two` function and `tests` module that has a test
function to exercise the code, as shown in Listing 11-5:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use add_two;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

<figcaption>

Listing 11-5: Testing the function `add_two` in a child `tests` module

</figcaption>
</figure>

Notice in addition to the test function, we also added `use add_two;` within
the `tests` module. This brings the code we want to test into the scope of the
inner `tests` module, just like we’d need to do for any inner module. If we run
this test now with `cargo test`, it will pass:

```text
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

If we had forgotten to bring the `add_two` function into scope, we would get an
unresolved name error since the `tests` module wouldn’t know anything about the
`add_two` function:

```text
error[E0425]: unresolved name `add_two`
 --> src/lib.rs:9:23
  |
9 |         assert_eq!(4, add_two(2));
  |                       ^^^^^^^ unresolved name
```

If this module contained lots of code we wanted to test, it would be annoying
to list everything in the `use` statement in the tests. It’s common instead to
put `use super::*;` within a module’s `test` submodule in order to bring
everything into the `test` module scope at once.

#### Testing Private Functions

There’s controversy within the testing community about whether you should write
unit tests for private functions or not. Regardless of which testing ideology
you adhere to, Rust does allow you to test private functions due to the way
that the privacy rules work. Consider the code in Listing 11-6 with the private
function `internal_adder`:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use internal_adder;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

<figcaption>

Listing 11-6: Testing a private function

</figcaption>
</figure>

Because tests are just Rust code and the `tests` module is just another module,
we can import and call `internal_adder` in a test just fine. If you don’t think
private functions should be tested, there’s nothing in Rust that will compel
you to do so.

### Integration Tests

In Rust, integration tests are tests that are entirely external to your
library. They use your library in the same way any other code would. Their
purpose is to test that many parts of your library work correctly together.
Units of code that work correctly by themselves could have problems when
integrated, so test coverage of the integrated code is important as well.

#### The *tests* Directory

Cargo has support for integration tests in the *tests* directory. If you make
one and put Rust files inside, Cargo will compile each of the files as an
individual crate. Let’s give it a try!

First, make a *tests* directory at the top level of your project directory,
next to *src*. Then, make a new file, *tests/integration_test.rs*, and put the
code in Listing 11-7 inside:

<figure>
<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

<figcaption>

Listing 11-7: An integration test of a function in the `adder` crate

</figcaption>
</figure>

We now have `extern crate adder` at the top, which we didn’t need in the unit
tests. Each test in the `tests` directory is an entirely separate crate, so we
need to import our library into each of them. This is also why `tests` is a
suitable place to write integration-style tests: they use the library like any
other consumer of it would, by importing the crate and using only the public
API.

We also don’t need a `tests` module in this file. The whole directory won’t be
compiled unless we’re running the tests, so we don’t need to annotate any part
of it with `#[cfg(test)]`. Also, each test file is already isolated into its
own crate, so we don’t need to separate the test code further.

Let’s run the integration tests, which also get run when we run `cargo test`:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
     Running target/debug/deps/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Now we have three sections of output: the unit tests, the integration test, and
the doc tests. Note that adding more unit tests in any *src* file will add more
lines to the unit tests section. Adding more test functions to the integration
test file we created will add more lines to that section. If we add more
integration test *files* in the *tests* directory, there will be more
integration test sections: one for each file.

Specifying a test function name argument with `cargo test` will also match
against test function names in any integration test file. To run all of the
tests in only one particular integration test file, use the `--test` argument
of `cargo test`:

```text
$ cargo test --test integration_test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

#### Submodules in Integration Tests

As you add more integration tests, you may want to make more than one file in
the `tests` directory in order to group the test functions by the functionality
they’re testing, for example. As we mentioned before, that will work fine,
given that Cargo treats every file as its own crate.

Eventually, you may have a set of helper functions that are common to all
integration tests, for example, functions that set up common scenarios. If you
extract these into a file in the *tests* directory, like *tests/common.rs* for
example, this file will be compiled into a separate crate just like the Rust
files in this directory that contain test functions are. There will be a
separate section in the test output for this file. Since this is probably not
what you want, it’s recommended to instead use a *mod.rs* file in a
subdirectory, like *tests/common/mod.rs*, for helper functions. Files in
subdirectories of the *tests* directory do not get compiled as separate crates
or have sections in the test output.

#### Integration Tests for Binary Crates

If your project is a binary crate that only contains a *src/main.rs* and does
not have a *src/lib.rs*, it is not possible to create integration tests in the
*tests* directory and use `extern crate` to import the functions in
*src/main.rs*. This is one of the reasons Rust projects that provide a binary
have a straightforward *src/main.rs* that calls logic that lives in
*src/lib.rs*. With that structure, integration tests *can* test the library
crate by using `extern crate` to cover the important functionality, and if that
works, the small amount of code in *src/main.rs* will work as well and does not
need to be tested.

## Summary

Rust’s testing features provide a way to specify how code should function to
ensure the code continues to work in the specified ways even as we make
changes. Unit tests exercise different parts of a library separately and can
test private implementation details. Integration tests cover the use of many
parts of the library working together, and use the library’s public API to test
the code in the same way other code will use it. Rust’s type system and
ownership rules help prevent some kinds of bugs, but tests are an important
part of reducing logic bugs having to do with how your code is expected to
behave.

Let’s put together the knowledge from this chapter and other previous chapters
and work on a project in the next chapter!
