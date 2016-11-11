## Generics Syntax

We've already hinted at the concept of generics in previous chapters, but we
never dug into what exactly they are or how to use them. In places where we
specify a type, like function signatures or structs, instead we can use
*generics*. Generics are stand-ins that represent more than one particular
concept. In this section, we're going to cover generic *data types*.

You can recognize when any kind of generics are used by the way that they fit
into Rust's syntax: any time you see angle brackets, `<>`, you're dealing with
generics. Types we've seen before, like in Chapter 8 where we discussed vectors
with types like `Vec<i32>`, employ generics. The type that the standard library
defines for vectors is `Vec<T>`. That `T` is called a *type parameter*, and it
serves a similar function as parameters to functions: you fill in the parameter
with a concrete type, and that determines how the overall type works. In the
same way that a function like `foo(x: i32)` can be called with a specific value
such as `foo(5)`, a `Vec<T>` can be created with a specific type, like
`Vec<i32>`.

### Duplicated Enum Definitions

Let's dive into generic data types in more detail. We learned about how to use
the `Option<T>` enum in Chapter 6, but we never examined its definition. Let's
try to imagine how we'd write it! We'll start from duplicated code like we did
in the "Removing Duplication by Extracting a Function" section. This time,
we'll remove the duplication by extracting a generic data type instead of
extracting a function, but the mechanics of doing the extraction will be
similar. First, let's consider an `Option` enum with a `Some` variant that can
only hold an `i32`. We'll call this enum `OptionalNumber`:

```rust
enum OptionalNumber {
    Some(i32),
    None,
}

let number = OptionalNumber::Some(5);
let no_number = OptionalNumber::None;
```

This works just fine for `i32`s. But what if we also wanted to store `f64`s? We
would have to duplicate code to define a separate `Option` enum type for each
type we wanted to be able to hold in the `Some` variants. For example, here is
how we could define and use `OptionalFloatingPointNumber`:

```rust
enum OptionalFloatingPointNumber {
    Some(f64),
    None,
}

let number = OptionalFloatingPointNumber::Some(5.0);
let no_number = OptionalFloatingPointNumber::None;
```

We've made the enum's name a bit long in order to drive the point home. With
what we currently know how to do in Rust, we would have to write a unique type
for every single kind of value we wanted to have either `Some` or `None` of. In
other words, the idea of "an optional value" is a higher-order concept than one
specific type. We want it to work for any type at all.

### Generic Type Parameters

We can define an `Option` type that works for any type by using generics. In
fact, that's how the actual `Option` type works in Rust. Let's check out its
definition:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The angle brackets (`<>`) go after the enum name and before the curly brace in
the enum definition. Inside the brackets is the declaration of the generic type
name `T`. The defintion of the enum's variants can then use `T` to stand in for
any type. In this case, the definition `Some(T)` says that the `Some` variant
can hold a value of any type. If we were to read this definition aloud, we'd
say, "`Option` is an `enum` with one type parameter, `T`. It has two variants:
`Some`, which has a value with type `T`, and `None`, which has no value." A bit
of a mouthful!

This definition allows using `Option` with any type, like these uses of
`Option` with an `i32` and with an `f64`:

```rust
let integer = Option::Some(5);
let float = Option::Some(5.0);
```

We've left in the `Option::` namespace for consistency with the previous
examples, but since `use Option::*` is in the prelude, it's not needed. Usually
using `Option` looks like this:

```rust
let integer = Some(5);
let float = Some(5.0);
```

### Removing Duplication by Extracting a Generic Data Type

Let's return to our two non-generic `enum`s to see how to get from duplicated
types to the generic type. Here are the definitions side-by-side:

```text
enum OptionalNumber {   enum OptionalFloatingPointNumber {
    Some(i32),              Some(f64),
    None,                   None,
}                       }
```

Aside from the names, we have one line where the two definitions are very
close, but still different: the line with the `Some` definitions. The only
difference is the type of the data in that variant, `i32` and `f64`.

Just like we can parameterize arguments to a function by choosing a name, we
can parameterize the type by choosing a name. In this case, we've chosen the
name `T`. We could choose any identifier here, but Rust style has type
parameters follow the same style as types themselves: CamelCase. In addition,
they tend to be short, often one letter. `T` is the traditional default choice,
short for 'type'. Let's use that name in our `Some` variant definitions where
the `i32` and `f64` types were:

```text
enum OptionalNumber {   enum OptionalFloatingPointNumber {
    Some(T),                Some(T),
    None,                   None,
}                       }
```

There's one problem, though: we've *used* `T`, but not defined it. This would
be similar to using an argument to a function in the body without declaring it
in the signature. We need to tell Rust that we've introduced a generic
parameter. The syntax to do that is the angle brackets, like this:

```text
enum OptionalNumber<T> {   enum OptionalFloatingPointNumber<T> {
    Some(T),                Some(T),
    None,                   None,
}                       }
```

