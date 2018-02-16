% Raw Pointers

<small>There is a new edition of the book and this is an old link.</small>

> Raw pointers are allowed to ignore many of the rules that references have to follow.

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 19.01 — Unsafe Rust, section Dereferencing a Raw Pointer][2]**
* <small>[In the first edition: Ch 3.35 — Raw Pointers][1]</small>


[1]: first-edition/raw-pointers.html
[2]: second-edition/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer
