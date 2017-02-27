# Advanced Lifetimes

Back in Chapter 10, we learned how you can help Rust understand your references
with the 'lifetime' syntax. As a quick recap, most of the time, Rust will let
you elide lifetimes, but every reference has one. If you need to be explicit,
they look like this:

```rust
fn explicit_lifetime<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
# a
# }
```

There are three more features of lifetimes that we haven't learned yet, though:
*lifetime subtyping*, *trait object lifetimes*, and *higher ranked trait
bounds*.

## Lifetime subtyping

Imagine that we want to write a parser. To do this, we'll have a structure
with the string that we're parsing, a 'context'. We'll write individual parsers
that parse this string, and return success or failure. The parsers will need to
borrow the context to do the parsing. We'd end up with something like the
following. We've left off the lifetime anntations for now; this code won't
compile:

```rust,ignore
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        // do the parsing
    }
}
```

For simplicity's sake, our `parse` function returns a `Result<(), &str>`, that
is, we don't do anything on success, and the failure is the part of our string
that didn't parse correctly. A real implementation would have more error
information than that, and would actually do something on success, but we're
since this isn't relevant to our example, we're leaving that stuff off.

Okay, so, how do we fill in the lifetimes? The most straightforward thing to do
is to use the same lifetime everywhere:

```rust,ignore
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}
```

As is, this compiles.  Let's implement our `parse` method now.  Let's say that
we always produce an error, and the error happened after the first character.
Like this:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        // a real implementation would do a lot more, of course...
        Err(&self.context.0[1..])
    }
}
```

So far, so good. Next, let's write a function that takes a context, and then
uses a `Parser` to parse that context. This won't quite work...

```rust,ignore
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        // a real implementation would do a lot more, of course...
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

We get quite the error message:

```text
error: borrowed value does not live long enough
  --> <anon>:16:5
   |
16 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
17 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^ starting here...
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^ ...ending here

error: `context` does not live long enough
  --> <anon>:16:24
   |
16 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
17 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^ starting here...
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^ ...ending here
```

Let's break this error down:

```text
error: borrowed value does not live long enough
  --> <anon>:16:5
   |
16 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
17 | }
   | - temporary value only lives until here
   |
```

Fundamentally, the issue is that our `Parser` is temporary, and it needs to
live for longer than that. But why? We use it to calculate the result, but
there's no other reason for it to stick around.

For that, we need to look at the next part of the message:

```text
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^ starting here...
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^ ...ending here
```

Ah! So, Rust expects that it needs to live for the entire function, but it
doesn't; it only lives for this one line. Why? Let's keep looking at the
message.

```text
error: `context` does not live long enough
  --> <anon>:16:24
   |
16 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
17 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^ starting here...
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^ ...ending here
```

This is the same thing, but for `context` rather than for `Parser`. Rust
expects them to live longer... let's look at their definitions again:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}
```

Ah, right. We said `&'a Context<'a>`, that is, the `Context` has a lifetime
that's the same as the reference to it. That's fine, but...

```rust,ignore
    fn parse(&self) -> Result<(), &str> {
```

Remember the elision rules? This is the same as

```rust,ignore
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

That is, the error part of `parse`'s return value is tied to the parser. That
makes sense, as it's a pointer to the `Context that it holds. So that's the
problem, in `parse_context`, we return this result from `parse`, which is tied
to the lifetime of the `Parser`. But the `Parser` won't live past the end of
the function; it's temporary. Hence the lifetime issue.

However, this is safe: we know that the only reason that the result is tied to
the `Parser` is because it's referencing the `Parser`'s `Context`, so it's
_really_ the `Context` that we care about. We need a way to tell Rust that the
`Context` and the `Parser may have different lifetimes.

We could try that like this, but it doesn't quite work:

```rust,ignore
struct Context<'a>(&'a str);

struct Parser<'a, 'b> {
    context: &'a Context<'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&self) -> Result<(), &str> {
        // a real implementation would do a lot more, of course...
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

Here's the error:

```text
error[E0491]: in type `&'a main::Context<'b>`, reference has a longer lifetime than the data it references
 --> <anon>:5:5
  |
5 |     context: &'a Context<'b>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'a as defined on the struct at 4:0
 --> <anon>:4:1
  |
4 |   struct Parser<'a, 'b> {
  |  _^ starting here...
5 | |     context: &'a Context<'b>,
6 | | }
  | |_^ ...ending here
note: but the referenced data is only valid for the lifetime 'b as defined on the struct at 4:0
 --> <anon>:4:1
  |
4 |   struct Parser<'a, 'b> {
  |  _^ starting here...
5 | |     context: &'a Context<'b>,
6 | | }
  | |_^ ...ending here
help: consider using an explicit lifetime parameter as shown: fn main()
 --> <anon>:1:1
  |
1 | fn main() {
  | ^
```

Rust doesn't know of any relationship between `'b` and `'a`, so now that we've
said `&'a Context<'b>`, `'b` needs to _outlive_ `'a`, or else, we'd be pointing
to invalid state.

This is the feature we're talking about in this section. That was a very
long-winded example, but like we said at the start of this chapter, the tools
here are fairly niche. :) We need to be able to say "hey Rust: `'b` will live
at least as long as `'a`." And we have some simple syntax for that: `'b: 'a`.

