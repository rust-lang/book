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
fn some_function<'a>(parameter: &'a i32) {
```

Notice anything? In the same way that generic type declarations go inside angle
brackets after the function name, lifetime declarations also go inside those
same angle brackets. We can even write functions that take both a lifetime
declaration and a generic type declaration:

```rust,ignore
fn some_function<'a, T>(parameter: &'a T) {
```

This function takes one parameter, a reference to some type, `T`, and the
reference has the lifetime `'a`. In the same way that we parameterize functions
that take generic types, we parameterize references with lifetimes.

So, that's the syntax, but *why*? What does a lifetime do, anyway?

### Lifetimes Prevent Dangling References

Consider the program in listing 10-8. There's an outer scope and an inner
scope. The outer scope declares a variable named `r` with no initial value, and
the inner scope declares a variable named `x` with the initial value of 5.
Inside the inner scope, we attempt to set the value of `r` to a reference to
`x`. Then the inner scope ends and we attempt to print out the value in `r`:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 10-8: An attempt to use a reference whose value has gone out of scope

</figcaption>
</figure>

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
program because the lifetime `'b` is shorter than the lifetime of `'a`—the
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
for a function or method when one of the parameters or return values is a
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
function, we can't know beforehand exactly all of the values that it could be
called with and how long they will be valid for. We have to explain to Rust
what we expect the lifetime of the parameter to be (we'll learn about how to
know what you expect the lifetime to be in a bit). This is similar to writing a
function that has a parameter of a generic type: we don't know what type the
values will actually end up being when the function gets called. Lifetimes are
the same idea, but they are generic over the scope of a reference, rather than
a type.


### Lifetime Annotations in Function Signatures

Lifetime annotations for functions go on the function signature, but we don't
have to annotate any of the code in the function body with lifetimes. That's
because Rust can analyze the specific code inside the function without any
help. When a function interacts with references that come from or go to code
outside that function, however, the lifetimes of those parameters or return
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

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
# fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
#     if x.len() > y.len() {
#         x
#     } else {
#         y
#     }
# }
#
fn main() {
    let a = String::from("abcd");
    let b = "xyz";

    let c = longest(a.as_str(), b);
    println!("The longest string is {}", c);
}
```

<figcaption>

Listing 10-9: A `main` function that demonstrates how we'd like to use the
`longest` function

</figcaption>
</figure>

Note that we want the function to take string slices because we don't want the
`longest` function to take ownership of its parameters, and we want the function
to be able to accept slices of a `String` (like `a` is) as well as string
literals (`b`). Refer back to the "String Slices as Parameters" section of
Chapter 4 for more discussion about why these are the parameters we want.

Here's the start of an implementation of the `longest` function that won't
compile yet:

<span class="filename">Filename: src/main.rs</span>

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

<span class="filename">Filename: src/main.rs</span>

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
named `'a`, it will have two parameters, both which are string slices that live
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
instead always returned the first parameter, we wouldn't need to specify a
lifetime on `y`. This code compiles:

<span class="filename">Filename: src/main.rs</span>

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

The lifetime parameter for the return type needs to be specified and needs to
match one of the value parameters' lifetime parameters. If the reference
returned does *not* refer to one of the parameters, the only other possibility
is that it refers to a value created within this function, and that would be a
dangling reference since the value will go out of scope at the end of the
function. Consider this attempted implementation of `longest`:

<span class="filename">Filename: src/main.rs</span>

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
parameters and return values of functions. Once they're connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.

### Lifetime Elision

If every reference has a lifetime, and we need to provide them for functions
that use references as parameters or return values, then why did this function
from the "String Slices" section of Chapter 4 compile? We haven't annotated any
lifetimes here, yet Rust happily compiles this function:

<span class="filename">Filename: src/lib.rs</span>

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

Lifetimes on function parameters are called *input lifetimes*, and lifetimes on
return values are called *output lifetimes*. There's one rule related to how
Rust infers input lifetimes in the absence of explicit annotations:

1. Each function parameter that is a reference and therefore needs a lifetime
  parameter gets its own. In other words, a function with one parameter gets one
  lifetime parameter: `fn foo<'a>(x: &'a i32)`, a function with two parameters
  gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b
  i32)`, and so on.

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
method has a reference as a parameter and is returning a reference; the
lifetime of the return value will be the lifetime of `self`:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Config {}
#
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
chapters. Let's move on to testing.
