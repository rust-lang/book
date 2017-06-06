
[TOC]

# More about Cargo and Crates.io

So far we've used only the most basic features of Cargo to build, run, and test
our code, but it can do a lot more. Here we'll go over some of its other, more
advanced features to show you how to:

* Customize your build through release profiles
* Publish libraries on crates.io
* Organize larger projects with workspaces
* Install binaries from crates.io
* Extend Cargo with your own custom commands

Cargo can do even more than what we can cover in this chapter too, so for a
full explanation, see its documentation at.

<!--can you give a link to the documentation?-->

## Customizing Builds with Release Profiles

In Rust *release profiles* are pre-defined, and customizable, profiles with
different configurations, to allow the user more control over various options
for compiling your code. Ech profile is configured independently of the others.

<!-- To be clear, are these release profiles pre-defined profiles that you use
for different things? Can you lay that out more explicitly, give a more
detailed definition? That seems super useful, but I'm not sure I'm following
what they actually are. -->

Cargo supports four profiles:

<!-- Hm, so these profiles aren't built-in, just supported? and used for what
for cargo build? How do you use a particular profile in a build, is it chosen
by default? Do you have to specify? -->

* `dev`: used for `cargo build`
* `release` used for `cargo build --release`
* `test` used for `cargo test`
* `doc` used for `cargo doc`

This may be familar from the output of your builds, which will hint at the
profile used in the build:

<!-- Above-is that what you meant here? -->

```text
$ cargo build
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

The "debug" and "release" notifications here indicate that the compiler is
using different profiles.

<!-- which profile is "debug" associated with? As you can probably tell, I'm
not confident in my interpretation here, I think we need more info -->

### Customizing Release Profiles

<!-- Do we mean that the profiles are all already stored in Cargo.toml, or you
have to add the entire code to cargo.toml? It seems like the former from the
writing, but looking through toml files I've made the latter seems to be true.
If you have multiple profiles in the toml, how do you choose which one to use?
-->

The release profiles are stored in your `Cargo.tml` file, and we can customize
the `[profile.*]` sections to tweak various compiler options for these
profiles. For example, here is the `opt-level` configuration for the `dev` and
`release` profiles that gives you control over the speed of compilation:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls how many optimizations Rust will apply to your
code, with a range of zero to three. Applying more optimizations takes more
time at compile time, so if you're in development and compiling very often,
you'd want compiling to be fast at the expense of the resulting code running
slower, and would want a lower `opt-level`. When you're ready to release, it's
better to spend more time compiling, since you will only be doing it once, as a
trade off for faster running code when you use it, so you'd want a higher
`opt-level`.

We can override these defaults by changing them in `Cargo.toml`. If we wanted
to use optimization level 1 in the development profile:

<!-- So do we choose which profile to use when? How do we do that? Or is that
determined automatically by Rust, and if so, how? I think we need to show that
somewhere around here -->

```toml
[profile.dev]
opt-level = 1
```

This overrides the default setting of `0`, and now our builds in the
development profile will use more optimizations than the default, but not as
much as a release build.

For the full list of settings, configuration options, and defaults for each
profile, see Cargo's documentation. at *http://doc.crates.io/*

## Publishing a Crate to Crates.io

We've used crates from crates.io as dependencies of our project, but you can
also share your code for other people to use by publishing your own crates.
Crates.io distributes the source code of your packages, so primarily hosts code
that's open source.

Rust and Cargo have features that help make documentation clear and useful, to
make your published package easier for people to find and use. We'll talk about
some of those features, then cover how to publish a package.

### Making useful Documentation Comments

Accurately documenting your packages will help other users know how and when to
use them, so it's worth paying attention to. In Chapter 3, we discussed how to
comment Rust code with `//`. Rust also has particular kind of comment for
documentation, known convenientely as *documentation comments*, that will
generate in HTML in your documentation. The HTML displays the contents of
documentation comments for public API items, intended for programmers
interested in knowing how to *use* your crate, as opposed to how your crate is
*implemented*. Note that documentation is only generated for library crates,
since binary crates don't have a public API.

<!-- Doc comments support markdown but dont require markdown, is that right?
Just wanted to make that distinction -->

