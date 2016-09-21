
[TOC]

# Structs

<!--- So is a struct a bit like a tuple whose values you can give a label to?
That's kind of what I gathered from later on. If so, do you want to mention
something along those lines here? -->

A `struct`, short for "structure", is a data type that lets us name and package
together multiple related values that make up a meaningful group. If you come
from an object-oriented language, a `struct` is like an object's data
attributes. The `struct` and `enum` (that we will talk about in the next
chapter) concepts are the building blocks for creating new types in your
program's domain in order to take full advantage of Rust's compile-time type
checking.

## An Example Program

To examine structs, let’s write a program that calculates the distance between
two points. We’ll start off with single variable bindings, and then refactor
the program to use `struct`s instead.

Make a new project with Cargo:

```bash
$ cargo new --bin points
$ cd points
```

Here’s a short program that calculates the distance between two points. Enter
the following into your `src/main.rs`:

Filename: src/main.rs

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

Now try running this program with `cargo run` and as output you'll get:

<!-- Did you want to start taking out the compiling output, when showing
output? So we could just show it from Point 1: (0,5) ... -->

```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
     Running `target/debug/points`
Point 1: (0, 5)
Point 2: (12, 0)
Distance: 13
```

Let's take a quick look at the `distance()` fuction before we move forward. To
find the distance between two points, we use the Pythagorean Theorem, named
after Pythagoras, the first person to mathematically prove this formula. The
details aren't that important; just know the theorem says that the formula for
the distance between two points is equal to:

- squaring the distance between the points horizontally (the "x" direction)
- squaring the distance between the points vertically (the "y" direction)
- adding those together
- and taking the square root of that.

So that's what we're implementing here.

```rust,ignore
f64::powi(2.0, 3)
```

The double colon (`::`) here is a namespace operator. The `powi` function
raises some number to a specified power. We haven’t talked about namespaces in
depth yet, but you can think of the `powi()` function as being scoped inside of
another name. In this case, the name is `f64`, the same as the type.

<!--- Is it worth breifly saying what it means for powi to be scoped inside f64
here, on a practical level? Does it mean that powi will only work here for f64
types? I'm not clear on that -->

The `powi()` function takes two arguments: the first is a number, and the
second is the power that it raises that number to. In this case, the second
number is an integer, hence the `i` in `powi()`.

<!-- If it weren't an integer, say it were a floating point, what would this be
instead? -->

Similarly, `sqrt()` is a function under the `f64` name, which takes the square
root of its argument.

### Refactoring with Tuples

Our little program works okay; it figures out the distance between two points
using the Pythagorean theorem. But we can do better. The issue with this method
is evident in the signature of `distance()`:

```rust,ignore
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
```

The distance function is supposed to calculate the distance between two points,
but our distance function calculates some distance between four numbers. The
first two and last two arguments are related, but that’s not expressed anywhere
in our program itself. It would be more readable and more manageable if we had
a way to group `(x1, y1)` and `(x2, y2)` together.

We’ve already discussed one way we might do that: tuples. Here’s a version of
our program which uses tuples:

Filename: src/main.rs

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

In one way, this is a little better. Tuples let us add a little bit of
structure, and we’re now passing just two arguments. But in another way this
method less clear: tuples don’t give names to their elements, so our
calculation has gotten more confusing because we have to use the tuple's index:

<!-- Is that right, we're using the index here? -->

```rust,ignore
p2.0 - p1.0
p2.1 - p1.1
```

When writing this example, your authors almost got it wrong themselves!
Distance is all about `x` and `y` points, but our code is talking about `0` and
`1`. This isn’t great programming.

### Refactoring with Structs: the Right Way

Here is where we bring in `struct`s. We can transform our tuples into a data
type with a name for the whole as well as names for the parts:

```rust,ignore
let p1 = (0.0, 5.0);

struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 0.0, y: 5.0 };
```

Here we've defined a `struct` and given it the name `Point`. Inside the `{}` we
are defining the _fields_ of the struct, each of which will hold one piece of
data. We can have as many or as few fields as we'd like, and we give each a
name and specify its type. Here we have two fields named `x` and `y`, and they
both hold `f64`s.

<!-- Above --- is that right, 1 field = 1 piece of data? Also, this presumably
means a struct can have different types, if we have to specify each one
individually? I don't think we've mentioned that in the chapter, seems
important?-->

In the same way that we can access an element of a tuple, we can access any
element of a struct but we use its name rather than its index:

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

<!-- I'm not 100% sure what this is showing, could you give a few lines of
explanation afterwards? -->

