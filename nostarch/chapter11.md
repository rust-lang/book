
[TOC]

<!--I've suggested adding a few more listing numbers and captions throughout
--- don't worry about fixing the numbering here, I can do that when I covert it
from markdown -->

# Testing

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
>
> Edsger W. Dijkstra, "The Humble Programmer" (1972)

<!--Can you pinpoint what you mean by correctness in this context, I feel like
it could be interpreted in different ways -- you mean correct outcomes? -->

Rust is a programming language that cares a lot about correctness, but
correctness is a complex topic and isn't easy to assert. Rust's type system
shoulders a huge part of this burden, helping to ensure that our programs do
what we intend, but the type system cannot handle everything. As such, Rust
includes support for writing software tests within the language itself.

As an example, say we write a function called `add_two` that adds two to a
number input by the user. This function's signature accepts an integer as an
argument and returns an integer as a result. When we implement and compile that
function, Rust will do all the type checking and borrow checking that we've
seen so far, but Rust *can't* check that this function will do precisely what
we intend: return the argument plus two, rather than say the argument plus 10
or the argument minus 50! That's where tests come in.

We can write tests that pass `3` to the `add_two` function and check that we
get `5` back, and run the tests whenever we make changes to our code to make
sure any existing correcting behavior has not changed.

Testing is a complex skill, and we cannot hope to cover everything about how to
write good tests in one chapter of a book, so here we'll just discuss the
mechanics of Rust's testing facilities. We'll talk about the annotations and
macros available to you when writing your tests, the default behavior and
options provided for running your tests, and how to organize tests into unit
tests and integration tests.

## Building Accurate Tests

<!-- seems like we aren't actually writing a test here, but using the template
from a generated test? -->

Tests are merely Rust functions that verify non-test code is functioning in the
program in the expected manner.

<!--Everything we've discussed about Rust code applies to Rust tests as well!
-- This seems vague, do you want to expand? Otherwise, we might just cut and
discuss it when we get to it? -->

Let's first look at the features Rust provides specifically for writing tests:
the `test` attribute, a few macros, and the `should_panic` attribute.

### The Anatomy of a Test

At its simplest, a test in Rust is merely a normal function that's annotated
with the `test` attribute. When you make a new library project with Cargo, a
test function is automatically generated. We'll use that as the template to
examine tests more closely, and then build on it to make a more accurate test.
Let's create a new library project called `adder`:

<!-- Can you lay out briefly what the test atrribute is/does, what it brings to
the normal function that makes it a test? Does it just tell Rust to run the
function when cargo test is called? -->

<!-- is it annotated with `test` by the user, or only automatically? I think
it's the latter, and has edited with a more active tone to make that clear, but
please change if I'm wrong -->

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

Cargo will automatically generate a simple test. The contents of the
`src/lib.rs` file in your adder library should be as follows:

<!--Did we see an automatically generated test in an earlier chapter, briefly?
It rings a bell. If so, we should cross ref here -->

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

Listing 11-1: An automatically generated test

For now, let's ignore the top two lines and focus on the function to see how
this works. Note the `#[test]` annotation before the `fn` line: this attribute
indicates this is a test function. The function currently has no body, which
means there is no code to fail the test; that's good enough to pass!

<!-- I'm not following, good enough to pass what? You mean an empty function
will pass the test? -->

You run a test with the `cargo test` command. Run the test in Listing 11-2 now:

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

Listing 11-2: Running the adder test

Cargo compiled and ran our test. There are two sets of output here; for this
chapter we're only interested in the first set. The second set of output is for
documentation tests, which we'll talk about in Chapter 14. For now, note this
line:

<!-- what is the first test output, if the second is for documentation tests?
Why aren't we interested in documentation tests? A brief line before we move on
would be useful -->

```text
test it_works ... ok
```

<!-- I might suggest changing the name of the function, could be misconstrued
as part of the test output! -->

The `it_works` text is the name of our function, and the `ok` tells us the test
passed.

We also get a summary line that tells us the aggregate results of all the tests
that we have:

