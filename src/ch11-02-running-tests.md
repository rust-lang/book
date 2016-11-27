## Running tests

Just like `cargo run` compiles your code and then runs the resulting binary,
`cargo test` compiles your code in test mode and runs the resulting test
binary. The default behavior of the binary that `cargo test` produces is to run
all the tests in parallel and to capture output generated during test runs so
that it's easier to read the output about the test results.

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

If you don't want this behavior, or if you want more fine-grained control over
the number of threads used, you can send the `--test-threads` flag and the
number of threads to the test binary. Setting the number of test threads to 1
means to not use any parallelism:

```text
$ cargo test -- --test-threads=1
```

### Tests Capture Output

By default, Rust's test library captures and discards output to standard out
and standard error, unless the test fails. For example, if you call
`println!()` in a test and the test passes, you won't see the `println!` output
in your terminal. This behavior can be disabled by sending the `--nocapture`
flag to the test binary:

```text
$ cargo test -- --nocapture
```

### Running a Subset of Tests by Name

Sometimes, running a full test suite can take a long time. If you're only
working on code in a particular area, you might want to only run the tests
having to do with that code. `cargo test` takes an argument that allows you to
only run certain tests, specified by name.

Let's create three tests with the following names as shown in Listing 11-3:

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

<caption>
Listing 11-3: Three tests with a variety of names
</caption>

Running with different arguments will run different subsets of the tests. No
arguments, as we've already seen, runs all the tests:

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

<caption>
Listing 11-4: Tests in two modules named `adding` and `subtracting`
</caption>

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
most runs of `cargo test`, we'd like to exclude them. Instead of having to
construct an argument to `cargo test` to run all tests except these and
remember to use that argument every time, we can annotate these tests with the
`ignore` attribute:

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

Now if we run our tests, we'll see `it_works` is run, but `expensive_test` is
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
When you're at a point that it makes sense to check the results of the
`ignored` tests and you have time to wait for the results, you can choose to
run `cargo test -- --ignored` instead.