Let’s convert our program to use our `Point` struct. Here’s what it should look
like now:

Filename: src/main.rs

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

Our function signature for `distance()` now says exactly what we mean:
calculate the distance between two `Point`s, using `x` and `y` rather than `0`
and `1`. This is a win for clarity.

## Derived Traits

You may have noticed something else new in this last example: this line above
the `struct` declaration:

```rust,ignore
#[derive(Debug,Copy,Clone)]
struct Point {
```

This is an annotation that gives the compiler instructions about how we want
our struct formatted, by giving some default behavior for the `Debug`, `Copy`,
and `Clone` traits. We talked about marking that types can be `Copy` and
`Clone`-able in Chapter XX when we discussed ownership. <!-- So are we telling
the compiler that the struct should be copy and clone here? Can you just say
what those aspects are doing, explicitly? -->

`Debug` is a trait that enables us to print out our struct so that we can see
its value while we are debugging our code.

The alternative would be the `println!` macro we've used so far, but here we
use this debugging trait instead because if we try `println!` with a struct
we'll get an error. This is because the `println!` function has some default
formatting instructions that can only apply to basic data types, and our struct
is more complicated. As as example, say we have the following program:

Filename: src/main.rs

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

This code tries to print the `p1` point directly, which may seem innocuous, but
running it produces an error, the core message of which is *the trait bound
`Point: std::fmt::Display` is not satisfied*. The `println!` function can do
many kinds of formatting and by default, `{}` implements a kind of formatting
known as `Display`: output intended for direct end-user consumption. The
primitive types

<!--- Above, do we mean {} in conjunction with println! produces this kind of
formatting? -->

we’ve seen so far implement `Display` by default, as there’s only one way you’d
want to show a `1` or any other primitive type to a user. But with structs, the
output is less clear as there are more display options: Do you want commas or
not? Do you want to print the `{}`s? Should all the fields be shown?

More complex types in the standard library and those that are defined by the
programmer do not automatically implement `Display` formatting for this reason.
Instead they implement `Debug` formatting, which is intended for the programmer
to see. The `#[derive(Debug)]` annotation lets us use a default implementation
of `Debug` formatting to easily get this ability for types we've defined. To
ask `println!` to use `Debug` formatting with our `Point` struct, we add the
annotation to derive the trait and include `:?` in the print string, like this:

<!-- So are we saying we include the syntax :? in the print string in order to
apply the Debug formatting? I'm not qutie clear -->

Filename: src/main.rs

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

You’ll see this repeated later with other types, and we’ll cover traits fully
in Chapter XX.

## Method Syntax

<!-- This seems like a bit of a segue, is there a reason we include this method
section here? If it's arbitrary, we might want to look at the arrangement of
the chapters, there ought to be a logical arrangement so the reader can
navigate easily. Otherwise, can you include some kind of introduction that says
why we're going into methods here? -->

In Chapter 4 when we discussed ownership, we made several references to
*methods*. Here's an example of the `clone` method in use:

```rust
let s1 = "hello";

// call a method on s1
let s2 = s1.clone();

println!("{}", s1);
```

The call to `clone()` is attached to `s1` with a dot. This is the *method
syntax*, and it’s a way to call certain functions with a different style.

<!-- Can you expand on what we mean by a different style? I'm not sure this
sections sells methods fully, is there no other reason you'd use a method that
for easier nesting? It doesn't seem like we go into deeper, ownership reasons,
like we say we will below -->

Why have two ways to call functions? We’ll talk about some deeper reasons
related to ownership in a moment, but one big reason is that methods are much
more readable when chained together than functions. Here's the same chaining
example using both methods and functions:

```rust,ignore
// with functions
h(g(f(x)));

// with methods
x.f().g().h();
```

The nested-functions version is read by Rust in reverse: the program executes
`f()`, then `g()`, then `h()`, but we read it left-to-right as `h()`, then
`g()`, then `f()`. This could be confusing if you need your functions exectuted
in a specific order.

The method syntax on the other hand is executed in the same order as we would
read it, and is listed rather than nested to make it much easier to read.

Before we get into the details, let’s talk about how to define your own methods.

### Defining Methods

We can define methods with the `impl` keyword, short for *implementation*.
Doing so looks like this:

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

Let’s break this down. First, we define our `Point` struct from earlier in the
chapter. Next comes our first use of the `impl` keyword, followed by a call to
`Point`:

```rust,ignore
impl Point {
    // ...
}
```

