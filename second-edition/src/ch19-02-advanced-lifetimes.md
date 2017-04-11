## Advanced Lifetimes

Back in Chapter 10, we learned how to annotate references with lifetime
parameters to help Rust understand how the lifetimes of different references
relate. We saw how most of the time, Rust will let you elide lifetimes, but
every reference has a lifetime. There are three advanced features of lifetimes
that we haven't covered though: *lifetime subtyping*, *trait object lifetimes*,
and *higher ranked trait bounds*.

### Lifetime subtyping

Imagine that we want to write a parser. To do this, we'll have a structure that
holds a reference to the string that we're parsing, and we'll call that struct
`Context`. We'll write a parser that will parse this string and return success
or failure. The parser will need to borrow the context to do the parsing.
Implementing this would look like the code in Listing 19-12, which won't
compile because we've left off the lifetime annotations for now:

```rust,ignore
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<span class="caption">Listing 19-12: Defining a `Context` struct that holds a
string slice, a `Parser` struct that holds a reference to a `Context` instance,
and a `parse` method that always returns an error referencing the string
slice</span>

For simplicity's sake, our `parse` function returns a `Result<(), &str>`. That
is, we don't do anything on success, and on failure we return the part of the
string slice that didn't parse correctly. A real implementation would have more
error information than that, and would actually return something created when
parsing succeeds, but we're leaving those parts of the implementation off since
they aren't relevant to the lifetimes part of this example. We're also defining
`parse` to always produce an error after the first byte. Note that this may
panic if the first byte is not on a valid character boundary; again, we're
simplifying the example in order to concentrate on the lifetimes involved.

So how do we fill in the lifetime parameters for the string slice in `Context`
and the reference to the `Context` in `Parser`? The most straightforward thing
to do is to use the same lifetime everywhere, as shown in Listing 19-13:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<span class="caption">Listing 19-13: Annotating all references in `Context` and
`Parser` with the same lifetime parameter</span>

This compiles fine. Next, in Listing 19-14, let's write a function that takes
an instance of `Context`, uses a `Parser` to parse that context, and returns
what `parse` returns. This won't quite work:

```rust,ignore
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<span class="caption">Listing 19-14: An attempt to add a `parse_context`
function that takes a `Context` and uses a `Parser`</span>

We get two quite verbose errors when we try to compile the code with the
addition of the `parse_context` function:

```text
error: borrowed value does not live long enough
  --> <anon>:16:5
   |
16 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
17 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the
body at 15:55...
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
note: borrowed value must be valid for the anonymous lifetime #1 defined on the
body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^ starting here...
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^ ...ending here
```

These errors are saying that both the `Parser` instance we're creating and the
`context` parameter live from the line that the `Parser` is created until the
end of the `parse_context` function, but they both need to live for the entire
lifetime of the function.

In other words, `Parser` and `context` need to *outlive* the entire function
and be valid before the function starts as well as after it ends in order for
all the references in this code to always be valid. Both the `Parser` we're
creating and the `context` parameter go out of scope at the end of the
function, though (since `parse_context` takes ownership of `context`).

Let's look at the definitions in Listing 19-13 again, especially the signature
of the `parse` method:

```rust,ignore
    fn parse(&self) -> Result<(), &str> {
```

Remember the elision rules? If we annotate the lifetimes of the references, the
signature would be:

```rust,ignore
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

That is, the error part of the return value of `parse` has a lifetime that is
tied to the `Parser` instance's lifetime (that of `&self` in the `parse` method
signature). That makes sense, as the returned string slice references the
string slice in the `Context` instance that the `Parser` holds, and we've
specified in the definition of the `Parser` struct that the lifetime of the
reference to `Context` that `Parser` holds and the lifetime of the string slice
that `Context` holds should be the same.

The problem is that the `parse_context` function returns the value returned
from `parse`, so the lifetime of the return value of `parse_context` is tied to
the lifetime of the `Parser` as well. But the `Parser` instance created in the
`parse_context` function won't live past the end of the function (it's
temporary), and the `context` will go out of scope at the end of the function
(`parse_context` takes ownership of it).

We're not allowed to return a reference to a value that goes out of scope at
the end of the function. Rust thinks that's what we're trying to do because we
annotated all the lifetimes with the same lifetime parameter. That told Rust
the lifetime of the string slice that `Context` holds is the same as that of
the lifetime of the reference to `Context` that `Parser` holds.

The `parse_context` function can't see that within the `parse` function, the
string slice returned will outlive both `Context` and `Parser`, and that the
reference `parse_context` returns refers to the string slice, not to `Context`
or `Parser`.

By knowing what the implementation of `parse` does, we know that the only
reason that the return value of `parse` is tied to the `Parser` is because it's
referencing the `Parser`'s `Context`, which is referencing the string slice, so
it's really the lifetime of the string slice that `parse_context` needs to care
about. We need a way to tell Rust that the string slice in `Context` and the
reference to the `Context` in `Parser` have different lifetimes and that the
return value of `parse_context` is tied to the lifetime of the string slice in
`Context`.

We could try only giving `Parser` and `Context` different lifetime parameters
as shown in Listing 19-15. We've chosen the lifetime parameter names `'s` and
`'c` here to be clearer about which lifetime goes with the string slice in
`Context` and which goes with the reference to `Context` in `Parser`. Note that
this won't completely fix the problem, but it's a start and we'll look at why
this isn't sufficient when we try to compile.

