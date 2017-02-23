# Smart Pointers

Now that we've learned quite a bit of Rust, we can start digging into some more
complicated concepts. In this chapter, we'll learn about a design pattern in
Rust called a *smart pointer*. This pattern allows us to leverage Rust's
ownership and borrowing features to manage all kinds of resources in a safe way.

TODO: more motivation

We'll be covering:

* What smart pointers are
* The `Deref` and `Drop` traits
* `Box<T>`, the simplest smart pointer
* Other common smart pointers in the standard library, like `Rc<T>`
* A family of smart pointers called *cells* which give you *interior
  mutability*

Let's dive in!

## Smart Pointers Own and Manage Data or Resources

So what are smart pointers, anyway? Well, we've learned about references in
Rust in Chapter 4. *Pointer* is a generic programming term for something like a
reference, that is, pointers "point at" data somewhere else. References are a
kind of pointer that only borrow data; by contrast, in many cases, smart
pointers *own* the data that they point to. That is, they have extra
capabilities that references don't, hence the "smart" nickname.

We've actually already encountered a few smart pointers in this book, we didn't
call them that by name, though. For example, in a certain sense, `String` and
`Vec<T>` from Chapter 8 are both smart pointers. They own some memory and allow
you to manipulate it. Another good example is `File`, which we used for our
I/O project in Chapter 12: it owns and manages a file handle that the operating
system gives us.

Given that this is a general design pattern in Rust, this chapter won't cover
every smart pointer that exists. Many libraries will build their own as well,
and you may write some for your own code. The ones we cover here will be the
most common ones from the standard library.

## `Box<T>` Points to Data on the Heap and Has a Known Size

The most straightforward smart pointer is a *box*, whose type is more properly
written as `Box<T>`. Boxes allow you to put a single value on the heap, and the
pointer to that value lives on the stack. (We talked about the stack vs. the
heap in Chapter 4.) Listing 15-1 shows how to use a box to store an `i32` on
the heap:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let b = Box::new(5);

    println!("b = {}", b);

} // b goes out of scope here, and the data on the heap is deallocated
```

<figcaption>

Listing 15-1: Storing an `i32` value on the heap using a box

</figcaption>
</figure>

This will print `b = 5`. In this case, we can access the data in the box in a
similar way as we would if this data was on the stack. Just like any value that
has ownership, when a box goes out of scope, it will be deallocated, which
means the box (stored on the stack) and the data (stored on the heap) will both
be deallocated.

Putting a single value on the heap isn't very useful, so you won't use boxes by
themselves in the way that Listing 15-1 does very often. A time when boxes are
useful are when you want to ensure that your type has a known size. For
example, consider Listing 15-2, which contains an enum definition for a *cons
list*, a type of data structure that comes from functional programming. A cons
list is a list where each item contains the next item until the end of the
list, which is signified by a value called `Nil`. Note that we aren't
introducing the idea of "nil" or "null" that we discussed in Chapter 6, this is
just a regular enum variant name we're using because it's the canonical name to
use when describing this data structure.

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
enum List<T> {
    Cons(T, List<T>),
    Nil,
}
```

<figcaption>

Listing 15-2: An enum definition for a cons list data structure

</figcaption>
</figure>

Using a cons list to store the list `1, 2, 3` would look like this:

```rust,ignore
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

The first `Cons` value holds `1` and another `List<T>` value. This `List<T>`
value is another `Cons` value that holds `2` and another `List<T>` value. This
is one more `Cons` value that holds `3` and a `List<T>` value, which is finally
`Nil`, the non-recursive variant that signals the end of the list.

However, if we try to compile the above code, we get the error shown in Listing
15-3:

<figure>

```text
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 |   enum List<T> {
  |  _^ starting here...
2 | |     Cons(T, List<T>),
3 | |     Nil,
4 | | }
  | |_^ ...ending here: recursive type has infinite size
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

<figcaption>

Listing 15-3: The error we get when attempting to define a recursive enum

</figcaption>
</figure>

The error says this type 'has infinite size'. Why is that? It's because we've
defined `List<T>` to have a variant that is recursive: it holds another value
of itself. This means Rust can't figure out how much space it needs in order to
store a `List<T>` value. Let's break this down a bit: recall the `Message` enum
we defined in Listing 6-2 when we discussed enum definitions in Chapter 6:

