## Publishing a Crate to Crates.io

We've added crates from crates.io as dependencies of our project. We can choose
to share our code for other people to use as well. Crates.io distributes the
source code of your packages, so it is primarily used to distribute code that's
open source.

Rust and Cargo have some features that can make your published package easier
for people to find and use. We'll talk about some of those features, then cover
how to publish a package.

### Documentation Comments

In Chapter 3, we saw comments in Rust that start with `//`. Rust also has a
second kind of comment: the *documentation comment*. While comments can be
useful if someone is reading your code, you can generate HTML documentation
that displays the contents of documentation comments for public API items meant
for someone who's interested in knowing how to *use* your crate, as opposed to
how your crate is *implemented*. Note that documentation is only generated for
library crates, since binary crates don't have a public API that people need to
know how to use.

Documentation comments use `///` instead of `//` and support Markdown notation
inside. They go just before the item they are documenting. Here's documentation
comments for an `add_one` function:

<figure>

<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 14-1: A documentation comment for a function

</figcaption>
</figure>

`cargo doc` runs a tool distributed with Rust, `rustdoc`, to generate HTML
documentation from these comments. To try this out locally, you can run `cargo
doc --open`, which will build the documentation for your current crate (as well
as all of your crate's dependencies) and open it in a web browser. Navigate to
the `add_one` function and you'll see how the text in the documentation
comments gets rendered.

Adding examples in code blocks in your documentation comments is a way to
clearly demonstrate how to use your library. There's an additional bonus reason
to do this: `cargo test` will run the code examples in your documentation as
tests! Nothing is better than documentation with examples. Nothing is worse
than examples that don't actually work because the code has changed since the
documentation has been written. Try running `cargo test` with the documentation
for the `add_one` function in Listing 14-1; you'll see a section in the test
results like this:

```test
   Doc-tests add-one

running 1 test
test add_one_0 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Try changing the function or the example to see that `cargo test` will catch
that the example no longer works!

There's another style of doc comment, `//!`, to comment containing items (e.g.
crates, modules or functions), instead of the items following it. These are
typically used inside the crate root (lib.rs) or a module's root (mod.rs) to
document the crate or the module as a whole, respectively. Here's the
documentation within the `libstd` module that contains the entire standard
library:

```
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

### Exporting a Convenient Public API with `pub use`

In Chapter 7, we covered how to organize our code into modules with the `mod`
keyword, how to make items public with the `pub` keyword, and how to bring
items into a scope with the `use` keyword. When publishing a crate for people
unfamiliar with the implementation to use, it's worth taking time to consider
if the structure of your crate that's useful for you as you're developing is
what would be useful for people depending on your crate. If the structure isn't
convenient to use from another library, you don't have to rearrange your
internal organization: you can choose to re-export items to make a different
public structure with `pub use`.

For example, say that we made a library named `art` consisting of a `kinds`
module containing an enum named `Color` and a `utils` module containing a
function named `mix` as shown in Listing 14-2:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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
#         SecondaryColor::Green
    }
}
```

<figcaption>

Listing 14-2: An `art` library with items organized into `kinds` and `utils`
modules

</figcaption>
</figure>

In order to use this library, another crate would have `use` statements as in
Listing 14-3:

<figure>
<span class="filename">Filename: src/main.rs</span>

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

<figcaption>

Listing 14-3: A program using the `art` crate's items with its internal
structure exported

</figcaption>
</figure>

Users of this crate shouldn't need to know that `PrimaryColor` and
`SecondaryColor` are in the `kinds` module, and `mix` is in the `utils` module;
that structure might be useful for internal organization but doesn't have much
meaning from the outside looking in.

To change this, we can add the following `pub use` statements to the code from
Listing 14-2 to re-export the types at the top level, as shown in Listing 14-4:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    // ...snip...
#     pub enum PrimaryColor;
#     pub enum SecondaryColor;
# }
#
# pub mod utils {
#     pub fn mix() {}
# }
```

<figcaption>

Listing 14-4: Adding `pub use` statements to re-export items

</figcaption>
</figure>

<!-- Will add ghosting in libreoffice /Carol -->

Re-exports are listed and linked on the front page of the crate's API
documentation. Users of the `art` crate can still see and choose to use the
internal structure as in Listing 14-3, or they can use the more convenient
structure from Listing 14-4, as shown in Listing 14-5:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate art;

use art::PrimaryColor;
use art::mix;

fn main() {
    // ...snip...
}
```

