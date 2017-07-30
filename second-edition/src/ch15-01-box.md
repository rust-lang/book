## `Box<T>` Points to Data on the Heap and Has a Known Size

The most straightforward smart pointer is a *box*, whose type is written
`Box<T>`. Boxes allow you to put a single value on the heap (we talked about
the stack vs. the heap in Chapter 4). Listing 15-1 shows how to use a box to
store an `i32` on the heap:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

<span class="caption">Listing 15-1: Storing an `i32` value on the heap using a
box</span>

This will print `b = 5`. In this case, we can access the data in the box in a
similar way as we would if this data was on the stack. Just like any value that
has ownership of data, when a box goes out of scope like `b` does at the end of
`main`, it will be deallocated. The deallocation happens for both the box
(stored on the stack) and the data it points to (stored on the heap).

Putting a single value on the heap isn’t very useful, so you won’t use boxes by
themselves in the way that Listing 15-1 does very often. A time when boxes are
useful is when you want to ensure that your type has a known size. For example,
consider Listing 15-2, which contains an enum definition for a *cons list*, a
type of data structure that comes from functional programming. Note that this
won’t compile quite yet:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
enum List {
    Cons(i32, List),
    Nil,
}
```

<span class="caption">Listing 15-2: The first attempt of defining an enum to
represent a cons list data structure of `i32` values</span>

We’re implementing a cons list that holds only `i32` values. We
could have also chosen to implement a cons list independent of the
type of value by using generics as discussed in Chapter 10.

> #### More Information About the Cons List
>
> A *cons list* is a data structure that comes from the Lisp programming
> language and its dialects. In Lisp, the `cons` function (short for “construct
> function”) constructs a new list from its two arguments, which usually are a
> single value and another list.
>
> The cons function concept has made its way into more general functional
> programming jargon; “to cons x onto y” informally means to construct a new
> container instance by putting the element x at the start of this new
> container, followed by the container y.
>
> A cons list is produced by recursively calling the `cons` function.
> The canonical name to denote the base case of the recursion is `Nil`, which
> announces the end of the list. Note that this is not the same as the “null”
> or “nil” concept from Chapter 6, which is an invalid or absent value.

A cons list is a list where each element contains both a single value as well
as the remains of the list at that point. The remains of the list are defined
by nested cons lists. The end of the list is signified by the value `Nil`. Cons
lists aren’t used very often in Rust; `Vec<T>` is usually a better choice.
Implementing this data structure is a good example of a situation where
`Box<T>` is useful, though. Let’s find out why!

Using a cons list to store the list `1, 2, 3` would look like this:

```rust,ignore
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

```text
error[E0072]: recursive type `List` has infinite size
 -->
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |     --------------- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

<span class="caption">Listing 15-3: The error we get when attempting to define
a recursive enum</span>

The error says this type ‘has infinite size’. Why is that? It’s because we’ve
defined `List` to have a variant that is recursive: it holds another value of
itself. This means Rust can’t figure out how much space it needs in order to
store a `List` value. Let’s break this down a bit: first let’s look at how Rust
decides how much space it needs to store a value of a non-recursive type.
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
like `List` in Listing 15-2. The compiler tries to figure out how much memory
is needed to store a value of the `List` enum, and starts by looking at the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, so `Cons` needs an amount of space equal to the size of an `i32` plus
the size of a `List`. To figure out how much memory a `List` needs, it looks at
its variants, starting with the `Cons` variant. The `Cons` variant holds a
value of type `i32` and a value of type `List`, and this continues infinitely,
as shown in Figure 15-4.

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-4: An infinite `List` consisting of infinite
`Cons` variants</span>

Rust can’t figure out how much space to allocate for recursively defined types,
so the compiler gives the error in Listing 15-3. The error did include this
helpful suggestion:

```text
= help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
        make `List` representable
```

Because a `Box<T>` is a pointer, we always know how much space it needs: a
pointer takes up a `usize` amount of space. The value of the `usize` will be
the address of the heap data. The heap data can be any size, but the address to
the start of that heap data will always fit in a `usize`. We can change our
definition from Listing 15-2 to look like the definition in Listing 15-5 by
changing `main` to use `Box::new` for the values inside the `Cons` variants:

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 15-5: Definition of `List` that uses `Box<T>` in
order to have a known size</span>

The compiler will now be able to figure out the size it needs to store a `List`
value. Rust will look at `List`, and again start by looking at the `Cons`
variant. The `Cons` variant will need the size of `i32` plus the space to store
a `usize`, since a box always has the size of a `usize`, no matter what it’s
pointing to. Then Rust looks at the `Nil` variant, which does not store a
value, so `Nil` doesn’t need any space. We’ve broken the infinite, recursive
chain by adding in a box. Figure 15-6 shows what the `Cons` variant looks like
now:

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figure 15-6: A `List` that is not infinitely sized since
`Cons` holds a `Box`</span>

This is the main area where boxes are useful: breaking up an infinite data
structure so that the compiler can know what size it is. We’ll look at another
case where Rust has data of unknown size in Chapter 17 when we discuss trait
objects.

Even though you won’t be using boxes very often, they are a good way to
understand the smart pointer pattern. Two of the aspects of `Box<T>` that are
commonly used with smart pointers are its implementations of the `Deref` trait
and the `Drop` trait. Let’s investigate how these traits work and how smart
pointers use them.
