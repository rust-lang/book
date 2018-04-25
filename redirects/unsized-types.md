% Unsized Types

<small>There is a new edition of the book and this is an old link.</small>

> Sometimes referred to as ‘DSTs’ or ‘unsized types’, these types let us talk about types whose size we can only know at runtime.
> The `Sized` trait is automatically implemented for everything the compiler knows the size of at compile time.
> A trait bound on `?Sized` is the opposite of a trait bound on `Sized`; that is, we would read this as “`T` may or may not be `Sized`”.

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
    // ...snip...
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 19.04 — Advanced Types, section Dynamically Sized Types][2]**
* <small>[In the first edition: Ch 3.31 — Unsized Types][1]</small>


[1]: first-edition/unsized-types.html
[2]: second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-sized
