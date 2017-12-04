# D - Macros

We've used macros, such as `println!`, throughout this book. This appendix will
explain:

- What macros are and how they differ from functions
- How to define a declarative macro to do metaprogramming
- How to define a procedural macro to create custom `derive` traits

Macros are covered in an appendix because they're still evolving. They have
changed and will change more than the rest of the language and standard library
since Rust 1.0, so this section will likely get out of date more than the rest
of this book. The code shown here will still continue to work due to Rust's
stability guarantees, but there may be additional capabilities or easier ways
to write macros that aren't available at the time of this publication.

## Macros are More Flexible and Complex than Functions

Fundamentally, macros are a way of writing code that writes other code, which
is known as *metaprogramming*. In the previous appendix, we discussed the
`derive` attribute, which generates an implementation of various traits for
you. We've also used the `println!` and `vec!` macros. All of these macros
*expand* to produce more code than what you've written in your source code.

Metaprogramming is useful to reduce the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don't have, as we discussed in Chapter 1.
A function signature has to declare the number and type of parameters the
function has. Macros can take a variable number of parameters: we can call
`println!("hello")` with one argument, or `println!("hello {}", name)` with two
arguments. Also, macros are expanded before the compiler interprets the meaning
of the code, so a macro can, for example, implement a trait on a given type,
whereas a function can't because a function gets called at runtime and a trait
needs to be implemented at compile time.

The downside to implementing a macro rather than a function is that macro
definitions are more complex than function definitions. You're writing Rust
code that writes Rust code, and macro definitions are generally more difficult
to read, understand, and maintain than function definitions.

Another difference between macros and functions is that macro definitions
aren't namespaced within modules like function definitions are. In order to
prevent unexpected name clashes when using a crate, when bringing an external
crate into the scope of your project, you have to explicitly bring the macros
into the scope of your project as well with the `#[macro_use]` annotation. This
example would bring all the macros defined in the `serde` crate into the scope
of the current crate:

```rust,ignore
#[macro_use]
extern crate serde;
```

If `extern crate` also brought macros into scope by default, you wouldn't be
allowed to use two crates that happened to define macros with the same name. In
practice this conflict doesn't come up much, but the more crates you use, the
more likely it is.

One last important difference between macros and functions: macros must be
defined or brought into scope before they're called in a file. Unlike
functions, where we can define a function at the bottom of a file yet call it
at the top, we always have to define macros before we're able to call them.

## Declarative Macros with `macro_rules!` for General Metaprogramming

The first form of macros in Rust, and the one that's most widely used, is
called *declarative macros*. These are also sometimes referred to as *macros by
example*, *`macro_rules!` macros*, or just plain *macros*. At their core,
declarative macros allow you to write something similar to a Rust `match`
expression. As discussed in Chapter 6, `match` expressions are control
structures that take an expression, compare the resulting value of the
expression to patterns, and then choose the code specified with the matching
pattern when the program runs. Macros also have a value that is compared to
patterns that have code associated with them, but the value is the literal Rust
code passed to the macro, the patterns match the structure of that source code,
and the code associated with each pattern is the code that is generated to
replace the code passed to the macro. This all happens during compilation.

To define a macro, you use the `macro_rules!` construct. Let's explore how to
use `macro_rules!` by taking a look at how the `vec!` macro is defined. Chapter
8 covered how we can use the `vec!` macro to create a new vector that holds
particular values. For example, this macro creates a new vector with three
integers inside:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

We can also use `vec!` to make a vector of two integers or a vector of five
string slices. Because we don't know the number or type of values, we can't
define a function that is able to create a new vector with the given elements
like `vec!` can.

Let's take a look at a slightly simplified definition of the `vec!` macro:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

> Note: the actual definition of the `vec!` macro in the standard library also
> has code to pre-allocate the correct amount of memory up-front. That code
> is an optimization that we've chosen not to include here for simplicity.

