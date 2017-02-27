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
* The *interior mutability* pattern that lets you...
* Reference cycles, how they can leak memory, and how to prevent them

Let's dive in!

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
}
```

<figcaption>

Listing 15-1: Storing an `i32` value on the heap using a box

</figcaption>
</figure>

This will print `b = 5`. In this case, we can access the data in the box in a
similar way as we would if this data was on the stack. Just like any value that
has ownership of data, when a box goes out of scope like `b` does at the end of
`main`, it will be deallocated. The deallocation happens for both he box
(stored on the stack) and the data it points to (stored on the heap).

Putting a single value on the heap isn't very useful, so you won't use boxes by
themselves in the way that Listing 15-1 does very often. A time when boxes are
useful is when you want to ensure that your type has a known size. For
example, consider Listing 15-2, which contains an enum definition for a *cons
list*, a type of data structure that comes from functional programming.

A cons list is a list where each item contains a value and the next item until
the end of the list, which is signified by a value called `Nil`. Note that we
aren't introducing the idea of "nil" or "null" that we discussed in Chapter 6,
this is just a regular enum variant name we're using because it's the canonical
name to use when describing the cons list data structure.

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
 -->
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
store a `List<T>` value. Let's break this down a bit: first let's look at how
Rust decides how much space it needs to store a value of a non-recursive type.
Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:

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

Contrast this to what happens when the Rust compiler looks at a recursive type
like the `List<T>` in Listing 15-2. The compiler tries to figure out how much
memory is needed to store value of `List<T>`, and starts by looking at the
`Cons` variant. The `Cons` variant holds a value of type `T` and a value of
type `List<T>`, so it can use however much the size of `T` is plus the size of
`List<T>`. To figure out how much memory a `List<T>` needs, it looks at its
variants, starting with the `Cons` variant. The `Cons` variant holds a value of
type `T` and a value of type `List<T>`, and this continues infinitely. Rust
can't figure out how much space to allocate for recursively defined types, so
the compiler gives the error in Listing 15-3.

The compiler did give a helpful suggestion in the error output:

```text
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
        make `List` representable
