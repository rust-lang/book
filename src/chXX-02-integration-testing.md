# Integration testing

In the last section, we talked about unit tests. But there's still that other
category: integration testing. In Rust, an integration test is a test that is
entirely external to your library. It uses it in the same way any other code
would.

Cargo has support for integration tests through the `tests` directory. If you
make one, and put `.rs` files inside, Cargo will compile each of them as an
individual crate. Let's give it a try! First, make a `tests` directory at the
top level of your project, next to `src`. Then, make a new file,
`tests/integration_test.rs`, and put this inside:

```rust,ignore
extern crate adder;

#[test]
fn it_works() {
    assert_eq!(4, adder::add_two(2));
}
```

There's some small changes from our previous tests. We now have an `extern
crate adder` at the top. This is because each test in the `tests` directory is
an entirely separate crate, and so we need to import our library.  This is also
why `tests` is a suitable place to write integration-style tests: they use the
library like any other consumer of it would.

Let's run them:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///home/steve/tmp/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Now we have three sections: our previous test is also run, as well as our new
one.

That's all there is to the `tests` directory. The `tests` module isn't needed
here, since the whole thing is focused on tests.

## Submodules in integration tests

As your integration tests grow, you may want to make more than one file in the
`tests` directory. As we mentioned before, that works well, given that Cargo
treats every file as its own crate. But there's one small trap that can happen.

Imagine we wanted some common helper functions to be shared across our tests.
So we change our test to have a `common` module:

```rust,ignore
extern crate adder;

mod common;

#[test]
fn it_works() {
    common::helper();

    assert_eq!(4, adder::add_two(2));
}
```

And then, we create a `tests/common.rs` file to hold our common helpers:

```rust
pub fn helper() {
    // no implementation for now
}
```

Let's try running this:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///home/steve/tmp/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.25 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/common-c3635c69f3aeef92

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/integration_tests-6d6e12b4680b0368

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Wait a minute. Now we have four sections?

```text
     Running target/debug/common-c3635c69f3aeef92
```

Because `common.rs` is in our `tests` directory, Cargo is also compiling it as
its own crate. Because `common.rs` is so simple, we didn't get an error, but
with more complex code, this might not work. So what can we do?

The key is, always use the `common/mod.rs` form over the `common.rs` form when
making modules in integration tests. If we move `tests/common.rs` to
`tests/common/mod.rs`, we'll go back to our expected output:

```bash
$ mkdir tests/common
$ mv tests/common.rs tests/common/mod.rs
$ cargo test
   Compiling adder v0.1.0 (file:///home/steve/tmp/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/integration_tests-6d6e12b4680b0368

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
