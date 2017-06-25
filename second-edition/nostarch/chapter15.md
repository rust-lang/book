
[TOC]

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

## `Box<T>` Points to Data on the Heap and Has a Known Size

The most straightforward smart pointer is a *box*, whose type is written
`Box<T>`. Boxes allow you to put a single value on the heap (we talked about
the stack vs. the heap in Chapter 4). Listing 15-1 shows how to use a box to
store an `i32` on the heap:

Filename: src/main.rs

```
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

Listing 15-1: Storing an `i32` value on the heap using a box

This will print `b = 5`. In this case, we can access the data in the box in a
similar way as we would if this data was on the stack. Just like any value that
has ownership of data, when a box goes out of scope like `b` does at the end of
`main`, it will be deallocated. The deallocation happens for both the box
(stored on the stack) and the data it points to (stored on the heap).

Putting a single value on the heap isn’t very useful, so you won’t use boxes by
themselves in the way that Listing 15-1 does very often. A time when boxes are
useful is when you want to ensure that your type has a known size. For
example, consider Listing 15-2, which contains an enum definition for a *cons
list*, a type of data structure that comes from functional programming.

A cons list is a list where each item contains a value and the next item until
the end of the list, which is signified by a value called `Nil`. Note that we
aren’t introducing the idea of “nil” or “null” that we discussed in Chapter 6,
this is just a regular enum variant name we’re using because it’s the canonical
name to use when describing the cons list data structure. Cons lists aren’t
used very often in Rust, `Vec<T>` is a better choice most of the time, but
implementing this data structure is useful as an example.

Here’s our first try at defining a cons list as an enum; note that this won’t
compile quite yet:

Filename: src/main.rs

```
enum List {
    Cons(i32, List),
    Nil,
}
```

Listing 15-2: The first attempt of defining an enum to represent a cons list
data structure of `i32` values

We’re choosing to implement a cons list that only holds `i32` values, but we
could have chosen to implement it using generics as we discussed in Chapter 10
to define a cons list concept independent of the type of value stored in the
cons list.

Using a cons list to store the list `1, 2, 3` would look like this:

```
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

The first `Cons` value holds `1` and another `List` value. This `List`
value is another `Cons` value that holds `2` and another `List` value. This
is one more `Cons` value that holds `3` and a `List` value, which is finally
`Nil`, the non-recursive variant that signals the end of the list.

If we try to compile the above code, we get the error shown in Listing 15-3:

```
error[E0072]: recursive type `List` has infinite size
 -->
  |
1 |   enum List {
  |  _^ starting here...
2 | |     Cons(i32, List),
3 | |     Nil,
4 | | }
  | |_^ ...ending here: recursive type has infinite size
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

Listing 15-3: The error we get when attempting to define a recursive enum

The error says this type ‘has infinite size’. Why is that? It’s because we’ve
defined `List` to have a variant that is recursive: it holds another value of
itself. This means Rust can’t figure out how much space it needs in order to
store a `List` value. Let’s break this down a bit: first let’s look at how Rust
decides how much space it needs to store a value of a non-recursive type.
Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

When Rust needs to know how much space to allocate for a `Message` value, it
can go through each of the variants and see that `Message::Quit` does not need
any space, `Message::Move` needs enough space to store two `i32` values, and so
forth. Therefore, the most space a `Message` value will need is the space it
would take to store the largest of its variants.

Contrast this to what happens when the Rust compiler looks at a recursive type
like `List` in Listing 15-2. The compiler tries to figure out how much memory
is needed to store value of `List`, and starts by looking at the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, so `Cons` needs an amount of space equal to the size of an `i32` plus
the size of a `List`. To figure out how much memory a `List` needs, it looks at
its variants, starting with the `Cons` variant. The `Cons` variant holds a
value of type `i32` and a value of type `List`, and this continues infinitely,
as shown in Figure 15-4.

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

Figure 15-4: An infinite `List` consisting of infinite `Cons` variants

Rust can’t figure out how much space to allocate for recursively defined types,
so the compiler gives the error in Listing 15-3. The error did include this
helpful suggestion:

```
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
        make `List` representable
