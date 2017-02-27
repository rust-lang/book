# Smart Pointers

Now that we've learned quite a bit of Rust, we can start digging into some more
complicated concepts. In this chapter, we'll learn about a design pattern in
Rust called a *smart pointer*. This pattern allows us to leverage Rust's
ownership and borrowing features to manage all kinds of resources in a safe
way, often without much more syntax than using plain old references.

So what are smart pointers, anyway? Well, we've learned about references in
Rust in Chapter 4. *Pointer* is a generic programming term for something like a
reference, that is, pointers "point at" data somewhere else. References are a
kind of pointer that only borrow data; by contrast, in many cases, smart
pointers *own* the data that they point to. They also often hold metadata about
the data. Smart pointers have extra capabilities that references don't, hence
the "smart" nickname.

We've actually already encountered a few smart pointers in this book, we didn't
call them that by name, though. For example, in a certain sense, `String` and
`Vec<T>` from Chapter 8 are both smart pointers. They own some memory and allow
you to manipulate it, and have metadata (like their capacity) and extra
capabilities or guarantees (`String` data will always be valid UTF-8). Another
good example is `File`, which we used for our I/O project in Chapter 12: it
owns and manages a file handle that the operating system gives us, and allows
us to access the data in the file.

Given that this is a general design pattern in Rust, this chapter won't cover
every smart pointer that exists. Many libraries will build their own as well,
and you may write some for your own code. The ones we cover here will be the
most common ones from the standard library: `Box<T>`, `Rc<T>`, and
`RefCell<T>`. Along the way, we'll also cover:

* The `Deref` and `Drop` traits that make smart pointers convenient to work with
* The *interior mutability* pattern where an immutable type exposes an API for
  mutating an interior value, and the borrowing rules apply at runtime instead
  of compile time
* Reference cycles, how they can leak memory, and how to prevent them

Let's dive in!
