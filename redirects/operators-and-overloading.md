% Operators and Overloading

<small>There is a new edition of the book and this is an old link.</small>

> Rust does not allow you to create your own operators or overload arbitrary operators, but the operations and corresponding traits listed in `std::ops` can be overloaded by implementing the traits associated with the operator.

```rust
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 19.03 — Advanced Traits, section Operator Overloading][2]**
* [In the Rust documentation: `std::ops`][3]
* <small>[In the first edition: Ch 3.32 — Operators and Overloading][1]</small>

[1]: first-edition/operators-and-overloading.html
[2]: second-edition/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading
[3]: ../std/ops/index.html
