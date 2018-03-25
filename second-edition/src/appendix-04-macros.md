## Appendix D: Macros

We’ve used macros like `println!` throughout this book, but haven’t fully
explored what a macro is and how it works. This appendix will explain:

- What macros are and how they differ from functions
- How to define a declarative macro to do metaprogramming
- How to define a procedural macro to create custom `derive` traits

We’re covering the details of macros in an appendix because they’re still
evolving in Rust. Macros have changed and, in the near future, will change at a
quicker rate than the rest of the language and standard library since Rust 1.0,
so this section is more likely to date than the rest of the book. The code
shown here will still continue to work with future versions, due to Rust’s
stability guarantees, but there may be additional capabilities or easier ways
to write macros that weren’t available at the time of this publication. Bear
that in mind if you try to implement anything from this appendix.

### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code, known
as *metaprogramming*. In Appendix C, we discussed the `derive` attribute, which
generates an implementation of various traits for you. We’ve also used the
`println!` and `vec!` macros throughout the book. All of these macros *expand*
to produce more code than the code you’ve written yourself.

Metaprogramming is useful for reducing the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don’t have.

A function signature has to declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: we can call `println!("hello")` with one argument, or
`println!("hello {}", name)` with two arguments. Also, macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can’t, because it gets
called at runtime and a trait needs to be implemented at compile time.

The downside to implementing a macro over a function is that macro definitions
are more complex than function definitions because you’re writing Rust code
that writes Rust code. Due to this indirection, macro definitions are generally
more difficult to read, understand, and maintain than function definitions.

Another difference between macros and functions is that macro definitions
aren’t namespaced within modules like function definitions are. In order to
prevent unexpected name clashes when using external crates, you have to
explicitly bring the macros into the scope of your project at the same time as
bringing the external crate into scope, using the `#[macro_use]` annotation.
The following example would bring all the macros defined in the `serde` crate
into the scope of the current crate:

```rust,ignore
#[macro_use]
extern crate serde;
```

If `extern crate` was able to bring macros into scope by default without this
explicit annotation, you would be prevented from using two crates that happened
to define macros with the same name. In practice this conflict doesn’t come up
much, but the more crates you use, the more likely it is.

One last important difference between macros and functions: macros must be
defined or brought into scope *before* they’re called in a file, whereas
functions can be defined anywhere and called anywhere.

### Declarative Macros with `macro_rules!` for General Metaprogramming

The most widely used form of macros in Rust are *declarative macros*. These are
also sometimes referred to as *macros by example*, *`macro_rules!` macros*, or
just plain *macros*. At their core, declarative macros allow you to write
something similar to a Rust `match` expression. As discussed in Chapter 6,
`match` expressions are control structures that take an expression, compare the
resulting value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that have
code associated with them, but in this case the value is the literal Rust
source code passed to the macro, the patterns are compared with the structure
of that source code, and the code associated with each pattern is the code that
replaces the code passed to the macro. This all happens during compilation.

To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by taking a look at how the `vec!` macro is defined. Chapter
8 covered how we can use the `vec!` macro to create a new vector with
particular values. For example, this macro creates a new vector with three
integers inside:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

We could also use the `vec!` macro to make a vector of two integers or a vector
of five string slices---we wouldn’t be able to use a function to do the same
because we wouldn’t know the number or type of values up front.

Let’s take a look at a slightly simplified definition of the `vec!` macro in
Listing AD-1:

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

Listing AD-1: A simplified version of the `vec!` macro definition

> Note: the actual definition of the `vec!` macro in the standard library
> includes code to pre-allocate the correct amount of memory up-front. That
> code is an optimization that we’ve chosen not to include here to make the
> example simpler.

The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which we’re defining the macro is imported.
Without this annotation, even if someone depending on this crate uses the
`#[macro_use]` annotation, the macro would not be brought into scope.

We then start the macro definition with `macro_rules!` and the name of the
macro we’re defining *without* the exclamation mark. The name---in this case
`vec`---is followed by curly brackets denoting the body of the macro definition.

The structure in the `vec!` body is similar to the structure of a `match`
expression. Here we have one arm with the pattern `( $( $x:expr ),* )`,
followed by `=>` and the block of code associated with this pattern. If the
pattern matches, the associated block of code will be emitted. Given that this
is the only pattern in this macro, there’s only one valid way to match; any
other will be an error. More complex macros will have more than one arm.

Valid pattern syntax in macro definitions is different than the pattern syntax
covered in Chapter 18 because macro patterns are matched against Rust code
structure rather than values. Let’s walk through what the pieces of the pattern
in Listing AD-1 mean; for the full macro pattern syntax, see [the reference].

[the reference]: ../../reference/macros.html

