## How to Write Tests

Tests are Rust functions that verify that the non-test code is functioning in
the expected manner. The bodies of test functions typically perform these three
actions:

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

Let’s look at the features Rust provides specifically for writing tests that
take these actions, which include the `test` attribute, a few macros, and the
`should_panic` attribute.

### The Anatomy of a Test Function

At its simplest, a test in Rust is a function that’s annotated with the `test`
attribute. Attributes are metadata about pieces of Rust code; one example is
the `derive` attribute we used with structs in Chapter 5. To change a function
into a test function, add `#[test]` on the line before `fn`. When you run your
tests with the `cargo test` command, Rust builds a test runner binary that runs
the functions annotated with the `test` attribute and reports on whether each
test function passes or fails.

When we make a new library project with Cargo, a test module with a test
function in it is automatically generated for us. This module helps you start
writing your tests so you don’t have to look up the exact structure and syntax
of test functions every time you start a new project. You can add as many
additional test functions and as many test modules as you want!

We’ll explore some aspects of how tests work by experimenting with the template
test generated for us without actually testing any code. Then we’ll write some
real-world tests that call some code that we’ve written and assert that its
behavior is correct.

Let’s create a new library project called `adder`:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

The contents of the *src/lib.rs* file in your `adder` library should look like
Listing 11-1.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

<span class="caption">Listing 11-1: The test module and function generated
automatically by `cargo new`</span>

For now, let’s ignore the top two lines and focus on the function to see how it
works. Note the `#[test]` annotation before the `fn` line: this attribute
indicates this is a test function, so the test runner knows to treat this
function as a test. We could also have non-test functions in the `tests` module
to help set up common scenarios or perform common operations, so we need to
indicate which functions are tests by using the `#[test]` attribute.

The function body uses the `assert_eq!` macro to assert that 2 + 2 equals 4.
This assertion serves as an example of the format for a typical test. Let’s run
it to see that this test passes.

The `cargo test` command runs all tests in our project, as shown in Listing
11-2.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

<span class="caption">Listing 11-2: The output from running the automatically
generated test</span>

Cargo compiled and ran the test. After the `Compiling`, `Finished`, and
`Running` lines is the line `running 1 test`. The next line shows the name
of the generated test function, called `it_works`, and the result of running
that test, `ok`. The overall summary of running the tests appears next. The
text `test result: ok.` means that all the tests passed, and the portion that
reads `1 passed; 0 failed` totals the number of tests that passed or failed.

Because we don’t have any tests we’ve marked as ignored, the summary shows `0
ignored`. We also haven’t filtered the tests being run, so the end of the
summary shows `0 filtered out`. We’ll talk about ignoring and filtering out
tests in the next section, [“Controlling How Tests Are
Run.”][controlling-how-tests-are-run]<!-- ignore -->

The `0 measured` statistic is for benchmark tests that measure performance.
Benchmark tests are, as of this writing, only available in nightly Rust. See
[the documentation about benchmark tests][bench] to learn more.

[bench]: ../unstable-book/library-features/test.html

The next part of the test output, which starts with `Doc-tests adder`, is for
the results of any documentation tests. We don’t have any documentation tests
yet, but Rust can compile any code examples that appear in our API
documentation. This feature helps us keep our docs and our code in sync! We’ll
discuss how to write documentation tests in the [“Documentation Comments as
Tests”][doc-comments]<!-- ignore --> section of Chapter 14. For now, we’ll
ignore the `Doc-tests` output.

Let’s change the name of our test to see how that changes the test output.
Change the `it_works` function to a different name, such as `exploration`, like
so:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Then run `cargo test` again. The output now shows `exploration` instead of
`it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Let’s add another test, but this time we’ll make a test that fails! Tests fail
when something in the test function panics. Each test is run in a new thread,
and when the main thread sees that a test thread has died, the test is marked
as failed. We talked about the simplest way to cause a panic in Chapter 9,
which is to call the `panic!` macro. Enter the new test, `another`, so your
*src/lib.rs* file looks like Listing 11-3.

<span class="filename">Filename: src/lib.rs</span>

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs:here}}
```

