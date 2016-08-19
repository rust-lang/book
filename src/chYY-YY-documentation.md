## Documentation

<!-- Insert why documentation is important here, who your audience is for documentation -->

### Documentation comments

Rust has another kind of comment: a documentation comment. These
comments don’t affect the way that the code works, but they do work with Rust’s
tools. More specifically, the `rustdoc` tool that comes with Rust reads
documentation comments and produces HTML documentation from them.

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

This comment would then be interpreted by `rustdoc` as documenting the thing
that follows it: `foo()` and `bar()`.

Because documentation comments have semantic meaning to `rustdoc`, the compiler
will pay attention to the placement of your documentation comments. For
example, a program with only this:

```rust,ignore
/// What am I documenting?
```

Will give a compiler error:

```bash
src/main.rs:1:1: 1:27 error: expected item after doc comment
src/main.rs:1 /// What am I documenting?
              ^~~~~~~~~~~~~~~~~~~~~~~~~~
```

### Generating HTML documentation
