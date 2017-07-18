## Extensible Concurrency with the `Sync` and `Send` Traits

One interesting aspect of Rust’s concurrency model is that the language knows
*very* little about concurrency. Almost everything we’ve been talking about so
far has been part of the standard library, not the language itself. Because we
don’t need the language to provide everything we need to program in a
concurrent context, we’re not limited to the concurrency options that the
standard library or language provide: we can write our own or use ones others
have written.

We said *almost* everything wasn’t in the language, so what is? There are two
traits, both in `std::marker`: `Sync` and `Send`.

### `Send` for Indicating Ownership May Be Transferred to Another Thread

The `Send` marker trait indicates that ownership of that type may be
transferred between threads. Almost every Rust type is `Send`, but there are
some exceptions. One type provided by the standard library that is not `Send`
is `Rc<T>`: if we clone an `Rc<T>` value and try to transfer ownership of the
clone to another thread, both threads might update the reference count at the
same time. As we mentioned in the previous section, `Rc<T>` is implemented for
use in single-threaded situations where you don’t want to pay the performance
penalty of having a threadsafe reference count.

Because `Rc<T>` is not marked `Send`, Rust’s type system and trait bounds
ensure that we can never forget and accidentally send an `Rc<T>` value across
threads unsafely. We tried to do this in Listing 16-14, and we got an error
that said `the trait Send is not implemented for Rc<Mutex<i32>>`. When we
switched to `Arc<T>`, which is `Send`, the code compiled.

Any type that is composed entirely of `Send` types is automatically marked as
`Send` as well. Almost all primitive types are `Send`, aside from raw pointers,
which we’ll discuss in Chapter 19. Most standard library types are `Send`,
aside from `Rc<T>`.

### `Sync` for Indicating Access from Multiple Threads is Safe

The `Sync` marker trait indicates that a type is safe to have references to a
value from multiple threads. Another way to say this is for any type `T`, `T`
is `Sync` if `&T` (a reference to `T`) is `Send` so that the reference can be
sent safely to another thread. In a similar manner as `Send`, primitive types
are `Sync` and types composed entirely of types that are `Sync` are also `Sync`.

`Rc<T>` is also not `Sync`, for the same reasons that it’s not `Send`.
`RefCell<T>` (which we talked about in Chapter 15) and the family of related
`Cell<T>` types are not `Sync`. The implementation of the borrow checking at
runtime that `RefCell<T>` does is not threadsafe. `Mutex<T>` is `Sync`, and can
be used to share access with multiple threads as we saw in the previous section.

### Implementing `Send` and `Sync` Manually is Unsafe

Usually, we don’t need to implement the `Send` and `Sync` traits, since types
that are made up of `Send` and `Sync` traits are automatically also `Send` and
`Sync`. Because they’re marker traits, they don’t even have any methods to
implement. They’re just useful for enforcing concurrency-related invariants.

Implementing the guarantees that these traits are markers for involves
implementing unsafe Rust code. We’re going to be talking about using unsafe
Rust code in Chapter 19; for now, the important information is that building
new concurrent types that aren’t made up of `Send` and `Sync` parts requires
careful thought to make sure the safety guarantees are upheld. [The Nomicon]
has more information about these guarantees and how to uphold them.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/

## Summary

This isn’t the last time we’ll see concurrency in this book; the project in
Chapter 20 will use these concepts in a more realistic situation than the
smaller examples we discussed in this chapter.

As we mentioned, since very little of how Rust deals with concurrency has to be
part of the language, there are many concurrency solutions implemented as
crates. These evolve more quickly than the standard library; search online for
the current state-of-the-art crates for use in multithreaded situations.

Rust provides channels for message passing and smart pointer types like
`Mutex<T>` and `Arc<T>` that are safe to use in concurrent contexts. The type
system and the borrow checker will make sure the code we write using these
solutions won’t have data races or invalid references. Once we get our code
compiling, we can rest assured that our code will happily run on multiple
threads without the kinds of hard-to-track-down bugs common in other
programming languages. Concurrent programming is no longer something to be
afraid of: go forth and make your programs concurrent, fearlessly!

Next, let’s talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger, and how Rust’s idioms relate to those you
might be familiar with from Object Oriented Programming.
