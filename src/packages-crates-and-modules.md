# Packages, Crates, & Modules

Now that we've got a better understanding of some of the fundamentals, let's
change gears a bit. Let's talk about the structure of larger Rust programs
and libraries. This will also help you understand the standard library and
its organization, as it is itself a large Rust library!

Rust's system works a bit differently than other languages' that you may have
used in the past. It's worth reading this section carefully; sometimes new
Rustaceans have incorrect expectations based on previous experience.

## Some terminology

First, let's talk about some vocabulary:

* **Crates** are Rust's 'unit of compilation': a library or binary
* **Packages** are a collection of crates
* **Modules** allow you to create namespaces within a crate

Everything forms a tree-like hierarchy: a package contains one or more crates,
and crates contain one or more modules.

We'll start in the middle, though, as in some sense, crates are what's
fundamental here. We can't talk about a collection of crates without knowing
what a crate is! Let's talk about crates.

## Crates

A 'crate' is Rust's name for a library or binary. We mean 'binary' in the sense
of 'executable program' here: both libraries and binaries are compiled into
machine code.

We've already made a number of crates as we've worked through the book! Each of
the examples we've made is a crate. The simplest crates have only one file: our
`lib.rs` or `main.rs`, depending on if we're making a library or an executable.

In the bullet-point above, we defined a crate as a 'unit of compilation'. In
other words, Rust compiles each crate as a whole. You won't get parallel
compiles of a single crate. But, if your crate has dependencies, Cargo will
compile all of those dependencies in parallel, as they're separate crates.

There's one more term related to crates: 'the crate root'. We'll talk about
that more in the 'modules' section below.

Let's make this a bit more concrete. Let's make a simple crate, and then, over
the rest of this section, explore the package, crate, and module system by
changing it. More specifically, let's make a crate that 

## Packages 

## Modules


