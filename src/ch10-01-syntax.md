## Generic Data Types

We can use generics to create definitions for items like function signatures or
structs, which we can then use with many different concrete data types. Let’s
first look at how to define functions, structs, enums, and methods using
generics. Then we’ll discuss how generics affect code performance.

### In Function Definitions

When defining a function that uses generics, we place the generics in the
signature of the function where we would usually specify the data types of the
parameters and return value. Doing so makes our code more flexible and provides
more functionality to callers of our function while preventing code duplication.

Continuing with our `largest` function, Listing 10-4 shows two functions that
both find the largest value in a slice.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<span class="caption">Listing 10-4: Two functions that differ only in their
names and the types in their signatures</span>

The `largest_i32` function is the one we extracted in Listing 10-3 that finds
the largest `i32` in a slice. The `largest_char` function finds the largest
`char` in a slice. The function bodies have the same code, so let’s eliminate
the duplication by introducing a generic type parameter in a single function.

To parameterize the types in the new function we’ll define, we need to name the
type parameter, just as we do for the value parameters to a function. You can
use any identifier as a type parameter name. But we’ll use `T` because, by
convention, parameter names in Rust are short, often just a letter, and Rust’s
type-naming convention is CamelCase. Short for “type,” `T` is the default
choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the
parameter name in the signature so the compiler knows what that name means.
Similarly, when we use a type parameter name in a function signature, we have
to declare the type parameter name before we use it. To define the generic
`largest` function, place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list, like this:

```rust,ignore
fn largest<T>(list: &[T]) -> T {
```

We read this definition as: the function `largest` is generic over some type
`T`. This function has one parameter named `list`, which is a slice of values
of type `T`. The `largest` function will return a value of the
same type `T`.

Listing 10-5 shows the combined `largest` function definition using the generic
data type in its signature. The listing also shows how we can call the function
with either a slice of `i32` values or `char` values. Note that this code won’t
compile yet, but we’ll fix it later in this chapter.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<span class="caption">Listing 10-5: A definition of the `largest` function that
uses generic type parameters but doesn’t compile yet</span>

If we compile this code right now, we’ll get this error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

The note mentions `std::cmp::PartialOrd`, which is a *trait*. We’ll talk about
traits in the next section. For now, this error states that the body of
`largest` won’t work for all possible types that `T` could be. Because we want
to compare values of type `T` in the body, we can only use types whose values
can be ordered. To enable comparisons, the standard library has the
`std::cmp::PartialOrd` trait that you can implement on types (see Appendix C
for more on this trait). You’ll learn how to specify that a generic type has a
particular trait in the [“Traits as Parameters”][traits-as-parameters]<!--
ignore --> section, but let’s first explore other ways of using generic type
parameters.

### In Struct Definitions

We can also define structs to use a generic type parameter in one or more
fields using the `<>` syntax. Listing 10-6 shows how to define a `Point<T>`
struct to hold `x` and `y` coordinate values of any type.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<span class="caption">Listing 10-6: A `Point<T>` struct that holds `x` and `y`
values of type `T`</span>

The syntax for using generics in struct definitions is similar to that used in
function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then we can use the generic
type in the struct definition where we would otherwise specify concrete data
types.

Note that because we’ve used only one generic type to define `Point<T>`, this
definition says that the `Point<T>` struct is generic over some type `T`, and
the fields `x` and `y` are *both* that same type, whatever that type may be. If
we create an instance of a `Point<T>` that has values of different types, as in
Listing 10-7, our code won’t compile.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<span class="caption">Listing 10-7: The fields `x` and `y` must be the same
type because both have the same generic data type `T`.</span>

In this example, when we assign the integer value 5 to `x`, we let the
compiler know that the generic type `T` will be an integer for this instance of
`Point<T>`. Then when we specify 4.0 for `y`, which we’ve defined to have the
same type as `x`, we’ll get a type mismatch error like this:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters. For example, in
Listing 10-8, we can change the definition of `Point` to be generic over types
`T` and `U` where `x` is of type `T` and `y` is of type `U`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<span class="caption">Listing 10-8: A `Point<T, U>` generic over two types so
that `x` and `y` can be values of different types</span>

Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. When you need lots of generic types in your code, it
could indicate that your code needs restructuring into smaller pieces.

### In Enum Definitions

As we did with structs, we can define enums to hold generic data types in their
variants. Let’s take another look at the `Option<T>` enum that the standard
library provides, which we used in Chapter 6:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This definition should now make more sense to you. As you can see, `Option<T>`
is an enum that is generic over type `T` and has two variants: `Some`, which
holds one value of type `T`, and a `None` variant that doesn’t hold any value.
By using the `Option<T>` enum, we can express the abstract concept of having an
optional value, and because `Option<T>` is generic, we can use this abstraction
no matter what the type of the optional value is.

Enums can use multiple generic types as well. The definition of the `Result`
enum that we used in Chapter 9 is one example:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, `T` and `E`, and has two variants:
`Ok`, which holds a value of type `T`, and `Err`, which holds a value of type
`E`. This definition makes it convenient to use the `Result` enum anywhere we
have an operation that might succeed (return a value of some type `T`) or fail
(return an error of some type `E`). In fact, this is what we used to open a
file in Listing 9-3, where `T` was filled in with the type `std::fs::File` when
the file was opened successfully and `E` was filled in with the type
`std::io::Error` when there were problems opening the file.

When you recognize situations in your code with multiple struct or enum
definitions that differ only in the types of the values they hold, you can
avoid duplication by using generic types instead.

### In Method Definitions

We can implement methods on structs and enums (as we did in Chapter 5) and use
generic types in their definitions, too. Listing 10-9 shows the `Point<T>`
struct we defined in Listing 10-6 with a method named `x` implemented on it.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<span class="caption">Listing 10-9: Implementing a method named `x` on the
`Point<T>` struct that will return a reference to the `x` field of type
`T`</span>

Here, we’ve defined a method named `x` on `Point<T>` that returns a reference
to the data in the field `x`.

Note that we have to declare `T` just after `impl` so we can use it to specify
that we’re implementing methods on the type `Point<T>`.  By declaring `T` as a
generic type after `impl`, Rust can identify that the type in the angle
brackets in `Point` is a generic type rather than a concrete type.

We could, for example, implement methods only on `Point<f32>` instances rather
than on `Point<T>` instances with any generic type. In Listing 10-10 we use the
concrete type `f32`, meaning we don’t declare any types after `impl`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<span class="caption">Listing 10-10: An `impl` block that only applies to a
struct with a particular concrete type for the generic type parameter `T`</span>

This code means the type `Point<f32>` will have a method named
`distance_from_origin` and other instances of `Point<T>` where `T` is not of
type `f32` will not have this method defined. The method measures how far our
point is from the point at coordinates (0.0, 0.0) and uses mathematical
operations that are available only for floating point types.

Generic type parameters in a struct definition aren’t always the same as those
you use in that struct’s method signatures. For example, Listing 10-11 defines
the method `mixup` on the `Point<T, U>` struct from Listing 10-8. The method
takes another `Point` as a parameter, which might have different types from the
`self` `Point` we’re calling `mixup` on. The method creates a new `Point`
instance with the `x` value from the `self` `Point` (of type `T`) and the `y`
value from the passed-in `Point` (of type `W`).

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<span class="caption">Listing 10-11: A method that uses different generic types
from its struct’s definition</span>

In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct
that has a string slice for `x` (with value `"Hello"`) and a `char` for `y`
(with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`,
which will have an `i32` for `x`, because `x` came from `p1`. The `p3` variable
will have a `char` for `y`, because `y` came from `p2`. The `println!` macro
call will print `p3.x = 5, p3.y = c`.

The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with `impl` and some are declared with the method
definition. Here, the generic parameters `T` and `U` are declared after `impl`,
because they go with the struct definition. The generic parameters `V` and `W`
are declared after `fn mixup`, because they’re only relevant to the method.

### Performance of Code Using Generics

You might be wondering whether there is a runtime cost when you’re using
generic type parameters. The good news is that Rust implements generics in such
a way that your code doesn’t run any slower using generic types than it would
with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using
generics at compile time. *Monomorphization* is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled.

In this process, the compiler does the opposite of the steps we used to create
the generic function in Listing 10-5: the compiler looks at all the places
where generic code is called and generates code for the concrete types the
generic code is called with.

Let’s look at how this works with an example that uses the standard library’s
`Option<T>` enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization. During that
process, the compiler reads the values that have been used in `Option<T>`
instances and identifies two kinds of `Option<T>`: one is `i32` and the other
is `f64`. As such, it expands the generic definition of `Option<T>` into
`Option_i32` and `Option_f64`, thereby replacing the generic definition with
the specific ones.

The monomorphized version of the code looks like the following. The generic
`Option<T>` is replaced with the specific definitions created by the compiler:

<span class="filename">Filename: src/main.rs</span>

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

Because Rust compiles generic code into code that specifies the type in each
instance, we pay no runtime cost for using generics. When the code runs, it
performs just as it would if we had duplicated each definition by hand. The
process of monomorphization makes Rust’s generics extremely efficient at
runtime.

[traits-as-parameters]: ch10-02-traits.html#traits-as-parameters
