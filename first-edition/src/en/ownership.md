# Ownership

This is the first of three sections presenting Rust’s ownership system. This is one of
Rust’s most distinct and compelling features, with which Rust developers should
become quite acquainted. Ownership is how Rust achieves its largest goal,
memory safety. There are a few distinct concepts, each with its own
chapter:

* ownership, which you’re reading now
* [borrowing][borrowing], and their associated feature ‘references’
* [lifetimes][lifetimes], an advanced concept of borrowing

These three chapters are related, and in order. You’ll need all three to fully
understand the ownership system.

[borrowing]: references-and-borrowing.html
[lifetimes]: lifetimes.html

# Meta

Before we get to the details, two important notes about the ownership system.

Rust has a focus on safety and speed. It accomplishes these goals through many
‘zero-cost abstractions’, which means that in Rust, abstractions cost as little
as possible in order to make them work. The ownership system is a prime example
of a zero-cost abstraction. All of the analysis we’ll talk about in this guide
is _done at compile time_. You do not pay any run-time cost for any of these
features.

However, this system does have a certain cost: learning curve. Many new users
to Rust experience something we like to call ‘fighting with the borrow
checker’, where the Rust compiler refuses to compile a program that the author
thinks is valid. This often happens because the programmer’s mental model of
how ownership should work doesn’t match the actual rules that Rust implements.
You probably will experience similar things at first. There is good news,
however: more experienced Rust developers report that once they work with the
rules of the ownership system for a period of time, they fight the borrow
checker less and less.

With that in mind, let’s learn about ownership.

# Ownership

[Variable bindings][bindings] have a property in Rust: they ‘have ownership’
of what they’re bound to. This means that when a binding goes out of scope,
Rust will free the bound resources. For example:

```rust
fn foo() {
    let v = vec![1, 2, 3];
}
```

When `v` comes into scope, a new [vector][vectors] is created on [the stack][stack],
and it allocates space on [the heap][heap] for its elements. When `v` goes out
of scope at the end of `foo()`, Rust will clean up everything related to the
vector, even the heap-allocated memory. This happens deterministically, at the
end of the scope.

We covered [vectors] in the previous chapter; we use them
here as an example of a type that allocates space on the heap at runtime. They
behave like [arrays], except their size may change by `push()`ing more
elements onto them.

Vectors have a [generic type][generics] `Vec<T>`, so in this example `v` will have type
`Vec<i32>`. We'll cover [generics] in detail in a later chapter.

[arrays]: primitive-types.html#arrays
[vectors]: vectors.html
[heap]: the-stack-and-the-heap.html#the-heap
[stack]: the-stack-and-the-heap.html#the-stack
[bindings]: variable-bindings.html
[generics]: generics.html

# Move semantics

There’s some more subtlety here, though: Rust ensures that there is _exactly
one_ binding to any given resource. For example, if we have a vector, we can
assign it to another binding:

```rust
let v = vec![1, 2, 3];

let v2 = v;
```

But, if we try to use `v` afterwards, we get an error:

```rust,ignore
let v = vec![1, 2, 3];

let v2 = v;

println!("v[0] is: {}", v[0]);
```

It looks like this:

```text
error: use of moved value: `v`
println!("v[0] is: {}", v[0]);
                        ^
```

A similar thing happens if we define a function which takes ownership, and
try to use something after we’ve passed it as an argument:

```rust,ignore
fn take(v: Vec<i32>) {
    // What happens here isn’t important.
}

let v = vec![1, 2, 3];

take(v);

println!("v[0] is: {}", v[0]);
```

Same error: ‘use of moved value’. When we transfer ownership to something else,
we say that we’ve ‘moved’ the thing we refer to. You don’t need some sort of
special annotation here, it’s the default thing that Rust does.

## The details

The reason that we cannot use a binding after we’ve moved it is subtle, but
important.

When we write code like this:

```rust
let x = 10;
```

Rust allocates memory for an integer [i32] on the [stack][sh], copies the bit
pattern representing the value of 10 to the allocated memory and binds the
variable name x to this memory region for future reference.

