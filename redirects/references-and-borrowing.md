% References and Borrowing

<small>There is a new edition of the book and this is an old link.</small>

> A reference _refers_ to a value but does not own it.
> Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 4.02 — References and Borrowing][2]**
* <small>[In the first edition: Ch 3.9 — References and Borrowing][1]</small>


[1]: first-edition/references-and-borrowing.html
[2]: second-edition/ch04-02-references-and-borrowing.html