Documentation comments use `///` instead of `//` and support Markdown notation
inside. You should be place them just before the item they are documenting.
Here's an example of documentation comments for an `add_one` function:

Filename: src/lib.rs

````rust
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
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

Listing 14-1: A documentation comment for a function

<!-- At some point, a screenshot of how this is renderd in HTML could be really
useful here, what you do think? -->

Here, we give a description and example run of the `add_one` function in
comments before defining the function itself. These comments will....

<!--Above - I added this line to describe what we're doing, encourage good
practice, can you add/edit where necessary? These will generate as HTML when
the code is run, is that how it works? -->

We can test this code locally by running it with `cargo doc`, which runs the
`rustdoc` tool distributed with Rust to generate HTML documentation from these
comments. Try this out locally by running `cargo doc --open`, which will build
the documentation for your current crate (as well as all of your crate's
dependencies) and open it in a web browser. Navigate to the `add_one` function
and you'll see how the text in the documentation comments gets rendered.

#### Documentation Comment as Tests

Adding examples in code blocks in your documentation comments is a way to
clearly demonstrate how to use your library, but also has an additional bonus:
running the crate with `cargo test` will run the code examples in your
documentation as tests! Nothing is better than documentation with examples.
Nothing is worse than examples that don't actually work because the code has
changed since the documentation has been written. Try running `cargo test` with
the documentation for the `add_one` function like in Listing 14-1; you should
see a section in the test results like this:

```test
   Doc-tests add-one

running 1 test
test add_one_0 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Now try changing the function for the example and running this again, and you
should see that `cargo test` catches that the example no longer works!

#### Commenting Contained Items

<!-- I'm not clear what this comment does that's different, what do you mean by
"comment containing items"? The lingo might just be going over my head here -->

There's another style of doc comment, `//!`, that comments containing
items---for example crates, modules, or functions---instead of the items
following the comments. These are typically used inside the crate root (lib.rs)
or a module's root (mod.rs) to document the crate or the module as a whole.
Here's the documentation within the `libstd` module that contains the entire
standard library:

```
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

<!-- I'm not sure what we're looking at here, that's different from just using
///, can you point it out, talk about it? -->

### Exporting a Convenient Public API with `pub use`

In Chapter 7, we covered how to organize our code into modules with the `mod`
keyword, how to make items public with the `pub` keyword, and how to bring
items into a scope with the `use` keyword. This is a useful structure to use
when developing, but you may find it doesn't work so well for other users.

<!-- Can you outline why, briefly, here? Reading on, is it something like:
because some useful functions might be buried within modules that the user is
unaware of -->

This is a major consideration when publishing a crate, especially if people who
might use it are unfamiliar with the implementation to use.

The good news is that, if the structure *isn't* convenient for others to use
from another library, you don't have to rearrange your internal organization:
you can choose to re-export items to make a public structure that's different
to your private structure, using `pub use`.

<!-- Can you give a quick definition of "re-export" here? -->

For example, say we made a library named `art`for modeling artistic concepts.
Within this library is a `kinds` module containing an enum named `Color` and a
`utils` module containing a function named `mix`, as shown in Listing 14-2:

Filename: src/lib.rs

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // ...snip...
    }
}
```

Listing 14-2: An `art` library with items organized into `kinds` and `utils`
modules

Another crate using this library would need `use` statements that call on
`art`, as in Listing 14-3:

Filename: src/main.rs

```rust,ignore
extern crate art;

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

Listing 14-3: A crate using the `art` crate's items with its internal
structure exported

<!--Below -- just to clarify, the "users of this crate" refers to people using
the crate in 14-3 that `uses` art, is that right? I want to make sure I'm
following accurately! -->

However, this requires users of this crate to know that `PrimaryColor` and
`SecondaryColor` are in the `kinds` module, and `mix` is in the `utils` module;
your users shouldn't need to know that. This structure might be useful for
internal organization but doesn't have much meaning from the outside looking in.

To change this, we can add the following `pub use` statements to the original
`art` code from Listing 14-2 to re-export the types to bring them to the top
level, as shown in Listing 14-4:

Filename: src/lib.rs

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    // ...snip...
```

Listing 14-4: Adding `pub use` statements to re-export items

