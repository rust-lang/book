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
test code inside the module, but now let's focus on the code surrounding our
tests.

First of all, there's a new attribute, `cfg`. The `cfg` attribute lets us
declare that something should only be included given a certain *configuration*.
Rust provides the `test` configuration for compiling and running tests. By
using this attribute, Cargo only compiles our test code if we're currently
trying to run the tests.

Next, the `tests` module holds all of our test functions, while our code is
outside of the `tests` module. The name of the `tests` module is a convention;
otherwise this is a regular module that follows the usual visibility rules we
covered in Chapter 7. Because we're in an inner module, we need to bring the
code under test into scope. This can be annoying if you have a large module, so
this is a common use of globs.

Up until now in this chapter, we've been writing tests in our `adder` project
that don't actually call any code we've written. Let's change that now! In
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
unresolved name error since the `tests` module wouldn't know anything about the
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
we can import and call `internal_adder` in a test just fine. If you don't think
private functions should be tested, there's nothing in Rust that will compel
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
individual crate. Let's give it a try!

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
     Running target/debug/deps/adder-abcabcabc

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