```

Because a `Box<T>` is a pointer, we always know how much space it needs: a
pointer takes up a `usize` amount of space. The value of the `usize` will be
the address of the heap data. The heap data can be any size, but the address to
the start of that heap data will always fit in a `usize`. So if we change our
definition from Listing 15-2 to look like the definition here in Listing 15-5,
and change `main` to use `Box::new` for the values inside the `Cons` variants
like so:

Filename: src/main.rs

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

Listing 15-5: Definition of `List` that uses `Box<T>` in order to have a
known size

The compiler will be able to figure out the size it needs to store a `List`
value. Rust will look at `List`, and again start by looking at the `Cons`
variant. The `Cons` variant will need the size of `i32` plus the space to store
a `usize`, since a box always has the size of a `usize`, no matter what it’s
pointing to. Then Rust looks at the `Nil` variant, which does not store a
value, so `Nil` doesn’t need any space. We’ve broken the infinite, recursive
chain by adding in a box. Figure 15-6 shows what the `Cons` variant looks like
now:

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

Figure 15-6: A `List` that is not infinitely sized since `Cons` holds a `Box`

This is the main area where boxes are useful: breaking up an infinite data
structure so that the compiler can know what size it is. We’ll look at another
case where Rust has data of unknown size in Chapter 17 when we discuss trait
objects.

Even though you won’t be using boxes very often, they are a good way to
understand the smart pointer pattern. Two of the aspects of `Box<T>` that are
commonly used with smart pointers are its implementations of the `Deref` trait
and the `Drop` trait. Let’s investigate how these traits work and how smart
pointers use them.

## The `Deref` Trait Allows Access to the Data Through a Reference

The first important smart pointer-related trait is `Deref`, which allows us to
override `*`, the dereference operator (as opposed to the multiplication
operator or the glob operator). Overriding `*` for smart pointers makes
accessing the data behind the smart pointer convenient, and we’ll talk about
what we mean by convenient when we get to deref coercions later in this section.

We briefly mentioned the dereference operator in Chapter 8, in the hash map
section titled “Update a Value Based on the Old Value”. We had a mutable
reference, and we wanted to change the value that the reference was pointing
to. In order to do that, first we had to dereference the reference. Here’s
another example using references to `i32` values:

```
let mut x = 5;
{
    let y = &mut x;

    *y += 1
}

assert_eq!(6, x);
```

We use `*y` to access the data that the mutable reference in `y` refers to,
rather than the mutable reference itself. We can then modify that data, in this
case by adding 1.

With references that aren’t smart pointers, there’s only one value that the
reference is pointing to, so the dereference operation is straightforward.
Smart pointers can also store metadata about the pointer or the data. When
dereferencing a smart pointer, we only want the data, not the metadata, since
dereferencing a regular reference only gives us data and not metadata. We want
to be able to use smart pointers in the same places that we can use regular
references. To enable that, we can override the behavior of the `*` operator by
implementing the `Deref` trait.

Listing 15-7 has an example of overriding `*` using `Deref` on a struct we’ve
defined to hold mp3 data and metadata. `Mp3` is, in a sense, a smart pointer:
it owns the `Vec<u8>` data containing the audio. In addition, it holds some
optional metadata, in this case the artist and title of the song in the audio
data. We want to be able to conveniently access the audio data, not the
metadata, so we implement the `Deref` trait to return the audio data.
Implementing the `Deref` trait requires implementing one method named `deref`
that borrows `self` and returns the inner data:

Filename: src/main.rs

```
use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn main() {
    let my_favorite_song = Mp3 {
        // we would read the actual audio data from an mp3 file
        audio: vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells Like Teen Spirit")),
    };

    assert_eq!(vec![1, 2, 3], *my_favorite_song);
}
```

Listing 15-7: An implementation of the `Deref` trait on a struct that holds mp3
file data and metadata

Most of this should look familiar: a struct, a trait implementation, and a
main function that creates an instance of the struct. There is one part we
haven’t explained thoroughly yet: similarly to Chapter 13 when we looked at the
Iterator trait with the `type Item`, the `type Target = T;` syntax is defining
an associated type, which is covered in more detail in Chapter 19. Don’t worry
about that part of the example too much; it is a slightly different way of
declaring a generic parameter.

In the `assert_eq!`, we’re verifying that `vec![1, 2, 3]` is the result we get
when dereferencing the `Mp3` instance with `*my_favorite_song`, which is what
happens since we implemented the `deref` method to return the audio data. If
we hadn’t implemented the `Deref` trait for `Mp3`, Rust wouldn’t compile the
code `*my_favorite_song`: we’d get an error saying type `Mp3` cannot be
dereferenced.

The reason this code works is that what the `*` operator is doing behind
the scenes when we call `*my_favorite_song` is:

```
*(my_favorite_song.deref())
```

This calls the `deref` method on `my_favorite_song`, which borrows
`my_favorite_song` and returns a reference to `my_favorite_song.audio`, since
that’s what we defined `deref` to do in Listing 15-5. `*` on references is
defined to just follow the reference and return the data, so the expansion of
`*` doesn’t recurse for the outer `*`. So we end up with data of type
`Vec<u8>`, which matches the `vec![1, 2, 3]` in the `assert_eq!` in Listing
15-5.

The reason that the return type of the `deref` method is still a reference and
why it’s necessary to dereference the result of the method is that if the
`deref` method returned just the value, using `*` would always take ownership.

### Implicit Deref Coercions with Functions and Methods

Rust tends to favor explicitness over implicitness, but one case where this
does not hold true is *deref coercions* of arguments to functions and methods.
A deref coercion will automatically convert a reference to a pointer or a smart
pointer into a reference to that pointer’s contents. A deref coercion happens
when a value is passed to a function or method, and only happens if it’s needed
to get the type of the value passed in to match the type of the parameter
defined in the signature. Deref coercion was added to Rust to make calling
functions and methods not need as many explicit references and dereferences
with `&` and `*`.

Using our `Mp3` struct from Listing 15-5, here’s the signature of a function to
compress mp3 audio data that takes a slice of `u8`:

```
fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    // the actual implementation would go here
}
```

If Rust didn’t have deref coercion, in order to call this function with the
audio data in `my_favorite_song`, we’d have to write:

```
compress_mp3(my_favorite_song.audio.as_slice())
```

That is, we’d have to explicitly say that we want the data in the `audio` field
of `my_favorite_song` and that we want a slice referring to the whole
`Vec<u8>`. If there were a lot of places where we’d want process the `audio`
data in a similar manner, `.audio.as_slice()` would be wordy and repetitive.

However, because of deref coercion and our implementation of the `Deref` trait
on `Mp3`, we can call this function with the data in `my_favorite_song` by
using this code:

```
let result = compress_mp3(&my_favorite_song);
```

Just an `&` and the instance, nice! We can treat our smart pointer as if it was
a regular reference. Deref coercion means that Rust can use its knowledge of
our `Deref` implementation, namely: Rust knows that `Mp3` implements the
`Deref` trait and returns `&Vec<u8>` from the `deref` method. Rust also knows
the standard library implements the `Deref` trait on `Vec<T>` to return `&[T]`
from the `deref` method (and we can find that out too by looking at the API
documentation for `Vec<T>`). So, at compile time, Rust will see that it can use
`Deref::deref` twice to turn `&Mp3` into `&Vec<u8>` and then into `&[T]` to
match the signature of `compress_mp3`. That means we get to do less typing!
Rust will analyze types through `Deref::deref` as many times as it needs to in
order to get a reference to match the parameter’s type, when the `Deref` trait
is defined for the types involved. The indirection is resolved at compile time,
so there is no run-time penalty for taking advantage of deref coercion.

There’s also a `DerefMut` trait for overriding `*` on `&mut T` for use in
assignment in the same fashion that we use `Deref` to override `*` on `&T`s.

Rust does deref coercion when it finds types and trait implementations in three
cases:

* From `&T` to `&U` when `T: Deref<Target=U>`.
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`.
* From `&mut T` to `&U` when `T: Deref<Target=U>`.

