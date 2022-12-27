# An I/O Project: Building a Command Line Program

This chapter is a recap of the many skills you’ve learned so far and an
exploration of a few more standard library features. We’ll build a command line
tool that interacts with file and command line input/output to practice some of
the Rust concepts you now have under your belt.

Rust’s speed, safety, single binary output, and cross-platform support make it
an ideal language for creating command line tools, so for our project, we’ll
make our own version of the classic command line search tool `grep`
(**g**lobally search a **r**egular **e**xpression and **p**rint). In the
simplest use case, `grep` searches a specified file for a specified string. To
do so, `grep` takes as its arguments a file path and a string. Then it reads
the file, finds lines in that file that contain the string argument, and prints
those lines.

Along the way, we’ll show how to make our command line tool use the terminal
features that many other command line tools use. We’ll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We’ll also print error messages to the standard error console stream (`stderr`)
instead of standard output (`stdout`), so, for example, the user can redirect
successful output to a file while still seeing error messages onscreen.

One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version will be fairly simple, but this chapter will give you some of the
background knowledge you need to understand a real-world project such as
`ripgrep`.

Our `grep` project will combine a number of concepts you’ve learned so far:

* Organizing code (using what you learned about modules in [Chapter 7][ch7]<!--
  ignore -->)
* Using vectors and strings (collections, [Chapter 8][ch8]<!-- ignore -->)
* Handling errors ([Chapter 9][ch9]<!-- ignore -->)
* Using traits and lifetimes where appropriate ([Chapter 10][ch10]<!-- ignore
  -->)
* Writing tests ([Chapter 11][ch11]<!-- ignore -->)

We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters [13][ch13]<!-- ignore --> and [17][ch17]<!-- ignore --> will cover in
detail.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html
