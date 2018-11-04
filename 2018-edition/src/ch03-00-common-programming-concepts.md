# Common Programming Concepts

This chapter covers concepts that appear in almost every programming language
and how they work in Rust. Many programming languages have much in common at
their core. None of the concepts presented in this chapter are unique to Rust,
but we’ll discuss them in the context of Rust and explain the conventions
around using these concepts.

Specifically, you’ll learn about variables, basic types, functions, comments,
and control flow. These foundations will be in every Rust program, and learning
them early will give you a strong core to start from.

## Keywords

The Rust language has a set of *keywords* that are reserved for use by
the language only, much as in other languages. Keep in mind that you cannot
use these words as names of variables or functions. Most of the keywords have
special meanings, and you’ll be using them to do various tasks in your Rust
programs; a few have no current functionality associated with them but have
been reserved for functionality that might be added to Rust in the future. You
can find a list of the keywords in Appendix A.

## Identifiers

We're going to be explaining a bunch of concepts in this book: variables,
functions, structs, lots of things. All of these things need names. A name
in Rust is called an "identifier," and can be made up of any nonempty ASCII
string, with some restrictions:

Either:

* The first character is a letter.
* The remaining characters are alphanumeric or _.

or:

* The first character is _.
* The identifier is more than one character. _ alone is not an identifier.
* The remaining characters are alphanumeric or _.

### Raw identifiers

Sometimes, you may need to use a name that's a keyword for another purpose.
Maybe you need to call a function named *match* that is coming from a C
library, where 'match' is not a keyword. To do this, you can use a "raw identifier."
Raw identifiers start with `r#`:

```rust,ignore
let r#fn = "this variable is named 'fn' even though that's a keyword";

// call a function named 'match'
r#match();
```

You won't need raw identifiers often, but when you do, you *really* need them.