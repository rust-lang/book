% References and Borrowing

There is a new edition of the book and this is an old link.

> A reference _refers_ to a value but does not own it.
> Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.9 — References and Borrowing][1]

* [In the second edition: Ch 4.02 — References and Borrowing][2]


[1]: first-edition/references-and-borrowing.html
[2]: second-edition/ch04-02-references-and-borrowing.html
