# Primitive Types

We’ve seen that every value in Rust has a type of some kind.
There are a number of types which are built into the language itself.
We call these types ‘primitive’ types, since you can’t re-create them yourself.
There are, of course, many non-primitive types provided by the standard library as well.

Remember, you can rely on type inference to figure out the type of a binding, or you can annotate it explicitly:

```rust
fn main() {
    let x: i32 = 5;
}
```

## Integers

You’ve already seen one primitive type: `i32`.
There are a number of built-in number types in Rust.

Here’s a chart of Rust’s integer types:

|        | signed | unsigned |
|--------|--------|----------|
|  8-bit |  i8    |  u8      |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

We have both signed and unsigned variants of numbers, and each variant has an explicit size.
Unsigned numbers are always positive, and signed numbers can be positive or negative.
(Think ‘plus sign’ or ‘minus sign’: that’s a signed number.)
Signed numbers are stored using ‘two’s compliment’ representation.

Finally, `isize` and `usize` are different sizes based on the kind of computer your program is running on.
If you are on a 64-bit architecture, they are 64 bits, and if you’re on a 32-bit one, they’re 32 bits.

So how do you choose from all these options? Well, if you really don’t know, the defualts are a good choice:
integer types default to `i32`.
The primary use case for `isize`/`usize` is when indexing some sort of collection.
We’ll talk more about our first collection, arrays, in just a moment.

## Floating-point numbers

Rust also has two primitive floating-point numbers: `f32` and `f64`.
They are 32 bits and 64 bits in size, respectively.
The default is `f64`.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard.
`f32` is a single-precision float, `f64` is double-precision.

## Tuples

The other type we’ve seen previously is the tuple type.
Tuples have an ‘arity’, or size.
We might say “that’s a 3-tuple” or “that’s a 5-tuple.”

Each position in a tuple has a distinct type:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
}
```

Tuples are used sparingly in Rust code.
This is because the elements of a tuple are anonymous, which can make code hard to read.

### Tuple indexing

To access an element of a tuple, we use a `.` followed by the index we want to access:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

As you can see, the first index is `0`.

### Single-element tuples

There’s one last trick with tuples: `(5)` is actually ambiguous: is it a tuple, or is it a `5` in parethesis?
If you need to disambiguate, use a comma:

```rust
fn main() {
    let x = (5); // x is an i32, no tuple. Think of it like (5 + 1) without the + 1, they’re for grouping.

    let x = (5,); // x is a (i32), a tuple with one element.
}
```

## Functions

There’s one more type that we’ve been using, but you haven’t seen written explicitly.
Functions!
Functions also have a type, and yes, you can even have variables which hold functions!
Here’s an example:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let f = plus_one;
    let g: fn(i32) -> i32 = plus_one; // with an explicit type annotation

    let five = f(4);
}
```

As you can see, the type is very similar to the declaration.
Here, let’s put them side by side:

```rust,ignore
fn(i32) -> i32 // type
fn plus_one(x: i32) -> i32 { // declaration
```

If we take the declaration, and drop the name...

```rust,ignore
fn(i32) -> i32 // type
fn(x: i32) -> i32 {
```

And then drop the names of the arguments...

```rust,ignore
fn(i32) -> i32 // type
fn(i32) -> i32 {
```

It’s the same! Well, we need to drop that `{` as well.

Finally, if you’ll notice in that example, we can create a binding with a function in it:

```rust,ignore
fn main() {
    let f = plus_one;

    let five = f(4);
}
```

... and call it with `()`s just like if we had used the original name.

### Functions as arguments

So why not just use the original name?
Well, we can pass functions as arguments to other functions!
Check this out:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    x + 2
}

fn twice(x: i32, f: fn(i32) -> i32) -> i32 {
    let mut result = x;

    result = f(result);
    result = f(result);

    result 
}

fn main() {
    let x = 5;

    let y = twice(x, plus_one);
    let z = twice(x, plus_two);

    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
```

If we compile and run this, we’ll get this output:

```text
The value of y is: 7
The value of z is: 9
```

Let’s investigate in more detail.

```rust,ignore
fn twice(x: i32, f: fn(i32) -> i32) -> i32 {
```

This says “`twice()` is a function which takes two arguments.
`x` is a thirty-two bit integer, and `f` is a function which takes an `i32` and returns an `i32`.”

Inside of `twice()`, as you might imagine, we call the function `f` twice on `x`, and return the result.


```rust,ignore
let y = twice(x, plus_one);
let z = twice(x, plus_two);
```

The first time we call `twice()`, we pass `plus_one()` as an argument.
And `x` is `5`.
So `5 + 1 + 1 == 7`, hence our first line of output.
The second time, we pass `plus_two()` instead.
`5 + 2 + 2` is `9`, and our second line checks out too.

Passing functions to functions is very, very powerful.

## Booleans

Somewhat fundamental to all computing, Rust has a boolean type, `bool`, with two possible values:

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explict type annotation
}
```

That’s really all there is to say about that!

## Arrays

## Slices

## char

## str