```text
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

<!--I don't think we actually go into what `measured` means in this chapter,
right? Do you want to add a brief line here, so it's covered? -->

We'll look at the `ignored` and `measured` parts later, but for now this line
tells us that one test passed and no tests failed. The function is empty and
there is no functionality to test, but it passes because any test that doesn't
`panic!` counts as a pass.

### Adding Condition with the `assert!` Macro

<!--What does assert actually do, what do we use it for? Can you lay that out
up front? I've read on a bit and added some sample text here, can you check,
update? -->

The general `#test` function is useful for checking whether a function works
overall, but it tell us much about the conditions a function will work in. To
test that, we can use the `assert!` macro to add conditions and confirm whether
the function we're testing is providing the functionality we intend.

Let's make the test fail using the `assert!` macro to see it in action:

Filename: src/lib.rs

```rust
#[test]
fn it_works() {
    assert!(false);
}
```

<!-- what kind of thing can be passed as an argument? Presumably when we use it
for real we won't pass it `true` or `false` as an argument, but some condition
that will evaluate to true or false? In which case, should below be phrased "If
the argument evaluates to true" and an exaplanation of that? Or maybe even a
working example would be better, this could be misleading -->

The `assert!` macro is provided by the standard library, and it takes one
argument. If the argument is `true`, nothing happens and the test passes. If
the argument is `false`, the macro will `panic!` and fail the `#[test]`. Let's
run our tests again:

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

PROD: add wingdings and delete repeated lines here

And shows that the test failed because the `assert!` macro in `src/lib.rs` on
line 5 got a `false` value:

```text
thread 'it_works' panicked at 'assertion failed: false', src/lib.rs:5
```

The test failure is also reflected in the summary line:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

<!-- why did this fail--because we told it to fail with the argument? I'm not
sure I follow what this is actually useful for, at the moment -->

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros

A common way to test functionality is to compare the result of the code under
test to the value you expect it to output, to check that they're equal. You
could do this using the `assert!` macro and passing it an expression with the
`==` macro. However, this is such a common test that the standard library
provides a pair of macros to perform this test more efficiently: `assert_eq!`
and `assert_ne!`. These macros compare two arguments for equality or
inequality, respectively. They'll also print out the two values if the
assertion fails, so that it's easier to see *why* the test failed, while the
`assert!` macro can only tell that it got a `false` value for the `==`
expression and not why.

The example test uses each of these macros and both will pass and so will
return `ok` values:

Filename: src/lib.rs

```rust
#[test]
fn it_works() {
    assert_eq!("Hello", "Hello");

    assert_ne!("Hello", "world");
}
```

<!-- Can you talk this through a little --- what will they see to know that it
has passed, two `true`s in the output, or an OK like earlier? -->

If we expand the logic of the macros, we can see the operators they use to
perform these comaprisons, shown in Listing 11-X.

```rust,ignore
// assert_eq! - panic if the values aren't equal
if left_val != right_val {
    panic!(
        "assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`): {}"
        left_val,
        right_val,
        <!--optional_custom_message-->
    )
}

// assert_ne! - panic if the values are equal
if left_val == right_val {
    panic!(
        "assertion failed: `(left != right)` (left: `{:?}`, right: `{:?}`): {}"
        left_val,
        right_val,
        <!--optional_custom_message-->
    )
}
```

Listing 11-X: The `assert_eq!` and `assert_ne!` macros expanded out

You can also add a custom message to be printed with the failure message as an
optional third argument. Let's take a look at an `assert_ne!` test that will
fail because `hello` is not equal to `world`. We've added the custom error
message, `greeting operation failed`:

Filename: src/lib.rs

<!--Below -- I'm not sure what you mean by "this value would come from running
your code", you mean from code that the test is testing? -->

```rust
#[test]
fn a_simple_case() {
    let result = "hello"; // this value would come from running your code
    assert_eq!(result, "world", "greeting operation failed");
}
```

<!-- can you point out the new code that adds the message, to make sure they've
seen how that works? Can you also just talk about the code in general? This
seems significantly different to the first tests in this sections, where the
two objects to be compared were on the same line -->

Running this indeed fails, and the output explains why it failed and includes
the custom error message we specified:

```text
---- a_simple_case stdout ----
	thread 'a_simple_case' panicked at 'assertion failed: `(left == right)`
    (left: `"hello"`, right: `"world"`): greeting operation failed',
    src/main.rs:4
