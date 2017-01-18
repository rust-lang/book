## Vectors

The first type we'll look at is `Vec<T>`, also known as a *vector*. Vectors
allow us to store more than one value in a single data structure that puts all
the values next to each other in memory. Vectors can only store values of the
same type. They are useful in situations where you have a list of items, such
as the lines of text in a file or the prices of items in a shopping cart.

### Creating a New Vector

To create a new, empty vector, we can call the `Vec::new` function:

```rust
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here. Since we aren't inserting any values
into this vector, Rust doesn't know what kind of elements we intend to store.
This is an important point. Vectors are homogeneous: they may store many values,
but those values must all be the same type. Vectors are implemented using
generics, which Chapter 10 will cover how to use in your own types. For now,
all you need to know is that the `Vec` type provided by the standard library
can hold any type, and when a specific `Vec` holds a specific type, the type
goes within angle brackets. We've told Rust that the `Vec` in `v` will hold
elements of the `i32` type.

In real code, Rust can infer the type of value we want to store once we insert
values, so you rarely need to do this type annotation. It's more common to
create a `Vec` that has initial values, and Rust provides the `vec!` macro for
convenience. The macro will create a new `Vec` that holds the values we give
it. This will create a new `Vec<i32>` that holds the values `1`, `2`, and `3`:

```rust
let v = vec![1, 2, 3];
```

Because we've given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn't necessary. Let's look at how to
modify a vector next.

### Updating a Vector

To create a vector then add elements to it, we can use the `push` method:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

As with any variable as we discussed in Chapter 3, if we want to be able to
change its value, we need to make it mutable with the `mut` keyword. The
numbers we place inside are all `i32`s, and Rust infers this from the data, so
we don't need the `Vec<i32>` annotation.

### Dropping a Vector Drops its Elements

Like any other `struct`, a vector will be freed when it goes out of scope:

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```

When the vector gets dropped, all of its contents will also be dropped, meaning
those integers it holds will be cleaned up. This may seem like a
straightforward point, but can get a little more complicated once we start to
introduce references to the elements of the vector. Let's tackle that next!

### Reading Elements of Vectors

Now that you know how to create, update, and destroy vectors, knowing how to
read their contents is a good next step. There are two ways to reference a
value stored in a vector. In the examples, we've annotated the types of the
values that are returned from these functions for extra clarity.

This example shows both methods of accessing a value in a vector either with
indexing syntax or the `get` method:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

There are a few things to note here. First, that we use the index value of `2`
to get the third element: vectors are indexed by number, starting at zero.
Second, the two different ways to get the third element are: using `&` and
`[]`s, which gives us a reference, or using the `get` method with the index
passed as an argument, which gives us an `Option<&T>`.

The reason Rust has two ways to reference an element is so that you can choose
how the program behaves when you try to use an index value that the vector
doesn't have an element for. As an example, what should a program do if it has
a vector that holds five elements then tries to access an element at index 100
like this:

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

When you run this, you will find that with the first `[]` method, Rust will
cause a `panic!` when a non-existent element is referenced. This method would
be preferable if you want your program to consider an attempt to access an
element past the end of the vector to be a fatal error that should crash the
program.

When the `get` method is passed an index that is outside the array, it will
return `None` without `panic!`ing. You would use this if accessing an element
beyond the range of the vector will happen occasionally under normal
circumstances. Your code can then have logic to handle having either
`Some(&element)` or `None`, as we discussed in Chapter 6. For example, the
index could be coming from a person entering a number. If they accidentally
enter a number that's too large and your program gets a `None` value, you could
tell the user how many items are in the current `Vec` and give them another
chance to enter a valid value. That would be more user-friendly than crashing
the program for a typo!

#### Invalid References

Once the program has a valid reference, the borrow checker will enforce the
ownership and borrowing rules covered in Chapter 4 to ensure this reference and
any other references to the contents of the vector stay valid. Recall the rule
that says we can't have mutable and immutable references in the same scope.
That rule applies in this example, where we hold an immutable reference to the
first element in a vector and try to add an element to the end:

```rust,ignore
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

Compiling this will give us this error:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  |
4 | let first = &v[0];
  |              - immutable borrow occurs here
5 |
6 | v.push(6);
  | ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

This code might look like it should work: why should a reference to the first
element care about what changes about the end of the vector? The reason why
this code isn't allowed is due to the way vectors work. Adding a new element
onto the end of the vector might require allocating new memory and copying the
old elements over to the new space, in the circumstance that there isn't enough
room to put all the elements next to each other where the vector was. In that
case, the reference to the first element would be pointing to deallocated
memory. The borrowing rules prevent programs from ending up in that situation.

> Note: For more on this, see [The Nomicon][nomicon].

[nomicon]: https://doc.rust-lang.org/stable/nomicon/vec.html

### Using an Enum to Store Multiple Types

At the beginning of this chapter, we said that vectors can only store values
that are all the same type. This can be inconvenient; there are definitely use
cases for needing to store a list of things of different types. Luckily, the
variants of an enum are all defined under the same enum type, so when we need to
store elements of a different type in a vector, we can define and use an enum!

For example, let's say we want to get values from a row in a spreadsheet, where
some of the columns in the row contain integers, some floating point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and then all of the enum variants will be considered the same
type, that of the enum. Then we can create a vector that holds that enum and
so, ultimately, holds different types:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

The reason Rust needs to know exactly what types will be in the vector at
compile time is so that it knows exactly how much memory on the heap will be
needed to store each element. A secondary advantage to this is that we can be
explicit about what types are allowed in this vector. If Rust allowed a vector
to hold any type, there would be a chance that one or more of the types would
cause errors with the operations performed on the elements of the vector. Using
an enum plus a `match` means that Rust will ensure at compile time that we
always handle every possible case, as we discussed in Chapter 6.

<!-- Can you briefly explain what the match is doing here, as a recap? How does
it mean we always handle every possible case? I'm not sure it's totally clear.
-->
<!-- Because this is a focus of chapter 6 rather than this chapter's focus, we
don't think we should repeat it here as well, but we added a reference. /Carol
-->

If you don't know at the time that you're writing a program the exhaustive set
of types the program will get at runtime to store in a vector, the enum
technique won't work. Instead, you can use a trait object, which we'll cover in
Chapter 13.

Now that we've gone over some of the most common ways to use vectors, be sure
to take a look at the API documentation for all of the many useful methods
defined on `Vec` by the standard library. For example, in addition to `push`
there's a `pop` method that will remove and return the last element. Let's move
on to the next collection type: `String`!

<!-- Do you mean the Rust online documentation here? Are you not including it
in the book for space reasons? We might want to justify sending them out of the
book if we don't want to cover it here -->

<!-- Yes, there are many, many methods on Vec: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
Also there are occasionally new methods available with new versions of the
language, so there's no way we can be comprehensive here. We want the reader to
use the API documentation in these situations since the purpose of the online
docs is to be comprehensive and up to date. I personally wouldn't expect a book
like this to duplicate the info that's in the API docs, so I don't think a
justification is necessary here. /Carol  -->
