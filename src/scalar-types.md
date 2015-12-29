# Scalar Types

We’ve seen that every value in Rust has a type of some kind. There are a number
of types which are built into the language itself. First, we’ll take a look at
‘scalar’ types, that is, types which represent a single value.

Remember, you can rely on type inference to figure out the type of a binding,
or you can annotate it explicitly:

```rust
fn main() {
    let x: i32 = 5;
}
```

## Integers

You’ve already seen one primitive type: `i32`. There are a number of built-in
number types in Rust.

Here’s a chart of Rust’s integer types:

|        | signed | unsigned |
|--------|--------|----------|
|  8-bit |  i8    |  u8      |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

We have both signed and unsigned variants of numbers, and each variant has an
explicit size. Unsigned numbers are always positive, and signed numbers can be
positive or negative. (Think ‘plus sign’ or ‘minus sign’: that’s a signed
number.) Signed numbers are stored using ‘two’s compliment’ representation.

Finally, `isize` and `usize` are different sizes based on the kind of computer
your program is running on. If you are on a 64-bit architecture, they are 64
bits, and if you’re on a 32-bit one, they’re 32 bits.

So how do you choose from all these options? Well, if you really don’t know,
the defaults are a good choice: integer types default to `i32`. The primary use
case for `isize`/`usize` is when indexing some sort of collection. We’ll talk
more about our first collection, arrays, in just a moment.

## Floating-point numbers

Rust also has two primitive floating-point numbers: `f32` and `f64`. They are
32 bits and 64 bits in size, respectively. The default is `f64`.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard.
`f32` is a single-precision float, `f64` is double-precision.

## Booleans

Somewhat fundamental to all computing, Rust has a boolean type, `bool`, with
two possible values:

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explict type annotation
}
```

That’s really all there is to say about that!

## Characters

We’ve only worked with numbers so far, but what about letters? Rust’s most
primitive alphabetic type is the `char`:

```rust
fn main() {
   let c = 'z'; 
   let z = 'ℤ'; 
}
```

Rust’s `char` represents a [Unicode Scalar Value], which means that it can
represent a lot more than just ASCII. “Character” isn’t really a concept in
Unicode, however: your human intutition for what a ‘character’ is may not match
up with a `char`. It also means that `char`s are four bytes each.

[Unicode Scalar Value]: http://www.unicode.org/glossary/#unicode_scalar_value

The single quotes are important: to define a literal single character, we use
single quotes. If we used double quotes, we’d be defining a `&str`. Let’s talk
about that next!
