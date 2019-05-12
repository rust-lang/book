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

* **[in the current edition: Ch 13.02 — Iterators][2]**
* <small>[In the first edition: Ch 4.5 — Iterators][1]</small>


[1]: https://doc.rust-lang.org/1.30.0/book/first-edition/iterators.html
[2]: ch13-02-iterators.html
