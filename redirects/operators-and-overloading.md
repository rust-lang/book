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

You can find the latest version of this information
[here](ch19-03-advanced-traits.html).
