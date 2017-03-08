# An I/O Project

We've learned a lot over the last few chapters. Let's take that new knowledge
and apply it by building a project together. Along the way, we'll learn a bit
more about Rust's standard library.

So what should we build? One that uses Rust's strengths. A great use of Rust is
for command line tools: Rust's speed, safety, 'single binary' output, and
cross-platform support make it a good language choice for this kind of task. So
we'll make our own version of a classic command line tool: `grep`. `grep` is
short for "Globally search a Regular Expression and Print." In the
simplest use case, it does this:

1. Takes a filename and a string as arguments.
2. Reads the file.
3. Finds lines in the file that contain the string argument.
4. Prints out those lines.

In addition, we'll add one extra feature: an environment variable that will
allow us to search for the string argument in a case-insensitive way.

There's another great reason to use `grep` as an example project: a very
fully-featured version of `grep` has already been created in Rust by a
community member, Andrew Gallant. It's called `ripgrep`, and it's very,
very fast. While our version of `grep` will be fairly simple, you'll have
some of the background knowledge to understand that project if you want to see
something more real-world.

This project will bring together a number of things we learned previously:

- Organize code (using what we learned in modules, Chapter 7)
- Use vectors and strings (collections, Chapter 8)
- Handle errors (Chapter 9)
- Use traits and lifetimes where appropriate (Chapter 10)
- Have tests (Chapter 11)

Additionally, we'll briefly introduce closures, iterators, and trait objects,
which Chapters XX, YY, and ZZ respectively are about to cover in detail.

Let's create a new project with, as always, `cargo new`:

```text
$ cargo new --bin greprs
     Created binary (application) `greprs` project
$ cd greprs
```

We're calling our version of `grep` 'greprs', so that we don't confuse any of
our users into thinking that it's the more fully-featured version of `grep`
they may already have installed on their system.
