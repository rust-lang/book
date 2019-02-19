## Packages and Crates

The first parts of the module system we’ll cover are *packages* and *crates*.

A *crate* is a binary or library.

<!--- Below: who is using the source file to build the crate? Can you specify
in text? --->
<!-- Done /Carol -->

The *crate root* is a source file that the Rust compiler starts from, and makes
up the root module of your crate (we'll be explaining modules in depth in the
["Defining Modules to Control Scope and Privacy"][modules]<!-- ignore -->
section). A *package* is one or more crates that, together, provide a set of
functionality. A package contains a *Cargo.toml* that describes how to build
those crates.

<!--- Above: can you say what a package is? don't think this is technically
correct but something structured like "A package is a group of files that
describe how to build one or more crate. They include a Cargo.toml file. ": or
something like that. --->
<!-- Done /Carol -->

There are several rules about what a package can contain. A package *must*
contain zero or one library crates, and no more. It can contain as many binary
crates as you’d like, but it must contain at least one crate (either library or
binary).

<!--- are they already familiar with the distinction between these two
types of crate? --->
<!-- Yes, we covered this in chapter 2. /Carol -->

Now let’s walk through what happens when you create a package. First, we enter
the command `cargo new`:

```text
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

<!-- Should there be mention of src.main? -->
<!-- No, I've clarified /Carol -->

When we entered the command, Cargo created a *Cargo.toml*, giving us a package.
If we look at the contents of *Cargo.toml*, there’s no mention of *src/main.rs*
because Cargo follows a convention that *src/main.rs* is the crate root of a
binary crate with the same name as the package. Likewise, Cargo knows that if
the package directory contains *src/lib.rs*, then the package contains a
library crate with the same name as the package, and *src/lib.rs* is its crate
root. Cargo passes the crate root files to `rustc` to actually build the
library or binary.

<!--- below: can you introduce these hypotheticals by describing what's in the
package above so the reader has a concrete example? --->
<!-- Done /Carol -->

Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains both
*src/main.rs* and *src/lib.rs*, then it has two crates: a library and a binary,
both with the same name as the package. A package can have multiple binary
crates by placing files in the *src/bin* directory: each file will be a
separate binary crate.

A crate groups related functionality together in a scope so that the
functionality is easy to share between multiple projects. For example, the
`rand` crate that we used in Chapter 2 provides functionality having to do with
generating random numbers. We can use that functionality in our own projects by
bringing the `rand` crate into our project’s scope. All of the functionality
provided by the `rand` crate is accessible through the crate’s name, `rand`.

In addition to making it clear whether functionality is defined in our crate or
the `rand` crate, keeping a crate’s functionality in its own scope prevents
conflicts that could arise. For example, the `rand` crate provides a trait
named `Rng`. We can also define a `struct` named `Rng` in our own crate.
Because a crate’s functionality is namespaced in its own scope, when we add
`rand` as a dependency, the compiler isn’t confused about what the name `Rng`
refers to. In our crate, it refers to the `struct Rng` that we defined. The
`Rng` trait from the `rand` crate is accessible as `rand::Rng`.

<!--- how do crates relate to scope? Could you address in text --->
<!-- Done, above /Carol -->

Now that we’ve covered crates, let’s talk about the module system!

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
