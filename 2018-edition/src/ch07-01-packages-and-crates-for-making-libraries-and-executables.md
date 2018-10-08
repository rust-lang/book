# Packages and crates for making libraries and executables

Let's talk about *packages* and *crates*. Here's a summary:

* A "crate" is a binary or library.
* The "crate root" is a source file that is used to build a crate.
* A "package" has a Cargo.toml that describes how to build one or more crates.
  At most one crate can be a library.

So when we type `cargo new`, we're creating a package:

```console
> cargo new foo
> ls foo
src
.gitignore
Cargo.toml
> ls foo/src
main.rs
```

There's a `Cargo.toml`, that checks out. And while there's no *description*
of `main.rs` inside of it, by convention, if you have a `src/main.rs` in the
same directory as a package's `Cargo.toml`, Cargo understands that to be a
*binary* crate with the same name as the package. Likewise, with `src/lib.rs`,
Cargo knows that's a *library* crate with the same name as the package.

What exactly is a crate in this case? Well, we call the `main.rs` or `lib.rs`
file the "crate root", that is, the source file that corresponds to the
crate. This file is passed by Cargo to `rustc` in order to actually build
the library or binary.

A package can have:

* Zero or one library crates
* As many binary crates as it would like
* There must be at least one crate

If we have both `src/main.rs` and `src/lib.rs`, then our package has two
crates: a library and a binary, both with the same name. If we only had one
of the two, we'd have either a single library or binary. But what about more
than one binary? We'll talk about that more in Chapter 14, "More about Cargo
and Crates.io."

That's all we'll say about packages until then. And to learn more about crates,
we need to understand "modules." Read on to find out more!