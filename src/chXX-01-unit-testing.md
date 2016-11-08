# Unit testing

As we mentioned before, testing is a large discipline, and so different people
can sometimes use different terminology. For our purposes, we tend to place
tests into two main categories: *unit tests* and *integration tests*. Unit
tests tend to be smaller, and more focused. In Rust, they can also test
non-public interfaces. Let's talk more about how to do unit testing in Rust.

## The tests module and `cfg(test)`

Remember when we generated our new project in the last section? Cargo had
generated some stuff for us:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

We deleted the module stuff so we could learn more about the mechanics of
tests. But there's a reason that Cargo generated this module for us: it's the
idiomatic way to organize unit tests in Rust. That is, unit tests are:

* Stored inside of the same tree as your source code.
* Placed inside their own module.

For a more realistic example of how this works, consider our `add_two` function
from before:

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

First of all, there's a new attribute, `cfg`. The `cfg` attribute lets us
declare that something should only be included given a certain configuration.
Rust provides the `test` configuration when compiling and running tests. By
using this attribute, Cargo only compiles our test code if we're currently
trying to run the tests. Given that they're not compiled at all during a
regular `cargo build`, this can save compile time. It also ensures that our
tests are entirely left out of the binary, saving space in a non-testing
context.

You'll notice one more change: the `use` declaration. The `tests` module is
only a convention, it's nothing that Rust understands directly. As such, we
have to follow the usual visibility rules. Because we're in an inner module,
we need to bring our test function into scope. This can be annoying if you have
a large module, and so this is a common use of globs. Let's change our
`src/lib.rs` to make use of it:

```rust,ignore
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

Note the different `use` line. Now we run our tests:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

It works!

## Testing internal functions

There's controversy within the testing community about unit testing private
functions. Regardless of which testing ideology you adhere to, Rust does allow
you to test them, due to the way that the privacy rules work. Consider this:

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

In this scenario, we have a non-`pub` function, `internal_adder`. Because tests
are just Rust code, and the `tests` module is just another module, we can
import and call `internal_adder` in a test just fine.

## Running a subset of tests

Sometimes, running a full test suite can take a long time. `cargo test` takes
an argument that allows you to only run certain tests, if you'd prefer to do that.
Let's say we had two tests of `add_two`:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use add_two;

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

Running with different arguments will run different subsets of the tests.
No arguments, as we've already seen, runs all the tests:

```text
$ cargo test
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/lol-06a75b4a1f2515e9

running 3 tests
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok
test tests::add_two_and_two ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

We can pass the name of any test function to run only that test:

```text
$ cargo test one_hundred
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/lol-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

We can also pass part of a name, and `cargo test` will run all tests
that match:

```text
$ cargo test add
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/lol-06a75b4a1f2515e9

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lol

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
