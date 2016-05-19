# Generics

We've been working with a `Point` struct that looks like this:

```rust
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64,
}
```

But what if we didn't want to always use an `f64` here? What about an `f32` for
when we need less precision? Or an `i32` if we only want integer coordinates?

## Generic data types

Rust has a feature that lets us define a structure like this. 'Generics' let us
write code that allows for several different types, while letting us have one
definition. A more generic `Point` would look like this:

```rust
#[derive(Debug,Copy,Clone)]
struct Point<T> {
    x: T,
    y: T,
}
```

There are two changes here, and they both involve this new `T`. The first change
is in the definition:

```rust
# #[derive(Debug,Copy,Clone)]
struct Point<T> {
#     x: T,
#     y: T,
# }
```

Our previous definition said, "We are defining a struct named Point." This
definition says something slightly different: "We are defining a struct named
Point with one type parameter `T`."

Let's talk about this term 'type parameter'. We've already seen one other thing
called a 'parameter' in Rust: function parameters:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Here, `x` is a parameter to this function. We can call this function with a
different value, and `x` will change each time it's called:

```rust
# fn plus_one(x: i32) -> i32 {
#     x + 1
# }
let six = plus_one(5);
let eleven = plus_one(10);
```

In the same way, a type parameter allows us to define a data type which can be
different each time we use it:

```rust
#[derive(Debug,Copy,Clone)]
struct Point<T> {
    x: T,
    y: T,
}

let integral_point = Point { x: 5, y: 5 };
let floating_point = Point { x: 5.0, y: 5.0 };
```

Here, `integral_point` uses `i32` values for `T`, and `floating_point` uses
`f64` values. This also leads us to talk about the second change we made to `Point`:

```rust
# #[derive(Debug,Copy,Clone)]
# struct Point<T> {
    x: T,
    y: T,
# }
```

Instead of saying `x: i32`, we say `x: T`. This `T` is the same one that we
used above in the struct declaration. Because `x` and `y` both use `T`, they'll
be the same type. We could give them different types:

```rust
#[derive(Debug,Copy,Clone)]
struct Point<T, OtherT> {
    x: T,
    y: OtherT,
}

let different = Point { x: 5, y: 5.0 };
let same = Point { x: 5.0, y: 5.0 };
```

Here, instead of a single parameter, `T`, we have two: `T` and `OtherT`. Type
parameters have the same naming convention as other types: `CamelCase`.
However, you'll often see short, one-letter names used for types. `T` is very
common, because it's short for 'type', but you can name them something longer
if you'd like. In this version of `Point`, we say that `x` has the type `T`,
and `y` has the type `OtherT`. This lets us give them two different types, but
they don't have to be.

## Generic methods

We've seen how to define methods with the `impl` keyword. Our generic `Point`
can have generic methods, too:

```rust
#[derive(Debug,Copy,Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn some_method(&self) {
        // ...
    }
}
```

We also need the `<T>` after `impl`. This line reads, "We will be implementing
methods with one generic type parameter, `T`, for a type, `Point`, which takes
one generic type `T`." In a sense, the `impl<T>` says "we will be using a type
`T`" and the `Point<T>` says "that `T` is used for `Point`." In this simple
case, this syntax can feel a bit redundant, but when we get into some of Rust's
more advanced features later, this distinction will become more useful.

## Generic functions

Regular old functions can also take generic parameters, with a syntax that looks
very similar:

```rust
fn foo<T>(x: T) {
    // ...
}
```

This `foo()` function has one generic parameter, `T`, and takes one argument,
`x`, which has the type `T`. Let's talk a little bit more about what this means.

### Monomorphization

To give you an idea of how generics work, let's consider our `foo()` function,
calling it with three different types of argument:

```rust
fn foo<T>(x: T) {
    // ...
}

foo(5);
foo(5.0);
foo("bar");
```

How does this actually _work_, though? Rust uses a technique called
"monomorphization" to implement this kind of generics. This is a fancy word for
'copy and paste'. In other words, the compiler reads our code above, but generates
code that looks like this:

```rust
fn foo_i32(x: i32) {
    // ...
}

fn foo_f64(x: f64) {
    // ...
}

fn foo_str(x: &str) {
    // ...
}

foo_i32(5);
foo_f64(5.0);
foo_str("bar");
```

Rust generates three functions, and replaces the generic type with a specific
one, one per type we called the function with. It then replaces each call to
`foo()` with a call to the specific version.

There's a tradeoff here: we pay no speed overhead for using generics. It's the
same thing as calling a regular function. However, we pay for it in binary
size: there are now three copies of the function in our binary, so it's a
little bit bigger. We can make a different tradeoff instead, but that requires
a feature we haven't talked about yet, 'trait objects'. We'll get there!

## There's more to the story

This section covered the basic syntax of generics, but it's not the full story.
For example, let's try to implement our `foo()` function: we'll have it print out
the value of `x`:

```rust,ignore
fn foo<T>(x: T) {
    println!("x is: {}", x);
}
```

We'll get an error:

```text
error: the trait `core::fmt::Display` is not implemented for the type `T` [E0277]
println!("x is: {}", x);
                     ^
```

We can't print out `x`! The error messages reference something we talked about
breifly before, the `Display` trait. In order to implement this function, we
need to talk about traits. But we only need to talk about traits to implement
our own generic functions; we don't need this understanding to use them. So
rather than get into more details about this right now, let's talk about other
useful Rust data types, and we can come back to implementing generic functions
in the chapter about traits.

For now, the important bits to understand:

* Generic type parameters are kind of like function parameters, but for types
  instead of values.
* Type parameters go inside `<>`s and are usually named things like `T`.
* Rust generates multiple copies of generic functions so that generics are
  fast, but this comes at the cost of binary size. We will later learn how
  to make the opposite tradeoff.

With that, let's talk about another fundamental Rust data type: enums.