<span class="caption">Listing 11-3: Adding a second test that will fail because
we call the `panic!` macro</span>

Run the tests again using `cargo test`. The output should look like Listing
11-4, which shows that our `exploration` test passed and `another` failed.

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

<span class="caption">Listing 11-4: Test results when one test passes and one
test fails</span>

Instead of `ok`, the line `test tests::another` shows `FAILED`. Two new
sections appear between the individual results and the summary: the first
section displays the detailed reason for each test failure. In this case,
`another` failed because it `panicked at 'Make this test fail'`, which happened
on line 10 in the *src/lib.rs* file. The next section lists just the names of
all the failing tests, which is useful when there are lots of tests and lots of
detailed failing test output. We can use the name of a failing test to run just
that test to more easily debug it; we’ll talk more about ways to run tests in
the [“Controlling How Tests Are Run”][controlling-how-tests-are-run]<!-- ignore
--> section.

The summary line displays at the end: overall, our test result is `FAILED`.
We had one test pass and one test fail.

Now that you’ve seen what the test results look like in different scenarios,
let’s look at some macros other than `panic!` that are useful in tests.

### Checking Results with the `assert!` Macro

The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`. We give the
`assert!` macro an argument that evaluates to a Boolean. If the value is
`true`, `assert!` does nothing and the test passes. If the value is `false`,
the `assert!` macro calls the `panic!` macro, which causes the test to fail.
Using the `assert!` macro helps us check that our code is functioning in the
way we intend.

In Chapter 5, Listing 5-15, we used a `Rectangle` struct and a `can_hold`
method, which are repeated here in Listing 11-5. Let’s put this code in the
*src/lib.rs* file and write some tests for it using the `assert!` macro.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs:here}}
```

<span class="caption">Listing 11-5: Using the `Rectangle` struct and its
`can_hold` method from Chapter 5</span>

The `can_hold` method returns a Boolean, which means it’s a perfect use case
for the `assert!` macro. In Listing 11-6, we write a test that exercises the
`can_hold` method by creating a `Rectangle` instance that has a width of 8 and
a height of 7 and asserting that it can hold another `Rectangle` instance that
has a width of 5 and a height of 1.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

<span class="caption">Listing 11-6: A test for `can_hold` that checks whether a
larger rectangle can indeed hold a smaller rectangle</span>