The first two are the same, except for mutability: if you have a `&T`, and
`T` implements `Deref` to some type `U`, you can get a `&U` transparently. Same
for mutable references. The last one is more tricky: if you have a mutable
reference, it will also coerce to an immutable one. The other case is _not_
possible though: immutable references will never coerce to mutable ones.

The reason that the `Deref` trait is important to the smart pointer pattern is
that smart pointers can then be treated like regular references and used in
places that expect regular references. We don’t have to redefine methods and
functions to take smart pointers explicitly, for example.

## The `Drop` Trait Runs Code on Cleanup

The other trait that’s important to the smart pointer pattern is the `Drop`
trait. `Drop` lets us run some code when a value is about to go out of scope.
Smart pointers perform important cleanup when being dropped, like deallocating
memory or decrementing a reference count. More generally, data types can manage
resources beyond memory, like files or network connections, and use `Drop` to
release those resources when our code is done with them. We’re discussing
`Drop` in the context of smart pointers, though, because the functionality of
the `Drop` trait is almost always used when implementing smart pointers.

In some other languages, we have to remember to call code to free the memory or
resource every time we finish using an instance of a smart pointer. If we
forget, the system our code is running on might get overloaded and crash. In
Rust, we can specify that some code should be run when a value goes out of
scope, and the compiler will insert this code automatically. That means we don’t
need to remember to put this code everywhere we’re done with an instance of
these types, but we still won’t leak resources!

The way we specify code should be run when a value goes out of scope is by
implementing the `Drop` trait. The `Drop` trait requires us to implement one
method named `drop` that takes a mutable reference to `self`.

Listing 15-8 shows a `CustomSmartPointer` struct that doesn’t actually do
anything, but we’re printing out `CustomSmartPointer created.` right after we
create an instance of the struct and `Dropping CustomSmartPointer!` when the
instance goes out of scope so that we can see when each piece of code gets run.
Instead of a `println!` statement, you’d fill in `drop` with whatever cleanup
code your smart pointer needs to run:

Filename: src/main.rs

```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    println!("Wait for it...");
}
```

Listing 15-8: A `CustomSmartPointer` struct that implements the `Drop` trait,
where we could put code that would clean up after the `CustomSmartPointer`.

The `Drop` trait is in the prelude, so we don’t need to import it. The `drop`
method implementation calls the `println!`; this is where you’d put the actual
code needed to close the socket. In `main`, we create a new instance of
`CustomSmartPointer` then print out `CustomSmartPointer created.` to be able to
see that our code got to that point at runtime. At the end of `main`, our
instance of `CustomSmartPointer` will go out of scope. Note that we didn’t call
the `drop` method explicitly.

When we run this program, we’ll see:

```
CustomSmartPointer created.
Wait for it...
Dropping CustomSmartPointer!
```

printed to the screen, which shows that Rust automatically called `drop` for us
when our instance went out of scope.

We can use the `std::mem::drop` function to drop a value earlier than when it
goes out of scope. This isn’t usually necessary; the whole point of the `Drop`
trait is that it’s taken care of automatically for us. We’ll see an example of
a case when we’ll need to drop a value earlier than when it goes out of scope
in Chapter 16 when we’re talking about concurrency. For now, let’s just see
that it’s possible, and `std::mem::drop` is in the prelude so we can just call
`drop` as shown in Listing 15-9:

Filename: src/main.rs

```
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("Wait for it...");
}
```

Listing 15-9: Calling `std::mem::drop` to explicitly drop a value before it
goes out of scope

Running this code will print the following, showing that the destructor code is
called since `Dropping CustomSmartPointer!` is printed between
`CustomSmartPointer created.` and `Wait for it...`:

```
CustomSmartPointer created.
Dropping CustomSmartPointer!
Wait for it...
```

Note that we aren’t allowed to call the `drop` method that we defined directly:
if we replaced `drop(c)` in Listing 15-9 with `c.drop()`, we’ll get a compiler
error that says `explicit destructor calls not allowed`. We’re not allowed to
call `Drop::drop` directly because when Rust inserts its call to `Drop::drop`
automatically when the value goes out of scope, then the value would get
dropped twice. Dropping a value twice could cause an error or corrupt memory,
so Rust doesn’t let us. Instead, we can use `std::mem::drop`, whose definition
is:

```
pub mod std {
    pub mod mem {
        pub fn drop<T>(x: T) { }
    }
}
```

This function is generic over any type `T`, so we can pass any value to it. The
function doesn’t actually have anything in its body, so it doesn’t use its
parameter. The reason this empty function is useful is that `drop` takes
ownership of its parameter, which means the value in `x` gets dropped at the
end of this function when `x` goes out of scope.

Code specified in a `Drop` trait implementation can be used for many reasons to
make cleanup convenient and safe: we could use it to create our own memory
allocator, for instance! By using the `Drop` trait and Rust’s ownership system,
we don’t have to remember to clean up after ourselves since Rust takes care of
it automatically. We’ll get compiler errors if we write code that would clean
up a value that’s still in use, since the ownership system that makes sure
references are always valid will also make sure that `drop` only gets called
one time when the value is no longer being used.

Now that we’ve gone over `Box<T>` and some of the characteristics of smart
pointers, let’s talk about a few other smart pointers defined in the standard
library that add different kinds of useful functionality.

## `Rc<T>`, the Reference Counted Smart Pointer

In the majority of cases, ownership is very clear: you know exactly which
variable owns a given value. However, this isn’t always the case; sometimes,
you may actually need multiple owners. For this, Rust has a type called
`Rc<T>`. Its name is an abbreviation for *reference counting*. Reference
counting means keeping track of the number of references to a value in order to
know if a value is still in use or not. If there are zero references to a
value, we know we can clean up the value without any references becoming
invalid.

To think about this in terms of a real-world scenario, it’s like a TV in a
family room. When one person comes in the room to watch TV, they turn it on.
Others can also come in the room and watch the TV. When the last person leaves
the room, they’ll turn the TV off since it’s no longer being used. If someone
turns off the TV while others are still watching it, though, the people
watching the TV would get mad!

`Rc<T>` is for use when we want to allocate some data on the heap for multiple
parts of our program to read, and we can’t determine at compile time which part
of our program using this data will finish using it last. If we knew which part
would finish last, we could make that part the owner of the data and the normal
ownership rules enforced at compile time would kick in.

Note that `Rc<T>` is only for use in single-threaded scenarios; the next
chapter on concurrency will cover how to do reference counting in
multithreaded programs. If you try to use `Rc<T>` with multiple threads,
you’ll get a compile-time error.

### Using `Rc<T>` to Share Data

Let’s return to our cons list example from Listing 15-5. In Listing 15-11, we’re
going to try to use `List` as we defined it using `Box<T>`. First we’ll create
one list instance that contains 5 and then 10. Next, we want to create two more
lists: one that starts with 3 and continues on to our first list containing 5
and 10, then another list that starts with 4 and *also* continues on to our
first list containing 5 and 10. In other words, we want two lists that both
share ownership of the third list, which conceptually will be something like
Figure 15-10:

<img alt="Two lists that share ownership of a third list" src="img/trpl15-03.svg" class="center" />

Figure 15-10: Two lists, `b` and `c`, sharing ownership of a third list, `a`

Trying to implement this using our definition of `List` with `Box<T>` won’t
work, as shown in Listing 15-11:

Filename: src/main.rs

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

Listing 15-11: Having two lists using `Box<T>` that try to share ownership of a
third list won’t work

If we compile this, we get this error:

```
error[E0382]: use of moved value: `a`
  --> src/main.rs:13:30
   |
12 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
13 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
   = note: move occurs because `a` has type `List`, which does not
   implement the `Copy` trait
```

The `Cons` variants own the data they hold, so when we create the `b` list it
moves `a` to be owned by `b`. Then when we try to use `a` again when creating
`c`, we’re not allowed to since `a` has been moved.

We could change the definition of `Cons` to hold references instead, but then
we’d have to specify lifetime parameters and we’d have to construct elements of
a list such that every element lives at least as long as the list itself.
Otherwise, the borrow checker won’t even let us compile the code.

Instead, we can change our definition of `List` to use `Rc<T>` instead of
`Box<T>` as shown here in Listing 15-12:

Filename: src/main.rs

```
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, a.clone());
    let c = Cons(4, a.clone());
}
```

Listing 15-12: A definition of `List` that uses `Rc<T>`

Note that we need to add a `use` statement for `Rc` because it’s not in the
prelude. In `main`, we create the list holding 5 and 10 and store it in a new
`Rc` in `a`. Then when we create `b` and `c`, we call the `clone` method on `a`.

