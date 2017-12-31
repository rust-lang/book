% Patterns

<small>There is a new edition of the book and this is an old link.</small>

> Patterns are a special syntax within Rust for matching against the structure of our types, complex or simple.
> A pattern is made up of some combination of literals; destructured arrays, enums, structs, or tuples; variables, wildcards, and placeholders.
> These pieces describe the “shape” of the data we’re working with.

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 18.03 — Patterns][2]**
* [In the second edition: Ch 6.02 — Match][3]
* <small>[In the first edition: Ch 3.15 — Patterns][1]</small>


[1]: first-edition/patterns.html
[2]: second-edition/ch18-00-patterns.html
[3]: second-edition/ch06-02-match.html#patterns-that-bind-to-values
