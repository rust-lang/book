## Extensible Concurrency with the `Sync` and `Send` Traits

One interesting aspect of Rust's concurrency model is that the language knows
*very* little about concurrency. Almost everything we've been talking about so
far has been part of the standard library, not the language itself. Because we
don't need the language to provide everything we need to program in a
concurrent context, we're not limited to the concurrency options that the
standard library or language provide: we can write our own or use ones others
have written.

We said *almost* everything wasn't in the language, so what is? There are two
traits, both in `std::marker`: `Sync` and `Send`.

### `Send` for Indicating Ownership May Be Transferred to Another Thread

The `Send` marker trait indicates that ownership of that type may be
transferred between threads. Almost every Rust type is `Send`, but there are
some exceptions. One type provided by the standard library that is not `Send`
is `Rc<T>`: if we clone an `Rc<T>` value and try to transfer ownership of the
clone to another thread, both threads might update the reference count at the
same time. As we mentioned in the previous section, `Rc<T>` is implemented for
use in single-threaded situations where you don't want to pay the performance
penalty of having a threadsafe reference count.

Because `Rc<T>` is not marked `Send`, Rust's type system and trait bounds
ensure that we can never forget and accidentally send an `Rc<T>` value across
threads unsafely. We tried to do this in Listing 16-14, and we got an error
that said `the trait Send is not implemented for Rc<Mutex<i32>>`. When we
switched to `Arc<T>`, which is `Send`, the code compiled.

Any type that is composed entirely of `Send` types is automatically marked as
`Send` as well. Almost all primitive types are `Send`, aside from raw pointers,
which we'll discuss in Chapter 19. Most standard library types are `Send`,
aside from `Rc<T>`.

### `Sync` for Indicating Access from Multiple Threads is Safe

The `Sync` marker trait indicates that a type is safe to have references to a value from multiple threads. Another way to say this is for any type `T`, `T` is `Sync` if `&T` (a reference to `T`) is `Send` so that the reference can be sent safely to another thread. In a similar manner as `Send`, primitive types are `Sync` and types composed entirely of types that are `Sync` are also `Sync`.





This is why, when we tried to share our
`Rc<T>` across threads, we got an error: `Rc<T>` is not `Sync`. `Arc<T>`,
however, is, as long as its `T` is also `Sync`. `Mutex<T>` is `Sync`, and so
`Arc<Mutex<T>>` is `Sync`. Non-threadsafe types with interior mutability, like
`RefCell<T>` are not `Sync`, and so `Arc<RefCell<T>>`, for example, would not
be `Sync`.



These two traits define two important properties when it comes to concurrency.
All of the traits in `std::marker` are called "marker traits", and that's
because they don't have any methods. They simply mark a type has having a given
property.

It's the same for `Send` and `Sync`: they don't add any methods, they only
indicate concurrency properties.

## Summary

- project at the end of the book
- check out libraries
- go forth and be fearlessly concurrent
