## Data Types in Rust

Every value in Rust is of a certain *type*, which tells Rust what kind of data
is being given so it knows how to work with that data. You can usually rely on
Rust's ability to infer types to figure out the type of a binding, or you can
annotate it explicitly if needed. In this section, we'll look at a number of
types built into the language itself split into two subsets of Rust data types:
scalar and compound. First, let's look at how Rust deals with types.

### Type Inference and Annotation

Rust is a *statically typed* language, which means that it must know the types
of all bindings at compile time. However, you may have noticed that we didn’t
declare a type for `x` or `y` in our previous examples.

This is because Rust can often tell the type of a binding without you having to
declare it. Annotating every single binding with a type can take uneccesary
time and make code noisy. To avoid this, Rust uses *type inference*, meaning
that it attempts to figure out the types of your bindings by looking at how the
binding is used. Let’s look at the the first `let` statement you wrote again:

```rust
fn main() {
    let x = 5;
}
```

When we bind `x` to `5`, the compiler determines that `x` should be a numeric
type based on the value it is bound to. Without any other information, it sets
the `x` variable's type to `i32` (a thirty-two bit integer type) by default.

If we were to declare the type with the variable binding, that would be called
a *type annotation*. A `let` statement that includes a type annotation would
look like this:

```text
let PATTERN: TYPE = VALUE;
```

The `let` statement now has a colon after the `PATTERN`, followed by the `TYPE`
name. Note that the colon and the `TYPE` go _after_ the `PATTERN`, not inside
the pattern itself. Given this structure, here's how you'd rewrite `let x = 5`
to use type annotation:

```rust
fn main() {
    let x: i32 = 5;
}
```

This does the same thing as `let x = 5` but explicitly states that `x` should
be of the `i32` type. This is a simple case, but more complex patterns with
multiple bindings can use type annotation, too. A binding with two variables
would look like this:

```rust
fn main() {
    let (x, y): (i32, i32) = (5, 6);
}
```

In the same way as we place the `VALUE` and the `PATTERN` in corresponding
positions, we also match up the position of the `TYPE` with the `PATTERN` it
corresponds to.

There are times when multiple types could be correct, and there is not enough
information in the surrounding context for Rust to be able to tell which type
you want to use. In those cases type annotations are required. We will look at
some of those situations later, but for now, let's look at the types available
in Rust.

### Scalar Types

A *scalar* type is one that represents a single value. There are four key
scalar types in Rust: integers, floating point numbers, booleans, and
characters. You'll likely recognize these from other programming languages, but
let's jump into how they work in Rust.

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

*Table 3-1: Integer types in Rust. Each code (for example, i32) can be used to
declare the type of a value.*

Each variant can be either signed or unsigned and has an explicit size. Signed
and unsigned merely refers to whether it is possible for the number to be
either negative or positive, meaning the number needs to have a sign with it
("signed"), or whether it will only ever be positive and can therefore be
represented without a sign ("unsigned"). It's like writing numbers on paper:
when the sign matters, a number is shown with a plus sign or minus sign, but
when it's safe to assume the number is positive, it's shown with no sign.
Signed numbers are stored using two’s complement representation (if you're
unsure what this is you can search for it online; an explanation is outside the
scope of this text).

Finally, the `isize` and `usize` types depend on the kind of computer your
program is running on: 64-bits if you're on a 64-bit architecture, and 32-bits
if you’re on a 32-bit architecture.

So how do you know which type of integer to use? If you're unsure, Rust's
defaults are generally good choices, and integer types default to `i32`: it’s
generally the fastest, even on 64-bit systems. The primary situation in which
you'd need to specify `isize` or `usize` is when indexing some sort of
collection, which we'll talk about in the "Arrays" section.

#### Floating-Point Types

Rust also has two primitive types for *floating-point numbers*, which are
numbers with decimal points. Rust's floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`,
as it’s roughly the same speed as `f32`, but has a larger precision. It is
possible to use an `f64` on 32 bit systems, but it will be slower than using an
`f32` on those systems. Most of the time, trading potential worse performance
for better precision is a reasonable initial choice, and you should benchmark
your code if you suspect floating-point size is a problem in your case.

Here's an example showing floating-point numbers in action:

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
these number types--addition, subtraction, multiplication, division, and
modulo. This code shows how you'd use each one in a `let` statement:

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

    // modulo
    let remainder = 43 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates
to a single value, which is then bound to a variable.

#### The Boolean Type

