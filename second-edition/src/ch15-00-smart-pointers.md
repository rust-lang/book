# Smart Pointers

A *pointer* is a general concept for a variable that contains an address in
memory. This address refers to, or “points at”, some other data. The most
common kind of pointer in Rust is a *reference*, which we learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value that
they point to. They don’t have any special abilities other than referring to
data. They also don’t have any overhead, so they’re used the most often.

*Smart pointers*, on the other hand, are data structures that act like a
pointer, but they also have additional metadata and capabilities. The concept
of smart pointers isn’t unique to Rust; it originated in C++ and exists in
other languages as well. The different smart pointers defined in Rust’s
standard library provide extra functionality beyond what references provide.
One example that we’ll explore in this chapter is the *reference counting*
smart pointer type, which enables you to have multiple owners of data. The
reference counting smart pointer keeps track of how many owners there are, and
when there aren’t any remaining, the smart pointer takes care of cleaning up
the data.

In Rust, where we have the concept of ownership and borrowing, an additional
difference between references and smart pointers is that references are a kind
of pointer that only borrow data; by contrast, in many cases, smart pointers
*own* the data that they point to.

We’ve actually already encountered a few smart pointers in this book, such as
`String` and `Vec<T>` from Chapter 8, though we didn’t call them smart pointers
at the time. Both these types count as smart pointers because they own some
memory and allow you to manipulate it. They also have metadata (such as their
capacity) and extra capabilities or guarantees (such as `String` ensuring its
data will always be valid UTF-8).

Smart pointers are usually implemented using structs. The characteristics that
distinguish a smart pointer from an ordinary struct are that smart pointers
implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance
of the smart pointer struct to behave like a reference so that we can write
code that works with either references or smart pointers. The `Drop` trait
allows us to customize the code that gets run when an instance of the smart
pointer goes out of scope. In this chapter, we’ll be discussing both of those
traits and demonstrating why they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every smart pointer that exists.
Many libraries have their own smart pointers and you can even write some
yourself. We’ll just cover the most common smart pointers from the standard
library:

* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counted type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time

Along the way, we’ll cover the *interior mutability* pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
*reference cycles*, how they can leak memory, and how to prevent them.

Let’s dive in!