Note that we’ve added a new line inside the `tests` module: `use super::*;`.
The `tests` module is a regular module that follows the usual visibility rules
we covered in Chapter 7 in the [“Paths for Referring to an Item in the Module
Tree”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->
section. Because the `tests` module is an inner module, we need to bring the
code under test in the outer module into the scope of the inner module. We use
a glob here so anything we define in the outer module is available to this
`tests` module.

We’ve named our test `larger_can_hold_smaller`, and we’ve created the two
`Rectangle` instances that we need. Then we called the `assert!` macro and
passed it the result of calling `larger.can_hold(&smaller)`. This expression
is supposed to return `true`, so our test should pass. Let’s find out!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

It does pass! Let’s add another test, this time asserting that a smaller
rectangle cannot hold a larger rectangle:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Because the correct result of the `can_hold` function in this case is `false`,
we need to negate that result before we pass it to the `assert!` macro. As a
result, our test will pass if `can_hold` returns `false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Two tests that pass! Now let’s see what happens to our test results when we
introduce a bug in our code. Let’s change the implementation of the `can_hold`
method by replacing the greater than sign with a less than sign when it
compares the widths:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Running the tests now produces the following:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Our tests caught the bug! Because `larger.width` is 8 and `smaller.width` is
5, the comparison of the widths in `can_hold` now returns `false`: 8 is not
less than 5.

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

A common way to test functionality is to compare the result of the code under
test to the value you expect the code to return to make sure they’re equal. You
could do this using the `assert!` macro and passing it an expression using the
`==` operator. However, this is such a common test that the standard library
provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test
more conveniently. These macros compare two arguments for equality or
inequality, respectively. They’ll also print the two values if the assertion
fails, which makes it easier to see *why* the test failed; conversely, the
`assert!` macro only indicates that it got a `false` value for the `==`
expression, not the values that led to the `false` value.

In Listing 11-7, we write a function named `add_two` that adds `2` to its
parameter and returns the result. Then we test this function using the
`assert_eq!` macro.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

<span class="caption">Listing 11-7: Testing the function `add_two` using the
`assert_eq!` macro</span>

Let’s check that it passes!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

The first argument we gave to the `assert_eq!` macro, `4`, is equal to the
result of calling `add_two(2)`. The line for this test is `test
tests::it_adds_two ... ok`, and the `ok` text indicates that our test passed!

Let’s introduce a bug into our code to see what it looks like when a test that
uses `assert_eq!` fails. Change the implementation of the `add_two` function to
instead add `3`:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Run the tests again:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Our test caught the bug! The `it_adds_two` test failed, displaying the message
`` assertion failed: `(left == right)` `` and showing that `left` was `4` and
`right` was `5`. This message is useful and helps us start debugging: it means
the `left` argument to `assert_eq!` was `4` but the `right` argument, where we
had `add_two(2)`, was `5`.

Note that in some languages and test frameworks, the parameters to the
functions that assert two values are equal are called `expected` and `actual`,
and the order in which we specify the arguments matters. However, in Rust,
they’re called `left` and `right`, and the order in which we specify the value
we expect and the value that the code under test produces doesn’t matter. We
could write the assertion in this test as `assert_eq!(add_two(2), 4)`, which
would result in a failure message that displays `` assertion failed: `(left ==
right)` `` and that `left` was `5` and `right` was `4`.

The `assert_ne!` macro will pass if the two values we give it are not equal and
fail if they’re equal. This macro is most useful for cases when we’re not sure
what a value *will* be, but we know what the value definitely *won’t* be if our
code is functioning as we intend. For example, if we’re testing a function that
is guaranteed to change its input in some way, but the way in which the input
is changed depends on the day of the week that we run our tests, the best thing
to assert might be that the output of the function is not equal to the input.

Under the surface, the `assert_eq!` and `assert_ne!` macros use the operators
`==` and `!=`, respectively. When the assertions fail, these macros print their
arguments using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All the primitive types and most
of the standard library types implement these traits. For structs and enums
that you define, you’ll need to implement `PartialEq` to assert that values of
those types are equal or not equal. You’ll need to implement `Debug` to print
the values when the assertion fails. Because both traits are derivable traits,
as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward
as adding the `#[derive(PartialEq, Debug)]` annotation to your struct or enum
definition. See Appendix C, [“Derivable Traits,”][derivable-traits]<!-- ignore
--> for more details about these and other derivable traits.

### Adding Custom Failure Messages

You can also add a custom message to be printed with the failure message as
optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any
arguments specified after the one required argument to `assert!` or the two
required arguments to `assert_eq!` and `assert_ne!` are passed along to the
`format!` macro (discussed in Chapter 8 in the [“Concatenation with the `+`
Operator or the `format!`
Macro”][concatenation-with-the--operator-or-the-format-macro]<!-- ignore -->
section), so you can pass a format string that contains `{}` placeholders and
values to go in those placeholders. Custom messages are useful to document
what an assertion means; when a test fails, you’ll have a better idea of what
the problem is with the code.

For example, let’s say we have a function that greets people by name and we
want to test that the name we pass into the function appears in the output:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

The requirements for this program haven’t been agreed upon yet, and we’re
pretty sure the `Hello` text at the beginning of the greeting will change. We
decided we don’t want to have to update the test when the requirements change,
so instead of checking for exact equality to the value returned from the
`greeting` function, we’ll just assert that the output contains the text of the
input parameter.

Let’s introduce a bug into this code by changing `greeting` to not include
`name` to see what this test failure looks like:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Running this test produces the following:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

This result just indicates that the assertion failed and which line the
assertion is on. A more useful failure message in this case would print the
value we got from the `greeting` function. Let’s change the test function,
giving it a custom failure message made from a format string with a placeholder
filled in with the actual value we got from the `greeting` function:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Now when we run the test, we’ll get a more informative error message:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

We can see the value we actually got in the test output, which would help us
debug what happened instead of what we were expecting to happen.

### Checking for Panics with `should_panic`

In addition to checking that our code returns the correct values we expect,
it’s also important to check that our code handles error conditions as we
expect. For example, consider the `Guess` type that we created in Chapter 9,
Listing 9-13. Other code that uses `Guess` depends on the guarantee that `Guess`
instances will contain only values between 1 and 100. We can write a test that
ensures that attempting to create a `Guess` instance with a value outside that
range panics.

We do this by adding another attribute, `should_panic`, to our test function.
This attribute makes a test pass if the code inside the function panics; the
test will fail if the code inside the function doesn’t panic.

Listing 11-8 shows a test that checks that the error conditions of `Guess::new`
happen when we expect them to.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

<span class="caption">Listing 11-8: Testing that a condition will cause a
`panic!`</span>

We place the `#[should_panic]` attribute after the `#[test]` attribute and
before the test function it applies to. Let’s look at the result when this test
passes:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Looks good! Now let’s introduce a bug in our code by removing the condition
that the `new` function will panic if the value is greater than 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

When we run the test in Listing 11-8, it will fail:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

We don’t get a very helpful message in this case, but when we look at the test
function, we see that it’s annotated with `#[should_panic]`. The failure we got
means that the code in the test function did not cause a panic.

Tests that use `should_panic` can be imprecise because they only indicate that
the code has caused some panic. A `should_panic` test would pass even if the
test panics for a different reason from the one we were expecting to happen. To
make `should_panic` tests more precise, we can add an optional `expected`
parameter to the `should_panic` attribute. The test harness will make sure that
the failure message contains the provided text. For example, consider the
modified code for `Guess` in Listing 11-9 where the `new` function panics with
different messages depending on whether the value is too small or too large.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

<span class="caption">Listing 11-9: Testing that a condition will cause a
`panic!` with a particular panic message</span>

This test will pass because the value we put in the `should_panic` attribute’s
`expected` parameter is a substring of the message that the `Guess::new`
function panics with. We could have specified the entire panic message that we
expect, which in this case would be `Guess value must be less than or equal to
100, got 200.` What you choose to specify in the expected parameter for
`should_panic` depends on how much of the panic message is unique or dynamic
and how precise you want your test to be. In this case, a substring of the
panic message is enough to ensure that the code in the test function executes
the `else if value > 100` case.

To see what happens when a `should_panic` test with an `expected` message
fails, let’s again introduce a bug into our code by swapping the bodies of the
`if value < 1` and the `else if value > 100` blocks:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

This time when we run the `should_panic` test, it will fail:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

The failure message indicates that this test did indeed panic as we expected,
but the panic message did not include the expected string `'Guess value must be
less than or equal to 100'`. The panic message that we did get in this case was
`Guess value must be greater than or equal to 1, got 200.` Now we can start
figuring out where our bug is!

### Using `Result<T, E>` in Tests

So far, we’ve written tests that panic when they fail. We can also write tests
that use `Result<T, E>`! Here’s the test from Listing 11-1, rewritten to use
`Result<T, E>` and return an `Err` instead of panicking:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs}}
```

The `it_works` function now has a return type, `Result<(), String>`. In the
body of the function, rather than calling the `assert_eq!` macro, we return
`Ok(())` when the test passes and an `Err` with a `String` inside when the test
fails.

Writing tests so they return a `Result<T, E>` enables you to use the question
mark operator in the body of tests, which can be a convenient way to write
tests that should fail if any operation within them returns an `Err` variant.

You can’t use the `#[should_panic]` annotation on tests that use `Result<T,
E>`. To assert that an operation returns an `Err` variant, *don’t* use the
question mark operator on the `Result<T, E>` value. Instead, use
`assert!(value.is_err())`.

Now that you know several ways to write tests, let’s look at what is happening
when we run our tests and explore the different options we can use with `cargo
test`.

[concatenation-with-the--operator-or-the-format-macro]:
ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro
[controlling-how-tests-are-run]:
ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
