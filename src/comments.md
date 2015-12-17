# Comments

We strive to make our programs easy to understand, but sometimes, some extra explanation is warranted.
We can leave notes in our source code that the compiler will ignore.
These notes are called ‘comments’.

Here’s a comment:

```rust
// Hello, world.
```

Comments start with two slashes, and last until the end of the line.
Larger comments will need more lines:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also go at the end of lines:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above:

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.

## Documentation comments

However, Rust has another kind of comment: a documentation comment.
These comments don’t affect the way that the code works, but they do work with Rust’s tools.
More specifically, the `rustdoc` tool that comes with Rust reads documentation comments and produces HTML documentation from them.

Documentation comments use an extra slash:

```rust
/// The foo function doesn’t really do much.
fn foo() {
}

/// We also can use
/// multiple comments here too,
/// like we did before
fn bar() {
}
```

This comment would then be interpreted by `rustdoc` as documenting the thing that follows it: `foo()` and `bar()`.

Because documentation comments have semantic meaning to `rustdoc`, the compiler will pay attention to the placement
of your documentation comments.
For example, a program with only this:

```rust,ignore
/// What am I documenting?
```

Will give a compiler error:

```text
src/main.rs:1:1: 1:27 error: expected item after doc comment
src/main.rs:1 /// What am I documenting?
              ^~~~~~~~~~~~~~~~~~~~~~~~~~
```

### Inner documentation comments

There is a secondary form of a documentation comment, an ‘inner’ documentation comment.
Inner documentation comments look like this: `//!`.
They are only really used to document modules, however, and so we will talk about them in the modules section.