[i32]: primitive-types.html#numeric-types

Now consider the following code fragment:

```rust
let v = vec![1, 2, 3];

let mut v2 = v;
```

The first line allocates memory for the vector object `v` on the stack like
it does for `x` above. But in addition to that it also allocates some memory
on the [heap][sh] for the actual data (`[1, 2, 3]`). Rust copies the address
of this heap allocation to an internal pointer, which is part of the vector
object placed on the stack (let's call it the data pointer).

It is worth pointing out (even at the risk of stating the obvious) that the
vector object and its data live in separate memory regions instead of being a
single contiguous memory allocation (due to reasons we will not go into at
this point of time). These two parts of the vector (the one on the stack and
one on the heap) must agree with each other at all times with regards to
things like the length, capacity, etc.

When we move `v` to `v2`, Rust actually does a bitwise copy of the vector
object `v` into the stack allocation represented by `v2`. This shallow copy
does not create a copy of the heap allocation containing the actual data.
Which means that there would be two pointers to the contents of the vector
both pointing to the same memory allocation on the heap. It would violate
Rust’s safety guarantees by introducing a data race if one could access both
`v` and `v2` at the same time.

For example if we truncated the vector to just two elements through `v2`:

```rust
# let v = vec![1, 2, 3];
# let mut v2 = v;
v2.truncate(2);
```

and `v` were still accessible we'd end up with an invalid vector since `v`
would not know that the heap data has been truncated. Now, the part of the
vector `v` on the stack does not agree with the corresponding part on the
heap. `v` still thinks there are three elements in the vector and will
happily let us access the non existent element `v[2]` but as you might
already know this is a recipe for disaster. Especially because it might lead
to a segmentation fault or worse allow an unauthorized user to read from
memory to which they don't have access.

This is why Rust forbids using `v` after we’ve done the move.

[sh]: the-stack-and-the-heap.html

It’s also important to note that optimizations may remove the actual copy of
the bytes on the stack, depending on circumstances. So it may not be as
inefficient as it initially seems.

## `Copy` types

We’ve established that when ownership is transferred to another binding, you
cannot use the original binding. However, there’s a [trait][traits] that changes this
behavior, and it’s called `Copy`. We haven’t discussed traits yet, but for now,
you can think of them as an annotation to a particular type that adds extra
behavior. For example:

```rust
let v = 1;

let v2 = v;

println!("v is: {}", v);
```

In this case, `v` is an `i32`, which implements the `Copy` trait. This means
that, just like a move, when we assign `v` to `v2`, a copy of the data is made.
But, unlike a move, we can still use `v` afterward. This is because an `i32`
has no pointers to data somewhere else, copying it is a full copy.

All primitive types implement the `Copy` trait and their ownership is
therefore not moved like one would assume, following the ‘ownership rules’.
To give an example, the two following snippets of code only compile because the
`i32` and `bool` types implement the `Copy` trait.

```rust
fn main() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);
}

fn double(x: i32) -> i32 {
    x * 2
}
```

```rust
fn main() {
    let a = true;

    let _y = change_truth(a);
    println!("{}", a);
}

fn change_truth(x: bool) -> bool {
    !x
}
```

If we had used types that do not implement the `Copy` trait,
we would have gotten a compile error because we tried to use a moved value.

```text
error: use of moved value: `a`
println!("{}", a);
               ^
```

We will discuss how to make your own types `Copy` in the [traits][traits]
section.

[traits]: traits.html

# More than ownership

Of course, if we had to hand ownership back with every function we wrote:

```rust
fn foo(v: Vec<i32>) -> Vec<i32> {
    // Do stuff with `v`.

    // Hand back ownership.
    v
}
```

This would get very tedious. It gets worse the more things we want to take ownership of:

```rust
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // Do stuff with `v1` and `v2`.

    // Hand back ownership, and the result of our function.
    (v1, v2, 42)
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let (v1, v2, answer) = foo(v1, v2);
```

Ugh! The return type, return line, and calling the function gets way more
complicated.

Luckily, Rust offers a feature which helps us solve this problem.
It’s called borrowing and is the topic of the next section!

