# Introduction

Welcome to “The Rust Programming Language”, an introductory book about Rust.
Rust is a programming language that’s focused on safety, concurrency, and
speed. It maintains these goals without having a garbage collector, making it a
useful language for a number of use cases other languages aren’t good at:
embedding in other languages, programs with specific space and time
requirements, and writing low-level code, like device drivers and operating
systems. It improves on current languages targeting this space by having a
number of compile-time safety checks that produce no runtime overhead, while
eliminating all data races. Rust also aims to achieve ‘zero-cost abstractions’
even though some of these abstractions feel like those of a high-level
language. Even then, Rust still allows precise control like a low-level
language would.

This book is written for a reader who already knows how to program in at least
one programming language. Which language that is does not matter very much,
though you may have an easier time if you’ve programmed in a low-level language
with manual memory allocation.

After reading this book, you should be comfortable writing Rust programs. We’ll
be learning Rust through small, focused examples that demonstrate each topic.
The chapters build upon each other, so if you skip ahead, you may have to skip
back to refer to a previous concept.

## Contributing to the book

This book is open source. If you find an error, please don’t hesitate to file an
issue or send a pull request [on GitHub].

[on GitHub]: https://github.com/rust-lang/book
