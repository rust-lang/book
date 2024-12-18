## Preface

This version of the text assumes you’re using Rust 1.62.0 (released 2022-06-30)
or later with `edition="2021"` in *Cargo.toml* of all projects to use Rust 2021
Edition idioms. See “Installation” on page 1 for instructions on installing or
updating Rust, and see Appendix E for information on editions.

The 2021 Edition of the Rust language includes a number of improvements that
make Rust more ergonomic and correct some inconsistencies. On top of a general
update to reflect these improvements, this rendition of the book has a number
of improvements to address specific feedback:

* Chapter 7 contains a new quick reference section on organizing your code into
multiple files with modules.
* Chapter 13 has new and improved closure examples that more clearly illustrate
captures, the `move` keyword, and the `Fn` traits.
* We fixed a number of small errors and imprecise wording throughout the book.
Thank you to the readers who reported them!
Note that any code from earlier renditions of this book that compiled will
continue to compile with the relevant edition in the project’s *Cargo.toml*,
even as you update the Rust compiler version you’re using. That’s Rust’s
backward compatibility guarantees at work!