<!-- Will add ghosting in libreoffice /Carol -->

You list and link re-exports on the front page of the crate's API
documentation. Users of the `art` crate can still see and choose to use the
internal structure as in Listing 14-3, or they can use the more convenient
structure from Listing 14-4, as shown in Listing 14-5:

Filename: src/main.rs

```rust,ignore
extern crate art;

use art::PrimaryColor;
use art::mix;

fn main() {
    // ...snip...
}
```

Listing 14-5: A program using the re-exported items from the `art` crate

<!-- Will add ghosting in libreoffice /Carol -->

Creating a useful public API structure is more of an art than a science, and
you can perfect it to suit you. Choosing `pub use` gives you flexibility in how
you expose your crate's internal structure to users. Take a look at some of the
code of crates you've installed to see if their internal structure differs from
their public API.

### Setting up a Crates Acount

Before you can publish any crates, you need to create an account on crates.io
at *https://crates.io* and get an API token. To do so, visit the home page at
*https://crates.io* and log in via a GitHub account---the GitHub account is a
requirement for now, but the site may support other ways of creating an account
in the future. Once you're logged in, visit your account settings at
*https://crates.io/me* and retreive you API key. Then run the `cargo login`
command with your API key, like this:

```text
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

This command will inform Cargo of your API token and store it locally in
*~/.cargo/config*. Note that this token is a **secret** and should not be
shared with anyone else. If it is shared with anyone for any reason, you should
regenerate it immediately by clicking **Reset my API key** from
*https://crates.io/me*.

### Before Publishing a New Crate

Now you have an account, and let's say you already have a crate you want to
publish. Before publishing, you'll need to add some metadata to your crate by
adding it to the `[packages]` section of your `cargo.toml` file.

<!-- Is this right, everything here is relevant to cargo.toml?-->

Your crate will first need a unique name. While you're working on a crate
locally, you may name a crate whatever you'd like, but crate names on crates.io
at *https://crates.io* are allocated on a first-come-first- serve basis. Once a
crate name is taken, it cannot be used for another crate, so search for the
name you'd like to use on the site to find out if it has been taken. Add the
name to `cargo.toml` under `[packages]` like so:

```toml
[package]
name = "guessing_game"
```

However, if you try to publish the crate at this point--using the `cargo
publish command`---as it is when generated by `cargo new`, you'll get a warning
and then an error:

```text
$ cargo publish
    Updating registry `https://github.com/rust-lang/crates.io-index`
warning: manifest has no description, license, license-file, documentation,
homepage or repository.
...snip...
error: api errors: missing or empty metadata fields: description, license.
Please see http://doc.crates.io/manifest.html#package-metadata for how to
upload metadata
```

This is because we're missing some crucial information: a description and
license are required so that people will know what your crate does and under
what terms they may use it. To rectify this error, we need to include this
information in *Cargo.toml*.

Make a description that's just a sentence or two, as it will appear with your
crate in search results and on your crate's page. For the `license` field, you
need to give a *license identifier value*. The Linux Foundation's Software
Package Data Exchange (SPDX) at *http://spdx.org/licenses/* lists some licenses
you can use for this value.

<!-- Can you give an example of what a license identifier value looks like? It
is a alphanumerical code? -->

If you want to use a license that doesn't appear in the SPDX, you need to place
the text of that license in a file and include the file in your project, then
use `license-file` to specify the name of that file instead of giving the
`license` key.

Guidance on which license is right for your project is out of scope for this
book. Many people in the Rust community choose to license their projects in the
same way as Rust itself, with a dual license of `MIT/Apache-2.0`---this
demonstrates that you can also specify multiple license identifiers separated
by a slash.

So, with a unique name, version, author details, your description, and the
license you chose added, the *Cargo.toml* for a project that's ready to publish
might look like this:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A fun game where you guess what number the computer has chosen."
license = "MIT/Apache-2.0"

[dependencies]
```

Be sure to check out the documentation on crates.io at
*http://doc.crates.io/manifest.html#package-metadata*, which describes other
metadata you can specify to ensure your crate can be discovered and used more
easily!

### Publishing to Crates.io

