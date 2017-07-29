# Smart Pointers

*Pointer* is a generic programming term for something that refers to a location
that stores some other data. We learned about Rust’s references in Chapter 4;
they’re a plain sort of pointer indicated by the `&` symbol and borrow the
value that they point to. *Smart pointers* are data structures that act like a
pointer, but also have additional metadata and capabilities, such as reference
counting. The smart pointer pattern originated in C++. In Rust, an additional
difference between plain references and smart pointers is that references are a
kind of pointer that only borrow data; by contrast, in many cases, smart
pointers *own* the data that they point to.

We’ve actually already encountered a few smart pointers in this book, even
though we didn’t call them that by name at the time. For example, in a certain
sense, `String` and `Vec<T>` from Chapter 8 are both smart pointers. They own
some memory and allow you to manipulate it, and have metadata (like their
capacity) and extra capabilities or guarantees (`String` data will always be
valid UTF-8). The characteristics that distinguish a smart pointer from an
ordinary struct are that smart pointers implement the `Deref` and `Drop`
traits, and in this chapter we’ll be discussing both of those traits and why
they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every smart pointer that exists.
Many libraries have their own and you may write some yourself. The ones we
cover here are the most common ones from the standard library:

* `Box<T>`, for allocating values on the heap
* `Rc<T>`, a reference counted type so data can have multiple owners
* `RefCell<T>`, which isn’t a smart pointer itself, but manages access to the
  smart pointers `Ref` and `RefMut` to enforce the borrowing rules at runtime
  instead of compile time

Along the way, we’ll also cover:

* The *interior mutability* pattern where an immutable type exposes an API for
  mutating an interior value, and the borrowing rules apply at runtime instead
  of compile time
* Reference cycles, how they can leak memory, and how to prevent them

Let’s dive in!