The `#[macro_export]` annotation indicates that this macro should be made
available when other crates import the crate in which we're defining this
macro. Without this annotation, even if someone depending on this crate uses
the `#[macro_use]` annotation, this macro would not be brought into scope.

Macro definitions start with `macro_rules!` and the name of the macro we're
defining without the exclamation mark, which in this case is `vec`. This is
followed by curly brackets denoting the body of the macro definition.

Inside the body is a structure similar to the structure of a `match`
expression. This macro definition has one arm with the pattern `( $( $x:expr
),* )`, followed by `=>` and the block of code associated with this pattern. If
this pattern matches, then the block of code will be emitted. Given that this
is the only pattern in this macro, there's only one valid way to match; any
other will be an error. More complex macros will have more than one arm.

The pattern syntax valid in macro definitions is different than the pattern
syntax covered in Chapter 18 because the patterns are for matching against Rust
code structure rather than values. Let's walk through what the pieces of the
pattern used here mean; for the full macro pattern syntax, see [the reference].

[the reference]: ../../reference/macros.html

The `$x:expr` part of the pattern matches any Rust expression and gives the
expression the name `$x`. The `*` specifies that the pattern matches zero or
more of whatever precedes the `*`. In this case, `*` is preceded by `$(),` so
this pattern matches zero or more of whatever is inside the parentheses,
delimited by a comma. When we call this macro with `vec![1, 2, 3];`, the
pattern matches the three expressions `1`, `2`, and `3`.

In the body of the code associated with this arm, the `$()*` part is generated
for each part that matches `$()` in the pattern, zero or more times depending
on how many times the pattern matches. The `$x` in the code associated with the
arm is replaced with each expression matched. When we call this macro with
`vec![1, 2, 3];`, the code generated that replaces this macro call will be:

```rust,ignore
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

We've defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.

Given that most Rust programmers will *use* macros more than *write* macros,
that's all we'll discuss about `macro_rules!` in this book. To learn more about
how to write macros, consult the online documentation or other resources such
as [The Little Book of Rust Macros][tlborm].

[tlborm]: https://danielkeep.github.io/tlborm/book/index.html

## Procedural Macros for Custom `derive`

The second form of macros is called *procedural macros* because they're more
like functions (which are a type of procedure). Procedural macros accept some
Rust code as an input, operate on that code, and produce some Rust code as an
output, rather than matching against patterns and replacing the code with other
code as declarative macros do. Today, the only thing you can define procedural
macros for is to allow your traits to be implemented on a type by specifying
the trait name in a `derive` annotation.

Let's create a crate named `hello-world` that defines a trait named
`HelloWorld` with one associated function named `hello_world`. Rather than
making users of our crate implement the `HelloWorld` trait for each of their
types, we'd like users to be able to annotate their type with
`#[derive(HelloWorld)]` to get a default implementation of the `hello_world`
function associated with their type. The default implementation will print
`Hello world, my name is TypeName!` where `TypeName` is the name of the type on
which this trait has been defined.

In other words, we're going to write a crate that enables another programmer to
write code that looks like Listing A4-1 using our crate:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate hello_world;
#[macro_use]
extern crate hello_world_derive;

use hello_world::HelloWorld;

#[derive(HelloWorld)]
struct Pancakes;

fn main() {
    Pancakes::hello_world();
}
```

<span class="caption">Listing A4-1: The code a user of our crate will be able
to write when we've written the procedural macro</span>

This code will print `Hello world, my name is Pancakes!` when we're done. Let's
get started!

Let's make a new library crate:

```text
$ cargo new hello-world
```

First, we'll define the `HelloWorld` trait and associated function:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait HelloWorld {
    fn hello_world();
}
```

At this point, a user of our crate could implement the trait themselves to
achieve the functionality we wanted to enable, like so:

```rust,ignore
extern crate hello_world;

use hello_world::HelloWorld;

struct Pancakes;

impl HelloWorld for Pancakes {
    fn hello_world() {
        println!("Hello world, my name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_world();
}
```