```

The two arguments to `assert_eq!` are named "left" and "right", though the
order that you pass them to the macro---the order of the value that comes from
your code and the value hardcoded into your test---isn't important because they
are being tested against each other.

Under the surface these macros use the operators `==` and `!=` and print the
values using debug formatting, which means the values being compared must
implement the `PartialEq` and `Debug` traits. All types that are provided by
Rust implement these traits by default, but for structs and enums that you
define, you'll need to add `PartialEq` in order to be able to assert that
values of those types are equal or not equal and `Debug` in order to be able to
print out the values in the case that the assertion fails. Because both of
these traits are derivable traits, as we mentioned in Chapter 5, this is
usually as straightforward as adding the `#[derive(PartialEq, Debug)]`
annotation to your struct or enum definition. See Appendix C for more details
about these and other derivable traits.

## Test for Failure with `should_panic`

We can use another attribute, `should_panic`, to test specifically whether a
particular function will cause an error. As an example, let's test something
that we know will panic from Chapter

<!--In what kind of circumstances would you want to test for a panic over
testing for succes?-->

<!--PROD for CE: Do we have a listing cross-ref here?-->

8: attempting to create a slice using range syntax with byte indices that
aren't on character boundaries. Add the `#[should_panic]` attribute before the
function along with the `#[test]` attribute, as shown in Listing 11-1:

Filename: src/lib.rs

```rust
#[test]
#[should_panic]
fn slice_not_on_char_boundaries() {
    let s = "Здравствуйте";
    &s[0..1];
}
```

Listing 11-1: A test expecting a `panic!`

<!--- what do success and failure look like in this test? -->

We told the test that we expect it to panic with `#[should_panic]` and it does,
so this test succeeds. If we run this code and it does not cause a `panic!`,
this test would fail.

`should_panic` tests can be vague, however, because they only tell you that the
code will cause a panic and not the reason for the panic, so you don't know
that your code isn't panicking for a reason different to the one you were
expecting. To narrow this down, you can add an optional `expected` parameter to
the `should_panic` attribute. The test harness will make sure that the failure
message contains the provided text. A more robust version of Listing 11-1 would
be the following, in Listing 11-2:

Filename: src/lib.rs

```rust
#[test]
#[should_panic(expected = "do not lie on character boundary")]
fn slice_not_on_char_boundaries() {
    let s = "Здравствуйте";
    &s[0..1];
}
```

<!-- I will add ghosting in libreoffice /Carol -->

Listing 11-2: A test expecting a `panic!` with a particular message

<!-- So is this checking that the error message matches the expected panic
error message? Does that mean you need to know the error message you can expect
to get precisely? Can you describe what should/does happen, and what this test
actually does, I'm not quite following -->

Try on your own to see what happens when a `should_panic` test panics but
doesn't match the expected message: cause a `panic!` that happens for a
different reason in this test, or change the expected panic message to
something that doesn't match the character boundary panic message.

## Controlling How Tests are Run

Just as `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. There are certain options you can add and certain default behavior you
can change when running a test, to have more control over how tests are run and
displayed. For example, default behavior of the binary produced by `cargo test`
runs all the tests in parallel and captures output generated during test runs,
preventing it from being displayed to make it easier to read the output related
to the test results. You can change this default behavior by specifying command
line options.

Some command line options can be passed to `cargo test`, and some need to be
passed instead to the resulting test binary. To separate these two types of
arguments, you list the arguments that go to `cargo test`, then the separator
`--`, and then the arguments that go to the test binary.

<!--- how will they know which are to go to cargo test and which to the test
binary? -->

### Running Tests in Parallel or Consecutively

<!-- Are we safe assuming the reader will know enough about threads in this
context? -->

When multiple tests are run, by dafault they run in parallel using threads. For
this reason, you should take care that your tests do not depend on each other
or on any shared state, including a shared environment such as the current
working directory or environment variables.

<!-- why does the tests being run using threads mean you have to take care in
this way, what would happen otherwise? They would easily be invalidated? -->

<!--Below: line originally read "If you don't want this behavior" -- I wasn't
sure what behavior we meant, if we don't want to run tests in parallel? -->

If you don't want to run the tests in parallel, or if you want more
fine-grained control over the number of threads used, you can send the
`--test-threads` flag and the number of threads you intend to use to the test
binary. For example:

```text
$ cargo test -- --test-threads=1
```

We set the number of test threads to 1, telling the program not to use any
parallelism.

<!-- So does this mean that if you have multiple tests, they will instead run
one by one? What is the benefit of running tests one-by-one over parallel, and
vice versa? -->

### Showing Function Output

<!-- by "capture output" do you mean prevent the function output from being
displayed? I think so, and have been editing as such, but wanted to confirm. -->

By default, if a test passes, Rust's test library captures output, returning
only the test results. For example, if you call `println!` in a test and the
test passes, you won't see the `println!` output in your terminal but only
whether the test passes or fails. This behavior can be disabled by sending the
`--nocapture` flag to the test binary:

```text
$ cargo test -- --nocapture
```

<!-- and what would the result be now, a test that passes shows the results and
the normal function output? -->

### Running a Subset of Tests by Name

Sometimes, running a full test suite can take a long time. If you're working on
code in a particular area, you might want to run only the tests pertaining to
that code. You can choose which tests to run by passing `cargo test` the name
or names of the test/s you want to run as an argument.

To show you how to run a subset of tests, we'll create three tests as shown in
Listing 11-3 and choose which ones to run:

Filename: src/lib.rs

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

Listing 11-3: Three tests with a variety of names

If we run the tests without passing any arguments, as we've already seen, all
the tests will run in parallel:

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

Running `cargo test` with different arguments will run different subsets of the tests

#### Running Single Tests

We can pass the name of any test function to run only that test:

```text
$ cargo test one_hundred
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

You can add more than one name to run multiple chosen tests.

<!--Above, is this true? Or you can only pass one? -->

#### Filtering to Run Certain Tests

We can also pass part of a name, and `cargo test` will run all tests with names
that contain that part:

```text
$ cargo test add
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test add_three_and_two ... ok
test add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

This ran all tests with `add` in the name.

#### Running Tests in a Module

You can pass module names in a similar way to run just the tests for a
particular module. For example, say our code was organized into two modules
named `adding` and `subtracting` with tests in each, as in Listing 11-4:

Filename: src/lib.rs

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

Listing 11-4: Tests in two modules named `adding` and `subtracting`

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

Running `cargo test adding` would run just the tests in the `adding` module and
none of the tests in the subtracting module:

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

This is of most benefit when

<!-- in what kind of situation might you need to run only some tests, when you
have lots and lots in a program? -->

### Ignore Some Tests Unless Specifically Requested

Sometimes a few specific tests can be very time-consuming to execute, so you
might want to exclude them during most runs of `cargo test`. Rather than
listing as arguments all tests you do want to run, we can instead annotate the
time consuming tests with the `ignore` attribute to exclude them:

Filename: src/lib.rs

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

We add the `#[ignore]` line to the test we want to exlcude, after `#[test]`.
Now if we run our tests, we'll see `it_works` runs, but `expensive_test` does
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

`expensive_test` is listed as `ignored`. If we want to run the expensive test
on its own, we can negate the `ignore` command by adding a `-- --ignored`
command when we run the test:

<!-- what does the double `-- --` mean? That seems interesting -->

<!-- is that right, this way the program knows to run only the test with
`ignore` if we add this, or it knows to run all tests? -->

