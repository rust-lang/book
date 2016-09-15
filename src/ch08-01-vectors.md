# Vectors

The first type we'll look at is `Vec<T>`, also known as a *vector*. Vectors
allow you to store more than one value in a single data structure that puts all
the values next to each other in memory.

## Creating a new vector

To create a new vector, you can call the `new` function:

```rust
let v: Vec<i32> = Vec::new();
```

You'll note that we added a type annotation here. Because we don't actually do
anything with the vector, Rust doesn't know what kind of elements we intend to
store. This is an important point. Vectors are homogenous: they may store many
values, but those values must all be the same type. Vectors are generic over
the type you store inside them (we'll talk about Generics more throroughly in
Chapter XX), and the angle brackets here tell Rust that this vector will hold
elements of the `i32` type.

That said, in real code, you very rarely need to do this type annotation since
Rust can infer the type of value we want to store once we insert values. Let's
see this in action. To put elements in the vector, we can use the `push` method:

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

## Destroying a vector

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

## Reading elements of vectors

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
elements in a vector depends on whether you consider an attempted access past
the end of the vector to be an error, in which case you would want the `panic!`
behavior, or whether this will happen occasionally under normal circumstances
and your code will have logic to handle getting `Some(&element)` or `None`.

Once we have a valid reference, the borrow checker will enforce the ownership
and borrowing rules we covered in Chapter 4 in order to ensure this and other
references to the contents of the vector stay valid. For example, here's code
that looks like it should be allowed, but it won't compile because the
references actually aren't valid anymore:

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

Be sure to take a look at the API documentation for all the methods defined on
`Vec` by the standard library. For example, in addition to `push` there's a
`pop` method that will remove and return the last element. Let's move on to the
next collection type: `String`!
