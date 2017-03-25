# Introduction

Welcome to “The Rust Programming Language,” an introductory book about Rust.
Rust is a programming language that’s focused on safety, speed, and
concurrency. Its design lets you create programs that have the performance and
control of a low-level language, but with the powerful abstractions of a
high-level language. These properties make Rust suitable for programmers who
have experience in languages like C and are looking for a safer alternative, as
well as those from languages like Python who are looking for ways to write code
that performs better without sacrificing expressiveness.

Rust performs the majority of its safety checks and memory management decisions
at compile time, so that your program's runtime performance isn't impacted. This
makes it useful in a number of use cases that other languages aren’t good at:
programs with predictable space and time requirements, embedding in other
languages, and writing low-level code, like device drivers and operating
systems. It's also great for web applications: it powers the Rust package
registry site, [crates.io]!  We're excited to see what *you* create with Rust.

[crates.io]: https://crates.io/

This book is written for a reader who already knows how to program in at least
one programming language. After reading this book, you should be comfortable
writing Rust programs. We’ll be learning Rust through small, focused examples
that build on each other to demonstrate how to use various features of Rust as
well as how they work behind the scenes.

## Open Source

Just like the Rust Programming Language, this book is open source.
This means you are free to modify, build, copy, and reuse to the book,
subject to the license conditions.
You can find the source code and license information [on GitHub].

You can also report errata by filing an
issue or sending a pull request [on GitHub].

As this book is currently pre-release,
other contributions are also appreciated.
Please see [CONTRIBUTING.md][contrib] on GitHub for more details.

[on GitHub]: https://github.com/rust-lang/book
[contrib]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md