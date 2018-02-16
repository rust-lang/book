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

Here are the relevant sections in the new and old books:

* **[In second edition: Ch 8.02 — Strings][2]**
* [In second edition: Ch 4.01 — Ownership, section The String Type][3]
* [In second edition: Ch 4.03 — Slices, section String Slices][4]
* <small>[In the first edition: Ch 3.17 — Strings][1]</small>


[1]: first-edition/strings.html
[2]: second-edition/ch08-02-strings.html
[3]: second-edition/ch04-01-what-is-ownership.html#the-string-type
[4]: second-edition/ch04-03-slices.html#string-slices
