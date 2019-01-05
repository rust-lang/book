## Packages and Crates

The first parts of the module system we'll cover are *packages* and *crates*.

A *crate* is a binary or library.

<!--- Below: who is using the source file to build the crate? Can you specify
in text? --->

The *crate root* is a source file that X uses to know how to build a crate.
A *package* has a *Cargo.toml* that describes how to build one or more crates.

<!--- Above: can you say what a package is? don't think this is technically
correct but something structured like "A package is a group of files that
describe how to build one or more crate. They include a Cargo.toml file. ": or
something like that. --->

There are several rules about what a package can contain. A package *must*
contain zero or one library crates, and no more. It can contain as many binary
crates as you’d like, but it must contain at least one crate (either library or
binary). <!--- are they already familiar with the distinction between these two
types of crate? --->

Now let's walk through what happens when you create a package. First, we enter
the command `cargo new`:

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

When we entered the command, Cargo created a *Cargo.toml*, giving us a package.
If we look at the contents of *Cargo.toml*, there’s no mention of *src/main.rs*.

<!-- Should there be mention of src.main? -->

However, Cargo’s conventions are such that if you have a *src* directory
containing *main.rs* in the same directory as a package’s *Cargo.toml*, Cargo
will know the pckage contains a binary crate with the same name as the package,
and *src/main.rs* is its crate root. Likewise, Cargo knows that if the package
directory contains *src/lib.rs*, then the package contains a library crate with
the same name as the package, and *src/lib.rs* is its crate root. Cargo passes
the crate root files to `rustc` to actually build the library or binary.

<!--- below: can you introduce these hypotheticals by describing what's in the
package above so the reader has a concrete example? --->

Here, we have a package that contains [. . .] , meaning [. . .]. If a package
contains both *src/main.rs* and *src/lib.rs*, then it has two crates: a library
and a binary, both with the same name. <!--- what name ---> If we only had one
of the two, the package would have either a single library or binary crate. A
package can have multiple binary crates by placing files in the *src/bin*
directory: each file will be a separate binary crate.

Now that we've covered crates, let’s talk about the module system!

<!--- how do crates relate to scope? Could you address in text --->
