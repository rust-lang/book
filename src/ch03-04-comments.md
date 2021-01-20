## Comments

All programmers strive to make their code easy to understand, but sometimes
extra explanation is warranted. In these cases, programmers leave notes, or
*comments*, in their source code that the compiler will ignore but people
reading the source code may find useful.

Here’s a simple comment:

```rust
// hello, world
```

In Rust, the idiomatic comment style starts a comment with two slashes, and the
comment continues until the end of the line. For comments that extend beyond a
single line, you’ll need to include `//` on each line, like this:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Rust also supports the block-style comments that comment out a block of text delimited by `/* */`.
For this, you'll need to add `/*` before and `*/` after the text you want to comment out. 

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-35-block-comments/src/main.rs}}
```


Comments can also be placed at the end of lines containing code:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

But you’ll more often see them used in this format, with the comment on a
separate line above the code it’s annotating:

<span class="filename">Filename: src/main.rs</span>

```rust
{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust also has another kind of comment, documentation comments, which we’ll
discuss in the “Publishing a Crate to Crates.io” section of Chapter 14.

All comments in Rust are listed in [Table B-7](./appendix-02-operators.md) 
in the "Operators and Symbols" section of Chapter 21.