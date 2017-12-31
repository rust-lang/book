% Associated Types

There is a new edition of the book and this is an old link.

> Associated types are a way of associating a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.30 — Associated Types][1]

* [In the second edition: Ch 19.03 — Advanced Traits][2]


[1]: first-edition/associated-types.html
[2]: second-edition/ch19-03-advanced-traits.html#associated-types
