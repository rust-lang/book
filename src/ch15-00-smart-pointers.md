# Smart Pointers

A *pointer* is a general concept for a variable that contains an address in
memory. This address refers to, or “points at,” some other data. The most
common kind of pointer in Rust is a reference, which you learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value they
point to. They don’t have any special capabilities other than referring to
data. Also, they don’t have any overhead and are the kind of pointer we use
most often.

*Smart pointers*, on the other hand, are data structures that not only act like
a pointer but also have additional metadata and capabilities. The concept of
smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist
in other languages as well. In Rust, the different smart pointers defined in
the standard library provide functionality beyond that provided by references.
One example that we’ll explore in this chapter is the *reference counting*
smart pointer type. This pointer enables you to have multiple owners of data by
keeping track of the number of owners and, when no owners remain, cleaning up
the data.

In Rust, which uses the concept of ownership and borrowing, an additional
difference between references and smart pointers is that references are
pointers that only borrow data; in contrast, in many cases, smart pointers
*own* the data they point to.

We’ve already encountered a few smart pointers in this book, such as `String`
and `Vec<T>` in Chapter 8, although we didn’t call them smart pointers at the
time. Both these types count as smart pointers because they own some memory and
allow you to manipulate it. They also have metadata (such as their capacity)
and extra capabilities or guarantees (such as with `String` ensuring its data
will always be valid UTF-8).

Smart pointers are usually implemented using structs. The characteristic that
distinguishes a smart pointer from an ordinary struct is that smart pointers
implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance
of the smart pointer struct to behave like a reference so you can write code
that works with either references or smart pointers. The `Drop` trait allows
you to customize the code that is run when an instance of the smart pointer
goes out of scope. In this chapter, we’ll discuss both traits and demonstrate
why they’re important to smart pointers.

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
