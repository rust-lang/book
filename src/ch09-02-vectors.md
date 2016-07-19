# Vectors

The first type we'll look at is `Vec<T>`, also known as a 'vector'. Vectors
allow you to store more than one value in a single data structure, next to each
other in memory.

## Creating a new vector

To create a new vector, you can call the `new` function:

```rust
let v: Vec<i32> = Vec::new();
```

You'll note that we added a type annotation here. Because we don't actually do
anything with the vector, Rust doesn't know what sort of elements we intend to
store. This is an important point. Vectors are homogenous; they may store many
values, but those values must be all of the same type. Vectors are generic over
the type you store inside them, so we use the angle brackets to tell Rust that
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

This macro does the exact same thing as our previous example, but it's much
more convenient.

How does this all work? Under the hood, vectors look approximately like this:

```rust,ignore
struct Vec<T> {
    data: &mut T,
    capacity: usize,
    length: usize,
}
```

This is not literally true, but will help you gain some intutions about it at
a high level. The actual representation is quite involved, and you can [read a
chapter in the Nomicon][nomicon] for the full details.

[nomicon]: https://doc.rust-lang.org/stable/nomicon/vec.html

At a high level, though, this is okay: a vector has a reference to some data,
a capacity, and a length. Let's go through these lines of code, and see how
the vector changes.

```rust
let mut v = Vec::new();
# 
# v.push(5);
# v.push(6);
# v.push(7);
# v.push(8);
```

We've created our new vector. It will look like this:

```text
Vec<i32> {
    data: <invalid>,
    capacity: 0,
    length: 0,
}
```

We don't have anything stored yet, so the vector hasn't made any room for any
elements. Since we don't have any elements, `data` isn't valid either.

```rust
# let mut v = Vec::new();
# 
v.push(5);
# v.push(6);
# v.push(7);
# v.push(8);
```

Next, we insert one value into the vector. `push` looks at the current capacity
and length, and sees that there's no room for this `5` to go. Since there's
currently room for zero elements, it will request enough memory from the
operating system for a single element, and then copy `5` into that memory.  It
then updates the internal details, and now they look like this:

```text
struct Vec<T> {
    data: <address of first element>,
    capacity: 1,
    length: 1,
}
```

Our `data` now points to the start of this memory, and `capacity` and `length`
are both set to one. If they're both set to the same value, what's the
difference? We will see that shortly. But first, let's insert another value
into the vector:


```rust
# let mut v = Vec::new();
# 
# v.push(5);
v.push(6);
# v.push(7);
# v.push(8);
```

Same thing, we `push` a `6`. And, in this case, the same thing will happen:
`push` looks at the values of `capacity` and `len`, and sees that we don't have
room for a second element. So here's what it does: it requests room for twice
as many elements as we currently have. In this case, that means two elements.
It then copies the existing element over into the new memory allocation, and
then copies the `6` into the next open slot. After updating the internal
values, it now looks like this:


```text
struct Vec<T> {
    data: <address of first element>,
    capacity: 2,
    length: 2,
}
```

Our `capacity` and `length` both show two here. Let's try inserting another
elment into the vector:

```rust
# let mut v = Vec::new();
# 
# v.push(5);
# v.push(6);
v.push(7);
# v.push(8);
```

Same story here: `push` looks and sees that our vector is full. However,
this time, something is slightly different. We currently have a capacity of
two elements. So the vector will request memory for double that number of
elements from the operating system: four. But why does it double every time?
We'll learn about that soon. For now, `push` will then copy the first two
elements over to the new memory, copy our `7` onto the end, and then update
the internal state. What's it look like now?

```text
struct Vec<T> {
    data: <address of first element>,
    capacity: 4,
    length: 3,
}
```

Ah ha! A difference. In essence, `capacity` tracks how much memory we've got
saved away for holding elements. `length` keeps track of how many elements we
are actually storing. Why not just allocate exactly how much we need? In order
to get more heap memory, we have to talk to the operating system, and that's
slower. Not only that, but we have to copy the contents of the vector from the
old memory to the new memory each time. So we make a tradeoff: we allocate a
bit more memory than we need, but in exchange, we get speed.

We can see this speed the next time we call `push`:

```rust
# let mut v = Vec::new();
# 
# v.push(5);
# v.push(6);
v.push(7);
# v.push(8);
```

Now, `push` looks at the capacity versus the length: we have room for four
elements, but we only have three elements. Perfect! We skip that "request more
memory from the OS and copy everything" business, and just copy the `7` into
the existing memory. After updating `capacity` and `len`, it looks like this:

```text
struct Vec<T> {
    data: <address of first element>,
    capacity: 4,
    length: 4,
}
```

We can do even better than this, though. While `new` allocates a new empty
vector, we can use `with_capacity` if we know how many things we're going to
insert:

```rust
let mut v = Vec::with_capacity(4);
# 
# v.push(5);
# v.push(6);
# v.push(7);
# v.push(8);
```

Here, our initial vector looks like this:

```text
struct Vec<T> {
    data: <invalid>,
    capacity: 4,
    length: 0,
}
```

Now, when we do our `push`es, we won't need to allocate until we `push` our
fifth element. The `vec!` macro uses a similar trick, so don't worry about it!

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
We've added in the return types for extra clarity:

```rust
let v = vec![1, 2, 3, 4, 5];

let three: &i32 = &v[2];
let three: Option<&i32> = v.get(2);
```

First, note that we use `2` to get the third element: vectors are indexed by
number, starting at zero. Secondly, we have two different ways to do this: `&`
and `[]`s, and a method, `get`. The square brackets give us a reference, and
`get` gives us an `Option<&T>`. Why two ways? Well, what happens if we tried to
do this:

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

With the `[]`s, Rust will cause a `panic!`. With the `get` method, it will
instead return `None`. But it's important to note that while `panic!`s will
cause your program to stop executing, they do not cause memory unsafety.

The borrow checker remains vigilant about references to the contents of the
vector, and will make sure that everything stays valid. For example, here's
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

What's the matter here? Remember what can happen when we `push` to a vector: it
might have to go get more memory, and copy all of the values to that new
memory. If it has to do this, our `first` would be pointing to old, invalid
memory!
