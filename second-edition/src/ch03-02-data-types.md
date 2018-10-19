## Data Types

Every value in Rust is of a certain *data type*, which tells Rust what kind of
data is being specified so it knows how to work with that data. We’ll look at
two data type subsets: scalar and compound.

Keep in mind that Rust is a *statically typed* language, which means that it
must know the types of all variables at compile time. The compiler can usually
infer what type we want to use based on the value and how we use it. In cases
when many types are possible, such as when we converted a `String` to a numeric
type using `parse` in the “Comparing the Guess to the Secret Number” section in
Chapter 2, we must add a type annotation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the type annotation here, Rust will display the following
error, which means the compiler needs more information from us to know which
type we want to use:

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

You’ll see different type annotations for other data types.

### Scalar Types

A *scalar* type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters. You may recognize
these from other programming languages. Let’s jump into how they work in Rust.

#### Integer Types

An *integer* is a number without a fractional component. We used one integer
type in Chapter 2, the `u32` type. This type declaration indicates that the
value it’s associated with should be an unsigned integer (signed integer types
start with `i`, instead of `u`) that takes up 32 bits of space. Table 3-1 shows
the built-in integer types in Rust. Each variant in the Signed and Unsigned
columns (for example, `i16`) can be used to declare the type of an integer
value.

<span class="caption">Table 3-1: Integer Types in Rust</span>

| Length | Signed  | Unsigned |
|--------|---------|----------|
| 8-bit  | `i8`    | `u8`     |
| 16-bit | `i16`   | `u16`    |
| 32-bit | `i32`   | `u32`    |
| 64-bit | `i64`   | `u64`    |
| arch   | `isize` | `usize`  |

Each variant can be either signed or unsigned and has an explicit size.
*Signed* and *unsigned* refer to whether it’s possible for the number to be
negative or positive—in other words, whether the number needs to have a sign
with it (signed) or whether it will only ever be positive and can therefore be
represented without a sign (unsigned). It’s like writing numbers on paper: when
the sign matters, a number is shown with a plus sign or a minus sign; however,
when it’s safe to assume the number is positive, it’s shown with no sign.
Signed numbers are stored using two’s complement representation (if you’re
unsure what this is, you can search for it online; an explanation is outside
the scope of this book).

Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n -
1</sup> - 1 inclusive, where *n* is the number of bits that variant uses. So an
`i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals
-128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1,
so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.

Additionally, the `isize` and `usize` types depend on the kind of computer your
program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits
if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note
that all number literals except the byte literal allow a type suffix, such as
`57u8`, and `_` as a visual separator, such as `1_000`.

<span class="caption">Table 3-2: Integer Literals in Rust</span>

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

So how do you know which type of integer to use? If you’re unsure, Rust’s
defaults are generally good choices, and integer types default to `i32`: this
type is generally the fastest, even on 64-bit systems. The primary situation in
which you’d use `isize` or `usize` is when indexing some sort of collection.

#### Floating-Point Types

Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust’s floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`
because on modern CPUs it’s roughly the same speed as `f32` but is capable of
more precision.

Here’s an example that shows floating-point numbers in action:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a single-precision float, and `f64` has double precision.

#### Numeric Operations

Rust supports the basic mathematical operations you’d expect for all of the
number types: addition, subtraction, multiplication, division, and remainder.
The following code shows how you’d use each one in a `let` statement:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable. Appendix B contains a
list of all operators that Rust provides.

#### The Boolean Type

As in most other programming languages, a Boolean type in Rust has two possible
values: `true` and `false`. The Boolean type in Rust is specified using `bool`.
For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to use Boolean values is through conditionals, such as an `if`
expression. We’ll cover how `if` expressions work in Rust in the “Control Flow”
section.

#### The Character Type

So far we’ve worked only with numbers, but Rust supports letters too. Rust’s
`char` type is the language’s most primitive alphabetic type, and the following
code shows one way to use it. (Note that the `char` type is specified with
single quotes, as opposed to strings, which use double quotes.)

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust’s `char` type represents a Unicode Scalar Value, which means it can
represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and
Korean characters; emoji; and zero-width spaces are all valid `char` values in
Rust. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to
`U+10FFFF` inclusive. However, a “character” isn’t really a concept in Unicode,
so your human intuition for what a “character” is may not match up with what a
`char` is in Rust. We’ll discuss this topic in detail in “Strings” in Chapter 8.

### Compound Types

*Compound types* can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.

#### The Tuple Type

A tuple is a general way of grouping together some number of other values with
a variety of types into one compound type.

We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a type, and the types of the
different values in the tuple don’t have to be the same. We’ve added optional
type annotations in this example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple, because a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

This program first creates a tuple and binds it to the variable `tup`. It then
uses a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring*, because it breaks
the single tuple into three parts. Finally, the program prints the value of
`y`, which is `6.4`.

In addition to destructuring through pattern matching, we can access a tuple
element directly by using a period (`.`) followed by the index of the value we
want to access. For example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates a tuple, `x`, and then makes new variables for each
element by using their index. As with most programming languages, the first
index in a tuple is 0.

#### The Array Type

Another way to have a collection of multiple values is with an *array*. Unlike
a tuple, every element of an array must have the same type. Arrays in Rust are
different from arrays in some other languages because arrays in Rust have a
fixed length: once declared, they cannot grow or shrink in size.

In Rust, the values going into an array are written as a comma-separated list
inside square brackets:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than
the heap (we will discuss the stack and the heap more in Chapter 4) or when
you want to ensure you always have a fixed number of elements. An array isn’t
as flexible as the vector type, though. A vector is a similar collection type
provided by the standard library that *is* allowed to grow or shrink in size.
If you’re unsure whether to use an array or a vector, you should probably use a
vector. Chapter 8 discusses vectors in more detail.

An example of when you might want to use an array rather than a vector is in a
program that needs to know the names of the months of the year. It’s very
unlikely that such a program will need to add or remove months, so you can use
an array because you know it will always contain 12 items:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

##### Accessing Array Elements

An array is a single chunk of memory allocated on the stack. You can access
elements of an array using indexing, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1`, because
that is the value at index `[0]` in the array. The variable named `second` will
get the value `2` from index `[1]` in the array.

##### Invalid Array Element Access

What happens if you try to access an element of an array that is past the end
of the array? Say you change the example to the following code, which will
compile but exit with an error when it runs:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```

Running this code using `cargo run` produces the following result:

```text
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
 10', src/main.rs:6
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

The compilation didn’t produce any errors, but the program resulted in a
*runtime* error and didn’t exit successfully. When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than the length, Rust will
*panic*, which is the term Rust uses when a program exits with an error.

This is the first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects you against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. Chapter 9 discusses more of Rust’s error handling.