```text
$ cargo test -- --ignored
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

By controlling which tests run, you can make sure your `cargo test` results
will be fast. When you're at a point that it makes sense to check the results
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

Both kinds of tests are important to ensure that the pieces of your library are
doing what you expect them to separately and together.

### Unit Tests

The purpose of unit tests is to test each unit of code in isolation from the
rest of the code, in order to be able to quickly pinpoint where code is and is
not working as expected. Unit tests live in the *src* directory, separated out
into their own `tests` module in the same files as the code they are testing.

<!-- above--as in, they are automatically generated there, or this is where
they must live in order to test the correct code? Passive tone, I'm not sure if
this is user-done or Rust-done -->

#### The Tests Module and `cfg(test)`

<!-- Can you connect this to unit tests, at the moment I can't see that it's
unit-test-specific? -->

The `cfg` annotation on the tests module tells Rust to compile and run the test
code only when we run `cargo test`, and not when we run `cargo build`. This
saves compile time when we only want to build the library, and saves space in
the resulting compiled artifact since the tests are not included.

Remember that when we generated the new `adder` project in the last section,
Cargo generated this code for us:

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

This is the automatically generated test module. Earlier we ignored the module
stuff so we could concentrate on the mechanics of the test code inside the
module, but now let's focus on the code surrounding our tests.

The new attribute `cfg` stands for *configruation*, and tells Rust that the
following object should only be included given a certain configuration. In this
case, the configuration is `test`, provided by Rust for compiling and running
tests. By using this attribute, Cargo only compiles our test code if we
actively try to run the tests with `cargo test`.

<!-- Hm, if the `[cfg (test)]` code is what makes it only run when we run cargo
test, I'm even more unclear on what [test] on its own does! I think that needs
clarifying earlier on. Also, I'm still unsure how this section pertains only to
unit tests, this seems like information that woul dbe useful right at the start
of the whole chatper? -->

The `tests` module holds all of the test functions, while the main code of the
program is outside of the `tests` module. The name of the `tests` module is a
convention, but apart from that this is a regular module that follows the usual
visibility rules we covered in Chapter 7. Because we're in an inner module, we
need to bring the code under test into scope, commonly done with use of globs.

Up until now in this chapter, we've been writing tests in our `adder` project
that don't actually call any code we've written. Let's change that now! In
*src/lib.rs*, place the `add_two` function and `tests` module, that has a test
function to exercise the code, from Listing 11-5.

Filename: src/lib.rs

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

Listing 11-5: Testing the function `add_two` in a child `tests` module

Notice in addition to the test function, we also added `use add_two;` within
the `tests` module. This brings the code we want to test into the scope of the
inner `tests` module, just like we'd need to do for any inner module. If we run
this test now with `cargo test`, it will pass:

```text
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

If you forget to bring the `add_two` function into scope, you'll get an
unresolved name error, since the `tests` module wouldn't know anything about
the `add_two` function:

```text
error[E0425]: unresolved name `add_two`
 --> src/lib.rs:9:23
  |
9 |         assert_eq!(4, add_two(2));
  |                       ^^^^^^^ unresolved name
```

However, this method could get tedious if the module contained lots of code you
want to test, so it's common instead to put `use super::*;` within a module's
`test` submodule in order to bring everything into the `test` module scope at
once.

#### Testing Private Functions

There's debate within the testing community about whether unit tests should be
used for private functions or not. Regardless of which testing ideology you
adhere to, Rust's privacy rules do allow you to test private functions.
Consider the code in Listing 11-6 with the private function `internal_adder`:

Filename: src/lib.rs

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

Listing 11-6: Testing a private function

<!-- I'm not clear on why we would assume this might not be fine, why are we
highlighting this specifically? -->

Because tests are just Rust code and the `tests` module is just another module,
we can import and call `internal_adder` in a test just fine. If you don't think
private functions should be tested, there's nothing in Rust that will compel
you to do so.

### Integration Tests

In Rust, integration tests are entirely external to your library, and use your
library in the same way any other code would. Their purpose is to test that
many parts of your library work correctly together. Units of code that work
correctly by themselves could have problems when integrated, so test coverage
of the integrated code is important as well. To apply integration tests you
first need a test directory.

#### The *tests* Directory

