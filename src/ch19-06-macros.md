## Macros

We’ve used macros like `println!` throughout this book, but we haven’t fully
explored what a macro is and how it works. *Macros* refers to a family of
features in Rust:

* *Declarative* macros with `macro_rules!`
* *Procedural* macros, which come in three kinds:
    * Custom `#[derive]` macros
    * Attribute-like macros
    * Function-like macros

We’ll talk about each of these in turn, but first, why do we even need macros
when we already have functions?

### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code, which
is known as *metaprogramming*. In Appendix C, we discuss the `derive`
attribute, which generates an implementation of various traits for you. We’ve
also used the `println!` and `vec!` macros throughout the book. All of these
macros *expand* to produce more code than the code you’ve written manually.

Metaprogramming is useful for reducing the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don’t have.

A function signature must declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: we can call `println!("hello")` with one argument or
`println!("hello {}", name)` with two arguments. Also, macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can’t, because it gets
called at runtime and a trait needs to be implemented at compile time.

The downside to implementing a macro instead of a function is that macro
definitions are more complex than function definitions because you’re writing
Rust code that writes Rust code. Due to this indirection, macro definitions are
generally more difficult to read, understand, and maintain than function
definitions.

There is one last important difference between macros and functions: you must
define or bring macros into scope *before* you call them in a file, whereas you
can define functions anywhere and call them anywhere.

### Declarative Macros with `macro_rules!` for General Metaprogramming

The most widely used form of macros in Rust are *declarative macros*. These are
also sometimes referred to as “macros by example”, “`macro_rules!` macros”, or
just plain “macros”. At their core, declarative macros are written in a form
similar to a Rust `match` expression. As discussed in Chapter 6, `match`
expressions are control structures that take an expression, compare the
resulting value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that have
code associated with them; in this situation, the value is the literal Rust
source code passed to the macro, the patterns are compared with the structure
of that source code, and the code associated with each pattern is the code that
replaces the code passed to the macro. This all happens during compilation.

To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by looking at how the `vec!` macro is defined. Chapter 8
covered how we can use the `vec!` macro to create a new vector with particular
values. For example, the following macro creates a new vector with three
integers inside:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

We could also use the `vec!` macro to make a vector of two integers or a vector
of five string slices. We wouldn’t be able to use a function to do the same
because we wouldn’t know the number or type of values up front.

Let’s look at a slightly simplified definition of the `vec!` macro in Listing
19-36.

<span class="filename">Filename: src/lib.rs</span>

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

<span class="caption">Listing 19-36: A simplified version of the `vec!` macro
definition</span>

> Note: The actual definition of the `vec!` macro in the standard library
> includes code to preallocate the correct amount of memory up front. That code
> is an optimization that we don’t include here to make the example simpler.

The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which we’re defining the macro is brought into
scope. Without this annotation, the macro can’t be brought into scope.

We then start the macro definition with `macro_rules!` and the name of the
macro we’re defining *without* the exclamation mark. The name, in this case
`vec`, is followed by curly brackets denoting the body of the macro definition.

The structure in the `vec!` body is similar to the structure of a `match`
expression. Here we have one arm with the pattern `( $( $x:expr ),* )`,
followed by `=>` and the block of code associated with this pattern. If the
pattern matches, the associated block of code will be emitted. Given that this
is the only pattern in this macro, there is only one valid way to match; any
other will be an error. More complex macros will have more than one arm.

Valid pattern syntax in macro definitions is different from the pattern syntax
covered in Chapter 18 because macro patterns are matched against Rust code
structure rather than values. Let’s walk through what the pieces of the pattern
in Listing 19-36 mean; for the full macro pattern syntax, see [the reference].

[the reference]: ../reference/macros.html

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
the `temp_vec.push()` code within the `$()*` part is generated for each part
that matches `$()` in the pattern, zero or more times depending on how many
times the pattern matches. The `$x` is replaced with each expression matched.
When we call this macro with `vec![1, 2, 3];`, the code generated that replaces
this macro call will be the following:

```rust,ignore
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

We’ve defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.

There are some strange corners with `macro_rules!`. In the future, there
will be a second kind of declarative macro with the `macro` keyword that
will work in a similar fashion but fix some of these edge cases. After that
is done, `macro_rules!` will be effectively deprecated. With this
in mind, as well as the fact that most Rust programmers will *use* macros
more than *write* macros, we won’t discuss `macro_rules!` any further. To
learn more about how to write macros, consult the online documentation or
other resources, such as [“The Little Book of Rust Macros”][tlborm].

[tlborm]: https://danielkeep.github.io/tlborm/book/index.html

### Procedural Macros for Generating Code from Attributes

The second form of macros is called *procedural macros* because they’re more
like functions (which are a type of procedure). Procedural macros accept some
Rust code as an input, operate on that code, and produce some Rust code as an
output rather than matching against patterns and replacing the code with other
code as declarative macros do.

There are three kinds of procedural macros, but they all work in a similar
fashion. First, the definitions must reside in their own crate with a special
crate type. This is for complex technical reasons that we hope to eliminate in
the future.

Second, using any of these kinds of macros takes on a form like the code shown
in Listing 19-37, where `some_attribute` is a placeholder for using a specific
macro.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

<span class="caption">Listing 19-37: An example of using a procedural
macro</span>

Procedural macros consist of a function, which is how they get their name:
“procedure” is a synonym for “function.” Why not call them “functional macros”?
Well, one of the types is “function-like,” and that would get confusing.
Anyway, the function defining a procedural macro takes a `TokenStream` as an
input and produces a `TokenStream` as an output. This is the core of the macro:
the source code that the macro is operating on makes up the input
`TokenStream`, and the code the macro produces is the output `TokenStream`.
Finally, the function has an attribute on it; this attribute says which kind of
procedural macro we’re creating. We can have multiple kinds of procedural
macros in the same crate.

Given that the kinds of macros are so similar, we’ll start with a custom derive
macro. Then we’ll explain the small differences that make the other forms
different.

### How to Write a Custom `derive` Macro

Let’s create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making our crate users implement the `HelloMacro` trait for each of their
types, we’ll provide a procedural macro so users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined. In other words, we’ll write a crate that enables another
programmer to write code like Listing 19-38 using our crate.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

<span class="caption">Listing 19-38: The code a user of our crate will be able
to write when using our procedural macro</span>

This code will print `Hello, Macro! My name is Pancakes!` when we’re done. The
first step is to make a new library crate, like this:

```text
$ cargo new hello_macro --lib
```

Next, we’ll define the `HelloMacro` trait and its associated function:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

We have a trait and its function. At this point, our crate user could implement
the trait to achieve the desired functionality, like so:

```rust,ignore
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

However, they would need to write the implementation block for each type they
wanted to use with `hello_macro`; we want to spare them from having to do this
work.

Additionally, we can’t yet provide a default implementation for the
`hello_macro` function that will print the name of the type the trait is
implemented on: Rust doesn’t have reflection capabilities, so it can’t look up
the type’s name at runtime. We need a macro to generate code at compile time.

The next step is to define the procedural macro. At the time of this writing,
procedural macros need to be in their own crate. Eventually, this restriction
might be lifted. The convention for structuring crates and macro crates is as
follows: for a crate named `foo`, a custom derive procedural macro crate is
called `foo_derive`. Let’s start a new crate called `hello_macro_derive` inside
our `hello_macro` project:

```text
$ cargo new hello_macro_derive --lib
```

Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we’ll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, and programmers using these crates will need to add
both as dependencies and bring them both into scope. We could instead have the
`hello_macro` crate use `hello_macro_derive` as a dependency and reexport the
procedural macro code. But the way we’ve structured the project makes it
possible for programmers to use `hello_macro` even if they don’t want the
`derive` functionality.

We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We’ll also need functionality from the `syn` and `quote` crates, as you’ll see
in a moment, so we need to add them as dependencies. Add the following to the
*Cargo.toml* file for `hello_macro_derive`:

<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>

```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.14.4"
quote = "0.6.3"
```

To start defining the procedural macro, place the code in Listing 19-39 into
your *src/lib.rs* file for the `hello_macro_derive` crate. Note that this code
won’t compile until we add a definition for the `impl_hello_macro` function.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

<!--
This usage of `extern crate` is required for the moment with 1.31.0, see:
https://github.com/rust-lang/rust/issues/54418
https://github.com/rust-lang/rust/pull/54658
https://github.com/rust-lang/rust/issues/55599
-->

```rust,ignore
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
```

<span class="caption">Listing 19-39: Code that most procedural macro crates
will need to have for processing Rust code</span>

Notice the way we’ve split the functions in Listing 19-39; this will be the
same for almost every procedural macro crate you see or create, because it
makes writing a procedural macro more convenient. What you choose to do in the
place where the `impl_hello_macro` function is called will be different
depending on your procedural macro’s purpose.

We’ve introduced three new crates: `proc_macro`, [`syn`], and [`quote`]. The
`proc_macro` crate comes with Rust, so we didn’t need to add that to the
dependencies in *Cargo.toml*. The `proc_macro` crate is the compiler’s API to
be able to read and manipulate Rust code from our code. The `syn` crate
parses Rust code from a string into a data structure that we can perform
operations on. The `quote` crate takes `syn` data structures and turns them
back into Rust code. These crates make it much simpler to parse any sort of
Rust code we might want to handle: writing a full parser for Rust code is no
simple task.

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

The `hello_macro_derive` function will get called when a user of our library
specifies `#[derive(HelloMacro)]` on a type. The reason is that we’ve annotated
the `hello_macro_derive` function here with `proc_macro_derive` and specified
the name, `HelloMacro`, which matches our trait name; that’s the convention
most procedural macros follow.

