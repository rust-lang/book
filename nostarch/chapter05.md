
[TOC]

# Structs

A `struct`, short for *structure*, is a custom data type that lets us name and
package together multiple related values that make up a meaningful group. If
you come from an object-oriented language, a `struct` is like an object’s data
attributes. In the next section of this chapter, we’ll talk about how to define
methods on our structs; methods are how you specify the *behavior* that goes
along with a struct’s data. The `struct` and `enum` (that we will talk about in
Chapter 6) concepts are the building blocks for creating new types in your
program’s domain in order to take full advantage of Rust’s compile-time type
checking.

One way of thinking about structs is that they are similar to tuples, which we
talked about in Chapter 3. Like tuples, the pieces of a struct can be different
types. Unlike tuples, we name each piece of data so that it’s clearer what the
values mean. Structs are more flexible as a result of these names: we don’t
have to rely on the order of the data to specify or access the values of an
instance.

To define a struct, we enter the keyword `struct` and give the whole struct a
name. A struct’s name should describe what the significance is of these pieces
of data being grouped together. Then, inside curly braces, we define the names
of the pieces of data, which we call *fields*, and specify each field’s type.
For example, Listing 5-1 shows a struct to store information about a user
account:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<caption>
Listing 5-1: A `User` struct definition
</caption>

To use a struct once we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. Creating an instance is
done by stating the name of the struct, then curly braces with `key: value`
pairs inside it where the keys are the names of the fields and the values are
the data we want to store in those fields. The fields don’t have to be
specified in the same order in which the struct declared them. In other words,
the struct definition is like a general template for the type, and instances
fill in that template with particular data to create values of the type. For
example, we can declare a particular user like this:

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

To get a particular value out of a struct, we can use dot notation. If we
wanted just this user’s email address, we can say `user1.email`.

## Ownership of Struct Data

In the `User` struct definition in Listing 5-1, we used the owned `String` type
rather than the `&str` string slice type. This is a deliberate choice because
we want instances of this struct to own all of its data, and for that data to
be valid for as long as the entire struct is valid.

It is possible for structs to store references to data owned by something else,
but to do so requires the use of *lifetimes*, a feature of Rust that we’ll
discuss in Chapter 10. Lifetimes ensure that the data a struct references is
valid for as long as the struct is. If you try to store a reference in a struct
without specifying lifetimes, like this:

Filename: src/main.rs

```rust,ignore
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

The compiler will complain that it needs lifetime specifiers:

```text
error[E0106]: missing lifetime specifier
 -->
  |
2 |     username: &str,
  |               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 -->
  |
3 |     email: &str,
  |            ^ expected lifetime parameter
