
[TOC]

# Writing Automated Tests

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
> Edsger W. Dijkstra, “The Humble Programmer” (1972)

Correctness in our programs means that our code does what we intend for it to
do. Rust is a programming language that cares a lot about correctness, but
correctness is a complex topic and isn’t easy to prove. Rust’s type system
shoulders a huge part of this burden, but the type system cannot catch every
kind of incorrectness. As such, Rust includes support for writing software
tests within the language itself.

As an example, say we write a function called `add_two` that adds two to
whatever number is passed to it. This function’s signature accepts an integer
as a parameter and returns an integer as a result. When we implement and
compile that function, Rust will do all the type checking and borrow checking
that we’ve seen so far to make sure that, for instance, we aren’t passing a
`String` value or an invalid reference to this function. What Rust *can’t*
check is that this function will do precisely what we intend: return the
parameter plus two, rather than, say, the parameter plus 10 or the parameter
minus 50! That’s where tests come in.

We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, we get `5` back. We can run these tests whenever we make
changes to our code to make sure any existing correct behavior has not changed.

Testing is a complex skill, and we cannot hope to cover everything about how to
write good tests in one chapter of a book, so here we’ll just discuss the
mechanics of Rust’s testing facilities. We’ll talk about the annotations and
macros available to you when writing your tests, the default behavior and
options provided for running your tests, and how to organize tests into unit
tests and integration tests.

## How to Write Tests

Tests are Rust functions that verify that the non-test code is functioning in
the expected manner. The bodies of test functions typically perform some setup,
run the code we want to test, then assert whether the results are what we
expect. Let’s look at the features Rust provides specifically for writing
tests: the `test` attribute, a few macros, and the `should_panic` attribute.

### The Anatomy of a Test Function

At its simplest, a test in Rust is a function that’s annotated with the `test`
attribute. Attributes are metadata about pieces of Rust code: the `derive`
attribute that we used with structs in Chapter 5 is one example. To make a
function into a test function, we add `#[test]` on the line before `fn`. When
we run our tests with the `cargo test` command, Rust will build a test runner
binary that runs the functions annotated with the `test` attribute and reports
on whether each test function passes or fails.

We saw in Chapter 7 that when you make a new library project with Cargo, a test
module with a test function in it is automatically generated for us. This is to
help us get started writing our tests so we don’t have to go look up the
exact structure and syntax of test functions every time we start a new project.
We can add as many additional test functions and as many test modules as we
want, though!

We’re going to explore some aspects of how tests work by experimenting with the
template test generated for us, without actually testing any code. Then we’ll
write some real-world tests that call some code that we’ve written and assert
that its behavior is correct.

Let’s create a new library project called `adder`:

```
$ cargo new adder
     Created library `adder` project
$ cd adder
```

The contents of the `src/lib.rs` file in your adder library should be as
follows:

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

Listing 11-1: The test module and function generated automatically for us by
`cargo new`

For now, let’s ignore the top two lines and focus on the function to see how it
works. Note the `#[test]` annotation before the `fn` line: this attribute
indicates this is a test function, so that the test runner knows to treat this
function as a test. We could also have non-test functions in the `tests` module
to help set up common scenarios or perform common operations, so we need to
indicate which functions are tests with the `#[test]` attribute.

The function currently has no body, which means there is no code to fail the
test; an empty test is a passing test! Let’s run it and see that this test
passes.

The `cargo test` command runs all tests we have in our project, as shown in
Listing 11-2:

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Listing 11-2: The output from running the one automatically generated test

Cargo compiled and ran our test. After the `Compiling`, `Finished`, and
`Running` lines, we see the line `running 1 test`. The next line shows the name
of the generated test function, called `it_works`, and the result of running
that test, `ok`. Then we see the overall summary of running the tests: `test
result: ok.` means all the tests passed. `1 passed; 0 failed` adds up the
number of tests that passed or failed.

We don’t have any tests we’ve marked as ignored, so the summary says `0
ignored`. We’re going to talk about ignoring tests in the next section on
different ways to run tests. The `0 measured` statistic is for benchmark tests
that measure performance. Benchmark tests are, as of this writing, only
available in nightly Rust. See Appendix D for more information about nightly
Rust.

The next part of the test output that starts with `Doc-tests adder` is for the
results of any documentation tests. We don’t have any documentation tests yet,
but Rust can compile any code examples that appear in our API documentation.
This feature helps us keep our docs and our code in sync! We’ll be talking
about how to write documentation tests in the “Documentation Comments” section
of Chapter 14. We’re going to ignore the `Doc-tests` output for now.

