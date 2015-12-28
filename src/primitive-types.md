# Primitive Types

We’ve seen that every value in Rust has a type of some kind. There are a number
of types which are built into the language itself. We call these types
‘primitive’ types, since you can’t re-create them yourself. There are, of
course, many non-primitive types provided by the standard library as well.

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

## Tuples

The other type we’ve seen previously is the tuple type. Tuples have an ‘arity’,
or size. We might say “that’s a 3-tuple” or “that’s a 5-tuple.”

Each position in a tuple has a distinct type:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
}
```

Tuples are used sparingly in Rust code. This is because the elements of a tuple
are anonymous, which can make code hard to read.

### Tuple indexing

To access an element of a tuple, we use a `.` followed by the index we want to
access:

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

There’s one last trick with tuples: `(5)` is actually ambiguous: is it a tuple,
or is it a `5` in parethesis? If you need to disambiguate, use a comma:

```rust
fn main() {
    let x = (5); // x is an i32, no tuple. Think of it like (5 + 1) without the + 1, they’re for grouping.

    let x = (5,); // x is a (i32), a tuple with one element.
}
```

## Functions

There’s one more type that we’ve been using, but you haven’t seen written
explicitly. Functions! Functions also have a type, and yes, you can even have
variables which hold functions! Here’s an example:

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

As you can see, the type is very similar to the declaration. Here, let’s put
them side by side:

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

Finally, if you’ll notice in that example, we can create a binding with a
function in it:

```rust,ignore
fn main() {
    let f = plus_one;

    let five = f(4);
}
```

... and call it with `()`s just like if we had used the original name.

### Functions as arguments

So why not just use the original name? Well, we can pass functions as arguments
to other functions! Check this out:

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

This says “`twice()` is a function which takes two arguments. `x` is a
thirty-two bit integer, and `f` is a function which takes an `i32` and returns
an `i32`.”

Inside of `twice()`, as you might imagine, we call the function `f` twice on
`x`, and return the result.


```rust,ignore
let y = twice(x, plus_one);
let z = twice(x, plus_two);
```

The first time we call `twice()`, we pass `plus_one()` as an argument. And `x`
is `5`. So `5 + 1 + 1 == 7`, hence our first line of output. The second time,
we pass `plus_two()` instead. `5 + 2 + 2` is `9`, and our second line checks
out too.

Passing functions to functions is very, very powerful.

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

## Arrays

So far, we’ve only represented single values in a binding. Sometimes, though,
it’s useful to have more than one value. These kinds of data structures are
called ‘collections’, and arrays are the ones we’ll learn about first. Arrays
look like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

An array’s type consists of the type of the elements it contains, as well as
the length:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

An array is a single chunk of memory, allocated on the stack.

We can access elements of an array using indexing:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, `first` will hold the value `1`, and `second` will be bound to
`2`. Note that these values are copied out of the array; if the array changes,
these bindings will not. Here’s an example, which also shows us how we can
modify elements of the array:

```rust
fn main() {
    let mut a = [1, 2, 3, 4, 5];

    let first = a[0];
    
    a[0] = 7;

    println!("The value of first is: {}", first);
}
```

Running this example will show that `first` is still `1`. If we didn’t want a
copy, but instead wanted to refer to the first element, whatever its value was,
we need a new concept. We’ll talk about ‘references’ in Section 4.

One last thing: now that we are modifying the array, `a` needs to be declared
`mut`.

Arrays are our first real data structure, and so there’s a few other concepts
that we haven’t covered in full yet. There are two: the `panic!` macro, and a
new way of printing things: `Debug`.

### Panic

We showed what happens when you access elements of an array, but what if we
give an invalid index?

```rust,should_panic
fn main() {
    let a = [1, 2, 3, 4, 5];

    let invalid = a[10];

    println!("The value of invalid is: {}", invalid);
}
```

If we run this example, we will get an error. Let’s re-use our `functions`
project from before. Change your `src/main.rs` to look like the example, and
run it:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///home/steve/tmp/functions)
     Running `target/debug/functions`
thread ‘<main>’ panicked at ‘index out of bounds: the len is 5 but the index is 10’, src/main.rs:4
Process didn’t exit successfully: `target/debug/functions` (exit code: 101)
```

It says that our thread panicked, and that our program didn’t exit
successfully. There’s also a reason: we had a length of five, but an index of
10.

A ‘panic’ can also be induced manually, with the `panic!` macro:

```rust,should_panic
fn main() {
    panic!("Oh no!");
}
```

When the `panic!` macro runs, it will cause a panic. When a Rust program
panics, it starts a kind of controlled crash. The current thread of execution
will stop entirely. As such, panics are reserved for serious, program-ending
errors. They’re not a general error-handling mechanism.

So why did this code panic? Well, arrays know how many elements they hold. When
we access an element via indexing, Rust will check that the index is less than
the length. If it’s greater, it will panic, as something is very wrong. This is
our first example of Rust’s safety principles in action. In many low-level
languages, this kind of check is not done. If you have an incorrect index,
invalid memory can be accessed. Rust protects us against this kind of error.

**Steve’s note: this next bit might be our first ‘advanced’ section, on get()?**

### Debug

So far, we’ve been printing values using `{}`. If we try that with an array,
though...

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is: {}", a);
}
```

... we will get an error:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///home/steve/tmp/functions)
src/main.rs:4:25: 4:26 error: the trait `core::fmt::Display` is not implemented for the type `[_; 5]` [E0277]
src/main.rs:4     println!(“a is {}”, a);
                                      ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:4:5: 4:28 note: in this expansion of println! (defined in <std macros>)
src/main.rs:4:25: 4:26 help: run `rustc --explain E0277` to see a detailed explanation
src/main.rs:4:25: 4:26 note: `[_; 5]` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
src/main.rs:4:25: 4:26 note: required by `core::fmt::Display::fmt`
error: aborting due to previous error
```

Whew! The core of the error is this part: the trait `core::fmt::Display` is not
implemented. We haven’t discussed traits yet, so this is bound to be confusing!
Here’s all we need to know for now: `println!` can do many kinds of formatting.
By default, `{}` implements a kind of formatting known as `Display`: output for
end-users. The primitive types we’ve seen so far implement `Display`, as
there’s only one way you’d show a `1` to a user. But with arrays, the output is
less clear. Do you want commas or not? What about the `[]`s?

Due to these questions, more complex types in the standard library do not
implement `Display` formatting. There is another kind of formatting, `Debug`,
which is a bit different: output for programmers and debuggers. We can ask
`println!` to use `Debug` formatting with `:?`:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is {:?}", a);
}
```

This will work:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///home/steve/tmp/functions)
     Running `target/debug/functions`
a is [1, 2, 3, 4, 5]
```

You’ll see this repeated later, with other types. And we’ll cover traits fully
later in the book, Section 9.

## char

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

## str

We can declare literal strings with `"`s. We’ve seen them already, with
`println!`:

```rust
fn main() {
    println!("println! takes a literal string as an argument.");

    let s = "We can also create bindings to string literals.";

    let s: &str = "Here’s one with a type annotation.";
}
```

String literals are immutable, and of a fixed length. Rust has a second string
type, `String`, that we’ll discuss in section 8.

`&str`s are UTF-8 encoded.