```rust
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

When the Rust compiler looks at a type like the enum in Listing 15-2, it tries
to figure out how much memory is needed to store value of `List<T>`, and starts
by looking at the `Cons` variant. The `Cons` variant holds a value of type `T`
and a value of type `List<T>`, so it can use however much the size of `T` is
plus the size of `List<T>`. To figure out how much memory a `List<T>` needs, it
looks at its variants, starting with the `Cons` variant. The `Cons` variant
holds a value of type `T` and a value of type `List<T>`, and this continues
infinitely. Rust can't figure out how much space to allocate for recursively
defined types, so the compiler gives the error in Listing 15-3.

The compiler did give a helpful suggestion in the error output:

```text
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
        make `List` representable
```

Because a `Box<T>` is a pointer, we always know what size it is: a `usize`,
which is the size of a pointer. The value of the `usize` will be the address of
the heap data. The heap data can be any size, but the address to the start of
that heap data will always fit in a `usize`. So if we change our definition
from Listing 15-2 to look like this:

```rust,ignore
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
```

The compiler will be able to figure out the size of a `List<T>` value. It will
look at this type, and start by looking at the `Cons` variant. The `Cons`
variant will need the size of whatever `T` is, plus the space to store a
`usize`, since a box always has the size of a `usize`. Then it looks at the
`Nil` variant, which does not store a value, so it doesn't need any extra
memory. We've broken the infinite, recursive chain by adding in a box. This is
the main area where boxes are useful: breaking up an infinite data structure so
that the compiler can know what size it is. We'll look at another case where
Rust has data of unknown size in Chapter 17 when we discuss trait objects.

Even though you won't be using boxes very often, they are a good way to
understand the smart pointer pattern. The attribute of the `Box<T>` type that
makes it a smart pointer is that it implements two important traits: `Deref`
and `Drop`. Let's investigate how boxes use these in more detail.

## The `Deref` Trait Allows Access to the Data Through a Reference

The first important smart pointer-related trait is `Deref`, which allows us to
override the `*` operator, the dereference operator (as opposed to the
multiplication operator). Overriding `*` for smart pointers makes accessing the
data behind the smart pointer convenient, and we'll talk about what we mean by
convenient in the next section about deref coercions. Remember using `*` when
we talked about references, like this:

TODO: put this back in ch 4

```rust
let mut x = 5;
{
    let y = &mut x;

    *y += 1
}

assert_eq!(6, x);
```

We use `*y` to access the data that `y` refers to, rather than `y` itself,
which is a reference.

Listing 15-4 has an example of overloading `*` using `Deref` on a struct we've
defined to hold mp3 data and metadata. `Mp3` is, in a sense, a smart pointer:
it owns the `Vec<u8>` data containing the audio. In addition, it holds some
optional metadata, in this case the artist and title of the song in the audio
data. We want to be able to conveniently access the audio data, not the
metadata, so we implement the `Deref` trait to return the audio data.
Implementing the `Deref` trait requires implementing one method named `deref`
that borrows `self` and returns the inner data:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: String,
    title: String,
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
        artist: String::from("Nirvana"),
        title: String::from("Smells Like Teen Spirit"),
    };

    assert_eq!(vec![1, 2, 3], *my_favorite_song);
}
```

<figcaption>

Listing 15-4: An implementation of the `Deref` trait on a struct that holds mp3
file data and metadata

</figcaption>
</figure>

Most of this should look familliar: a struct, a trait implementation, a main
function that creates an instance of the struct. There is one part we haven't
explained thoroughly yet: similarly to Chapter 13 when we looked at the
Iterator trait with the `type Item`, the `type Target = T;` syntax is defining
an associated type, which is covered in more detail in Chapter 20. Don't worry
about that part of the example too much; it is a slightly different way of
declaring a generic parameter.

In the `assert_eq!`, we're verifying that `vec![1, 2, 3]` is the result we get
when dereferencing the `Mp3` instance with `*my_favorite_song`, which is what
happens since we implemented the `deref` method to return the audio data. If
we hadn't implemented the `Deref` trait for `Mp3`, Rust wouldn't compile the
code `*my_favorite_song`: we'd get an error saying type `Mp3` cannot be
dereferenced.

The reason this code works is that what the `*` operator is doing behind
the scenes when we call `*my_favorite_song` is:

TODO: which is better?
A:

```rust,ignore
*Deref::deref(&my_favorite_song)
```

We borrow `my_favorite_song` and call the `Deref::deref` method

or B:

