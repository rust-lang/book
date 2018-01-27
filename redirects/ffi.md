% FFI

<small>There is a new edition of the book and this is an old link.</small>

> Sometimes, your Rust code may need to interact with code written in another language.
> To do this, Rust has a keyword, `extern`, that facilitates creating and using a _Foreign Function Interface_ (FFI).

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 19.01 — Unsafe Rust, section `extern` functions][2]**
* <small>[In the first edition: Ch 4.9 — FFI][1]</small>


[1]: first-edition/ffi.html
[2]: second-edition/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
