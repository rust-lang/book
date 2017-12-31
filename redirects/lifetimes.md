% Lifetimes

<small>There is a new edition of the book and this is an old link.</small>

> Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
> Most of the time lifetimes are implicit and inferred.

```rust
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 10.03 — Lifetimes][2]**
* [In the second edition: Ch 19.02 — Advanced Lifetimes][3]
* <small>[In the first edition: Ch 3.10 — Lifetimes][1]</small>


[1]: first-edition/lifetimes.html
[2]: second-edition/ch10-03-lifetime-syntax.html
[3]: second-edition/ch19-02-advanced-lifetimes.html
