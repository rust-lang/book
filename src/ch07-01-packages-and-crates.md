## Packages and Crates

The first parts of the module system we’ll cover are packages and crates.

A _crate_ is the smallest amount of code that the Rust compiler considers at a
time. Even if you run `rustc` rather than `cargo` and pass a single source code
file (as we did all the way back in [“Rust Program Basics”][basics]<!-- ignore
--> in Chapter 1), the compiler considers that file to be a crate. Crates can
contain modules, and the modules may be defined in other files that get
compiled with the crate, as we’ll see in the coming sections.

A crate can come in one of two forms: a binary crate or a library crate.
_Binary crates_ are programs you can compile to an executable that you can run,
such as a command line program or a server. Each must have a function called
`main` that defines what happens when the executable runs. All the crates we’ve
created so far have been binary crates.

_Library crates_ don’t have a `main` function, and they don’t compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects. For example, the `rand` crate we used in [Chapter
2][rand]<!-- ignore --> provides functionality that generates random numbers.
Most of the time when Rustaceans say “crate,” they mean library crate, and they
use “crate” interchangeably with the general programming concept of a “library.”

The _crate root_ is a source file that the Rust compiler starts from and makes
up the root module of your crate (we’ll explain modules in depth in [“Control
Scope and Privacy with Modules”][modules]<!-- ignore -->).

A _package_ is a bundle of one or more crates that provides a set of
functionality. A package contains a _Cargo.toml_ file that describes how to
build those crates. Cargo is actually a package that contains the binary crate
for the command line tool you’ve been using to build your code. The Cargo
package also contains a library crate that the binary crate depends on. Other
projects can depend on the Cargo library crate to use the same logic the Cargo
command line tool uses.

A package can contain as many binary crates as you like, but at most only one
library crate. A package must contain at least one crate, whether that’s a
library or binary crate.

Let’s walk through what happens when we create a package. First, we enter the
command `cargo new my-project`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

After we run `cargo new my-project`, we use `ls` to see what Cargo creates. In
the _my-project_ directory, there’s a _Cargo.toml_ file, giving us a package.
There’s also a _src_ directory that contains _main.rs_. Open _Cargo.toml_ in
your text editor and note that there’s no mention of _src/main.rs_. Cargo
follows a convention that _src/main.rs_ is the crate root of a binary crate
with the same name as the package. Likewise, Cargo knows that if the package
directory contains _src/lib.rs_, the package contains a library crate with the
same name as the package, and _src/lib.rs_ is its crate root. Cargo passes the
crate root files to `rustc` to build the library or binary.

Here, we have a package that only contains _src/main.rs_, meaning it only
contains a binary crate named `my-project`. If a package contains _src/main.rs_
and _src/lib.rs_, it has two crates: a binary and a library, both with the same
name as the package. A package can have multiple binary crates by placing files
in the _src/bin_ directory: Each file will be a separate binary crate.

[basics]: ch01-02-hello-world.html#rust-program-basics
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