First, a set of parentheses encompasses the whole pattern. Next comes a dollar
sign (`$`) followed by a set of parentheses, which captures values that match
the pattern within the parentheses for use in the replacement code. Within
`$()` is `$x:expr`, which matches any Rust expression and gives the expression
the name `$x`.

The comma following `$()` indicates that a literal comma separator character
could optionally appear after the code that matches the code captured in `$()`.
The `*` following the comma specifies that the pattern matches zero or more of
whatever precedes the `*`.

When we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three
times with the three expressions `1`, `2`, and `3`.

Now let’s look at the pattern in the body of the code associated with this arm:
The `temp_vec.push()` code within the `$()*` part is generated for each part
that matches `$()` in the pattern, zero or more times depending on how many
times the pattern matches. The `$x` is replaced with each expression matched.
When we call this macro with `vec![1, 2, 3];`, the code generated that replaces
this macro call will be:

<!-- Above What about temp_vec.push, do you want to quickly mention that? Or do
you mean "The `$()*` part and the content of the parentheses is generated for
each part that matches `$()` in the pattern"-->
<!-- The latter, I've tried to clarify /Carol -->

```rust,ignore
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

We’ve defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.

Given that most Rust programmers will *use* macros more than *write* macros,
that’s all we’ll discuss about `macro_rules!` in this book. To learn more about
how to write macros, consult the online documentation or other resources such
as [The Little Book of Rust Macros][tlborm].

[tlborm]: https://danielkeep.github.io/tlborm/book/index.html

### Procedural Macros for Custom `derive`

The second form of macros is called *procedural macros* because they’re more
like functions (which are a type of procedure). Procedural macros accept some
Rust code as an input, operate on that code, and produce some Rust code as an
output, rather than matching against patterns and replacing the code with other
code as declarative macros do. At the time of writing, you can only really
define procedural macros to allow your traits to be implemented on a type by
specifying the trait name in a `derive` annotation.

We’re going to create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making users of our crate implement the `HelloMacro` trait for each of their
types, we’ll provide a procedural macro so users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined.

In other words, we’re going to write a crate that enables another programmer to
write code like Listing AD-2 using our crate:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

<span class="caption">Listing AD-2: The code a user of our crate will be able
to write with use of our procedural macro</span>

This code will print `Hello, Macro! My name is Pancakes!` when we’re done. Let’s
get started! First we need to make a new library crate:

```text
$ cargo new hello_macro --lib
```

Now we’ll define the `HelloMacro` trait and its associated function:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

We have a trait and its function. At this point, a user of our crate would be
able to implement the trait themselves to achieve the desired functionality,
like so:

```rust,ignore
extern crate hello_macro;

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

However, they would need to write out the implementation block for each type
they wanted to use with `hello_macro`; we’d like to save them this work.

Additionally, we can’t yet provide a default implementation for the
`hello_macro` function that will print out the name of the type the trait is
implemented on: Rust doesn’t have reflection capabilities, so can’t look up the
type’s name at runtime. We need a macro to generate code at compile time.

<!--Defining Procedural Macros Requires a Separate Crate--> <!-- Since this is
a lone subheading, okay to merge with the general procedural macros section? -->
<!-- Sure /Carol -->

The next step is to define the procedural macro. At the time of writing,
procedural macros need to be in their own crate. Eventually, this restriction
may be lifted. The convention for structuring crates and macro crates is as
such: for a crate named `foo`, a custom derive procedural macro crate is called
`foo-derive`. Let’s start a new crate called `hello_macro_derive` inside our
`hello_macro` project:

```text
$ cargo new hello_macro_derive --lib
```

Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we’ll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, though, and programmers using these crates will need
to add both as dependencies and bring them both into scope. We could instead
have the `hello_macro` crate use `hello_macro_derive` as a dependency and
re-export the procedural macro code, but the way we’ve structured the project
makes it possible for programmers to use `hello_macro` even if they don’t want
the `derive` functionality.

We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We’ll also need functionality from the `syn` and `quote` crates, as we’ll see
in a moment, so we need to add them as dependencies. Add the following to the
*Cargo.toml* file for `hello_macro_derive`:

<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>

```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

To start defining the procedural macro, place the code from Listing AD-3 in
your *src/lib.rs* for the `hello_macro_derive` crate. Note that this won’t
compile until we add a definition for the `impl_hello_macro` function.

Note the way we’ve split the functions in AD-3; this will be the same for
almost every procedural macro crate you see or create, as it makes writing a
procedural macro more convenient. What you choose to do in the place where the
`impl_hello_macro` function is called will be different depending on the
purpose of your procedural macro.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_macro(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```

<span class="caption">Listing AD-3: Code that most procedural macro crates will
need to have for processing Rust code</span>

