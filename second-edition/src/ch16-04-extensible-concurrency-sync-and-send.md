## Extensible Concurrency with the `Sync` and `Send` Traits

Interestingly, the Rust language itself knows *very* little about concurrency.
Almost everything we’ve talked about so far in this chapter has been part of
the standard library, not the language. Our concurrency options are not limited
to the language or the standard library, meaning we can write our own
concurrency options or use ones others have written.

There *are* two concurrency concepts embedded in the language, however: the
`std::marker` traits `Sync` and `Send`.

### Allowing Transference of Ownership Between Threads with `Send`

The `Send` marker trait indicates that ownership of the type implementing
`Send` may be transferred between threads. Almost every Rust type is `Send`,
but there are some exceptions, including `Rc<T>`: this cannot be `Send` because
if we cloned an `Rc<T>` value and tried to transfer ownership of the clone to
another thread, both threads might update the reference count at the same time.
For this reason, `Rc<T>` is implemented for use in single-threaded situations
where you don’t want to pay the threadsafe performance penalty.

In this way Rust’s type system and trait bounds ensure we can never
accidentally send an `Rc<T>` value across threads unsafely. When we tried to do
this in Listing 16-14, we got an error that said `the trait Send is not
implemented for Rc<Mutex<i32>>`. When we switched to `Arc<T>`, which is `Send`,
the code compiled.

Any type composed entirely of `Send` types is automatically marked as `Send` as
well. Almost all primitive types are `Send`, aside from raw pointers, which
we’ll discuss in Chapter 19.

### Allowing Access from Multiple Threads with `Sync`

The `Sync` marker trait indicates that it is safe for the type implementing
`Sync` to be referenced from multiple threads. Another way to say this is that
any type `T` is `Sync` if `&T` (a reference to `T`) is `Send`, meaning the
reference can be sent safely to another thread. In a similar manner as `Send`,
primitive types are `Sync` and types composed entirely of types that are `Sync`
are also `Sync`.

`Rc<T>` is also not `Sync`, for the same reasons that it’s not `Send`.
`RefCell<T>` (which we talked about in Chapter 15) and the family of related
`Cell<T>` types are not `Sync`. The implementation of borrow checking that
`RefCell<T>` does at runtime is not threadsafe. `Mutex<T>` is `Sync`, and can
be used to share access with multiple threads as we saw in the previous section.

### Implementing `Send` and `Sync` Manually is Unsafe

Because types that are made up of `Send` and `Sync` traits are automatically
also `Send` and `Sync`, we don’t have to implement those traits ourselves. As
marker traits, they don’t even have any methods to implement. They’re just
useful for enforcing concurrency-related invariants.

Manually implementing these traits involves implementing unsafe Rust code.
We’re going to be talking about using unsafe Rust code in Chapter 19; for now,
the important information is that building new concurrent types not made up of
`Send` and `Sync` parts requires careful thought, in order to uphold the safety
guarantees. [The Nomicon] has more information about these guarantees and how
to uphold them.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/

## Summary

This isn’t the last we’ll see of concurrency in this book; the project in
Chapter 20 will use these concepts in a more realistic situation than the
smaller examples discussed here.

As we mentioned, since very little of how Rust deals with concurrency is part
of the language, many concurrency solutions are implemented as crates. These
evolve more quickly than the standard library; search online for the current
state-of-the-art crates to use in multithreaded situations.

Rust provides channels for message passing and smart pointer types like
`Mutex<T>` and `Arc<T>` that are safe to use in concurrent contexts. The type
system and the borrow checker will make sure the code using these solutions
won’t end up with data races or invalid references. Once we get our code
compiling, we can rest assured that it will happily run on multiple threads
without the kinds of hard-to-track-down bugs common in other languages.
Concurrent programming is no longer something to be afraid of: go forth and
make your programs concurrent, fearlessly!

Next, let’s talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger, and how Rust’s idioms relate to those you
might be familiar with from Object Oriented Programming.