<figcaption>

Listing 14-5: Using the re-exported items from the `art` crate

</figcaption>
</figure>

<!-- Will add ghosting in libreoffice /Carol -->

Creating a useful public API structure is more of an art than a science.
Choosing `pub use` gives you flexibility in how you expose your crate's
internal structure to users. Take a look at some of the code of crates you've
installed to see if their internal structure differs from their public API.

### Before Your First Publish

Before being able to publish any crates, you'll need to create an account on
[crates.io] and get an API token. To do so, [visit the home page][crates.io]
and log in via a GitHub account. A GitHub account is a requirement for now, but
the site might support other ways of creating an account in the future. Once
you're logged in, visit your [Account Settings] page and run the `cargo login`
command with the API key as the page specifies, which will look something like
this:

[crates.io]: https://crates.io
[Account Settings]: https://crates.io/me

```text
$ cargo login abcdefghijklmnopqrstuvwxyz012345
```

This command will inform Cargo of your API token and store it locally in
*~/.cargo/config*. Note that this token is a **secret** and should not be
shared with anyone else. If it gets shared with anyone for any reason, you
should regenerate it immediately.

### Before Publishing a New Crate

First, your crate will need a unique name. While you're working on a crate
locally, you may name a crate whatever you'd like, but crate names on
[crates.io] are allocated on a first-come-first- serve basis. Once a crate name
is taken, it cannot be used for another crate, so check on the site that the
name you'd like is available.

If you try to publish a crate as generated by `cargo new`, you'll get a warning
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

We can include more information about our package in *Cargo.toml*. Some of
these fields are optional, but a description and a license are required in
order to publish so that people will know what your crate does and under what
terms they may use it.

The description appears with your crate in search results and on your crate's
page. Descriptions are usually a sentence or two. The `license` field takes a
license identifier value, and the possible values have been specified by the
Linux Foundation's [Software Package Data Exchange (SPDX)][spdx]. If you would
like to use a license that doesn't appear there, instead of the `license` key,
you can use `license-file` to specify the name of a file in your project that
contains the text of the license you want to use.

Guidance on which license is right for your project is out of scope for this
book. Many people in the Rust community choose to license their projects in the
same way as Rust itself, with a dual license of `MIT/Apache-2.0`, which
demonstrates that you can specify multiple license identifiers separated by a
slash. So the *Cargo.toml* for a project that is ready to publish might look
like this:

[spdx]: http://spdx.org/licenses/

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A fun game where you guess what number the computer has chosen."
license = "MIT/Apache-2.0"

[dependencies]
```

Be sure to check out the [documentation on crates.io][other-metadata] that
describes other metadata you can specify to ensure your crate can be discovered
and used more easily!

[other-metadata]: http://doc.crates.io/manifest.html#package-metadata

### Publishing to Crates.io

Now that we've created an account, saved our API token, chosen a name for our
crate, and specified the required metadata, we're ready to publish! Publishing
a crate is when a specific version is uploaded to be hosted on crates.io.

Take care when publishing a crate, because a publish is **permanent**. The
version can never be overwritten, and the code cannot be deleted. However,
there is no limit to the number of versions which can be published.

Let's run the `cargo publish` command, which should succeed this time since
we've now specified the required metadata:

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
change the `version` value specified in your *Cargo.toml*. Use the [Semantic
Versioning rules][semver] to decide what an appropriate next version number is
based on the kinds of changes you've made. Then run `cargo publish` to upload
the new version.

[semver]: http://semver.org/

### Removing Versions from Crates.io with `cargo yank`

Occasions may arise where you publish a version of a crate that actually ends
up being broken for one reason or another, such as a syntax error or forgetting
to include a file. For situations such as this, Cargo supports *yanking* a
version of a crate.

Marking a version of a crate as yanked means that no projects will be able to
start depending on that version, but all existing projects that depend on that
version will continue to be allowed to download and depend on that version. One
of the major goals of crates.io is to act as a permanent archive of code so
that builds of all projects will continue to work, and allowing deletion of a
version would go against this goal. Essentially, a yank means that all projects
with a *Cargo.lock* will not break, while any future *Cargo.lock* files
generated will not use the yanked version.

A yank **does not** delete any code. The yank feature is not intended for
deleting accidentally uploaded secrets, for example. If that happens, you must
reset those secrets immediately.

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
