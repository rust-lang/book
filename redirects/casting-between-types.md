% Casting between types

<small>There is a new edition of the book and this is an old link.</small>

> A type cast expression is denoted with the binary operator `as`.
> Executing an `as` expression casts the value on the left-hand side to the type on the right-hand side.

```rust
# fn sum(values: &[f64]) -> f64 { 0.0 }
# fn len(values: &[f64]) -> i32 { 0 }

fn average(values: &[f64]) -> f64 {
    let sum: f64 = sum(values);
    let size: f64 = len(values) as f64;
    sum / size
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Appendix A — Keywords][2]**
* [In the Rust Reference: Type Cast Expressions][3]
* [In the Rust documentation: `mem::transmute`][4]
* <small>[In the first edition: Ch 3.29 — Casting between types][1]</small>


[1]: first-edition/casting-between-types.html
[2]: second-edition/appendix-01-keywords.html
[3]: ../reference/expressions/operator-expr.html#type-cast-expressions
[4]: ../std/mem/fn.transmute.html