Now that you've created an account, saved your API token, chosen a name for
your crate, and specified the required metadata, you're ready to publish!
Publishing a crate uploads a specific version to crates.io for others to use.

Take care when publishing a crate, because a publish is *permanent*. The
version can never be overwritten, and the code cannot be deleted. One major
goal of *https://crates.io* is to act as a permanent archive of code so that
builds of all projects will continue to work. Allowing deletion of versions
would work against this goal. However, there is no limit to the number of
versions of that crate you can publish.

Let's run the `cargo publish` command again. It should succeed now we've now
specified the required metadata:

```text
$ cargo publish
 Updating registry `https://github.com/rust-lang/crates.io-index`
Packaging guessing_game v0.1.0 (file:///projects/guessing_game)
Verifying guessing_game v0.1.0 (file:///projects/guessing_game)
Compiling guessing_game v0.1.0
(file:///projects/guessing_game/target/package/guessing_game-0.1.0)
 Finished debug [unoptimized + debuginfo] target(s) in 0.19 secs
Uploading guessing_game v0.1.0 (file:///projects/guessing_game)
```

Congratulations! You've now shared your code with the Rust community, and
anyone can easily add your crate as a dependency to their project.

### Publishing a New Version of an Existing Crate

When you've made changes to your crate and are ready to release a new version,
you simply need to change the `version` value specified in your *Cargo.toml*
and republish. Use the Semantic Versioning rules at *http://semver.org/* to
decide what an appropriate next version number is based on the kinds of changes
you've made. Then run `cargo publish` to upload the new version.

### Removing Versions from Crates.io with `cargo yank`

While you can't remove previous versions of a crate, you can prevent any future
projects calling them. This is useful when a version of a crate ends up being
broken for one reason or another. For situations such as this, Cargo supports
*yanking* a version of a crate.

Yanking a version prevents new projects from starting a dependency on that
version, while allowing all existing projects that depend on it to continue to
download and depend on that version. Essentially, a yank means that all
projects with a *Cargo.lock* will not break, while any future *Cargo.lock*
files generated will not use the yanked version.

To yank a version of a crate, run `cargo yank` and specify which version you
want to yank:

```text
$ cargo yank --vers 1.0.1
```

You can also undo a yank, and allow projects to start depending on a version
again, by adding `--undo` to the command:

```text
$ cargo yank --vers 1.0.1 --undo
```

A yank *does not* delete any code. The yank feature is not intended for
deleting accidentally uploaded secrets, for example. If that happens, you must
reset those secrets immediately.

## Cargo Workspaces

In Chapter 12, we built a package that included both a binary crate and a
library crate. You may find, as your project develops, that the library crate
continues to get bigger and you want to split your package up further into
multiple library crates. In this situation, Cargo has a feature called
*workspaces* that can help us manage multiple related packages that are
developed in tandem.

A *workspace* is a set of packages that will all share the same *Cargo.lock*
and output directory. Let's make a project using a workspace, using trivial
code so we can concentrate on the structure of a workspace. We'll have a binary
that uses two libraries: one that will provide an `add_one` method and a second
that will provide an `add_two` method. We'll start by creating a new crate for
the binary:

```text
$ cargo new --bin adder
     Created binary (application) `adder` project
$ cd adder
```

We need to modify the binary package's *Cargo.toml* and add a `[workspace]`
section to tell Cargo the `adder` package is a workspace. Add this at the
bottom of the file:

```toml
[workspace]
```

Like many Cargo features, workspaces support convention over configuration: we
don't need to add anything more than this to *Cargo.toml* to define our
workspace as long as we follow the convention.

<!-- Below -- any crates what depends on, specifically? The program? -->

#### Workspace Dependencies

The workspace convention says that any crates the XXX depends on as
sub-directories will be part of the workspace. We need to add a path dependency
to the `adder` crate to XXX by changing the `[dependencies]` section of
*Cargo.toml* to look like this:

<!-- Above, what is the path dependency actually doing here, can you fill out
the paragraph above? -->

```toml
[dependencies]
add-one = { path = "add-one" }
```

If we add dependencies to *Cargo.toml* that don't have a `path` specified,
those dependencies will be normal dependencies that aren't in this workspace.

#### Building the First Crate

<!-- You can see I'm adding headings, here, trying to add some more navigable
structure -- can you improve these? I'm not sure mine are accurate -->

Next, generate an `add-one` crate within the `adder` directory:

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

In *add-one/src/lib.rs*, let's add an implementation of the `add_one` function:

Filename: add-one/src/lib.rs

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

<!-- below -- Where are we adding the extern crate line? -->

Open up *src/main.rs* for `adder` and add an `extern crate` line to bring the
new `add-one` library crate into scope, and then change the `main` function to
use the `add_one` function. This should look like Listing 14-X:

```rust,ignore
extern crate add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

Listing 14-X:

Let's build it!

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished debug [unoptimized + debuginfo] target(s) in 0.68 secs
```

Note that running `cargo build` in the *adder* directory builds both the
`adder` crate and the `add-one` crate in *adder/add-one*. However, it creates
only one *Cargo.lock* and one *target* directory, both in the *adder*
directory. See if you can add an `add-two` crate in the same way.

<!-- Above -- I have no idea what this means for our project here, can you put
it in more practical terms, or otherwise maybe just explain what this means for
the user? -->

#### Using a Crate in your Workspace

Let's now say that we'd like to use the `rand` crate in our `add-one` crate. As
usual, we'll add it to the `[dependencies]` section in the `Cargo.toml` for
that crate:

Filename: add-one/Cargo.toml

```toml
[dependencies]

rand = "0.3.14"
```

We than have to add `extern crate rand;` to *add-one/src/lib.rs*, then when we
run `cargo build`, it will succeed:

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

The top level *Cargo.lock* now contains information about `add-one`'s
dependency on `rand`. However, even though `rand` is used somewhere in the
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

To fix this, you can edit *Cargo.toml* for the top level and indicate that
`rand` is a dependency for the containing `adder` crate.

#### Adding a Test to a Workshop

For another enhancement, let's add a test of the `add_one::add_one` function
within the `add_one` crate:

Filename: add-one/src/lib.rs

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
see that `cargo test` in a workspace only runs tests for the top level crate.
To run tests for the other crates, we need to specify the package we want to
run a test for with the `-p` argument, like so:

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

<!-- What does that mean, we have to publish them all one at a time?-->

As your project grows, you should consider using a workspace: smaller
components are easier to understand individually than one big blob of code.
Keeping the crates in a workspace can make coordination among them easier if
they work together and are often changed at the same time.

## Installing Binaries from Crates.io with `cargo install`

The `cargo install` command allows you to install and use binary crates
locally. This isn't intended to replace system packages; it's meant to be a
convenient way for Rust developers to install tools that others have shared on

<!--What is a binary target, and how do you know if a package has one? -->

crates.io. Only packages that have binary targets can be installed, and all
binaries are installed into the installation root's *bin* folder. If you
installed Rust using *rustup.rs* and don't have any custom configurations, this
will be `$HOME/.cargo/bin`. Add that directory to your `$PATH` to be able to
run programs you've installed through `cargo install`.

For example, we mentioned in Chapter 12 that there's a Rust implementation of
the `grep` tool for searching files called `ripgrep`. If we want to install
`ripgrep`, we can run:

```text
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 ...snip...
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg
```

The last line of the output shows the location and the name of the installed
binary, which in the case of `ripgrep` is `rg`. As long as the installation
directory is in your `$PATH` as mentioned above, you can then run `rg --help`
and start using a faster, rustier tool for searching files!

## Extending Cargo with Custom Commands

Cargo is designed so you can extend it with new subcommands without having to
modify Cargo itself. If a binary in your `$PATH` is named `cargo-something`,
you can run it as if it were a Cargo subcommand by running `cargo something`.
Custom commands like this are also listed when you run `cargo --list`. Being
able to `cargo install` extensions and then run them just like the built-in
Cargo tools is a super convenient benefit of Rust!

## Summary

Sharing code with Cargo and crates.io is part of what makes the Rust ecosystem
useful for many different tasks. Rust's standard library is small and stable,
but crates are easy to share, use, and improve on a timeline different to the
language itself. Don't be shy about sharing code that's useful to you on
crates.io; it's likely that it will be useful to someone else as well!
