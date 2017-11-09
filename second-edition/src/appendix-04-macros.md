# D - Macros

Way back in Chapter 1, when talking about "Hello World", we said this:

> The second important part is println!. This is calling a Rust macro, which
> is how metaprogramming is done in Rust. If it were calling a function
> instead, it would look like this: println (without the !). We’ll discuss Rust
> macros in more detail in Appendix E, but for now you just need to know that
> when you see a ! that means that you’re calling a macro instead of a normal
> function.

Finally, at the very end of the book, in this appendix, we'll actually explain
what's going on here.

Fundamentally, macros are a way of writing code that writes other code. In
the previous appendix, we discussed the `derive` attribute, which generates
an implementation of various traits for you. We've also used the `println!`
and `vec!` macros. All of these macros *expand* to produce more code than
what you've written in your source code.

Before we dive into more detail on macros, we're going to go over some
history, to give you context for macros in Rust.

## Macro history

Why is this section an appendix rather than a chapter in the book? We're at
sort of a transitional period in Rust's history with regards to macros, and so
we want to explain *some* of what's going on here, but also, it will be different
in the future, and so we don't want to explain too much, either.

As Rust developed, two major forms of macros evolved. One was called “macros”
and the other was called “compiler plugins.” Both of these systems were
pretty good, but both of them also had a lot of problems. We’ll get into the
details of each of these kinds of macros below, but there’s one thing we
should talk about first. In the final period before Rust 1.0, we had to make
a tough decision: what do we do about macros? We know that the system has
these flaws, but fixing them was going to take a long time. Were we willing
to delay the release of Rust for what was possibly going to be years, to wait
on a redesign of the macro system? Could we simply remove macros, even though
"Hello world" uses macros?

In the end, we decided that we would stablize macros, but not compiler plugins.
Additionally, we'd change the keyword to declare them from `macro` to `macro_rules`, a slightly more awkward name, with the intent of using the
`macro` keyword for an improved macro system later. Even though macros have
flaws, they're too useful to leave out of Rust.

Designing a language is hard.

So, as far as Rust's macros goes today, there are two kinds of macros:
declarative macros and procedural macros. Declarative macros are macros
like `vec!`, and procedural macros are like `derive`. In this appendex,
we'll cover the state of both of these macro systems today, and then
discuss where we're planning on taking Rust's macros in the future.

## Declarative Macros with `macro_rules`

The first form of Macros in Rust, and the one that's most widely used, are
called "declarative macros," sometimes "macros by example," sometimes
"macro_rules macros," or sometimes just plain "macros." At their core,
declarative macros allow you to write something similar to a Rust `match`
statement:

```rust
match x {
    4 => println!("four!"),
    5 => println!("five!"),
    _ => println!("something else"),
}
```

With `match`, `x`'s structure and value will be evaluated, and the right arm will execute based on what matches. So if `x` is five, the second arm happens. Etc.
We discussed `match` in Chapter 6, section 2.

These kinds of macros work in the same way: you set up some sort of pattern, and then, if that pattern matches, some code is generated. One important difference here is that in `match`, `x` gets evaluated. With macros, `x` does not get evaluated.

To define a macro, you use something similar to `match`, called `macro_rules`.
Earlier in the book, we used the `vec!` macro to create vectors. It looks like
this:

```rust
let x: Vec<u32> = vec![1, 2, 3];
```

This macro creates a new vector with three elements inside. Here's what the
macro could look like, written with `macro_rules!`:

```rust
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

Whew! That's a bunch of stuff. The most important line is here:

```rust
    ( $( $x:expr ),* ) => {
```

This `pattern => block` looks similar to the `match` statement above. If this pattern matches, then the block of code will be emitted. Given that this is the only pattern in this macro, there's only one valid way to match; any other will be an error. More complex macros will have more than one rule.

The `$x:expr` part matches an expression, and gives it the name `$x`. The `$(),*` part matches zero or more of these expressions. Then, in the body of the macro, the `$()*` part is generated for each part that matches, and the `$x` within is replaced with each expression that was matched.

These macros are fine, but there's a number of bugs and rough edges. For example, there's no namespacing: if a macro exists, it's everywhere. In order to prevent name clashes, this means that you have to explicitly import the macros when using a crate:

```rust
#[macro_use]
extern crate serde;
```

Otherwise, you couldn't import two crates that had the same macro name. In
practice this conflict doesn't come up much, but the more crates you use, the
more likely it is. The hygiene of `macro_rules` is there, but not perfect.
(Only local variables and labels are hygienic...) Et cetera.

Given that most Rust programmers will *use* macros more than *write* macros,
that's all we'll discuss about `macro_rules` in this book. To learn more
about how to write macros, consult the online documentation, or other
resources such as [The Little Book of Rust
Macros](https://danielkeep.github.io/tlborm/book/index.html).

## Procedural Macros for custom `derive`

In opposition to the pattern-based declarative macros, the second form are
called "procedural macros" because they're functions: they accept some Rust
code as an input, and produce some Rust code as an output. We say "code" but
we don't mean that literally. Today, the only thing you can use procedural
macros for is to allow your traits to be `derive`d. Let's build an example
together.

The first thing we need to do is start a new crate for our project:

```bash
$ cargo new --bin hello-world
```

All we want is to be able to call `hello_world()` on a derived type. Something
like this:

```rust,ignore
#[derive(HelloWorld)]
struct Pancakes;

fn main() {
    Pancakes::hello_world();
}
```

With some kind of nice output, like `Hello, World! My name is Pancakes.`.

Let's go ahead and write up what we think our macro will look like from a
user perspective. In `src/main.rs` we write:

```rust,ignore
#[macro_use]
extern crate hello_world_derive;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
struct FrenchToast;

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}
```

Great. So now we just need to actually write the procedural macro. At the
moment, procedural macros need to be in their own crate. Eventually, this
restriction may be lifted, but for now, it's required. As such, there's a
convention; for a crate named `foo`, a custom derive procedural macro is
called `foo-derive`. Let's start a new crate called `hello-world-derive`
inside our `hello-world` project.

```bash
$ cargo new hello-world-derive
```

To make sure that our `hello-world` crate is able to find this new crate
we've created, we'll add it to our toml:

```toml
[dependencies]
hello-world-derive = { path = "hello-world-derive" }
```

As for the source of our `hello-world-derive` crate, here's an example:

```rust,ignore
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
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