The `<>`s after the enum name indicate a list of type parameters, just like
`()` after a function name indicates a list of value parameters. Now the only
difference between our two `enum`s is the name. Since we've made them generic,
they're not specific to integers or floating point numbers anymore, so they can
have the same name:

```text
enum Option<T> {	enum Option<T> {
    Some(T),            Some(T),
    None,               None,
}                   }
```

Now they're identical! We've made our type fully generic. When you recognize
situations with almost-duplicate types like this, you can follow this process
to reduce duplication using generics.

### Monomorphization at Compile Time

Understanding this refactoring process is also useful in understanding how
generics work behind the scenes: the compiler does the exact opposite of this
process when compiling your code. *Monomorphization* means taking code that is
generic over some type and generating code that is specific for the concrete
types that are used with the generic code. Monomorphization is why Rust's
generics are extremely efficient. Consider this code that uses the standard
library's `Option`:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it will perform monomorphization. What this means
is the compiler will see that we've used two kinds of `Option<T>`: one where
`T` is `i32`, and one where `T` is `f64`. As such, it will expand the generic
definition of `Option<T>` into `OptionInteger<i32>` and `OptionFloat<f64>`,
thereby replacing the generic definition with the specific ones. The more
specific version looks like the duplicated code we started with at the
beginning of this section:

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

In other words, we can write the non-duplicated form that uses generics in our
code, but Rust will compile that into code that acts as though we wrote the
specific type out in each instance. This means we pay no runtime cost for using
generics; it's just like we duplicated each particular definition.

### Generic Structs

In a similar fashion as we did with enums, we can use `<>`s with structs as
well in order to define structs that could hold data values of any type.
Listing 10-2 shows the definition and use of a `Point` struct that could hold
`x` and `y` coordinate values that are any type:

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

<caption>
Listing 10-2: A `Point` struct that holds `x` and `y` values of type `T`
</caption>

The syntax is the same with structs: add a `<T>` after the name of the struct,
then use `T` in the definition where you want to use that generic type instead
of a specific type.

### Multiple Type Parameters

Note that in the `Point` definition in Listing 10-2, we've used the same `T`
parameter for both fields. This means `x` and `y` must always be values of the
same type. Trying to instantiate a `Point` that uses an `i32` for `x` and an
`f64` for `y`, like this:

```rust,ignore
let p = Point { x: 5, y: 20.0 };
```

results in a compile-time error that indicates the type of `y` must match the
type of `x`:

```bash
error[E0308]: mismatched types
  |
7 | let p = Point { x: 5, y: 20.0 };
  |                          ^^^^ expected integral variable, found floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
```

If we need to be able to have fields with generic but different types, we can
declare multiple type parameters within the angle brackets, separated by a
comma. Listing 10-3 shows how to define a `Point` that can have different types
for `x` and `y`:

```rust
struct Point<X, Y> {
    x: X,
    y: Y,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
let p = Point { x: 5, y: 20.0 };
```

<caption>
Listing 10-2: A `Point` struct that holds an `x` value of type `X` and a `y`
value of type `Y`
</caption>

Now `x` will have the type of `X`, and `y` will have the type of `Y`, and we
can instantiate a `Point` with an `i32` for `x` and an `f64` for `y`.

We can make `enum`s with multiple type parameters as well. Recall the enum
`Result<T, E>` from Chapter 9 that we used for recoverable errors. Here's its
definition:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Each variant stores a different kind of information, and they're both generic.

You can have as many type parameters as you'd like. Similarly to parameters of
values in function signatures, if you have a lot of parameters, the code can
get quite confusing, so try to keep the number of parameters defined in any one
type small if you can.

### Generic Functions and Methods

In a similar way to data structures, we can use the `<>` syntax in function or
method definitions. The angle brackets for type parameters go after the
function or method name and before the argument list in parentheses:

```rust
fn generic_function<T>(argument: T) {
    // code goes here
}
```

We can use the same process that we used to refactor duplicated type
definitions using generics to refactor duplicated function definitions using
generics. Consider these two side-by-side function signatures that differ in
the type of `argument`:

```text
fn takes_integer(argument: i32) {       fn takes_float(argument: f64) {
    // code goes here                       // code goes here
}                                       }
```

We can add a type parameter list that declares the generic type `T` after the function names, then use `T` where the specific `i32` and `f64` types were:

```text
fn takes_integer<T>(argument: T) {    fn takes_float<T>(argument: T) {
    // code goes here                     // code goes here
}                                     }
```

At this point, only the names differ, so we could unify the two functions into one:

```rust,ignore
fn takes<T>(argument: T) {
    // code goes here
}
```

There's one problem though. We've got some function _definitions_ that work,
but if we try to use `argument` in code in the function body, we'll get an
error. For example, this function definition tries to print out the value of
`argument` in its body:

```rust,ignore
fn print<T>(argument: T) {
    println!("Got an argument: {}", argument);
}
```

Compiling this definition results in an error:

```bash
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
section, we'll learn how to make this compile.