If we add that to our definition for `Parser`...

```rust
struct Context<'a>(&'a str);

struct Parser<'a, 'b: 'a> {
    context: &'a Context<'b>,
}
```

Now, the `Parser`'s `Context` and the reference to it have different
lifetimes, and we've ensured that it's longer than the reference to it.

We also need to adjust the `impl` block to take both lifetimes...

```rust,ignore
impl<'a, 'b> Parser<'a, 'b> {
```

... and then, the signature of `parse` needs to make use of `'b`, to show that
the result comes from the `Context`:

```rust,ignore
    fn parse(&self) -> Result<(), &'b str> {
```

After those minor changes, it will work! Here's the full code:


```rust
struct Context<'a>(&'a str);

struct Parser<'a, 'b: 'a> {
    context: &'a Context<'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&self) -> Result<(), &'b str> {
        // a real implementation would do a lot more, of course...
        Err(&self.context.0[1..])
    }
}

fn parse_context<'a>(context: Context<'a>) -> Result<(), &'a str> {
    Parser { context: &context }.parse()
}
```

As a recap: `'b: 'a` says that "the lifetime b will live at least as long as
the lifetime a." You don't often need this syntax, but it can come up in
situations like this one, where you need to refer to something you have a
reference to that also has lifetimes.

## Lifetime bounds

We've used traits to bound generic types before, but you can also use lifetimes
for those bounds. For example, let's say we wanted to make a wrapper over
references. Using no bounds at all gives an error:

```rust,ignore
struct Ref<T>(&T);
```

Like this:

```text
error[E0309]: the parameter type `T` may not live long enough
 --> <anon>:2:19
  |
2 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> <anon>:2:19
  |
2 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

Rust helpfully gave us good advice:

> consider adding an explicit lifetime bound `T: 'a` so that the reference type
> `&'a T` does not outlive the data it points to.

This works:

```rust
struct Ref<'a, T: 'a>(&'a T);
```

The `T: 'a` syntax says "T can be any type, but if it contains any references,
it must live as long as `'a`."

We could sort of do the reverse with `'static`:

```rust
struct StaticRef<T: 'static>(&'static T);
```

This says "If `T` contains any references, they must be `'static` ones.

Types with no references inside count as `'static`, and since `'static` is
longer than any other lifetime, a type like `T: 'a` can be a type with no
references.

## Lifetimes in trait objects

In chapter 17, we learned about trait objects, like this:

```rust
trait Foo { }

impl Foo for i32 { }

let obj = Box::new(5) as Box<Foo>;
```

However, what if the type implementing our trait has a lifetime?

```rust
trait Foo { }

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> { }

let num = 5;

let obj = Box::new(Bar { x: &num }) as Box<Foo>;
```

This code works. But how? We haven't said anything about the liftimes of the
object.

Well, as it turns out, there are rules. For a trait object like `Box<Foo>`,
we can add a lifetime bound as well, like `Box<Foo + 'a>`, for example. Just as
with the other bounds, this means "Any implementor of `Foo` which has a
lifetime inside must be `'a`." But we didn't need to explicitly write this.
Here are the rules:

* The default begins as 'static.
* If you have `&'a X` or `&'a mut X`, then the default is `'a`.
* If you have a single `T: 'a` clasues, then the default is `'a`.
* If you have multiple `T: 'a`-like clauses, then there is no default; you must
  be explicit.

If you need to be explicit, `Box<Foo + 'a>` or `Box<Foo + 'static>` is the way
to do it.

## Higher ranked trait bounds

Sometimes, you may write a function which accepts a closure, and that closure
takes a reference as an argument:

```rust
fn call_with_ref<F>(some_closure:F) -> i32
    where F: Fn(&i32) -> i32 {

    let value = 0;

    some_closure(&value)
}
```

This code compiles just fine, but what about the lifetime here? With the
elision rules, we don't actually *need* to write out the lifetime, but what if
we did?

You might think that you'd write it something like this:

```rust
fn call_with_ref<'a, F>(some_closure:F) -> i32
    where F: Fn(&'a i32) -> i32 {
# 
#     let value = 0;
# 
#     some_closure(&value)
# }
```

This will compile, but it's actually taking advantage of one last bit of syntax
sugar. Because our trait is generic, yet it also *contains* a generic lifetime,
we need a way to say that our generic is generic. In general, these kinds of
"generic of generic" issues are referred to with the words "higher", like
"higher kinded type." In this case, it's a "higher rank type." What that means
isn't important, but the implication is that Rust is doing something special
here for us.

If we wanted to write it out entirely, we'd use this syntax, with `for<>`:

```rust
fn call_with_ref<F>(some_closure:F) -> i32
    where F: for<'a> Fn(&'a i32) -> i32 {
# 
#     let value = 0;
# 
#     some_closure(&value)
# }
```

This says "for any lifetime `'a`." Think of it as similar to how a generic
function says "for any type `T`."

This comes up extremely rarely in Rust code. It's an explicit goal of one of
the members of the language design team that you should never need to write an
explicit `for<'a>`, but you can if you'd like to.
