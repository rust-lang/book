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

The `powi()` function takes two arguments: the first is a
number, and the second is the power that it raises that number to. In this
case, the second number is an integer, hence the `i` in `powi()`.

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

When writing this example, your authors almost got it wrong themselves! Distance
is all about `x` and `y` points, but our code is talking about `0` and `1`.
This isn’t great programming.

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
`Clone`-able in Chapter XX when we discussed ownership.

<!-- So are we telling the compiler that the struct should be copy and clone
here? Can you just say what those aspects are doing, explicitly? -->

`Debug` is a trait that enables us to print out our
struct so that we can see its value while we are debugging our code.

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
running it produces an error, the core message of which is

<!--- ```bash
$ cargo run
   Compiling points v0.1.0 (file:///projects/points)
error: the trait bound `Point: std::fmt::Display` is not satisfied [--explain E0277]
 -->

 <!-- src/main.rs:8:29
8 |>     println!("Point 1: {}", p1);
  |>                             ^^
<std macros>:2:27: 2:58: note: in this expansion of format_args!
<std macros>:3:1: 3:54: note: in this expansion of print! (defined in <std macros>)
src/main.rs:8:5: 8:33: note: in this expansion of println! (defined in <std macros>)
note: `Point` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
note: required by `std::fmt::Display::fmt`
```

Whew! The core of the error is:-->

<!-- Do we need to print the whole output? If not, I'd suggest just included
the important error line, draw more attention to it -->

*the trait bound `Point: std::fmt::Display` is not satisfied*. The `println!`
function can do many kinds of formatting and by default, `{}` implements a kind
of formatting known as `Display`: output intended for direct end-user
consumption. The primitive types

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

You’ll see this repeated later with other types, and we’ll cover traits fully in
Chapter XX.