```

We will talk about how to fix these errors in order to store references in
structs in Chapter 10, but for now, fix errors like these by switching to owned
types like `String` instead of references like `&str`.

## An Example Program

To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start off with single variables, then
refactor our program until we’re using structs instead.

Let’s make a new binary project with Cargo called *rectangles* that will take
the length and width of a rectangle specified in pixels and will calculate the
area of the rectangle. Listing 5-2 has a short program with one way of doing
just that in our project’s *src/main.rs*:

Filename: src/main.rs

```rust
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
```

<caption>
Listing 5-2: Calculating the area of a rectangle specified by its length and
width in separate variables
</caption>

Let’s try running this program with `cargo run`:

```text
The area of the rectangle is 1500 square pixels.
```

### Refactoring with Tuples

Our little program works okay; it figures out the area of the rectangle by
calling the `area` function with each dimension. But we can do better. The
length and the width are related to each other since together they describe one
rectangle.

The issue with this method is evident in the signature of `area`:

```rust,ignore
fn area(length: u32, width: u32) -> u32 {
```

The `area` function is supposed to calculate the area of one rectangle, but our
function takes two arguments. The arguments are related, but that’s not
expressed anywhere in our program itself. It would be more readable and more
manageable to group length and width together.

We’ve already discussed one way we might do that in Chapter 3: tuples. Listing
5-3 has a version of our program which uses tuples:

Filename: src/main.rs

```rust
fn main() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

<caption>
Listing 5-3: Specifying the length and width of the rectangle with a tuple
</caption>

<!-- I will add ghosting & wingdings once we're in libreoffice /Carol -->

In one way, this is a little better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way this method less clear:
tuples don’t give names to their elements, so our calculation has gotten more
confusing because we have to index into the parts of the tuple:

<!-- I will change this to use wingdings instead of repeating this code once
we’re in libreoffice /Carol -->

```rust,ignore
dimensions.0 * dimensions.1
```

It doesn’t matter if we mix up length and width for the area calculation, but
if we were to draw the rectangle on the screen it would matter! We would have
to remember that `length` was the tuple index `0` and `width` was the tuple
index `1`. If someone else was to work on this code, they would have to figure
this out and remember it as well. It would be easy to forget or mix these
values up and cause errors, since we haven’t conveyed the meaning of our data
in our code.

### Refactoring with Structs: Adding More Meaning

Here is where we bring in structs. We can transform our tuple into a data type
with a name for the whole as well as names for the parts, as shown in Listing
5-4:

Filename: src/main.rs

```rust
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
```

<caption>
Listing 5-4: Defining a `Rectangle` struct
</caption>

<!-- Will add ghosting & wingdings once we're in libreoffice /Carol -->

Here we’ve defined a struct and given it the name `Rectangle`. Inside the `{}`
we defined the fields to be `length` and `width`, both of which have type
`u32`. Then in `main`, we create a particular instance of a `Rectangle` that
has a length of 50 and a width of 30.

Our `area` function now takes one argument that we’ve named `rectangle` whose
type is an immutable borrow of a struct `Rectangle` instance. As we covered in
Chapter 4, we want to borrow the struct rather than take ownership of it so
that `main` keeps its ownership and can continue using `rect1`, so that’s why
we have the `&` in the function signature and at the call site.

The `area` function accesses the `length` and `width` fields of the `Rectangle`
instance it got as an argument. Our function signature for `area` now says
exactly what we mean: calculate the area of a `Rectangle`, using its `length`
and `width` fields. This conveys that the length and width are related to each
other, and gives descriptive names to the values rather than using the tuple
index values of `0` and `1`. This is a win for clarity.

### Adding Useful Functionality with Derived Traits

It’d be nice to be able to print out an instance of our `Rectangle` while we’re
debugging our program and see the values for all its fields. Listing 5-5 tries
using the `println!` macro as we have been:

Filename: src/main.rs

```rust,ignore
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {}", rect1);
}
```

<caption>
Listing 5-5: Attempting to print a `Rectangle` instance
</caption>

If we run this, we get an error with this core message:

```text
error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
```

The `println!` macro can do many kinds of formatting, and by default, `{}`
tells `println!` to use formatting known as `Display`: output intended for
direct end-user consumption. The primitive types we’ve seen so far implement
`Display` by default, as there’s only one way you’d want to show a `1` or any
other primitive type to a user. But with structs, the way `println!` should
format the output is less clear as there are more display possibilities: Do you
want commas or not? Do you want to print the struct `{}`s? Should all the
fields be shown? Because of this ambiguity, Rust doesn’t try to guess what we
want and structs do not have a provided implementation of `Display`.

If we keep reading the errors, though, we’ll find this helpful note:

```text
note: `Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
```

Let’s try it! The `println!` will now look like
`println!("rect1 is {:?}", rect1);`. Putting the specifier `:?` inside
the `{}` tells `println!` we want to use an output format called `Debug`.
`Debug` is a trait that enables us to print out our struct in a way that is
useful for developers so that we can see its value while we are debugging our
code.

Let’s try running with this change and… drat. We still get an error:

```text
error: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
```

Again, though, the compiler has given us a helpful note!

```text
note: `Rectangle` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
```

Rust *does* include functionality to print out debugging information, but we
have to explicitly opt-in to having that functionality be available for our
struct. To do that, we add the annotation `#[derive(Debug)]` just before our
struct definition, as shown in Listing 5-6:

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
}
```

<caption>
Listing 5-6: Adding the annotation to derive the `Debug` trait and printing the
`Rectangle` instance using debug formatting
</caption>

At this point, if we run this program, we won’t get any errors and we’ll see
the following output:

```text
rect1 is Rectangle { length: 50, width: 30 }
```

Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. If we want
output that is a bit prettier and easier to read, which can be helpful with
larger structs, we can use `{:#?}` in place of `{:?}` in the `println!` string.
If we use the pretty debug style in this example, the output will look like:

```
rect1 is Rectangle {
    length: 50,
    width: 30
}
```

There are a number of traits Rust has provided for us to use with the `derive`
annotation that can add useful behavior to our custom types. Those traits and
their behaviors are listed in Appendix C. We’ll be covering how to implement
these traits with custom behavior, as well as creating your own traits, in
Chapter 10.

Our `area` function is pretty specific—it only computes the area of rectangles.
It would be nice to tie this behavior together more closely with our
`Rectangle` struct, since it’s behavior that our `Rectangle` type has
specifically. Let’s now look at how we can continue to refactor this code by
turning the `area` function into an `area` *method* defined on our `Rectangle`
type.

## Method Syntax

*Methods* are similar to functions: they’re declared with the `fn` keyword and
their name, they can take arguments and return values, and they contain some
code that gets run when they’re called from somewhere else. Methods are
different from functions, however, because they’re defined within the context
of a struct (or an enum or a trait object, which we will cover in Chapters 6
and 13, respectively), and their first argument is always `self`, which
represents the instance of the struct that the method is being called on.

### Defining Methods

Let’s change our `area` function that takes a `Rectangle` instance as an
argument and instead make an `area` method defined on the `Rectangle` struct,
as shown in Listing 5-7:

Filename: src/main.rs

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

<caption>
Listing 5-7: Defining an `area` method on the `Rectangle` struct
</caption>

<!-- Will add ghosting and wingdings here in libreoffice /Carol -->

In order to make the function be defined within the context of `Rectangle`, we
start an `impl` block (`impl` is short for *implementation*). Then we move the
function within the `impl` curly braces, and change the first (and in this
case, only) argument to be `self` in the signature and everywhere within the
body. Then in `main` where we called the `area` function and passed `rect1` as
an argument, we can instead use *method syntax* to call the `area` method on
our `Rectangle` instance. Method syntax is taking an instance and adding a dot
followed by the method name, parentheses, and any arguments.

In the signature for `area`, we get to use `&self` instead of `rectangle:
&Rectangle` because Rust knows the type of `self` is `Rectangle` due to this
method being inside the `impl Rectangle` context. Note we still need to have
the `&` before `self`, just like we had `&Rectangle`. Methods can choose to
take ownership of `self`, borrow `self` immutably as we’ve done here, or borrow
`self` mutably, just like any other argument.

We’ve chosen `&self` here for the same reason we used `&Rectangle` in the
function version: we don’t want to take ownership, and we just want to be able
to read the data in the struct, not write to it. If we wanted to be able to
change the instance that we’ve called the method on as part of what the method
does, we’d put `&mut self` as the first argument instead. Having a method that
takes ownership of the instance by having just `self` as the first argument is
rarer; this is usually used when the method transforms `self` into something
else and we want to prevent the caller from using the original instance after
the transformation.

The main benefit of using methods over functions, in addition to getting to use
method syntax and not having to repeat the type of `self` in every method’s
signature, is for organization. We’ve put all the things we can do with an
instance of a type together in one `impl` block, rather than make future users
of our code search for capabilities of `Rectangle` all over the place.

PROD: START BOX

### Where’s the `->` Operator?

In languages like C++, there are two different operators for calling methods:
`.` if you’re calling a method on the object directly, and `->` if you’re
calling the method on a pointer to the object and thus need to dereference the
pointer first. In other words, if `object` is a pointer, `object->something()`
is like `(*object).something()`.

Rust doesn’t have an equivalent to the `->` operator; instead, Rust has a
feature called *automatic referencing and dereferencing*. Calling methods is
one of the few places in Rust that has behavior like this.

Here’s how it works: when you call a method with `object.something()`, Rust
will automatically add in `&`, `&mut`, or `*` so that `object` matches the
signature of the method. In other words, these are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much, much cleaner. This automatic referencing behavior
works because methods have a clear receiver — the type of `self`. Given the
receiver and name of a method, Rust can figure out definitively whether the
method is just reading (so needs `&self`), mutating (so `&mut self`), or
consuming (so `self`). The fact that Rust makes borrowing implicit for method
receivers is a big part of making ownership ergonomic in practice.

PROD: END BOX

### Methods with More Arguments

Let’s practice some more with methods by implementing a second method on our
`Rectangle` struct. This time, we’d like for an instance of `Rectangle` to take
another instance of `Rectangle` and return `true` if the second rectangle could
fit completely within `self` and `false` if it would not. That is, if we run
the code in Listing 5-8, once we’ve defined the `can_hold` method:

Filename: src/main.rs

```rust,ignore
fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

<caption>
Listing 5-8: Demonstration of using the as-yet-unwritten `can_hold` method
</caption>

We want to see this output, since both of `rect2`’s dimensions are smaller than
`rect1`’s, but `rect3` is wider than `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

We know we want to define a method, so it will be within the `impl Rectangle`
block. The method name will be `can_hold`, and it will take an immutable borrow
of another `Rectangle` as an argument. We can tell what the type of the
argument will be by looking at a call site: `rect1.can_hold(&rect2)` passes in
`&rect2`, which is an immutable borrow to `rect2`, an instance of `Rectangle`.
This makes sense, since we only need to read `rect2` (rather than write, which
would mean we’d need a mutable borrow) and we want `main` to keep ownership of
`rect2` so that we could use it again after calling this method. The return
value of `can_hold` will be a boolean, and the implementation will check to see
if `self`’s length and width are both greater than the length and width of the
other `Rectangle`, respectively. Let’s add this new method to the `impl` block
from Listing 5-7:

Filename: src/main.rs

```rust
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
Methods can take multiple arguments that we add to the signature after the
`self` parameter, and those arguments work just like arguments in functions do.

### Associated Functions

One more useful feature of `impl` blocks: we’re allowed to define functions
within `impl` blocks that *don’t* take `self` as a parameter. These are called
*associated functions*, since they’re associated with the struct. They’re still
functions though, not methods, since they don’t have an instance of the struct
to work with. You’ve already used an associated function: `String::from`.

Associated functions are often used for constructors that will return a new
instance of the struct. For example, we could provide an associated function
that would take one dimension argument and use that as both length and width,
thus making it easier to create a square `Rectangle` rather than having to
specify the same value twice:

Filename: src/main.rs

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```

To call this associated function, we use the `::` syntax with the struct name:
`let sq = Rectange::square(3);`, for example. This function is namespaced by
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
