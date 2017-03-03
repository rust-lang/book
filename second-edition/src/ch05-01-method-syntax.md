## Method Syntax

*Methods* are similar to functions: they’re declared with the `fn` keyword and
their name, they can have parameters and return values, and they contain some
code that gets run when they’re called from somewhere else. Methods are
different from functions, however, because they’re defined within the context
of a struct (or an enum or a trait object, which we will cover in Chapters 6
and 13, respectively), and their first parameter is always `self`, which
represents the instance of the struct that the method is being called on.

### Defining Methods

Let’s change our `area` function that has a `Rectangle` instance as a parameter
and instead make an `area` method defined on the `Rectangle` struct, as shown
in Listing 5-7:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

<figcaption>

Listing 5-7: Defining an `area` method on the `Rectangle` struct

</figcaption>
</figure>

<!-- Will add ghosting and wingdings here in libreoffice /Carol -->

In order to make the function be defined within the context of `Rectangle`, we
start an `impl` block (`impl` is short for *implementation*). Then we move the
function within the `impl` curly braces, and change the first (and in this
case, only) parameter to be `self` in the signature and everywhere within the
body. Then in `main` where we called the `area` function and passed `rect1` as
an argument, we can instead use *method syntax* to call the `area` method on
our `Rectangle` instance. Method syntax is taking an instance and adding a dot
followed by the method name, parentheses, and any arguments.

In the signature for `area`, we get to use `&self` instead of `rectangle:
&Rectangle` because Rust knows the type of `self` is `Rectangle` due to this
method being inside the `impl Rectangle` context. Note we still need to have
the `&` before `self`, just like we had `&Rectangle`. Methods can choose to
take ownership of `self`, borrow `self` immutably as we’ve done here, or borrow
`self` mutably, just like any other parameter.

We’ve chosen `&self` here for the same reason we used `&Rectangle` in the
function version: we don’t want to take ownership, and we just want to be able
to read the data in the struct, not write to it. If we wanted to be able to
change the instance that we’ve called the method on as part of what the method
does, we’d put `&mut self` as the first parameter instead. Having a method that
takes ownership of the instance by having just `self` as the first parameter is
rarer; this is usually used when the method transforms `self` into something
else and we want to prevent the caller from using the original instance after
the transformation.

The main benefit of using methods over functions, in addition to getting to use
method syntax and not having to repeat the type of `self` in every method’s
signature, is for organization. We’ve put all the things we can do with an
instance of a type together in one `impl` block, rather than make future users
of our code search for capabilities of `Rectangle` all over the place.

<!-- PROD: START BOX -->

> ### Where’s the `->` operator?
>
> In languages like C++, there are two different operators for calling methods:
> `.` if you’re calling a method on the object directly, and `->` if you’re
> calling the method on a pointer to the object and thus need to dereference the
> pointer first. In other words, if `object` is a pointer, `object->something()`
> is like `(*object).something()`.
>
> Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a
> feature called *automatic referencing and dereferencing*. Calling methods is
> one of the few places in Rust that has behavior like this.
>
> Here’s how it works: when you call a method with `object.something()`, Rust
> will automatically add in `&`, `&mut`, or `*` so that `object` matches the
> signature of the method. In other words, these are the same:
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> The first one looks much, much cleaner. This automatic referencing behavior
> works because methods have a clear receiver — the type of `self`. Given the
> receiver and name of a method, Rust can figure out definitively whether the
> method is just reading (so needs `&self`), mutating (so `&mut self`), or
> consuming (so `self`). The fact that Rust makes borrowing implicit for method
> receivers is a big part of making ownership ergonomic in practice.

<!-- PROD: END BOX -->

### Methods with More Parameters

Let’s practice some more with methods by implementing a second method on our
`Rectangle` struct. This time, we’d like for an instance of `Rectangle` to take
another instance of `Rectangle` and return `true` if the second rectangle could
fit completely within `self` and `false` if it would not. That is, if we run
the code in Listing 5-8, once we've defined the `can_hold` method:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

<figcaption>

Listing 5-8: Demonstration of using the as-yet-unwritten `can_hold` method

</figcaption>
</figure>

We want to see this output, since both of `rect2`’s dimensions are smaller than
`rect1`’s, but `rect3` is wider than `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as a parameter. We can tell what the type of the
parameter will be by looking at a call site: `rect1.can_hold(&rect2)` passes in
`&rect2`, which is an immutable borrow to `rect2`, an instance of `Rectangle`.
This makes sense, since we only need to read `rect2` (rather than write, which
would mean we’d need a mutable borrow) and we want `main` to keep ownership of
`rect2` so that we could use it again after calling this method. The return
value of `can_hold` will be a boolean, and the implementation will check to see
if `self`’s length and width are both greater than the length and width of the
other `Rectangle`, respectively. Let’s add this new method to the `impl` block
from Listing 5-7:

<span class="filename">Filename: src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     length: u32,
#     width: u32,
# }
#
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

<!-- Will add ghosting here in libreoffice /Carol -->

If we run this with the `main` from Listing 5-8, we will get our desired output!
Methods can have multiple parameters that we add to the signature after the
`self` parameter, and those parameters work just like parameters in functions
do.

### Associated Functions

One more useful feature of `impl` blocks: we’re allowed to define functions
within `impl` blocks that *don’t* take `self` as a parameter. These are called
*associated functions*, since they’re associated with the struct. They’re still
functions though, not methods, since they don’t have an instance of the struct
to work with. You’ve already used an associated function: `String::from`.

Associated functions are often used for constructors that will return a new
instance of the struct. For example, we could provide an associated function
that would have one dimension parameter and use that as both length and width,
thus making it easier to create a square `Rectangle` rather than having to
specify the same value twice:

<span class="filename">Filename: src/main.rs</span>

```rust
# #[derive(Debug)]
# struct Rectangle {
#     length: u32,
#     width: u32,
# }
#
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```

To call this associated function, we use the `::` syntax with the struct name:
`let sq = Rectangle::square(3);`, for example. This function is namespaced by
the struct: the `::` syntax is used for both associated functions and
namespaces created by modules, which we’ll learn about in Chapter 7.

## Summary

Structs let us create custom types that are meaningful for our domain. By using
structs, we can keep associated pieces of data connected to each other and name
each piece to make our code clear. Methods let us specify the behavior that
instances of our structs have, and associated functions let us namespace
functionality that is particular to our struct without having an instance
available.

Structs aren’t the only way we can create custom types, though; let’s turn to
the `enum` feature of Rust and add another tool to our toolbox.
