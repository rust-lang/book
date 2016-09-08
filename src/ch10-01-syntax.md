# Generics

We've already hinted at generics previously, but never dug into what exactly
they are. You can always recognize when generics are used by the way that they
fit into Rust's syntax: Any time you see angle brackets, `<>`, you're dealing
with generics.

The types we've seen before like `Vec<i32>`? That's employing generics. The
proper type for vectors is `Vec<T>`. That `T` is called a *type parameter*, and
it serves a similar function to parameters to functions: you fill in the
parameter with a concrete type, and that determines how the overall type works.
In the same way that a function like `foo(x: i32)` can be called with a
specific value such as `foo(5)`, a `Vec<T>` can be created with a specific
type, like `Vec<i32>`.

## Generic data types

Let's dive into generic data types in a bit more detail. We previously learned
about the `Option<T>` type, but we never examined its definition. Let's try to
imagine how we'd write it. First, let's consider an `Option` of only a number:

```rust
enum OptionalNumber {
    Some(i32),
    None,
}

let number = OptionalNumber::Some(5);
let no_number = OptionalNumber::None;
```

This works just fine for `i32`s. But what if we also wanted to store `f64`s?
Or `String`s? We would have to add code like this for each type we wanted:

```rust
enum OptionalFloatingPointNumber {
    Some(f64),
    None,
}

let number = OptionalFloatingPointNumber::Some(5.0);
let no_number = OptionalFloatingPointNumber::None;
```

We've made the enum's name a bit long in order to drive the point home. With our
current knowledge, we would have to write a unique type for every single kind
of option. In other words, the idea of "an optional value" is a higher-order
concept than one specific type. We want it to work for any type at all.

We can do that with generics. In fact, that's how the actual `Option` type works
in Rust. Let's check out its definition:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

There's those angle brackets. If we were to read this definition aloud, we'd
say "`Option` is an `enum` with one type parameter, `T`. It has two variants:
`Some`, which has a value with type `T`, and `None`, which has no value." A
bit of a mouthful! But this will work with any type:

```rust
let integer = Option::Some(5);
let float = Option::Some(5.0);
```

We've left in the `Option::` bit for consistency with the previous examples, but
since `Option<T>` is in the prelude, it's not needed:

```rust
let integer = Some(5);
let float = Some(5.0);
```

So, what's up with this syntax. Let's compare our two non-generic `enum`s side
by side:

```text
enum OptionalNumber {   enum OptionalFloatingPointNumber {
    Some(i32),              Some(f64),
    None,                   None,
}                       }
```

We have one line that's very close, but different: the `Some` bit. The only
difference is the type of the data, `i32` and `f64`. Just like we can
parameterize arguments to a function by choosing a name, we can parameterize
the type by choosing a name, in this case, `T`. We could choose any identifier
here, but traditionally, type parameters follow the same style as types
themselves: CamelCase. In addition, they tend to be short, often one letter.
`T` is the traditional choice, short for 'type'. So let's do that:

```text
enum OptionalNumber {   enum OptionalFloatingPointNumber {
    Some(T),                Some(T),
    None,                   None,
}                       }
```

We've replaced `i32` and `f64` with `T`. There's one problem, though: we've
*used* `T`, but not defined it. This would be similar to using an argument to
a function without declaring it. We need to tell Rust that we've introduced a
generic parameter. We can do that with the angle brackets; let's try it:

```text
enum OptionalNumber<T> {   enum OptionalFloatingPointNumber<T> {
    Some(T),                Some(T),
    None,                   None,
}                       }
```

The `<>`s indicate a list of type parameters, just like `()` indicates a
list of value parameters. Now, the only difference between our two `enum`s
is the name. And since we've made them generic, they're not specific to numbers
or floating point numbers. So let's give them the same name:

```text
enum Option<T> {	enum Option<T> {
    Some(T),            Some(T),
    None,               None,
}                   }
```

Now they're identical! We've made our type fully generic. Understanding this
process is important, because the compiler actually does the exact opposite of
this when compiling your code. Taking code that is generic over some type and
generating code that is specific for the concrete types that are used with the
generic code is called *monomorphization*, and it's why Rust's generics are
extremely efficient. Consider this code that uses the standard library's
`Option`:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it will perform monomorphization. What this means
is that the compiler will see that we've used two kinds of `Option<T>`: one
where `T` is `i32`, and one where `T` is `f64`. As such, it will expand the
generic definition of `Option<T>` into `Option<i32>` and `Option<f64>`, and
replace the calls with the specific versions. Like this:

```rust
enum OptionInteger {
    Some(i32),
    None,
}

enum OptionFloat {
    Some(f64),
    None,
}

let integer = OptionInteger::Some(5);
let float = OptionFloat::Some(5.0);
```
In other words, we can write the non-duplicated form, but Rust will act as
though we wrote the specific type out in each instance. This means we pay no
runtime cost for using generics; it's just like we copy/pasted each particular
definition.

In a similar fashion, we can use `<>`s with structs as well:

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

The process is the same: add a `<T>` by the name, then put `T` in where the
type name goes.

If we need multiple type parameters, we can use a comma. Consider a universe in
which `x` and `y` need different types:

```rust
struct Point<X, Y> {
    x: X,
    y: Y,
}
```

Now `x` will have the type of `X`, and `y` will have the type of `Y`. We can
make `enum`s with multiple type parameters as well. Remember `Result<T, E>`
from the error handling chapter? Here's its definition:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Each variant stores a different kind of information, and they're both generic.
You can have as many type parameters as you'd like. Similarly to parameters of
values, if you have a lot of them, it can get quite confusing, so try to keep
the number of them small if you can.

## Generic functions and methods

In a similar way to data structures, we can use the `<>` syntax to write
functions:

```rust
fn generic_function<T>(argument: T) {
    // code goes here
}
```

and methods:

```rust
struct Foo;

impl Foo {
    fn method<T>(argument: T) {
        // code goes here
    }
}
```

We can use the same process to refactor duplicated specific code into code that
uses generics. If we had these two functions:

```text
fn takes_integer(argument: i32) {       fn takes_float(argument: f64) {
    // code goes here                       // code goes here
}                                       }
```

We'd replace their parameter with `T`:

```text
fn takes_integer(argument: T) {       fn takes_float(argument: T) {
    // code goes here                     // code goes here
}                                     }
```

Add the `T` parameter to the type parameter list:

```text
fn takes_integer<T>(argument: T) {       fn takes_float<T>(argument: T) {
    // code goes here                       // code goes here
}                                       }
```

And then rename them:

```text
fn takes<T>(argument: T) {       fn takes<T>(argument: T) {
    // code goes here                // code goes here
}                                }
```

Now they're the same!

There's one problem though. We've got some function _definitions_ that work,
but if we try to use our argument in the function body, we'll get an error. To
see what we mean here, try compiling this function:

```rust,ignore
fn print<T>(argument: T) {
    println!("Got an argument: {}", argument);
}
```

You'll get an error that looks like this:

```text
	error[E0277]: the trait bound `T: std::fmt::Display` is not satisfied
 --> <anon>:3:37
  |
3 |     println!("Got an argument: {}", argument);
  |                                     ^^^^^^^^ trait `T: std::fmt::Display` not satisfied
  |
  = help: consider adding a `where T: std::fmt::Display` bound
  = note: required by `std::fmt::Display::fmt`

error: aborting due to previous error(s)
```

This error mentions something we haven't learned about yet: traits. In the next
section, we'll figure out how to make this compile.