Let’s change the name of our test and see how that changes the test output.
Give the `it_works` function a different name, such as `exploration`, like so:

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }
}
```

And run `cargo test` again. In the output, we’ll now see `exploration` instead
of `it_works`:

```
running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Let’s add another test, but this time we’ll make a test that fails! Tests fail
when something in the test function panics. Each test is run in a new thread,
and when the main thread sees that a test thread has died, the test is marked
as failed. We talked about the simplest way to cause a panic in Chapter 9: call
the `panic!` macro! Type in the new test so that your `src/lib.rs` now looks
like Listing 11-3:

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

Listing 11-3: Adding a second test; one that will fail since we call the
`panic!` macro

And run the tests again with `cargo test`. The output should look like Listing
11-4, which shows that our `exploration` test passed and `another` failed:

```
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
	thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

Listing 11-4: Test results when one test passes and one test fails

Instead of `ok`, the line `test tests::another` says `FAILED`. We have two new
sections between the individual results and the summary: the first section
displays the detailed reason for the test failures. In this case, `another`
failed because it `panicked at 'Make this test fail'`, which happened on
*src/lib.rs* line 9. The next section lists just the names of all the failing
tests, which is useful when there are lots of tests and lots of detailed
failing test output. We can use the name of a failing test to run just that
test in order to more easily debug it; we’ll talk more about ways to run tests
in the next section.

Finally, we have the summary line: overall, our test result is `FAILED`. We had
1 test pass and 1 test fail.

Now that we’ve seen what the test results look like in different scenarios,
let’s look at some macros other than `panic!` that are useful in tests.

### Checking Results with the `assert!` Macro

The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`. We give the
`assert!` macro an argument that evaluates to a boolean. If the value is `true`,
`assert!` does nothing and the test passes. If the value is `false`, `assert!`
calls the `panic!` macro, which causes the test to fail. This is one macro that
helps us check that our code is functioning in the way we intend.

Remember all the way back in Chapter 5, Listing 5-9, where we had a `Rectangle`
struct and a `can_hold` method, repeated here in Listing 11-5. Let’s put this
code in *src/lib.rs* instead of *src/main.rs* and write some tests for it using
the `assert!` macro.

Filename: src/lib.rs

```
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

Listing 11-5: The `Rectangle` struct and its `can_hold` method from Chapter 5

The `can_hold` method returns a boolean, which means it’s a perfect use case
for the `assert!` macro. In Listing 11-6, let’s write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a length of 8 and
a width of 7, and asserting that it can hold another `Rectangle` instance that
has a length of 5 and a width of 1:

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

Listing 11-6: A test for `can_hold` that checks that a larger rectangle indeed
holds a smaller rectangle

Note that we’ve added a new line inside the `tests` module: `use super::*;`.
The `tests` module is a regular module that follows the usual visibility rules
we covered in Chapter 7. Because we’re in an inner module, we need to bring the
code under test in the outer module into the scope of the inner module. We’ve
chosen to use a glob here so that anything we define in the outer module is
available to this `tests` module.

We’ve named our test `larger_can_hold_smaller`, and we’ve created the two
`Rectangle` instances that we need. Then we called the `assert!` macro and
passed it the result of calling `larger.can_hold(&smaller)`. This expression is
supposed to return `true`, so our test should pass. Let’s find out!

```
running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

It does pass! Let’s add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
```

Because the correct result of the `can_hold` function in this case is `false`,
we need to negate that result before we pass it to the `assert!` macro. This
way, our test will pass if `can_hold` returns `false`:

```
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Two passing tests! Now let’s see what happens to our test results if we
introduce a bug in our code. Let’s change the implementation of the `can_hold`
method to have a less-than sign when it compares the lengths where it’s
supposed to have a greater-than sign:

```
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}
```

Running the tests now produces:

```
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
	thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Our tests caught the bug! Since `larger.length` is 8 and `smaller.length` is 5,
the comparison of the lengths in `can_hold` now returns `false` since 8 is not
less than 5.

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

A common way to test functionality is to take the result of the code under test
and the value we expect the code to return and check that they’re equal. We
could do this using the `assert!` macro and passing it an expression using the
`==` operator. However, this is such a common test that the standard library
provides a pair of macros to perform this test more conveniently: `assert_eq!`
and `assert_ne!`. These macros compare two arguments for equality or
inequality, respectively. They’ll also print out the two values if the
assertion fails, so that it’s easier to see *why* the test failed, while the
`assert!` macro only tells us that it got a `false` value for the `==`
expression, not the values that lead to the `false` value.

In Listing 11-7, let’s write a function named `add_two` that adds two to its
parameter and returns the result. Then let’s test this function using the
`assert_eq!` macro:

Filename: src/lib.rs

```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

