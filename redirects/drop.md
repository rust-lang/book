% Drop

There is a new edition of the book and this is an old link.

> `Drop` lets us customize what happens when a value is about to go out of scope.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.20 — Drop][1]

* [In the second edition: Ch 15.03 — The `Drop` Trait Runs Code on Cleanup][2]


[1]: first-edition/drop.html
[2]: second-edition/ch15-03-drop.html
