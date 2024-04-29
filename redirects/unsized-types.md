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

You can find the latest version of this information
[here](ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait).