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

You can find the latest version about constants
[here](ch03-01-variables-and-mutability.html#constants),
and about statics
[here](ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable).


