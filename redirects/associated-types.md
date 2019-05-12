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

You can find the latest version of this information
[here](ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types).