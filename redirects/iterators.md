% Iterators

<small>There is a new edition of the book and this is an old link.</small>

> The iterator pattern allows you to perform some task on a sequence of items in turn.
> An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 13.02 — Iterators][2]**
* <small>[In the first edition: Ch 4.5 — Iterators][1]</small>


[1]: first-edition/iterators.html
[2]: second-edition/ch13-02-iterators.html
