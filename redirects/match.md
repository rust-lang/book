% Match

There is a new edition of the book and this is an old link.

> `match` allows us to compare a value against a series of patterns and then execute code based on which pattern matches.
> Patterns can be made up of literal values, variable names, wildcards, and many other things.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.14 — Match][1]

* [In the second edition: Ch 6.02 — The `match` Control Flow Operator][2]

* [In the second edition: Ch 18.00 — Patterns][3]


[1]: first-edition/match.html
[2]: second-edition/ch06-02-match.html
[3]: second-edition/ch18-00-patterns.html

