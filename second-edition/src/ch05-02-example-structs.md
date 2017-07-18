## An Example Program Using Structs

To understand when we might want to use structs, let’s write a program that
calculates the area of a rectangle. We’ll start with single variables, and then
refactor the program until we’re using structs instead.

Let’s make a new binary project with Cargo called *rectangles* that will take
the length and width of a rectangle specified in pixels and will calculate the
area of the rectangle. Listing 5-8 shows a short program with one way of doing
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

<span class="caption">Listing 5-8: Calculating the area of a rectangle
specified by its length and width in separate variables</span>

Now, run this program using `cargo run`:

```text
The area of the rectangle is 1500 square pixels.
```

### Refactoring with Tuples

Even though Listing 5-8 works and figures out the area of the rectangle by
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
manageable to group length and width together. We’ve already discussed one way
we might do that in the Grouping Values into Tuples section of Chapter 3 on
page XX: by using tuples. Listing 5-9 shows another version of our program that
uses tuples:

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

<span class="caption">Listing 5-8: Specifying the length and width of the
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
parts, as shown in Listing 5-10:

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

<span class="caption">Listing 5-10: Defining a `Rectangle` struct</span>

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
fields. Listing 5-11 uses the `println!` macro as we have been in earlier
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

<span class="caption">Listing 5-11: Attempting to print a `Rectangle`
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
definition, as shown in Listing 5-12:

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

<span class="caption">Listing 5-12: Adding the annotation to derive the `Debug`
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
struct, because it won’t work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.
