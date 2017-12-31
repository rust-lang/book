% Method Syntax

<small>There is a new edition of the book and this is an old link.</small>

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

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 5.03 — Method Syntax][2]**
* <small>[In the first edition: Ch 3.16 — Method Syntax][1]</small>


[1]: first-edition/method-syntax.html
[2]: second-edition/ch05-03-method-syntax.html
