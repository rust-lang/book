## Vectors

The first type we'll look at is `Vec<T>`, also known as a *vector*. Vectors
allow us to store more than one value in a single data structure that puts all
the values next to each other in memory.

### Creating a New Vector

To create a new vector, we can call the `new` function:

```rust
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here. Since we don't actually do
anything with the vector, Rust doesn't know what kind of elements we intend to
store. This is an important point. Vectors are homogenous: they may store many
values, but those values must all be the same type. Vectors are generic over
the type stored inside them (we'll talk about Generics more throroughly in
Chapter 10), and the angle brackets here tell Rust that this vector will hold
elements of the `i32` type.

That said, in real code, we very rarely need to do this type annotation since
Rust can infer the type of value we want to store once we insert values. Let's
look at how to modify a vector next.

### Updating a Vector

To put elements in the vector, we can use the `push` method:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Since these numbers are `i32`s, Rust infers the type of data we want to store
in the vector, so we don't need the `<i32>` annotation.

We can improve this code even further. Creating a vector with some initial
values like this is very common, so there's a macro to do it for us:

```rust
let v = vec![5, 6, 7, 8];
```

This macro does a similar thing to our previous example, but it's much more
convenient.

### Dropping a Vector Drops its Elements

Like any other `struct`, a vector will be freed when it goes out of scope:

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```

When the vector gets dropped, it will also drop all of its contents, so those
integers are going to be cleaned up as well. This may seem like a
straightforward point, but can get a little more complicated once we start to
introduce references to the elements of the vector. Let's tackle that next!

### Reading Elements of Vectors

Now that we know how creating and destroying vectors works, knowing how to read
their contents is a good next step. There are two ways to reference a value
stored in a vector. In the following examples of these two ways, we've
annotated the types of the values that are returned from these functions for
extra clarity:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

First, note that we use the index value of `2` to get the third element:
vectors are indexed by number, starting at zero. Secondly, the two different
ways to get the third element are using `&` and `[]`s and using the `get`
method. The square brackets give us a reference, and `get` gives us an
`Option<&T>`. The reason we have two ways to reference an element is so that we
can choose the behavior we'd like to have if we try to use an index value that
the vector doesn't have an element for:

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

With the `[]`s, Rust will cause a `panic!`. With the `get` method, it will
instead return `None` without `panic!`ing. Deciding which way to access
elements in a vector depends on whether we consider an attempted access past
the end of the vector to be an error, in which case we'd want the `panic!`
behavior, or whether this will happen occasionally under normal circumstances
and our code will have logic to handle getting `Some(&element)` or `None`.

Once we have a valid reference, the borrow checker will enforce the ownership
and borrowing rules we covered in Chapter 4 in order to ensure this and other
references to the contents of the vector stay valid. This means in a function
that owns a `Vec`, we can't return a reference to an element since the `Vec`
will be cleaned up at the end of the function:

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

Here's another example of code that looks like it should be allowed, but it
won't compile because the references actually aren't valid anymore:

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
method needs to have a mutable borrow to the `Vec`, and we aren't allowed to
have any immutable borrows while we have a mutable borrow.

Why is it an error to have a reference to the first element in a vector while
we try to add a new item to the end, though? Due to the way vectors work,
adding a new element onto the end might require allocating new memory and
copying the old elements over to the new space if there wasn't enough room to
put all the elements next to each other where the vector was. If this happened,
our reference would be pointing to deallocated memory. For more on this, see
[The Nomicon](https://doc.rust-lang.org/stable/nomicon/vec.html).

### Using an Enum to Store Multiple Types

Let's put vectors together with what we learned about enums in Chapter 6. At
the beginning of this section, we said that vectors will only store values that
are all the same type. This can be inconvenient; there are definitely use cases
for needing to store a list of things that might be different types. Luckily,
the variants of an enum are all the same type as each other, so when we're in
this scenario, we can define and use an enum!

For example, let's say we're going to be getting values for a row in a
spreadsheet. Some of the columns contain integers, some floating point numbers,
and some strings. We can define an enum whose variants will hold the different
value types. All of the enum variants will then be the same type, that of the
enum. Then we can create a vector that, ultimately, holds different types:

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

This has the advantage of being explicit about what types are allowed in this
vector. If we allowed any type to be in a vector, there would be a chance that
the vector would hold a type that would cause errors with the operations we
performed on the vector. Using an enum plus a `match` where we access elements
in a vector like this means that Rust will ensure at compile time that we
always handle every possible case.

Using an enum for storing different types in a vector does imply that we need
to know the set of types we'll want to store at compile time. If that's not the
case, instead of an enum, we can use a trait object. We'll learn about those in
Chapter XX.

Now that we've gone over some of the most common ways to use vectors, be sure
to take a look at the API documentation for other useful methods defined on
`Vec` by the standard library. For example, in addition to `push` there's a
`pop` method that will remove and return the last element. Let's move on to the
next collection type: `String`!
