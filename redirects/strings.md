% Strings

<small>There is a new edition of the book and this is an old link.</small>

> A `String` is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
> You can create a `String` from a string literal using the `from` function.
> A _string slice_ is a reference to part of a `String`.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

---

You can find the latest version of this information
[here](ch08-02-strings.html).