This function first converts the `input` from a `TokenStream` to a data
structure that we can then interpret and perform operations on. This is where
`syn` comes into play. The `parse` function in `syn` takes a `TokenStream` and
returns a `DeriveInput` struct representing the parsed Rust code. Listing 19-40
shows the relevant parts of the `DeriveInput` struct we get from parsing the
string `struct Pancakes;`:

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<span class="caption">Listing 19-40: The `DeriveInput` instance we get when
parsing the code that has the macro’s attribute in Listing 19-38</span>

The fields of this struct show that the Rust code we’ve parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
documentation for `DeriveInput`][syn-docs] for more information.

[syn-docs]: https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html

At this point, we haven’t defined the `impl_hello_macro` function, which is
where we’ll build the new Rust code we want to include. But before we do, note
that its output is also a `TokenStream`. The returned `TokenStream` is added to
the code that our crate users write, so when they compile their crate, they’ll
get extra functionality that we provide.

You might have noticed that we’re calling `unwrap` to panic if the call to the
`syn::parse` function fails here. Panicking on errors is necessary in
procedural macro code because `proc_macro_derive` functions must return
`TokenStream` rather than `Result` to conform to the procedural macro API.
We’ve chosen to simplify this example by using `unwrap`; in production code,
you should provide more specific error messages about what went wrong by using
`panic!` or `expect`.

Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `DeriveInput` instance, let’s generate the code that implements the
`HelloMacro` trait on the annotated type as shown in Listing 19-41.

<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>

```rust,ignore
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

<span class="caption">Listing 19-41: Implementing the `HelloMacro` trait using
the parsed Rust code</span>

We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. The struct in Listing 19-40 shows that the
`ident` we get when the `impl_hello_macro` function is run on the code in
Listing 19-38 will have the `ident` field with a value of `"Pancakes"`. Thus,
the `name` variable in Listing 19-41 will contain an `Ident` struct instance
that, when printed, will be the string `"Pancakes"`, the name of the struct in
Listing 19-38.

The `quote!` macro lets us write the Rust code that we want to return. The
direct result of the `quote!` macro’s execution isn’t what’s expected by the
compiler and needs to be converted to a `TokenStream`. We do this by calling
the `into` method, which consumes this intermediate representation and returns
a value of the required `TokenStream` type.

The `quote!` macro also provides some very cool templating mechanics; we can
write `#name`, and `quote!` will replace it with the value in the variable
named `name`. You can even do some repetition similar to the way regular macros
work. Check out [the `quote` crate’s docs][quote-docs] for a thorough
introduction.

[quote-docs]: https://docs.rs/quote

We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has one function, `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.

The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different from `format!` or
`println!`, which evaluate the expression and then turn the result into a
`String`. There is a possibility that the `#name` input might be an expression
to print literally, so we use `stringify!`. Using `stringify!` also saves an
allocation by converting `#name` to a string literal at compile time.

At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let’s hook up these crates to the code in Listing
19-38 to see the procedural macro in action! Create a new binary project in
your *projects* directory using `cargo new pancakes`. We need to add
`hello_macro` and `hello_macro_derive` as dependencies in the `pancakes`
crate’s *Cargo.toml*. If you’re publishing your versions of `hello_macro` and
`hello_macro_derive` to *https://crates.io/*, they would be regular
dependencies; if not, you can specify them as `path` dependencies as follows:

```toml
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

Put the code from Listing 19-38 into *src/main.rs*, and run `cargo run`: it
should print `Hello, Macro! My name is Pancakes!` The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` added the
trait implementation.

Next, let’s explore how the other kinds of procedural macros differ from custom
derive macros.

### Attribute-like macros

Attribute-like macros are similar to custom derive macros, but instead of
generating code for the `derive` attribute, they allow you to create new
attributes. They’re also more flexible; `derive` only works for structs and
enums; attributes can go on other items as well, like functions. As an example
of using an attribute-like macro, you might have an attribute named `route`
that annotates functions when using a web application framework:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

This `#[route]` attribute would be defined by the framework itself as a
procedural macro. The macro definition function’s signature would look like
this:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Here, we have two parameters of type `TokenStream`; the first is for the
contents of the attribute itself, that is, the `GET, "/"` part. The second is
the body of the item the attribute is attached to, in this case, `fn index()
{}` and the rest of the function’s body.

Other than that, attribute-like macros work the same way as custom derive
macros: create a crate with the `proc-macro` crate type and implement a
function that generates the code you want!

### Function-like macros

Finally, function-like macros define macros that look like function calls. For
example, an `sql!` macro that might be called like so:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside of it and check that it’s
syntactically correct. This macro would be defined like this:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

This is similar to the custom derive macro’s signature: we get in the tokens
that are inside of the parentheses, and return the code we wanted to generate.

## Summary

Whew! Now you have some features of Rust in your toolbox that you won’t use
often, but you’ll know they’re available in very particular circumstances.
We’ve introduced several complex topics so that when you encounter them in
error message suggestions or in other peoples’ code, you’ll be able to
recognize these concepts and syntax. Use this chapter as a reference to guide
you to solutions.

Next, we’ll put everything we’ve discussed throughout the book into practice
and do one more project!
