## Generic Data Types

Using generics where we usually place types, like in function signatures or
structs, lets us create definitions that we can use for many different concrete
data types. Let’s take a look at how to define functions, structs, enums, and
methods using generics, and at the end of this section we’ll discuss the
performance of code using generics.

### Using Generic Data Types in Function Definitions

We can define functions that use generics in the signature of the function
where the data types of the parameters and return value go. In this way, the
code we write can be more flexible and provide more functionality to callers of
our function, while not introducing code duplication.

Continuing with our `largest` function, Listing 10-4 shows two functions
providing the same functionality to find the largest value in a slice. The
first function is the one we extracted in Listing 10-3 that finds the largest
`i32` in a slice. The second function finds the largest `char` in a slice:

<span class="filename">Filename: src/main.rs</span>

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
#    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
#    assert_eq!(result, 'y');
}
```

<span class="caption">Listing 10-4: Two functions that differ only in their
names and the types in their signatures</span>

Here, the functions `largest_i32` and `largest_char` have the exact same body,
so it would be nice if we could turn these two functions into one and get rid
of the duplication. Luckily, we can do that by introducing a generic type
parameter!

To parameterize the types in the signature of the one function we’re going to
define, we need to create a name for the type parameter, just like how we give
names for the value parameters to a function. We’re going to choose the name
`T`. Any identifier can be used as a type parameter name, but we’re choosing
`T` because Rust’s type naming convention is CamelCase. Generic type parameter
names also tend to be short by convention, often just one letter. Short for
“type”, `T` is the default choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the
parameter in the signature so that the compiler knows what that name in the
body means. Similarly, when we use a type parameter name in a function
signature, we have to declare the type parameter name before we use it. Type
name declarations go in angle brackets between the name of the function and the
parameter list.

The function signature of the generic `largest` function we’re going to define
will look like this:

```rust,ignore
fn largest<T>(list: &[T]) -> T {
```

We would read this as: the function `largest` is generic over some type `T`. It
has one parameter named `list`, and the type of `list` is a slice of values of
type `T`. The `largest` function will return a value of the same type `T`.

Listing 10-5 shows the unified `largest` function definition using the generic
data type in its signature, and shows how we’ll be able to call `largest` with
either a slice of `i32` values or `char` values. Note that this code won’t
compile yet!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<span class="caption">Listing 10-5: A definition of the `largest` function that
uses generic type parameters but doesn’t compile yet</span>

If we try to compile this code right now, we’ll get this error:

```text
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

The note mentions `std::cmp::PartialOrd`, which is a *trait*. We’re going to
talk about traits in the next section, but briefly, what this error is saying
is that the body of `largest` won’t work for all possible types that `T` could
be; since we want to compare values of type `T` in the body, we can only use
types that know how to be ordered. The standard library has defined the trait
`std::cmp::PartialOrd` that types can implement to enable comparisons. We’ll
come back to traits and how to specify that a generic type has a particular
trait in the next section, but let’s set this example aside for a moment and
explore other places we can use generic type parameters first.

<!-- Liz: this is the reason we had the topics in the order we did in the first
draft of this chapter; it's hard to do anything interesting with generic types
in functions unless you also know about traits and trait bounds. I think this
ordering could work out okay, though, and keep a stronger thread with the
`longest` function going through the whole chapter, but we do pause with a
not-yet-compiling example here, which I know isn't ideal either. Let us know
what you think. /Carol -->

### Using Generic Data Types in Struct Definitions

We can define structs to use a generic type parameter in one or more of the
struct’s fields with the `<>` syntax too. Listing 10-6 shows the definition and
use of a `Point` struct that can hold `x` and `y` coordinate values of any type:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 10-6: A `Point` struct that holds `x` and `y`
values of type `T`</span>

The syntax is similar to using generics in function definitions. First, we have
to declare the name of the type parameter within angle brackets just after the
name of the struct. Then we can use the generic type in the struct definition
where we would specify concrete data types.

Note that because we’ve only used one generic type in the definition of
`Point`, what we’re saying is that the `Point` struct is generic over some type
`T`, and the fields `x` and `y` are *both* that same type, whatever it ends up
being. If we try to create an instance of a `Point` that has values of
different types, as in Listing 10-7, our code won’t compile:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

<span class="caption">Listing 10-7: The fields `x` and `y` must be the same
type because both have the same generic data type `T`</span>

If we try to compile this, we’ll get the following error:

```text
error[E0308]: mismatched types
 -->
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integral variable, found
  floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
```

When we assigned the integer value 5 to `x`, the compiler then knows for this
instance of `Point` that the generic type `T` will be an integer. Then when we
specified 4.0 for `y`, which is defined to have the same type as `x`, we get a
type mismatch error.

If we wanted to define a `Point` struct where `x` and `y` could have different
types but still have those types be generic, we can use multiple generic type
parameters. In listing 10-8, we’ve changed the definition of `Point` to be
generic over types `T` and `U`. The field `x` is of type `T`, and the field `y`
is of type `U`:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

<span class="caption">Listing 10-8: A `Point` generic over two types so that
`x` and `y` may be values of different types</span>

Now all of these instances of `Point` are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few gets
hard to read and understand. If you get to a point of needing lots of generic
types, it’s probably a sign that your code could use some restructuring to be
separated into smaller pieces.

### Using Generic Data Types in Enum Definitions

Similarly to structs, enums can be defined to hold generic data types in their
variants. We used the `Option<T>` enum provided by the standard library in
Chapter 6, and now its definition should make more sense. Let’s take another
look:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

In other words, `Option<T>` is an enum generic in type `T`. It has two
variants: `Some`, which holds one value of type `T`, and a `None` variant that
doesn’t hold any value. The standard library only has to have this one
definition to support the creation of values of this enum that have any
concrete type. The idea of “an optional value” is a more abstract concept than
one specific type, and Rust lets us express this abstract concept without lots
of duplication.

Enums can use multiple generic types as well. The definition of the `Result`
enum that we used in Chapter 9 is one example:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, `T` and `E`. `Result` has two
variants: `Ok`, which holds a value of type `T`, and `Err`, which holds a value
of type `E`. This definition makes it convenient to use the `Result` enum
anywhere we have an operation that might succeed (and return a value of some
type `T`) or fail (and return an error of some type `E`). Recall Listing 9-2
when we opened a file: in that case, `T` was filled in with the type
`std::fs::File` when the file was opened successfully and `E` was filled in
with the type `std::io::Error` when there were problems opening the file.

When you recognize situations in your code with multiple struct or enum
definitions that differ only in the types of the values they hold, you can
remove the duplication by using the same process we used with the function
definitions to introduce generic types instead.

### Using Generic Data Types in Method Definitions

Like we did in Chapter 5, we can implement methods on structs and enums that
have generic types in their definitions. Listing 10-9 shows the `Point<T>`
struct we defined in Listing 10-6. We’ve then defined a method named `x` on
`Point<T>` that returns a reference to the data in the field `x`:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

<span class="caption">Listing 10-9: Implementing a method named `x` on the
`Point<T>` struct that will return a reference to the `x` field, which is of
type `T`.</span>

Note that we have to declare `T` just after `impl` in order to use `T` in the
type `Point<T>`. Declaring `T` as a generic type after the `impl` is how Rust
knows the type in the angle brackets in `Point` is a generic type rather than a
concrete type. For example, we could choose to implement methods on
`Point<f32>` instances rather than `Point` instances with any generic type.
Listing 10-10 shows that we don’t declare anything after the `impl` in this
case, since we’re using a concrete type, `f32`:

```rust
# struct Point<T> {
#     x: T,
#     y: T,
# }
#
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

<span class="caption">Listing 10-10: Building an `impl` block which only
applies to a struct with a specific type is used for the generic type parameter
`T`</span>

This code means the type `Point<f32>` will have a method named
`distance_from_origin`, and other instances of `Point<T>` where `T` is not of
type `f32` will not have this method defined. This method measures how far our
point is from the point of coordinates (0.0, 0.0) and uses mathematical
operations which are only available for floating-point types.

Generic type parameters in a struct definition aren’t always the same generic
type parameters you want to use in that struct’s method signatures. Listing
10-11 defines a method `mixup` on the `Point<T, U>` struct from Listing 10-8.
The method takes another `Point` as a parameter, which might have different
types than the `self` `Point` that we’re calling `mixup` on. The method creates
a new `Point` instance that has the `x` value from the `self` `Point` (which is
of type `T`) and the `y` value from the passed-in `Point` (which is of type
`W`):

<span class="filename">Filename: src/main.rs</span>

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

<span class="caption">Listing 10-11: Methods that use different generic types
than their struct’s definition</span>

In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). `p2` is a `Point` that has a string
slice for `x` (with value `"Hello"`) and a `char` for `y` (with value `c`).
Calling `mixup` on `p1` with the argument `p2` gives us `p3`, which will have
an `i32` for `x`, since `x` came from `p1`. `p3` will have a `char` for `y`,
since `y` came from `p2`. The `println!` will print `p3.x = 5, p3.y = c`.

Note that the generic parameters `T` and `U` are declared after `impl`, since
they go with the struct definition. The generic parameters `V` and `W` are
declared after `fn mixup`, since they are only relevant to the method.

### Performance of Code Using Generics

You may have been reading this section and wondering if there’s a run-time cost
to using generic type parameters. Good news: the way that Rust has implemented
generics means that your code will not run any slower than if you had specified
concrete types instead of generic type parameters!

Rust accomplishes this by performing *monomorphization* of code using generics
at compile time. Monomorphization is the process of turning generic code into
specific code with the concrete types that are actually used filled in.

What the compiler does is the opposite of the steps that we performed to create
the generic function in Listing 10-5. The compiler looks at all the places that
generic code is called and generates code for the concrete types that the
generic code is called with.

Let’s work through an example that uses the standard library’s `Option` enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it will perform monomorphization. The compiler
will read the values that have been passed to `Option` and see that we have two
kinds of `Option<T>`: one is `i32`, and one is `f64`. As such, it will expand
the generic definition of `Option<T>` into `Option_i32` and `Option_f64`,
thereby replacing the generic definition with the specific ones.

The monomorphized version of our code that the compiler generates looks like
this, with the uses of the generic `Option` replaced with the specific
definitions created by the compiler:

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

We can write the non-duplicated code using generics, and Rust will compile that
into code that specifies the type in each instance. That means we pay no
runtime cost for using generics; when the code runs, it performs just like it
would if we had duplicated each particular definition by hand. The process of
monomorphization is what makes Rust’s generics extremely efficient at runtime.
