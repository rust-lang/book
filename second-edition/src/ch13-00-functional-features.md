# Functional Language features in Rust: Iterators and Closures

Rust’s design has taken inspiration from a lot of existing languages and
techniques, and one significant influence is *functional programming*.
Programming in a functional style often includes using functions as values, by
passing them in arguments, returning them from other functions, assigning them
to variables for later execution, and so forth. We won’t debate here the issue
of what, exactly, functional programming is or is not, but will instead show
off some features of Rust that are similar to features in many languages often
referred to as functional.

More specifically, we’re going to cover:

* *Closures*: a function-like construct you can store in a variable.
* *Iterators*: a way of processing a series of elements.
* How to use these features to improve on the I/O project from Chapter 12.
* The performance of these features. Spoiler alert: they’re faster than you
  might think!

There are other Rust features influenced by the functional style, like pattern
matching and enums, that we’ve covered in other chapters as well. Mastering
closures and iterators is an important part of writing idiomatic, fast Rust
code, so we’re devoting an entire chapter to them here.
