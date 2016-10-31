## Data Types

Every value in Rust is of a certain *type*, which tells Rust what kind of data
is being given so it knows how to work with that data. In this section, we'll
look at a number of types built into the language itself. We split the types
into two subsets: scalar and compound.

Something to keep in mind throughout this section: Rust is a *statically typed*
language, which means that it must know the types of all variables at compile
time. The compiler can usually infer what type we want to use based on the
value and how we use it. In cases when many types are possible, such as when we
converted a `String` to a numeric type using `parse` in Chapter 2, we must
add a type annotation, like this:

```rust
let guess: u32 = "42".parse().unwrap();
```

If we don't put the type annotation here, Rust will give us this error that
means the compiler needs more information from us to know which possible type
we want:

```bash
error: unable to infer enough type information about `_`; type annotations or
generic parameter binding required [--explain E0282]
 -->
  |>
3 |> let guess = "42".parse().unwrap();
  |>     ^^^^^
```

You will see some type annotations as we discuss the various data types.

### Scalar Types

A *scalar* type represents a single value. There are four primary scalar
types in Rust: integers, floating point numbers, booleans, and characters.
You'll likely recognize these from other programming languages, but let's jump
into how they work in Rust.

#### Integer Types

An *integer* is a number without a fractional component. We've used one integer
type already in this chapter, the `i32` type. This type declaration indicates
that the value it's associated with should be a signed integer (hence the `i`,
as opposed to a `u` for unsigned) for a 32-bit system. There are a number of
built-in integer types in Rust, shown in Table 3-1.

| Length | signed | unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

*Table 3-1: Integer types in Rust. Each variant in the signed and unsigned
columns (for example, `i32`) can be used to declare the type of an integer
value.*

Each variant can be either signed or unsigned and has an explicit size. Signed
and unsigned merely refers to whether it is possible for the number to be
either negative or positive; in other words, whether the number needs to have a
sign with it (signed), or whether it will only ever be positive and can
therefore be represented without a sign (unsigned). It's like writing numbers
on paper: when the sign matters, a number is shown with a plus sign or minus
sign, but when it's safe to assume the number is positive, it's shown with no
sign. Signed numbers are stored using two’s complement representation (if
you're unsure what this is you can search for it online; an explanation is
outside the scope of this text).

Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n -
1</sup> - 1 inclusive, where `n` is the number of bits that variant uses. So an
`i8` can store from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals -128
to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1, so a
`u8` can store from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.

Finally, the `isize` and `usize` types depend on the kind of computer your
program is running on: 64-bits if you're on a 64-bit architecture, and 32-bits
if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note that
all number literals except for the byte literal allow a type suffix, such as
`57u8`, and `_` as a visual separator, such as `1_000`.

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

*Table 3-2: Integer literals in Rust.*

So how do you know which type of integer to use? If you're unsure, Rust's
defaults are generally good choices, and integer types default to `i32`: it’s
generally the fastest, even on 64-bit systems. The primary situation in which
you'd use `isize` or `usize` is when indexing some sort of collection.

#### Floating-Point Types

Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust's floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`,
as it’s roughly the same speed as `f32`, but has a larger precision. It is
possible to use an `f64` on 32 bit systems, but it will be slower than using an
`f32` on those systems. Most of the time, trading potential worse performance
for better precision is a reasonable initial choice, and you should benchmark
your code if you suspect floating-point size is a problem in your case. See
Chapter XX for how to run benchmarks.

Here's an example showing floating-point numbers in action:

Filename: src/main.rs

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a single-precision float, while `f64` has double-precision.

#### Numeric Operations

Rust supports the usual basic mathematic operations you’d expect for all of
these number types: addition, subtraction, multiplication, division, and
remainder. This code shows how you'd use each one in a `let` statement:

Filename: src/main.rs

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
to a single value, which is then bound to a variable. Appendix XX contains a
list of all operators that Rust provides.

#### The Boolean Type

