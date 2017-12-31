% Strings

There is a new edition of the book and this is an old link.

> A `String` is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
> You can create a `String` from a string literal using the `from` function.
> A _string slice_ is a reference to part of a `String`.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.17 — Strings][1]

* [In second edition: Ch 8.02 — Strings][2]

* [In second edition: Ch 4.01 — Ownership, section The String Type][3]

* [In second edition: Ch 4.03 — Slices, section String Slices][4]


[1]: first-edition/strings.html
[2]: second-edition/ch08-02-strings.html
[3]: second-edition/ch04-01-what-is-ownership.html#the-string-type
[4]: second-edition/ch04-03-slices.html#string-slices
