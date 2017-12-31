% Deref coercions

<small>There is a new edition of the book and this is an old link.</small>

> Implementing the `Deref` trait allows us to customize the behavior of the _dereference operator_ `*`.
> By implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, we can write code that operates on references and use that code with smart pointers too.

```rust
use std::ops::Deref;

# struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 15.02 — Treating Smart Pointers like Regular References with the `Deref` Trait][2]**
* <small>[In the first edition: Ch 3.33 — Deref coercions][1]</small>


[1]: first-edition/deref-coercions.html
[2]: second-edition/ch15-02-deref.html
