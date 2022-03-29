## Packages and Crates

The first parts of the module system we’ll cover are packages and crates.

A *package* is one or more crates that provide a set of functionality. A
package contains a *Cargo.toml* file that describes how to build those crates.

A *crate* can be a binary crate or a library crate. *Binary crates* are
programs you can compile to an executable that you can run, such as a
command-line program or a server. They must have a function called `main` that
defines what happens when the executable runs. All the crates we’ve created so
far have been binary crates.

*Library crates* don’t have a `main` function, and they don’t compile to an
executable. They define functionality intended to be shared with multiple
projects. For example, the `rand` crate we used in [Chapter 2][rand]<!-- ignore
--> provides functionality that generates random numbers.

The *crate root* is a source file that the Rust compiler starts from and makes
up the root module of your crate (we’ll explain modules in depth in the
[“Defining Modules to Control Scope and Privacy”][modules]<!-- ignore -->
section).

Several rules determine what a package can contain. A package can contain at
most one library crate. It can contain as many binary crates as you’d like, but
it must contain at least one crate (either library or binary).

Let’s walk through what happens when we create a package. First, we enter the
command `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

When we entered the command, Cargo created a *Cargo.toml* file, giving us a
package. Looking at the contents of *Cargo.toml*, there’s no mention of
*src/main.rs* because Cargo follows a convention that *src/main.rs* is the
crate root of a binary crate with the same name as the package. Likewise, Cargo
knows that if the package directory contains *src/lib.rs*, the package contains
a library crate with the same name as the package, and *src/lib.rs* is its
crate root. Cargo passes the crate root files to `rustc` to build the library
or binary.

Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
