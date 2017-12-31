% Mutability

<small>There is a new edition of the book and this is an old link.</small>

> Variables are immutable only by default; we can make them mutable by adding mut in front of the variable name.

```rust
let mut x = 5;
println!("The value of x is: {}", x);
x = 6;
println!("The value of x is: {}", x);
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 3.01 — Variables and Mutability][2]**
* <small>[In the first edition: Ch 3.11 — Mutability][1]</small>


[1]: first-edition/mutability.html
[2]: second-edition/ch03-01-variables-and-mutability.html
