# Introduction

Welcome to “The Rust Programming Language,” an introductory book about Rust.

Rust is a programming language that helps you write faster, more robust
software. High-level ergonomics and low-level control are often at odds with
each other in programming language design; Rust stands to challenge that.
Through balancing powerful technical capacity and a great developer experience,
Rust gives you the option to control low-level details (such as memory usage)
without all the hassle traditionally associated with such control.

## Who Rust is For

Rust is great for many people for a variety of reasons. Let's discuss a few of
the most important groups.

### Teams of Developers

Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. In other
languages, developers often have to ensure safety manually through code
reviews. In Rust, the compiler plays a gatekeeper role by refusing to compile
code that does not follow rules of memory safety. By working alongside the
compiler, the team can spend more time focusing on the logic of the program
rather than the safety.

Rust also brings contemporary developer tools to the systems programming world:

* Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies consistent across the Rust ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers IDE integration for code completion and
  inline error messages.

By using these and other tools in the Rust ecosystem, developers can be
productive while writing systems-level code.

### Students

Rust is for students and people who are interested in learning about systems
concepts. Many people have learned about topics like operating systems
development through Rust. The community is happy to answer student questions.
Through efforts such as this book, the Rust teams want to make systems concepts
more accessible to more people, especially those getting started with
programming.

### Companies

Rust is used in production by hundreds of companies, large and small, for a
variety of tasks, such as command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, internet of things applications, machine
learning, and even major parts of the Firefox web browser.

### Open Source Developers

Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We’d love for you to contribute to the Rust
language.

### People Who Value Speed and Stability

By speed, we mean both the speed of the programs that Rust lets you create and
the speed at which Rust lets you write them. The Rust compiler's checks ensure
stability through feature additions and refactoring, as opposed to brittle
legacy code in languages without these checks that developers are afraid to
modify. By striving for zero-cost abstractions, higher level features that
compile to lower level code as fast as code written manually, Rust endeavors to
make safe code be fast code as well.

This isn't a complete list of everyone the Rust language hopes to support, but
these are some of the biggest stakeholders. Overall, Rust's greatest ambition
is to take trade-offs that have been accepted by programmers for decades and
eliminate the dichotomy. Safety *and* productivity. Speed *and* ergonomics.
Give Rust a try, and see if its choices work for you.

## Who This Book is For

This book assumes that you've written code in some other programming language,
but doesn't make any assumptions about which one. We've tried to make the
material broadly accessible to those from a wide variety of programming
backgrounds. We don't spend a lot of time talking about what programming *is*
or how to think about it; someone new to programming entirely would be better
served by reading a book specifically providing an introduction to programming.

## How to Use This Book

This book generally assumes that you're reading it front-to-back, that is,
later chapters build on top of concepts in earlier chapters, and earlier
chapters may not dig into details on a topic, revisiting the topic in a later
chapter.

There are two kinds of chapters in this book: concept chapters, and project
chapters. In concept chapters, you'll learn about an aspect of Rust. In the
project chapters, we'll build small programs together, applying what we've
learned so far. Chapters 2, 12, and 20 are project chapters; the rest are
concept chapters.

Additionally, Chapter 2 is a hands-on introduction to Rust as a language. We'll
cover concepts at a high level, and later chapters will go into them in detail.
If you're the kind of person who likes to get their hands dirty right away,
Chapter 2 is great for that. If you're *really* that kind of person, you may
even wish to skip over Chapter 3, which covers features that are very similar
to other programming languages, and go straight to Chapter 4 to learn about
Rust's ownership system. By contrast, if you're a particularly meticulous
learner who prefers to learn every detail before moving onto the next, you may
want to skip Chapter 2 and go straight to Chapter 3.

In the end, there's no wrong way to read a book: if you want to skip ahead, go
for it! You may have to jump back if you find things confusing. Do whatever
works for you.

An important part of the process of learning Rust is learning how to read the
error messages that the compiler gives you. As such, we'll be showing a lot of
code that doesn't compile, and the error message the compiler will show you in
that situation. As such, if you pick a random example, it may not compile!
Please read the surrounding text to make sure that you didn't happen to pick
one of the in-progress examples.

Finally, there are some appendices. These contain useful information about the
language in a more reference-like format.

## Contributing to the Book

This book is open source. If you find an error, please don’t hesitate to file
an issue or send a pull request [on GitHub]. Please see [CONTRIBUTING.md] for
more details.

[on GitHub]: https://github.com/rust-lang/book
[CONTRIBUTING.md]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md