```

Because a `Box<T>` is a pointer, we always know how much space it needs: a
pointer takes up a `usize` amount of space. The value of the `usize` will be
the address of the heap data. The heap data can be any size, but the address to
the start of that heap data will always fit in a `usize`. So if we change our
definition from Listing 15-2 to look like the definition here in Listing 15-4,
and change `main` to use `Box::new` for the values inside the `Cons` variants
like so:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
enum List<T> {
    Cons(T, Box<List<T>>),
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

<figcaption>

Listing 15-4: Definition of `List<T>` that uses `Box<T>` in order to have a
known size

</figcaption>
</figure>

The compiler will be able to figure out the size it needs to store a `List<T>`
value. Rust will look at `List<T>`, and again start by looking at the `Cons`
variant. The `Cons` variant will need the size of whatever `T` is, plus the
space to store a `usize`, since a box always has the size of a `usize`, no
matter what it's pointing to. Then Rust looks at the `Nil` variant, which does
not store a value, so `Nil` doesn't need any space. We've broken the infinite,
recursive chain by adding in a box. This is the main area where boxes are
useful: breaking up an infinite data structure so that the compiler can know
what size it is. We'll look at another case where Rust has data of unknown size
in Chapter 17 when we discuss trait objects.

Even though you won't be using boxes very often, they are a good way to
understand the smart pointer pattern. Two of the aspects of `Box<T>` that are
commonly used with smart pointers are its implementations of the `Deref` trait
and the `Drop` trait. Let's investigate how these traits work and how smart
pointers use them.

## The `Deref` Trait Allows Access to the Data Through a Reference

The first important smart pointer-related trait is `Deref`, which allows us to
override `*`, the dereference operator (as opposed to the multiplication
operator). Overriding `*` for smart pointers makes accessing the data behind
the smart pointer convenient, and we'll talk about what we mean by convenient
in the next section about deref coercions. Remember using `*` when we talked
about references, like this:

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

END TODO

Listing 15-5 has an example of overloading `*` using `Deref` on a struct we've
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

<figcaption>

Listing 15-5: An implementation of the `Deref` trait on a struct that holds mp3
file data and metadata

</figcaption>
</figure>

Most of this should look familiar: a struct, a trait implementation, and a
main function that creates an instance of the struct. There is one part we
haven't explained thoroughly yet: similarly to Chapter 13 when we looked at the
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

```rust,ignore
*(my_favorite_song.deref())
```

This calls the `deref` method on `my_favorite_song`, which borrows
`my_favorite_song` and returns a reference to `my_favorite_song.audio`, since
that's what we defined `deref` to do in Listing 15-5. `*` on references is
defined to just follow the reference and return the data, so the expansion of
`*` doesn't recurse for the outer `*`. So we end up with data of type
`Vec<u8>`, which matches the `vec![1, 2, 3]` in the `assert_eq!` in Listing
15-5.

The reason that the return type of the `deref` method is still a reference and
why it's necessary to dereference the result of the method is that if the
`deref` method returned just the value, using `*` would always take ownership.

### Implicit Deref Coercions with Functions and Methods

Rust tends to favor explicitness over implicitness, but one case where this
does not hold true is *deref coercions* of arguments to functions and methods.
A deref coercion will automatically convert references to pointers or smart
pointers, when passed to functions or methods, into references to the pointer's
contents if needed to match the types of the parameters defined in the
signature. Deref coercion was added to Rust to make calling functions and
methods not need as many explicit references and dereferences with `&` and `*`.

Using our `Mp3` struct from Listing 15-5, here's the signature of a function to
compress mp3 audio data that takes a slice of `u8`:

```rust,ignore
fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    // the actual implementation would go here
}
```

Because of deref coercion and our implementation of the `Deref` trait on `Mp3`,
we can call this function with the data in `my_favorite_song` by using this
code:

```rust,ignore
let result = compress_mp3(&my_favorite_song);
```

Without deref coercion and our `Deref` implementation on `Mp3`, we would have
to write `compress_mp3(my_favorite_song.audio.as_slice())` in order to call
`compress_mp3` with a slice of the audio data. Deref coercion means that Rust
can use its knowledge of our `Deref` implementation, namely: Rust knows that
`Mp3` implements the `Deref` trait and returns `&Vec<u8>` from the `deref`
method. Rust also knows the standard library implements the `Deref` trait on
`Vec<T>` to return `&[T]` from the `deref` method (and we can find that out too
by looking at the API documentation for `Vec<T>`). So, at compile time, Rust
will see that it can use `Deref::deref` twice to turn `&Mp3` into `&Vec<u8>`
and then into `&[T]` to match the signature of `compress_mp3`. That means we
get to do less typing! Rust will analyze types through `Deref::deref` as many
times as it needs to in order to get a reference to match the parameter's type,
when the `Deref` trait is defined for the types involved. The indirection is
resolved at compile time, so there is no run-time penalty for taking advantage
of deref coercion.

There's also a `DerefMut` trait for overriding `*` on `&mut T` for use in
assignment in the same fashion that we use `Deref` to override `*` on `&T`s.

TODO: insert x/y example here

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
places that expect regular references. We don't have to redefine methods and
functions to take smart pointers explicitly, for example.

## The `Drop` Trait

The other trait that's important to the smart pointer pattern is the `Drop`
trait. `Drop` lets us run some code when a value is about to go out of scope.
This is especially useful for smart pointers that manage a resource as opposed
to those that manage memory: often resources like files or network connections
need to be closed when our code is done with them. In other languages, if we
forget to call code to close these kinds of resources, the system our code is
running on might get overloaded and crash.

In Rust, we can specify that some code should be run when a value goes out of
scope. The compiler will insert this code automatically. That means we don't
need to remember to put this code everywhere we're done with an instance of
these types, but we still won't leak resources!

The way we specify code should be run when a value goes out of scope is by
implementing the `Drop` trait. The `Drop` trait requires us to implement one
method named `drop` that takes a mutable reference to `self`.

Listing 15-6 shows a `WebSocket` struct that doesn't actually connect to
anything, but it prints out `Closing the socket!` when we create the struct and
when it goes out of scope so that we can see when this code gets run:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
struct WebSocket {
    uri: String,
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        println!("Closing the socket!");
    }
}

fn main() {
    let w = WebSocket { uri: String::from("http://example.com/not-real") };
    println!("WebSocket created.");
    println!("Wait for it...");
}
```

