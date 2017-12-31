% if let

<small>There is a new edition of the book and this is an old link.</small>

> The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern and ignore the rest.

```rust
let some_u8_value = Some(3u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 6.03 — Concise Control Flow with `if let`][2]**
* <small>[In the first edition: Ch 3.21 — if let][1]</small>


[1]: first-edition/if-let.html
[2]: second-edition/ch06-03-if-let.html