As in most other programming languages, a boolean type in Rust has two possible
values: `true` and `false`. The boolean type in Rust is specified with `bool`.
For example:

Filename: src/main.rs

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

The main way to consume boolean values is through conditionals like an `if`
statement. We’ll cover how `if` statements work in Rust in the "Control Flow"
section of this chapter.

#### The Character Type

So far we’ve only worked with numbers, but Rust supports letters too. Rust’s
`char` type is the language's most primitive alphabetic type, and this code
shows one way to use it:

Filename: src/main.rs

```rust
fn main() {
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';
}
```

Rust’s `char` represents a Unicode Scalar Value, which means that it can
represent a lot more than just ASCII. Accented letters, Chinese/Japanese/Korean
ideographs, emoji, and zero width spaces are all valid `char`s in Rust. Unicode
Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF`
inclusive. A "character" isn’t really a concept in Unicode, however, so your
human intuition for what a "character" is may not match up with what a `char`
is in Rust. We'll discuss this in detail in the Strings section of Chapter 8.

### Compound Types

*Compound types* can group multiple values of other types into one type. Rust
has two primitive compound types: tuples and arrays.

#### Grouping Values into Tuples

A tuple is a general way of grouping together some number of other values with
distinct types into one compound type.

We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a distinct type, and the types of
the different values in the tuple do not have to be the same. We've added
optional type annotations in this example:

Filename: src/main.rs

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple, since a tuple is considered a
single compound element. To get the individual values out of a tuple, we can
use pattern matching to destructure a tuple value, like this:

Filename: src/main.rs

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

In this program, we first create a tuple and bind it to the variable `tup`. We
then use a pattern with `let` to take `tup` and turn it into three separate
variables, `x`, `y`, and `z`. This is called *destructuring*, because it breaks
the single tuple into three parts. Finally, we print the value of `y`, which is
`6.4`.

#### Tuple Indexing

In addition to destructuring through pattern matching, we can also access a
tuple element directly by using a period (`.`) followed by the index of the
value we want to access. For example:

Filename: src/main.rs

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates a tuple, `x`, and then makes new variables for each element
by using their index. As with most programming languages, the first index in a
tuple is 0.

### Arrays

Another way to have a collection of multiple values is with an
*array*. Unlike a tuple, every element of an array must have the same type.
Arrays in Rust are different than arrays in some other languages because arrays
in Rust have a fixed length: once declared, they cannot grow or shrink in size.

In Rust, the values going into an array are written as a comma separated list
inside square brackets:

Filename: src/main.rs

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

While arrays can be useful since they are a primitive type so using them can be
very fast, they aren't as flexible as the vector type. The vector type is a
similar collection type provided by the standard library that *is* allowed to
grow or shrink in size. If you're unsure whether to use an array or a vector,
you should probably go with a vector, and we'll discuss them in more detail in
Chapter 8.

An example of when we might want to use an array is storing the months of the
year. It's very unlikely that our program will need to add or remove months, so
we can use an array since we know we will always have 12 items:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

#### Accessing Array Elements

An array is a single chunk of memory, allocated on the stack. We can access
elements of an array using indexing, like this:

Filename: src/main.rs

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named `first` will get the value `1`, since that
is the value at index `[0]` in the array. The variable named `second` will get
the value `2` from index `[1]` in the array.

#### Invalid Array Element Access

What happens if you try to access an element of an array past the end of the
array? Say we changed our program to:

Filename: src/main.rs

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];

    let element = a[10];

    println!("The value of element is: {}", element);
}
```

Running this code with `cargo run` produces:

```bash
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
     Running `target/debug/arrays`
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is
10', src/main.rs:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/arrays` (exit code: 101)
```

We can see that the compilation did not give us any errors, but we got a
*runtime* error and our program didn't exit successfully. When we attempt to
access an element using indexing, Rust will check that the index we've
specified is less than the array length. If the index is greater than the
length, it will "panic", which is what it's called when a Rust program exits
with an error.

This is our first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects us against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. We'll discuss more of Rust’s error handling in Chapter XX.