<figcaption>

Listing 15-6: A `WebSocket` struct that implements the `Drop` trait, where we
could put code that would close the socket.

</figcaption>
</figure>

The `Drop` trait is in the prelude, so we don't need to import it. The `drop`
method implementation calls the `println!`; this is where you'd put the actual
code needed to close the socket. In `main`, we create a new instance of
`WebSocket` then print out `WebSocket created.` to be able to see that our code
got to that point at runtime. At the end of `main`, our instance of `WebSocket`
will go out of scope. Note that we didn't call the `drop` method explicitly.

When we run this program, we'll see:

```text
WebSocket created.
Wait for it...
Closing the socket!
```

printed to the screen, which shows that Rust automatically called `drop` for us
when our instance went out of scope.

We can use the `std::mem::drop` function to drop a value earlier than when it
goes out of scope. This isn't usually necessary; the whole point of the `Drop`
trait is that it's taken care of automatically for us. We'll see an example of
a case when we'll need to drop a value earlier than when it goes out of scope
in Chapter 16 when we're talking about concurrency. For now, let's just see
that it's possible, and `std::mem::drop` is in the prelude so we can just call
`drop` as shown in Listing 15-7:

<figure>

```rust,ignore
fn main() {
    let w = WebSocket { uri: String::from("http://example.com/not-real") };
    println!("WebSocket created.");
    drop(w);
    println!("Wait for it...");
}
```

<figcaption>

Listing 15-7: Calling `std::mem::drop` to explicitly drop a value before it
goes out of scope

</figcaption>
</figure>

Running this code will print the following, showing that the destructor code is
called since `Closing the socket!` is printed between `WebSocket created.` and
`Wait for it...`:

```text
WebSocket created.
Closing the socket!
Wait for it...
```

Note that we aren't allowed to call the `drop` method that we defined directly:
if we replaced `drop(w)` in Listing 15-7 with `w.drop()`, we'll get a compiler
error that says `explicit destructor calls not allowed`. TODO: why aren't we
allowed to call the drop method directly?

The definition of `std::mem::drop` is:

```rust
pub mod std {
    pub mod mem {
        pub fn drop<T>(_x: T) { }
    }
}
```

This function is generic over any type `T`, so we can pass any value to it. The
function doesn't actually have anything in its body, so it doesn't use its
parameter. The parameter is named `_x` because the `_` is a signal to the
compiler that we're intentionally not using the parameter, so it doesn't need
to warn us that we're not using it. The reason this empty function is useful is
that `drop` takes ownership of its parameter, which means the value gets
dropped at the end of this function when it goes out of scope.

Code specified in a `Drop` trait implementation can be used for many reasons to
make cleanup convenient and safe: we could use it to create our own memory
allocator, for instance! By using the `Drop` trait and Rust's ownership system,
we can't mess up and forget to clean up, or clean up a value that's still in
use. The ownership system makes sure that we call `drop` at the right time.

Now that we've gone over `Box<T>` and some of the characteristics of smart
pointers, let's talk about a few other smart pointers defined in the standard
library that add different kinds of useful functionality.

## The Reference Counted Smart Pointer `Rc<T>`

In the majority of cases, ownership is very clear: you know exactly which
variable owns a given value. However, this isn't always the case; sometimes,
you may actually need multiple owners. For this, Rust has a type called
`Rc<T>`. Its name is an abbreviation for *reference counting*. Reference
counting means keeping track of the number of references to a value in order to
know if a value is still in use or not. If there are zero references to a
value, we know we can clean up the value without any references becoming
invalid.

To think about this in terms of a real-world scenario, it's like a TV in a
family room. When one person comes in the room to watch TV, they turn it on.
Others can also come in the room and watch the TV. When the last person leaves
the room, they'll turn the TV off since it's no longer being used. If someone
turns off the TV while others are still watching it, though, the people
watching the TV would get mad!

