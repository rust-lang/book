% Closures

<small>There is a new edition of the book and this is an old link.</small>

> Anonymous functions you can save in a variable or pass as arguments to other functions.

```rust
# use std::thread;
# use std::time::Duration;

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
# expensive_closure(5);
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 13.01 — Closures][2]**
* <small>[In the first edition: Ch 3.23 — Closures][1]</small>


[1]: first-edition/closures.html
[2]: second-edition/ch13-01-closures.html
