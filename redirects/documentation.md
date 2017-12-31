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

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 14.02 — Publishing to crates.io, section Making useful documentation][2]**
* <small>[In the first edition: Ch 4.4 — Documentation][1]</small>


[1]: first-edition/documentation.html
[2]: second-edition/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