`Rc<T>` is for use when we want to allocate some read-only data on the heap,
share that data between multiple parts of our program, and we can't determine
at compile time which part of our program using this data will finish using it
last. If we knew which part would finish last, we could make that part the
owner of the data and the normal ownership rules enforced at compile time would
kick in.

Note that `Rc<T>` is only for use in single-threaded scenarios; the next
chapter on concurrency will cover how to do reference counting in
multithreaded programs. If you try to use `Rc<T>` with multiple threads,
you'll get a compile-time error.

### Using `Rc<T>` to Share Data

Let's return to our cons list example from Listing 15-4. In Listing 15-8, we're
going to try to use `List<T>` as we defined it using `Box<T>`. First we'll
create one list instance that contains 5 and then 10. Next, we want to create
two more lists: one that starts with 3 and continues on to our first list
containing 5 and 10, then another list that starts with 4 and *also* continues
on to our first list containing 5 and 10. In other words, we want two lists
that both share ownership of the third list, which conceptually will be
something like this:

```text
b -> 3 ---v
a ------> 5 -> 10 -> Nil
c -> 4 ---^
```

Trying to implement this using our definition of `List<T>` with `Box<T>` won't
work:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
enum List<T> {
    Cons(T, Box<List<T>>),
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

<figcaption>

Listing 15-8: Having two lists using `Box<T>` that try to share ownership of a
third list won't work

</figcaption>
</figure>

If we compile this, we get this error:

```text
error[E0382]: use of moved value: `a`
  --> src/main.rs:13:30
   |
12 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
13 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
   |
   = note: move occurs because `a` has type `List<i32>`, which does not
   implement the `Copy` trait
```

The `Cons` variants own the data they hold, so when we create the `b` list it
moves `a` to be owned by `b`. Then when we try to use `a` again when creating
`c`, we're not allowed to since `a` has been moved.

We could change the definition of `Cons` to hold references instead, but then
we'd have to specify lifetime parameters and we'd have to construct elements of
a list such that every element lives at least as long as the list itself.
Otherwise, the borrow checker won't even let us compile the code.

Instead, we can change our definition of `List<T>` to use `Rc<T>` instead of
`Box<T>` as shown here in Listing 15-9:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
enum List<T> {
    Cons(T, Rc<List<T>>),
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

<figcaption>

Listing 15-9: A definition of `List<T>` that uses `Rc<T>`

</figcaption>
</figure>

Note that we need to add a `use` statement for `Rc` because it's not in the
prelude. In `main`, we create the list holding 5 and 10 and store it in a new
`Rc` in `a`. Then when we create `b` and `c`, we call the `clone` method on `a`.

### Cloning an `Rc<T>` Increases the Reference Count

We've seen the `clone` method previously, where we used it for making a
complete copy of some data. With `Rc<T>`, though, it doesn't make a full copy.
`Rc<T>` holds a *reference count*, that is, a count of how many clones exist.
Let's change `main` as shown in Listing 15-10 to have an inner scope around
where we create `c`, and to print out the results of the `Rc::strong_count`
associated function at various points. `Rc::strong_count` returns the reference
count of the `Rc` value we pass to it, and we'll talk about why this function
is named `strong_count` in the section later in this chapter about preventing
reference cycles.

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
# enum List<T> {
#     Cons(T, Rc<List<T>>),
#     Nil,
# }
#
# use List::{Cons, Nil};
# use std::rc::Rc;
#
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

<figcaption>

Listing 15-10: Printing out the reference count

</figcaption>
</figure>

This will print out:

```text
rc = 1
rc after creating b = 2
rc after creating c = 3
rc after c goes out of scope = 2
```

We're able to see that `a` has an initial reference count of one. Then each
time we call `clone`, the count goes up by one. When `c` goes out of scope, the
count is decreased by one. What we can't see in this example is that when `b`
and then `a` go out of scope at the end of `main`, the count of references to
the list containing 5 and 10 is then 0, and the list is dropped. This strategy
lets us have multiple owners, as the count will ensure that the value remains
valid as long as any of the owners still exist.

In the beginning of this section, we said that `Rc<T>` is for read-only data,
that is, the value in an `Rc` must be immutable. If it were mutable, we'd run
into a similar problem with borrowing: two mutable borrows to the same place
can cause data races and inconsistencies. But mutating data is very useful! In
the next section, we'll discuss the interior mutability pattern and the
`RefCell<T>` type that we can use in conjunction with an `Rc<T>` to work with
this restriction on immutability.

## The Interior Mutability Pattern

*Interior mutability* is a design pattern in Rust for allowing you to mutate
data that's immutable. The interior mutability pattern involves using `unsafe`
code inside a data structure to bend Rust's usual rules around mutation and
borrowing. We haven't yet covered unsafe code, we will in Chapter 20. The
`unsafe` code is then wrapped in a safe API, and the outer type is still
immutable.

Let's explore this by looking at the `RefCell<T>` type that follows the
interior mutability pattern.

### `RefCell<T>` has Interior Mutability

Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
that it holds. So, what makes `RefCell<T>` different than a type like `Box<T>`?
Let's recall the borrowing rules we learned in Chapter 4:

1. At any given time, you can have *either* but not both of:
  * One mutable reference.
  * Any number of immutable references.
2. References must always be valid.

With references and `Box<T>`, the borrowing rules' invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced *at runtime*.
With references, if you break these rules, you'll get a compiler error. With
`RefCell<T>`, if you break these rules, you'll get a `panic!`.

Static analysis, like the Rust compiler performs, is inherently conservative.
That is, if Rust accepts an incorrect program, people would not be able to
trust in the guarantees Rust makes. If Rust rejects a correct program, the
programmer will be inconvenienced, but nothing catastrophic can occur.
`RefCell<T>` is useful in two situations:

1. When you know that the borrowing rules are respected, but when the compiler
  can't understand that that's true.
2. When you need interior mutability.

Similarly to `Rc<T>`, `RefCell<T>` is only for use in single-threaded
scenarios. We'll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in the next chapter on concurrency. For now, all you
need to know is that if you try to use `RefCell<T>` in a multithreaded
context, you'll get a compile time error.

With references, we use the `&` and `&mut` syntax to create references and
mutable references, respectively. But with `RefCell<T>`, we use the `borrow`
and `borrow_mut` methods, which are part of the safe API that `RefCell<T>` has.

Listing 15-11 shows what it looks like to use `RefCell<T>` with functions that
borrow their parameters immutably and mutably. Note that the `data` variable is
declared as immutable with `let data` rather than `let mut data`, yet we're
allowed to mutate the value by using `a_fn_that_mutably_borrows`!

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::cell::RefCell;

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(b: &mut i32) {
    *b += 1;
}

fn main() {
    let data = RefCell::new(5);
    a_fn_that_immutably_borrows(&data.borrow());
    a_fn_that_mutably_borrows(&mut data.borrow_mut());
    a_fn_that_immutably_borrows(&data.borrow());
}
```

<figcaption>

Listing 15-11: Using `RefCell<T>`, `borrow`, and `borrow_mut`

</figcaption>
</figure>

This example prints:

```text
a is 5
a is 6
```

Here, we've created a new `RefCell<T>` containing the value 5. We can get an
immutable reference to the value inside the `RefCell<T>` by calling the `borrow`
method. More interestingly, we can get a mutable reference to the value inside
the `RefCell<T>` with the `borrow_mut` method, and the function
`a_fn_that_mutably_borrows` is allowed to change the value. We can see that the
next time we print out the value, it's 6 instead of 5.

Recall that because of the borrowing rules, this code trying to create two
mutable borrows in the same scope won't compile:

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

We'll get this compiler error:

```text
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
scope *will* compile, but it will panic at runtime instead. This code:

```rust,ignore
use std::cell::RefCell;

fn main() {
    let mut s = RefCell::new(String::from("hello"));

    let r1 = s.borrow_mut();
    let r2 = s.borrow_mut();
}
```

compiles but panics with the following error when we `cargo run`:

```text
    Finished dev [unoptimized + debuginfo] target(s) in 0.83 secs
     Running `target/debug/refcell`
thread 'main' panicked at 'already borrowed: BorrowMutError',
/stable-dist-rustc/build/src/libcore/result.rs:868
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

This runtime `BorrowMutError` is similar to the compiler error: it says we've
already borrowed `s` mutably once, so we're not allowed to borrow it again. We
aren't getting around the borrowing rules, we're just choosing to have Rust
enforce them at runtime instead of compile time. You could choose to use
`RefCell<T>` everywhere all the time, but in addition to having to type
`RefCell` a lot, you'd find out about possible problems later (possibly in
production rather than during development). Also, checking the borrowing rules
while your program is running has a performance penalty.

So why would we choose to make the tradeoffs that using `RefCell<T>` involves?
Well, remember when we said that `Rc<T>` has to store immutable data? Given
that `RefCell<T>` is immutable, but has interior mutability, we can combine
`Rc<T>` and `RefCell<T>` to get a type that's both reference counted and
mutable. Listing 15-12 shows an example of how to do that, again going back to
our cons list from Listing 15-9. In this example, the type we're filling in for
`T` is `Rc<RefCell<i32>>`:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
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

<figcaption>

Listing 15-12: Using `Rc<RefCell<T>>` to create a `List<T>` that we can mutate

</figcaption>
</figure>

We're creating a value, which is an instance of `T`. We're storing it in a
variable named `value` because we want to be able to access it directly later.
Then we create a `List<T>` in `a` that has a `Cons` variant that holds `value`,
and `value` needs to be cloned since we want `value` to also have ownership in
addition to `a`. Then we wrap `a` in an `Rc<T>` so that we can create lists `b`
and `c` that start differently but both refer to `a`, similarly to what we did
in Listing 15-9.

Once we have the lists in `shared_list`, `b`, and `c` created, then we add 10
to the 5 in `value` by dereferencing the `Rc<T>` and calling `borrow_mut` on
the `RefCell`.

When we print out `shared_list`, `b`, and `c`, we can see that they all have
the modified value of 15:

```text
shared_list after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

This is pretty neat! We didn't have to modify our definition of `List<T>` at
all, since the generic parameter lets us substitute any immutable type. By
using `RefCell<T>`, we satisfy the immutable type requirement since
`RefCell<T>` is outwardly immutable, but we can use the methods on `RefCell<T>`
that provide access to its interior mutability to be able to modify our data
when we need to. The runtime checks of the borrowing rules that `RefCell<T>`
does protect us from data races, and we've decided that we want to trade a bit
of speed for the flexibility in our data structures.

`RefCell<T>` is not the only standard library type that provides interior
mutability. `Cell<T>` is similar but instead of giving references to the inner
value like `RefCell<T>` does, the value is copied in and out of the `Cell<T>`.
`Mutex<T>` offers interior mutability that is safe to use across threads, and
we'll be discussing its use in the next chapter on concurrency. Check out the
standard library docs for more details on the differences between these types.

## Creating Reference Cycles and Leaking Memory is Safe

Rust makes a number of guarantees that we've talked about, for example that
we'll never have a null value, and data races will be disallowed at compile
time. Rust's memory safety guarantees make it more difficult to create memory
that never gets cleaned up, which is known as a *memory leak*. Rust does not
make memory leaks *impossible*, however, preventing memory leaks is *not* one
of Rust's guarantees. In other words, memory leaks are memory safe.

By using `Rc<T>` and `RefCell<T>`, it is possible to create cycles of
references where items refer to each other in a cycle. This is bad because the
reference count of each item in the cycle will never reach 0, and the values
will never be dropped. Let's take a look at how that might happen and how to
prevent it.

In Listing 15-13, we're going to continue building on the `List<T>` definition
from Listing 15-12. We've added a `RefCell` to the `Cons` variant's definition
so that we can modify a `Cons` instance after we've created it. We've also
added a `next_item` method to make it convenient for us to access the second
item, if we have a `Cons` variant:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn next_item(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

<figcaption>

Listing 15-13: A cons list definition that holds a `RefCell` so that we can
modify what a `Cons` variant is referring to

</figcaption>
</figure>

TODO: insert diagram showing the "shape" of the `Cons` variant?

Next, in Listing 15-14, we're going to create a `List<T>` value in the variable
`a` that initially is a list of `5, Nil`. Then we'll create a `List<T>` value
in the variable `b` that is a list of the value 10 and then points to the list
in `a`. Finally, we'll modify `a` so that it points to `b` instead of `Nil`,
which will then create a cycle:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
# #[derive(Debug)]
# enum List<T> {
#     Cons(T, RefCell<Rc<List<T>>>),
#     Nil,
# }
#
# impl<T> List<T> {
#     fn next_item(&self) -> Option<&RefCell<Rc<List<T>>>> {
#         match *self {
#             Cons(_, ref item) => Some(item),
#             Nil => None,
#         }
#     }
# }
#
use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next_item());

    let b = Rc::new(Cons(10, RefCell::new(a.clone())));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.next_item());

    if let Some(ref link) = a.next_item() {
        *link.borrow_mut() = b.clone();
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.next_item());
}
```

<figcaption>

Listing 15-14: Creating a reference cycle of two `List<T>` values pointing to
each other

</figcaption>
</figure>

We use the `next_item` method to get a reference to the `RefCell` in `a`, which
we put in the variable `link`. Then we use the `borrow_mut` method on the
`RefCell` to change the value inside from an `Rc` that holds a `Nil` value to
the `Rc` in `b`. We've created a reference cycle that looks like:

TODO: insert diagram showing b and a pointing to each other

If you uncomment the last `println!`, Rust will try and print this cycle out
with `a` pointing to `b` pointing to `a` and so forth until it overflows the
stack.

Looking at the results of the `println!` calls before the last one, we'll see
that the reference count of both `a` and `b` are 2 after we change `a` to point
to `b`. At the end of `main`, Rust will try and drop `b` first, which will
decrease the count of the `Rc` by one. However, because `a` is still
referencing that `Rc`, its count is 1 rather than 0, so the memory the `Rc` has
on the heap won't be dropped. It'll just sit there with a count of one,
forever. In this specific case, the program ends right away, so it's not a
problem, but in a more complex program that allocates lots of memory in a cycle
and holds onto it for a long time, this would be a problem. The program would
be using more memory than it needs to be, and might overwhelm the system and
cause it to run out of memory available to use.

Now, as you can see, creating reference cycles is difficult and inconvenient in
Rust. To be honest, your authors had to look up previous discussions of an
example to get this right. But it's not impossible: preventing memory leaks in
the form of reference cycles is not one of the guarantees Rust makes. If you
have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, be aware
that you'll have to ensure that you don't create cycles on your. One way of
doing that is using `Weak<T>`.

### Prevent Reference Cycles: Turn an `Rc<T>` into a `Weak<T>`

To help with this problem, the Rust standard library contains another smart
pointer type: `Weak<T>`. We've been showing how cloning an `Rc<T>` increases
the `strong_count` of references; `Weak<T>` is a way to reference an `Rc<T>`
that does not increment the `strong_count`: instead it increments the
`weak_count` of references to an `Rc`. When an `Rc` goes out of scope, the
inner value will get dropped if the `strong_count` is 0, even if the
`weak_count` is not 0. When we attempt to use a `Weak<T>` reference, we'll get
an `Option<T>` that will be `Some` if the `Rc` value has not been dropped yet,
and `None` if the `Rc` value has been dropped.

So in order to make it possible to create lists that point to each other but not create reference cycles, in Listing 15-15, we're going to change our definition of `List<T>` again to hold a `Weak<T>` instead of an `Rc<T>`.

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Weak<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn next_item(&self) -> Option<Rc<List<T>>> {
        match *self {
            Cons(_, ref n) => (&*n.borrow()).upgrade(),
            Nil => None,
        }
    }
}
```

