# More about Cargo and Crates.io

We've used some features of Cargo in this book so far, but only the most basic
ones. We've used Cargo to build, run, and test our code, but it can do a lot
more. Let's go over some of its other features now. Cargo can do even more than
this; for a full explanation, see its documentation.

We're going to cover:

* Customizing your build through release profiles
* Organizing larger projects with 'workspaces'
* Extending Cargo with your own custom commands
* Publishing libraries on crates.io

## Release profiles

Cargo supports a notion of "release profiles." These profiles control various
options for compiling your code, and let you configure each profile
independently of the others. You've seen a hint of this feature in the output
of your builds:

```bash
$ cargo build
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

The "debug" and "release" notifications here are profile names. Rust
supports four profiles:

* `dev`: used for `cargo build`
* `release` used for `cargo build --release`
* `test` used for `cargo test`
* `doc` used for `cargo doc`

We can customize our `Cargo.toml` file with `[profile.*]` sections to
tweak various compiler options for these profiles. For example, here's
one of the default options for the `dev` and `release` profiles:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls how many optimizations Rust will apply to your
code. The setting goes from zero to three. Why would you ever want fewer
optimizations? Applying them takes time, and so leads to a slower build.

We could override these defaults by putting this into `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1
```

This overrides the default setting of `0`, and now our debug builds will use
more optimizations. Not as much as a release build, but a little bit more.

For the full list of settings, see Cargo's documentation.

## Cargo Workspaces

In Chapter XX, we built a project that included both a binary and a library.
But what if we want to split our project up into multiple libraries? As
projects grow, separating out major components can be quite useful. In this
situation, Cargo has a feature called "workspaces" that can help us manage
multiple packages.

Let's make a project using workspaces. The code will be trivial: one package
will provide an `add_one` method, a second will provide an `add_two` method,
and the third will be a binary that uses both. To do so, let's create a new
crate for the binary:

```bash
$ cargo new --bin adder
     Created binary (application) `adder` project
$ cd adder
```

We need to modify the binary package's `Cargo.toml`. Add this
at the bottom of the file:

```toml
[workspace]
```

This tells Cargo that this package is a workspace. Like many Cargo features,
workspaces support convention over configuration: we don't need to say anything
more than this as long as we follow the convention. What is the convention? Any
crates that we depend on as sub-directories will be part of the workspace. So
let's add a dependency. Change the `[dependencies]` section of `Cargo.toml`
to look like this:

```toml
[dependencies]
add-one = { path = "add-one" }
```

And then generate the `add-one` crate:

```bash
$ cargo new add-one
     Created library `add-one` project
```

Open up `src/main.rs` and add an `extern crate` line:

```rust,ignore
extern crate add_one;

fn main() {
    println!("Hello, world!");
}
```

Let's build it!

```bash
$ cargo build
   Compiling add-one v0.1.0 (file:///home/steve/tmp/adder/add-one)
   Compiling adder v0.1.0 (file:///home/steve/tmp/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.68 secs
```

See if you can add an `add-two` crate in the same way.

As your project grows, consider a workspace: smaller components are easier
to understand individually than one big blob of code.

## Getting your project ready to be published on Crates.io

We've often used the `[dependencies]` section of `Cargo.toml` to depend on
local crates, like the ones above. But Cargo also supports fetching code from a
central repository: crates.io. It's a repository of open source code shared
between Rust developers. By publishing a package on crates.io, you allow other
people to use your work, and by depending on their packages, you can use their
work. crates.io distributes the source code of your packages, so it is primarily
used to distribute code that's 'open source'.

Cargo can also fetch packages from other hosts as well, like a private
repository you might run yourself, but doing so requires configuration. See
Cargo's documentation for more.

Rust and Cargo have some features that can make your published package better.
We'll talk about some of those features, and then discover how to publish a
package.

### Documentation Comments

We've already seen comments in Rust; they start with `//`:

```rust
// This is a comment, the compiler ignores it.
```

Rust also has a second kind of comment; the documentation comment.
Documentation comments use `///` instead of `//`, and support Markdown notation
inside:

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
```

There is another style of doc comment, `//!`, to comment containing items (e.g.
crates, modules or functions), instead of the items following it. Commonly used
inside crates root (lib.rs) or modules root (mod.rs):

```
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

When writing doc comments, providing some examples of usage is very, very
helpful.

`cargo doc` will run a tool distributed with Rust, `rustdoc`, to generate
HTML documentation from these comments. Explaining your interface is an
important part of writing a package that's easier for others to use.

There's one additional bonus: `cargo test` will run the code examples in your
documentation as tests! Nothing is better than documentation with examples.
Nothing is worse than examples that don't actually work, because the code has
changed since the documentation has been written. Here's a fleshed-out
`src/lib.rs` with examples:

```rust,ignore
# // The next line exists to trick play.rust-lang.org into running our code as a
# // test:
# // fn main
#
//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
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

It is conventional to include the `# Examples` section, exactly like that, with
examples following.

It's worth noting that documentation is only generated for library crates, not
binary crates.

### `cargo publish`

Once you've got a library that you'd like to share with the world, it's time to
publish it on [crates.io]! Publishing a crate is when a specific version is
uploaded to be hosted on [crates.io].

Take care when publishing a crate, because a publish is **permanent**. The
version can never be overwritten, and the code cannot be deleted. There is no
limit to the number of versions which can be published, however.

# Before your first publish

First things first, you'll need an account on [crates.io] to acquire an API
token. To do so, [visit the home page][crates.io] and log in via a GitHub
account (required for now). After this, visit your [Account
Settings](https://crates.io/me) page and run the `cargo login` command
specified.

```bash
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

This command will inform Cargo of your API token and store it locally in your
`~/.cargo/config`. Note that this token is a **secret** and should not be
shared with anyone else. If it leaks for any reason, you should regenerate it
immediately.

# Before publishing a new crate

Keep in mind that crate names on [crates.io] are allocated on a first-come-first-
serve basis. Once a crate name is taken, it cannot be used for another crate.

## Uploading the crate

Crates can be uploaded to [crates.io] with the `cargo publish` command. And
that's it, you've now published your first crate!

```text
$ cargo publish
```

If you'd like to skip the `cargo package` step, the `cargo publish` subcommand
will automatically package up the local crate if a copy isn't found already.

Be sure to check out the [metadata you can
specify](manifest.html#package-metadata) to ensure your crate can be discovered
more easily!

# Publishing a new version of an existing crate

In order to release a new version, change the `version` value specified in your
`Cargo.toml` manifest. Keep in mind [the semver
rules](manifest.html#the-version-field). Then optionally run `cargo package` if
you want to inspect the `*.crate` file for the new version before publishing,
and run `cargo publish` to upload the new version.

### Metadata in your Cargo.toml

If you try to publish a new crate, you'll get a warning:

```bash
$ cargo publish
    Updating registry `https://github.com/rust-lang/crates.io-index`
warning: manifest has no description, license, license-file, documentation, homepage or repository.
```

We can include more information about our package. Some of these fields are
optional, but a good package will include at least these fields:

```toml
[package]
description = "..."
repository = "..."
keywords = ["...", "..."]
license = "..."
```

The `description` is displayed on crates.io to let people know what your
package does.

The `repository` field should be a URL where the source code is hosted.

`keywords` allow people to find your crate more easily; you can have up to
five.

The `license` field lets people know what terms they can use your crate under.

### Removing versions

Occasions may arise where you publish a version of a crate that actually ends up
being broken for one reason or another (syntax error, forgot to include a file,
etc.). For situations such as this, Cargo supports a "yank" of a version of a
crate.

```text
$ cargo yank --vers 1.0.1
$ cargo yank --vers 1.0.1 --undo
```

A yank **does not** delete any code. This feature is not intended for deleting
accidentally uploaded secrets, for example. If that happens, you must reset
those secrets immediately.

The semantics of a yanked version are that no new dependencies can be created
against that version, but all existing dependencies continue to work. One of
the major goals of [crates.io] is to act as a permanent archive of crates that
does not change over time, and allowing deletion of a version would go against
this goal. Essentially a yank means that all projects with a `Cargo.lock` will
not break, while any future `Cargo.lock` files generated will not list the
yanked version.
