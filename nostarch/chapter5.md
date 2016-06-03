# Structs

So far, all of the data types we’ve seen allow us to have a single value
at a time. `struct`s give us the ability to package up multiple values and
keep them in one related structure.

Let’s write a program which calculates the distance between two points.
We’ll start off with single variable bindings, and then refactor it to
use `struct`s instead.

Let’s make a new project with Cargo:

```bash
$ cargo new --bin points
$ cd points
```

Here’s a short program which calculates the distance between two points. Put
it into your `src/main.rs`:

```rust
fn main() {
    let x1 = 0.0;
    let y1 = 5.0;

    let x2 = 12.0;
    let y2 = 0.0;

    let answer = distance(x1, y1, x2, y2);

    println!("Point 1: ({}, {})", x1, y1);
    println!("Point 2: ({}, {})", x2, y2);
    println!("Distance: {}", answer);
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let x_squared = f64::powi(x2 - x1, 2);
    let y_squared = f64::powi(y2 - y1, 2);

    f64::sqrt(x_squared + y_squared)
}
```

Let's try running this program with `cargo run`:

```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
     Running `target/debug/points`
Point 1: (0, 5)
Point 2: (12, 0)
Distance: 13
```

Let's take a quick look at `distance()` before we move forward:

```rust
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let x_squared = f64::powi(x2 - x1, 2);
    let y_squared = f64::powi(y2 - y1, 2);

    f64::sqrt(x_squared + y_squared)
}
```

To find the distance between two points, we can use the Pythagorean Theorem.
The theorem is named after Pythagoras, who was the first person to mathematically
prove this formula. The details aren't that important, to be honest. There's a few
things that we haven't discussed yet, though.

```rust,ignore
f64::powi(2.0, 3)
```

The double colon (`::`) here is a namespace operator. We haven’t talked about
modules yet, but you can think of the `powi()` function as being scoped inside
of another name. In this case, the name is `f64`, the same as the type. The
`powi()` function takes two arguments: the first is a number, and the second is
the power that it raises that number to. In this case, the second number is an
integer, hence the ‘i’ in its name. Similarly, `sqrt()` is a function under the
`f64` module, which takes the square root of its argument.

## Why `struct`s?

Our little program is okay, but we can do better. The key is in the signature
of `distance()`:

```rust,ignore
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
```

The distance function is supposed to calculate the distance between two points.
But our distance function calculates some distance between four numbers. The
first two and last two arguments are related, but that’s not expressed anywhere
in our program itself. We need a way to group `(x1, y1)` and `(x2, y2)`
together.

We’ve already discussed one way to do that: tuples. Here’s a version of our program
which uses tuples:

```rust
fn main() {
    let p1 = (0.0, 5.0);

    let p2 = (12.0, 0.0);

    let answer = distance(p1, p2);

    println!("Point 1: {:?}", p1);
    println!("Point 2: {:?}", p2);
    println!("Distance: {}", answer);
}

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let x_squared = f64::powi(p2.0 - p1.0, 2);
    let y_squared = f64::powi(p2.1 - p1.1, 2);

    f64::sqrt(x_squared + y_squared)
}
```

This is a little better, for sure. Tuples let us add a little bit of structure.
We’re now passing two arguments, so that’s more clear. But it’s also worse.
Tuples don’t give names to their elements, and so our calculation has gotten
much more confusing:

```rust,ignore
p2.0 - p1.0
p2.1 - p1.1
```

When writing this example, your authors almost got it wrong themselves! Distance
is all about `x` and `y` points, but now it’s all about `0` and `1`. This isn’t
great.

Enter `struct`s. We can transform our tuples into something with a name:

```rust,ignore
let p1 = (0.0, 5.0);

struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 0.0, y: 5.0 };
```

Here’s what declaring a `struct` looks like:

```text
struct NAME {
    NAME: TYPE,
}
```

The `NAME: TYPE` bit is called a ‘field’, and we can have as many or as few of
them as you’d like. If you have none of them, drop the `{}`s:

```rust
struct Foo;
```

`struct`s with no fields are called ‘unit structs’, and are used in certain
advanced situations. We will just ignore them for now.

You can access the field of a struct in the same way you access an element of
a tuple, except you use its name:

```rust,ignore
let p1 = (0.0, 5.0);
let x = p1.0;

struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 0.0, y: 5.0 };
let x = p1.x;
```

Let’s convert our program to use our `Point` `struct`. Here’s what it looks
like now:

```rust
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 0.0, y: 5.0};

    let p2 = Point { x: 12.0, y: 0.0};

    let answer = distance(p1, p2);

    println!("Point 1: {:?}", p1);
    println!("Point 2: {:?}", p2);
    println!("Distance: {}", answer);
}

fn distance(p1: Point, p2: Point) -> f64 {
    let x_squared = f64::powi(p2.x - p1.x, 2);
    let y_squared = f64::powi(p2.y - p1.y, 2);

    f64::sqrt(x_squared + y_squared)
}
```

Our function signature for `distance()` now says exactly what we mean: it
calculates the distance between two `Point`s. And rather than `0` and `1`,
we’ve got back our `x` and `y`. This is a win for clarity.

There’s one other thing that’s a bit strange here, this annotation on our
`struct` declaration:

```rust,ignore
#[derive(Debug,Copy,Clone)]
struct Point {
```

We haven’t yet talked about traits, but we did talk about `Debug` when we
discussed arrays. This `derive` attribute allows us to tweak the behavior of
our `Point`. In this case, we are opting into copy semantics, and everything
that implements `Copy` must implement `Clone`.
# Method Syntax

In the last section on ownership, we made several references to ‘methods’.
Methods look like this:

```rust
let s1 = String::from("hello");

// call a method on our String
let s2 = s1.clone();

println!("{}", s1);
```

The call to `clone()` is attatched to `s1` with a dot. This is called ‘method
syntax’, and it’s a way to call certain functions with a different style.

Why have two ways to call functions? We’ll talk about some deeper reasons
related to ownership in a moment, but one big reason is that methods look nicer
when chained together:

```rust,ignore
// with functions
h(g(f(x)));

// with methods
x.f().g().h();
```

The nested-functions version reads in reverse: we call `f()`, then `g()`, then
`h()`, but it reads as `h()`, then `g()`, then `f()`.

Before we get into the details, let’s talk about how to define your own
methods.

## Defining methods

We can define methods with the `impl` keyword. `impl` is short for
‘implementation’. Doing so looks like this:

```rust
#[derive(Debug,Copy,Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
   fn distance(&self, other: &Point) -> f64 {
       let x_squared = f64::powi(other.x - self.x, 2);
       let y_squared = f64::powi(other.y - self.y, 2);

       f64::sqrt(x_squared + y_squared)
   }
}

let p1 = Point { x: 0.0, y: 0.0 };
let p2 = Point { x: 5.0, y: 6.5 };

assert_eq!(8.200609733428363, p1.distance(&p2));
```

Let’s break this down. First, we have our `Point` struct from earlier in the
chapter. Next comes our first use of the `impl` keyword:

```
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
# 
impl Point {
#    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
# 
#        f64::sqrt(x_squared + y_squared)
#    }
}
# 
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
# 
# assert_eq!(8.200609733428363, p1.distance(&p2));
```

Everything we put inside of the curly braces will be methods implemented on
`Point`.

```
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
# 
# impl Point {
    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
# 
#        f64::sqrt(x_squared + y_squared)
    }
# }
# 
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
# 
# assert_eq!(8.200609733428363, p1.distance(&p2));
```

Next is our definition. This looks very similar to our previous definition of
`distance()` as a function:

```rust
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
fn distance(p1: Point, p2: Point) -> f64 {
#     let x_squared = f64::powi(p2.x - p1.x, 2);
#     let y_squared = f64::powi(p2.y - p1.y, 2);
# 
#     f64::sqrt(x_squared + y_squared)
# }
```

Other than this, the rest of the example is familliar: an implementation of
`distance()`, and using the method to find an answer.

There are two differences. The first is in the first argument. Instead of a name
and a type, we have written `&self`. This is what distinguishes a method from a
function: using `self` inside of an `impl` block. Because we already know that
we are implementing this method on `Point`, we don’t need to write the type of
`self` out. However, we have written `&self`, not only `self`. This is because
we want to take our argument by reference rather than by ownership. In other
words, these two forms are the same:

```rust,ignore
fn foo(self: &Point)
fn foo(&self)
```

Just like any other parameter, you can take `self` in three forms. Here’s the
list, with the most common form first:

```rust,ignore
fn foo(&self) // take self by reference
fn foo(&mut self) // take self by mutable reference
fn foo(self) // take self by ownership
```

In this case, we only need a reference. We don’t plan on taking ownership, and
we don’t need to mutate either point. Taking by reference is by far the most
common form of method, followed by a mutable reference, and then occasionally
by ownership.

### Methods and automatic referencing

We’ve left out an important detail. It’s in this line of the example:

```
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
# 
# impl Point {
#    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
# 
#        f64::sqrt(x_squared + y_squared)
#    }
# }
# 
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
# 
assert_eq!(8.200609733428363, p1.distance(&p2));
```

When we defined `distance()`, we took both `self` and the other argument by
reference. Yet, we needed a `&` for `p2` but not `p1`. What gives?

This feature is called ‘automatic referencing’, and calling methods is one
of the few places in Rust that has behavior like this. Here’s how it works:
when you call a method with `self.(`, Rust will automatically add in `&`s
or `&mut`s to match the signature. In other words, these three are the same:

```rust
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
# 
# impl Point {
#    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
# 
#        f64::sqrt(x_squared + y_squared)
#    }
# }
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
p1.distance(&p2);
(&p1).distance(&p2);
Point::distance(&p1, &p2);
```

The first one looks much, much cleaner. Here’s another example:

```rust
let mut s = String::from("Hello,");

s.push_str(" world!");

// The above is the same as:
// (&mut s).push_str(" world!");

assert_eq!("Hello, world!", s);
```

Because [`push_str()`] has the following signature:

```rust,ignore
fn push_str(&mut self, string: &str) {
```

[`push_str()`]: http://doc.rust-lang.org/collections/string/struct.String.html#method.push_str

This automatic referencing behavior works because methods have a clear receiver
— the type of `self` — and in most cases it’s clear given the receiver and name
of a method whether the method is just reading (so needs `&self`), mutating (so
`&mut self`), or consuming (so `self`). The fact that Rust makes borrowing
implicit for method receivers is a big part of making ownership ergonomic in
practice.

## Methods can be called like functions

Furthermore, if we have a method, we can also call it like a function:

```rust
# #[derive(Debug,Copy,Clone)]
# struct Point {
#     x: f64,
#     y: f64,
# }
# 
# impl Point {
#    fn distance(&self, other: &Point) -> f64 {
#        let x_squared = f64::powi(other.x - self.x, 2);
#        let y_squared = f64::powi(other.y - self.y, 2);
# 
#        f64::sqrt(x_squared + y_squared)
#    }
# }
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
let d1 = p1.distance(&p2);
let d2 = Point::distance(&p1, &p2);

assert_eq!(d1, d2);
```

Instead of using `self.(`, we use `Point` and the namespace operator to call it
like a function instead. Because functions do not do the automatic referencing,
we must pass in `&p1` explicitly.

While methods can be called like functions, functions cannot be called like
methods. If the first argument isn’t named `self`, it cannot be called like a
method.
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

While our simple `Point` struct may be a bit too simple to bother making
generic in a real application, we're going to stick with it to show you the
syntax. Especially when building library code, generics allow for more code
re-use, and unlock a lot of powerful techniques.

## Generic data types

'Generics' let us write code that allows for several different types, while
letting us have one definition. A more generic `Point` would look like this:

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

With that, let's talk about another fundamental Rust data type: enums.
