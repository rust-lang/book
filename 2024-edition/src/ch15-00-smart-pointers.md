# Smart Pointers

A _pointer_ is a general concept for a variable that contains an address in
memory. This address refers to, or “points at,” some other data. The most
common kind of pointer in Rust is a reference, which you learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value they
point to. They don’t have any special capabilities other than referring to
data, and they have no overhead.

_Smart pointers_, on the other hand, are data structures that act like a
pointer but also have additional metadata and capabilities. The concept of
smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist
in other languages as well. Rust has a variety of smart pointers defined in the
standard library that provide functionality beyond that provided by references.
To explore the general concept, we’ll look at a couple of different examples of
smart pointers, including a _reference counting_ smart pointer type. This
pointer enables you to allow data to have multiple owners by keeping track of
the number of owners and, when no owners remain, cleaning up the data.

Rust, with its concept of ownership and borrowing, has an additional difference
between references and smart pointers: while references only borrow data, in
many cases smart pointers _own_ the data they point to.

Smart pointers are usually implemented using structs. Unlike an ordinary
struct, smart pointers implement the `Deref` and `Drop` traits. The `Deref`
trait allows an instance of the smart pointer struct to behave like a reference
so you can write your code to work with either references or smart pointers.
The `Drop` trait allows you to customize the code that’s run when an instance
of the smart pointer goes out of scope. In this chapter, we’ll discuss both of
these traits and demonstrate why they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We’ll
cover the most common smart pointers in the standard library:

- `Box<T>`, for allocating values on the heap
- `Rc<T>`, a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time

In addition, we’ll cover the _interior mutability_ pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
reference cycles: how they can leak memory and how to prevent them.

Let’s dive in!
