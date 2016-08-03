# Structs

A `struct`, short for "structure", gives us the ability to name and package
together multiple related values that make up a meaningful group. If you come
from an object-oriented language, a `struct` is like an object's data
attributes. `struct` and `enum` (that we talked about in the last chapter) are
the building blocks you can use in Rust to create new types in your program's
domain in order to take full advantage of Rust's compile-time type checking.

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

Let's take a quick look at `distance()` before we move forward. To find the
distance between two points, we can use the Pythagorean Theorem. The theorem is
named after Pythagoras, who was the first person to mathematically prove this
formula. The details aren't that important; just know the theorem says that the
formula for the distance between two points is equal to:

- squaring the distance between the points horizontally (the "x" direction)
- squaring the distance between the points vertically (the "y" direction)
- adding those together
- and taking the square root of that.

So that's what we're implementing here.

```rust,ignore
f64::powi(2.0, 3)
```

The double colon (`::`) here is a namespace operator. We haven’t talked about
modules and namespaces in depth yet, but you can think of the `powi()` function
as being scoped inside of another name. In this case, the name is `f64`, the
same as the type. The `powi()` function takes two arguments: the first is a
number, and the second is the power that it raises that number to. In this
case, the second number is an integer, hence the ‘i’ in its name. Similarly,
`sqrt()` is a function under the `f64` module, which takes the square root of
its argument.

## Why `struct`s?

Our little program is okay, but we can do better. The key to seeing this is in
the signature of `distance()`:

```rust,ignore
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
```

The distance function is supposed to calculate the distance between two points.
But our distance function calculates some distance between four numbers. The
first two and last two arguments are related, but that’s not expressed anywhere
in our program itself. It would be nicer if we had a way to group `(x1, y1)`
and `(x2, y2)` together.

We’ve already discussed one way to do that: tuples. Here’s a version of our
program which uses tuples:

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
We’re now passing two arguments, so that’s more clear. But it’s also worse:
tuples don’t give names to their elements, so our calculation has gotten more
confusing:

```rust,ignore
p2.0 - p1.0
p2.1 - p1.1
```

When writing this example, your authors almost got it wrong themselves! Distance
is all about `x` and `y` points, but our code is talking about `0` and `1`.
This isn’t great.

Enter `struct`s. We can transform our tuples into something with a name for the
whole as well as names for the parts:

```rust,ignore
let p1 = (0.0, 5.0);

struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 0.0, y: 5.0 };
```

Here we've defined a `struct` and given it the name `Point`. The parts inside
`{}` are defining the _fields_ of the struct. We can have as many or as few of
them as we'd like, and we give them a name and specify their type. Here we have
two fields named `x` and `y`, and they both hold `f64`s.

We can access the field of a struct in the same way we access an element of
a tuple, except we use its name:

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

## Derived Traits

There’s one other thing that’s a bit strange here, this stuff above the
`struct` declaration:

```rust,ignore
#[derive(Debug,Copy,Clone)]
struct Point {
```

This is an annotation that tells the compiler our struct should get some
default behavior for the `Debug`, `Copy`, and `Clone` traits. We talked about
marking that types can be `Copy` and `Clone`-able in Chapter XX when we
discussed ownership. `Debug` is the trait that enables us to print out our
struct so that we can see its value while we are debugging our code.

So far, we’ve been printing values using `{}` in a `println!` macro. If we try
that with a struct, however, by default, we'll get an error. Say we have the
following program:

```rust,ignore
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 0.0, y: 5.0};
    println!("Point 1: {}", p1);
}
```

This code tries to print the `p1` point directly, which may seem innocuous. But
running it produces the following output:

```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
error: the trait bound `Point: std::fmt::Display` is not satisfied [--explain E0277]
 --> src/main.rs:8:29
8 |>     println!("Point 1: {}", p1);
  |>                             ^^
<std macros>:2:27: 2:58: note: in this expansion of format_args!
<std macros>:3:1: 3:54: note: in this expansion of print! (defined in <std macros>)
src/main.rs:8:5: 8:33: note: in this expansion of println! (defined in <std macros>)
note: `Point` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
note: required by `std::fmt::Display::fmt`
```

