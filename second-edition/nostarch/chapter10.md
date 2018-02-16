
[TOC]

# Generic Types, Traits, and Lifetimes

Every programming language has tools to deal effectively with duplication of
concepts; in Rust, one of those tools is *generics*. Generics are abstract
stand-ins for concrete types or other properties. We can express properties of
generics, such as their behavior or how they relate to other generics, without
needing to know when we’re writing and compiling the code what will actually be
in their place.

In the same way that a function takes parameters whose value we don’t know in
order to write code once that will be run on multiple concrete values, we can
write functions that take parameters of some generic type instead of a concrete
type like `i32` or `String`. We’ve already used generics in Chapter 6 with
`Option<T>`, Chapter 8 with `Vec<T>` and `HashMap<K, V>`, and Chapter 9 with
`Result<T, E>`. In this chapter, we’ll explore how to define our own types,
functions, and methods with generics!

First, we’re going to review the mechanics of extracting a function that
reduces code duplication. Then we’ll use the same mechanics to make a generic
function out of two functions that only differ in the types of their
parameters. We’ll go over using generic types in struct and enum definitions
too.

After that, we’ll discuss *traits*, which are a way to define behavior in a
generic way. Traits can be combined with generic types in order to constrain a
generic type to those types that have a particular behavior, rather than any
type at all.

Finally, we’ll discuss *lifetimes*, which are a kind of generic that let us
give the compiler information about how references are related to each other.
Lifetimes are the feature in Rust that allow us to borrow values in many
situations and still have the compiler check that references will be valid.

## Removing Duplication by Extracting a Function

Before getting into generics syntax, let’s first review a technique for dealing
with duplication that doesn’t use generic types: extracting a function. Once
that’s fresh in our minds, we’ll use the same mechanics with generics to
extract a generic function! In the same way that you recognize duplicated code
to extract into a function, you’ll start to recognize duplicated code that can
use generics.

Consider a small program that finds the largest number in a list, shown in
Listing 10-1:

Filename: src/main.rs

```
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

Listing 10-1: Code to find the largest number in a list of numbers

This code takes a list of integers, stored here in the variable `numbers`. It
puts the first item in the list in a variable named `largest`. Then it iterates
through all the numbers in the list, and if the current value is greater than
the number stored in `largest`, it replaces the value in `largest`. If the
current value is smaller than the largest value seen so far, `largest` is not
changed. When all the items in the list have been considered, `largest` will
hold the largest value, which in this case is 100.

If we needed to find the largest number in two different lists of numbers, we
could duplicate the code in Listing 10-1 and have the same logic exist in two
places in the program, as in Listing 10-2:

Filename: src/main.rs

```
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
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
<!-- Yes, our audience will be familiar with this term. /Carol -->

To eliminate this duplication, we can create an abstraction, which in this case
will be in the form of a function that operates on any list of integers given
to the function in a parameter. This will increase the clarity of our code and
let us communicate and reason about the concept of finding the largest number
in a list independently of the specific places this concept is used.

In the program in Listing 10-3, we’ve extracted the code that finds the largest
number into a function named `largest`. This program can find the largest
number in two different lists of numbers, but the code from Listing 10-1 only
exists in one spot:

Filename: src/main.rs

```
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
}
```

Listing 10-3: Abstracted code to find the largest number in two lists

The function has a parameter, `list`, which represents any concrete slice of
`i32` values that we might pass into the function. The code in the function
definition operates on the `list` representation of any `&[i32]`. When we call
the `largest` function, the code actually runs on the specific values that we
pass in.

The mechanics we went through to get from Listing 10-2 to Listing 10-3 were
these steps:

1. We noticed there was duplicate code.
2. We extracted the duplicate code into the body of the function, and specified
   the inputs and return values of that code in the function signature.
3. We replaced the two concrete places that had the duplicated code to call the
   function instead.

We can use these same steps with generics to reduce code duplication in
different ways in different scenarios. In the same way that the function body
is now operating on an abstract `list` instead of concrete values, code using
generics will operate on abstract types. The concepts powering generics are the
same concepts you already know that power functions, just applied in different
ways.

What if we had two functions, one that found the largest item in a slice of
`i32` values and one that found the largest item in a slice of `char` values?
How would we get rid of that duplication? Let’s find out!

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

Filename: src/main.rs

```
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
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&chars);
    println!("The largest char is {}", result);
}
```

Listing 10-4: Two functions that differ only in their names and the types in
their signatures

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

```
fn largest<T>(list: &[T]) -> T {
```

We would read this as: the function `largest` is generic over some type `T`. It
has one parameter named `list`, and the type of `list` is a slice of values of
type `T`. The `largest` function will return a value of the same type `T`.

Listing 10-5 shows the unified `largest` function definition using the generic
data type in its signature, and shows how we’ll be able to call `largest` with
either a slice of `i32` values or `char` values. Note that this code won’t
compile yet!

Filename: src/main.rs

```
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
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
```

Listing 10-5: A definition of the `largest` function that uses generic type
parameters but doesn’t compile yet

If we try to compile this code right now, we’ll get this error:

```
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

Filename: src/main.rs

```
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

The syntax is similar to using generics in function definitions. First, we have
to declare the name of the type parameter within angle brackets just after the
name of the struct. Then we can use the generic type in the struct definition
where we would specify concrete data types.

Note that because we’ve only used one generic type in the definition of
`Point`, what we’re saying is that the `Point` struct is generic over some type
`T`, and the fields `x` and `y` are *both* that same type, whatever it ends up
being. If we try to create an instance of a `Point` that has values of
different types, as in Listing 10-7, our code won’t compile:

Filename: src/main.rs

```
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

Listing 10-7: The fields `x` and `y` must be the same type because both have
the same generic data type `T`

If we try to compile this, we’ll get the following error:

```
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

Filename: src/main.rs

```
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

Listing 10-8: A `Point` generic over two types so that `x` and `y` may be
values of different types

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

```
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

```
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

Filename: src/main.rs

```
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

Listing 10-9: Implementing a method named `x` on the `Point<T>` struct that
will return a reference to the `x` field, which is of type `T`.

Note that we have to declare `T` just after `impl`, so that we can use it when
we specify that we’re implementing methods on the type `Point<T>`.

Generic type parameters in a struct definition aren’t always the same generic
type parameters you want to use in that struct’s method signatures. Listing
10-10 defines a method `mixup` on the `Point<T, U>` struct from Listing 10-8.
The method takes another `Point` as a parameter, which might have different
types than the `self` `Point` that we’re calling `mixup` on. The method creates
a new `Point` instance that has the `x` value from the `self` `Point` (which is
of type `T`) and the `y` value from the passed-in `Point` (which is of type
`W`):

Filename: src/main.rs

```
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(&self, other: &Point<V, W>) -> Point<T, W> {
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

Listing 10-10: Methods that use different generic types than their struct’s
definition

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

```
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

Filename: src/main.rs

```
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

## Traits: Defining Shared Behavior

Traits allow us to use another kind of abstraction: they let us abstract over
behavior that types can have in common. A *trait* tells the Rust compiler about
functionality a particular type has and might share with other types. In
situations where we use generic type parameters, we can use *trait bounds* to
specify, at compile time, that the generic type may be any type that implements
a trait and therefore has the behavior we want to use in that situation.

> Note: *Traits* are similar to a feature often called ‘interfaces’ in other
> languages, though with some differences.

### Defining a Trait

The behavior of a type consists of the methods we can call on that type.
Different types share the same behavior if we can call the same methods on all
of those types. Trait definitions are a way to group method signatures together
in order to define a set of behaviors necessary to accomplish some purpose.

For example, say we have multiple structs that hold various kinds and amounts
of text: a `NewsArticle` struct that holds a news story filed in a particular
place in the world, and a `Tweet` that can have at most 140 characters in its
content along with metadata like whether it was a retweet or a reply to another
tweet.

We want to make a media aggregator library that can display summaries of data
that might be stored in a `NewsArticle` or `Tweet` instance. The behavior we
need each struct to have is that it’s able to be summarized, and that we can
ask for that summary by calling a `summary` method on an instance. Listing
10-11 shows the definition of a `Summarizable` trait that expresses this
concept:

Filename: lib.rs

```
pub trait Summarizable {
    fn summary(&self) -> String;
}
```

Listing 10-11: Definition of a `Summarizable` trait that consists of the
behavior provided by a `summary` method

We declare a trait with the `trait` keyword, then the trait’s name, in this
case `Summarizable`. Inside curly braces we declare the method signatures that
describe the behaviors that types that implement this trait will need to have,
in this case `fn summary(&self) -> String`. After the method signature, instead
of providing an implementation within curly braces, we put a semicolon. Each
type that implements this trait must then provide its own custom behavior for
the body of the method, but the compiler will enforce that any type that has
the `Summarizable` trait will have the method `summary` defined for it with
this signature exactly.

A trait can have multiple methods in its body, with the method signatures
listed one per line and each line ending in a semicolon.

### Implementing a Trait on a Type

Now that we’ve defined the `Summarizable` trait, we can implement it on the
types in our media aggregator that we want to have this behavior. Listing 10-12
shows an implementation of the `Summarizable` trait on the `NewsArticle` struct
that uses the headline, the author, and the location to create the return value
of `summary`. For the `Tweet` struct, we’ve chosen to define `summary` as the
username followed by the whole text of the tweet, assuming that tweet content
is already limited to 140 characters.

Filename: lib.rs

```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Listing 10-12: Implementing the `Summarizable` trait on the `NewsArticle` and
`Tweet` types

Implementing a trait on a type is similar to implementing methods that aren’t
related to a trait. The difference is after `impl`, we put the trait name that
we want to implement, then say `for` and the name of the type that we want to
implement the trait for. Within the `impl` block, we put the method signatures
that the trait definition has defined, but instead of putting a semicolon after
each signature, we put curly braces and fill in the method body with the
specific behavior that we want the methods of the trait to have for the
particular type.

Once we’ve implemented the trait, we can call the methods on instances of
`NewsArticle` and `Tweet` in the same manner that we call methods that aren’t
part of a trait:

```
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

This will print `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.

Note that because we’ve defined the `Summarizable` trait and the `NewsArticle`
and `Tweet` types all in the same `lib.rs` in Listing 10-12, they’re all in the
same scope. If this `lib.rs` is for a crate we’ve called `aggregator`, and
someone else wants to use our crate’s functionality plus implement the
`Summarizable` trait on their `WeatherForecast` struct, their code would need
to import the `Summarizable` trait into their scope first before they could
implement it, like in Listing 10-13:

Filename: lib.rs

```
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}
```

Listing 10-13: Bringing the `Summarizable` trait from our `aggregator` crate
into scope in another crate

This code also assumes `Summarizable` is a public trait, which it is because we
put the `pub` keyword before `trait` in Listing 10-11.

One restriction to note with trait implementations: we may implement a trait on
a type as long as either the trait or the type are local to our crate. In other
words, we aren’t allowed to implement external traits on external types. We
can’t implement the `Display` trait on `Vec`, for example, since both `Display`
and `Vec` are defined in the standard library. We are allowed to implement
standard library traits like `Display` on a custom type like `Tweet` as part of
our `aggregator` crate functionality. We could also implement `Summarizable` on
`Vec` in our `aggregator` crate, since we’ve defined `Summarizable` there. This
restriction is part of what’s called the *orphan rule*, which you can look up
if you’re interested in type theory. Briefly, it’s called the orphan rule
because the parent type is not present. Without this rule, two crates could
implement the same trait for the same type, and the two implementations would
conflict: Rust wouldn’t know which implementation to use. Because Rust enforces
the orphan rule, other people’s code can’t break your code and vice versa.

### Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods
in a trait, instead of making every implementation on every type define custom
behavior. When we implement the trait on a particular type, we can choose to
keep or override each method’s default behavior.

Listing 10-14 shows how we could have chosen to specify a default string for
the `summary` method of the `Summarize` trait instead of only choosing to only
define the method signature like we did in Listing 10-11:

Filename: lib.rs

```
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Listing 10-14: Definition of a `Summarizable` trait with a default
implementation of the `summary` method

If we wanted to use this default implementation to summarize instances of
`NewsArticle` instead of defining a custom implementation like we did in
Listing 10-12, we would specify an empty `impl` block:

```
impl Summarizable for NewsArticle {}
```

Even though we’re no longer choosing to define the `summary` method on
`NewsArticle` directly, since the `summary` method has a default implementation
and we specified that `NewsArticle` implements the `Summarizable` trait, we can
still call the `summary` method on an instance of `NewsArticle`:

```
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summary());
```

This code prints `New article available! (Read more...)`.

Changing the `Summarizable` trait to have a default implementation for
`summary` does not require us to change anything about the implementations of
`Summarizable` on `Tweet` in Listing 10-12 or `WeatherForecast` in Listing
10-13: the syntax for overriding a default implementation is exactly the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.

Default implementations are allowed to call the other methods in the same
trait, even if those other methods don’t have a default implementation. In this
way, a trait can provide a lot of useful functionality and only require
implementers to specify a small part of it. We could choose to have the
`Summarizable` trait also have an `author_summary` method whose implementation
is required, then a `summary` method that has a default implementation that
calls the `author_summary` method:

```
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}
```

In order to use this version of `Summarizable`, we’re only required to define
`author_summary` when we implement the trait on a type:

```
impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Once we define `author_summary`, we can call `summary` on instances of the
`Tweet` struct, and the default implementation of `summary` will call the
definition of `author_summary` that we’ve provided.

```
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

This will print `1 new tweet: (Read more from @horse_ebooks...)`.

Note that it is not possible to call the default implementation from an
overridden implementation.

### Trait Bounds

Now that we’ve defined traits and implemented those traits on types, we can use
traits with generic type parameters. We can constrain generic types so that
rather than being any type, the compiler will ensure that the type will be
limited to those types that implement a particular trait and thus have the
behavior that we need the types to have. This is called specifying *trait
bounds* on a generic type.

For example, in Listing 10-12, we implemented the `Summarizable` trait on the
types `NewsArticle` and `Tweet`. We can define a function `notify` that calls
the `summary` method on its parameter `item`, which is of the generic type `T`.
To be able to call `summary` on `item` without getting an error, we can use
trait bounds on `T` to specify that `item` must be of a type that implements
the `Summarizable` trait:

```
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
```

Trait bounds go with the declaration of the generic type parameter, after a
colon and within the angle brackets. Because of the trait bound on `T`, we can
call `notify` and pass in any instance of `NewsArticle` or `Tweet`. The
external code from Listing 10-13 that’s using our `aggregator` crate can call
our `notify` function and pass in an instance of `WeatherForecast`, since
`Summarizable` is implemented for `WeatherForecast` as well. Code that calls
`notify` with any other type, like a `String` or an `i32`, won’t compile, since
those types do not implement `Summarizable`.

We can specify multiple trait bounds on a generic type by using `+`. If we
needed to be able to use display formatting on the type `T` in a function as
well as the `summary` method, we can use the trait bounds `T: Summarizable +
Display`. This means `T` can be any type that implements both `Summarizable`
and `Display`.

For functions that have multiple generic type parameters, each generic has its
own trait bounds. Specifying lots of trait bound information in the angle
brackets between a function’s name and its parameter list can get hard to read,
so there’s an alternate syntax for specifying trait bounds that lets us move
them to a `where` clause after the function signature. So instead of:

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

We can write this instead with a `where` clause:

```
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

