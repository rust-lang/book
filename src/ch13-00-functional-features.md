# Functional Language features in Rust - Iterators and Closures

Rust's design has taken inspiration from a lot of previous work. One of Rust's
influences is functional programming, where functions are values that can be
used as arguments or return values to other functions, assigned to variables,
and so forth. We're going to sidestep the issue of what, exactly, functional
programming is or is not, and instead show off some features of Rust that
are similar to features in many languages referred to as functional.

More specifically, we're going to cover:

* *Closures*, a function-like construct you can store in a variable.
* *Iterators*, a way of processing series of elements.
* How to use these features to improve upon the project from the last chapter.
* The performance of these features. Spoiler alert: they're faster than you
  might think!

This is not a complete list of Rust's influence from the functional style:
pattern matching, enums, and many other features are too. But mastering
closures and iterators are an important part of writing idiomatic, fast Rust
code.
