% Borrow and AsRef

<small>There is a new edition of the book and this is an old link.</small>

> A cheap reference-to-reference conversion.
> Used to convert a value to a reference value within generic code.

```rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}
```

---

This chapter does not exist in [the second edition][2].
The best place to learn more about this is [the Rust documentation][3].

* **[In the Rust documentation: `convert::AsRef`][3]**
* <small>[In the first edition: Ch 4.10 — Borrow and AsRef][1]</small>


[1]: first-edition/borrow-and-asref.html
[2]: second-edition/index.html
[3]: ../std/convert/trait.AsRef.html