This is less cluttered and makes this function’s signature look more similar to
a function without lots of trait bounds, in that the function name, parameter
list, and return type are close together.

### Fixing the `largest` Function with Trait Bounds

So any time you want to use behavior defined by a trait on a generic, you need
to specify that trait in the generic type parameter’s type bounds. We can now
fix the definition of the `largest` function that uses a generic type parameter
from Listing 10-5! When we set that code aside, we were getting this error:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

In the body of `largest` we wanted to be able to compare two values of type `T`
using the greater-than operator. That operator is defined as a default method
on the standard library trait `std::cmp::PartialOrd`. So in order to be able to
use the greater-than operator, we need to specify `PartialOrd` in the trait
bounds for `T` so that the `largest` function will work on slices of any type
that can be compared. We don’t need to bring `PartialOrd` into scope because
it’s in the prelude.

```
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

If we try to compile this, we’ll get different errors:

```
error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut largest = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref largest` or `ref mut largest`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

The key to this error is `cannot move out of type [T], a non-copy array`.
With our non-generic versions of the `largest` function, we were only trying to
find the largest `i32` or `char`. As we discussed in Chapter 4, types like
`i32` and `char` that have a known size can be stored on the stack, so they
implement the `Copy` trait. When we changed the `largest` function to be
generic, it’s now possible that the `list` parameter could have types in it
that don’t implement the `Copy` trait, which means we wouldn’t be able to move
the value out of `list[0]` and into the `largest` variable.

If we only want to be able to call this code with types that are `Copy`, we can
add `Copy` to the trait bounds of `T`! Listing 10-15 shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into `largest` implement both the `PartialOrd`
and `Copy` traits, like `i32` and `char`:

Filename: src/main.rs

```
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}
```

Listing 10-15: A working definition of the `largest` function that works on any
generic type that implements the `PartialOrd` and `Copy` traits

If we don’t want to restrict our `largest` function to only types that
implement the `Copy` trait, we could specify that `T` has the trait bound
`Clone` instead of `Copy` and clone each value in the slice when we want the
`largest` function to have ownership. Using the `clone` function means we’re
potentially making more heap allocations, though, and heap allocations can be
slow if we’re working with large amounts of data. Another way we could
implement `largest` is for the function to return a reference to a `T` value in
the slice. If we change the return type to be `&T` instead of `T` and change
the body of the function to return a reference, we wouldn’t need either the
`Clone` or `Copy` trait bounds and we wouldn’t be doing any heap allocations.
Try implementing these alternate solutions on your own!

Traits and trait bounds let us write code that uses generic type parameters in
order to reduce duplication, but still specify to the compiler exactly what
behavior our code needs the generic type to have. Because we’ve given the trait
bound information to the compiler, it can check that all the concrete types
used with our code provide the right behavior. In dynamically typed languages,
if we tried to call a method on a type that the type didn’t implement, we’d get
an error at runtime. Rust moves these errors to compile time so that we’re
forced to fix the problems before our code is even able to run. Additionally,
we don’t have to write code that checks for behavior at runtime since we’ve
already checked at compile time, which improves performance compared to other
languages without having to give up the flexibility of generics.

There’s another kind of generics that we’ve been using without even realizing
it called *lifetimes*. Rather than helping us ensure that a type has the
behavior we need it to have, lifetimes help us ensure that references are valid
as long as we need them to be. Let’s learn how lifetimes do that.

## Validating References with Lifetimes

When we talked about references in Chapter 4, we left out an important detail:
every reference in Rust has a *lifetime*, which is the scope for which that
reference is valid. Most of the time lifetimes are implicit and inferred, just
like most of the time types are inferred. Similarly to when we have to annotate
types because multiple types are possible, there are cases where the lifetimes
of references could be related in a few different ways, so Rust needs us to
annotate the relationships using generic lifetime parameters so that it can
make sure the actual references used at runtime will definitely be valid.

Yes, it’s a bit unusual, and will be different to tools you’ve used in other
programming languages. Lifetimes are, in some ways, Rust’s most distinctive
feature.

Lifetimes are a big topic that can’t be covered in entirety in this chapter, so
we’ll cover common ways you might encounter lifetime syntax in this chapter to
get you familiar with the concepts. Chapter 19 will contain more advanced
information about everything lifetimes can do.

### Lifetimes Prevent Dangling References

The main aim of lifetimes is to prevent dangling references, which will cause a
program to reference data other than the data we’re intending to reference.
Consider the program in Listing 10-16, with an outer scope and an inner scope.
The outer scope declares a variable named `r` with no initial value, and the
inner scope declares a variable named `x` with the initial value of 5. Inside
the inner scope, we attempt to set the value of `r` as a reference to `x`. Then
the inner scope ends, and we attempt to print out the value in `r`:

```
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

Listing 10-16: An attempt to use a reference whose value has gone out of scope

> #### Uninitialized Variables Cannot Be Used
>
> The next few examples declare variables without giving them an initial value,
> so that the variable name exists in the outer scope. This might appear to be
> in conflict with Rust not having null. However, if we try to use a variable
> before giving it a value, we’ll get a compile-time error. Try it out!

When we compile this code, we’ll get an error:

```
error: `x` does not live long enough
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

The variable `x` doesn’t “live long enough.” Why not? Well, `x` is going to go
out of scope when we hit the closing curly brace on line 7, ending the inner
scope. But `r` is valid for the outer scope; its scope is larger and we say
that it “lives longer.” If Rust allowed this code to work, `r` would be
referencing memory that was deallocated when `x` went out of scope, and
anything we tried to do with `r` wouldn’t work correctly. So how does Rust
determine that this code should not be allowed?

#### The Borrow Checker

The part of the compiler called the *borrow checker* compares scopes to
determine that all borrows are valid. Listing 10-17 shows the same example from
Listing 10-16 with annotations showing the lifetimes of the variables:

```
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

Listing 10-17: Annotations of the lifetimes of `x` and `r`, named `'a` and `'b`
respectively

<!-- Just checking I'm reading this right: the inside block is the b lifetime,
correct? I want to leave a note for production, make sure we can make that
clear -->
<!-- Yes, the inside block for the `'b` lifetime starts with the `let x = 5;`
line and ends with the first closing curly brace on the 7th line. Do you think
the text art comments work or should we make an SVG diagram that has nicer
looking arrows and labels? /Carol -->

We’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with
`'b`. As you can see, the inner `'b` block is much smaller than the outer `'a`
lifetime block. At compile time, Rust compares the size of the two lifetimes
and sees that `r` has a lifetime of `'a`, but that it refers to an object with
a lifetime of `'b`. The program is rejected because the lifetime `'b` is
shorter than the lifetime of `'a`: the subject of the reference does not live
as long as the reference.

Let’s look at an example in Listing 10-18 that doesn’t try to make a dangling
reference and compiles without any errors:

```
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+
```

Listing 10-18: A valid reference because the data has a longer lifetime than
the reference

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This
means `r` can reference `x`: Rust knows that the reference in `r` will always
be valid while `x` is valid.

Now that we’ve shown where the lifetimes of references are in a concrete
example and discussed how Rust analyzes lifetimes to ensure references will
always be valid, let’s talk about generic lifetimes of parameters and return
values in the context of functions.

### Generic Lifetimes in Functions

Let’s write a function that will return the longest of two string slices. We
want to be able to call this function by passing it two string slices, and we
want to get back a string slice. The code in Listing 10-19 should print `The
longest string is abcd` once we’ve implemented the `longest` function:

Filename: src/main.rs

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

Listing 10-19: A `main` function that calls the `longest` function to find the
longest of two string slices

Note that we want the function to take string slices (which are references, as
we talked about in Chapter 4) since we don’t want the `longest` function to
take ownership of its arguments. We want the function to be able to accept
slices of a `String` (which is the type of the variable `string1`) as well as
string literals (which is what variable `string2` contains).

<!-- why is `a` a slice and `b` a literal? You mean "a" from the string "abcd"? -->
<!-- I've changed the variable names to remove ambiguity between the variable
name `a` and the "a" from the string "abcd". `string1` is not a slice, it's a
`String`, but we're going to pass a slice that refers to that `String` to the
`longest` function (`string1.as_str()` creates a slice that references the
`String` stored in `string1`). We chose to have `string2` be a literal since
the reader might have code with both `String`s and string literals, and the way
most readers first get into problems with lifetimes is involving string slices,
so we wanted to demonstrate the flexibility of taking string slices as
arguments but the issues you might run into because string slices are
references.
All of the `String`/string slice/string literal concepts here are covered
thoroughly in Chapter 4, which is why we put two back references here (above
and below). If these topics are confusing you in this context, I'd be
interested to know if rereading Chapter 4 clears up that confusion.
/Carol -->

Refer back to the “String Slices as Arguments” section of Chapter 4 for more
discussion about why these are the arguments we want.

If we try to implement the `longest` function as shown in Listing 10-20, it
won’t compile:

Filename: src/main.rs

```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Listing 10-20: An implementation of the `longest` function that returns the
longest of two string slices, but does not yet compile

Instead we get the following error that talks about lifetimes:

```
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`
```

The help text is telling us that the return type needs a generic lifetime
parameter on it because Rust can’t tell if the reference being returned refers
to `x` or `y`. Actually, we don’t know either, since the `if` block in the body
of this function returns a reference to `x` and the `else` block returns a
reference to `y`!

As we’re defining this function, we don’t know the concrete values that will be
passed into this function, so we don’t know whether the `if` case or the `else`
case will execute. We also don’t know the concrete lifetimes of the references
that will be passed in, so we can’t look at the scopes like we did in Listings
10-17 and 10-18 in order to determine that the reference we return will always
be valid. The borrow checker can’t determine this either, because it doesn’t
know how the lifetimes of `x` and `y` relate to the lifetime of the return
value. We’re going to add generic lifetime parameters that will define the
relationship between the references so that the borrow checker can perform its
analysis.

### Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references involved live.
In the same way that functions can accept any type when the signature specifies
a generic type parameter, functions can accept references with any lifetime
when the signature specifies a generic lifetime parameter. What lifetime
annotations do is relate the lifetimes of multiple references to each other.

Lifetime annotations have a slightly unusual syntax: the names of lifetime
parameters must start with an apostrophe `'`. The names of lifetime parameters
are usually all lowercase, and like generic types, their names are usually very
short. `'a` is the name most people use as a default. Lifetime parameter
annotations go after the `&` of a reference, and a space separates the lifetime
annotation from the reference’s type.

Here’s some examples: we’ve got a reference to an `i32` without a lifetime
parameter, a reference to an `i32` that has a lifetime parameter named `'a`,
and a mutable reference to an `i32` that also has the lifetime `'a`:

```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning: lifetime
annotations tell Rust how the generic lifetime parameters of multiple
references relate to each other. If we have a function with the parameter
`first` that is a reference to an `i32` that has the lifetime `'a`, and the
function has another parameter named `second` that is another reference to an
`i32` that also has the lifetime `'a`, these two lifetime annotations that have
the same name indicate that the references `first` and `second` must both live
as long as the same generic lifetime.

### Lifetime Annotations in Function Signatures

Let’s look at lifetime annotations in the context of the `longest` function
we’re working on. Just like generic type parameters, generic lifetime
parameters need to be declared within angle brackets between the function name
and the parameter list. The constraint we want to tell Rust about for the
references in the parameters and the return value is that they all must have
the same lifetime, which we’ll name `'a` and add to each reference as shown in
Listing 10-21:

Filename: src/main.rs

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Listing 10-21: The `longest` function definition that specifies all the
references in the signature must have the same lifetime, `'a`

This will compile and will produce the result we want when used with the `main`
function in Listing 10-19.

The function signature now says that for some lifetime `'a`, the function will
get two parameters, both of which are string slices that live at least as long
as the lifetime `'a`. The function will return a string slice that also will
last at least as long as the lifetime `'a`. This is the contract we are telling
Rust we want it to enforce.

By specifying the lifetime parameters in this function signature, we are not
changing the lifetimes of any values passed in or returned, but we are saying
that any values that do not adhere to this contract should be rejected by the
borrow checker. This function does not know (or need to know) exactly how long
`x` and `y` will live, but only needs to knows that there is some scope that
can be substituted for `'a` that will satisfy this signature.

When annotating lifetimes in functions, the annotations go on the function
signature, and not in any of the code in the function body. This is because
Rust is able analyze the code within the function without any help, but when a
function has references to or from code outside that function, the lifetimes of
the arguments or return values will potentially be different each time the
function is called. This would be incredibly costly and often impossible for
Rust to figure out. In this case, we need to annotate the lifetimes ourselves.

When concrete references are passed to `longest`, the concrete lifetime that
gets substituted for `'a` is the part of the scope of `x` that overlaps with
the scope of `y`. Since scopes always nest, another way to say this is that the
generic lifetime `'a` will get the concrete lifetime equal to the smaller of
the lifetimes of `x` and `y`. Because we’ve annotated the returned reference
with the same lifetime parameter `'a`, the returned reference will therefore be
guaranteed to be valid as long as the shorter of the lifetimes of `x` and `y`.

Let’s see how this restricts the usage of the `longest` function by passing in
references that have different concrete lifetimes. Listing 10-22 is a
straightforward example that should match your intuition from any language:
`string1` is valid until the end of the outer scope, `string2` is valid until
the end of the inner scope, and `result` references something that is valid
until the end of the outer scope. The borrow checker approves of this code; it
will compile and print `The longest string is long string is long` when run:

Filename: src/main.rs

```
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

Listing 10-22: Using the `longest` function with references to `String` values
that have different concrete lifetimes

Next, let’s try an example that will show that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We’ll move the
declaration of the `result` variable outside the inner scope, but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Next, we’ll move the `println!` that uses `result` outside of the
inner scope, after it has ended. The code in Listing 10-23 will not compile:

Filename: src/main.rs

```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

Listing 10-23: Attempting to use `result` after `string2` has gone out of scope
won’t compile

If we try to compile this, we’ll get this error:

```
error: `string2` does not live long enough
   |
6  |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
```

The error is saying that in order for `result` to be valid for the `println!`,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values with the same lifetime parameter, `'a`.

We can look at this code as humans and see that `string1` is longer, and
therefore `result` will contain a reference to `string1`. Because `string1` has
not gone out of scope yet, a reference to `string1` will still be valid for the
`println!`. However, what we’ve told Rust with the lifetime parameters is that
the lifetime of the reference returned by the `longest` function is the same as
the smaller of the lifetimes of the references passed in. Therefore, the borrow
checker disallows the code in Listing 10-23 as possibly having an invalid
reference.

Try designing some more experiments that vary the values and lifetimes of the
references passed in to the `longest` function and how the returned reference
is used. Make hypotheses about whether your experiments will pass the borrow
checker or not before you compile, then check to see if you’re right!

### Thinking in Terms of Lifetimes

The exact way to specify lifetime parameters depends on what your function is
doing. For example, if we changed the implementation of the `longest` function
to always return the first argument rather than the longest string slice, we
wouldn’t need to specify a lifetime on the `y` parameter. This code compiles:

Filename: src/main.rs

```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

In this example, we’ve specified a lifetime parameter `'a` for the parameter
`x` and the return type, but not for the parameter `y`, since the lifetime of
`y` does not have any relationship with the lifetime of `x` or the return value.

When returning a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter of one of the arguments. If
the reference returned does *not* refer to one of the arguments, the only other
possibility is that it refers to a value created within this function, which
would be a dangling reference since the value will go out of scope at the end
of the function. Consider this attempted implementation of the `longest`
function that won’t compile:

Filename: src/main.rs

```
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Even though we’ve specified a lifetime parameter `'a` for the return type, this
implementation fails to compile because the return value lifetime is not
related to the lifetime of the parameters at all. Here’s the error message we
get:

```
error: `result` does not live long enough
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the block
at 1:44...
  |
1 | fn longest<'a>(x: &str, y: &str) -> &'a str {
  |                                             ^
```

The problem is that `result` will go out of scope and get cleaned up at the end
of the `longest` function, and we’re trying to return a reference to `result`
from the function. There’s no way we can specify lifetime parameters that would
change the dangling reference, and Rust won’t let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so that the calling function is then responsible for
cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various
arguments and return values of functions. Once they’re connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.

### Lifetime Annotations in Struct Definitions

Up until now, we’ve only defined structs to hold owned types. It is possible
for structs to hold references, but we need to add a lifetime annotation on
every reference in the struct’s definition. Listing 10-24 has a struct named
`ImportantExcerpt` that holds a string slice:

Filename: src/main.rs

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

Listing 10-24: A struct that holds a reference, so its definition needs a
lifetime annotation

This struct has one field, `part`, that holds a string slice, which is a
reference. Just like with generic data types, we have to declare the name of
the generic lifetime parameter inside angle brackets after the name of the
struct so that we can use the lifetime parameter in the body of the struct
definition.

The `main` function here creates an instance of the `ImportantExcerpt` struct
that holds a reference to the first sentence of the `String` owned by the
variable `novel`.

### Lifetime Elision

In this section, we’ve learned that every reference has a lifetime, and we need
to specify lifetime parameters for functions or structs that use references.
However, in Chapter 4 we had a function in the “String Slices” section, shown
again in Listing 10-25, that compiled without lifetime annotations:

Filename: src/lib.rs

