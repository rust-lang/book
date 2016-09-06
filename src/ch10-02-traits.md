# Traits

At the end of the last section, we had this code:

```rust,ignore
fn print<T>(argument: T) {
    println!("Got an argument: {}", argument);
}
```

Which gave this error:

```text
	error[E0277]: the trait bound `T: std::fmt::Display` is not satisfied
 --> <anon>:3:37
  |
3 |     println!("Got an argument: {}", argument);
  |                                     ^^^^^^^^ trait `T: std::fmt::Display` not satisfied
  |
  = help: consider adding a `where T: std::fmt::Display` bound
  = note: required by `std::fmt::Display::fmt`

error: aborting due to previous error(s)
```

The error message here refers to a *trait bound*. What's up with that?

Rust has a feature called *traits*. Traits are similar to a feature often
called 'interfaces' in other languages, but are also different. Traits let us
do another kind of abstraction: they let us abstract over a group of methods.

Here's a trait:

```rust
trait Printable {
    fn print(&self);
}
```

We declare a trait with the `trait` keyword, and then the trait's name. In this
case, our trait will describe types which can be printed. Inside of some curly
braces, we declare a method signature, but instead of providing an
implementation, we use a semicolon. A trait can also have multiple methods:

```rust
trait Printable {
    fn print(&self);

    fn print_debug(&self);
}
```

Once we have a trait, we can use the `impl` keyword to implement that trait
for a type. It works like this:

```rust
struct Point {
    x: i32,
    y: i32,
}

trait Printable {
    fn print(&self);
}

impl Printable for Point {
    fn print(&self) {
        println!("I'm a Point! I have an x of {} and a y of {}.", self.x, self.y);
    }
}
```

In the same way `impl` let us define methods, we've also defined methods that
pertain to our trait. We can call methods that our trait has defined just like
we called other methods:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
# 
# trait Printable {
#     fn print(&self);
# }
# 
# impl Printable for Point {
#     fn print(&self) {
#         println!("I'm a Point! I have an x of {} and a y of {}.", self.x, self.y);
#     }
# }
# 
let p = Point { x: 1, y: 10 };

p.print();
```

There's a twist, though. We can only do this if our trait is in scope. For example,
if we had our trait in a module:

```rust
mod point {
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    
    pub trait Printable {
        fn print(&self);
    }
    
    impl Printable for Point {
        fn print(&self) {
            println!("I'm a Point! I have an x of {} and a y of {}.", self.x, self.y);
        }
    }
}

// Without this line, we'd get an error:
use point::Printable;

fn main() {
    let p = point::Point { x: 1, y: 10 };

    p.print();
}
```

You'll notice we also had to make everything `pub`, as per the privacy rules we
talked about in Chapter 7.

Why do we need the trait in scope? Imagine we had two traits with the same
method definition, and our `Point` struct implemented both. We wouldn't know
which method we were trying to call. `use` makes it explicit.

## Trait bounds

We previously knew how to define methods, so what makes traits special? Well,
imagine we had a function that wanted to call `print` for any type that supports
printing. We could write it like this:

```rust,ignore
fn print(value: v) {
    v.print();
}
```

But we have a problem. What happens if we tried to pass something to `print`
that did not implement the `print` method? Because of this, Rust won't let the
above code compile.

Let's take a step back and think about what we've written. There's a mis-match:
above, we said "a function that wanted to call `print` for any type that
supports it", but what we said in our code was "for any type T, any type at
all." So how do we say "for any type T that implements `Printable`? Like this:

```rust
trait Printable {
    fn print(&self);
}

fn print<T: Printable>(value: T) {
    value.print();
}
```

The `T: Printable` syntax says, "the type parameter `T` represents any type
that implements the `Printable` trait." This full example will work just
fine:

```rust
struct Point {
    x: i32,
    y: i32,
}

trait Printable {
    fn print(&self);
}

impl Printable for Point {
    fn print(&self) {
        println!("I'm a Point! I have an x of {} and a y of {}.", self.x, self.y);
    }
}

fn print<T: Printable>(value: T) {
    value.print();
}

let p = Point { x: 0, y: 10 };

print(p);
```

Traits are an extremely useful feature of Rust. You'll almost never see generic
functions without an accompanying trait bound. There are many traits in the
standard library, and they're used for many, many different things. For
example, our `Printable` trait is similar to one of those traits, `Display`.
And in fact, that's how `println!` decides how to format things with `{}`. The
`Display` trait has a `fmt` method that determines how to format something.

Here's our original example, but fixed:

```rust
use std::fmt::Display;

fn print<T: Display>(argument: T) {
    println!("Got an argument: {}", argument);
}
```

Now that we've said "for any type that implements `Display`," this works well.

## Where syntax

When bounds start getting complicated, there is another syntax that's a bit
cleaner: `where`. And in fact, our original error referred to it. It looks
like this:

```rust
use std::fmt::Display;

fn print<T>(argument: T) where T: Display {
    println!("Got an argument: {}", argument);
}
```

Instead of the `T: Display` going inside the angle brackets, they go after a
`where`, placed at the end of the function signature. This can make complex
signatures easier to read.
