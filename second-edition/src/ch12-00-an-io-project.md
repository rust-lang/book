# An I/O Project Building a Small Grep

This chapter is both a recap of the many skills you've learned so far and an
exploration of a few more standard library features. We're going to build a
command-line tool that interacts with file and command line input/output to
practice some of the Rust you now have under your belt.

Rust's speed, safety, 'single binary' output, and cross-platform support make
it a good language for creating command line tools, so for our project we'll
make our own version of the classic command line tool `grep`. Grep is an
acronym for "Globally search a Regular Expression and Print." In the simplest
use case, `grep` searches a specified file for a specified string using the
following steps:

- Take as arguments a filename and a string.
- Read the file.
- Find lines in the file that contain the string argument.
- Print out those lines.

We'll also show how to use environment variables and print to standard error
instead of standard out; these techniques are commonly used in command line
tools.

One Rust community member, Andrew Gallant, has already created a
fully-featured, very fast version of `grep`, called `ripgrep`. By comparison,
our version of `grep` will be fairly simple, this chapter will give you some of
the background knowledge to help you understand a real-world project like
`ripgrep`.

This project will bring together a number of concepts you've learned so far:

- Organizing code (using what we learned in modules, Chapter 7)
- Using vectors and strings (collections, Chapter 8)
- Handling errors (Chapter 9)
- Using traits and lifetimes where appropriate (Chapter 10)
- Writing tests (Chapter 11)

We'll also briefly introduce closures, iterators, and trait objects, which
Chapters 13 and 17 will cover in detail.

Let's create a new project with, as always, `cargo new`. We're calling our
project `minigrep` to distinguish from the `grep` tool that you may already have
on your system:

```text
$ cargo new --bin minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```
