
[TOC]

# Generics

One of the core tools a programming language gives you is the ability to deal
effectively with duplication of code. It's important to minimize the amount of
code that is duplicated throughout a program to make maintenace easier and
minimize logic errors. Maintenance will be easier if there's only one place
that you need to change the code if you change your mind about how the program
should work, rather than multiple places in the code. If your program's logic
is duplicated in different places and those places don't match, you'll get
errors or unexpected and undesired behavior from your program that could be
hard to track down. Rust has the concept of *generics* as one way to eliminate
duplicate code. Generics come in the form of generic types, traits that those
generic types have, and generic lifetimes. We'll cover how to use all of these
in this chapter.

## Removing Duplication by Extracting a Function

Let's first go through a technique for dealing with duplication that you're
probably familiar with: extracting a function. Consider a small program that
finds the largest number in a list, shown in Listing 10-1:

Filename: src/main.rs

```rust
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if largest > number {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

<caption>
Listing 10-1: Code to find the largest number in a list of numbers
</caption>

If we needed to find the largest number in two different lists of numbers, we
could duplicate the code in Listing 10-1 and have the same logic exist in two
places in the program:

Filename: src/main.rs

```rust
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if largest > number {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if largest > number {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

Copying code is tedious and error-prone, plus now we have two places to update
the logic if we need it to change. Rust, like many languages, gives us a way to
deal with this duplication by creating an abstraction, and in this case the
abstraction we'll use is a function. Here's a program where we've extracted the
code in Listing 10-1 that finds the largest number into a function named
`largest`. This program can find the largest number in two different lists of
numbers, but the code from Listing 10-1 only exists in one spot:

Filename: src/main.rs

```rust
fn largest(numbers: Vec<i32>) {
    let mut largest = numbers[0];

    for number in numbers {
        if largest > number {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    largest(numbers);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    largest(numbers);
}
```

The function takes an argument, `numbers`, which represents any concrete
`Vec<i32>` that we might pass into the function. The code in the function
definition operates on the `numbers` representation of any `Vec<i32>`. When
we call the `largest` function, the code actually runs on the specific values
that we pass in.

Functions aren't the only way to eliminate duplication. For example, our
`largest` function only works for vectors of `i32`. What if we wanted to find
the largest number in a list of floats? Or the largest value in some sort of
custom `struct` or `enum`? We can't solve those kinds of duplication with
regular functions.

To solve these kinds of problems, Rust provides a feature called *generics*. In
the same way that functions allow us to abstract over common code, generics
allow us to abstract over types. This ability gives us tremendous power to
write code that works in a large number of situations. First, we'll examine the
syntax of generics. Then, we'll talk about another feature that's used to
augment generics: traits. Finally, we'll discuss one of Rust's most unique uses
of generics: lifetimes.

## Generics Syntax

We've already hinted at the idea of generics in previous chapters, but we
never dug into what exactly they are or how to use them. In places where we
specify a type, like function signatures or structs, instead we can use
*generics*. Generics are stand-ins that represent an abstract set instead of something concrete. In this section, we're going to cover generic *data types*.

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

Filename: src/main.rs

```rust
enum OptionalNumber {
    Some(i32),
    None,
}

fn main() {
    let number = OptionalNumber::Some(5);
    let no_number = OptionalNumber::None;
}
```

This works just fine for `i32`s. But what if we also wanted to store `f64`s? We
would have to duplicate code to define a separate `Option` enum type for each
type we wanted to be able to hold in the `Some` variants. For example, here is
how we could define and use `OptionalFloatingPointNumber`:

Filename: src/main.rs

```rust
enum OptionalFloatingPointNumber {
    Some(f64),
    None,
}

fn main() {
    let number = OptionalFloatingPointNumber::Some(5.0);
    let no_number = OptionalFloatingPointNumber::None;
}
```

We've made the enum's name a bit long in order to drive the point home. With
what we currently know how to do in Rust, we would have to write a unique type
for every single kind of value we wanted to have either `Some` or `None` of. In
other words, the idea of "an optional value" is a more abstract concept than one
specific type. We want it to work for any type at all.

### Removing Duplication by Extracting a Generic Data Type

Let's see how to get from duplicated types to the generic type. Here are the
definitions of our two enums side-by-side:

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

Now they're identical! We've made our type fully generic. This definition is
also how `Option` is defined in the standard library. If we were to read this
definition aloud, we'd say, "`Option` is an `enum` with one type parameter,
`T`. It has two variants: `Some`, which has a value with type `T`, and `None`,
which has no value." We can now use the same `Option` type whether we're holding an `i32` or an `f64`:

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

When you recognize situations with almost-duplicate types like this in your
code, you can follow this process to reduce duplication using generics.

### Monomorphization at Compile Time

Understanding this refactoring process is also useful in understanding how
generics work behind the scenes: the compiler does the exact opposite of this
process when compiling your code. *Monomorphization* means taking code that
uses generic type parameters and generating code that is specific for each
concrete type that is used with the generic code. Monomorphization is why
Rust's generics are extremely efficient at runtime. Consider this code that
uses the standard library's `Option`:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it will perform monomorphization. What this means
is the compiler will see that we've used two kinds of `Option<T>`: one where
`T` is `i32`, and one where `T` is `f64`. As such, it will expand the generic
definition of `Option<T>` into `Option_i32` and `Option_f64`, thereby replacing
the generic definition with the specific ones. The more specific version looks
like the duplicated code we started with at the beginning of this section:

Filename: src/main.rs

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

In other words, we can write the non-duplicated form that uses generics in our
code, but Rust will compile that into code that acts as though we wrote the
specific type out in each instance. This means we pay no runtime cost for using
generics; it's just like we duplicated each particular definition.

### Generic Structs

In a similar fashion as we did with enums, we can use `<>`s with structs as
well in order to define structs that have a generic type parameter in one or
more of their fields. Generic structs also get monomorphized into specialized
types at compile time. Listing 10-2 shows the definition and use of a `Point`
struct that could hold `x` and `y` coordinate values that are any type:

Filename: src/main.rs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
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

Filename: src/main.rs

```rust
struct Point<X, Y> {
    x: X,
    y: Y,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point { x: 5, y: 20.0 };
}
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
fn generic_function<T>(value: T) {
    // code goes here
}
```

We can use the same process that we used to refactor duplicated type
definitions using generics to refactor duplicated function definitions using
generics. Consider these two side-by-side function signatures that differ in
the type of `value`:

```text
fn takes_integer(value: i32) {          fn takes_float(value: f64) {
    // code goes here                       // code goes here
}                                       }
```

We can add a type parameter list that declares the generic type `T` after the
function names, then use `T` where the specific `i32` and `f64` types were:

```text
fn takes_integer<T>(value: T) {       fn takes_float<T>(value: T) {
    // code goes here                     // code goes here
}                                     }
```

At this point, only the names differ, so we could unify the two functions into
one:

```rust,ignore
fn takes<T>(value: T) {
    // code goes here
}
```

There's one problem though. We've got some function *definitions* that work,
but if we try to use `value` in code in the function body, we'll get an
error. For example, the function definition in Listing 10-3 tries to print out
`value` in its body:

Filename: src/lib.rs

```rust,ignore
fn show_anything<T>(value: T) {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

<caption>
Listing 10-3: A `show_anything` function definition that does not yet compile
</caption>

Compiling this definition results in an error:

```bash
	error[E0277]: the trait bound `T: std::fmt::Display` is not satisfied
 --> <anon>:3:37
  |
3 |     println!("It's: {}", value);
  |                          ^^^^^ trait `T: std::fmt::Display` not satisfied
  |
  = help: consider adding a `where T: std::fmt::Display` bound
  = note: required by `std::fmt::Display::fmt`

error: aborting due to previous error(s)
```

This error mentions something we haven't learned about yet: traits. In the next
section, we'll learn how to make this compile.

## Traits

*Traits* are similar to a feature often called 'interfaces' in other languages,
but are also different. Traits let us do another kind of abstraction: they let
us abstract over *behavior* that types can have in common.

When we use a generic type parameter, we are telling Rust that any type is
valid in that location. When other code *uses* a value that could be of any
type, we need to also tell Rust that the type has the functionality that we
need. Traits let us specify that, for example, we need any type `T` that has
methods defined on it that allow us to print a value of that type. This is
powerful because we can still leave our definitions generic to allow use of
many different types, but we can constrain the type at compile-time to types
that have the behavior we need to be able to use.

Here's an example definition of a trait named `Printable` that has a method
named `print`:

Filename: src/lib.rs

```rust
trait Printable {
    fn print(&self);
}
```

<caption>
Listing 10-4: A `Printable` trait definition with one method, `print`
</caption>

We declare a trait with the `trait` keyword, then the trait's name. In this
case, our trait will describe types which can be printed. Inside of curly
braces, we declare a method signature, but instead of providing an
implementation inside curly braces, we put a semicolon after the signature. A
trait can have multiple methods in its body, with the method signatures listend one per line and each line ending in a semicolon.

Implementing a trait for a particular type looks similar to implementing
methods on a type since it's also done with the `impl` keyword, but we specify
the trait name as well. Inside the `impl` block, we specify definitions for the
trait's methods in the context of the specific type. Listing 10-5 has an
example of implementing the `Printable` trait from Listing 10-4 (that only has
the `print` method) for a `Temperature` enum:

Filename: src/lib.rs

```rust
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

impl Printable for Temperature {
    fn print(&self) {
        match *self {
            Temperature::Celsius(val) => println!("{}°C", val),
            Temperature::Fahrenheit(val) => println!("{}°F", val),
        }
    }
}
```

<caption>
Listing 10-5: Implementing the `Printable` trait on a `Temperature` enum
</caption>

In the same way `impl` lets us define methods, we've used it to define methods
that pertain to our trait. We can call methods that our trait has defined just
like we can call other methods:

Filename: src/main.rs

```rust
fn main() {
    let t = Temperature::Celsius(37);

    t.print();
}
```

Note that in order to use a trait's methods, the trait itself must be in scope.
If the definition of `Printable` was in a module, the definition would need to
be defined as `pub` and we would need to `use` the trait in the scope where we
wanted to call the `print` method. This is because it's possible to have two
traits that both define a method named `print`, and our `Temperature` enum might
implement both. Rust wouldn't know which `print` method we wanted unless we
brought the trait we wanted into our current scope with `use`.

### Trait Bounds

Defining traits with methods and implementing the trait methods on a particular
type gives Rust more information than just defining methods on a type directly.
The information Rust gets is that the type that implements the trait can be
used in places where the code specifies that it needs some type that implements
a trait. To illustrate this, Listing 10-6 has a `print_anything` function
definition. This is similar to the `show_anything` function from Listing 10-3,
but this function has a *trait bound* on the generic type `T` and uses the
`print` function from the trait. A trait bound constrains the generic type to
be any type that implements the trait specified, instead of any type at all.
With the trait bound, we're then allowed to use the trait method `print` in the
function body:

Filename: src/lib.rs

```rust
fn print_anything<T: Printable>(value: T) {
    println!("I have something to print for you!");
    value.print();
}
```

<caption>
Listing 10-6: A `print_anything` function that uses the trait bound `Printable`
on type `T`
</caption>

Trait bounds are specified in the type name declarations within the angle
brackets. After the name of the type that you want to apply the bound to, add a
colon (`:`) and then specify the name of the trait. This function now specifies
that it takes a `value` parameter that can be of any type, as long as that type
implements the trait `Printable`. We need to specify the `Printable` trait in
the type name declarations because we want to be able to call the `print`
method that is part of the `Printable` trait.

Now we are able to call the `print_anything` function from Listing 10-6 and
pass it a `Temperature` instance as the `value` parameter, since we implemented
the trait `Printable` on `Temperature` in Listing 10-5:

Filename: src/main.rs

```rust
fn main() {
    let temperature = Temperature::Fahrenheit(98);
    print_anything(temperature);
}
```

If we implement the `Printable` trait on other types, we can use them with the
`print_anything` method too. If we try to call `print_anything` with an `i32`,
which does *not* implement the `Printable` trait, we get a compile-time error
that looks like this:

```bash
error[E0277]: the trait bound `{integer}: Printable` is not satisfied
   |
29 | print_anything(3);
   | ^^^^^^^^^^^^^^ trait `{integer}: Printable` not satisfied
   |
   = help: the following implementations were found:
   = help:   <Point as Printable>
   = note: required by `print_anything`
```

Traits are an extremely useful feature of Rust. You'll almost never see generic
functions without an accompanying trait bound. There are many traits in the
standard library, and they're used for many, many different things. For
example, our `Printable` trait is similar to one of those traits, `Display`.
And in fact, that's how `println!` decides how to format things with `{}`. The
`Display` trait has a `fmt` method that determines how to format something.

Listing 10-7 shows our original example from Listing 10-3, but this time using
the standard library's `Display` trait in the trait bound on the generic type
in the `show_anything` function:

Filename: src/lib.rs

```rust
use std::fmt::Display;

fn show_anything<T: Display>(value: T) {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

<caption>
Listing 10-7: The `show_anything` function with trait bounds
</caption>

Now that this function specifies that `T` can be any type as long as that type
implements the `Display` trait, this code will compile.

### Multiple Trait Bounds and `where` Syntax

Each generic type can have its own trait bounds. The signature for a function
that takes a type `T` that implements `Display` and a type `U` that implements
`Printable` looks like:

```rust,ignore
fn some_function<T: Display, U: Printable>(value: T, other_value: U) {
```

To specify multiple trait bounds on one type, list the trait bounds in a list
with a `+` between each trait. For example, here's the signature of a function
that takes a type `T` that implements `Display` and `Clone` (which is another
standard library trait we have mentioned):

```rust,ignore
fn some_function<T: Display + Clone>(value: T) {
```

When trait bounds start getting complicated, there is another syntax that's a
bit cleaner: `where`. And in fact, the error we got when we ran the code from
Listing 10-3 referred to it:

```bash
help: consider adding a `where T: std::fmt::Display` bound
```

The `where` syntax moves the trait bounds after the function arguments list.
This definition of `show_anything` means the exact same thing as the definition
in Listing 10-7, just said a different way:

Filename: src/lib.rs

```rust
use std::fmt::Display;

fn show_anything<T>(value: T) where T: Display {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

Instead of `T: Display` going inside the angle brackets, they go after the
`where` keyword at the end of the function signature. This can make complex
signatures easier to read. The `where` clause and its parts can also go on new
lines. Here's the signature of a function that takes three generic type
parameters that each have multiple trait bounds:

```rust,ignore
fn some_function<T, U, V>(t: T, u: U, v: V)
    where T: Display + Clone,
          U: Printable + Debug,
          V: Clone + Printable
{
```

Generic type parameters and trait bounds are part of Rust's rich type system.
Another important kind of generic in Rust interacts with Rust's ownership and
references features, and they're called *lifetimes*.

## Lifetime Syntax

Generic type parameters let us abstract over types, and traits let us abstract
over behavior. There's one more way that Rust allows us to do something
similar: *lifetimes* allow us to be generic over scopes of code.

Scopes of code? Yes, it's a bit unusual. Lifetimes are, in some ways, Rust's
most distinctive feature. They are a bit different than the tools you have used
in other programming languages. Lifetimes are a big topic, so we're not going
to cover everything about them in this chapter. What we *are* going to do is
talk about the very basics of lifetimes, so that when you see the syntax in
documentation or other places, you'll be familiar with the concepts. Chapter 20
will contain more advanced information about everything lifetimes can do.

### Core Syntax

We talked about references in Chapter 4, but we left out an important detail.
As it turns out, every reference in Rust has a *lifetime*, which is the scope
for which that reference is valid. Most of the time, lifetimes are implicit,
but just like we can choose to annotate types everywhere, we can choose to
annotate lifetimes.

Lifetimes have a slightly unusual syntax:

```rust,ignore
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
```

The `'a` there is a *lifetime* with the name `a`. A single apostrophe indicates
that this name is for a lifetime. Lifetime names need to be declared before
they're used. Here's a function signature with lifetime declarations and
annotations:

```rust,ignore
fn some_function<'a>(argument: &'a i32) {
```

Notice anything? In the same way that generic type declarations go inside angle
brackets after the function name, lifetime declarations also go inside those
same angle brackets. We can even write functions that take both a lifetime
declaration and a generic type declaration:

```rust,ignore
fn some_function<'a, T>(argument: &'a T) {
```

This function takes one argument, a reference to some type, `T`, and the
reference has the lifetime `'a`. In the same way that we parameterize functions
that take generic types, we parameterize references with lifetimes.

So, that's the syntax, but *why*? What does a lifetime do, anyway?

### Lifetimes Prevent Dangling References

Consider the program in listing 10-8. There's an outer scope and an inner
scope. The outer scope declares a variable named `r` with no initial value, and
the inner scope declares a variable named `x` with the initial value of 5.
Inside the inner scope, we attempt to set the value of `r` to a reference to
`x`. Then the inner scope ends and we attempt to print out the value in `r`:

```rust,ignore
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

<caption>
Listing 10-8: An attempt to use a reference whose value has gone out of scope
</caption>

If we compile this code, we get an error:

```text
	error: `x` does not live long enough
  --> <anon>:6:10
   |
6  |     r = &x;
   |          ^ does not live long enough
7  | }
   | - borrowed value only lives until here
...
10 | }
   | - borrowed value needs to live until here
```

The variable `x` doesn't "live long enough." Why not? Well, `x` is going to go
out of scope when we hit the closing curly brace on line 7, ending the inner
scope. But `r` is valid for the outer scope; its scope is larger and we say
that it "lives longer." If Rust allowed this code to work, `r` would be
referencing memory that was deallocated when `x` went out of scope. That'd be
bad! Once it's deallocated, it's meaningless.

So how does Rust determine that this code should not be allowed? Part of the
compiler called the *borrow checker* compares scopes to determine that all
borrows are valid. Here's the same example from Listing 10-8 with some
annotations:

```rust,ignore
{
    let r;         // -------+-- 'a
                   //        |
    {              //        |
        let x = 5; // -+-----+-- 'b
        r = &x;    //  |     |
    }              // -+     |
                   //        |
    println!("r: {}", r); // |
                   //        |
                   // -------+
}
```

Here, we've annotated the lifetime of `r` with `'a` and the lifetime of `x`
with `'b`. Rust looks at these lifetimes and sees that `r` has a lifetime of
`'a`, but that it refers to something with a lifetime of `'b`. It rejects the
program because the lifetime `'b` is shorter than the lifetime of `'a`-- the
value that the reference is referring to does not live as long as the reference
does.

Let's look at a different example that compiles because it does not try to make
a dangling reference, and see what the lifetimes look like:

```rust
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
                          // -----+
}
```

Here, `x` lives for `'b`, which in this case is larger than `'a`. This is
allowed: Rust knows that the reference in `r` will always be valid, as it has a
smaller scope than `x`, the value it refers to.

Note that we didn't have to name any lifetimes in the code itself; Rust figured
it out for us. One situation in which Rust can't figure out the lifetimes is
for a function or method when one of the arguments or return values is a
reference, except for a few scenarios we'll discuss in the lifetime elision
section.

### Lifetime Annotations in Struct Definitions

Another time that Rust can't figure out the lifetimes is when structs have a
field that holds a reference. In that case, naming the lifetimes looks like
this:

```rust
struct Ref<'a> {
    x: &'a i32,
}
```

Again, the lifetime names are declared in the angle brackets where generic type
parameters are declared, and this is because lifetimes are a form of generics.
In the examples above, `'a` and `'b` were concrete lifetimes: we knew about `r`
and `x` and how long they would live exactly. However, when we write a
function, we can't know beforehand exactly all of the arguments that it could
be called with and how long they will be valid for. We have to explain to Rust
what we expect the lifetime of the argument to be (we'll learn about how
to know what you expect the lifetime to be in a bit). This is similar to
writing a function that has an argument of a generic type: we don't know what
type the arguments will actually end up being when the function gets called.
Lifetimes are the same idea, but they are generic over the scope of a
reference, rather than a type.

### Lifetime Annotations in Function Signatures

Lifetime annotations for functions go on the function signature, but we don't
have to annotate any of the code in the function body with lifetimes. That's
because Rust can analyze the specific code inside the function without any
help. When a function interacts with references that come from or go to code
outside that function, however, the lifetimes of those arguments or return
values will potentially be different each time that function gets called. Rust
would have to analyze every place the function is called to determine that
there were no dangling references. That would be impossible because a library
that you provide to someone else might be called in code that hasn't been
written yet, at the time that you're compiling your library.

Lifetime parameters specify generic lifetimes that will apply to any specific
lifetimes the function gets called with. The annotation of lifetime parameters
tell Rust what it needs to know in order to be able to analyze a function
without knowing about all possible calling code. Lifetime annotations do not
change how long any of the references involved live. In the same way that
functions can accept any type when the signature specifies a generic type
parameter, functions can accept references with any lifetime when the signature
specifies a generic lifetime parameter.

To understand lifetime annotations in context, let's write a function that will
return the longest of two string slices. The way we want to be able to call
this function is by passing two string slices, and we want to get back a string
slice. The code in Listing 10-9 should print `The longest string is abcd` once
we've implemented the `longest` function:

Filename: src/main.rs

```rust
fn main() {
    let a = String::from("abcd");
    let b = "xyz";

    let c = longest(a.as_str(), b);
    println!("The longest string is {}", c);
}
```

<caption>
Listing 10-9: A `main` function that demonstrates how we'd like to use the
`longest` function
</caption>

Note that we want the function to take string slices because we don't want the
`longest` function to take ownership of its arguments, and we want the function
to be able to accept slices of a `String` (like `a`) is as well as string
literals (`b`). Refer back to the "String Slices as Arguments" section of
Chapter 4 for more discussion about why these are the arguments we want.

Here's the start of an implementation of the `longest` function that won't
compile yet:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

If we try to compile this, we get an error that talks about lifetimes:

```text
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

The help text is telling us that the return type needs a generic lifetime
parameter on it because this function is returning a reference and Rust can't
tell if the reference being returned refers to `x` or `y`. Actually, we don't
know either, since in the `if` block in the body of this function returns a
reference to `x` and the `else` block returns a reference to `y`! The way to
specify the lifetime parameters in this case is to have the same lifetime for
all of the input parameters and the return type:

Filename: src/main.rs

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This will compile and will produce the result we want with the `main` function
in Listing 10-9. This function signature is now saying that for some lifetime
named `'a`, it will get two arguments, both which are string slices that live
at least as long as the lifetime `'a`. The function will return a string slice
that also will last at least as long as the lifetime `'a`. This is the contract
we are telling Rust we want it to enforce. By specifying the lifetime
parameters in this function signature, we are not changing the lifetimes of any
values passed in or returned, but we are saying that any values that do not
adhere to this contract should be rejected by the borrow checker. This function
does not know (or need to know) exactly how long `x` and `y` will live since it
knows that there is some scope that can be substituted for `'a` that will
satisfy this signature.

The exact way to specify lifetime parameters depends on what your function is
doing. If the function didn't actually return the longest string slice but
instead always returned the first argument, we wouldn't need to specify a
lifetime on `y`. This code compiles:

Filename: src/main.rs

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

The lifetime parameter for the return type needs to be specified and needs to
match one of the arguments' lifetime parameters. If the reference returned does
*not* refer to one of the arguments, the only other possibility is that it
refers to a value created within this function, and that would be a dangling
reference since the value will go out of scope at the end of the function.
Consider this attempted implementation of `longest`:

Filename: src/main.rs

```rust,ignore
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Even though we've specified a lifetime for the return type, this function fails
to compile with the following error message:

```text
error: `result` does not live long enough
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the block at 1:44...
  |
1 | fn longest<'a>(x: &str, y: &str) -> &'a str {
  |                                             ^
```

The problem is that `result` will go out of scope and get cleaned up at the end
of the `longest` function, and we're trying to return a reference to `result`
from the function. There's no way we can specify lifetime parameters that would
change the dangling reference, and Rust won't let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so that the calling function is then responsible for
cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various
arguments and return values of functions. Once they're connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.

### Lifetime Elision

If every reference has a lifetime, and we need to provide them for functions
that use references as arguments or return values, then why did this function
from the "String Slices" section of Chapter 4 compile? We haven't annotated any
lifetimes here, yet Rust happily compiles this function:

Filename: src/lib.rs

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

The answer is historical: in early versions of pre-1.0 Rust, this would not
have compiled. Every reference needed an explicit lifetime. At that time, the
function signature would have been written like this:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

After writing a lot of Rust code, some patterns developed. The Rust team
noticed that the vast majority of code followed the pattern, and being forced
to use explicit lifetime syntax on every reference wasn't a very great
developer experience.

To make it so that lifetime annotations weren't needed as often, they added
*lifetime elision rules* to Rust's analysis of references. This feature isn't
full inference: Rust doesn't try to guess what you meant in places where there
could be ambiguity. The rules are a very basic set of particular cases, and if
your code fits one of those cases, you don't need to write the lifetimes
explicitly. Here are the rules:

Lifetimes on function arguments are called *input lifetimes*, and lifetimes on
return values are called *output lifetimes*. There's one rule related to how
Rust infers input lifetimes in the absence of explicit annotations:

1. Each argument that is a reference and therefore needs a lifetime parameter
  gets its own. In other words, a function with one argument gets one lifetime
  parameter: `fn foo<'a>(x: &'a i32)`, a function with two arguments gets two
  separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, and
  so on.

And two rules related to output lifetimes:

2. If there is exactly one input lifetime parameter, that lifetime is assigned
  to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. If there are multiple input lifetime parameters, but one of them is `&self`
  or `&mut self`, then the lifetime of `self` is the lifetime assigned to all
  output lifetime parameters. This makes writing methods much nicer.

If none of these three rules apply, then you must explicitly annotate input and
output lifetimes. These rules do apply in the `first_word` function, which is
why we didn't have to specify any lifetimes.

These rules cover the vast majority of cases, allowing you to write a lot of
code without needing to specify explicit lifetimes. However, Rust is always
checking these rules and the lifetimes in your program, and cases in which the
lifetime elision rules do not apply are cases where you'll need to add lifetime
parameters to help Rust understand the contracts of your code.

### Lifetime Annotations in Method Definitions

Now that we've gone over the lifetime elision rules, defining methods on
structs that hold references will make more sense. The lifetime name needs to
be declared after the `impl` keyword and then used after the struct's name,
since the lifetime is part of the struct's type. The lifetimes can be elided in
any methods where the output type's lifetime is the same as that of the
struct's because of the third elision rule. Here's a struct called `App` that
holds a reference to another struct, `Config`, defined elsewhere. The
`append_to_name` method does not need lifetime annotations even though the
method has a reference as an argument and is returning a reference; the
lifetime of the return value will be the lifetime of `self`:

Filename: src/lib.rs

```rust
struct App<'a> {
    name: String,
    config: &'a Config,
}

impl<'a> App<'a> {
    fn append_to_name(&mut self, suffix: &str) -> &str {
        self.name.push_str(suffix);
        self.name.as_str()
    }
}
```

### The Static Lifetime

There is *one* special lifetime that Rust knows about: `'static`. The `'static`
lifetime is the entire duration of the program. All string literals have the
`'static` lifetime:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the binary of your program and
the binary of your program is always available. Therefore, the lifetime of all
string literals is `'static`. You may see suggestions to use the `'static`
lifetime in error message help text, but before adding it, think about whether
the reference you have is one that actually lives the entire lifetime of your
program or not (or even if you want it to live that long, if it could). Most of
the time, the problem in the code is an attempt to create a dangling reference
or a mismatch of the available lifetimes, and the solution is fixing those
problems, not specifying the `'static` lifetime.

## Summary

We've covered the basics of Rust's system of generics. Generics are the core to
building good abstractions, and can be used in a number of ways. There's more
to learn about them, particularly lifetimes, but we'll cover those in later
chapters. Let's move on to I/O functionality.
