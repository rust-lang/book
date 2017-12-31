% Unsized Types

There is a new edition of the book and this is an old link.

> Sometimes referred to as ‘DSTs’ or ‘unsized types’, these types let us talk about types whose size we can only know at runtime.
> The `Sized` trait is automatically implemented for everything the compiler knows the size of at compile time.
> A trait bound on `?Sized` is the opposite of a trait bound on `Sized`; that is, we would read this as “`T` may or may not be `Sized`”.

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
    // ...snip...
}
```

---

You can [continue to the exact older page][1].
If you're trying to learn Rust, checking out [the second edition][2] might be a better choice.

* [In the first edition: Ch 3.31 — Unsized Types][1]

* [In the second edition: Ch 19.04 — Advanced Types, section Dynamically Sized Types][2]


[1]: first-edition/unsized-types.html
[2]: second-edition/ch19-04-advanced-types.html#dynamically-sized-types--sized
