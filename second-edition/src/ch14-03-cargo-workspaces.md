## Cargo Workspaces

In Chapter 12, we built a package that included both a binary crate and a
library crate. You may find, as your project develops, that the library crate
continues to get bigger and you want to split your package up further into
multiple library crates. In this situation, Cargo has a feature called
*workspaces* that can help manage multiple related packages that are developed
in tandem.

A *workspace* is a set of packages that will all share the same *Cargo.lock*
and output directory. Let’s make a project using a workspace, using trivial
code so we can concentrate on the structure of a workspace. We’ll have a binary
that uses two libraries: one library that will provide an `add_one` function
and a second library that will provide an `add_two` function. These three
crates will all be part of the same workspace. We’ll start by creating a new
crate for the binary:

```text
$ cargo new --bin adder
     Created binary (application) `adder` project
$ cd adder
```

We need to modify the binary package’s *Cargo.toml* and add a `[workspace]`
section to tell Cargo the `adder` package is a workspace. Add this at the
bottom of the file:

<span class="filename">Filename: Cargo.toml</span>

```toml
[workspace]
```

Like many Cargo features, workspaces support convention over configuration: we
don’t need to add anything more than this to *Cargo.toml* to define our
workspace as long as we follow the convention.

### Specifying Workspace Dependencies

By default, Cargo will include all transitive path dependencies. A *path
dependency* is when any crate, whether in a workspace or not, specifies that it
has a dependency on a crate in a local directory by using the `path` attribute
on the dependency specification in *Cargo.toml*. If a crate has the
`[workspace]` key, or if the crate is itself part of a workspace, and we
specify path dependencies where the paths are subdirectories of the crate’s
directory, those dependent crates will be considered part of the workspace.
Let’s specify in the *Cargo.toml* for the top-level `adder` crate that it will
have a dependency on an `add-one` crate that will be in the `add-one`
subdirectory, by changing *Cargo.toml* to look like this:

<span class="filename">Filename: Cargo.toml</span>

```toml
[dependencies]
add-one = { path = "add-one" }
```

If we add dependencies to *Cargo.toml* that don’t have a `path` specified,
those dependencies will be normal dependencies that aren’t in this workspace
and are assumed to come from Crates.io.

### Creating the Second Crate in the Workspace

Next, while in the `adder` directory, generate an `add-one` crate:

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

In *add-one/src/lib.rs*, let’s add an `add_one` function:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Open up *src/main.rs* for `adder` and add an `extern crate` line at the top of
the file to bring the new `add-one` library crate into scope. Then change the
`main` function to call the `add_one` function, as in Listing 14-11:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

<span class="caption">Listing 14-11: Using the `add-one` library crate from the
`adder` crate</span>

Let’s build the `adder` crate by running `cargo build` in the *adder* directory!

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68 secs
```

Note that this builds both the `adder` crate and the `add-one` crate in
*adder/add-one*. Now your *adder* directory should have these files:

```text
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── src
│   └── main.rs
└── target
```

The workspace has one *target* directory at the top level; *add-one* doesn’t
have its own *target* directory. Even if we go into the `add-one` directory and
run `cargo build`, the compiled artifacts end up in *adder/target* rather than
*adder/add-one/target*. The crates in a workspace depend on each other. If each
crate had its own *target* directory, each crate in the workspace would have to
recompile each other crate in the workspace in order to have the artifacts in
its own *target* directory. By sharing one *target* directory, the crates in
the workspace can avoid rebuilding the other crates in the workspace more than
necessary.

#### Depending on an External Crate in a Workspace

Also notice the workspace only has one *Cargo.lock*, rather than having a
top-level *Cargo.lock* and *add-one/Cargo.lock*. This ensures that all crates
are using the same version of all dependencies. If we add the `rand` crate to
both *Cargo.toml* and *add-one/Cargo.toml*, Cargo will resolve both of those to
one version of `rand` and record that in the one *Cargo.lock*. Making all
crates in the workspace use the same dependencies means the crates in the
workspace will always be compatible with each other. Let’s try this out now.

Let’s add the `rand` crate to the `[dependencies]` section in
*add-one/Cargo.toml* in order to be able to use the `rand` crate in the
`add-one` crate:

<span class="filename">Filename: add-one/Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

We can now add `extern crate rand;` to *add-one/src/lib.rs*, and building the
whole workspace by running `cargo build` in the *adder* directory will bring in
and compile the `rand` crate:

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
   ...snip...
   Compiling rand v0.3.14
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18 secs
```

The top level *Cargo.lock* now contains information about `add-one`’s
dependency on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* as well. If we add `extern crate rand;` to
*src/main.rs* for the top level `adder` crate, for example, we’ll get an error:

```text
$ cargo build
   Compiling adder v0.1.0 (file:///projects/adder)
error[E0463]: can't find crate for `rand`
 --> src/main.rs:1:1
  |
1 | extern crate rand;
  | ^^^^^^^^^^^^^^^^^^^ can't find crate
```

To fix this, edit *Cargo.toml* for the top level `adder` crate and indicate
that `rand` is a dependency for that crate as well. Building the `adder` crate
will add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo has ensured for us that
any crate in the workspace using the `rand` crate will be using the same
version. Using the same version of `rand` across the workspace saves space
since we won’t have multiple copies and ensures that the crates in the
workspace will be compatible with each other.

#### Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:

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
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/adder-f0253159197f7841

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Wait a second, zero tests? We just added one! If we look at the output, we can
see that `cargo test` in a workspace only runs tests for the top level crate.
To run tests for all of the crates in the workspace, we need to pass the
`--all` flag:

```text
$ cargo test --all
    Finished dev [unoptimized + debuginfo] target(s) in 0.37 secs
     Running target/debug/deps/add_one-abcabcabc

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-abcabcabc

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

When passing `--all`, `cargo test` will run the tests for all of the crates in
the workspace. We can also choose to run tests for one particular crate in a
workspace from the top level directory by using the `-p` flag and specifying
the name of the crate we want to test:

```text
$ cargo test -p add-one
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

This output shows `cargo test` only ran the tests for the `add-one` crate and
didn’t run the `adder` crate tests.

If you choose to publish the crates in the workspace to crates.io, each crate
in the workspace will get published separately. The `cargo publish` command
does not have an `--all` flag or a `-p` flag, so it is necessary to change to
each crate’s directory and run `cargo publish` on each crate in the workspace
in order to publish them.

Now try adding an `add-two` crate to this workspace in a similar way as the
`add-one` crate for some more practice!

As your project grows, consider using a workspace: smaller components are
easier to understand individually than one big blob of code. Keeping the crates
in a workspace can make coordination among them easier if they work together
and are often changed at the same time.
