% Generics

<small>There is a new edition of the book and this is an old link.</small>

> Generics are abstract stand-ins for concrete types or other properties.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 10.00 — Generic Types, Traits, and Lifetimes][2]**
* <small>[In the first edition: Ch 3.18 — Generics][1]</small>


[1]: first-edition/generics.html
[2]: second-edition/ch10-00-generics.html

