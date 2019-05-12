# The Rust Programming Language

*by Steve Klabnik and Carol Nichols, with contributions from the Rust Community*

Welcome to The Rust Programming Language book! This version of the text assumes
you are using Rust 1.31.0 or later, with `edition="2018"` in *Cargo.toml* of
all projects to use Rust 2018 Edition idioms. See the [“Installation” section
of Chapter 1][install]<!-- ignore --> to install or update Rust, and see the
new [Appendix E][editions]<!-- ignore --> for information on what editions of
Rust are.

The 2018 Edition of the Rust language includes a number of improvements to make
Rust more ergonomic and easier to learn. This printing of the book has a number
of changes to reflect the improvements:

- Chapter 7, "Managing Growing Projects with Packages, Crates, and Modules",
  has been mostly rewritten. The module system and the way paths work in the
  2018 Edition have been made more consistent.
- Chapter 10 has new sections titled "Traits as Parameters" and "Returning
  Types that Implement Traits" that explain the new `impl Trait` syntax.
- Chapter 11 has a new section "Using `Result<T, E>` in Tests" that shows how
  to write tests that can use the `?` operator.
- The "Advanced Lifetimes" section of Chapter 19 has been removed as compiler
  improvements have made the constructs in that section even more rare.
- The previous Appendix D on macros has been expanded to include procedural
  macros, and has been moved to the "Macros" section in Chapter 19.
- Appendix A, "Keywords", also explains the new raw identifiers feature that
  enables code written in Rust 2015 and Rust 2018 to interoperate.
- Appendix D now covers useful development tools that have been recently
  released.
- We fixed a number of small errors and imprecise wording throughout the book.
  Thank you to the readers who reported them!

Note that any code in the first printing of *The Rust Programming Language*
that compiled will continue to compile without `edition="2018"` in the
project's *Cargo.toml*, even as you update the version of the Rust compiler
that you're using. That's Rust's backwards compatibility guarantees at work!

The HTML format is available online at
[https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)
and offline with installations of Rust made with `rustup`; run `rustup docs
--book` to open.

This text is available in [paperback and ebook format from No Starch
Press][nsprust].

[install]: ch01-01-installation.html
[editions]: appendix-05-editions.html
[nsprust]: https://nostarch.com/rust