```rust,ignore
*(my_favorite_song.deref())
```

We call the `deref` method on `my_favorite_song`, which borrows
`my_favorite_song` and returns a reference to `my_favorite_song.audio`, since
that's what we defined `deref` to do in Listing 15-4. `*` on references is
built-in, so the expansion of `*` to call `deref` doesn't recurse. So we end up
with data of type `Vec<u8>`, which matches the `vec![1, 2, 3]` in the
`assert_eq!` in Listing 15-4.

The reason that the return type of the `deref` method is still a reference and
why it's necessary to dereference the result of the method is that if the
`deref` method returned just the value, using `*` would always take ownership.

### Implicit `deref` Coercions with Functions and Methods

TODO: consistent spelling of `deref` coercion (`Deref` coercion? deref coercion?)

Rust tends to favor explicitness over implicitness, but one case where this
does not hold true is *`deref` coercions* of arguments to functions and
methods. A `deref` coercion will automatically convert arguments passed to
functions or methods that are references to pointers or smart pointers into
references to the pointer's contents if needed to match the types of the
parameters defined in the signature. This was added to make calling functions
and methods not need as many explicit references and dereferences with `&` and
`*`.

Using our `Mp3` struct from Listing 15-4, here's the signature of a function to
compress mp3 audio data that takes a slice of `u8`:

```rust,ignore
fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    // the actual implementation would go here
}
```

Because of `deref` coercion and our implementation of the `Deref` trait on
`Mp3`, we can call this function with the data in `my_favorite_song` by using
this code:

```rust,ignore
let result = compress_mp3(&my_favorite_song);
```

Without `deref` coercsion and our `Deref` implementation, we would have to
write `compress_mp3(my_favorite_song.audio.as_slice())`. Rust knows that `Mp3`
implements the `Deref` trait and returns `&Vec<u8>` from the `deref` method.
The standard library also implements the `Deref` trait on `Vec<T>` to return
`&[T]` from the `deref` method. So Rust will implicitly call `Deref::deref`
twice to turn `&Mp3` into `&Vec<u8>` and then into `&[T]` to match the
signature of `compress_mp3`. That means we get to do less typing! Rust will
call `Deref::deref` as many times as it needs to in order to get a reference to
match the parameter's type, when the `Deref` trait is defined for the types
involved.

TODO: is there a runtime cost for this? no, the compiler resolves the indirection?

There's also a `DerefMut` trait for overriding `*` on `&mut T` for use in
assignment in the same fashion that we use `Deref` to override `*` on `&T`s.

// TODO: insert x/y example here

Rust does this kind of coercion in three cases:

* From `&T` to `&U` when `T: Deref<U>`.
* From `&mut T` to `&mut U` when `T: DerefMut<U>`.
* From `&mut T` to `&U` when `T: Deref<U>`.

The first two are the same, except for mutability: if you have a `&T`, and
`T` implements `Deref` to some type `U`, you can get a `&U` transparently. Same
for mutable references. The last one is more tricky: if you have a mutable
reference, it will also coerce to an immutable one. The other case is _not_
possible though: immutable references will never coerce to mutable ones.

The reason that the `Deref` trait is important to the smart pointer pattern is
that smart pointers can then be treated like regular pointers for places that
expect regular pointers. We don't have to redefine methods and functions to
take smart pointers explicitly, for example.

## The `Drop` trait

The other trait that's important to the smart pointer pattern is the `Drop`
trait. `Drop` lets us run some code when a value is about to go out of scope.
Listing 15-5 shows an implementation of

```rust
struct Fireworks {
    number: i32,
}

// Drop is in the prelude, so we don't need to import it
impl Drop for Fireworks {
    fn drop(&mut self) {
        println!("BOOM! {} fireworks explode!", self.number);
    }
}

fn main() {
    let f = Fireworks { number: 10 };
    println!("Fireworks created.");
    println!("Wait for it...");
}
```

When we run this program, we'll see

```text
Fireworks created.
Wait for it...
BOOM! 10 fireworks explode!
```

printed to the screen. Our message is printed, and then when `f` goes out of
scope at the end of `main`, `drop` is called, and we see the message about
exploding printed. We can also use the `Drop::drop` function to call `Drop` a
bit earlier:

```rust,ignore
fn main() {
    let f = Fireworks { number: 10 };
    println!("Fireworks created.");
    drop(f);
    println!("Wait for it...");
}
```

This will print:

