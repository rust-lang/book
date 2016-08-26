# Vectors

The first type we'll look at is `Vec<T>`, also known as a 'vector'. Vectors
allow you to store more than one value in a single data structure next to each
other in memory.

## Creating a new vector

To create a new vector, you can call the `new` function:

```rust
let v: Vec<i32> = Vec::new();
```

You'll note that we added a type annotation here. Because we don't actually do
anything with the vector, Rust doesn't know what kind of elements we intend to
store. This is an important point. Vectors are homogenous: they may store many
values, but those values must all be the same type. Vectors are generic over
the type you store inside them, and the angle brackets here tell Rust that
this vector will hold elements of the `i32` type.

That said, in real code, you very rarely need to do this type annotation. Let's
insert some values to see this in action. To put elements in the vector, we can
use the `push` method:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Since these numbers are `i32`s, Rust can infer the type of the vector, so we
don't need the annotation. That said, creating a vector with some initial
values like this is very common, and so there's a macro to do it for us:

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
integers are going to be taken care of as well. This may seem like a
straightforward point now, but can get a little more complicated once we start
to introduce references to the elements of the vector. Let's tackle that next!

## Reading elements of vectors

Now that we know how to make vectors, knowing how to read their contents is a
good next step. There are two ways to reference a value stored in a vector.
We've annotated the types of the values that are returned from these functions
for extra clarity:

```rust
let v = vec![1, 2, 3, 4, 5];

let three: &i32 = &v[2];
let three: Option<&i32> = v.get(2);
```

First, note that we use `2` to get the third element: vectors are indexed by
number, starting at zero. Secondly, we have two different ways to do this:
using `&` and `[]`s and using a method, `get()`. The square brackets give us a
reference, and `get()` gives us an `Option<&T>`. Why two ways? Well, what
happens if we tried to do this:

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

With the `[]`s, Rust will cause a `panic!`. With the `get` method, it will
instead return `None` without `panic!`ing. Remember that while `panic!`s will
cause your program to stop executing, they do not cause memory unsafety.

The borrow checker remains vigilant about references to the contents of the
vector and will make sure that everything stays valid. For example, here's
a case that looks okay, but actually isn't:

```rust,ignore
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

This will give us this error:

```text
error: cannot borrow `v` as mutable because it is also borrowed as immutable [E0502]
v.push(6);
^
note: immutable borrow occurs here
let first = &v[0];
             ^
note: immutable borrow ends here
}
^
error: aborting due to previous error(s)
```

Why is this an error? Due to the way vectors work, adding a new element onto
the end might require allocating new memory and copying the old elements over.
If this happened, our reference would be pointing to deallocated memory. For
more on this, see the Nomicon: https://doc.rust-lang.org/stable/nomicon/vec.html
