# An I/O Project: Building a Command Line Program

This chapter is both a recap of the many skills you’ve learned so far and an
exploration of a few more standard library features. We’re going to build a
command line tool that interacts with file and command line input/output to
practice some of the Rust you now have under your belt.

Rust’s speed, safety, *single binary* output, and cross-platform support make
it a good language for creating command line tools, so for our project we’ll
make our own version of the classic command line tool `grep`. Grep is an
acronym for “**G**lobally search a **R**egular **E**xpression and **P**rint.”
In the simplest use case, `grep` searches a specified file for a specified
string. To do so, `grep` takes a filename and a string as its arguments, then
reads the file and finds lines in that file that contain the string argument.
It’ll then print out those lines.

Along the way, we’ll show how to make our command line tool use features of the
terminal that many command line tools use. We’ll read the value of an
environment variable in order to allow the user to configure the behavior of
our tool. We’ll print to the standard error console stream (`stderr`) instead
of standard output (`stdout`) so that, for example, the user can choose to
redirect successful output to a file while still seeing error messages on the
screen.

One Rust community member, Andrew Gallant, has already created a
fully-featured, very fast version of `grep`, called
[`ripgrep`](https://github.com/BurntSushi/ripgrep)<!-- ignore -->. By
comparison, our version of `grep` will be fairly simple, but this chapter will
give you some of the background knowledge to help you understand a real-world
project like `ripgrep`.

This project will bring together a number of concepts you’ve learned so far:

* Organizing code (using what we learned in modules, Chapter 7)
* Using vectors and strings (collections, Chapter 8)
* Handling errors (Chapter 9)
* Using traits and lifetimes where appropriate (Chapter 10)
* Writing tests (Chapter 11)

We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters 13 and 17 will cover in detail.
