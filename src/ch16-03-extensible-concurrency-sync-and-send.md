# Extensible Concurrency with `Sync` and `Send`

One interesting aspect of Rust's concurrency model is that the language knows
_very_ little about concurrency. Almost everything we've been talking about so
far has been part of the standard library, not the language itself:
`std::thread`, `std::sync`, etc. Due to this strategy, we're not limited to the
concurrency options that the standard library or language provide: we can write
our own, or use ones others have written.

We said *almost* everything wasn't in the language, so what is? The answer lies
in the last error message from the previous section:

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>: std::marker::Sync` is not satisfied
<snip>
   = note: required because of the requirements on the impl of `std::marker::Send` for `&std::rc::Rc<std::sync::Mutex<i32>>`
```

That is, there's two traits here, both in `std::marker`: `Sync`, and `Send`.
These two traits define two important properties when it comes to concurrency.
All of the traits in `std::marker` are called "marker traits", and that's
because they don't have any methods. They simply mark a type has having a given
property. As an example:

```rust
// Create our own marker trait as an example:
trait Marker {}

// We implement it for types where it makes sense:
impl Marker for i32 {}

// We can then use it as a bound to a function to only accept types that have
// that property:
fn takes_marker<T: Marker>(t: T) {
    // ...
}
```

It's the same for `Send` and `Sync`: they don't add any methods, they only
indicate concurrency properties.

## `Send`

The `Send` marker trait indicates that a type is allowed to have ownership
transferred between threads. Almost every Rust type is `Send`, but there are
still sometimes exceptions. For example, OpenGL, the graphics library, is not
threadsafe in many circumstances. The Rust wrappers for it need to take this
into account. But in pure Rust code, virtually everything is `Send`, so we
won't spend more time on it.

## `Sync`

`Sync`, on the other hand, is more interesting. If your type is `Sync`, it's
safe to access from multiple threads. This is why, when we tried to share our
`Rc<T>` across threads, we got an error: `Rc<T>` is not `Sync`. `Arc<T>`,
however, is, as long as its `T` is also `Sync`. `Mutex<T>` is `Sync`, and so
`Arc<Mutex<T>>` is `Sync`. Non-threadsafe types with interior mutability, like
`RefCell<T>` are not `Sync`, and so `Arc<RefCell<T>>`, for example, would not
be `Sync`.

## Building your own abstractions

Should we include crossbeam here?
