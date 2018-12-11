% Match

<small>There is a new edition of the book and this is an old link.</small>

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

Here are the relevant sections in the new and old books:

* **[in the current edition: Ch 6.02 — The `match` Control Flow Operator][2]**
* [in the current edition: Ch 18.00 — Patterns][3]
* <small>[In the first edition: Ch 3.14 — Match][1]</small>


[1]: https://doc.rust-lang.org/1.30.0/book/first-edition/match.html
[2]: ch06-02-match.html
[3]: ch18-00-patterns.html

