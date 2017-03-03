## Cargo Workspaces

In Chapter 12, we built a package that included both a binary crate and a
library crate. But what if the library crate continues to get bigger and we
want to split our package up further into multiple library crates? As packages
grow, separating out major components can be quite useful. In this situation,
Cargo has a feature called *workspaces* that can help us manage multiple
related packages that are developed in tandem.

A *workspace* is a set of packages that will all share the same *Cargo.lock*
and output directory. Let's make a project using a workspace where the code
will be trivial so that we can concentrate on the structure of a workspace.
We'll have a binary that uses two libraries: one that will provide an `add_one`
method and a second that will provide an `add_two` method. Let's start by
creating a new crate for the binary:

```text
$ cargo new --bin adder
     Created binary (application) `adder` project
$ cd adder
```

We need to modify the binary package's *Cargo.toml* to tell Cargo the `adder`
package is a workspace. Add this at the bottom of the file:

```toml
[workspace]
```

Like many Cargo features, workspaces support convention over configuration: we
don't need to say anything more than this as long as we follow the convention.
The convention is that any crates that we depend on as sub-directories will be
part of the workspace. Let's add a path dependency to the `adder` crate by
changing the `[dependencies]` section of *Cargo.toml* to look like this:

```toml
[dependencies]
add-one = { path = "add-one" }
```

If we add dependencies that don't have a `path` specified, those will be normal
dependencies that aren't in this workspace.

Next, generate the `add-one` crate within the `adder` directory:

```text
$ cargo new add-one
     Created library `add-one` project
```

Your `adder` directory should now have these directories and files:

```text
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── src
    └── main.rs
```

In *add-one/src/lib.rs*, let's add an implementation of an `add_one` function:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Open up *src/main.rs* for `adder` and add an `extern crate` line to bring the
new `add-one` library crate into scope, and change the `main` function to use
the `add_one` function:

```rust,ignore
extern crate add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

Let's build it!

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.68 secs
```

Note that running `cargo build` in the *adder* directory built both that crate
and the `add-one` crate in *adder/add-one*, but created only one *Cargo.lock*
and one *target* directory, both in the *adder* directory. See if you can add
an `add-two` crate in the same way.

Let's now say that we'd like to use the `rand` crate in our `add-one` crate.
As usual, we'll add it to the `[dependencies]` section in the `Cargo.toml` for
that crate:

<span class="filename">Filename: add-one/Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

And if we add `extern crate rand;` to *add-one/src/lib.rs* then run `cargo
build`, it will succeed:

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
   ...snip...
   Compiling rand v0.3.14
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 10.18 secs
```

The top level *Cargo.lock* now contains information about the dependency
`add-one` has on `rand`. However, even though `rand` is used somewhere in the
workspace, we can't use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* as well. If we add `extern crate rand;` to
*src/main.rs* for the top level `adder` crate, for example, we'll get an error:

```text
$ cargo build
   Compiling adder v0.1.0 (file:///projects/adder)
error[E0463]: can't find crate for `rand`
 --> src/main.rs:1:1
  |
1 | extern crate rand;
  | ^^^^^^^^^^^^^^^^^^^ can't find crate
```

To fix this, edit *Cargo.toml* for the top level and indicate that `rand` is a
dependency for the `adder` crate.

For another enhancement, let's add a test of the `add_one::add_one` function
within that crate:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Now run `cargo test` in the top-level *adder* directory:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/adder-f0253159197f7841

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Wait a second, zero tests? We just added one! If we look at the output, we can
see that `cargo test` in a workspace only runs the tests for the top level
crate. To run tests for the other crates, we need to use the `-p` argument to
indicate we want to run tests for a particular package:

```text
$ cargo test -p add-one
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/add_one-abcabcabc

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Similarly, if you choose to publish the workspace to crates.io, each crate in
the workspace will get published separately.

As your project grows, consider a workspace: smaller components are easier to
understand individually than one big blob of code. Keeping the crates in a
workspace can make coordination among them easier if they work together and are
often changed at the same time.
