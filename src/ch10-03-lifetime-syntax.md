# Lifetime syntax

So far, we've talked about two ways in which Rust allows you to abstract over
different things: generic type parameters let us abstract over types, and
traits let us abstract over a collection of methods. There's one more way that
Rust allows us to do something similar: *lifetimes* allow us to be generic over
scopes of code.

Scopes of code? Yes, it's a bit unusual. Lifetimes are, in some ways, Rust's
most distinctive feature. They are a bit different than the tools you have
used in other programming languages. Lifetimes are a big topic, and so we're
not going to cover everything about them in this chapter. What we _are_ going
to do is talk about the very basics of lifetimes, so that when you see the
syntax in documentation or other places, you won't be totally lost. Chapter XX
will contain more advanced information about everything lifetimes can do.

## Core syntax

We've talked about references previously, but we left something important out.
As it turns out, every reference in Rust has a lifetime. Lifetimes have a
slightly unusual syntax:

```rust,ignore
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
```

The `'a` there is a *lifetime* with the name `a`. A single apostrophe indicates
that this name is for a lifetime. But where does that name come from? Here's
a function signature with lifetime annotations:

```rust,ignore
fn lifetime<'a>(argument: &'a i32) {
```

Notice anything? In the same way that generic type parameters go inside angle
brackets after the function name, lifetimes also go inside angle brackets. We
can even write functions that take both:

```rust,ignore
fn lifetime<'a, T>(argument: &'a T) {
```

This function takes one argument, and that argument is a reference to some
type, `T`, but with the lifetime `'a`. In the same way that we parameterize
functions that take generic types, we parameterize references with lifetimes.

So, that's the syntax, but _why_? What does a lifetime do, anyway?

## No dangling references!

Consider this program:

```rust,ignore
let r;

{
    let x = 5;
    r = &x;
}

println!("r: {}", r);
```

If we compile it, we get an error:

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
out of scope when we hit the closing curly brace. But `r` is outside of the
curly brace.  So it has a larger scope. If Rust allowed this code to work, `r`
would be referencing memory that we've deallocated. That'd be bad! Once it's
deallocated, it's meaningless.

So how does Rust determine that this code is bad? It compares scopes. Here's an
example with some annotations:

```rust,ignore
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
```

Here, we've annotated the lifetime of `r` with `'a`, and the lifetime of `x`
with `'b`. Rust looks at these lifetimes, sees that `r` has a lifetime of `'a`,
but refers to something with a lifetime of `'b`, and rejects the program, since
`'b` doesn't live as long as `'a` does.

What about an example that _does_ work?

```rust
let x = 5;            // -----+-- 'a
                      //      |
let r = &x;           // --+--+-- 'b
                      //   |  |
println!("r: {}", r); //   |  |
                      // --+  |
                      // -----+
```

Here, `x` lives for `'a`, which is larger than `'b`. So this is okay: we know
that `r` will always be valid, as it has a smaller scope than the thing it
refers to, `x`.

Note that we didn't have to name any lifetimes here; Rust figured it out for
us. We only name lifetimes when we accept a reference as an argument, either
for a function:

```rust,ignore
fn lifetime<'a, T>(argument: &'a T) {
```

Or in a struct:

```rust
struct Ref<'a> {
    x: &'a i32,
}
```

This is because lifetimes are a form of generics. In the examples above, `'a`
and `'b` were concrete lifetimes: we knew about `x` and `r`, and how long they
would live exactly. But when we write a function, we can't know exactly all of
the arguments that it would be called with; so we have to explain to Rust what
we'd expect the lifetime of the argument to be. This is similar to how when we
write a generic function, we don't know what type the argument would be, but
it's for the scope of a reference, rather than a type.

## Two lifetimes, intertwined

So what does a lifetime parameter do, anyway? Consider this example:

```rust,ignore
fn foo(x: &i32, y: &i32) -> &i32 {
```

Which reference is the return value connected to? If the function looks
like this:

```rust,ignore
fn foo(x: &i32, y: &i32) -> &i32 {
    x
}
```

Then it's the first, but if it's like this:

```rust,ignore
fn foo(x: &i32, y: &i32) -> &i32 {
    y
}
```

Then it's the second.

This is what lifetime parameters give us control over. If we say

```rust,ignore
fn foo<'a>(x: &'a i32, y: &i32) -> &'a i32 {
```

Then we know that the return value's lifetime is tied to the lifetime of `x`.
And likewise:

```rust,ignore
fn foo<'a>(x: &i32, y: &'a i32) -> &'a i32 {
```

This lets us know that it's tied to the value of `y`. Ultimately, this is what
lifetime syntax is about: connecting the lifetimes of various parameters and
return values of functions, so that Rust can understand how long your
references are supposed to live.

## Lifetime elision

If every reference has a lifetime, and we need to provide them for functions
that take references, then why does this function compile?

```rust
fn do_nothing(x: &i32) -> &i32 {
    x
}
```

We haven't annotated any lifetime here, yet Rust happily compiles this
function.

The answer here is historical: in ancient versions of pre-1.0 Rust, this
would not have compiled. Every reference needed an explicit lifetime. So
this would have to be written like this:

```rust
fn do_nothing<'a>(x: &'a i32) -> &'a i32 {
    x
}
```

However, after writing a lot of Rust code, some patterns developed. We noticed
that the vast majority of code followed these patterns. And being forced to use
explicit lifetime syntax on every reference wasn't a very great developer
experience.

To fix this, we developed the 'lifetime elision' rules. It's not full
inference: Rust doesn't try to guess what you meant, and then infer that. It's
a very basic set of patterns, and if they match, you don't need to write
the lifetimes explicitly. Here's the rules:

Lifetimes on function arguments are called "input lifetimes", and lifetimes on
return values are called "output lifetimes." With that in mind, there's one
rule related to input lifetimes:

* Each argument that needs a lifetime parameter gets its own. In other words,
  `fn foo<'a>(x: &'a i32)`, and `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, and
  so on.

And two related to output lifetimes:

* If there is exactly one input lifetime parameter, that lifetime is assigned
  to all output lifetime parameters.
* If there are multiple input lifetime parameters, but one of them is `&self`
  or `&mut self`, then the lifetime of `self` is the lifetime assigned to all
  output lifetime parameters.

If none of these things are true, then you must explicitly annotate lifetimes.

These rules sound a little bit technical, and they are. But we've found that
they cover the vast majority of cases, allowing you to write a lot of code
without needing to consider explicit lifetimes. But it's not 100% of the time,
and so you may see explicit lifetimes used in various places.

## The static lifetime