However, they would need to write out the implementation block for each type
they wanted to be able to use with `hello_world`; we'd like to make using our
trait more convenient for other programmers by saving them this work.

Additionally, we can't provide a default implementation for the `hello_world`
function that has the behavior we want of printing out the name of the type the
trait is implemented on: Rust doesn't have reflection capabilities, so we can't
look up the type's name at runtime. We need a macro to generate code at compile
time.

### Defining Procedural Macros Requires a Separate Crate

The next step is to define the procedural macro. At the moment, procedural
macros need to be in their own crate. Eventually, this restriction may be
lifted, but for now, it's required. As such, there's a convention: for a crate
named `foo`, a custom derive procedural macro crate is called `foo-derive`.
Let's start a new crate called `hello-world-derive` inside our `hello-world`
project:

```text
$ cargo new hello-world-derive
```

We've chosen to create the procedural macro crate within the directory of our
`hello-world` crate because the two crates are tightly related: if we change
the trait definition in `hello-world`, we'll have to change the implementation
of the procedural macro in `hello-world-derive` as well. The two crates will
need to be published separately, and programmers using these crates will need
to add both as dependencies and bring them both into scope. It's possible to
have the `hello-world` crate use `hello-world-derive` as a dependency and
re-export the procedural macro code, but structuring the project this way makes
it possible for programmers to easily decide they only want to use
`hello-world` if they don't want the `derive` functionality.

We need to declare that the `hello-world-derive` crate is a procedural macro
crate. We also need to add dependencies on the `syn` and `quote` crates to get
useful functionality for operating on Rust code. To do these two things, add
the following to the *Cargo.toml* for `hello-world-derive`:

<span class="filename">Filename: hello-world-derive/Cargo.toml</span>

```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

To start defining the procedural macro, place the code from Listing A4-2 in
*src/lib.rs* for the `hello-world-derive` crate. Note that this won't compile
until we add a definition for the `impl_hello_world` function. We've split the
code into functions in this way because the code in Listing A4-2 will be the
same for almost every procedural macro crate; it's code that makes writing a
procedural macro more convenient. What you choose to do in the place where the
`impl_hello_world` function is called will be different and depend on the
purpose of your procedural macro.

<span class="filename">Filename: hello-world-derive/src/lib.rs</span>

```rust,ignore
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```

<span class="caption">Listing A4-2: Code that most procedural macro crates will
need to have for processing Rust code</span>

We have introduced three new crates: `proc_macro`, [`syn`], and [`quote`]. The
`proc_macro` crate comes with Rust, so we didn't need to add that to the
dependencies in *Cargo.toml*. The `proc_macro` crate allows us to convert Rust
code into a string containing that Rust code. The `syn` crate parses Rust code
from a string into a data structure that we can perform operations on. The
`quote` crate takes `syn` data structures and turns them back into Rust code.
These crates make it much simpler to parse any sort of Rust code we might want
to handle: writing a full parser for Rust code is no simple task.

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

The `hello_world_derive` function is the code that will get called when a user
of our library specifies the `#[derive(HelloWorld)]` annotation on a type
because we've annotated the `hello_world_derive` function here with
`proc_macro_derive` and specified the same name, `HelloWorld`. This name
matches our trait named `HelloWorld`; that's the convention most procedural
macros follow.

The first thing this function does is convert the `input` from a `TokenStream`
to a `String` by calling `to_string`. This `String` is a string representation
of the Rust code for which we are deriving `HelloWorld`. In the example in
Listing A4-1, `s` will have the `String` value `struct Pancakes;` because
that's the Rust code we added the `#[derive(HelloWorld)]` annotation to.

At the moment, the only thing you can do with a `TokenStream` is convert it to
a string. A richer API will exist in the future.

What we really need is to be able to parse the Rust code `String` into a data
structure that we can then interpret and perform operations on. This is where
`syn` comes to play. The `parse_derive_input` function in `syn` takes a
`String` and returns a `DeriveInput` struct representing the parsed Rust code.
Here's the relevant parts of the `DeriveInput` struct we get from parsing the
string `struct Pancakes;`:

```text
DeriveInput {
    // --snip--

    ident: Ident(
        "Pancakes"
    ),
    body: Struct(
        Unit
    )
}
```

The fields of this struct show that the Rust code we've parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
API docs for `DeriveInput`][syn-docs] for more information.

[syn-docs](https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html)

We haven't defined the `impl_hello_world` function; that's where we'll build
the new Rust code we want to include. Before we get to that, the last part of
this `hello_world_derive` function is using the `quote` crate's `parse`
function to turn the output of the `impl_hello_world` function back into a
`TokenStream`. The returned `TokenStream` is added to the code that users of
our crate write so that when they compile their crate, they get extra
functionality we provide.

You may have noticed that we're calling `unwrap` to panic if the calls to the
`parse_derive_input` or `parse` functions fail because they're unable to parse
the `TokenStream` or generate a `TokenStream`. Panicking on errors is necessary
in procedural macro code because `proc_macro_derive` functions must return
`TokenStream` rather than `Result` in order to conform to the procedural macro
API. We've chosen to keep this example simple by using `unwrap`; in production
code you should provide more specific error messages about what went wrong by
using `expect` or `panic!`.

Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `String` and into a `DeriveInput` instance, let's write the code that
will generate the code implementing the `HelloWorld` trait on the annotated
type:

<span class="filename">Filename: hello-world-derive/src/lib.rs</span>

```rust,ignore
fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}
```

We are able to get an `Ident` struct instance containing the name (identifier)
of the annotated type using `ast.ident`. With the code from Listing A4-1,
`name` will be `Ident("Pancakes")`.

The `quote!` macro from the `quote` crate lets us write up the Rust code that
we wish to return and convert it into `quote::Tokens`. The `quote!` macro lets
us use some really cool templating mechanics; we can write `#name` and `quote!`
will replace it with the value in the variable named `name`. You can even do
some repetition similar to the way regular macros work. Check out [the `quote`
crate's docs][quote-docs] for a thorough introduction.

[quote-docs]: https://docs.rs/quote

What we want to do for our procedural macro is generate an implementation of
our `HelloWorld` trait for the type the user of our crate has annotated, which
we can get by using `#name`. The trait implementation has one function,
`hello_world`, and the function body contains the functionality we want to
provide: printing `Hello, World! My name is` and then the name of the type the
user of our crate has annotated. The `stringify!` macro used here is built into
Rust. It takes a Rust expression, such as `1 + 2`, and at compile time turns
the expression into a string literal, such as `"1 + 2"`. This is different than
`format!` or `println!`, which evaluate the expression and then turn the result
into a `String`. There's a possibility that `#name` would be an expression that
we would want to print out literally, and `stringify!` also saves an allocation
by converting `#name` to a string literal at compile time.

At this point, `cargo build` should complete successfully in both `hello-world`
and `hello-world-derive`. Let's hook these crates up to the code in Listing
A4-1 to see it in action! Create a new binary project in your `projects`
directory with `cargo new --bin pancakes`. We need to add both `hello-world`
and `hello-world-derive` as dependencies in the `pancakes` crate's
*Cargo.toml*. If you've chosen to publish your versions of `hello-world` and
`hello-world-derive` to *https://crates.io* they would be regular dependencies;
if not, you can specify them as `path` dependencies as follows:

```toml
[dependencies]
hello_world = { path = "../hello-world" }
hello_world_derive = { path = "../hello-world/hello-world-derive" }
```

Put the code from Listing A4-1 into *src/main.rs*, and executing `cargo run`
should print `Hello, World! My name is Pancakes`! The implementation of the
`HelloWorld` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloWorld)]` took care
of adding the trait implementation.

## The Future of Macros

In the future, we'll be expanding both declarative and procedural macros. A
better declarative macro system will be used with the `macro` keyword, and
we'll add more types of procedural macros, for more powerful tasks than only
`derive`. These systems are still under development at the time of publication;
please consult the online Rust documentation for the latest information.
