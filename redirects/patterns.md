% Patterns

<small>There is a new edition of the book and this is an old link.</small>

> Patterns are a special syntax within Rust for matching against the structure of our types, complex or simple.
> A pattern is made up of some combination of literals; destructured arrays, enums, structs, or tuples; variables, wildcards, and placeholders.
> These pieces describe the “shape” of the data we’re working with.

```rust
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}
```

---

You can find the latest version of this information
[here](ch06-02-match.html).