<figcaption>

Listing 15-15: Modifying `List<T>` to have `Weak<T>` references instead of
`Rc<T>`

</figcaption>
</figure>

We've also modified the `next_item` method: not only does it return `None` when
`self` is `Nil` and doesn't have a `next_item`, it now also returns `None` if
the value that the `Weak<T>` references has been dropped. The `upgrade` method
on a `Weak<T>` value returns `Some` containing an `Rc` if the value has not yet
been dropped and `None` if the value has been dropped.

TODO: is this bad software design, collapsing two cases that return `None` like this???

To create `Weak<T>` values, we call the `Rc::downgrade` associated function,
which takes an `&Rc<T>` as an argument, and gives a `Weak<T>` back.

Listing 15-16 shows a `main` method where we're trying to create `a` and `b` lists that point to each other, similarly to what we did in Listing 15-14, but this time we won't have a reference cycle and the values will be dropped when they go out of scope at the end of `main`:

<figure>

```rust
# #[derive(Debug)]
# enum List<T> {
#     Cons(T, RefCell<Weak<List<T>>>),
#     Nil,
# }
#
# impl<T> List<T> {
#     fn next_item(&self) -> Option<Rc<List<T>>> {
#         match *self {
#             Cons(_, ref n) => (&*n.borrow()).upgrade(),
#             Nil => None,
#         }
#     }
# }
#
use List::{Cons, Nil};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
    let nil = Rc::new(Nil);

    let a = Rc::new(Cons(5, RefCell::new(Rc::downgrade(&nil))));

    println!("a.next_item() = {:?}", a.next_item());

    {
        let b = Rc::new(Cons(10, RefCell::new(Rc::downgrade(&a.clone()))));

        if let Cons(_, ref link) = *a {
            *link.borrow_mut() = Rc::downgrade(&b.clone());
        }

        println!("a.next_item() = {:?}", a.next_item());
        println!("b.next_item() = {:?}", b.next_item());
    }

    println!("a.next_item() = {:?}", a.next_item());
}
```

