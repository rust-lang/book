# Structs

A `struct`, short for *structure*, is a custom data type that lets us name and
package together multiple related values that make up a meaningful group. If
you come from an object-oriented language, a `struct` is like an object's data
attributes. In the next section of this chapter, we'll talk about how to define
methods on our structs; methods are how you specify the *behavior* that goes
along with a struct's data. The `struct` and `enum` (that we will talk about in
Chapter 6) concepts are the building blocks for creating new types in your
program's domain in order to take full advantage of Rust's compile-time type
checking.

One way of thinking about structs is that they are similar to tuples that we
talked about in Chapter 3. Like tuples, the pieces of a struct can be different
types. Unlike tuples, we name each piece of data so that it's clearer what the
values mean. Structs are more flexible as a result of these names: we don't
have to rely on the order of the data to specify or access the values of an
instance.

To define a struct, we enter the keyword `struct` and give the whole struct a
name. A struct's name should describe what the significance is of these pieces
of data being grouped together. Then, inside curly braces, we define the names
of the pieces of data, which we call *fields*, and specify each field's type.
For example, a struct to store information about a user account might look like:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To use a struct, we create an *instance* of that struct by specifying concrete
values for each of the fields. Creating an instance is done by declaring a
binding with `let`, stating the name of the struct, then curly braces with
`key: value` pairs inside it where the keys are the names of the fields and the
values are the data we want to store in those fields. The fields don't have to
be specified in the same order in which the struct declared them. In other
words, the struct definition is like a general template for the type, and
instances fill in that template with particular data to create values of the
type. For example, we can declare a particular user like this:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

To get a particular value out of a struct, we can use dot notation. If we
wanted just this user's email address, we can say `user1.email`.

## An Example Program

To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start off with single variable
bindings, then refactor our program until we're using `struct`s instead.

Let’s make a new binary project with Cargo called *rectangles* that will take
the length and width of a rectangle specified in pixels and will calculate the
area of the rectangle. Here’s a short program that has one way of doing just
that to put into our project's `src/main.rs`:

Filename: src/main.rs

```rust
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!("The area of the rectangle is {}", area(length1, width1));
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
```

Let's try running this program with `cargo run`:

```bash
The area of the rectangle is 1500
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

The area function is supposed to calculate the area of one rectangle, but our
function takes two arguments. The arguments are related, but that's not
expressed anywhere in our program itself. It would be more readable and more
manageable to group length and width together.

We’ve already discussed one way we might do that in Chapter 3: tuples. Here’s a
version of our program which uses tuples:

Filename: src/main.rs

```rust
fn main() {
    let rect1 = (50, 30);

    println!("The area of the rectangle is {}", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

In one way, this is a little better. Tuples let us add a little bit of
structure, and we’re now passing just one argument. But in another way this
method less clear: tuples don’t give names to their elements, so our
calculation has gotten more confusing because we have to use the tuple's index:

```rust,ignore
dimensions.0 * dimensions.1
```

It doesn't matter if we mix up length and width for the area calculation, but
if we were to draw the rectangle on the screen it would matter! We would have
to remember that `length` was the tuple index `0` and `width` was the tuple
index `1`. If someone else was to work on this code, they would have to figure
this out and remember it as well. It would be easy to forget or mix these
values up and cause errors, since we haven't conveyed the meaning of our data
in our code.

### Refactoring with Structs: Adding More Meaning

Here is where we bring in `struct`s. We can transform our tuple into a data
type with a name for the whole as well as names for the parts:

Filename: src/main.rs

```rust
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("The area of the rectangle is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
```

Here we've defined a `struct` and given it the name `Rectangle`. Inside the
`{}` we defined the fields to be `length` and `width`, both of which have type
`u32`. Then in `main`, we create a particular instance of a `Rectangle` that
has a length of 50 and a width of 30.

Our `area` function now takes one argument that we've named `rectangle` whose
type is an immutable borrow of a struct `Rectangle` instance. As we covered in
Chapter 4, we want to borrow the struct rather than take ownership of it so
that `main` keeps its ownership and can continue using `rect1`, so that's why
we have the `&` in the function signature and at the call site.

The `area` function accesses the `length` and `width` fields of the `Rectangle`
instance it got as an argument. Our function signature for `area` now says
exactly what we mean: calculate the area of a `Rectangle`, using its `length`
and `width` fields. This conveys that the length and width are related to each
other, and gives descriptive names to the values rather than using the index
values of `0` and `1`. This is a win for clarity.

### Adding Useful Functionality with Derived Traits

It'd be nice to be able to print out an instance of our `Rectangle` while we're
debugging our program and be able to see the values for all its fields. Let's
try using the `println!` macro as we have been and see what happens:

Filename: src/main.rs

```rust,ignore
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("The rectangle is {}", rect1);
}
```

If we run this, we get an error with the core message ``the trait bound
`Rectangle: std::fmt::Display`` is not satisfied`. The `println!` macro can do
many kinds of formatting, and by default, `{}` tells `println!` to use a kind
of formatting known as `Display`: output intended for direct end-user
consumption. The primitive types we’ve seen so far implement `Display` by
default, as there’s only one way you’d want to show a `1` or any other
primitive type to a user. But with structs, the way `println!` should format
the output is less clear as there are more display options: Do you want commas
or not? Do you want to print the struct `{}`s? Should all the fields be shown?
Because of this ambiguity, Rust doesn't try to guess what we want and structs
do not have a provided implementation of `Display`.

If we keep reading the error messages, though, we'll find ``note: `Rectangle`
cannot be formatted with the default formatter; try using `:?` instead if you
are using a format string``. Let's try it! The `println!` will now look like
`println!("The rectangle is {:?}", rect1);`. Putting the specifier `:?` inside
the `{}` tells `println!` we want to use an output format called `Debug`.
`Debug` is a trait that enables us to print out our struct in a way that is
useful for developers so that we can see its value while we are debugging our
code.

Let's try running with this change and... drat. We still get an error: ``the
trait bound `Rectangle: std::fmt::Debug` is not satisfied``. Again, though, the
compliler has given us a helpful note! ``note: `Rectangle` cannot be formatted
using `:?`; if it is defined in your crate, add `#[derive(Debug)]` or manually
implement it``.

Rust *does* include functionality to print out debugging information, but we
have to explicitly opt-in to having that functionality be available for our
struct. To do that, we add the annotation `#[derive(Debug)]` just before our
struct definition, so now our program looks like this:

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("The rectangle is {:?}", rect1);
}
```

*Now* if we run this program, we won't get any errors and we'll see the
following output:

```bash
The rectangle is Rectangle { length: 50, width: 30 }
```

Neat! It's not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging.

There are a number of traits Rust has provided for us to use with the `derive`
annotation that can add useful behavior to our custom types. Those traits and
their behaviors are listed in Appendix XX. We'll be covering how to implement
these traits with different behavior, as well as creating your own traits, in
Chapter 10.

Our `area` function is pretty specific-- it only computes the area of
rectangles. It would be nice to tie this behavior together more closely with our
`Rectangle` struct, since it's behavior that our `Rectangle` type has
specifically. Let's now look at how we can continue to refactor this code by
turning the `area` function into an `area` *method* defined on our `Rectangle`
type.
