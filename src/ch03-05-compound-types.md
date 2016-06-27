# Compound Types

Now that we’ve discussed scalar types, let’s talk about compound types.
These types can group multiple values of scalar types into another type.

## Tuples

We’ve seen tuples before, in the guise of binding or returning multiple values
at once. It turns out that there’s no magic here: tuples are a general way of
making a compound value that groups some number of other values with distinct
types. The number of values grouped is the ‘arity’ of the tuple.

We create a tuple by writing a comma-separated list of values inside
parentheses; each position in the tuple has a distinct type:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Note that, unlike the examples of multiple bindings, here we bound the
single name `tup` to the entire tuple. We can then use pattern
matching to destructure this tuple value:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

Tuples are used sparingly in Rust code. This is because the elements of a tuple
are anonymous, which can make code hard to read.

### Tuple indexing

In addition to destructuring through pattern matching, we can also access a
tuple element directly using `.`, followed by the index we want to access:

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
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
thread ‘<main>’ panicked at ‘index out of bounds: the len is 5 but the index is 10’, src/main.rs:4
Process didn’t exit successfully: `target/debug/functions` (exit code: 101)
```

It says that our thread panicked, and that our program didn’t exit
successfully. There’s also a reason: we had a length of five, but an index of
10.

For now, all you need to know is that a panic will crash your program. Rust’s
error handling story is described in full in a later chapter.

So why did this code panic? Well, arrays know how many elements they hold. When
we access an element via indexing, Rust will check that the index is less than
the length. If it’s greater, it will panic, as something is very wrong. This is
our first example of Rust’s safety principles in action. In many low-level
languages, this kind of check is not done. If you have an incorrect index,
invalid memory can be accessed. Rust protects us against this kind of error.

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
   Compiling functions v0.1.0 (file:///projects/functions)
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
By default, `{}` implements a kind of formatting known as `Display`: output
intended for direct end-user consumption. The primitive types we’ve seen so far
implement `Display`, as there’s only one way you’d show a `1` to a user. But
with arrays, the output is less clear. Do you want commas or not? What about
the `[]`s?

Due to these questions, more complex types in the standard library do not
implement `Display` formatting. There is another kind of formatting, `Debug`,
which is a bit different: intended for programmer consumption. We can ask
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
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
a is [1, 2, 3, 4, 5]
```

You’ll see this repeated later, with other types. And we’ll cover traits fully
later in the book, Section 9.
