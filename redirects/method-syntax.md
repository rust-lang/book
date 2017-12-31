% Method Syntax

There is a new edition of the book and this is an old link.

> Methods are different from functions in that they’re defined within the context of a struct, and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

```rust
# struct Rectangle {
#     width: u32,
#     height: u32,
# }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.16 — Method Syntax][1]

* [In the second edition: Ch 5.03 — Method Syntax][2]


[1]: first-edition/method-syntax.html
[2]: second-edition/ch05-03-method-syntax.html