We have introduced three new crates: `proc_macro`, [`syn`], and [`quote`]. The
`proc_macro` crate comes with Rust, so we didn’t need to add that to the
dependencies in *Cargo.toml*. The `proc_macro` crate allows us to convert Rust
code into a string containing that Rust code. The `syn` crate parses Rust code
from a string into a data structure that we can perform operations on. The
`quote` crate takes `syn` data structures and turns them back into Rust code.
These crates make it much simpler to parse any sort of Rust code we might want
to handle: writing a full parser for Rust code is no simple task.

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

The `hello_macro_derive` function will get called when a user of our library
specifies `#[derive(HelloMacro)]` on a type, because we’ve annotated the
`hello_macro_derive` function here with `proc_macro_derive` and specified the
name, `HelloMacro`, which matches our trait name; that’s the convention most
procedural macros follow.

This function first converts the `input` from a `TokenStream` to a `String` by
calling `to_string`. This `String` is a string representation of the Rust code
for which we are deriving `HelloMacro`. In the example in Listing AD-2, `s`
will have the `String` value `struct Pancakes;` because that’s the Rust code we
added the `#[derive(HelloMacro)]` annotation to.

<!-- I'm not sure why we convert to a string then to a structure we can use,
will that be clear to the reader here? -->
<!-- This is just how procedural macros work and what you have to do, which is
why we have the next note. /Carol -->

> Note: At the time of writing, the only thing you can do with a `TokenStream`
> is convert it to a string. A richer API will exist in the future.

Now we need to be able to parse the Rust code `String` into a data structure
that we can then interpret and perform operations on. This is where `syn` comes
to play. The `parse_derive_input` function in `syn` takes a `String` and
returns a `DeriveInput` struct representing the parsed Rust code. The following
shows the relevant parts of the `DeriveInput` struct we get from parsing the
string `struct Pancakes;`:

```rust,ignore
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

The fields of this struct show that the Rust code we’ve parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
API docs for `DeriveInput`][syn-docs] for more information.

[syn-docs]: https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html

At this point we haven’t defined the `impl_hello_macro` function, which is
where we’ll build the new Rust code we want to include. Before we get to that,
the last part of this `hello_macro_derive` function is using the `parse`
function from the `quote` crate to turn the output of the `impl_hello_macro`
function back into a `TokenStream`. The returned `TokenStream` is added to the
code that users of our crate write so that when they compile their crate, they
get extra functionality we provide.

You may have noticed that we’re calling `unwrap` to panic if the calls to the
`parse_derive_input` or `parse` functions fail here. Panicking on errors is
necessary in procedural macro code because `proc_macro_derive` functions must
return `TokenStream` rather than `Result` in order to conform to the procedural
macro API. We’ve chosen to keep this example simple by using `unwrap`; in
production code you should provide more specific error messages about what went
wrong by using `expect` or `panic!`.

Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `String` and a `DeriveInput` instance, let’s generate the code
implementing the `HelloMacro` trait on the annotated type:

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    }
}
```

We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. With the code from Listing AD-2, `name` will
be `Ident("Pancakes")`.

The `quote!` macro lets us write up the Rust code that we want to return and
convert it into `quote::Tokens`. This macro also provides some really cool
templating mechanics; we can write `#name` and `quote!` will replace it with
the value in the variable named `name`. You can even do some repetition similar
to the way regular macros work. Check out [the `quote` crate’s
docs][quote-docs] for a thorough introduction.

[quote-docs]: https://docs.rs/quote

We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has one function, `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.

The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different than `format!` or
`println!`, which evaluate the expression and then turn the result into a
`String`. There’s a possibility that the `#name` input might be an expression
to print out literally so we use `stringify!`. Using `stringify!` also saves an
allocation by converting `#name` to a string literal at compile time.

At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let’s hook these crates up to the code in Listing
AD-2 to see it in action! Create a new binary project in your `projects`
directory with `cargo new --bin pancakes`. We need to add both `hello_macro`
and `hello_macro_derive` as dependencies in the `pancakes` crate’s
*Cargo.toml*. If you’ve chosen to publish your versions of `hello_macro` and
`hello_macro_derive` to *https://crates.io* they would be regular dependencies;
if not, you can specify them as `path` dependencies as follows:

```toml
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

Put the code from Listing AD-2 into *src/main.rs*, and when you run `cargo run`
it should print `Hello, Macro! My name is Pancakes!` The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` took care
of adding the trait implementation.

### The Future of Macros

In the future, we’ll be expanding both declarative and procedural macros. Rust
will use better declarative macro system with the `macro` keyword, and we’ll
add more types of procedural macros, for more powerful tasks than just
`derive`. These systems are still under development at the time of publication;
please consult the online Rust documentation for the latest information.
