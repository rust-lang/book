% Raw Pointers

<small>There is a new edition of the book and this is an old link.</small>

> Raw pointers are allowed to ignore many of the rules that references have to follow.

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

---

You can find the latest version of this information
[here](ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer)