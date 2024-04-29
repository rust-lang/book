# Smart Pointers

A *pointer* is a general concept for a variable that contains an address in
memory. This address refers to, or “points at,” some other data. The most
common kind of pointer in Rust is a reference, which you learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value they
point to. They don’t have any special capabilities other than referring to
data, and have no overhead.

*Smart pointers*, on the other hand, are data structures that act like a
pointer but also have additional metadata and capabilities. The concept of
smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist
in other languages as well. Rust has a variety of smart pointers defined in the
standard library that provide functionality beyond that provided by references.
To explore the general concept, we’ll look at a couple of different examples of
smart pointers, including a *reference counting* smart pointer type. This
pointer enables you to allow data to have multiple owners by keeping track of
the number of owners and, when no owners remain, cleaning up the data.

Rust, with its concept of ownership and borrowing, has an additional difference
between references and smart pointers: while references only borrow data, in
many cases, smart pointers *own* the data they point to.

Though we didn’t call them as such at the time, we’ve already encountered a few
smart pointers in this book, including `String` and `Vec<T>` in Chapter 8. Both
these types count as smart pointers because they own some memory and allow you
to manipulate it. They also have metadata and extra capabilities or guarantees.
`String`, for example, stores its capacity as metadata and has the extra
ability to ensure its data will always be valid UTF-8.

Smart pointers are usually implemented using structs. Unlike an ordinary
struct, smart pointers implement the `Deref` and `Drop` traits. The `Deref`
trait allows an instance of the smart pointer struct to behave like a reference
so you can write your code to work with either references or smart pointers.
The `Drop` trait allows you to customize the code that’s run when an instance
of the smart pointer goes out of scope. In this chapter, we’ll discuss both
traits and demonstrate why they’re important to smart pointers.

Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We’ll
cover the most common smart pointers in the standard library:

* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time

In addition, we’ll cover the *interior mutability* pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
*reference cycles*: how they can leak memory and how to prevent them.

Let’s dive in!