So there is a lot going on here. We have introduced two new crates: [`syn`]
and [`quote`]. As you may have noticed, `input: TokenSteam` is immediately
converted to a `String`. This `String` is a string representation of the Rust
code for which we are deriving `HelloWorld`. At the moment, the only thing
you can do with a `TokenStream` is convert it to a string. A richer API will
exist in the future.

So what we really need is to be able to _parse_ Rust code into something
usable. This is where `syn` comes to play. `syn` is a crate for parsing Rust
code. The other crate we've introduced is `quote`. It's essentially the dual
of `syn` as it will make generating Rust code really easy. We could write
this stuff on our own, but it's much simpler to use these libraries. Writing
a full parser for Rust code is no simple task.

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

The comments seem to give us a pretty good idea of our overall strategy. We
are going to take a `String` of the Rust code for the type we are deriving,
parse it using `syn`, construct the implementation of `hello_world` (using
`quote`), then pass it back to Rust compiler.

One last note: you'll see some `unwrap()`s there. If you want to provide an
error for a procedural macro, then you should `panic!` with the error
message. We'll talk more about this later, but in this case, we're keeping it
as simple as possible.

Great, so let's write `impl_hello_world(&ast)`.

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

So this is where quotes comes in. The `ast` argument is a struct that gives
us a representation of our type (which can be either a `struct` or an
`enum`). Check out the
[docs](https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html), there is
some useful information there. We are able to get the name of the type using
`ast.ident`. The `quote!` macro lets us write up the Rust code that we wish
to return and convert it into `Tokens`. `quote!` lets us use some really cool
templating mechanics; we simply write `#name` and `quote!` will replace it
with the variable named `name`. You can even do some repetition similar to
regular macros work. You should check out the [docs](https://docs.rs/quote)
for a good introduction.

So I think that's it. Oh, well, we do need to add dependencies for `syn` and
`quote` in the `cargo.toml` for `hello-world-derive`.

```toml
[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

That should be it. Let's try to compile `hello-world`.

```bash
error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
 --> hello-world-derive/src/lib.rs:8:3
  |
8 | #[proc_macro_derive(HelloWorld)]
  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Oh, so it appears that we need to declare that our `hello-world-derive` crate is
a `proc-macro` crate type. How do we do this? Like this:

```toml
[lib]
proc-macro = true
```

Ok so now, let's compile `hello-world`. Executing `cargo run` now yields:

```bash
Hello, World! My name is FrenchToast
Hello, World! My name is Waffles
```

We've done it!

### Custom Attributes

In some cases it might make sense to allow users some kind of configuration.
For example, the user might want to overwrite the name that is printed in the `hello_world()` method.

This can be achieved with custom attributes:

```rust,ignore
#[derive(HelloWorld)]
#[HelloWorldName = "the best Pancakes"]
struct Pancakes;

fn main() {
    Pancakes::hello_world();
}
```

If we try to compile this though, the compiler will respond with an error:

```bash
error: The attribute `HelloWorldName` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
```

The compiler needs to know that we're handling this attribute and to not
respond with an error. This is done in the `hello-world-derive` crate by
adding `attributes` to the `proc_macro_derive` attribute:

```rust,ignore
#[proc_macro_derive(HelloWorld, attributes(HelloWorldName))]
pub fn hello_world(input: TokenStream) -> TokenStream 
```

Multiple attributes can be specified that way.

### Raising Errors

Let's assume that we do not want to accept enums as input to our custom
derive method.

This condition can be easily checked with the help of `syn`. But how do we
tell the user, that we do not accept enums? The idiomatic way to report
errors in procedural macros is to panic:

```rust,ignore
fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    // Check if derive(HelloWorld) was specified for a struct
    if let syn::Body::Struct(_) = ast.body {
        // Yes, this is a struct
        quote! {
            impl HelloWorld for #name {
                fn hello_world() {
                    println!("Hello, World! My name is {}", stringify!(#name));
                }
            }
        }
    } else {
        //Nope. This is an Enum. We cannot handle these!
       panic!("#[derive(HelloWorld)] is only defined for structs, not for enums!");
    }
}
```

If a user now tries to derive `HelloWorld` from an enum they will be greeted
with following, hopefully helpful, error:

```bash
error: custom derive attribute panicked
  --> src/main.rs
   |
   | #[derive(HelloWorld)]
   |          ^^^^^^^^^^
   |
   = help: message: #[derive(HelloWorld)] is only defined for structs, not for enums!
```

## Macros future

In the future, we'll be expanding both kinds of macros. A better declarative
macro system will be used with the `macro` keyword, and we'll add more types
of procedural macros, for more powerful tasks than only `derive`. As these
systems are still under development, that's all we can say about them at
this time.