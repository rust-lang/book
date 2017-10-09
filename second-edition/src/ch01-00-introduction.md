# Introduction

Welcome to “The Rust Programming Language,” an introductory book about Rust.
Rust is a fast, friendly, and fearless programming language. As a
general-purpose programming language, Rust is used in production by over 100
companies for a variety of tasks, such as command line tools, web services,
devops tooling, embedded devices, audio and video analysis and transcoding,
cryptocurrencies, bioinformatics, search enginges, internet of things
applications, machine learning, and even major parts of the Firefox web
browser.

## What kind of language is Rust?

There are many ways to categorize and talk about programming languages; here
are some examples of ones that apply to Rust:

### Friendly

Rust believes that programming languages are just as much for people as they
are for computers. Notably, this means that both people and computers exist; we
can't ignore the programmer, but we can't ignore the computer either. On the
"people" side, this is expressed in many ways: through an inclusive project
management, to a focus on meaningful error messages, to a helpful community.
Rust should be a programming language that people enjoy working with, and
one that helps them get their program written and working well with ease.

Some programming languages attempt to accomplish these goals by completely
ignoring the computer side of the equation, and only focusing on the
programmer. Rust, however, does not completely hide the underlying machine's
concerns either. We'll talk more about this side of the equation in later
bullet points.

### AOT compiled

Rust's current implementation is "ahead-of-time compiled", meaning that you
take your program, run it through a compiler, and get a binary program back.
Rust does not have an interpreter, nor a virtual machine. If your program is
purely Rust, you will generally need no other libraries installed on a system
to run a Rust program. If you include C libraries as well, you can choose to
dynamically or statically link them, if that's something you're interested in.

### Statically typed, with powerful inference

Rust has a strong, statically typed type system. At the same time, it
contains a great type inference scheme as well. By "statically typed", we
mean that Rust must know at compile time what the type of each of your
variables and such are. However, with inference, the compiler will attempt to
figure this information out on its own, and so you generally don't need to
actually tell the compiler what the type of something is. This gives Rust a
bit of a blended feel between a statically and dynamically typed language in
some sense: you'll write type signatures for functions, but rarely use types
within the body of those functions.

### Fearless

There's more to types than just "is this a string or an integer?" however.
Rust's type system is able to prove many interesting properties about your
programs, and do them entirely at compile time. This has led us to call Rust
a "fearless" programming language, as the compiler is constantly double-checking
your work, making many tasks easier by pointing out where you've made mistakes.
Rust programmers, through a combination of strong typing and software testing,
generally report a high level of confidence that their programs work correctly.

### Fast

Rust is designed in and for the age of optimizing compilers. Rust is
generally as fast as they come, with speeds regularly as fast as languages
like C and C++. You can write slow programs in any language, of course, and
so Rust is not a panacea in this regard. However, the language is designed in
such a way that the "happy path" of writing Rust is quite speedy. You
shouldn't need to particularly contort your Rust code to achieve great speed.

One way in which Rust achieves this is by all of these compile-time checks.
Rust can check your program's correctness at compile time, and therefore, not
need to emit expensive run-time checks. The fastest code is code that's never
even executed.

### Low-level or "systems"

Above, we mentioned that Rust has no virtual machine or interpreter. It also
has no garbage collector, and has a similar level of runtime to C: not much.
Rust gives you the tools to reach into the lowest level of your computer;
there are several hobby operating systems in Rust, and many people working on
embedded devices, filesystems, and device drivers.

Some people call this kind of work "systems programming", but that term has
different meanings depending on who you ask. Applied to Rust, it means these
very-low-level concerns: no garbage collector, access to hardware, and
predictable code.

### High-level

At the same time, many Rust programmers write programs that have no need to
access these low-level details, and as a language, Rust can often feel more
like a higher-level one than a low-level one. For example, we don't write out
C-style `for` loops in Rust, we use iterators. Rust provides many features
that make writing it day-to-day feel like something a bit higher-level than
you may expect from a language suited to doing such low-level tasks.

### Concurrent

Finally, concurrency is very important to Rust. Several parts of Rust's type
system are concurrency-oriented, and Rust can even determine some concurrency
properties of your program at compile-time, which is quite a feat!
Specifically, Rust prevents "data races" at compile-time, which, for
languages without a garbage collector, is generaly novel in industry.

## Who this book is for

This book assumes that you've written code in some other programming
language, but doesn't make any assumptions about which ones. We've tried to
make the material broadly accessible to those from a wide variety of
programming backgrounds. We don't spend a lot of time talking about what
programming *is* or how to think about it; someone new to programming
entirely would be better served by reading a book specifically for those new
to programming.

## How to use this book

This book generally assumes that you're reading it back-to-front, that is, later
chapters build on top of concepts in earlier chapters, and earlier chapters may
not dig into details on a topic, revisiting the topic in a later chapter.

There are two kinds of chapters in this book: concept chapters, and project
chapters. In concept chapters, you'll learn something about some aspect of Rust.
In the project chapters, we'll build small programs together, applying what we've
learned so far. Chapters 2, 12, and 20 are project chapters, the rest are concept
chapters.

Additionally, Chapter 2 is a sort of hands-on introduction to Rust as a
language. We'll cover concepts at a high level, and later chapters will go
into them in detail. If you're the kind of person who likes to get their
hands dirty right away, Chapter 2 is great for that. If you're *really* that
kind of person, you may even wish to skip over chapter 3, which covers
features that are very similar to other programming languages. By contrast,
if you're a particularly meticulous learner who prefers to learn every detail
before moving onto the next, you may want to skip chapter two and go straight
to chapter 3.

In the end, there's no wrong way to read a book: if you want to skip ahead,
go for it! You may have to jump back if you find things confusing. Do
whatever works for you.

Finally, there are some appendices. These contain useful information about
the language in a more reference-like format.

## Contributing to the book

This book is open source. If you find an error, please don’t hesitate to file an
issue or send a pull request [on GitHub]. Please see [CONTRIBUTING.md] for
more details.

[on GitHub]: https://github.com/rust-lang/book
[CONTRIBUTING.md]: https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md
