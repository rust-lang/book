% Macros

<small>There is a new edition of the book and this is an old link.</small>

> While functions and types abstract over code, macros abstract at a syntactic level.

```rust
macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

fn main() {
    assert_eq!(25, five_times!(2 + 3));
}
```

---

Here are the relevant sections in the new and old books:

* **[In the current edition: Ch 19.06 Macros][2]**
* [Rust By Example: Macros][3]
* [In the Rust Reference: Ch 3.1 — Macros by Example][4]
* <small>[In the first edition: Ch 3.34 — Macros][1]</small>


[1]: https://doc.rust-lang.org/1.30.0/book/first-edition/macros.html
[2]: ch19-06-macros.html
[3]: https://rustbyexample.com/macros.html
[4]: ../reference/macros-by-example.html
