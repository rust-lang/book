## Packages and Crates for Making Libraries and Executables

Let's talk about *packages* and *crates*. Here's a summary:

* A *crate* is a binary or library.
* The *crate root* is a source file that is used to know how to build a crate.
* A *package* has a *Cargo.toml* that describes how to build one or more crates.
  At most one crate in a package can be a library.

So when we type `cargo new`, we're creating a package:

```text
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Because Cargo created a *Cargo.toml*, that means we now have a package. If we
look at the contents of *Cargo.toml*, there's no mention of *src/main.rs*.
However, Cargo's conventions are that if you have a *src* directory containing
*main.rs* in the same directory as a package's *Cargo.toml*, Cargo knows this
package contains a binary crate with the same name as the package, and
*src/main.rs* is its crate root. Another convention of Cargo's is that if the
package directory contains *src/lib.rs*, the package contains a library crate
with the same name as the package, and *src/lib.rs* is its crate root. The
crate root files are passed by Cargo to `rustc` to actually build the library
or binary.

A package can contain zero or one library crates and as many binary crates as
you'd like. There must be at least one crate (either a library or a binary) in
a package.

If a package contains both *src/main.rs* and *src/lib.rs*, then it has two
crates: a library and a binary, both with the same name. If we only had one of
the two, the package would have either a single library or binary crate. We'll
talk about how to have more than one binary crate in a package in the TODO
section of Chapter 14.

<!-- Steve, are you talking about the workspaces section of chapter 14? That
actually talks about having a binary and two libraries in a workspace..... we
don't talk about src/bin/*.rs in chapter 14 at all, which is what i expected
you to be alluding to here? /Carol -->

Next, let's talk about modules!
