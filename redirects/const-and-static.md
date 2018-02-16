% `const` and `static`

<small>There is a new edition of the book and this is an old link.</small>

> Constants are _always_ immutable, and may only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
>
> Global variables are called `static` in Rust.

```rust
const MAX_POINTS: u32 = 100_000;
static HELLO_WORLD: &str = "Hello, world!";
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 3.01 — Variables and Mutability, section Constants][2]**
* **[In the second edition: Ch 19.01 — Unsafe Rust, section Static Variables][3]**
* <small>[In the first edition: Ch 3.26 — `const` and `static`][1]</small>


[1]: first-edition/const-and-static.html
[2]: second-edition/ch03-01-variables-and-mutability.html#differences-between-variables-and-constants
[3]: second-edition/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable
