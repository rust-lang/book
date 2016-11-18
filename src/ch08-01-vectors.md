## Vectors

The first type we'll look at is `Vec<T>`, also known as a *vector*. Vectors
allow us to store more than one value in a single data structure that puts all
the values next to each other in memory. Vectors can only stored values of one
type.

<!-- Maybe give an exmaple of what a vector is useful for -->

### Creating a New Vector

To create a new, empty vector, we can call the `new` function with the `Vec`
command:

```rust
let v: Vec<i32> = Vec::new();
```
<!-- I'm not sure at this point what we mean by "since we don't actually do
anything with the vector", just that we haven't put values in it yet? I've
edited as such, can you please check? -->

Note that we added a type annotation here. Since we aren't filling this vector
with any values, Rust doesn't know what kind of elements we intend to store.
This is an important point. Vectors are homogenous: they may store many values,
but those values must all be the same type. Vectors are generic over

<!-- If we don't expect the reader to know anything about Rust generics, can we
delete the reference to it here?-->

the type stored inside them (we'll talk about Generics more throroughly in
Chapter 10), and the angle brackets here tell Rust that this vector will hold
elements of the `i32` type.

In real code, Rust can infer the type of value we want to store once we insert
values, so you rarely need to do this type annotation since. Let's look at how
to modify a vector next.

<!-- So is this an unusual way to create a vector, would we usually create and
fill a vector at the same time, or would we do it like this where we create it
then push the values? If the former, I think we should consider doing so here
or at least mentioning that, to show them how they'll usually do it -->

### Updating a Vector

<!-- So here we're making and filling it but this still isn't the usual way
they'll do, is that right? That's fine, but let's make it clear -->

To create a vector and put elements inside it, we can use the `push` method:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```
<!-- Can you mention why it's mut, is this right? Do all vectors need to be mut
then? -->

First we need to make the vector mutable so we can alter its entries. The
numbers we place inside are all `i32`s, and Rust infers this from the data, so
we don't need the `<i32>` annotation.

<!-- Ah! So this is the normal way to create a vector! Can you make sure it's
clear to the reader why we're looking at these ones first if this next one is
the usual method? -->

Creating a vector with some initial values like this is very common, so there's
actually a macro to do it for us. With the macro, the last program would look
like this:

```rust
let v = vec![5, 6, 7, 8];
```
<!-- Is this mutable?-->

This macro does a similar thing to our previous example, but it's much more
convenient both to write and to read.

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

Now that you know how to create and destroy vectors, knowing how to read their
contents is a good next step. There are two ways to reference a value stored in
a vector. In the examples, we've annotated the types of the values that are
returned from these functions for extra clarity.

This example shows both methods of accessing a value in a vector:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

There are a few things to note here. Firstly, that we use the index value of
`2` to get the third element: vectors are indexed by number, starting at zero.
Secondly, the two different ways to get the third element are: using `&` and
`[]`s, which gives us a reference, or using the `get` method with the index
passed as an argument, which gives us an `Option<&T>`.

The reason Rust has two ways to reference an element is so that you can choose
how the program behaves when you try to use an index value that the vector
doesn't have an element for, for example:

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

When you run this, you should find that with the first `[]` method, Rust will
cause a `panic!` when a non-existent element is referenced. This method would
be preferable if you want your program to consider an attempt to access an
element past the end of the vector to be an error.

With the `get` method, it will instead return `None` without `panic!`ing. You
would use this if accessing an element beyond the range of the vector will
happen occasionally under normal circumstances and your code will have logic to
handle getting `Some(&element)` or `None`.

<!-- I'm not clear what you mean by "your code will have logic to handle
getting `Some(&element)` or `None`" -- just that it will have a way to handle
an empty element? -->

#### Invalid References

Once the program has a valid reference, the borrow checker will enforce the
ownership and borrowing rules covered in Chapter 4 to ensure this reference and
any other references to the contents of the vector stay valid. This means that
in a function that owns a `Vec`, we can't return a reference to an element
since the `Vec` will be cleaned up at the end of the function. Try it out with
the following:

<!-- Hm, if we're referencing the vector element within the function that owns
the vector, wouldn't the vector still be in scope until the end of the
function, so the vector won't have been cleaned up? I don't think I'm following
this section, could you take a look here, see if it can be made clearer? -->

```rust,ignore
fn element() -> String {
    let list = vec![String::from("hi"), String::from("bye")];
    list[1]
}
```

Trying to compile this will result in the following error:

```bash
error: cannot move out of indexed content [--explain E0507]
  |>
4 |>     list[1]
  |>     ^^^^^^^ cannot move out of indexed content
```

Since `list` goes out of scope and gets cleaned up at the end of the function,
the reference `list[1]` cannot be returned because it would outlive `list`.

<!-- I think this is where I'm getting confused, I can't see where list goes
out of scope before it is referenced? -->

Here's another example of code that looks like it should be allowed, but won't
compile because the references aren't valid:

```rust,ignore
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

Compiling this will give us this error:

```bash
error: cannot borrow `v` as mutable because it is also borrowed as immutable
[--explain E0502]
  |>
5 |> let first = &v[0];
  |>              - immutable borrow occurs here
7 |> v.push(6);
  |> ^ mutable borrow occurs here
9 |> }
  |> - immutable borrow ends here
```

This violates one of the ownership rules we covered in Chapter 4: the `push`
method needs to have a mutable borrow to the `Vec`, and Rust doesn't allow any
immutable borrows while we have a mutable borrow.

The reason behind the fact that referencing the first element in a vector while
trying to add a new item to the end fails is due to the way vectors work:
adding a new element onto the end might require allocating new memory and
copying the old elements over to the new space, in the circumstance that there
isn't enough room to put all the elements next to each other where the vector
was. In that case, our reference would be pointing to deallocated memory. This
protects us from that.

> Note: For more on this, see
> The Nomicon at *https://doc.rust-lang.org/stable/nomicon/vec.html*.

### Using an Enum to Store Multiple Types

<!-- Let's put vectors together with what we learned about enums in Chapter 6.
-->

At the beginning of this chapter, we said that vectors can only store values
that are all the same type. This can be inconvenient; there are definitely use
cases for needing to store a list of things of different types. Luckily, the
variants of an enum are all umbrellaed under the same enum type, so when we
need to store elements of a different type in a vector this scenario, we can
define and use an enum!

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

The advantage to this is that we can be explicit about what types are allowed
in this vector. If Rust allowed a vector to hold any type, there would be a
chance that one or more of the types would cause errors with the operations
performed on the vector. Using an enum plus a `match` means that Rust will
ensure at compile time that we always handle every possible case.

<!-- Can you briefly explain what the match is doing here, as a recap? How does
it mean we always handle every possible case? I'm not sure it's totally clear.
-->

The caveat when using an enum for storing different types in a is that that you
need to know at compile time the set of types you'll want to store. If that's
not the case, instead of an enum, you can use a trait object. You'll learn
about those in Chapter XX.

Now that we've gone over some of the most common ways to use vectors, be sure
to take a look at the API documentation for other useful methods defined on
`Vec` by the standard library. For example, in addition to `push` there's a
`pop` method that will remove and return the last element. Let's move on to the
next collection type: `String`!

<!-- Do you mean the Rust online documentation here? Are you not including it
in the book for space reasons? We might want to justify sending them out of the
book if we don't want to cover it here -->