<figcaption>

Listing 15-16: Creating `List<T>` values using weak references

</figcaption>
</figure>

First, we create a variable for the `Rc<T>` that holds the `Nil` value, so that
it's clearer to see that we call `Rc::downgrade` and pass a reference to `nil`
when we create `a`. At that point, we print out the value of `a.next_item()`
and we can see that it's `Some(Nil)`.

We've added an inner scope in order to demonstrate what happens when `b` goes
out of scope. In the inner scope, we create `b` that has a weak reference to
`a`, and then we modify `a` to have a weak reference to `b` instead of to
`nil`. At the end of the inner scope, we can see that `a` and `b` are pointing
to each other, but through weak references.

At the end of the inner scope, `b` goes out of scope. The value that the
`Rc<T>` in `b` holds gets dropped, even though `a` still references it, because
the reference in `a` is a weak reference that doesn't count when Rust decides
whether the value in `b` should be dropped or not. At the end of `main` when we
print out `a.next_item()` again, we can see that we now get a `None` value
since the value that the weak reference in `a` was pointing to has been
dropped. Success! We've broken the cycle.

Add some `println!` statements at various places that display the values that
`Rc::strong_count` and `Rc::weak_count` return for `a` and `b` at different
points in this example to see which parts of the code increase or decrease the
strong or weak counts.