As in most other programming languages, a boolean type in Rust has two possible
values: `true` and `false`. The boolean type in Rust is specified with `bool`.
For example:

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explict type annotation
}
```

The main way to consume boolean values is through conditionals like an `if`
statement. We’ll cover how `if` statements work in Rust in the "Control Flow"
section of this chapter.

#### The Character Type

So far we’ve only worked with numbers, but Rust supports letters too. Rust’s
`char` type is the language's most primitive alphabetic type, and this code
shows one way to use it:

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
human intutition for what a "character" is may not match up with what a `char`
is in Rust. It also means that `char`s are four bytes each. You can learn more
about Unicode Scalar Values at
*http://www.unicode.org/glossary/#unicode_scalar_value* and find a chart for
all unicode code points at *http://www.unicode.org/charts/*.

### Compound Types

*Compound types* can group multiple values of other types into one type. Rust
has two primitive compound types: tuples and arrays. You can also put a
compound type inside another compound type.

#### Grouping Values into Tuples

We’ve seen tuples already, when binding multiple values at once. A tuple is a
general way of grouping together some number of other values with distinct
types into one compound type. The number of values is called the *arity* of the
tuple.

We create a tuple by writing a comma-separated list of values inside
parentheses. Each position in the tuple has a distinct type, as in this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Note that, unlike the examples of multiple bindings, here we bind the single
name `tup` to the entire tuple, emphasizing the fact that a tuple is considered
a single compound element. We could then use pattern matching to destructure
this tuple value, like this:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

In this program, we first create a tuple and bind it to the name `tup`. We then
use a pattern with `let` to take `tup` and turn it into three separate
bindings, `x`, `y`, and `z`. This is called ‘destructuring’, because it breaks
the single tuple into three parts.

Finally, we print the value of `y`, which is `6.4`.

#### Tuple Indexing

In addition to destructuring through pattern matching, we can also access a
tuple element directly by using a period (`.`) followed by the index of the
value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

This program creates a tuple, `x`, and then makes new bindings to each element
by using their index. As with most programming languages, the first index in a
tuple is 0.

### Arrays

Another way to bind a name to a collection of multiple values is with an
*array*. Unlike a tuple, every element of an array must have the same type.
Arrays in Rust are different than arrays in some other languages because arrays
in Rust have a fixed length-- once declared, they cannot grow or shrink in size.

In Rust, arrays look like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

The values going into an array are written as a comma separated list inside
square brackets.

#### Type Annotation for Arrays

When you specify an array’s type, you'd do so as such:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

Much like in a variable binding that uses type annotation, the array's type and
length come after the pattern name and a colon. This array has `5` values,
which are of the `i32` type. Unlike the values themselves, the type and array
length are separated by a semicolon.

#### Using Debug in the println! Macro

So far, we’ve been printing values using `{}` in a `println!` macro. If we try
that with an array, however, we'll get an error. Say we have the following
program:

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is: {}", a);
}
```

This code tries to print the `a` array directly, which may seem innocuous. But
running it produces the following output:

```bash
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
src/main.rs:4:25: 4:26 error: the trait bound `[_; 5]: std::fmt::Display` is not satisfied [E0277]
src/main.rs:4     println!("a is: {}", a);
                                       ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:4:5: 4:28 note: in this expansion of println! (defined in <std macros>)
src/main.rs:4:25: 4:26 help: run `rustc --explain E0277` to see a detailed explanation
src/main.rs:4:25: 4:26 note: `[_; 5]` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
src/main.rs:4:25: 4:26 note: required by `core::fmt::Display::fmt`
error: aborting due to previous error
```

Whew! The core of the error is this part: *the trait bound `[_; 5]:
std::fmt::Display` is not satisfied*. We haven’t discussed traits yet, so this
is bound to be confusing! Here’s all we need to know for now: `println!` can do
many kinds of formatting. By default, `{}` implements a kind of formatting
known as `Display`: output intended for direct end-user consumption. The
primitive types we’ve seen so far implement `Display`, as there’s only one way
you’d show a `1` to a user. But with arrays, the output is less clear. Do you
want commas or not? What about the `[]`s?

More complex types in the standard library do not automatically implement
`Display` formatting. Instead, Rust implements another kind of formatting
intended for the programmer. This formatting type is called `Debug`. To ask
`println!` to use `Debug` formatting, we include `:?` in the print string, like
this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is {:?}", a);
}
```

If you run this, it should print the five values in the `a` array as desired:

```bash
$ cargo run
   Compiling arrays v0.1.0 (file:///projects/arrays)
     Running `target/debug/arrays`
a is [1, 2, 3, 4, 5]
```

You’ll see this repeated later with other types. We’ll cover traits fully in
Chapter XX.

#### Accessing and Modifying Array Elements

An array is a single chunk of memory, allocated on the stack. We can access
elements of an array using indexing, like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the `first` variable will bind to `1` at index `[0]` in the
array, and `second` will bind to `2` at index `[1]` in the array. Note that
these values are copied out of the array and into `first` and `second` when the
`let` statement is called. That means if the array changes after the `let`
statements, these bindings will not, and the two variables should retain their
values. For example, imagine you have the following code:

```rust
fn main() {
    let mut a = [1, 2, 3, 4, 5];

    let first = a[0];

    a[0] = 7;

    println!("The value of first is: {}", first);
    println!("a is {:?}", a);
}
```

First, notice the use of `mut` in the array declaration. We had to declare
array `a` as `mut` to override Rust's default immutability. The line `a[0] =
7;` modifies the element at index 0 in the array, changing its value to `7`.
This happens after `first` is bound to the original value at index 0, so
`first` should still be equal to `1`. Running the code will show this is true:

```text
The value of first is: 1
a is [7, 2, 3, 4, 5]
```

#### Invalid array element access

What happens if you try to access an element of an array past the end of the
array? Say we changed our program to:

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
thread '<main>' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:4
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/arrays` (exit code: 101)
```

We can see that compiling did not give us any errors, but we got a *runtime*
error and our program didn't exit successfully. When we attempt to access an
element using indexing, Rust will check that the index we've specified is less
than the array length. If the index is greater than the length, it will
"panic", which is what it's called when a Rust program exits with an error.

This is our first example of Rust’s safety principles in action. In many
low-level languages, this kind of check is not done, and when you provide an
incorrect index, invalid memory can be accessed. Rust protects us against this
kind of error by immediately exiting instead of allowing the memory access and
continuing. We'll discuss more of Rust’s error handling in Chapter XX.
