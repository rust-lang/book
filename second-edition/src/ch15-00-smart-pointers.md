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

<!-- maybe a brief explanation what deref and drop? I'm not really sure what
reference counting is here too, can you outline that in brief?-->
<!-- We've added a quick explanation of reference counting here and a brief
explanation of deref and drop below. /Carol -->

<!--(regarding C++) if this is relevant here, can you expand? Are we saying
they will be familiar to C++ people? -->
<!-- We were trying to say that "smart pointer" isn't something particular to
Rust; we've tried to clarify. /Carol -->

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

<!-- Above: we said smart pointers don't own values earlier but in the
paragraph above we're saying String and Vec own memory, is that a
contradiction? -->
<!-- Our original text read: "In Rust, an additional difference between plain
references and smart pointers is that references are a kind of pointer that
only borrow data; by contrast, in many cases, smart pointers *own* the data
that they point to." You had edited this to say the opposite: "In Rust, smart
pointers can only borrow data, whereas in many other languages, smart pointers
*own* the data they point to." We had the "in rust" phrase not to distinguish
Rust's smart pointer implementation from other languages' smart pointer
implementations, but to acknowledge that the concept of borrowing and ownership
doesn't apply in many languages. The distinction between references borrowing
and smart pointers owning is important in the context of Rust. We've tried to
clarify the sentence talking about C++ and separate it from the discussion of
borrowing vs owning. So there shouldn't be a contradiction, and it should be
clearer that smart pointers usually own the data they point to. /Carol -->

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

<!-- Would it make sense to hyphenate reference-counted (and its derivations)
here? I think that would be more clear, but I don't want to do that if that's
not the Rust convention -->
<!-- The hyphenated version doesn't appear to be a general convention to me, it
looks like "reference counted" is most often not hyphenated. For example:
http://researcher.watson.ibm.com/researcher/files/us-bacon/Bacon01Concurrent.pdf
 We'd be interested to know if there's a standard that we don't know about
/Carol -->

* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counted type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time

<!-- Should we add Ref and RefMut to this list, too? -->
<!-- They were already sort of in the list; we've flipped the order to make it
clearer /Carol-->

Along the way, we’ll cover the *interior mutability* pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
*reference cycles*, how they can leak memory, and how to prevent them.

Let’s dive in!
