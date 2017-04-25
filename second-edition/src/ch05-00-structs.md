# Using Structs to Structure Related Data

A *struct*, or *structure*, is a custom data type that lets us name and package
together multiple related values that make up a meaningful group. If you’re
familiar with an object-oriented language, a *struct* is like an object’s data
attributes. In this chapter, we'll compare and contrast tuples with structs,
demonstrate how to use structs, and discuss how to define methods and
associated functions on structs to specify behavior associated with a struct's
data . The struct and *enum* (which is discussed in Chapter 6) concepts are the
building blocks for creating new types in your program’s domain to take full
advantage of Rust’s compile time type checking.

## Defining and Instantiating Structs

Structs are similar to tuples, which were discussed in Chapter 3. Like tuples,
the pieces of a struct can be different types. Unlike tuples, we name each
piece of data so it’s clear what the values mean. As a result of these names,
structs are more flexible than tuples: we don’t have to rely on the order of
the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A
struct’s name should describe the significance of the pieces of data being
grouped together. Then, inside curly braces, we define the names and types of
the pieces of data, which we call *fields*. For example, Listing 5-1 shows a
struct to store information about a user account:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Listing 5-1: A `User` struct definition</span>

To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct, and then add curly braces containing `key:
value` pairs where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user like this:

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

To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we can use `user1.email` wherever we want to
use this value.

> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want instances of this struct to own all of its data and for that
> data to be valid for as long as the entire struct is valid.
>
> It’s possible for structs to store references to data owned by something else,
> but to do so requires the use of *lifetimes*, a Rust feature that is discussed
> in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid
> for as long as the struct is. Let’s say you try to store a reference in a
> struct without specifying lifetimes, like this:
>
> <span class="filename">Filename: src/main.rs</span>
>
> ```rust,ignore
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> The compiler will complain that it needs lifetime specifiers:
>
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> We’ll discuss how to fix these errors so you can store references in structs
> in Chapter 10, but for now, we’ll fix errors like these using owned types like
> `String` instead of references like `&str`.

## An Example Program Using Structs

To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start with single variables, and then
refactor the program until we’re using structs instead.

Let’s make a new binary project with Cargo called *rectangles* that will take
the length and width of a rectangle specified in pixels and will calculate the
area of the rectangle. Listing 5-2 shows a short program with one way of doing
just that in our project’s *src/main.rs*:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 5-2: Calculating the area of a rectangle
specified by its length and width in separate variables</span>

Now, run this program using `cargo run`:

```text
The area of the rectangle is 1500 square pixels.
```

### Refactoring with Tuples

Even though Listing 5-2 works and figures out the area of the rectangle by
calling the `area` function with each dimension, we can do better. The length
and the width are related to each other because together they describe one
rectangle.

The issue with this method is evident in the signature of `area`:

```rust,ignore
fn area(length: u32, width: u32) -> u32 {
```

The `area` function is supposed to calculate the area of one rectangle, but the
function we wrote has two parameters. The parameters are related, but that’s
not expressed anywhere in our program. It would be more readable and more
manageable to group length and width together.

We’ve already discussed one way we might do that in the Grouping Values into
Tuples section of Chapter 3 on page XX: by using tuples. Listing 5-3 shows
another version of our program that uses tuples:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 5-3: Specifying the length and width of the
rectangle with a tuple</span>

In one way, this program is better. Tuples let us add a bit of structure, and
we’re now passing just one argument. But in another way this version is less
clear: tuples don’t name their elements, so our calculation has become more
confusing because we have to index into the parts of the tuple.

It doesn’t matter if we mix up length and width for the area calculation, but
if we want to draw the rectangle on the screen, it would matter! We would have
to keep in mind that `length` is the tuple index `0` and `width` is the tuple
index `1`. If someone else worked on this code, they would have to figure this
out and keep it in mind as well. It would be easy to forget or mix up these
values and cause errors, because we haven’t conveyed the meaning of our data in
our code.

### Refactoring with Structs: Adding More Meaning

We use structs to add meaning by labeling the data. We can transform the tuple
we’re using into a data type with a name for the whole as well as names for the
parts, as shown in Listing 5-4:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 5-4: Defining a `Rectangle` struct</span>

Here we’ve defined a struct and named it `Rectangle`. Inside the `{}` we
defined the fields as `length` and `width`, both of which have type `u32`. Then
in `main` we create a particular instance of a `Rectangle` that has a length of
50 and a width of 30.

Our `area` function is now defined with one parameter, which we’ve named
`rectangle`, whose type is an immutable borrow of a struct `Rectangle`
instance. As mentioned in Chapter 4, we want to borrow the struct rather than
take ownership of it. This way, `main` retains its ownership and can continue
using `rect1`, which is the reason we use the `&` in the function signature and
where we call the function.

The `area` function accesses the `length` and `width` fields of the `Rectangle`
instance. Our function signature for `area` now indicates exactly what we mean:
calculate the area of a `Rectangle` using its `length` and `width` fields. This
conveys that the length and width are related to each other, and gives
descriptive names to the values rather than using the tuple index values of `0`
and `1`—a win for clarity.

### Adding Useful Functionality with Derived Traits

It would be helpful to be able to print out an instance of the `Rectangle`
while we’re debugging our program in order to see the values for all its
fields. Listing 5-5 uses the `println!` macro as we have been in earlier
chapters:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 5-5: Attempting to print a `Rectangle`
instance</span>

When we run this code, we get an error with this core message:

```text
error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
```

The `println!` macro can do many kinds of formatting, and by default, `{}`
tells `println!` to use formatting known as `Display`: output intended for
direct end user consumption. The primitive types we’ve seen so far implement
`Display` by default, because there’s only one way you’d want to show a `1` or
any other primitive type to a user. But with structs, the way `println!` should
format the output is less clear because there are more display possibilities:
do you want commas or not? Do you want to print the curly braces? Should all
the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we
want and structs don’t have a provided implementation of `Display`.

If we continue reading the errors, we’ll find this helpful note:

```text
note: `Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
```

Let’s try it! The `println!` macro call will now look like `println!("rect1 is
{:?}", rect1);`. Putting the specifier `:?` inside the `{}` tells `println!` we
want to use an output format called `Debug`. `Debug` is a trait that enables us
to print out our struct in a way that is useful for developers so we can see
its value while we’re debugging our code.

Run the code with this change. Drat! We still get an error:

```text
error: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
```

But again, the compiler gives us a helpful note:

```text
note: `Rectangle` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
```

Rust *does* include functionality to print out debugging information, but we
have to explicitly opt-in to make that functionality available for our struct.
To do that, we add the annotation `#[derive(Debug)]` just before the struct
definition, as shown in Listing 5-6:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 5-6: Adding the annotation to derive the `Debug`
trait and printing the `Rectangle` instance using debug formatting</span>

Now when we run the program, we won’t get any errors and we’ll see the
following output:

```text
rect1 is Rectangle { length: 50, width: 30 }
```

Nice! It’s not the prettiest output, but it shows the values of all the fields
for this instance, which would definitely help during debugging. When we have
larger structs, it’s useful to have output that’s a bit easier to read; in
those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
When we use the `{:#?}` style in the example, the output will look like this:

```text
rect1 is Rectangle {
    length: 50,
    width: 30
}
```

Rust has provided a number of traits for us to use with the `derive` annotation
that can add useful behavior to our custom types. Those traits and their
behaviors are listed in Appendix C. We’ll cover how to implement these traits
with custom behavior as well as how to create your own traits in Chapter 10.

Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle`
struct, because it won't work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.
