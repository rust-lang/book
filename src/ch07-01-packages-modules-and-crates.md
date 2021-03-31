Before we do a deep dive into Packages, Crates and Modules, it's helpful to
have an understanding of how each fits in to the overall Rust ecosystem:

## Modules

*Modules* are source based logical organizational units recognized by the Rust
compiler that organize code into semi-autonomous sections with controllable
internal privacy, controllable external visibility, and independent scope.

The *root module* of your code is the first module the compiler encounters and
occupies the *root scope* of the crate. A root module may optionally declare
and import additional modules which are then placed in sub-scopes of the root
scope.

Although the Rust compiler technically allows a file to contain one or more
modules and allows a module to be spread out over multiple files, Cargo
generally expects each module to be in a separate specific file placed in a
specific location.  (Crate roots may contain multiple modules, however.)
We’ll explain modules in depth in the [“Defining Modules to Control Scope and
Privacy”][modules] section).

## Crates

A *crate* is is file based organizational unit understood by Cargo that
organizes source files into a specific directory structure. A crate can either
be a library crate (distributed as source code) that can be used by external
code, or a "binary" crate (also distributed as source code) that compiles to an
executable.

A *crate root* is the source file in a crate that Rust compiles first and which
contains the *root module* of a crate.  

## Packages

A *package* is a file based organizational unit provided by Cargo that contains
one or more Crates. Packages may contain zero or more binary crates (that
build to executables) in addition to zero or one library crate (distributed as
source code that other projects can use). Cargo packages have a pre-determined
directory structure:

```
📁 <package name>
 ├── Cargo.lock
 ├── Cargo.toml
 ├── 📁 src
 │    ├── main.rs    (the root source file for the <package name> binary crate)
 │    ├── lib.rs    (the root source file for the <package name> library crate)
 │    ├── <your_module_name_1>.rs    (a module source file imported with 'use')
 │    ├── 📁 <your_module_name_1>     (<your_module_name_1> submodules go here)
 │    │    ├── <your_submodule_name_1>.rs            (a sub-module source file)
 │    │    ├── ...                               (more sub-module source files)
 │    ├── ...                                   (more module files and folders)
 │    └── 📁 bin
 │         ├── <executable_name_1>.rs              (a single-file binary crate)
 │         ├── ...                             (more single-file binary crates)
 │         ├── 📁 <executable_name_2>             (a multi-source binary crate)
 │         │   ├── main.rs    (the root source file for the multi-source crate)
 │         │   ├── <your_module_name_1>.rs               (a module source file)
 │         │   └── ...
 │         ├── 📁 <executable_name_4>       (another multi-source binary crate)
 │         │   └── ...
 │         └── ...
 ├── 📁 benches
 │    └── ...            (benchmark test code goes here, available for nightly)
 ├── 📁 examples
 │    └── ...                    (example code for the library crate goes here)
 └── 📁 tests
      └── ...                                       (integration tests go here)
```

Packages contain:

* A primary binary crate if they have a main.rs *crate root* file in the
<package name>/src/ folder that will compile into an executable with the same
name as the package
 
* A library crate if they have a lib.rs *crate root* file
in the <package name>/src/ folder with the same name as the package.
 
* Additional binary crates if there are one or more .rs source files in the
<package name>/bin/ folder, each compiled into an executable with the same
name as the source.
 
* Additional binary crates if there are one or more folders in the <package
name>/bin/ folders with *crate root* main.rs files, each compiled into an
executable with the same name as the folder.

The rust compiler doesn't have any knowledge of packages or crates or their
directory structure. In order to use these features, you must use Cargo*,
which is why using Cargo is such an important part of Rust programming.

Because the root crates of a package share the same name as the package itself
and because there can only be one library crate in a package, a "library
package" and its "library crate" share the same name and the two terms are
somewhat synonymous. When we added the *rand dependence* in our example, Cargo
downloaded the *rand package* which contains the *rand library crate*.

(*Advanced readers may wish to know that Cargo automatically generates the
commands required by the Rust compiler to both compile library source crates
into binary library files and then link those files to the compiled binary
crate source code. See the Rust Reference for more information.)

Now let’s walk through what happens when we create a package. First, we enter
the command `cargo new`:

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
and *src/lib.rs*, it has two crates: a library and a binary, both with the same
name as the package. A package can have multiple binary crates by placing files
in the *src/bin* directory: each file will be a separate binary crate.

A crate will group related functionality together in a scope so the
functionality is easy to share between multiple projects. For example, the
`rand` crate we used in [Chapter 2][rand]<!-- ignore --> provides functionality
that generates random numbers. We can use that functionality in our own
projects by bringing the `rand` crate into our project’s scope. All the
functionality provided by the `rand` crate is accessible through the crate’s
name, `rand`.

Keeping a crate’s functionality in its own scope clarifies whether particular
functionality is defined in our crate or the `rand` crate and prevents
potential conflicts. For example, the `rand` crate provides a trait named
`Rng`. We can also define a `struct` named `Rng` in our own crate. Because a
crate’s functionality is namespaced in its own scope, when we add `rand` as a
dependency, the compiler isn’t confused about what the name `Rng` refers to. In
our crate, it refers to the `struct Rng` that we defined. We would access the
`Rng` trait from the `rand` crate as `rand::Rng`.

Let’s move on and talk about the module system!

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