Listing 11-7: Testing the function `add_two` using the `assert_eq!` macro

Let’s check that it passes!

```
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

The first argument we gave to the `assert_eq!` macro, 4, is equal to the result
of calling `add_two(2)`. We see a line for this test that says `test
tests::it_adds_two ... ok`, and the `ok` text indicates that our test passed!

Let’s introduce a bug into our code to see what it looks like when a test that
uses `assert_eq!` fails. Change the implementation of the `add_two` function to
instead add 3:

```
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

And run the tests again:

```
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
	thread 'tests::it_adds_two' panicked at 'assertion failed: `(left ==
    right)` (left: `4`, right: `5`)', src/lib.rs:11
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

Our test caught the bug! The `it_adds_two` test failed with the message ``
assertion failed: `(left == right)` (left: `4`, right: `5`) ``. This message is
useful and helps us get started debugging: it says the `left` argument to
`assert_eq!` was 4, but the `right` argument, where we had `add_two(2)`, was 5.

Note that in some languages and test frameworks, the parameters to the
functions that assert two values are equal are called `expected` and `actual`
and the order in which we specify the arguments matters. However, in Rust,
they’re called `left` and `right` instead, and the order in which we specify
the value we expect and the value that the code under test produces doesn’t
matter. We could write the assertion in this test as
`assert_eq!(add_two(2), 4)`, which would result in a failure message that says
`` assertion failed: `(left == right)` (left: `5`, right: `4`) ``.

The `assert_ne!` macro will pass if the two values we give to it are not equal
and fail if they are equal. This macro is most useful for cases when we’re not
sure exactly what a value *will* be, but we know what the value definitely
*won’t* be, if our code is functioning as we intend. For example, if we have a
function that is guaranteed to change its input in some way, but the way in
which the input is changed depends on the day of the week that we run our
tests, the best thing to assert might be that the output of the function is not
equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All of the primitive types and
most of the standard library types implement these traits. For structs and
enums that you define, you’ll need to implement `PartialEq` in order to be able
to assert that values of those types are equal or not equal. You’ll need to
implement `Debug` in order to be able to print out the values in the case that
the assertion fails. Because both of these traits are derivable traits, as we
mentioned in Chapter 5, this is usually as straightforward as adding the
`#[derive(PartialEq, Debug)]` annotation to your struct or enum definition. See
Appendix C for more details about these and other derivable traits.

### Custom Failure Messages

We can also add a custom message to be printed with the failure message as
optional arguments to `assert!`, `assert_eq!`, and `assert_ne!`. Any arguments
specified after the one required argument to `assert!` or the two required
arguments to `assert_eq!` and `assert_ne!` are passed along to the `format!`
macro that we talked about in Chapter 8, so you can pass a format string that
contains `{}` placeholders and values to go in the placeholders. Custom
messages are useful in order to document what an assertion means, so that when
the test fails, we have a better idea of what the problem is with the code.

For example, let’s say we have a function that greets people by name, and we
want to test that the name we pass into the function appears in the output:

Filename: src/lib.rs

```
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

The requirements for this program haven’t been agreed upon yet, and we’re
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don’t want to have to update the test for the name when that
happens, so instead of checking for exact equality to the value returned from
the `greeting` function, we’re just going to assert that the output contains
the text of the input parameter.

Let’s introduce a bug into this code to see what this test failure looks like,
by changing `greeting` to not include `name`:

```
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

Running this test produces:

```
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'assertion failed:
    result.contains("Carol")', src/lib.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::greeting_contains_name
```

This just tells us that the assertion failed and which line the assertion is
on. A more useful failure message in this case would print the value we did get
from the `greeting` function. Let’s change the test function to have a custom
failure message made from a format string with a placeholder filled in with the
actual value we got from the `greeting` function:

```
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

Now if we run the test again, we’ll get a much more informative error message:

```
---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain
    name, value was `Hello`', src/lib.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.

### Checking for Panics with `should_panic`

In addition to checking that our code returns the correct values we expect,
it’s also important to check that our code handles error conditions as we
expect. For example, consider the `Guess` type that we created in Chapter 9 in
Listing 9-8. Other code that uses `Guess` is depending on the guarantee that
`Guess` instances will only contain values between 1 and 100. We can write a
test that ensures that attempting to create a `Guess` instance with a value
outside that range panics.

We can do this by adding another attribute, `should_panic`, to our test
function. This attribute makes a test pass if the code inside the function
panics, and the test will fail if the code inside the function doesn’t panic.

