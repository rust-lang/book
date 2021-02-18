## Cargo Workspaces

In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split up your package further into
multiple library crates. In this situation, Cargo offers a feature called
*workspaces* that can help manage multiple related packages that are developed
in tandem.

### Creating a Workspace

A *workspace* is a set of packages that share the same *Cargo.lock* and output
directory. Let’s make a project using a workspace—we’ll use trivial code so we
can concentrate on the structure of the workspace. There are multiple ways to
structure a workspace; we’re going to show one common way. We’ll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function, and a second library an `add_two` function.
These three crates will be part of the same workspace. We’ll start by creating
a new directory for the workspace:

```console
$ mkdir add
$ cd add
```

Next, in the *add* directory, we create the *Cargo.toml* file that will
configure the entire workspace. This file won’t have a `[package]` section or
the metadata we’ve seen in other *Cargo.toml* files. Instead, it will start
with a `[workspace]` section that will allow us to add members to the workspace
by specifying the path to the package with our binary crate; in this case,
that path is *adder*:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace-with-adder-crate/add/Cargo.toml}}
```

Next, we’ll create the `adder` binary crate by running `cargo new` within the
*add* directory:

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
in your *add* directory should look like this:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The workspace has one *target* directory at the top level for the compiled
artifacts to be placed into; the `adder` package doesn’t have its own *target*
directory. Even if we were to run `cargo build` from inside the *adder*
directory, the compiled artifacts would still end up in *add/target* rather
than *add/adder/target*. Cargo structures the *target* directory in a workspace
like this because the crates in a workspace are meant to depend on each other.
If each crate had its own *target* directory, each crate would have to
recompile each of the other crates in the workspace to have the artifacts in
its own *target* directory. By sharing one *target* directory, the crates can
avoid unnecessary rebuilding.

### Creating the Second Package in the Workspace

Next, let’s create another member package in the workspace and call it `add-one`.
Change the top-level *Cargo.toml* to specify the *add-one* path in the
`members` list:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

Then generate a new library crate named `add-one`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
rm -rf add-one
cargo new add-one --lib
copy output below
-->

```console
$ cargo new add-one --lib
     Created library `add-one` package
```

Your *add* directory should now have these directories and files:

```text
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

In the *add-one/src/lib.rs* file, let’s add an `add_one` function:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add-one/src/lib.rs}}
```

Now that we have another package in the workspace, we can have the `adder`
package with our binary depend on the `add-one` package, that has our
library. First, we’ll need to add a path dependency on `add-one` to
*adder/Cargo.toml*.

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:7:9}}
```

Cargo doesn’t assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships between the crates.

Next, let’s use the `add_one` function from the `add-one` crate in the `adder`
crate. Open the *adder/src/main.rs* file and add a `use` line at the top to
bring the new `add-one` library crate into scope. Then change the `main`
function to call the `add_one` function, as in Listing 14-7.

<span class="filename">Filename: adder/src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

<span class="caption">Listing 14-7: Using the `add-one` library crate from the
 `adder` crate</span>

Let’s build the workspace by running `cargo build` in the top-level *add*
directory!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

To run the binary crate from the *add* directory, we can specify which
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

This runs the code in *adder/src/main.rs*, which depends on the `add-one` crate.

#### Depending on an External Package in a Workspace

Notice that the workspace has only one *Cargo.lock* file at the top level of
the workspace rather than having a *Cargo.lock* in each crate’s directory. This
ensures that all crates are using the same version of all dependencies. If we
add the `rand` package to the *adder/Cargo.toml* and *add-one/Cargo.toml*
files, Cargo will resolve both of those to one version of `rand` and record
that in the one *Cargo.lock*. Making all crates in the workspace use the same
dependencies means the crates in the workspace will always be compatible with
each other. Let’s add the `rand` crate to the `[dependencies]` section in the
*add-one/Cargo.toml* file to be able to use the `rand` crate in the `add-one`
crate:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add-one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add-one/Cargo.toml:7:8}}
```

We can now add `use rand;` to the *add-one/src/lib.rs* file, and building the
whole workspace by running `cargo build` in the *add* directory will bring in
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
  Downloaded rand v0.8.3
   --snip--
   Compiling rand v0.8.3
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
warning: unused import: `rand`
 --> add-one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

The top-level *Cargo.lock* now contains information about the dependency of
`add-one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add
`rand` to their *Cargo.toml* files as well. For example, if we add `use rand;`
to the *adder/src/main.rs* file for the `adder` package, we’ll get an error:

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

To fix this, edit the *Cargo.toml* file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo has ensured that every
crate in every package in the workspace using the `rand` package will be
using the same version. Using the same version of `rand` across the workspace
saves space because we won’t have multiple copies and ensures that the crates
in the workspace will be compatible with each other.

#### Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add-one/src/lib.rs}}
```

Now run `cargo test` in the top-level *add* directory:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test
   Compiling add-one v0.1.0 (file:///projects/add/add-one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running target/debug/deps/add_one-f0253159197f7841

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/adder-49979ff40686fa8e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The first section of the output shows that the `it_works` test in the `add-one`
crate passed. The next section shows that zero tests were found in the `adder`
crate, and then the last section shows zero documentation tests were found in
the `add-one` crate. Running `cargo test` in a workspace structured like this
one will run the tests for all the crates in the workspace.

We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add-one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add-one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This output shows `cargo test` only ran the tests for the `add-one` crate and
didn’t run the `adder` crate tests.

If you publish the crates in the workspace to [crates.io](https://crates.io/),
each crate in the workspace will need to be published separately. The `cargo
publish` command does not have an `--all` flag or a `-p` flag, so you must
change to each crate’s directory and run `cargo publish` on each crate in the
workspace to publish the crates.

For additional practice, add an `add-two` crate to this workspace in a similar
way as the `add-one` crate!

As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between them easier if they are
often changed at the same time.
