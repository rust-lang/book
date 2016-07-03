## Comments

All programmers strive to make their code easy to understand, but sometimes
some extra explanation is warranted. In these cases, we leave notes in our
source code that the compiler will ignore. These notes are called *comments*.

Here’s a simple comment:

```rust
// Hello, world.
```

In Rust, comments must start with two slashes, and will last until the end of
the line. For comments that extend beyond a single line, you'll need to include
`//` on each line, like this:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also be placed at the end of lines of code:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above, like so:

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.

### Documentation Comments

<!--- TR: I'm not clear on the advantages of one over the other, can you help
to clarify? Why would you need to use a normal comment, if this is better - are
there disadvantages? /Liz -->
<!-- Tried to add some clarification! /Carol -->

Rust has another kind of comment: a *documentation comment*. These comments
don’t affect the way that the code works, but they do work with Rust’s tools.
More specifically, the `rustdoc` tool can read documentation comments and
produce HTML documentation from them. This documentation's intended audience is
usually people who are using your code, so that they know how to interact with
it. Regular comments won't be shown in `rustdoc` generated HTML, so their
intended audience is people who are reading and editing your code.

Documentation comments use an extra slash, like this:

```rust
/// The foo function doesn’t really do much.
fn foo() {

}

/// Documentation comments can use
/// multiple line comments too,
/// like we did before.
fn bar() {

}
```

The `rustdoc` tool would interpret each comment in this example as documenting
the thing that follows it. The first comment would be used to document the
`foo()` function, and the second comment would document the `bar()` function.

Because documentation comments have semantic meaning to `rustdoc`, the compiler
will pay attention to the placement of your documentation comments. For
example, a program containing only this:

```rust,ignore
/// What am I documenting?
```

Will give the following compiler error:

```bash
src/main.rs:1:1: 1:27 error: expected item after doc comment
src/main.rs:1 /// What am I documenting?
              ^~~~~~~~~~~~~~~~~~~~~~~~~~
```

This happens because Rust expects a document comment to be associated with
whatever code comes directly after it, so it sees that a document comment alone
must be a mistake.

<!-- Steve-- since documenting free functions in a binary doesn't generate
useful rustdoc, but that's all that has been covered so far, can this section
end with a sentence that says something like "We'll cover generating
documentation for your libraries in chapter XX" or similar? Wherever you talk
about `cargo doc`? /Carol -->
