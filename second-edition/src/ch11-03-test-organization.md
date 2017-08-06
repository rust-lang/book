## Test Organization

As mentioned at the start of the chapter, testing is a large discipline, and
different people use different terminology and organization. The Rust community
tends to think about tests in terms of two main categories: *unit tests* and
*integration tests*. Unit tests are smaller and more focused, testing one
module in isolation at a time, and can test private interfaces. Integration
tests are entirely external to your library, and use your code in the same way
any other external code would, using only the public interface and exercising
multiple modules per test.

Writing both kinds of tests is important to ensure that the pieces of your
library are doing what you expect them to separately and together.

### Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the
rest of the code, in order to be able to quickly pinpoint where code is and is
not working as expected. We put unit tests in the *src* directory, in each file
with the code that they’re testing. The convention is that we create a module
named `tests` in each file to contain the test functions, and we annotate the
module with `cfg(test)`.

#### The Tests Module and `#[cfg(test)]`

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run
the test code only when we run `cargo test`, and not when we run `cargo build`.
This saves compile time when we only want to build the library, and saves space
in the resulting compiled artifact since the tests are not included. We’ll see
that since integration tests go in a different directory, they don’t need the
`#[cfg(test)]` annotation. Because unit tests go in the same files as the code,
though, we use `#[cfg(test)]`to specify that they should not be included in the
compiled result.

Remember that when we generated the new `adder` project in the first section of
this chapter, Cargo generated this code for us:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

This is the automatically generated test module. The attribute `cfg` stands for
*configuration*, and tells Rust that the following item should only be included
given a certain configuration option. In this case, the configuration option is
`test`, provided by Rust for compiling and running tests. By using this
attribute, Cargo only compiles our test code if we actively run the tests with
`cargo test`. This includes any helper functions that might be within this
module, in addition to the functions annotated with `#[test]`.

#### Testing Private Functions

There’s debate within the testing community about whether private functions
should be tested directly or not, and other languages make it difficult or
impossible to test private functions. Regardless of which testing ideology you
adhere to, Rust’s privacy rules do allow you to test private functions.
Consider the code in Listing 11-12 with the private function `internal_adder`:

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
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

<span class="caption">Listing 11-12: Testing a private function</span>

Note that the `internal_adder` function is not marked as `pub`, but because
tests are just Rust code and the `tests` module is just another module, we can
import and call `internal_adder` in a test just fine. If you don’t think
private functions should be tested, there’s nothing in Rust that will compel
you to do so.

### Integration Tests

In Rust, integration tests are entirely external to your library. They use your
library in the same way any other code would, which means they can only call
functions that are part of your library’s public API. Their purpose is to test
that many parts of your library work correctly together. Units of code that
work correctly by themselves could have problems when integrated, so test
coverage of the integrated code is important as well. To create integration
tests, you first need a *tests* directory.

#### The *tests* Directory

To write integration tests for our code, we need to make a *tests* directory at
the top level of our project directory, next to *src*. Cargo knows to look for
integration test files in this directory. We can then make as many test files
as we’d like in this directory, and Cargo will compile each of the files as an
individual crate.

Let’s give it a try! Keep the code from Listing 11-12 in *src/lib.rs*. Make a
*tests* directory, then make a new file named *tests/integration_test.rs*, and
enter the code in Listing 11-13.

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

<span class="caption">Listing 11-13: An integration test of a function in the
`adder` crate</span>

We’ve added `extern crate adder` at the top, which we didn’t need in the unit
tests. This is because each test in the `tests` directory is an entirely
separate crate, so we need to import our library into each of them. Integration
tests use the library like any other consumer of it would, by importing the
crate and using only the public API.

We don’t need to annotate any code in *tests/integration_test.rs* with
`#[cfg(test)]`. Cargo treats the `tests` directory specially and will only
compile files in this directory if we run `cargo test`. Let’s try running
`cargo test` now:

```text
cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/integration_test-ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Now we have three sections of output: the unit tests, the integration test, and
the doc tests. The first section for the unit tests is the same as we have been
seeing: one line for each unit test (we have one named `internal` that we added
in Listing 11-12), then a summary line for the unit tests.

The integration tests section starts with the line that says `Running
target/debug/deps/integration-test-ce99bcc2479f4607` (the hash at the end of
your output will be different). Then there’s a line for each test function in
that integration test, and a summary line for the results of the integration
test just before the `Doc-tests adder` section starts.

Note that adding more unit test functions in any *src* file will add more test
result lines to the unit tests section. Adding more test functions to the
integration test file we created will add more lines to the integration test
section. Each integration test file gets its own section, so if we add more
files in the *tests* directory, there will be more integration test sections.

We can still run a particular integration test function by specifying the test
function’s name as an argument to `cargo test`. To run all of the tests in a
particular integration test file, use the `--test` argument of `cargo test`
followed by the name of the file:

```text
$ cargo test --test integration_test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

This tests only the file that we specified from the *tests* directory.

#### Submodules in Integration Tests

As you add more integration tests, you may want to make more than one file in
the *tests* directory to help organize them; for example, to group the test
functions by the functionality they’re testing. As we mentioned, each file in
the *tests* directory is compiled as its own separate crate.

Treating each integration test file as its own crate is useful to create
separate scopes that are more like the way end users will be using your crate.
However, this means files in the *tests* directory don’t share the same
behavior as files in *src* do that we learned about in Chapter 7 regarding how
to separate code into modules and files.

The different behavior of files in the *tests* directory is usually most
noticeable if you have a set of helper functions that would be useful in
multiple integration test files, and you try to follow the steps from Chapter 7
to extract them into a common module. For example, if we create
*tests/common.rs* and place this function named `setup` in it, where we could
put some code that we want to be able to call from multiple test functions in
multiple test files:

<span class="filename">Filename: tests/common.rs</span>

```rust
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

If we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions, nor
are we calling the `setup` function from anywhere:

```text
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Having `common` show up in the test results with `running 0 tests` displayed
for it is not what we wanted; we just wanted to be able to share some code with
the other integration test files.

In order to not have `common` show up in the test output, we need to use the
other method of extracting code into a file that we learned about in Chapter 7:
instead of creating *tests/common.rs*, we’ll create *tests/common/mod.rs*. When
we move the `setup` function code into *tests/common/mod.rs* and get rid of the
*tests/common.rs* file, the section in the test output will no longer show up.
Files in subdirectories of the *tests* directory do not get compiled as
separate crates or have sections in the test output.

Once we have *tests/common/mod.rs*, we can use it from any of the integration
test files as a module. Here’s an example of calling the `setup` function from
the `it_adds_two` test in *tests/integration_test.rs*:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

Note the `mod common;` declaration is the same as the module declarations we
did in Chapter 7. Then in the test function, we can call the `common::setup()`
function.

#### Integration Tests for Binary Crates

If our project is a binary crate that only contains a *src/main.rs* and does
not have a *src/lib.rs*, we aren’t able to create integration tests in the
*tests* directory and use `extern crate` to import functions defined in
*src/main.rs*. Only library crates expose functions that other crates are able
to call and use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* that calls logic that lives in *src/lib.rs*. With
that structure, integration tests *can* test the library crate by using `extern
crate` to cover the important functionality. If the important functionality
works, the small amount of code in *src/main.rs* will work as well, and that
small amount of code does not need to be tested.

## Summary

Rust’s testing features provide a way to specify how code should function to
ensure it continues to work as we expect even as we make changes. Unit tests
exercise different parts of a library separately and can test private
implementation details. Integration tests cover the use of many parts of the
library working together, and they use the library’s public API to test the
code in the same way external code will use it. Even though Rust’s type system
and ownership rules help prevent some kinds of bugs, tests are still important
to help reduce logic bugs having to do with how your code is expected to behave.

Let’s put together the knowledge from this chapter and other previous chapters
and work on a project in the next chapter!
