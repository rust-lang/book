## Packages and Crates

The first parts of the module system we’ll cover are packages and crates.

A *crate* is the smallest amount of code that the Rust compiler considers at a
time. Even if you run `rustc` rather than `cargo` and pass a single source code
file (as we did all the way back in the “Writing and Running a Rust Program”
section of Chapter 1), the compiler considers that file to be a crate. Crates
can contain modules, and the modules may be defined in other files that get
compiled with the crate, as we’ll see in the coming sections.

A crate can come in one of two forms: a binary crate or a library crate.
*Binary crates* are programs you can compile to an executable that you can run,
such as a command-line program or a server. Each must have a function called
`main` that defines what happens when the executable runs. All the crates we’ve
created so far have been binary crates.

*Library crates* don’t have a `main` function, and they don’t compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects. For example, the `rand` crate we used in [Chapter
2][rand]<!-- ignore --> provides functionality that generates random numbers.
Most of the time when Rustaceans say “crate”, they mean library crate, and they
use “crate” interchangeably with the general programming concept of a “library".

The *crate root* is a source file that the Rust compiler starts from and makes
up the root module of your crate (we’ll explain modules in depth in the
[“Defining Modules to Control Scope and Privacy”][modules]<!-- ignore -->
section).

A *package* is a bundle of one or more crates that provides a set of
functionality. A package contains a *Cargo.toml* file that describes how to
build those crates. Cargo is actually a package that contains the binary crate
for the command-line tool you’ve been using to build your code. The Cargo
package also contains a library crate that the binary crate depends on. Other
projects can depend on the Cargo library crate to use the same logic the Cargo
command-line tool uses.

A package can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that’s a
library or binary crate.

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

After we run `cargo new`, we use `ls` to see what Cargo creates. In the project
directory, there’s a *Cargo.toml* file, giving us a package. There’s also a
*src* directory that contains *main.rs*. Open *Cargo.toml* in your text editor,
and note there’s no mention of *src/main.rs*. Cargo follows a convention that
*src/main.rs* is the crate root of a binary crate with the same name as the
package. Likewise, Cargo knows that if the package directory contains
*src/lib.rs*, the package contains a library crate with the same name as the
package, and *src/lib.rs* is its crate root. Cargo passes the crate root files
to `rustc` to build the library or binary.

Here, we have a package that only contains *src/main.rs*, meaning it only
contains a binary crate named `my-project`. If a package contains *src/main.rs*
and *src/lib.rs*, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
