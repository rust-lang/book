## Using `Box<T>` to Point to Data on the Heap

The most straightforward smart pointer is a box, whose type is written
`Box<T>`. _Boxes_ allow you to store data on the heap rather than the stack.
What remains on the stack is the pointer to the heap data. Refer to Chapter 4
to review the difference between the stack and the heap.

Boxes don’t have performance overhead, other than storing their data on the
heap instead of on the stack. But they don’t have many extra capabilities
either. You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want
  to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but
  ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that
  implements a particular trait rather than being of a specific type

We’ll demonstrate the first situation in [“Enabling Recursive Types with
Boxes”](#enabling-recursive-types-with-boxes)<!-- ignore -->. In the second
case, transferring ownership of a large amount of data can take a long time
because the data is copied around on the stack. To improve performance in this
situation, we can store the large amount of data on the heap in a box. Then,
only the small amount of pointer data is copied around on the stack, while the
data it references stays in one place on the heap. The third case is known as a
_trait object_, and [“Using Trait Objects That Allow for Values of Different
Types,”][trait-objects]<!-- ignore --> in Chapter 18 is devoted to that topic.
So what you learn here you’ll apply again in that section!

### Using `Box<T>` to Store Data on the Heap

Before we discuss the heap storage use case for `Box<T>`, we’ll cover the
syntax and how to interact with values stored within a `Box<T>`.

Listing 15-1 shows how to use a box to store an `i32` value on the heap.

<Listing number="15-1" file-name="src/main.rs" caption="Storing an `i32` value on the heap using a box">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

We define the variable `b` to have the value of a `Box` that points to the
value `5`, which is allocated on the heap. This program will print `b = 5`; in
this case, we can access the data in the box similarly to how we would if this
data were on the stack. Just like any owned value, when a box goes out of
scope, as `b` does at the end of `main`, it will be deallocated. The
deallocation happens both for the box (stored on the stack) and the data it
points to (stored on the heap).

Putting a single value on the heap isn’t very useful, so you won’t use boxes by
themselves in this way very often. Having values like a single `i32` on the
stack, where they’re stored by default, is more appropriate in the majority of
situations. Let’s look at a case where boxes allow us to define types that we
wouldn’t be allowed to define if we didn’t have boxes.

### Enabling Recursive Types with Boxes

A value of a _recursive type_ can have another value of the same type as part of
itself. Recursive types pose an issue because Rust needs to know at compile time
how much space a type takes up. However, the nesting of values of recursive
types could theoretically continue infinitely, so Rust can’t know how much space
the value needs. Because boxes have a known size, we can enable recursive types
by inserting a box in the recursive type definition.

As an example of a recursive type, let’s explore the _cons list_. This is a data
type commonly found in functional programming languages. The cons list type
we’ll define is straightforward except for the recursion; therefore, the
concepts in the example we’ll work with will be useful any time you get into
more complex situations involving recursive types.

#### More Information About the Cons List

A _cons list_ is a data structure that comes from the Lisp programming language
and its dialects, is made up of nested pairs, and is the Lisp version of a
linked list. Its name comes from the `cons` function (short for _construct
function_) in Lisp that constructs a new pair from its two arguments. By
calling `cons` on a pair consisting of a value and another pair, we can
construct cons lists made up of recursive pairs.

For example, here’s a pseudocode representation of a cons list containing the
list `1, 2, 3` with each pair in parentheses:

```text
(1, (2, (3, Nil)))
```

Each item in a cons list contains two elements: the value of the current item
and the next item. The last item in the list contains only a value called `Nil`
without a next item. A cons list is produced by recursively calling the `cons`
function. The canonical name to denote the base case of the recursion is `Nil`.
Note that this is not the same as the “null” or “nil” concept discussed in
Chapter 6, which is an invalid or absent value.

The cons list isn’t a commonly used data structure in Rust. Most of the time
when you have a list of items in Rust, `Vec<T>` is a better choice to use.
Other, more complex recursive data types _are_ useful in various situations,
but by starting with the cons list in this chapter, we can explore how boxes
let us define a recursive data type without much distraction.

Listing 15-2 contains an enum definition for a cons list. Note that this code
won’t compile yet because the `List` type doesn’t have a known size, which
we’ll demonstrate.

<Listing number="15-2" file-name="src/main.rs" caption="The first attempt at defining an enum to represent a cons list data structure of `i32` values">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> Note: We’re implementing a cons list that holds only `i32` values for the
> purposes of this example. We could have implemented it using generics, as we
> discussed in Chapter 10, to define a cons list type that could store values of
> any type.

Using the `List` type to store the list `1, 2, 3` would look like the code in
Listing 15-3.

<Listing number="15-3" file-name="src/main.rs" caption="Using the `List` enum to store the list `1, 2, 3`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

The first `Cons` value holds `1` and another `List` value. This `List` value is
another `Cons` value that holds `2` and another `List` value. This `List` value
is one more `Cons` value that holds `3` and a `List` value, which is finally
`Nil`, the non-recursive variant that signals the end of the list.

If we try to compile the code in Listing 15-3, we get the error shown in
Listing 15-4.

<Listing number="15-4" caption="The error we get when attempting to define a recursive enum">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

The error shows this type “has infinite size.” The reason is that we’ve defined
`List` with a variant that is recursive: it holds another value of itself
directly. As a result, Rust can’t figure out how much space it needs to store a
`List` value. Let’s break down why we get this error. First we’ll look at how
Rust decides how much space it needs to store a value of a non-recursive type.

#### Computing the Size of a Non-Recursive Type

Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

To determine how much space to allocate for a `Message` value, Rust goes
through each of the variants to see which variant needs the most space. Rust
sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough
space to store two `i32` values, and so forth. Because only one variant will be
used, the most space a `Message` value will need is the space it would take to
store the largest of its variants.

Contrast this with what happens when Rust tries to determine how much space a
recursive type like the `List` enum in Listing 15-2 needs. The compiler starts
by looking at the `Cons` variant, which holds a value of type `i32` and a value
of type `List`. Therefore, `Cons` needs an amount of space equal to the size of
an `i32` plus the size of a `List`. To figure out how much memory the `List`
type needs, the compiler looks at the variants, starting with the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, and this process continues infinitely, as shown in Figure 15-1.

<img alt="An infinite Cons list: a rectangle labeled 'Cons' split into two smaller rectangles. The first smaller rectangle holds the label 'i32', and the second smaller rectangle holds the label 'Cons' and a smaller version of the outer 'Cons' rectangle. The 'Cons' rectangles continue to hold smaller and smaller versions of themselves until the smallest comfortably-sized rectangle holds an infinity symbol, indicating that this repetition goes on forever" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-1: An infinite `List` consisting of infinite
`Cons` variants</span>

#### Using `Box<T>` to Get a Recursive Type with a Known Size

Because Rust can’t figure out how much space to allocate for recursively
defined types, the compiler gives an error with this helpful suggestion:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

In this suggestion, _indirection_ means that instead of storing a value
directly, we should change the data structure to store the value indirectly by
storing a pointer to the value instead.

Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>`
needs: a pointer’s size doesn’t change based on the amount of data it’s
pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead
of another `List` value directly. The `Box<T>` will point to the next `List`
value that will be on the heap rather than inside the `Cons` variant.
Conceptually, we still have a list, created with lists holding other lists, but
this implementation is now more like placing the items next to one another
rather than inside one another.

We can change the definition of the `List` enum in Listing 15-2 and the usage
of the `List` in Listing 15-3 to the code in Listing 15-5, which will compile.

<Listing number="15-5" file-name="src/main.rs" caption="Definition of `List` that uses `Box<T>` in order to have a known size">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

The `Cons` variant needs the size of an `i32` plus the space to store the
box’s pointer data. The `Nil` variant stores no values, so it needs less space
on the stack than the `Cons` variant. We now know that any `List` value will
take up the size of an `i32` plus the size of a box’s pointer data. By using a
box, we’ve broken the infinite, recursive chain, so the compiler can figure out
the size it needs to store a `List` value. Figure 15-2 shows what the `Cons`
variant looks like now.

<img alt="A rectangle labeled 'Cons' split into two smaller rectangles. The first smaller rectangle holds the label 'i32', and the second smaller rectangle holds the label 'Box' with one inner rectangle that contains the label 'usize', representing the finite size of the box's pointer" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figure 15-2: A `List` that is not infinitely sized
because `Cons` holds a `Box`</span>

Boxes provide only the indirection and heap allocation; they don’t have any
other special capabilities, like those we’ll see with the other smart pointer
types. They also don’t have the performance overhead that these special
capabilities incur, so they can be useful in cases like the cons list where the
indirection is the only feature we need. We’ll look at more use cases for boxes
in Chapter 18.

The `Box<T>` type is a smart pointer because it implements the `Deref` trait,
which allows `Box<T>` values to be treated like references. When a `Box<T>`
value goes out of scope, the heap data that the box is pointing to is cleaned
up as well because of the `Drop` trait implementation. These two traits will be
even more important to the functionality provided by the other smart pointer
types we’ll discuss in the rest of this chapter. Let’s explore these two traits
in more detail.

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
