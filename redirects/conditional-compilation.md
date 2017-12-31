% Conditional Compilation

There is a new edition of the book and this is an old link.

> Sometimes one wants to have different compiler outputs from the same code, depending on build target, such as targeted operating system, or to enable release builds.
> Configuration options are either provided by the compiler or passed in on the command line using.
> Rust code then checks for their presence using the `#[cfg(...)]` attribute

```rust
// The function is only included in the build when compiling for macOS
#[cfg(target_os = "macos")]
fn macos_only() {
  // ...
}
```

---

You can [continue to the exact older page][1].

This particular chapter does not exist in the second edition.
The best place to learn about it is [the Rust Reference][2].

* [In the first edition: Ch 4.3 — Conditional Compilation][1]

* [In the Rust Reference: Ch 5.3 — Attributes, Conditional Compilation section][2]


[1]: first-edition/conditional-compilation.html
[2]: ../reference/attributes.html#conditional-compilation