To perform integration tests on your code, you need to make a *tests* directory
and put the Rust files you want to test together inside, and Cargo will compile
each of the files as an individual crate. Let's give it a try!

First, make a *tests* directory at the top level of your project directory,
next to *src*. Then, make a new file, *tests/integration_test.rs*, and enter
the code in Listing 11-7.

Filename: tests/integration_test.rs

```rust,ignore
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

Listing 11-7: An integration test of a function in the `adder` crate

<!-- I'm not clear on what's special about the tests directory, it seems like a
normal user-made directory so far, is that right? If so, perhaps we should make
that clear, and make it clear that it's suitable because it's an external
directory and so treats the code like any consumer -->

We've added `extern crate adder` at the top, which we didn't need in the unit
tests. This is because each test in the `tests` directory is an entirely
separate crate, so we need to import our library into each of them. This is
also why the `tests` directory is a suitable place to write integration-style
tests: they use the library like any other consumer of it would, by importing
the crate and using only the public API.

We don't need to annotate any part of `tests` with `#[cfg(test)]`, as the whole
directory will only be compiled if we're running the tests. Each test file is
also already isolated into its own crate, so we don't need to separate the test
code further.

The integration tests are run when we run `cargo test` along with any other
tests, so we'll use that now:

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

<!-- what are the doc tests? How do we tell the difference between unit and
integration tests here? -->

Now we have three sections of output: the unit tests, the integration test, and
the doc tests. Note that adding more unit tests in any *src* file will add more
lines to the unit tests section. Adding more test functions to the integration
test file we created will add more lines to the integration test section, but
each file gets its own section, so if we add more integration test *files* in
the *tests* directory, there will be more integration test sections.

You can run a particular integration test file by specifying the test
function's name as an argument with `cargo test`. To run all of the tests in
only one particular integration test file, use the `--test` argument of `cargo
test` followed by the name of the file:

```text
$ cargo test --test integration_test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

This tests only the file you specify within the directory.

#### Submodules in Integration Tests

As you add more integration tests, you may want to make more than one file in
the `tests` directory to help organize them; for example, to group the test
functions by the functionality they're testing. This will work fine because
Cargo treats every file as its own crate.

Once you have several tests, you may have a set of helper functions that are
common to all integration tests: for example, functions that set up common
scenarios. If you extract these helper functions into a file in the *tests*
directory, like *tests/common.rs* for example, this file will be compiled into
a separate crate just like the test function files in this directory. The test
output will then have a separate section for this file. Since this is probably
not what you want,

<!-- why is this what you want? If you don't want this, why extract these
functions into a separate file in the first place? I think this section needs a
little more info, to help the reader take away what's useful here -->

we recommended you instead create a *mod.rs* file within a subdirectory, like
*tests/common/mod.rs*, for helper functions. Files in subdirectories of the
*tests* directory do not get compiled as separate crates or have sections in
the test output.

<!-- so what would the result look like?-->

#### Integration Tests for Binary Crates

If your project is a binary crate that only contains a *src/main.rs* and does
not have a *src/lib.rs* file, you aren't able to create integration tests in
the *tests* directory and use `extern crate` to import the functions in
*src/main.rs*.

<!-- can you say why this isn't possible, explicitly? -->

This is one of the reasons Rust projects that provide a binary have a
straightforward *src/main.rs* that calls logic that lives in *src/lib.rs*. With
that structure, integration tests *can* test the library crate by using `extern
crate` to cover the important functionality, and if that works, the small
amount of code in *src/main.rs* will work as well and does not need to be
tested.

## Summary

Rust's testing features provide a way to specify how code should function to
ensure it continues to work as we expect even as we make changes. Unit tests
exercise different parts of a library separately and can test private
implementation details. Integration tests cover the use of many parts of the
library working together, and use the library's public API to test the code in
the same way external code will use it. Rust's type system and ownership rules
help prevent some kinds of bugs, but tests are an important part of reducing
logic bugs having to do with how your code is expected to behave.

Let's put together the knowledge from this chapter and other previous chapters
and work on a project in the next chapter!