### Cloning an `Rc<T>` Increases the Reference Count

We’ve seen the `clone` method previously, where we used it for making a
complete copy of some data. With `Rc<T>`, though, it doesn’t make a full copy.
`Rc<T>` holds a *reference count*, that is, a count of how many clones exist.
Let’s change `main` as shown in Listing 15-13 to have an inner scope around
where we create `c`, and to print out the results of the `Rc::strong_count`
associated function at various points. `Rc::strong_count` returns the reference
count of the `Rc` value we pass to it, and we’ll talk about why this function
is named `strong_count` in the section later in this chapter about preventing
reference cycles.

Filename: src/main.rs

```
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("rc = {}", Rc::strong_count(&a));
    let b = Cons(3, a.clone());
    println!("rc after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, a.clone());
        println!("rc after creating c = {}", Rc::strong_count(&a));
    }
    println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
}
```

Listing 15-13: Printing out the reference count

This will print out:

```
rc = 1
rc after creating b = 2
rc after creating c = 3
rc after c goes out of scope = 2
```

We’re able to see that `a` has an initial reference count of one. Then each
time we call `clone`, the count goes up by one. When `c` goes out of scope, the
count is decreased by one, which happens in the implementation of the `Drop`
trait for `Rc<T>`. What we can’t see in this example is that when `b` and then
`a` go out of scope at the end of `main`, the count of references to the list
containing 5 and 10 is then 0, and the list is dropped. This strategy lets us
have multiple owners, as the count will ensure that the value remains valid as
long as any of the owners still exist.

In the beginning of this section, we said that `Rc<T>` only allows you to share
data for multiple parts of your program to read through immutable references to
the `T` value the `Rc<T>` contains. If `Rc<T>` let us have a mutable reference,
we’d run into the problem that the borrowing rules disallow that we discussed
in Chapter 4: two mutable borrows to the same place can cause data races and
inconsistencies. But mutating data is very useful! In the next section, we’ll
discuss the interior mutability pattern and the `RefCell<T>` type that we can
use in conjunction with an `Rc<T>` to work with this restriction on
immutability.

## `RefCell<T>` and the Interior Mutability Pattern

*Interior mutability* is a design pattern in Rust for allowing you to mutate
data even though there are immutable references to that data, which would
normally be disallowed by the borrowing rules. The interior mutability pattern
involves using `unsafe` code inside a data structure to bend Rust’s usual rules
around mutation and borrowing. We haven’t yet covered unsafe code; we will in
Chapter 19. The interior mutability pattern is used when you can ensure that
the borrowing rules will be followed at runtime, even though the compiler can’t
ensure that. The `unsafe` code involved is then wrapped in a safe API, and the
outer type is still immutable.

Let’s explore this by looking at the `RefCell<T>` type that follows the
interior mutability pattern.

### `RefCell<T>` has Interior Mutability

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
that it holds. So, what makes `RefCell<T>` different than a type like `Box<T>`?
Let’s recall the borrowing rules we learned in Chapter 4:

1. At any given time, you can have *either* but not both of:
  * One mutable reference.
  * Any number of immutable references.
2. References must always be valid.

With references and `Box<T>`, the borrowing rules’ invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced *at runtime*.
With references, if you break these rules, you’ll get a compiler error. With
`RefCell<T>`, if you break these rules, you’ll get a `panic!`.

Static analysis, like the Rust compiler performs, is inherently conservative.
There are properties of code that are impossible to detect by analyzing the
code: the most famous is the Halting Problem, which is out of scope of this
book but an interesting topic to research if you’re interested.

Because some analysis is impossible, the Rust compiler does not try to even
guess if it can’t be sure, so it’s conservative and sometimes rejects correct
programs that would not actually violate Rust’s guarantees. Put another way, if
Rust accepts an incorrect program, people would not be able to trust in the
guarantees Rust makes. If Rust rejects a correct program, the programmer will
be inconvenienced, but nothing catastrophic can occur. `RefCell<T>` is useful
when you know that the borrowing rules are respected, but the compiler can’t
understand that that’s true.

Similarly to `Rc<T>`, `RefCell<T>` is only for use in single-threaded
scenarios. We’ll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in the next chapter on concurrency. For now, all you
need to know is that if you try to use `RefCell<T>` in a multithreaded
context, you’ll get a compile time error.

With references, we use the `&` and `&mut` syntax to create references and
mutable references, respectively. But with `RefCell<T>`, we use the `borrow`
and `borrow_mut` methods, which are part of the safe API that `RefCell<T>` has.
`borrow` returns the smart pointer type `Ref`, and `borrow_mut` returns the
smart pointer type `RefMut`. These two types implement `Deref` so that we can
treat them as if they’re regular references. `Ref` and `RefMut` track the
borrows dynamically, and their implementation of `Drop` releases the borrow
dynamically.

Listing 15-14 shows what it looks like to use `RefCell<T>` with functions that
borrow their parameters immutably and mutably. Note that the `data` variable is
declared as immutable with `let data` rather than `let mut data`, yet
`a_fn_that_mutably_borrows` is allowed to borrow the data mutably and make
changes to the data!

Filename: src/main.rs

```
use std::cell::RefCell;

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn demo(r: &RefCell<i32>) {
    a_fn_that_immutably_borrows(&r.borrow());
    a_fn_that_mutably_borrows(&mut r.borrow_mut());
    a_fn_that_immutably_borrows(&r.borrow());
}

fn main() {
    let data = RefCell::new(5);
    demo(&data);
}
```

Listing 15-14: Using `RefCell<T>`, `borrow`, and `borrow_mut`

This example prints:

```
a is 5
a is 6
```

In `main`, we’ve created a new `RefCell<T>` containing the value 5, and stored
in the variable `data`, declared without the `mut` keyword. We then call the
`demo` function with an immutable reference to `data`: as far as `main` is
concerned, `data` is immutable!

In the `demo` function, we get an immutable reference to the value inside the
`RefCell<T>` by calling the `borrow` method, and we call
`a_fn_that_immutably_borrows` with that immutable reference. More
interestingly, we can get a *mutable* reference to the value inside the
`RefCell<T>` with the `borrow_mut` method, and the function
`a_fn_that_mutably_borrows` is allowed to change the value. We can see that the
next time we call `a_fn_that_immutably_borrows` that prints out the value, it’s
6 instead of 5.

### Borrowing Rules are Checked at Runtime on `RefCell<T>`

Recall from Chapter 4 that because of the borrowing rules, this code using
regular references that tries to create two mutable borrows in the same scope
won’t compile:

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

We’ll get this compiler error:

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 -->
  |
5 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
6 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
7 | }
  | - first borrow ends here
```

In contrast, using `RefCell<T>` and calling `borrow_mut` twice in the same
scope *will* compile, but it’ll panic at runtime instead. This code:

```
use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello"));

    let r1 = s.borrow_mut();
    let r2 = s.borrow_mut();
}
```

compiles but panics with the following error when we `cargo run`:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.83 secs
     Running `target/debug/refcell`
thread 'main' panicked at 'already borrowed: BorrowMutError',
/stable-dist-rustc/build/src/libcore/result.rs:868
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This runtime `BorrowMutError` is similar to the compiler error: it says we’ve
already borrowed `s` mutably once, so we’re not allowed to borrow it again. We
aren’t getting around the borrowing rules, we’re just choosing to have Rust
enforce them at runtime instead of compile time. You could choose to use
`RefCell<T>` everywhere all the time, but in addition to having to type
`RefCell` a lot, you’d find out about possible problems later (possibly in
production rather than during development). Also, checking the borrowing rules
while your program is running has a performance penalty.

### Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

So why would we choose to make the tradeoffs that using `RefCell<T>` involves?
Well, remember when we said that `Rc<T>` only lets you have an immutable
reference to `T`? Given that `RefCell<T>` is immutable, but has interior
mutability, we can combine `Rc<T>` and `RefCell<T>` to get a type that’s both
reference counted and mutable. Listing 15-15 shows an example of how to do
that, again going back to our cons list from Listing 15-5. In this example,
instead of storing `i32` values in the cons list, we’ll be storing
`Rc<RefCell<i32>>` values. We want to store that type so that we can have an
owner of the value that’s not part of the list (the multiple owners
functionality that `Rc<T>` provides), and so we can mutate the inner `i32`
value (the interior mutability functionality that `RefCell<T>` provides):

Filename: src/main.rs

```
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Cons(value.clone(), Rc::new(Nil));
    let shared_list = Rc::new(a);

    let b = Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Cons(Rc::new(RefCell::new(10)), shared_list.clone());

    *value.borrow_mut() += 10;

    println!("shared_list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

Listing 15-15: Using `Rc<RefCell<i32>>` to create a `List` that we can mutate

We’re creating a value, which is an instance of `Rc<RefCell<i32>>`. We’re
storing it in a variable named `value` because we want to be able to access it
directly later. Then we create a `List` in `a` that has a `Cons` variant that
holds `value`, and `value` needs to be cloned since we want `value` to also
have ownership in addition to `a`. Then we wrap `a` in an `Rc<T>` so that we
can create lists `b` and `c` that start differently but both refer to `a`,
similarly to what we did in Listing 15-12.

Once we have the lists in `shared_list`, `b`, and `c` created, then we add 10
to the 5 in `value` by dereferencing the `Rc<T>` and calling `borrow_mut` on
the `RefCell`.

When we print out `shared_list`, `b`, and `c`, we can see that they all have
the modified value of 15:

```
shared_list after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

This is pretty neat! By using `RefCell<T>`, we can have an outwardly immutable
`List`, but we can use the methods on `RefCell<T>` that provide access to its
interior mutability to be able to modify our data when we need to. The runtime
checks of the borrowing rules that `RefCell<T>` does protect us from data
races, and we’ve decided that we want to trade a bit of speed for the
flexibility in our data structures.

`RefCell<T>` is not the only standard library type that provides interior
mutability. `Cell<T>` is similar but instead of giving references to the inner
value like `RefCell<T>` does, the value is copied in and out of the `Cell<T>`.
`Mutex<T>` offers interior mutability that is safe to use across threads, and
we’ll be discussing its use in the next chapter on concurrency. Check out the
standard library docs for more details on the differences between these types.

## Creating Reference Cycles and Leaking Memory is Safe

Rust makes a number of guarantees that we’ve talked about, for example that
we’ll never have a null value, and data races will be disallowed at compile
time. Rust’s memory safety guarantees make it more difficult to create memory
that never gets cleaned up, which is known as a *memory leak*. Rust does not
make memory leaks *impossible*, however, preventing memory leaks is *not* one
of Rust’s guarantees. In other words, memory leaks are memory safe.

By using `Rc<T>` and `RefCell<T>`, it is possible to create cycles of
references where items refer to each other in a cycle. This is bad because the
reference count of each item in the cycle will never reach 0, and the values
will never be dropped. Let’s take a look at how that might happen and how to
prevent it.

In Listing 15-16, we’re going to use another variation of the `List` definition
from Listing 15-5. We’re going back to storing an `i32` value as the first
element in the `Cons` variant. The second element in the `Cons` variant is now
`RefCell<Rc<List>>`: instead of being able to modify the `i32` value this time,
we want to be able to modify which `List` a `Cons` variant is pointing to.
We’ve also added a `tail` method to make it convenient for us to access the
second item, if we have a `Cons` variant:

Filename: src/main.rs

```
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

Listing 15-16: A cons list definition that holds a `RefCell` so that we can
modify what a `Cons` variant is referring to

Next, in Listing 15-17, we’re going to create a `List` value in the variable
`a` that initially is a list of `5, Nil`. Then we’ll create a `List` value in
the variable `b` that is a list of the value 10 and then points to the list in
`a`. Finally, we’ll modify `a` so that it points to `b` instead of `Nil`, which
will then create a cycle:

Filename: src/main.rs

```
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = b.clone();
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

Listing 15-17: Creating a reference cycle of two `List` values pointing to
each other

We use the `tail` method to get a reference to the `RefCell` in `a`, which we
put in the variable `link`. Then we use the `borrow_mut` method on the
`RefCell` to change the value inside from an `Rc` that holds a `Nil` value to
the `Rc` in `b`. We’ve created a reference cycle that looks like Figure 15-18:

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

Figure 15-18: A reference cycle of lists `a` and `b` pointing to each other

If you uncomment the last `println!`, Rust will try and print this cycle out
with `a` pointing to `b` pointing to `a` and so forth until it overflows the
stack.

Looking at the results of the `println!` calls before the last one, we’ll see
that the reference count of both `a` and `b` are 2 after we change `a` to point
to `b`. At the end of `main`, Rust will try and drop `b` first, which will
decrease the count of the `Rc` by one. However, because `a` is still
referencing that `Rc`, its count is 1 rather than 0, so the memory the `Rc` has
on the heap won’t be dropped. It’ll just sit there with a count of one,
forever. In this specific case, the program ends right away, so it’s not a
problem, but in a more complex program that allocates lots of memory in a cycle
and holds onto it for a long time, this would be a problem. The program would
be using more memory than it needs to be, and might overwhelm the system and
cause it to run out of memory available to use.

Now, as you can see, creating reference cycles is difficult and inconvenient in
Rust. But it’s not impossible: preventing memory leaks in the form of reference
cycles is not one of the guarantees Rust makes. If you have `RefCell<T>` values
that contain `Rc<T>` values or similar nested combinations of types with
interior mutability and reference counting, be aware that you’ll have to ensure
that you don’t create cycles. In the example in Listing 15-14, the solution
would probably be to not write code that could create cycles like this, since
we do want `Cons` variants to own the list they point to.

With data structures like graphs, it’s sometimes necessary to have references
that create cycles in order to have parent nodes point to their children and
children nodes point back in the opposite direction to their parents, for
example. If one of the directions is expressing ownership and the other isn’t,
one way of being able to model the relationship of the data without creating
reference cycles and memory leaks is using `Weak<T>`. Let’s explore that next!

### Prevent Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

The Rust standard library provides `Weak<T>`, a smart pointer type for use in
situations that have cycles of references but only one direction expresses
ownership. We’ve been showing how cloning an `Rc<T>` increases the
`strong_count` of references; `Weak<T>` is a way to reference an `Rc<T>` that
does not increment the `strong_count`: instead it increments the `weak_count`
of references to an `Rc`. When an `Rc` goes out of scope, the inner value will
get dropped if the `strong_count` is 0, even if the `weak_count` is not 0. To
be able to get the value from a `Weak<T>`, we first have to upgrade it to an
`Option<Rc<T>>` by using the `upgrade` method. The result of upgrading a
`Weak<T>` will be `Some` if the `Rc` value has not been dropped yet, and `None`
if the `Rc` value has been dropped. Because `upgrade` returns an `Option`, we
know Rust will make sure we handle both the `Some` case and the `None` case and
we won’t be trying to use an invalid pointer.

Instead of the list in Listing 15-17 where each item knows only about the
next item, let’s say we want a tree where the items know about their children
items *and* their parent items.

Let’s start just with a struct named `Node` that holds its own `i32` value as
well as references to its children `Node` values:

Filename: src/main.rs

```
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

We want to be able to have a `Node` own its children, and we also want to be
able to have variables own each node so we can access them directly. That’s why
the items in the `Vec` are `Rc<Node>` values. We want to be able to modify what
nodes are another node’s children, so that’s why we have a `RefCell` in
`children` around the `Vec`. In Listing 15-19, let’s create one instance of
`Node` named `leaf` with the value 3 and no children, and another instance
named `branch` with the value 5 and `leaf` as one of its children:

Filename: src/main.rs

```
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![leaf.clone()]),
    });
}
```

Listing 15-19: Creating a `leaf` node and a `branch` node where `branch` has
`leaf` as one of its children but `leaf` has no reference to `branch`

The `Node` in `leaf` now has two owners: `leaf` and `branch`, since we clone
the `Rc` in `leaf` and store that in `branch`. The `Node` in `branch` knows
it’s related to `leaf` since `branch` has a reference to `leaf` in
`branch.children`. However, `leaf` doesn’t know that it’s related to `branch`,
and we’d like `leaf` to know that `branch` is its parent.

To do that, we’re going to add a `parent` field to our `Node` struct
definition, but what should the type of `parent` be? We know it can’t contain
an `Rc<T>`, since `leaf.parent` would point to `branch` and `branch.children`
contains a pointer to `leaf`, which makes a reference cycle. Neither `leaf` nor
`branch` would get dropped since they would always refer to each other and
their reference counts would never be zero.

So instead of `Rc`, we’re going to make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`:

Filename: src/main.rs

```
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

This way, a node will be able to refer to its parent node if it has one,
but it does not own its parent. A parent node will be dropped even if
it has child nodes referring to it, as long as it doesn’t have a parent
node as well. Now let’s update `main` to look like Listing 15-20:

Filename: src/main.rs

```
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![leaf.clone()]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

Listing 15-20: A `leaf` node and a `branch` node where `leaf` has a `Weak`
reference to its parent, `branch`

Creating the `leaf` node looks similar; since it starts out without a parent,
we create a new `Weak` reference instance. When we try to get a reference to
the parent of `leaf` by using the `upgrade` method, we’ll get a `None` value,
as shown by the first `println!` that outputs:

```
leaf parent = None
```

Similarly, `branch` will also have a new `Weak` reference, since `branch` does
not have a parent node. We still make `leaf` be one of the children of
`branch`. Once we have a new `Node` instance in `branch`, we can modify `leaf`
to have a `Weak` reference to `branch` for its parent. We use the `borrow_mut`
method on the `RefCell` in the `parent` field of `leaf`, then we use the
`Rc::downgrade` function to create a `Weak` reference to `branch` from the `Rc`
in `branch.`

When we print out the parent of `leaf` again, this time we’ll get a `Some`
variant holding `branch`. Also notice we don’t get a cycle printed out that
eventually ends in a stack overflow like we did in Listing 15-14: the `Weak`
references are just printed as `(Weak)`:

```
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The fact that we don’t get infinite output (or at least until the stack
overflows) is one way we can see that we don’t have a reference cycle in this
case. Another way we can tell is by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`. In Listing 15-21, let’s create a new
inner scope and move the creation of `branch` in there, so that we can see what
happens when `branch` is created and then dropped when it goes out of scope:

Filename: src/main.rs

```
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![leaf.clone()]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

Listing 15-21: Creating `branch` in an inner scope and examining strong and
weak reference counts of `leaf` and `branch`

Right after creating `leaf`, its strong count is 1 (for `leaf` itself) and its
weak count is 0. In the inner scope, after we create `branch` and associate
`leaf` and `branch`, `branch` will have a strong count of 1 (for `branch`
itself) and a weak count of 1 (for `leaf.parent` pointing to `branch` with a
`Weak<T>`). `leaf` will have a strong count of 2, since `branch` now has a
clone the `Rc` of `leaf` stored in `branch.children`. `leaf` still has a weak
count of 0.

When the inner scope ends, `branch` goes out of scope, and its strong count
decreases to 0, so its `Node` gets dropped. The weak count of 1 from
`leaf.parent` has no bearing on whether `Node` gets dropped or not, so we don’t
have a memory leak!

If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again like we did before `leaf` had a parent. At the end of the program,
`leaf` has a strong count of 1 and a weak count of 0, since `leaf` is now the
only thing pointing to it again.

All of the logic managing the counts and whether a value should be dropped or
not was managed by `Rc` and `Weak` and their implementations of the `Drop`
trait. By specifying that the relationship from a child to its parent should be
a `Weak<T>` reference in the definition of `Node`, we’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.

## Summary

We’ve now covered how you can use different kinds of smart pointers to choose
different guarantees and tradeoffs than those Rust makes with regular
references. `Box<T>` has a known size and points to data allocated on the heap.
`Rc<T>` keeps track of the number of references to data on the heap so that
data can have multiple owners. `RefCell<T>` with its interior mutability gives
us a type that can be used where we need an immutable type, and enforces the
borrowing rules at runtime instead of at compile time.

We’ve also discussed the `Deref` and `Drop` traits that enable a lot of smart
pointers’ functionality. We explored how it’s possible to create a reference
cycle that would cause a memory leak, and how to prevent reference cycles by
using `Weak<T>`.

If this chapter has piqued your interest and you now want to implement your own
smart pointers, check out The Nomicon at
*https://doc.rust-lang.org/stable/nomicon/vec.html* for even more useful
information.

Next, let’s talk about concurrency in Rust. We’ll even learn about a few new
smart pointers that can help us with it.