```
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

Listing 10-25: A function we defined in Chapter 4 that compiled without
lifetime annotations, even though the parameter and return type are references

The reason this function compiles without lifetime annotations is historical:
in early versions of pre-1.0 Rust, this indeed wouldn’t have compiled. Every
reference needed an explicit lifetime. At that time, the function signature
would have been written like this:

```
fn first_word<'a>(s: &'a str) -> &'a str {
```

After writing a lot of Rust code, the Rust team found that Rust programmers
were typing the same lifetime annotations over and over in particular
situations. These situations were predictable and followed a few deterministic
patterns. The Rust team then programmed these patterns into the Rust compiler’s
code so that the borrow checker can infer the lifetimes in these situations
without forcing the programmer to explicitly add the annotations.

We mention this piece of Rust history because it’s entirely possible that more
deterministic patterns will emerge and be added to the compiler. In the future,
even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the
*lifetime elision rules*. These aren’t rules for programmers to follow; the
rules are a set of particular cases that the compiler will consider, and if
your code fits these cases, you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference: if Rust deterministically
applies the rules but there’s still ambiguity as to what lifetimes the
references have, it won’t guess what the lifetime of the remaining references
should be. In this case, the compiler will give you an error that can be
resolved by adding the lifetime annotations that correspond to your intentions
for how the references relate to each other.

First, some definitions: Lifetimes on function or method parameters are called
*input lifetimes*, and lifetimes on return values are called *output lifetimes*.

Now, on to the rules that the compiler uses to figure out what lifetimes
references have when there aren’t explicit annotations. The first rule applies
to input lifetimes, and the second two rules apply to output lifetimes. If the
compiler gets to the end of the three rules and there are still references that
it can’t figure out lifetimes for, the compiler will stop with an error.

1. Each parameter that is a reference gets its own lifetime parameter. In other
   words, a function with one parameter gets one lifetime parameter: `fn
   foo<'a>(x: &'a i32)`, a function with two arguments gets two separate
   lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, and so on.

2. If there is exactly one input lifetime parameter, that lifetime is assigned
   to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

3. If there are multiple input lifetime parameters, but one of them is `&self`
   or `&mut self` because this is a method, then the lifetime of `self` is
   assigned to all output lifetime parameters. This makes writing methods much
   nicer.

Let’s pretend we’re the compiler and apply these rules to figure out what the
lifetimes of the references in the signature of the `first_word` function in
Listing 10-25 are. The signatures starts without any lifetimes associated with
the references:

```
fn first_word(s: &str) -> &str {
```

Then we (as the compiler) apply the first rule, which says each parameter gets
its own lifetime. We’re going to call it `'a` as usual, so now the signature is:

```
fn first_word<'a>(s: &'a str) -> &str {
```

On to the second rule, which applies because there is exactly one input
lifetime. The second rule says the lifetime of the one input parameter gets
assigned to the output lifetime, so now the signature is:

```
fn first_word<'a>(s: &'a str) -> &'a str {
```

Now all the references in this function signature have lifetimes, and the
compiler can continue its analysis without needing the programmer to annotate
the lifetimes in this function signature.

Let’s do another example, this time with the `longest` function that had no
lifetime parameters when we started working with in Listing 10-20:

```
fn longest(x: &str, y: &str) -> &str {
```

Pretending we’re the compiler again, let’s apply the first rule: each parameter
gets its own lifetime. This time we have two parameters, so we have two
lifetimes:

```
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Looking at the second rule, it doesn’t apply since there is more than one input
lifetime. Looking at the third rule, this also does not apply because this is a
function rather than a method, so none of the parameters are `self`. So we’re
out of rules, but we haven’t figured out what the return type’s lifetime is.
This is why we got an error trying to compile the code from Listing 10-20: the
compiler worked through the lifetime elision rules it knows, but still can’t
figure out all the lifetimes of the references in the signature.

Because the third rule only really applies in method signatures, let’s look at
lifetimes in that context now, and see why the third rule means we don’t have
to annotate lifetimes in method signatures very often.

### Lifetime Annotations in Method Definitions

<!-- Is this different to the reference lifetime annotations, or just a
finalized explanation? -->
<!-- This is about lifetimes on references in method signatures, which is where
the 3rd lifetime elision rule kicks in. It can also be confusing where lifetime
parameters need to be declared and used since the lifetime parameters could go
with the struct's fields or with references passed into or returned from
methods. /Carol -->

When we implement methods on a struct with lifetimes, the syntax is again the
same as that of generic type parameters that we showed in Listing 10-10: the
place that lifetime parameters are declared and used depends on whether the
lifetime parameter is related to the struct fields or the method arguments and
return values.

Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct’s name, since those lifetimes are part
of the struct’s type.

In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct’s fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren’t necessary in method signatures. Let’s look at some examples using the
struct named `ImportantExcerpt` that we defined in Listing 10-24.

First, here’s a method named `level`. The only parameter is a reference to
`self`, and the return value is just an `i32`, not a reference to anything:

```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

The lifetime parameter declaration after `impl` and use after the type name is
required, but we’re not required to annotate the lifetime of the reference to
`self` because of the first elision rule.

Here’s an example where the third lifetime elision rule applies:

```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.

### The Static Lifetime

There is *one* special lifetime we need to discuss: `'static`. The `'static`
lifetime is the entire duration of the program. All string literals have the
`'static` lifetime, which we can choose to annotate as follows:

```
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the binary of your program and
the binary of your program is always available. Therefore, the lifetime of all
string literals is `'static`.

<!-- How would you add a static lifetime (below)? -->
<!-- Just like you'd specify any lifetime, see above where it shows `&'static str`. /Carol -->

You may see suggestions to use the `'static` lifetime in error message help
text, but before specifying `'static` as the lifetime for a reference, think
about whether the reference you have is one that actually lives the entire
lifetime of your program or not (or even if you want it to live that long, if
it could). Most of the time, the problem in the code is an attempt to create a
dangling reference or a mismatch of the available lifetimes, and the solution
is fixing those problems, not specifying the `'static` lifetime.

### Generic Type Parameters, Trait Bounds, and Lifetimes Together

Let’s briefly look at the syntax of specifying generic type parameters, trait
bounds, and lifetimes all in one function!

```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This is the `longest` function from Listing 10-21 that returns the longest of
two string slices, but with an extra argument named `ann`. The type of `ann` is
the generic type `T`, which may be filled in by any type that implements the
`Display` trait as specified by the `where` clause. This extra argument will be
printed out before the function compares the lengths of the string slices,
which is why the `Display` trait bound is necessary. Because lifetimes are a
type of generic, the declarations of both the lifetime parameter `'a` and the
generic type parameter `T` go in the same list within the angle brackets after
the function name.

## Summary

We covered a lot in this chapter! Now that you know about generic type
parameters, traits and trait bounds, and generic lifetime parameters, you’re
ready to write code that isn’t duplicated but can be used in many different
situations. Generic type parameters mean the code can be applied to different
types. Traits and trait bounds ensure that even though the types are generic,
those types will have the behavior the code needs. Relationships between the
lifetimes of references specified by lifetime annotations ensure that this
flexible code won’t have any dangling references. And all of this happens at
compile time so that run-time performance isn’t affected!

Believe it or not, there’s even more to learn in these areas: Chapter 17 will
discuss trait objects, which are another way to use traits. Chapter 19 will be
covering more complex scenarios involving lifetime annotations. Chapter 20 will
get to some advanced type system features. Up next, though, let’s talk about
how to write tests in Rust so that we can make sure our code using all these
features is working the way we want it to!
