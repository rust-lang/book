## Advanced Functions and Closures

This section explores some advanced features related to functions and closures,
including function pointers and returning closures.

### Function Pointers

We’ve talked about how to pass closures to functions; you can also pass regular
functions to functions! This technique is useful when you want to pass a
function you’ve already defined rather than defining a new closure. Doing this
with function pointers will allow you to use functions as arguments to other
functions. Functions coerce to the type `fn` (with a lowercase f), not to be
confused with the `Fn` closure trait. The `fn` type is called a *function
pointer*. The syntax for specifying that a parameter is a function pointer is
similar to that of closures, as shown in Listing 19-27.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-27/src/main.rs}}
```

<span class="caption">Listing 19-27: Using the `fn` type to accept a function
pointer as an argument</span>

This code prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.

Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.

Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either
functions or closures.

An example of where you would want to only accept `fn` and not closures is when
interfacing with external code that doesn’t have closures: C functions can
accept functions as arguments, but C doesn’t have closures.

As an example of where you could use either a closure defined inline or a named
function, let’s look at a use of `map`. To use the `map` function to turn a
vector of numbers into a vector of strings, we could use a closure, like this:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-15-map-closure/src/main.rs:here}}
```

Or we could name a function as the argument to `map` instead of the closure,
like this:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-16-map-function/src/main.rs:here}}
```

Note that we must use the fully qualified syntax that we talked about earlier
in the [“Advanced Traits”][advanced-traits]<!-- ignore --> section because
there are multiple functions available named `to_string`. Here, we’re using the
`to_string` function defined in the `ToString` trait, which the standard
library has implemented for any type that implements `Display`.

Recall from the [“Enum values”][enum-values]<!-- ignore --> section of Chapter
6 that the name of each enum variant that we define also becomes an initializer
function. We can use these initializer functions as function pointers that
implement the closure traits, which means we can specify the initializer
functions as arguments for methods that take closures, like so:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-17-map-initializer/src/main.rs:here}}
```

Here we create `Status::Value` instances using each `u32` value in the range
that `map` is called on by using the initializer function of `Status::Value`.
Some people prefer this style, and some people prefer to use closures. They
compile to the same code, so use whichever style is clearer to you.

### Returning Closures

Closures are represented by traits, which means you can’t return closures
directly. In most cases where you might want to return a trait, you can instead
use the concrete type that implements the trait as the return value of the
function. But you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function
pointer `fn` as a return type, for example.

The following code tries to return a closure directly, but it won’t compile:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-18-returns-closure/src/lib.rs}}
```

The compiler error is as follows:

```console
{{#include ../listings/ch19-advanced-features/no-listing-18-returns-closure/output.txt}}
```

The error references the `Sized` trait again! Rust doesn’t know how much space
it will need to store the closure. We saw a solution to this problem earlier.
We can use a trait object:

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-19-returns-closure-trait-object/src/lib.rs}}
```

This code will compile just fine. For more about trait objects, refer to the
section [“Using Trait Objects That Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> in Chapter 17.

Next, let’s look at macros!

[advanced-traits]:
ch19-03-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
