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

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-10: Tests for a function that calls
`println!`</span>

The output we’ll see when we run these tests with `cargo test` is:

```text
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

```text
$ cargo test -- --nocapture
```

Running the tests from Listing 11-10 again with the `--nocapture` flag now
shows:

```text
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

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 11-11: Three tests with a variety of names</span>

If we run the tests without passing any arguments, as we’ve already seen, all
the tests will run in parallel:

```text
running 3 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

#### Running Single Tests

We can pass the name of any test function to `cargo test` to run only that test:

```text
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

```text
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

We add the `#[ignore]` line to the test we want to exclude, after `#[test]`.
Now if we run our tests, we’ll see `it_works` runs, but `expensive_test` does
not:

```text
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

```text
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