```text
Fireworks created.
BOOM! 10 fireworks explode!
Wait for it...
```

Normally, you wouldn't want to wait for fireworks _after_ they explode, but you
get the idea.

The ability to run some code when something goes out of scope is very powerful.
For example, here's something that looks similar to a box. This code won't
_work_, but it illustrates the concept:

```rust,ignore
struct MyBox<T> {
    data: &T,
}

impl<T> MyBox<T> {
    fn new(data: T) {
        // allocate some memory
        let mut memory = allocate_memory();

        // copy the data into that memory
        *memory = T;

        // return a box containing the reference to the memory
        MyBox {
            data: memory,
        }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        free_memory(self.data);
    }
}
```

TODO: replace this with closing a socket and just mention that this could be used to implement a custom memory allocator?



So first, we declare `MyBox<T>` to have a reference to some data. We can't use
actual `&T` references for this, we'd use `*const T`, but we won't talk about
*raw pointer*s until Chapter 20. So for now, pretend that this works; it's the
same idea.

In `new`, we allocate some memory by asking the operating system for it. This
function doesn't exist in Rust today, so we made up a function name,
`allocate_memory`, for it. This function would return a reference to the new
memory we've been granted. We then copy our data into it by using the `*`
operator. So after the assignment, `memory` will be pointing to a copy of our
`data`. Finally, we return our new `MyBox`, created from this memory.

That takes care of allocation, but what about deallocation? For that, we use
`Drop`, along with another fake function: `free_memory`. This function will
return the memory back to the operating system.

So we're automatically allocating memory in `new`, and then automatically
freeing it in `drop`. We can't mess it up. The ownership system makes sure that
we call `drop` at the right time, and things work out.

Now that you understand the basics of smart pointers, let's talk about a few
others defined in the standard library that add even more useful functionality.

## The Reference Counted Smart Pointer `Rc<T>`

In the majority of cases, ownership is very clear: you know exactly which
binding owns a given value. However, this isn't always the case;
sometimes, you may actually need multiple owners. For this, Rust has a type
called `Rc<T>`. Its name is an abbreviation for *reference counting*. Reference counting is... TODO

This should be used when we wish to dynamically allocate and share some data
(read-only) between various portions of your program, where it is not certain
which portion will finish using the pointer last. It's a viable alternative to
&T when &T is either impossible to statically check for correctness, or creates
extremely unergonomic code where the programmer does not wish to spend the
development cost of working with.

TODO: insert something like the turtle example
TODO: insert why reference counting is useful-- multiple owners may use the same data, and the data goes out of scope once all of its owners go out of scope and not before.

Let's look at an example:

```rust
use std::rc::Rc;

let r1 = Rc::new(5);
let r2 = r1.clone();
```

You've seen the `clone` method previously, it's usually used for making a
complete copy of some data. With `Rc<T>`, though, it doesn't make a full copy.
`Rc<T>` holds a 'reference count', that is, a count of how many clones exist.
Let's look at this example in more detail:

```rust
use std::rc::Rc;

fn main() {
    let r1 = Rc::new(5); // here, we have an Rc<i32> with a reference count of one.

    {
        let r2 = r1.clone(); // here, we increment the reference count; now both r1 and
                             // r2 both refer to the same 5; and we have a total count
                             // of two.
    }
    // Here, the scope is ending. r2 goes out of scope first. When it does, it
    // doesn't free the 5, like a Box<i32> would; it decrements the count. So after
    // r2 goes out of scope, the count is one.
}
// Now, r1 goes out of scope. When it does, it decrements the count. The count
// is now zero. Since the count is zero, the value is deallocated.
```

This is the basic strategy: you make an `Rc<T>` with `new`, and then call
`clone` for all of the other owners you need. With each `clone`, the count goes
up by one. When each `Rc<T>` goes out of scope, they decrease the count, and
when the count is zero, the value is deallocated. This strategy lets us have
multiple owners, as the count will ensure that the value remains valid as long
as any of the owners still exist.

This idea is simple enough, but there are a few twists! Here's the first: In
order for this to be okay, the data inside of an `Rc<T>` must be immutable.  If
it were mutable, we'd run into a similar problem with borrowing: two mutable
borrows to the same place can cause a lot of problems.

TODO: stop here, move this later?

There's another twist, too: an "`Rc<T>` cycle." While single ownership is easy
to reason about, multiple ownership is a lot trickier. Before we cover these
situations in detail, we need to talk about another type: `RefCell<T>`.