Whew! The core of the error is this part: *the trait bound `Point:
std::fmt::Display` is not satisfied*. `println!` can do many kinds of
formatting. By default, `{}` implements a kind of formatting known as
`Display`: output intended for direct end-user consumption. The primitive types
we’ve seen implement `Display`, as there’s only one way you’d show a `1` to a
user. But with structs, the output is less clear. Do you want commas or not?
What about the `{}`s? Should all the fields be shown?

More complex types in the standard library and that are defined by the
programmer do not automatically implement `Display` formatting. Standard
library types implement `Debug` formatting, which is intended for the
programmer to see. The `#[derive(Debug)]` annotation lets us use a default
implementation of `Debug` formatting to easily get this ability for types we've
defined. To ask `println!` to use `Debug` formatting with our `Point`, we add
the annotation to derive the trait and include `:?` in the print string, like
this:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 0.0, y: 5.0};
    println!("Point 1: {:?}", p1);
}
```

If you run this, it should print the values of each field in the `Point` struct
as desired:

```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
     Running `target/debug/points`
Point 1: Point { x: 0, y: 5 }
```

You’ll see this repeated later with other types. We’ll cover traits fully in
Chapter XX.

## Method Syntax

In the last section on ownership, we made several references to ‘methods’.
Methods look like this:

```rust
let s1 = "hello";

// call a method on s1
let s2 = s1.clone();

println!("{}", s1);
```

The call to `clone()` is attached to `s1` with a dot. This is called ‘method
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

The nested-functions version reads in reverse: the program executes `f()`, then
`g()`, then `h()`, but we read it left-to-right as `h()`, then `g()`, then
`f()`. The method syntax is executed in the same order as we would read it.

Before we get into the details, let’s talk about how to define your own
methods.

### Defining methods

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

```rust,ignore
impl Point {
    // ...
}
```

Everything we put inside of the curly braces will be methods implemented on
`Point`. Next is our definition:

```rust,ignore
fn distance(&self, other: &Point) -> f64 {
    // ...
}
```

Other than this, the rest of the example is familiar: an implementation of
`distance()` and using the method to find an answer.

Our definition of `distance()` here as a method looks very similar to our
previous definition of `distance()` as a function, but with two differences.
Here's the `distance()` function again:

```rust,ignore
fn distance(p1: Point, p2: Point) -> f64 {
    // ...
}
```

The first difference is in the first argument. Instead of a name and a type, we
have written `&self`. This is what distinguishes a method from a function:
using `self` inside of an `impl` block means we have a method. Because we
already know that we are implementing this method on `Point` because of the
surrounding `impl Point` block, we don’t need to write the type of `self` out.

Note that we have written `&self`, not just `self`. This is because we want to
take a reference to our argument's value rather than taking ownership of it. In
other words, these two forms are the same:

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

In this case, we only need a reference. We don’t need to mutate either `Point`
to get the distance between them, so we won't take a mutable reference to the
`Point` that we call the method on. Methods that take ownership of `self` are
rarely used. An example of a time to do that would be if we wanted to have a
method that would transform `self` into something else and prevent other code
from using the value of `self` after the transformation happens.

#### Methods and automatic referencing

We’ve left out an important detail. It’s in this line of the example:

```rust,ignore
assert_eq!(8.200609733428363, p1.distance(&p2));
```

When we defined `distance()`, we took both `self` and the other argument by
reference. Yet, we needed a `&` for `p2` but not `p1`. What gives?

This feature is called ‘automatic referencing’, and calling methods is one
of the few places in Rust that has behavior like this. Here’s how it works:
when you call a method with `self.(`, Rust will automatically add in `&`s
or `&mut`s to match the signature. In other words, these are the same:

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