```rust,ignore
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<span class="caption">Listing 19-15: Specifying different lifetime parameters
for the references to the string slice and to `Context`</span>

We've annotated the lifetimes of the references in all the same places that we
annotated them in Listing 19-13, but used different parameters depending on
whether the reference goes with the string slice or with `Context`. We've also
added an annotation to the string slice part of the return value of `parse` to
indicate that it goes with the lifetime of the string slice in `Context`.

Here's the error we get now:

```text
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
 --> src/main.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 |   struct Parser<'c, 's> {
  |  _^ starting here...
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^ ...ending here
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 |   struct Parser<'c, 's> {
  |  _^ starting here...
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^ ...ending here
```

Rust doesn't know of any relationship between `'c` and `'s`. In order to be
valid, the referenced data in `Context` with lifetime `'s` needs to be
constrained to guarantee that it lives longer than the reference to `Context`
that has lifetime `'c`. If `'s` is not longer than `'c`, then the reference to
`Context` might not be valid.

Which gets us to the point of this section: Rust has a feature called *lifetime
subtyping*, which is a way to specify that one lifetime parameter lives at
least as long as another one. In the angle brackets where we declare lifetime
parameters, we can declare a lifetime `'a` as usual, and declare a lifetime
`'b` that lives at least as long as `'a` by declaring `'b` with the syntax `'b:
'a`.

In our definition of `Parser`, in order to say that `'s` (the lifetime of the
string slice) is guaranteed to live at least as long as `'c` (the lifetime of
the reference to `Context`), we change the lifetime declarations to look like
this:

```rust
# struct Context<'a>(&'a str);
#
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

Now, the reference to `Context` in the `Parser` and the reference to the string
slice in the `Context` have different lifetimes, and we've ensured that the
lifetime of the string slice is longer than the reference to the `Context`.

That was a very long-winded example, but as we mentioned at the start of this
chapter, these features are pretty niche. You won't often need this syntax, but
it can come up in situations like this one, where you need to refer to
something you have a reference to.

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

This code works. But how? We haven't said anything about the lifetimes of the
object.

Well, as it turns out, there are rules. For a trait object like `Box<Foo>`,
we can add a lifetime bound as well, like `Box<Foo + 'a>`, for example. Just as
with the other bounds, this means "Any implementer of `Foo` which has a
lifetime inside must be `'a`." But we didn't need to explicitly write this.
Here are the rules:

* The default begins as 'static.
* If you have `&'a X` or `&'a mut X`, then the default is `'a`.
* If you have a single `T: 'a` clause, then the default is `'a`.
* If you have multiple `T: 'a`-like clauses, then there is no default; you must
  be explicit.

If you need to be explicit, `Box<Foo + 'a>` or `Box<Foo + 'static>` is the way
to do it.

## Higher ranked trait bounds

Sometimes, you may write a function which accepts a closure, and that closure
takes a reference as an argument:

```rust
fn call_with_ref<F>(some_closure: F) -> i32
    where F: Fn(&i32) -> i32 {

    let value = 0;

    some_closure(&value)
}
```

This code compiles just fine, but what about the lifetime here? With the
elision rules, we don't actually *need* to write out the lifetime, but what if
we did?

You might think that you'd write it something like this:

```rust,ignore
fn call_with_ref<'a, F>(some_closure:F) -> i32
    where F: Fn(&'a i32) -> i32 {
#
#     let value = 0;
#
#     some_closure(&value)
# }
```

This will not compile. Because our trait is generic, yet it also *contains* a
generic lifetime, we need a way to say that our generic is generic. In general,
these kinds of "generic of generic" issues are referred to with the words
"higher", like "higher kinded type." In this case, it's a "higher rank type."
What that means isn't important, but the implication is that Rust is doing
something special here for us.

If we wanted to write it out entirely, we'd use this syntax, with `for<>`:

```rust
fn call_with_ref<F>(some_closure: F) -> i32
    where F: for<'a> Fn(&'a i32) -> i32 {
#
#     let value = 0;
#
#     some_closure(&value)
# }
```

failures:
    Advanced_Lifetimes_19

test result: FAILED. 11 passed; 1 failed; 9 ignored; 0 measured


This says "for any lifetime `'a`." Think of it as similar to how a generic
function says "for any type `T`."

This comes up extremely rarely in Rust code. It's an explicit goal of one of
the members of the language design team that you should never need to write an
explicit `for<'a>`, but you can if you'd like to.
