# Common Programming Concepts in Rust

Let's look at concepts that appear in almost every programming language and see
how they work in Rust. Many programming languages have much in common at their
core. None of the concepts presented in this chapter are unique to Rust, but
we’ll discuss Rust’s particular syntax and conventions concerning these common
concepts.

Specifically, we’ll be talking about variable bindings, basic types, functions,
comments, and control flow. These foundations will be in every Rust
program, and learning them early will give you a strong core to start from.

PROD: START BOX

Keep in mind as we get into variables and functions that the Rust language has
a set of *keywords* that have been reserved for use by the language only, much
like other languages do. This means you cannot use these words as names of
variables or functions, for example. Most of these have special meaning and we
will be using them to do various things in our Rust programs; a few have no
current functionality associated but have been reserved for functionality that
might be in the Rust language in the future. You can find a list of the
keywords in Appendix XX.

PROD: END BOX

## Variable Bindings and Mutability

We mentioned in Chapter XX that by default, variable bindings are *immutable*.
This is one of many nudges that Rust's design has to encourage us to write our
code to get the most of the safety and easy concurrency that Rust has to offer.
We still have the option to make our bindings mutable, though. Let's explore
how and why Rust encourages us to favor immutability, and why we might want to
opt out of that.

Variable bindings being immutable means that once a value is bound, you can't
change that value. To illustrate this, let's generate a new project with Cargo.
Open a terminal, and navigate to the directory you want to store your projects
in. From there, run these commands:

```bash
$ cargo new --bin bindings
$ cd bindings
```

Then open *src/main.rs* and replace its code with the following:

```rust,ignore
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Save and run the program using `cargo run`, and you should receive an error
message, as in this output:

```bash
$ cargo run
   Compiling bindings v0.0.1 (file:///projects/bindings)
error: re-assignment of immutable variable `x` [--explain E0384]
 --> src/main.rs:4:5
4 |>     x = 6;
  |>     ^^^^^
note: prior assignment occurs here
 --> src/main.rs:2:9
2 |>     let x = 5;
  |>         ^
```

This is our first example of the compiler helping us find an error in our
program! Compiler errors can be frustrating. Keep in mind that they only mean
your program isn't safely doing what you want it to do yet; they do _not_ mean
that you're not a good programmer! Experienced Rustaceans still get compiler
errors. Try to keep in mind that the Rust compiler is trying to help your
program be the very best.

PROD: START BOX
######Extended Error Explanations

Now that you've seen an example of a Rust error, let's look at one particularly
useful aspect of errors. Rust encourages you to seek further information on the
kind of error you've received with output like this:

```bash
error: re-assignment of immutable variable `x` [--explain E0384]
```

This tells us that if we pass the `--explain` flag to `rustc` with the provided
error code, we can see an extended explanation which will try to explain common
causes of and solutions to that kind of error. Not every error has a longer
explanation, but many do. Here’s the explanation for the `E0384` error we
received:

````bash
$ rustc --explain E0384
This error occurs when an attempt is made to reassign an immutable variable.
For example:

```
fn main(){
    let x = 3;
    x = 5; // error, reassignment of immutable variable
}
```

By default, variables in Rust are immutable. To fix this error, add the keyword
`mut` after the keyword `let` when declaring the variable. For example:

```
fn main(){
    let mut x = 3;
    x = 5;
}
```
````

These explanations can really help if you’re stuck on an error, so don't
hesitate to look up the error code. The compiler is your friend, and it's there
to help.

PROD: END BOX

The error includes the message `re-assigment of immutable variable` because the
program tried to assign a second value to the `x` variable.

Getting compile-time errors when your code attempts to change a value that it
previously said was immutable is important because this very situation can lead
to bugs. If one part of your code operates on an assumption that a value it's
operating on will never change, and another part of your code changes that
value, it's possible that the first code won't do what it was designed to do.
Especially when the second piece of code only changes the value _sometimes_,
this cause of bugs can be difficult to track down after the fact.

In Rust, our code can know that a value our code assumes won't change really
won't change, because the compiler is enforcing that guarantee for us. When
reading and writing code, we don't have to keep track in our head how and where
a value might change. This can make code easier to reason about.

Mutability is really useful, though! Bindings are immutable only by default;
you can make them mutable by adding `mut` in front of the variable name. In
addition to telling the compiler it should allow this value to be changed, it
conveys intent to future readers of the code and says that other parts of the
code will be changing this value.

For example, change the program you just wrote to the following:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Running this, we get:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
The value of x is: 6
```

Using `mut`, we are allowed to change the value that `x` binds to from `5` to
`6`. You might want to make a binding mutable because it makes the code easier
to understand than an implementation that only uses immutable bindings. In
cases where you're using large data structures, mutating an instance in place
may be faster than copying and returning newly allocated instances. It all
depends on the tradeoffs you want to make in your situation.

### Shadowing

As we saw in the guessing game tutorial, we can declare new bindings with the
same name as a previous binding, and the new binding *shadows* the previous
binding. We say that the first binding is ‘shadowed’ by the second, which means
that the second binding's value is what you will see when you use the variable
after the second binding. This can be useful if you’d like to perform a few
transformations on a value, but have the binding be immutable after those
transformations have been completed. For example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

This program first binds `x` to a value of `5`. Then, it shadows `x` by saying
`let x =` again, taking the original value and adding `1` so that the value of
`x` is then `6`. The third `let` statement also shadows `x`, taking the
previous value and multiplying it by `2` to give `x` a final value of `12`. If
you run this, it will output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

Shadowing is useful because it lets us modify `x` without having to make the
binding mutable. This means the compiler will still keep us from accidentally
trying to mutate `x` directly later.

Now let's look at some of the types of values that we can bind variables to.

## Data Types

Every value in Rust is of a certain *type*, which tells Rust what kind of data
is being given so it knows how to work with that data. In this section, we'll
look at a number of types built into the language itself split into two subsets
of Rust data types: scalar and compound.

Something to keep in mind throughout this section: Rust is a *statically typed*
language, which means that it must know the types of all bindings at compile
time. The compiler can usually infer what type we want to use based on the
value and how we use it. When many types are possible, such as when we
converted a `String` to a numeric type using `parse()` in the guessing game
tutorial, we can add a type annotation, like this:

```rust,ignore
let x: i32 = 5;
```

You will see some type annotations as we discuss the various data types.

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

*Table 4-1: Integer types in Rust. Each code (for example, i32) can be used to
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
these number types: addition, subtraction, multiplication, division, and
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
is in Rust. It also means that `char`s are four bytes each. You can learn more
about Unicode Scalar Values at
*http://www.unicode.org/glossary/#unicode_scalar_value* and find a chart for
all unicode code points at *http://www.unicode.org/charts/*.

### Compound Types

*Compound types* can group multiple values of other types into one type. Rust
has two primitive compound types: tuples and arrays.

#### Grouping Values into Tuples

We’ve seen tuples already, when binding multiple values at once. A tuple is a
general way of grouping together some number of other values with distinct
types into one compound type.

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

In Rust, the values going into an array are written as a comma separated list
inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

While arrays can be useful since they are a primitive type, they aren't as
flexible as the `Vec` (short for "vector"), a similar collection type provided
by the standard library that _is_ allowed to grow or shrink in size. If you're
unsure whether to use an array or a `Vec`, you should probably go with a `Vec`,
and we'll discuss them in more detail in chapter XX.

#### Accessing Array Elements

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
values.

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

## How Functions Work

Functions are pervasive in Rust code. We’ve already seen one of the most
important functions in the language: the `main()` function that’s the entry
point of many programs. We've also seen the `fn` keyword, which allows us to
declare new functions.

Rust code uses *snake case* as the conventional style for function names. In
snake case, all letters are lower case, and there are underscores separating
words. (Rust also uses snake case for the names of variable bindings; we just
haven't used any variable bindings with enough letters to need underscores
yet). Here's a program containing an example function definition:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Function definitions in Rust start with `fn` and have a set of parentheses
after the function name. The curly braces tell the compiler where the function
body begins and ends.

We can call any function we’ve defined by entering its name followed by a pair
of parentheses. Since `another_function()` is defined in the program, it can be
called from inside the `main()` function. Note that we defined
`another_function()` _after_ the `main()` function in our source code; we could
have defined it before as well. Rust doesn’t care where you define your
functions, only that they are defined somewhere.

Let’s start a new project to explore functions further. Open a terminal, and
navigate to the directory you're keeping your projects in. From there, use
Cargo to generate a new project, as follows:

```bash
$ cargo new --bin functions
$ cd functions
```

Place the `another_function()` example in a file named *src/main.rs* and run
it. You should see the following output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order they appear in the `main()` function. First, our
“Hello, world!” message prints, and then `another_function()` is called and its
message is printed.

### Function Arguments

Functions can also take arguments. The following rewritten version of
`another_function()` shows what arguments look like in Rust:

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Try running this program, and you should get this output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

Since we passed `5` to `another_function()`, the `println!` macro put `5` where
the pair of curly braces were in the format string. The declaration of
`another_function()` shows that it takes one argument named `x`, and the type
of `x` is `i32`.

In function signatures, we _must_ declare the type. This is a deliberate
decision in the design of Rust; requiring type annotations in function
definitions means the compiler almost never needs you to use them elsewhere in
the code in order to figure out what you mean.

When you want a function to have multiple arguments, just separate them inside
the function signature with commas, like this:

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

In this example, we make a function with two arguments, both of which are
`i32`s. If your function has multiple arguments, they don’t need to be the same
type, but they just happen to be in this example. Our function then prints out
the values of both of its arguments.

Let’s try out this code. Replace the program currently in your `function`
project's `main.rs` file with the example above, and run it as follows:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Since `5` is passed as the `x` argument and `6` is passed as the `y` argument,
the two strings are printed with these values.

### Function Bodies

Function bodies are made up of a series of statements ending in an optional
expression. So far, we've only seen functions without an ending expression, but
we have seen expressions as parts of statements. Since Rust is an
expression-based language, this is an important distinction to understand.
Other languages don't have the same distinctions, so let's look at what
statements and expressions are and how their differences affect the bodies of
functions.

#### Statements and Expressions

We've already been using both statements and expressions. *Statements* are
instructions that perform some action and do not return a value. *Expressions*
evaluate to a resulting value. Let's look at some examples.

`Let` bindings are statements. They instruct the program to create a binding
name and assign a value to it. `let y = 6;` in this example is a statement:

```rust
fn main() {
    let y = 6;
}
```

Function definitions are also statements-- so the entire previous example is a
statement as well.

Statements do not return values themselves. Therefore, you can’t assign a `let`
binding to another binding, as this code tries to do:

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

If we were to run this program, we’d get an error like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
2 |>     let x = (let y = 6);
  |>              ^^^
note: variable declaration using `let` is a statement

error: aborting due to previous error
error: Could not compile `functions`.
```

The `let y = 6` statement does not return a value, so there isn't anything for
`x` to bind to. This is different than in other languages like C and Ruby where
the assignment returns the value of the assignment. In those languages, you
could write `x = y = 6` and have both `x` and `y` have the value `6`, but that
is not the case in Rust.

Expressions are most of the rest of the code that you will write in Rust.
Consider a simple math operation, like this:

```rust,ignore
5 + 6
```

This is an expression, and evaluating it results in the value `11`. Expressions
can be part of statements-- in the previous example that had the statement `let
y = 6;`, `6` is an expression that evaluates to the value `6`. Calling a
function is an expression. Calling a macro is an expression. The block that we
use to create new scopes, `{}`, is an expression, for example:

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

The expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, gets evaluated to `4`, which then gets bound to
`y` as part of the `let` statement.

Note that the line containing `x + 1` does not have a semicolon at the end like
most of the lines we've seen up until now have had. This is the most important
distinction between expressions and statements to remember: statements end in
semicolons while expressions do not. If you add a semicolon to the end of an
expression, that will turn it into a statement, which will then not return a
value. Keep this in mind as we explore function return values and expressions.

### Functions with Return Values

Functions can return values back to the code that calls them. We don’t name
return values, but we do declare their type, after an arrow (`->`). In Rust,
the "return value of the function” is synonymous with “the value of the final
expression in the block of the body of a function.” Here's an example of a
function that returns a value:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, or even `let` statements in the `five()`
function-- just the number `5` by itself. That's a perfectly valid function in
Rust. Note the function's return type, too. Try running this code, and the
output should look like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

The `5` in `five()` is actually the function's return value, which is why the
return type is `i32`. Let’s examine this in more detail. There are two
important bits. First, the line `let x = five();` in `main()` shows that we can
use the return value of a function to initialize a binding.

Because the function `five()` returns a `5`, that line is the same as saying:

```rust
let x = 5;
```

The second interesting bit is the `five()` function itself. It requires no
arguments and defines the type of the return value, but the body of the
function is a lonely `5` with no semicolon because it is an expression whose
value we want to return. Let's look at another example:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this code will print `The value of x is: 6`. What happens if we put a
semicolon at the end of the line containing `x + 1`, changing it from an
expression to a statement?

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Running this code gives an error, as follows:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: not all control paths return a value [--explain E0269]
 --> src/main.rs:7:1
7 |> fn plus_one(x: i32) -> i32 {
  |> ^
help: consider removing this semicolon:
 --> src/main.rs:8:10
8 |>     x + 1;
  |>          ^

error: aborting due to previous error
error: Could not compile `functions`.
```

The main error message, "not all control paths return a value", reveals the
core of the issue with this code. The definition of the function `plus_one`
says that it will return an `i32`, but statements don’t evaluate to a value.
Therefore, nothing is returned, which contradicts the function definition and
results in an error. In this output, Rust gives an option to rectify this: it
suggests removing the semicolon, which would fix the error.

## Comments

All programmers strive to make their code easy to understand, but sometimes
extra explanation is warranted. In these cases, we leave notes in our source
code that the compiler will ignore but people reading the source code may find
useful. These notes are called *comments*.

Here’s a simple comment:

```rust
// Hello, world.
```

In Rust, comments must start with two slashes and will last until the end of
the line. For comments that extend beyond a single line, you'll need to
include `//` on each line, like this:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also be placed at the end of lines of code:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above, like so:

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.

## Control Flow

Deciding whether or not to run some code depending on if a condition is true,
or deciding to run some code repeatedly while a condition is true, are basic
building blocks in most programming languages. The most common constructs that
let us control the flow of execution of our Rust code are `if` expressions and
loops.

### `if` Expressions

An `if` expression allows us to branch our code depending on conditions. We
provide a condition and then say, "If this condition is met, then run this
block of code. If the condition is not met, do not run this block of code."

Let’s make a new project to explore `if`. Navigate to your projects directory,
and use Cargo to make a new project called `branches`:

```bash
$ cargo new --bin branches
$ cd branches
```

Write this sample program using `if` and save it in the *branches* directory in
`src/main.rs`:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

All `if` expressions start with `if`, which is followed by a condition. In this
case, our condition is checking if our variable binding `number` has a value
that is less than 5. The block of code we want to execute if the condition is
true goes immediately after the condition, inside curly braces. These blocks
are sometimes called ‘arms’. We can optionally also include an `else`
statement, which we have chosen to do here. `else` gives the program a block of
code to execute should `condition` evaluate to false.

Try running this code, and you should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

Let’s try changing the value of `number` to a value that makes the condition
`false` to see what happens:

```rust,ignore
let number = 7;
```

Run the program again, and look at the output:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

It’s also worth noting that `condition` here _must_ be a `bool`. To see what
happens if the condition isn't a `bool`, try running this code:

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

The `if` condition evaluates to a value of `3` this time, and Rust will
complain about it:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
error: mismatched types [--explain E0308]
 --> src/main.rs:4:8
4 |>     if number {
  |>        ^^^^^^ expected bool, found integral variable
note: expected type `bool`
note:    found type `_`

error: aborting due to previous error
Could not compile `branches`.
```

The error tells us that Rust expected a `bool`, but got an integer. Rust will
not automatically try to convert non-boolean types to a boolean here, unlike
languages like Ruby or JavaScript. We must be explicit and always give `if` a
`boolean` as its condition. If your intention is for the `if` code block to be run if a number is not equal to `0`, for example, we would change the `if` expression to read:

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Running this will print "number was something other than zero".

#### Multiple Conditions with `else if`

We can have multiple coniditions by combining `if` and `else` in an `else if`
expression. For example:

```rust
fn main() {
    let number = 5;

    if number == 3 {
        println!("condition was 3");
    } else if number == 4 {
        println!("condition was 4");
    } else if number == 5 {
        println!("condition was 5");
    } else {
        println!("condition was something else");
    }
}
```

This program has four possible paths it can take. If you try running it, you
should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was 5
```

When this program executes, it will check each `if` expression in turn and
execute the first body for which the condition holds true.

Using too many `else if` expressions can clutter your code, so if you find
yourself with more than one, you may want to look at refactoring your code. In
Chapter XX, we'll talk about a powerful Rust branching construct called `match`
for these cases.

#### Using `if` in a Binding

The last detail you need to learn about `if` is that it’s an expression. That
means that we can use it on the right hand side of a `let` binding, for
instance:

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

The `number` variable will be bound to a value based on the outcome of the `if`
expression. Let’s run this to see what happens:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
The value of number is: 5
```

Remember, blocks of code evaluate to the last expression in them, and numbers
by themselves are also expressions. In this case, the value of the whole `if`
expression depends on which block of code executes. This means that the value
that results from both arms of the `if` must be the same type; in the previous
example, they were both `i32` integers. But what happens if the types are
mismatched, as in the following example?

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

The expression in the `if` block is an integer and the expresion in the `else`
block is a string. This can’t work, because variable bindings must have a
single type. If we try to run this, we’ll get an error:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
src/main.rs:4:18: 8:6 error: if and else have incompatible types:
 expected `_`,
    found `&‘static str`
(expected integral variable,
    found &-ptr) [E0308]
src/main.rs:4     let number = if condition {
src/main.rs:5         5
src/main.rs:6     } else {
src/main.rs:7         "six"
src/main.rs:8     };
src/main.rs:4:18: 8:6 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
Could not compile `branches`.
```

The `if` and `else` arms have value types that are incompatible, and Rust tells
us exactly where to find the problem in our program.

### Repetition with Loops

It’s often useful to be able to execute a block of code more than one time. For
this, Rust has several constructs called *loops*. A loop runs through the code
inside it to the end and then starts immediately back at the beginning. To try
out loops, let’s make a new project. Navigate to your *projects* folder and use
Cargo to make a new project:

```bash
$ cargo new --bin loops
$ cd loops
```

There are three kinds of loops in Rust: `loop`, `while`, and `for`. Let’s dig
in.

#### Repeating Code with `loop`

The `loop` keyword tells Rust to execute a block of code over and over again
forever or until we explicitly tell it to stop.

For an example, change the *src/main.rs* file in your *loops* directory to look
like this:

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

If we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support a keyboard shortcut,
`control-c`, to halt a program stuck in a continual loop. Give it a try:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

That `^C` there is where we hit `control-c`. You may or may not see "again!"
printed after the `^C`, depending on where the code was in the loop when it
received the signal to halt.

Fortunately, Rust provides another, more reliable way to break out of a loop.
We can place the `break` keyword within the loop to tell the program when to
stop executing the loop. Recall that we did this in the guessing game to exit
the program when the user won the game by guessing the number correctly.

#### Conditional Loops With `while`

A useful thing that many programs do is have a condition that can be evaluated
within a loop. While the condition is true, the loop runs. When the condition
ceases to be true, we call `break`, stopping the loop. This could be
implemented with a combination of `loop`, `if`, `else`, and `break`; try to do
that now if you'd like!

But this pattern is so common that Rust has a more efficient language construct
for it, called a `while` loop. Here's an example using `while`: this program
loops three times, counting down each time. Finally, after the loop, it prints
another message, then exits:

```rust
fn main() {
    let mut number = 3;

    while number != 0  {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

This gets rid of a lot of nesting that would be necessary if we used `loop`,
`if`, `else`, and `break`, and it's more clear. While a condition holds, run
this code; otherwise, exit the loop.

#### Looping Though a Collection with `for`

We could use this `while` construct to loop over the elements of a collection,
like an array. For example:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

Here, we're counting up through the elements in the array. We start at index 0,
then loop until we hit the final index of our array (that is, when `index < 5`
is no longer true). Running this will print out every element of the array:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `6` at some point, the loop stops executing before trying
to fetch a sixth value from the array.

This approach is error-prone, though; we could cause our program to panic by
getting the index length incorrect. It's also slow, as the compiler needs to
perform the conditional check on every element on every iteration through the
loop.

As a more efficient alternative, we can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like this:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

If we run this, we'll see the same output as the previous example. Importantly,
though, we've now increased the safety of our code and eliminated the chance of
bugs resulting from going beyond the end of the array or not going far enough
and missing some items.

For example, in the previous code that uses the `while` loop, if we removed an
item from the `a` array but forgot to update the condition to be `while index <
4`, our code would panic. Using the `for` loop means we would not need to
remember to change any other code if we changed the the number of values in the
array.

If you're wondering about the `.iter()` code in this example, keep reading! We
will cover method syntax generally in Chapter XX and iterators specifically in
Chapter XX.

The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations where you want to run some code a certain
number of times, like our countdown example that used a `while` loop, most
Rustaceans would use a `for` loop. The way to do that is using a `Range`, which
is a type provided by the standard library that generates numbers starting from
one number and ending before another number. Here's what the countdown would
look like with a for loop, and using another method we haven't yet talked
about, `.rev()`, to reverse the range:

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

That's a bit nicer, isn't it?

Now that you know how Rust does things that most other languages can do, let's
talk about a concept that _doesn't_ commonly exist: ownership.
