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

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

The contents of the `src/lib.rs` file in your adder library should be as
follows:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

<span class="caption">Listing 11-1: The test module and function generated
automatically for us by `cargo new`</span>

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

```text
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

<span class="caption">Listing 11-2: The output from running the one
automatically generated test</span>

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

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }
}
```

And run `cargo test` again. In the output, we’ll now see `exploration` instead
of `it_works`:

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-3: Adding a second test; one that will fail
since we call the `panic!` macro</span>

And run the tests again with `cargo test`. The output should look like Listing
11-4, which shows that our `exploration` test passed and `another` failed:

```text
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

<span class="caption">Listing 11-4: Test results when one test passes and one
test fails</span>

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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-5: The `Rectangle` struct and its `can_hold`
method from Chapter 5</span>

The `can_hold` method returns a boolean, which means it’s a perfect use case
for the `assert!` macro. In Listing 11-6, let’s write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a length of 8 and
a width of 7, and asserting that it can hold another `Rectangle` instance that
has a length of 5 and a width of 1:

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-6: A test for `can_hold` that checks that a
larger rectangle indeed holds a smaller rectangle</span>

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

```text
running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

It does pass! Let’s add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:

<span class="filename">Filename: src/lib.rs</span>

```rust
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

```text
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Two passing tests! Now let’s see what happens to our test results if we
introduce a bug in our code. Let’s change the implementation of the `can_hold`
method to have a less-than sign when it compares the lengths where it’s
supposed to have a greater-than sign:

```rust
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

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-7: Testing the function `add_two` using the
`assert_eq!` macro</span>

Let’s check that it passes!

```text
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

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

And run the tests again:

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

Running this test produces:

```text
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

```rust,ignore
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

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-8: Testing that a condition will cause a
`panic!`</span>

The `#[should_panic]` attribute goes after the `#[test]` attribute and before
the test function it applies to. Let’s see what it looks like when this test
passes:

```text
running 1 test
test tests::greater_than_100 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Looks good! Now let’s introduce a bug in our code, by removing the condition
that the `new` function will panic if the value is greater than 100:

```rust
# struct Guess {
#     value: u32,
# }
#
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

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-9: Testing that a condition will cause a
`panic!` with a particular panic message</span>

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

```rust,ignore
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {}.", value);
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {}.", value);
}
```

This time when we run the `should_panic` test, it will fail:

```text
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
