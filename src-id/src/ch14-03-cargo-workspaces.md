## Cargo Workspaces

In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split your package further into
multiple library crates. Cargo offers a feature called _workspaces_ that can
help manage multiple related packages that are developed in tandem.

### Creating a Workspace

A _workspace_ is a set of packages that share the same _Cargo.lock_ and output
directory. Let’s make a project using a workspace—we’ll use trivial code so we
can concentrate on the structure of the workspace. There are multiple ways to
structure a workspace, so we'll just show one common way. We’ll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function, and a second library an `add_two` function.
These three crates will be part of the same workspace. We’ll start by creating
a new directory for the workspace:

```console
$ mkdir add
$ cd add
```

Next, in the _add_ directory, we create the _Cargo.toml_ file that will
configure the entire workspace. This file won’t have a `[package]` section.
Instead, it will start with a `[workspace]` section that will allow us to add
members to the workspace by specifying the path to the package with our binary
crate; in this case, that path is _adder_:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```

Next, we’ll create the `adder` binary crate by running `cargo new` within the
_add_ directory:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
```

At this point, we can build the workspace by running `cargo build`. The files
in your _add_ directory should look like this:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The workspace has one _target_ directory at the top level that the compiled
artifacts will be placed into; the `adder` package doesn’t have its own
_target_ directory. Even if we were to run `cargo build` from inside the
_adder_ directory, the compiled artifacts would still end up in _add/target_
rather than _add/adder/target_. Cargo structures the _target_ directory in a
workspace like this because the crates in a workspace are meant to depend on
each other. If each crate had its own _target_ directory, each crate would have
to recompile each of the other crates in the workspace to place the artifacts
in its own _target_ directory. By sharing one _target_ directory, the crates
can avoid unnecessary rebuilding.

### Creating the Second Package in the Workspace

Next, let’s create another member package in the workspace and call it
`add_one`. Change the top-level _Cargo.toml_ to specify the _add_one_ path in
the `members` list:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Then generate a new library crate named `add_one`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
```

Your _add_ directory should now have these directories and files:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

In the _add_one/src/lib.rs_ file, let’s add an `add_one` function:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

Now we can have the `adder` package with our binary depend on the `add_one`
package that has our library. First, we’ll need to add a path dependency on
`add_one` to _adder/Cargo.toml_.

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo doesn’t assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships.

Next, let’s use the `add_one` function (from the `add_one` crate) in the
`adder` crate. Open the _adder/src/main.rs_ file and add a `use` line at the
top to bring the new `add_one` library crate into scope. Then change the `main`
function to call the `add_one` function, as in Listing 14-7.

<span class="filename">Filename: adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<span class="caption">Listing 14-7: Using the `add_one` library crate from the
`adder` crate</span>

Let’s build the workspace by running `cargo build` in the top-level _add_
directory!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

To run the binary crate from the _add_ directory, we can specify which
package in the workspace we want to run by using the `-p` argument and the
package name with `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

This runs the code in _adder/src/main.rs_, which depends on the `add_one` crate.

#### Depending on an External Package in a Workspace

Notice that the workspace has only one _Cargo.lock_ file at the top level,
rather than having a _Cargo.lock_ in each crate’s directory. This ensures that
all crates are using the same version of all dependencies. If we add the `rand`
package to the _adder/Cargo.toml_ and _add_one/Cargo.toml_ files, Cargo will
resolve both of those to one version of `rand` and record that in the one
_Cargo.lock_. Making all crates in the workspace use the same dependencies
means the crates will always be compatible with each other. Let’s add the
`rand` crate to the `[dependencies]` section in the _add_one/Cargo.toml_ file
so we can use the `rand` crate in the `add_one` crate:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

We can now add `use rand;` to the _add_one/src/lib.rs_ file, and building the
whole workspace by running `cargo build` in the _add_ directory will bring in
and compile the `rand` crate. We will get one warning because we aren’t
referring to the `rand` we brought into scope:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

The top-level _Cargo.lock_ now contains information about the dependency of
`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their _Cargo.toml_ files as well. For example, if we add `use rand;`
to the _adder/src/main.rs_ file for the `adder` package, we’ll get an error:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

To fix this, edit the _Cargo.toml_ file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in _Cargo.lock_, but no
additional copies of `rand` will be downloaded. Cargo has ensured that every
crate in every package in the workspace using the `rand` package will be using
the same version, saving us space and ensuring that the crates in the workspace
will be compatible with each other.

#### Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

Now run `cargo test` in the top-level _add_ directory. Running `cargo test` in
a workspace structured like this one will run the tests for all the crates in
the workspace:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The first section of the output shows that the `it_works` test in the `add_one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add_one` crate.

We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This output shows `cargo test` only ran the tests for the `add_one` crate and
didn’t run the `adder` crate tests.

If you publish the crates in the workspace to [crates.io](https://crates.io/),
each crate in the workspace will need to be published separately. Like `cargo
test`, we can publish a particular crate in our workspace by using the `-p`
flag and specifying the name of the crate we want to publish.

For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!

As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between crates easier if they
are often changed at the same time.