If you're doing complex things with `Rc<T>`s, you should investigate if it's
possible for you to have a cycle, and insert a `Weak<T>` so that you don't
create cycles and leak memory. The specifics depend on exactly what you're
doing, but luckily, this isn't a Rust-specific idea; all of this translates
over to other reference counting libraries in other languages, so doing some
reading about it should help you.

## Summary

We've now covered how you can use different kinds of smart pointers to choose
different guarantees and tradeoffs than those Rust makes with regular
references. `Box<T>` has a known size and points to data allocated on the heap.
`Rc<T>` keeps track of the number of references to data on the heap so that
data can have multiple owners. `RefCell<T>` with its interior mutability gives
us a type that can be used where we need an immutable type, and enforces the
borrowing rules at runtime instead of at compile time.

We've also discussed the `Deref` and `Drop` traits that enable a lot of smart
pointers' functionality. We explored how it's possible to create a reference
cycle that would cause a memory leak, and how to prevent reference cycles by
using `Weak<T>`.

If this chapter has piqued your interest and you now want to implement your own
smart pointers, check out [The Nomicon] for even more useful information.

[The Nomicon]: https://doc.rust-lang.org/stable/nomicon/vec.html

Next, let's talk about concurrency in Rust. We'll even learn about a few new
smart pointers that can help us with it.
