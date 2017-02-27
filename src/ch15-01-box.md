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
