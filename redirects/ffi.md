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

You can find the latest version of this information
[here](ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code)