## The Interior Mutability Pattern

*Interior mutability* is a design pattern in Rust for allowing you to mutate
something that's immutable. Wait, what? Let's compare these two pieces of code.
What's different about them?

```rust
use std::cell::Cell;

// one
let mut five = 5;

five = 6;

// two
let five = Cell::new(5); // no mut!

five.set(6);
```

There are three things that are different:

1. We use `5` in sample one, but `Cell::new(5)` in sample two.
2. We use `=` in sample one, but `set` in sample two.
3. `five` is mutable in sample one, but not in sample two.

That third difference? That's interior mutability.

Why are we allowed to do this? Well, to some degree, we're not. The reason we
call interior mutability a "pattern" is that it's not really a language
feature, it's a design pattern for libraries. More specifically, it's a pattern
that uses `unsafe` code inside to bend Rust's usual rules. We haven't yet
covered unsafe code, we will in Chapter 20. Luckily for us, you don't
have to understand how it works inside to use it. All you need to know is that
the various family of `Cell` types, as well as some others like `Mutex<T>`
(that we'll cover in the next chapter, on concurrency) follow this pattern.

TODO: but no, really why are we allowed to do this, everything is meaningless now

Why is this useful? Well, remember when we said that `Rc<T>` has to store
immutable data? Given that `RefCell<T>` is immutable, but has interior
mutability, we can combine `Rc<T>` and `RefCell<T>` to get a type that's both
reference counted and mutable. Like this:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let five = Rc::new(RefCell::new(5));

let other_rc = five.clone();

*other_rc.borrow_mut() = 6;
```

This is where interior mutability is useful: when you have something that
requires immutability, but you also need to mutate something. This comes up
with types like `Rc<T>`, but it can also come up in concurrency situations.  In
general, it's a fairly rare thing to need, but when you need it, it does exist.

## `RefCell<T>`

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the
data that it holds. So, what makes `RefCell<T>` different than a type like
`Box<T>`? For that, we'll have to recall the borrowing rules we learned in
Chapter 4:

1. At any given time, you can have either but not both of:
    * One mutable reference.
    * Any number of immutable references.
2. References must always be valid.

With references and `Box<T>`, these invariants are enforced at compile time.
But with `RefCell<T>`, these invariants are enforced *at runtime*. With
references, if you break these rules, you'll get a compiler error. With
`RefCell<T>`, if you break these rules, you'll get a `panic!`.

Static analysis, like Rust performs, is inherently conservative. That is, if we
accept an incorrect program, very bad things happen, but if we reject a correct
program, the programmer will be inconvenienced, but nothing catastrophic can
occur. `RefCell<T>` is useful in two situations:

1. When you know that the borrowing rules are respected, but when the compiler
  can't understand that that's true.
2. When you need "interior mutability."

// TODO: why might you want to choose your guarantees differently?
// - rustc can't reason about them (why can't it)

Before we talk about how to use `RefCell<T>`, we should mention one more thing:
`RefCell<T>` is only useful in single-threaded scenarios; it's not threadsafe.
We'll talk about concurrency and paralellism in the next chapter; for now, all
you need to know is that if you try to use `RefCell<T>` in a multi-threaded
context, you'll get a compile time error.


1:36 PM <steveklabnik> 18:35 < aturon> steveklabnik: then i think you can just give a quick shout-out to `Cell`
1:36 PM <steveklabnik> 18:35 < nmatsakis> but without it, I feel like I would never have fixed all the bugs we
1:36 PM <steveklabnik>                    found :)
1:36 PM <steveklabnik> 18:35 < aturon> steveklabnik: basically: "If you're working with `Copy` types, or othewise
1:36 PM <steveklabnik>                 only plan to move values in and out, `Cell` is more ergonomic and lightweight"
1:36 PM <steveklabnik> 18:35 < nmatsakis> (ps I agree with that order: RefCell as dynamic borrowing, then "oh some
1:36 PM <steveklabnik> times cell works too"P)
1:36 PM <steveklabnik> 18:35 < aturon> steveklabnik: with a pointer to the `std` docs

'Cell<T> is like RefCell<T> but the instead of giving references to the inner value, the value is copied in and out of the Cell<T>'.

### The `borrow` and `borrow_mut` methods

With references, we use the `&` and `&mut` syntax to create references and
mutable references, respectively. But with `RefCell<T>`, we use the `borrow`
and `borrow_mut` methods:

```rust
use std::cell::RefCell;

let five = RefCell::new(5);

// we need these scopes so we don't break the rules!
{
    let r1 = five.borrow();
    let r2 = five.borrow();
    let r3 = five.borrow();

    // r1, r2, and r3 are all immutable references.
}

{
    let r = five.borrow_mut();

    // r1 is a mutable reference
}
```

TODO: do something interesting with these

If we call both `borrow` and `borrow_mut` in the same scope, we'll get a panic.

So why do we need `RefCell<T>`? What good is it to enforce the rules at runtime,
instead of compile time?

## Leaking Memory by Creating Cycles is Safe

We've shown that with `Rc<T>`s, when the last one goes out of scope, the value
is deallocated. But what about this program?

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Cycle {
    really_bad: RefCell<Option<Rc<Cycle>>>,
    leaked_data: i32,
}

fn main() {
    let mut oh_no = Rc::new(Cycle {
        really_bad: RefCell::new(None),
        leaked_data: 5,
    });

    let clone = oh_no.clone();

    *oh_no.really_bad.borrow_mut() = Some(clone);
}
```

TODO: add diagram of oh_no pointing to itself

There's a lot going on here. `Cycle` is a type that contains an `Rc<Cycle>`.
This means that we can have a 'reference cycle', hence the name of the struct.
Here, we've constructed one in `main`: we have `oh_no`, and then we create a
clone of it, `clone`. Since we've made a clone of it, the reference count is
two: one for the initial `Rc<T>`, and one for the clone. When `oh_no` goes out
of scope at the end of `main`, it will decrement the count to one. But that's
it: `clone` never really goes out of scope, since it was moved into `oh_no`.
This means that this memory is now unreachable, yet will never be cleaned up.
It'll just sit there with a count of one, forever. In this specific case, the
program ends right away, so it's not a problem, but in a more complex program
that allocates lots of memory in a cycle and holds onto it for a long time,
this would be a problem.

Now, as you can see, creating reference cycles is difficult and inconvenient in
Rust. To be honest, your authors had to look up previous discussions of an
example to get this right. But it's not impossible: preventing memory leaks in
the form of reference cycles is not one of the guarantees Rust makes. In other
words, memory leaks are memory safe. If you have an `Rc<T>` that contains an
`Rc<T>`, be aware that you'll have to ensure that you don't create cycles on
your. One way of doing that is using `Weak<T>`.

### Prevent Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

To help with this problem, the Rust standard library contains another smart
pointer type: `Weak<T>`. Let's see what replacing the inner `Rc<T>` that
`Cycle` holds with a `Weak<T>` looks like:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Cycle {
    really_bad: RefCell<Option<Weak<Cycle>>>,
    leaked_data: i32,
}

fn main() {
    let mut oh_no = Rc::new(Cycle {
        really_bad: RefCell::new(None),
        leaked_data: 5,
    });

    let clone = Rc::downgrade(&oh_no.clone());

    *oh_no.really_bad.borrow_mut() = Some(clone);
}
```

`Weak<T>` is exactly like `Rc<T>`, except that its reference count doesn't,
well, count when determining if something should be dropped. You can turn an
`Rc<T>` into a `Weak<T>` with the `Rc::downgrade` associated function, which
takes an `&Rc<T>` as an argument, and gives a `Weak<T>` back.

TODO: print out values

So in this example, when we call `oh_no.clone()`, we increment the count to
two. But when we pass that clone to `downgrade`, that count goes down again, to
one. Now, at the end of the function, when `oh_no` goes out of scope, it
reduces the count from one to zero, and the memory is freed. Success! We've
broken the cycle.

TODO: what does `clone` hold after `oh_no` goes out of scope then?

If you're doing complex things with `Rc<T>`s, you should investigate if it's
possible for you to have a cycle, and insert a `Weak<T>` so that you don't
create cycles and leak memory. The specifics depend on exactly what you're
doing, but luckily, this isn't a Rust-specific idea; all of this translates
over to other reference counting libraries in other languages, so doing some
reading about it should help you.

## Summary

Whew! Smart pointers are powerful, but complex. We've covered the basics of
smart pointers, and how to use some of the most common smart pointers.
Implementing your own smart pointers is out of the scope of this book; you
should check out the Nomicon if you're interested in building these kinds of
abstractions.

Next, let's talk about concurrency in Rust. We'll even learn about a few new
smart pointers that can help us with it.
