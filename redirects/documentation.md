% Documentation

<small>There is a new edition of the book and this is an old link.</small>

> Documentation comments use `///` instead of `//` and support Markdown notation for formatting the text if you’d like.
> You place documentation comments just before the item they are documenting. 

```rust,no_run
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

---

You can find the latest version of this information
[here](ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments).