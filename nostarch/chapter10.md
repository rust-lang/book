
[TOC]

<!-- General note: I think this chapter could be made more interactive, could
you try altering some of the programs to encourage the reader to try them out,
see the concepts in action? We're using a lot of hypothetical examples,
somthing more hands-on might help a reader get used to using the syntax more -->

# Generic Types, Traits, and Lifetimes

Every programming language has tools to deal effectively with duplication of
code; in Rust, one of those tools is *generics*.

Minimizing duplicated code in a program is important to reduce risk of logic
errors: if the logic in multiple places in your code doesn't match, you'll get
errors or unexpected and undesired behavior that can be hard to track down.
Code is also easier to maintain if there's you only need to change it in one
place when you make a new decision about how the program should work, rather
than having to find multiple places in the code.

Rust's concept of *generics* gives one way to eliminate duplicate code.
Generics allows you to to abstract over type, behavior, and scope in a
function; a program that uses a generic type may, for example, be able to
accept any type as input rather than being restricted to just integers or
strings. We've seen generics briefly in previous chapters, when ...

<!-- Could you give an early, outright definition of what we mean by generics?
I've started sample text above -->

Generics come in the form of generic types, traits that those generic types
have, and generic lifetimes. We'll cover how to use each of these in this
chapter.

## Removing Duplication by Extracting a Function

Before gettng into generics syntax, let's first go through a technique for
dealing with duplication that doesn't use generics : extracting a function.
Consider a small program that finds the largest number in a list, shown in
Listing 10-1:

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

Listing 10-1: Code to find the largest number in a list of numbers

<!--Could you highlight the main functionality in text, make sure they're
following? -->

This is simple code that...

If we needed to find the largest number in two different lists of numbers, we
could duplicate the code in Listing 10-1 and have the same logic exist in two
places in the program, as in Listing 10-2:

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

Listing 10-2: Code to find the largest number in *two* lists of numbers

While this code works, duplicating code is tedious and error-prone, and means
we have multiple places to update the logic if we need to change it.

<!-- Are we safe assuming the reader will be familiar with the term
"abstraction" in this context, or do we want to give a brief definition? -->

To deal with this duplication more simply we can create an *abstraction*; in
this case we'll abstract out the function to remove duplication. In the program
in Listing 10-3 we've extracted the code that finds the largest number into a
function named `largest`. This program can find the largest number in two
different lists of numbers, but the code copied from Listing 10-1 only exists
in one spot:

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

Listing 10-3: Abstracted code to find the largest number in two lists

The function takes an argument, `numbers`, which represents any concrete
`Vec<i32>` that we might pass into the function. The code in the function
definition operates on the `numbers` representation of any `Vec<i32>`. When we
call the `largest` function, the code actually runs on the specific values that
we pass in.

<!-- Can you make it clear why we're showing them this second, non-generic,
example rather than going straight to the generics solution? What is this
showing the reader? -->

While this code works, the function extraction method has its limitations. For
example, our `largest` function only works for vectors of `i32`. What if we
wanted to find the largest number in a list of floats? Or the largest value in
some sort of custom `struct` or `enum`? We can't solve those kinds of
duplication with regular functions.

<!--- Do we mean to use generics instead of the function extraction method, or
along with it? -->

Instead, we use Rust's *generics*. In the same way that functions allow us to
abstract over common code, generics allow us to abstract over types. This
ability gives us tremendous power to write code that works in a large number of
situations. First, we'll examine the syntax of generics and discuss generic
types.

## Generic Data Types

We've mentioned generics in previous chapters, but we never dug into what
exactly they are or how to use them. A generic type is like a placeholder type:
in places where we specify a type, like function signatures or structs, we can
instead use a *generic* as a stand-in vlaue that represent an abstract set
rather than a concrete value.

### Syntax of Generics

A generic type is recognizable by its syntax: any time you see angle brackets,
`<>`, you're dealing with generics, like in Chapter 8 where we discussed
vectors with the type `Vec<i32>`. The type defined by the standard library for
vectors is `Vec<T>`. That `T` is a *type parameter*, and serves a similar
function as parameters to functions: you fill in the parameter with a concrete
type (i32, here), and that determines how the overall type works. In the same
way that a function like `foo(x: i32)` can be called with a specific value such
as `foo(5)`, a `Vec<T>` can be created with a specific type, like `Vec<i32>`.

### Enum Definitions Using Generics

Let's dive into generic data types in more detail.

<!-- What are we writing here? Why are we jumping to generics, can you set this
up a little? -->

We'll use our highest number in two lists examples, and write a program like
the one in Listing 10-X that removes duplication, but this time we'll extract a
generic data type instead of extracting a function.

<!--What are we using the option enum for, exactly? -->

We'll use the `Option<T>` enum that we first saw in Chapter 6. First, let's
solve our problem with a `Some` variant that can only hold an `i32`, shown in
Listing 10-4. We'll call this enum `OptionalNumber`:

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
Listing 10-4: Using an enum to find the largest number in two lists

<!-- Can you talk this through briefly, show how it works so they can see the
difference when we write the new function? Also, is that right, this is for 2
lists? -->

This works just fine for `i32`s, but for no other type. If we also wanted to
store `f64`s, we'd have to duplicate code to define a separate `Option` enum
type for that type, and any type we wanted the `Some` variants to be able to
hold. Listing 10-5 shows the enum `OptionalFloatingPointNumber` that can store
floating points:

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

With what we currently know how to do in Rust, we would have to write a unique
type for every single kind of value we want `Some` or `None` to be able to
hold. The idea of "an optional value" is a more abstract concept than one
specific type, but we want our program to work for any type.

#### Removing Duplication by Extracting a Generic Data Type

Let's see how to get from two duplicated types to one generic type we can use
in our program. Here are the definitions of our two enums side-by-side:

```text
enum OptionalNumber {   enum OptionalFloatingPointNumber {
    Some(i32),              Some(f64),
    None,                   None,
}                       }
```

The two enums have different names, and a different data type in the variant
`Some`, but are otherwise the same.

To parameterize these enums we need to create a name for the parameters, just
like how we parameterize arguments to a function. In this case, we choose the
name `T`---any identifier can be used here, but we choose `T` because Rust's
default style is CamelCase,, and because type parameters they tend to be short,
often just one letter. Short for "type", `T` is the traditional default choice.

Let's replace the specific types in our `Some` variant definitions with `T`:

```text
enum OptionalNumber {   enum OptionalFloatingPointNumber {
    Some(T),                Some(T),
    None,                   None,
}                       }
```

The content of the enum definitions are now the same, but there's a problem:
we've *used* `T`, but not defined it. This would be similar to using an
argument in the body of a function without declaring it in the signature. We
need to tell Rust that we've introduced a generic parameter. We use the angle
bracket syntax to introduce generic parameters, like so:

```text
enum OptionalNumber<T> {   enum OptionalFloatingPointNumber<T> {
    Some(T),                Some(T),
    None,                   None,
}                       }
```

The `<>`s after the enum name indicate a list of type parameters, just like
`()` after a function name indicates a list of value parameters.

Now the only difference between our two `enum`s is in the name, and since we've
made them generic and no longer specific to integers or floating point numbers,
we can give them the same name, as in Listing 10-X.

```text
enum Option<T> {	enum Option<T> {
    Some(T),            Some(T),
    None,               None,
}                   }
```

Listing 10-X: The Option enum defition

Now they're identical! We've made our type fully generic. This definition is
also how `Option` is defined in the standard library. In plain English, we
might read this definition as: "`Option` is an `enum` with one type parameter,
`T`. It has two variants: `Some`, with a value with type `T`; and `None`, with
no value."

We can now use the same `Option` type whether we're holding an `i32` or an
`f64`:

```rust
let integer = Option::Some(5);
let float = Option::Some(5.0);
```

> Note: Here we've left in the `Option::` namespace for consistency with the
> previous examples, but actually since `use Option::*` is in the prelude, the
> namespace is not needed. We can instead use `Option` like this:
>
> ```rust
> let integer = Some(5);
> let float = Some(5.0);
> ```

When you recognize situations with almost-duplicate types like this in your
code, you can follow this process to reduce duplication and accept any type
using generics.

### Monomorphization at Compile Time

Understanding this refactoring process is also useful in understanding how
generics work behind the scenes: the compiler does the exact opposite of this
process when compiling your code. *Monomorphization* means taking code with
generic type parameters and generating code that is specific for each concrete
type used with the generic code.

<!-- I think I'm following, so monomorphization is the process that turns the
generic into a specific value type once indication of the type is given, is
that right? I'm not 100%, but maybe something like:

Monomorphization is the process that transforms the generic type into a
specific type, once a value is given to indicate what type the generic should be
? -->

Consider this code that uses the standard library's `Option` enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it will perform monomorphization. The compiler
will read the values that have been passed to `Option` and see that we have two
kinds of `Option<T>`: one is `i32`, and one is `f64`. As such, it will expand
the generic definition of `Option<T>` into `Option_i32` and `Option_f64`,
thereby replacing the generic definition with the specific ones.

The monomorpohized version expanded out looks like the duplicated code we
started with at the beginning of this section:

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

We can write the non-duplicated code using generics, and Rust will compile that
into code that specifies the type in each instance, meaning we pay no runtime
cost for using generics; it's just like we duplicated each particular
definition. This process is what makes Rust's generics extremely efficient at
runtime.

### Generic Structs

We can define structs to use a generic type parameter in one or more of the
struct's fields with the `<>` syntax too. Generic structs will also get
monomorphized into specialized types at compile time. Listing 10-6 shows the
definition and use of a `Point` struct that can hold `x` and `y` coordinate
values of any type:

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

Listing 10-6: A `Point` struct that holds `x` and `y` values of type `T`

The syntax is the same as with enums: define the generic by adding a `<T>`
after the name of the struct, then use `T` in the places in the definition you
want to use that generic type.

### Multiple Type Parameters

Note that in the `Point` definition in Listing 10-6, we've used the same `T`
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

To create a struct with generic type fields that can take take types different
types, we can declare multiple type parameters within the angle brackets.
Listing 10-7 shows how to define a `Point` struct that can have different types
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

Listing 10-7: A `Point` struct that can have different types for `x` and `y`
values

The brackets can hold as many generic parameters as you want, each separated by
a comma. Now `x` will have the type of `X`, and `y` will have the type of `Y`,
and we can instantiate a `Point` with an `i32` for `x` and an `f64` for `y`.

Enums can also have multiple type parameters. Recall the enum `Result<T, E>`
from Chapter 9 that we used for recoverable errors. Here's its definition:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Each variant stores different kind of information, and they're both generic.
For example, `Ok` might return a ...

<!-- perhaps an example, something like "OK needs to return an integer while
the error has to be delivered in text form so needs to be able to hold a
string"...?-->

Though you can have as many type parameters as you'd like, it's useful to
remember though that too many parameters can make your code confusing, so try
to keep the number of parameters defined in any one type small where you can.

### Generic Functions and Methods

The `<>` syntax can also be used to introduce generic types in function or
method definitions. Place angle brackets for type parameters after the function
or method name and before the argument list in parentheses, like so:

```rust
fn generic_function<T>(value: T) {
    // code goes here
}
```

To refactor duplicated function definitions with generics, you use the same
process as we do for enums and structs. Consider these two side-by-side
function signatures that differ in the type of `value`:

```text
fn takes_integer(value: i32) {          fn takes_float(value: f64) {
    // code goes here                       // code goes here
}                                       }
```

We'll add a type parameter list that declares the generic type `T` after the
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

Two functions reduced into one! There's one problem though. The function
*definitions* work, but if we try to use `value` in code in the function body,
we'll get an error. For example, the function definition in Listing 10-8 tries
to print out `value` in its body:

Filename: src/lib.rs

```rust,ignore
fn show_anything<T>(value: T) {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

Listing 10-8: A `show_anything` function definition that does not yet compile

Compiling this definition results in this error:

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

<!--I feel like we should give some indication, even a small one, as to why
this didn't work before moving on---is this because our code implies some
condition our value doesn't meet? Otherwise, maybe we should introduce function
generics after discussing traits? It feels a little misleading -->

## Abstracting Behaviors with Traits

Traits allow us another kind of abstraction: they let us abstract over
*behavior* that types can have in common. A trait tells the Rust compiler about
functionality a type must provide for it to be accepted by the compiler.

<!-- I copied this line from the existing online documentation, I think it's a
nice little summary, gives you an idea of what we're focusing on up front -->

> Note: *Traits* are similar to a feature often called 'interfaces' in other
> languages, though with some differences.

When we use a generic type parameter, we are telling Rust that any type is
valid in that location. When other code *uses* a generic value
type, we need to also tell Rust what functionality we need the type to have.

<!-- I'm not clear whether traits allow to the user to define the kind of
functionality a generic type needs to have, or just describe the functionality
of the type? Also, do we * need to tell Rust what functionality the type must
need? I don't think we have in the past -->

<!-- I struggled to follow this definition, in <> below, but I'm wary of
changing meaning in case I'm reading it wrong. Is this correct: "Traits let us
specify that, for example, for the program to compile any type `T` must be of a
type that allows us to print its value." If so, would this work as a
defition?-->

Traits let us specify that, for example, we need any type `T` that has methods
defined on it that allow us to print a value of that type.

This is powerful because we can define our methods with generics to allow use
of many different types, but we can constrain the type at compile-time to those
that have the behavior we need.

Here's an example definition of a trait named `Printable` that has a method
named `print`:

Filename: src/lib.rs

```rust
trait Printable {
    fn print(&self);
}
```

Listing 10-7: A `Printable` trait definition with one method, `print`

We declare a trait with the `trait` keyword, then the trait's name. In this
case, our trait will describe types that can be printed. Inside curly braces we
declare a method signature, but instead of providing an implementation, we put
a semicolon after the signature. A trait can have multiple methods in its body,
with the method signatures listed one per line and each line ending in a
semicolon. This has declared the trait `Printable`, defined as "methods that
are able to print values".

<!--Why do we do it like this, with the semicolon instead of
implementation---is that what makes it generic? (above) I'm also still quite
unclear on what a trait is/does -- I added this last line to try to wrap up
what this Printable trait is for, but I'm not confident in that, can you please
check and change? -->

Now we have the trait written we can implement it on a type. We use the `impl`
keyword, and we must also specify the trait name. Listing 10-8 shows an example
implementing the `Printable` trait from Listing 10-7 (that only has the `print`
method) for a `Temperature` enum:

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

Listing 10-8: Implementing the `Printable` trait on a `Temperature` enum

<!-- I think this code needs a little more talking through, point out what's
new and what we're doing. For eg, where does `match` come in? -->

Inside the `impl` block, we specify definitions for the trait's methods in the
context of the specific type. In the same way `impl` lets us define methods,
we've used it to define methods that pertain to our trait.

We can call methods that our trait has defined just like we can call other
methods:

<!--I'm afraid I'm still not clear on what this will actually do, the t.print
command -- just print the value of t? And where is the trait brought into
scope? I see the Temperature enum, but not the Printable trait. Perhaps a
little more explanation after this example would help make this all clearer -->

Filename: src/main.rs

```rust
fn main() {
    let t = Temperature::Celsius(37);

    t.print();
}
```

Note that in order to use a trait's methods, the trait itself must be in scope.
If the definition of `Printable` was in a module rather than within the same
program, it would need to be defined as `pub` and we would need to `use` the
trait in the scope where we wanted to call the `print` method. This is because
it's possible to have two traits that both define a method named `print`, and
is our `Temperature` enum implemented both Rust wouldn't know which `print`
method we inteded with `use`ing it first.

### Trait Bounds

This technique---defining traits with methods and implementing the trait methods
on a particular type---gives Rust more information than just defining methods
on a type directly. We are telling Rust that the type that implements the trait
can be used in places where the code specifies that it needs some type that
implements

<!--I am again finding this hard to follow, this line above is quite
circuitous! Any way to slow it down, map it out? Are we saying:

We are telling Rust that where we use the type that implements the trait in our
code, that type must have the specific functionality defined by the trait---so
if any type that doesn't have that functionality is used the program will not
compile.

? So by applying a trait bound, we are saying: " any type T must be a value
that is printable"? What values would that exclude?
 -->

a trait. To illustrate this, try out Listing 10-6, which has a `print_anything`
function definition similar to the `show_anything` function from Listing 10-3,
but this time it has a *trait bound* on the generic type `T` and uses the
`print` function from the trait. A trait bound constrains the generic type so
that is can only be a type that implements the trait specified, and not just
any type. With the trait bound, we're then allowed to use the trait method
`print` in the function body:

<!-- So when we used this before it failed because we didn't use a trait bound,
is that right? But why did that cause it to fail previously? I'm not clear on
that. What are the specific situations where trait bounds are required? -->

Filename: src/lib.rs

```rust
fn print_anything<T: Printable>(value: T) {
    println!("I have something to print for you!");
    value.print();
}
```

Listing 10-6: A `print_anything` function that uses the trait bound `Printable`
on type `T`

You specify a trait bound within angle brackets in the type name declaration.
After the name of the type that you want to apply the bound to, you add a colon
(`:`) and then give the name of the trait. This function now specifies that it
takes a `value` parameter that can be of any type, as long as that type
implements the trait `Printable`. We need to specify the `Printable` trait in
the type name declaration to be able to call the `print` method that is part of
the trait.

Now we are able to call the `print_anything` function from Listing 10-6 and,
since we implemented the trait `Printable` on `Temperature` in Listing 10-5,
pass it a `Temperature` instance as the `value` parameter:

Filename: src/main.rs

```rust
fn main() {
    let temperature = Temperature::Fahrenheit(98);
    print_anything(temperature);
}
```

If we implement the `Printable` trait on other types, we can use them with the
`print_anything` method too. However, if we try to call `print_anything` with
an `i32`, which does *not* implement the `Printable` trait, we get a
compile-time error that looks like this:

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
example, our `Printable` trait is similar tothe standard trait `Display`, which
is in fact how `println!` decides how to format things with `{}`. The `Display`
trait has a `fmt` method that determines how to format something.

<!-- Didn't we discuss the display trait in an earlier chapter, when something
wasn't compiling because we hadn't called it? I can't remember which one, but
it would be useful to cross-ref that here -->

Listing 10-7 shows our original example from Listing 10-3, this time using the
standard library's `Display` trait in the trait bound on the generic type:

Filename: src/lib.rs

```rust
use std::fmt::Display;

fn show_anything<T: Display>(value: T) {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

Listing 10-7: The `show_anything` function with trait bounds

Now that this function specifies that `T` can be any type as long as that type
implements the `Display` trait, this code will compile.

<!-- I wonder if it would simplify this section to use this show_anything
example throughout, rather than print_anything, to explain traits and trait
bounds? -->

### Multiple Trait Bounds

Each generic type can have its own trait bound. If we wanted to create a
function with two generics, 'T' and 'U', and give each generic its own trait
bound, `Display` and `Printable` respectively, the signature would look like
this:

```rust,ignore
fn some_function<T: Display, U: Printable>(value: T, other_value: U) {
```

As you might expect!

You can also specify multiple trait bounds on one type, by listing the trait
bounds in a list with a `+` between each trait. For example, here's the
signature of a function that takes a type `T` that implements both `Display`
and `Clone` ( another standard library trait we mentioned in Chapter XX):

```rust,ignore
fn some_function<T: Display + Clone>(value: T) {
```

<!-- does this mean the type must satisfy both bounds, so it's AND rather than
OR? -->

#### Organizing Multiple Trait Bounds with where

When trait bounds start getting complicated, the `where` syntax can help you
make your code a bit cleaner. The `where` syntax allows you to move the trait
bounds to after the function arguments list, so it doesn't clutter up your
function.

<!-- How does that help clean it up? Is this right, it just de-clutters? -->

And in fact, the error we got when we ran the code from Listing 10-3 referred
to it:

```bash
help: consider adding a `where T: std::fmt::Display` bound
```

<!-- So why does where create an error, if it's just for cleaning up code? -->

The definition of `show_anything` in Listing 10-X means the exact same thing as
the definition in Listing 10-7, but we've used `where` to move the trait bound
to the end of the `fn` line:

Filename: src/lib.rs

```rust
use std::fmt::Display;

fn show_anything<T>(value: T) where T: Display {
    println!("I have something to show you!");
    println!("It's: {}", value);
}
```

Listing 10-X: A tidier function with `where`

Instead of the `T: Display` trait bound going inside the angle brackets, it's
moved to the end of the function signature. This can make complex signatures
easier to read.

You can also place a `where` clause and its parts on a new line. Here's the
signature of a function that takes three generic type parameters that each have
the same two bounds:

```rust,ignore
fn some_function<T, U, V>(t: T, u: U, v: V)
    where T: Display + Clone,
          U: Printable + Debug,
          V: Clone + Printable
{
```

This makes is very clear that each generic type has two traits. Generic type
parameters and trait bounds are part of Rust's rich type system.

<!-- To wrap this up, can you summarize what trait bounds bring to Rust -- is
this a safety measure, for example, to prevent a program compiling with
incompatible data? -->

A final important kind of generic interacts with Rust's ownership and
references features, and they're called *lifetimes*.

<!--This lifetime section was a bit of a struggle -- I had to read it through
and come back to the start to wrap my head around it; I think we could rethink
the ordering of the sections within it to make it clearer. I'd like to know up
front what a lifetime does/is for and how it's made, for example, perhaps
starting with:

## Lifetime Syntax
### Lifetime Annotations in Function Signatures

Can you have a look at the structure, perhaps do a little re-ordering here?
You'll see throughout where I've mentioned that it would be useful to hear a
particular section earlier.

-->

## Abstracting Scopes with Lifetimes

Generic type parameters let us abstract over types, and traits let us abstract
over behavior. *Lifetimes* allow us to be generic over *scopes*. A lifetime is
merely the scope that an object is valid for, and ensure that we get no
dangling references in our programs.

<!-- What's the difference between scope and lifetime, then? Have we been
referring to lifetimes as scopes, before now? -->

Yes, it's a bit unusual, and will be different to tools you've used in other
programming languages. Lifetimes are, in some ways, Rust's most distinctive
feature.

Lifetimes are a big topic that can't be covered in entirety in this chapter, so
will just talk about the very basics of lifetimes here to get you familiar with
the concepts. Chapter 20 will contain more advanced information about
everything lifetimes can do.

### Lifetime Annotation Syntax

When we talked about references in Chapter 4, we left out an important detail:
every reference in Rust has a *lifetime*, which is the scope for which that
reference is valid. Most of the time lifetimes are implicit, but just like we
can choose to annotate types everywhere, we can choose to annotate lifetimes.

<!-- It would be really useful here to have the note about lifetime annotations
not changing the lifetime, and let the reader know what they actually do -->

Lifetime annotations have a slightly unusual syntax, recognized by an
apostrophe `'`:

```rust,ignore
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
```

Here `'a` is a *lifetime*: the `a` is the name, and the single apostrophe
indicates that this name is allocated to a lifetime. Unlike variables and other
Rust objects, lifetime names need to be declared *before* they're used.

<!-- Why do lifetime declarations need to come before their usage? And what is
the lifetime actually doing, here, how does it change/affect the reference
exactly? Do we need to give `a` a value for the lifetime to apply? I think we
need a little more info. -->

Here's a function signature with lifetime declarations and annotations:

```rust,ignore
fn some_function<'a>(argument: &'a i32) {
```

As you can see, lifetime annotations also go inside the generic angle brackets
after the function name.

<!-- so what is this annotation saying, exactly? That the lifetime only applies
to i32 types? Perhaps a real example the reader can run, here, would be
helpful? -->

We can even write functions that take both a lifetime declaration and a generic
type declaration:

```rust,ignore
fn some_function<'a, T>(argument: &'a T) {
```

This function takes one argument, a reference to some type, `T`, and the
reference has the lifetime `'a`. In the same way that we parameterize functions
that take generic types, we parameterize references with lifetimes.

So, that's the syntax, but *why*? What does a lifetime do, anyway?

<!-- Ah! Here we're getting to the explanation, though I do suggest we give an
explanation earlier, I found myself confused through the past two sections, it
could frustrate a reader -->

### Lifetimes Prevent Dangling References

The main aim of lifetimes is to prevent dangling references, which will cause a
program to error. Consider the program in listing 10-8, with an outer scope and
an inner scope. The outer scope declares a variable named `r` with no initial
value, and the inner scope declares a variable named `x` with the initial value
of 5. Inside the inner scope, we attempt to set the value of `r` as a reference
to `x`. Then the inner scope ends and we attempt to print out the value in `r`:

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

Listing 10-8: An attempt to use a reference whose value has gone out of scope

When you compile this code, you'll get an error:

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
referencing memory that was deallocated when `x` went out of scope. So how does
Rust determine that this code should not be allowed?

#### The Borrow Checker

The part of the compiler called the *borrow checker* compares scopes to
determine that all borrows are valid. Here's the same example from Listing 10-8
with some annotations:

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

Listing 10-9:

<!-- Just checking I'm reading this right: the inside block is the b lifetime,
correct? I want to leave a note for production, make sure we can make that
clear -->

We've annotated the lifetime of `r` with `'a` and the lifetime of `x` with
`'b`. As you can see, the inner `'b` block is much smaller than the outer `'a`
lifetime block. At compile time, Rust compares the size of the two lifetimes
and sees that `r` has a lifetime of `'a`, but that it refers to an object with
a lifetime of `'b`. The program is rejected because the lifetime `'b` is
shorter than the lifetime of `'a`-- the subject of the reference does not live
as long as the reference.

Let's look at an example does not try to make a dangling reference, and so runs
normally:

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

Listing 10-10: A working lifetime reference

Here, `x` has lifetime `'b`, which in this case is larger than `'a`. This means
that `r` can reference `x`: Rust knows that the reference in `r` will always be
valid while `x` is valid.

Note that we didn't have to annotate the code with the lifetimes; Rust figured
them out for us. This is often the case, though there are situations when Rust
can't figure out the lifetimes and annotations are required. We'll look at one
situation now.

### Lifetime Annotations in Struct Definitions

<!-- Why can't Rust figure out the lifetime itself here, what's different about
it? Can you lay that out? -->

When you have a struct with a field that holds a reference you need to annotate
the lifetimes yourself. It should look something like this:

```rust
struct Ref<'a> {
    x: &'a i32,
}
```

Again, you declare the lifetime name in the angle brackets where generic type
parameters are declared, because lifetimes are a form of generics.

In the examples in Listing 10-9 and 10-10, `'a` and `'b` were concrete
lifetimes: we knew exactly how long `r` and `x` would live. However, when we
write a function, it's unlikely that beforehand we'll know every single
argument it could be called with and how long they will be valid for. We have
be explicit about what we expect the lifetime of the argument to be (we'll
learn about how to know what you expect the lifetime to be in a bit).

<!--where in the code do we explain what we expect the lifetime to be, can you
point that out? How does annotating it change how the struct works/is used?-->

This is similar to writing a function that has an argument of a generic type:
we don't know all the types the arguments might be when the function gets
called. Lifetimes are the same idea, but being generic over the scope of a
reference, rather than a type.

### Lifetime Annotations in Function Signatures

When annotating lifetimes in functions, the annotations go on the function
signature, and not in any of the code in the function body. This is because
Rust is able analyze the code within the function without any help, but when a
function has references to or from code outside that function, the lifetimes of
the arguments or return values will potentially be different each time the
function is called. This would be incredibly costly and often impossible for
Rust to figure out. In this case, we need to annotate the lifetimes ourselves.

<!-- What is it that Rust needs to know, below? -->

Lifetime annotation parameters tell Rust what it needs to know to be able to
analyze a function without knowing about all possible calling code. Lifetime
parameters specify generic lifetimes that will apply to any specific lifetimes
the function gets called with.

<!--Ah! This is very useful to know, I had an aha moment here, can we explain
this much earlier on? (below) So if a lifetime parameter can accept a reference
with any lifetime, how does giving the lifetime parameter help Rust analyze
it?-->

Lifetime annotations do not change how long any of the references involved
live. In the same way that functions can accept any type when the signature
specifies a generic type parameter, functions can accept references with any
lifetime when the signature specifies a generic lifetime parameter.

To understand lifetime annotations in context, let's write a function that will
return the longest of two string slices. We want to be able to call this
function by passing it two string slices, and we want to get back a string
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

Listing 10-9: A `main` function to find the longest string

Note that we want the function to take string slices (which are, as you
remember, references) because we don't want the `longest` function to take
ownership of its arguments, and we want the function to be able to accept
slices of a `String` (like `a`) is as well as string literals (`b`).

<!-- why is `a` a slice and `b` a literal? You mean "a" from the string "abcd"? -->

Refer back to the "String Slices as Arguments" section of Chapter 4 for more
discussion about why these are the arguments we want.

If we try an implementation of the `longest` function without annotating the
lifetime, it won't compile:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Instead we get the following error that specifies lifetimes as a cause:

```text
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
```

The help text is telling us that the return type needs a generic lifetime
parameter on it because Rust can't tell if the reference being returned refers
to `x` or `y`. Actually, we don't know either, since in the `if` block in the
body of this function returns a reference to `x` and the `else` block returns a
reference to `y`!

To specify the lifetime parameters in this case we need to have the same
lifetime for all of the input parameters and the return type:

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
in Listing 10-9.

The function signature now says that for some lifetime `'a` it will get two
arguments, both of which are string slices that live at least as long as the
lifetime `'a`. The function will return a string slice that also will last at
least as long as the lifetime `'a`. This is the contract we are telling Rust we
want it to enforce.

By specifying the lifetime parameters in this function signature, we are not
changing the lifetimes of any values passed in or returned, but we are saying
that any values that do not adhere to this contract should be rejected by the
borrow checker. This function does not know (or need to know) exactly how long
`x` and `y` will live, but only needs to knows that there is some scope that
can be substituted for `'a` that will satisfy this signature.

<!-- Would this restrict the function? These seem like strict parameters -->

### HEADING

<!-- I think we could use a heading here, seems like we wrapped up that example
(and gave quite a lot of info) -->

The exact way to specify lifetime parameters depends on what your function is
doing. For example, if we wanted our function to always returned the first
argument, rather than the longest string slice, we wouldn't need to specify a
lifetime on `y`. This code compiles:

Filename: src/main.rs

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

We specify a lifetime for `a` and but for `y`. The lifetime parameter for the
return type needs to match the lifetime parameter of one of the arguments. If
the reference returned does *not* refer to one of the arguments, the only other
possibility is that it refers to a value created within this function, which
would be a dangling reference since the value will go out of scope at the end
of the function. Consider this attempted implementation of `longest`:

Filename: src/main.rs

```rust,ignore
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Even though we've specified a lifetime for the return type, this function fails
to compile, with the following error message:

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

Now we know that every reference has a lifetime, and we need to provide the
lifetimes for functions that use references as arguments or return values.
However, in Chapter 4 we had a function in the "String Slices" section, shown
again in Listing 10-X, that compiled without lifetime annotations:

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

Listing 10-X

The reason for this is historical: in early versions of pre-1.0 Rust, this
would not have compiled. Every reference needed an explicit lifetime. At that
time, the function signature would have been written like this:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

After writing a lot of Rust code, we found that some patterns developed when it
came to lifetimes. The Rust team

<!-- by pattern, do you mean you found that rust tended to calculate lifetimes without needing to be told? I'm not clear what patterns we mean. -->

noticed that the vast majority of code followed the pattern, and forcing
developers to use explicit lifetime syntax on every reference didn't lead to a
great developer experience.

The Rust team introduced *lifetime elision rules* to Rust's analysis of
references so that lifetime annotations weren't required as often. The elisions
rules don't provide full inference: Rust doesn't try to guess what you meant in
places where there could be ambiguity. The rules are a basic set of particular
cases, and if your code fits one of those cases, you don't need to write the
lifetimes explicitly. There are three main rules that these cases must comply
with.

Lifetimes on function arguments are called *input lifetimes*, and lifetimes on
return values are called *output lifetimes*. One rule relates to how Rust
infers input lifetimes.

A case doesn't require explicit annotations if:

<!-- does the case have to comply with all 3 rules, or just 1? -->

1. Each argument that is a reference gets its own lifetime parameter. In other
   words, a function with one argument gets one lifetime parameter: `fn
   foo<'a>(x: &'a i32)`, a function with two arguments gets two separate
   lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, and so on.

And two rules relate to output lifetimes:

2. If there is exactly one input lifetime parameter, that lifetime is assigned
   to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. If there are multiple input lifetime parameters, but one of them is `&self`
   or `&mut self`, then the lifetime of `self` is assigned to all
   output lifetime parameters. This makes writing methods much nicer.

If none of these three rules apply to the functions, then you must explicitly
annotate input and output lifetimes. Because these rules do apply in the
`first_word` function, we didn't have to specify any lifetimes.

<!-- which rules apply, all three? Does that need a little explanation? -->

These rules will cover the vast majority of cases, allowing you to write a lot
of code without needing to specify explicit lifetimes.

<!-- I think it would be useful to mention ealier on that most cases don't
require lifetime annotations, perhaps in the opening syntax section -->

### Lifetime Annotations in Method Definitions

<!-- Is this different to the reference lifetime annotations, or just a
finalized explanation? -->

Now that we've gone over the lifetime elision rules, defining methods on
structs that hold references will make more sense. The lifetime name needs to
be declared after the `impl` keyword and then used after the struct's name,
since the lifetime is part of the struct's type. The lifetimes can be elided in
any methods where the output type's lifetime is the same as that of the
struct's, because of the third elision rule. In Listing 10-X we have a struct
called `App` that holds a reference to another struct, `Config`, defined
elsewhere.

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

The `append_to_name` method does not need lifetime annotations even though the
method has a reference as an argument and is returning a reference; the
lifetime of the return value will be the lifetime of `self`.

### The Static Lifetime

There is *one* final special lifetime we need to discuss: `'static`. The
`'static` lifetime is the entire duration of the program. All string literals
have the `'static` lifetime:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the binary of your program and
the binary of your program is always available. Therefore, the lifetime of all
string literals is `'static`.

<!-- How would you add a static lifetime (below)? -->

You may see suggestions to use the `'static` lifetime in error message help
text, but before adding it, think about whether the reference you have is one
that actually lives the entire lifetime of your program or not (or even if you
want it to live that long, if it could). Most of the time, the problem in the
code is an attempt to create a dangling reference or a mismatch of the
available lifetimes, and the solution is fixing those problems, not specifying
the `'static` lifetime.

## Summary

We've covered the basics of Rust's system of generics. Generics are the core to
building good abstractions, and can be used in a number of ways. There's more
to learn about them, particularly lifetimes, and we'll cover that in later
chapters as we go through. Let's move on to I/O functionality.
