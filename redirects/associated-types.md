% Associated Types

<small>There is a new edition of the book and this is an old link.</small>

> Associated types are a way of associating a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 19.03 — Advanced Traits][2]**
* <small>[In the first edition: Ch 3.30 — Associated Types][1]</small>


[1]: first-edition/associated-types.html
[2]: second-edition/ch19-03-advanced-traits.html#associated-types-specify-placeholder-types-in-trait-definitions