Listing 11-8 shows how we’d write a test that checks the error conditions of
`Guess::new` happen when we expect:

Filename: src/lib.rs

```
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

Listing 11-8: Testing that a condition will cause a `panic!`

The `#[should_panic]` attribute goes after the `#[test]` attribute and before
the test function it applies to. Let’s see what it looks like when this test
passes:

```
running 1 test
test tests::greater_than_100 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Looks good! Now let’s introduce a bug in our code, by removing the condition
that the `new` function will panic if the value is greater than 100:

```
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}
```

If we run the test from Listing 11-8, it will fail:

```
running 1 test
test tests::greater_than_100 ... FAILED

failures:

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

We don’t get a very helpful message in this case, but once we look at the test
function, we can see that it’s annotated with `#[should_panic]`. The failure we
got means that the code in the function, `Guess::new(200)`, did not cause a
panic.

`should_panic` tests can be imprecise, however, because they only tell us that
the code has caused some panic. A `should_panic` test would pass even if the
test panics for a different reason than the one we were expecting to happen. To
make `should_panic` tests more precise, we can add an optional `expected`
parameter to the `should_panic` attribute. The test harness will make sure that
the failure message contains the provided text. For example, consider the
modified code for `Guess` in Listing 11-9 where the `new` function panics with
different messages depending on whether the value was too small or too large:

Filename: src/lib.rs

```
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

Listing 11-9: Testing that a condition will cause a `panic!` with a particular
panic message

This test will pass, because the value we put in the `expected` parameter of
the `should_panic` attribute is a substring of the message that the
`Guess::new` function panics with. We could have specified the whole panic
message that we expect, which in this case would be `Guess value must be less
than or equal to 100, got 200.` It depends on how much of the panic message is
unique or dynamic and how precise you want your test to be. In this case, a
substring of the panic message is enough to ensure that the code in the
function that gets run is the `else if value > 100` case.

To see what happens when a `should_panic` test with an `expected` message
fails, let’s again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:

```
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {}.", value);
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {}.", value);
}
```

This time when we run the `should_panic` test, it will fail:

```
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
	thread 'tests::greater_than_100' panicked at 'Guess value must be greater
    than or equal to 1, got 200.', src/lib.rs:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: Panic did not include expected string 'Guess value must be less than or
equal to 100'

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

The failure message indicates that this test did indeed panic as we expected,
but the panic message `did not include expected string 'Guess value must be
less than or equal to 100'`. We can see the panic message that we did get,
which in this case was `Guess value must be greater than or equal to 1, got
200.` We could then start figuring out where our bug was!

Now that we’ve gone over ways to write tests, let’s look at what is happening
when we run our tests and talk about the different options we can use with
`cargo test`.

## Controlling How Tests are Run

Just as `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. There are options you can use to change the default behavior of `cargo
test`. For example, the default behavior of the binary produced by `cargo test`
is to run all the tests in parallel and capture output generated during test
runs, preventing it from being displayed to make it easier to read the output
related to the test results. You can change this default behavior by specifying
command line options.

Some command line options can be passed to `cargo test`, and some need to be
passed instead to the resulting test binary. To separate these two types of
arguments, you list the arguments that go to `cargo test`, then the separator
`--`, and then the arguments that go to the test binary. Running `cargo test
--help` will tell you about the options that go with `cargo test`, and running
`cargo test -- --help` will tell you about the options that go after the
separator `--`.

### Running Tests in Parallel or Consecutively

When multiple tests are run, by default they run in parallel using threads.
This means the tests will finish running faster, so that we can get faster
feedback on whether or not our code is working. Since the tests are running at
the same time, you should take care that your tests do not depend on each other
or on any shared state, including a shared environment such as the current
working directory or environment variables.

For example, say each of your tests runs some code that creates a file on disk
named `test-output.txt` and writes some data to that file. Then each test reads
the data in that file and asserts that the file contains a particular value,
which is different in each test. Because the tests are all run at the same
time, one test might overwrite the file between when another test writes and
reads the file. The second test will then fail, not because the code is
incorrect, but because the tests have interfered with each other while running
in parallel. One solution would be to make sure each test writes to a different
file; another solution is to run the tests one at a time.

If you don’t want to run the tests in parallel, or if you want more
fine-grained control over the number of threads used, you can send the
`--test-threads` flag and the number of threads you want to use to the test
binary. For example:

```
$ cargo test -- --test-threads=1
```

We set the number of test threads to 1, telling the program not to use any
parallelism. This will take longer than running them in parallel, but the tests
won’t be potentially interfering with each other if they share state.

### Showing Function Output

By default, if a test passes, Rust’s test library captures anything printed to
standard output. For example, if we call `println!` in a test and the test
passes, we won’t see the `println!` output in the terminal: we’ll only see the
line that says the test passed. If a test fails, we’ll see whatever was printed
to standard output with the rest of the failure message.

For example, Listing 11-10 has a silly function that prints out the value of
its parameter and then returns 10. We then have a test that passes and a test
that fails:

Filename: src/lib.rs

```
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

Listing 11-10: Tests for a function that calls `println!`

The output we’ll see when we run these tests with `cargo test` is:

```
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
	I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left ==
right)` (left: `5`, right: `10`)', src/lib.rs:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Note that nowhere in this output do we see `I got the value 4`, which is what
gets printed when the test that passes runs. That output has been captured. The
output from the test that failed, `I got the value 8`, appears in the section
of the test summary output that also shows the cause of the test failure.

If we want to be able to see printed values for passing tests as well, the
output capture behavior can be disabled by using the `--nocapture` flag:

```
$ cargo test -- --nocapture
```

Running the tests from Listing 11-10 again with the `--nocapture` flag now
shows:

```
running 2 tests
I got the value 4
I got the value 8
test tests::this_test_will_pass ... ok
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left ==
right)` (left: `5`, right: `10`)', src/lib.rs:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
test tests::this_test_will_fail ... FAILED

failures:

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Note that the output for the tests and the test results is interleaved; this is
because the tests are running in parallel as we talked about in the previous
section. Try using both the `--test-threads=1` option and the `--nocapture`
function and see what the output looks like then!

### Running a Subset of Tests by Name

Sometimes, running a full test suite can take a long time. If you’re working on
code in a particular area, you might want to run only the tests pertaining to
that code. You can choose which tests to run by passing `cargo test` the name
or names of the test(s) you want to run as an argument.

To demonstrate how to run a subset of tests, we’ll create three tests for our
`add_two` function as shown in Listing 11-11 and choose which ones to run:

Filename: src/lib.rs

```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

Listing 11-11: Three tests with a variety of names

If we run the tests without passing any arguments, as we’ve already seen, all
the tests will run in parallel:

```
running 3 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

#### Running Single Tests

We can pass the name of any test function to `cargo test` to run only that test:

```
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

We can’t specify the names of multiple tests in this way, only the first value
given to `cargo test` will be used.

#### Filtering to Run Multiple Tests

However, we can specify part of a test name, and any test whose name matches
that value will get run. For example, since two of our tests’ names contain
`add`, we can run those two by running `cargo test add`:

```
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

This ran all tests with `add` in the name. Also note that the module in which
tests appear becomes part of the test’s name, so we can run all the tests in a
module by filtering on the module’s name.

### Ignore Some Tests Unless Specifically Requested

Sometimes a few specific tests can be very time-consuming to execute, so you
might want to exclude them during most runs of `cargo test`. Rather than
listing as arguments all tests you do want to run, we can instead annotate the
time consuming tests with the `ignore` attribute to exclude them:

Filename: src/lib.rs

```
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

We add the `#[ignore]` line to the test we want to exclude, after `#[test]`.
Now if we run our tests, we’ll see `it_works` runs, but `expensive_test` does
not:

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

`expensive_test` is listed as `ignored`. If we want to run only the ignored
tests, we can ask for them to be run with `cargo test -- --ignored`:

```
$ cargo test -- --ignored
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

By controlling which tests run, you can make sure your `cargo test` results
will be fast. When you’re at a point that it makes sense to check the results
of the `ignored` tests and you have time to wait for the results, you can
choose to run `cargo test -- --ignored` instead.

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

Filename: src/lib.rs

```
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

Filename: src/lib.rs

```
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

Listing 11-12: Testing a private function

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

Filename: tests/integration_test.rs

```
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

Listing 11-13: An integration test of a function in the `adder` crate

We’ve added `extern crate adder` at the top, which we didn’t need in the unit
tests. This is because each test in the `tests` directory is an entirely
separate crate, so we need to import our library into each of them. Integration
tests use the library like any other consumer of it would, by importing the
crate and using only the public API.

We don’t need to annotate any code in *tests/integration_test.rs* with
`#[cfg(test)]`. Cargo treats the `tests` directory specially and will only
compile files in this directory if we run `cargo test`. Let’s try running
`cargo test` now:

```
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

```
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

Filename: tests/common.rs

```
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

If we run the tests again, we’ll see a new section in the test output for the
*common.rs* file, even though this file doesn’t contain any test functions, nor
are we calling the `setup` function from anywhere:

```
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

Filename: tests/integration_test.rs

```
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
