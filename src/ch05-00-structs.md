# Structs

`struct`s, short for "structures", give us the ability to name and package
together multiple related values that make up a meaningful group. If you come
from an object-oriented language, `struct`s are like an object's data
attributes. `structs`, along with `enum`s that we talked about in the last
chapter, are the building blocks you can use in Rust to create new types in
your program's domain in order to take full advantage of Rust's compile-time
type checking.

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

Let's take a quick look at `distance()` before we move forward:

```rust
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let x_squared = f64::powi(x2 - x1, 2);
    let y_squared = f64::powi(y2 - y1, 2);

    f64::sqrt(x_squared + y_squared)
}
```

To find the distance between two points, we can use the Pythagorean Theorem.
The theorem is named after Pythagoras, who was the first person to mathematically
prove this formula. The details aren't that important, to be honest. There's a few
things that we haven't discussed yet, though.

```rust,ignore
f64::powi(2.0, 3)
```

The double colon (`::`) here is a namespace operator. We haven’t talked about
modules yet, but you can think of the `powi()` function as being scoped inside
of another name. In this case, the name is `f64`, the same as the type. The
`powi()` function takes two arguments: the first is a number, and the second is
the power that it raises that number to. In this case, the second number is an
integer, hence the ‘i’ in its name. Similarly, `sqrt()` is a function under the
`f64` module, which takes the square root of its argument.

## Why `struct`s?

Our little program is okay, but we can do better. The key is in the signature
of `distance()`:

```rust,ignore
fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
```

The distance function is supposed to calculate the distance between two points.
But our distance function calculates some distance between four numbers. The
first two and last two arguments are related, but that’s not expressed anywhere
in our program itself. We need a way to group `(x1, y1)` and `(x2, y2)`
together.

We’ve already discussed one way to do that: tuples. Here’s a version of our program
which uses tuples:

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
We’re now passing two arguments, so that’s more clear. But it’s also worse.
Tuples don’t give names to their elements, and so our calculation has gotten
much more confusing:

```rust,ignore
p2.0 - p1.0
p2.1 - p1.1
```

When writing this example, your authors almost got it wrong themselves! Distance
is all about `x` and `y` points, but now it’s all about `0` and `1`. This isn’t
great.

Enter `struct`s. We can transform our tuples into something with a name:

```rust,ignore
let p1 = (0.0, 5.0);

struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 0.0, y: 5.0 };
```

Here’s what declaring a `struct` looks like:

```text
struct NAME {
    NAME: TYPE,
}
```

The `NAME: TYPE` bit is called a ‘field’, and we can have as many or as few of
them as you’d like. If you have none of them, drop the `{}`s:

```rust
struct Foo;
```

`struct`s with no fields are called ‘unit structs’, and are used in certain
advanced situations. We will just ignore them for now.

You can access the field of a struct in the same way you access an element of
a tuple, except you use its name:

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

There’s one other thing that’s a bit strange here, this annotation on our
`struct` declaration:

```rust,ignore
#[derive(Debug,Copy,Clone)]
struct Point {
```

We haven’t yet talked about traits, but we did talk about `Debug` when we
discussed arrays. This `derive` attribute allows us to tweak the behavior of
our `Point`. In this case, we are opting into copy semantics, and everything
that implements `Copy` must implement `Clone`.