<!-- We might want to use the wingding numbers here too, to save repetition of
code in the text and put the explanations in context. -->

Everything we put inside the curly braces here is a method to be implemented on
`Point`. Next we give our definition of the method, here doing the same as our
`distance` fuction from earlier.

Other than this, the rest of the example is familiar: an implementation of
`distance()` and use of the method to find an answer.

Our definition of `distance()` here as a method looks very similar to our
previous definition of `distance()` as a function, but with two differences.
Here's the `distance()` function again followed by our new `distance()` method:

```rust,ignore
fn distance(p1: Point, p2: Point) -> f64 {
    // ...
}

fn distance(&self, other: &Point) -> f64 {
    // method
    // ...
}
```

<!-- What's the second difference? I can't see that we go over it, unless you
mean the assert line? In which case, we might need to rephrase, this reads like
there are two differences in this line alone-->

The first difference is in the first argument. In the method version we replace
the name and type with `&self`. This is the main distinction from a function;
we use `self` inside of an `impl` block in a method because we already know
that we are implementing this method on `Point`, due to the surrounding `impl
Point` block, so we don’t need to write the type of `self` out again.

<!-- Have we mentioned `self` before here in the book? If not, I'd suggest
going into a little more detail, I'm not sure this makes it clear what self is
and how you'd use it in other situations. Especially since in these two
examples we seem to be giving the type, f64, in both anyway? Is this a
siginificant enough concept to warrant its own section? If it isn't that
significant, I wonder if we want to make this self section a box, it seems
quite abrupt here? -->

Note that we have written `&self`, with the reference syntax, rather than
`self`, because we want to take a reference to our argument's value rather than
taking ownership of it.

<!-- Do you want to point out here why we're taking a reference and not
ownership in this example? -->

In other words, these two forms are the same:

```rust,ignore
fn foo(self: &Point)
fn foo(&self)
```

In both we are taking a reference of `Point`.

Self is a parameter like any other, and as such you can take `self` in three
forms:

```rust,ignore
fn foo(&self) // take self by reference
fn foo(&mut self) // take self by mutable reference
fn foo(self) // take self by ownership
```

<!-- It might help to explain why it's so much more common to take self by
immutable reference that any other way? -->

Taking `self` by reference is the most common, followed by mutable reference,
and the least common is taking `self` by ownsership. In this case, we only need
a reference of `Point`, and we don’t need to mutate either `Point` to get the
distance between them, so we won't take a mutable reference to . Methods that
take ownership of `self` are rarely used. One of the few times we might take
ownership of `self` would be if we needed a method that would transform `self`
into something else and prevent other code from using the value of `self` after
the transformation happens.

<!-- We haven't mentioned the `other` terminology here, but I think that's new
too, right? Could you add an explanation? I think we ought to, since it's new,
but if you are wanting to save it for another chapter maybe just mention that,
so the reader's not left wondering. -->

#### Methods and Automatic Referencing

There's another new bit of information in this last script, this last line of
the example:

```rust,ignore
assert_eq!(8.200609733428363, p1.distance(&p2));
```

<!-- what is this final line, what does the assert section mean/do? What's the
long floating point for? -->

When we defined `distance()`, we took both `self` and the `other` argument by
reference. Yet, in this final line, we needed a `&` for `p2` but not `p1`. What
gives?

This feature is called *automatic referencing*, and calling methods is one of
the few places in Rust that has behavior like this. When you call a method with
`self.(`, Rust will automatically add in `&`s or `&mut`s to match the
signature. In other words, these are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much, much cleaner. <!-- I think I'm following, but why do
we add the reference to the second entry, p2, and not p1? -->

Here’s another example:

```rust
let mut s = String::from("Hello,");

s.push_str(" world!");

// The above is the same as:
// (&mut s).push_str(" world!");

assert_eq!("Hello, world!", s);
```

<!-- is this an unfinished sentence below? Got a bit lost! -->

Because `push_str()` has the following signature:

```rust,ignore
fn push_str(&mut self, string: &str) {
```

This automatic referencing behavior works because methods have a clear receiver
— the type of `self` — and in most cases it’s clear given the receiver and name
of a method whether the method is just reading (so needs `&self`), mutating (so
`&mut self`), or consuming (so `self`). The fact that Rust makes borrowing
implicit for method receivers is a big part of making ownership ergonomic in
practice.

<!-- I dd find this automatic referencing section quite hard to read, I think
it could use a little fleshing out to make it clearer, talk through the
examples a bit more, that kind of thing. -->

<!-- Could you give a chapter summary? -->
