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
