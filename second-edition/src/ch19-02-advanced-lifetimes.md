## Advanced Lifetimes

Back in Chapter 10, we learned how to annotate references with lifetime
parameters to help Rust understand how the lifetimes of different references
relate. We saw how most of the time, Rust will let you elide lifetimes, but
every reference has a lifetime. There are three advanced features of lifetimes
that we haven’t covered though: *lifetime subtyping*, *lifetime bounds*, and
*trait object lifetimes*.

### Lifetime Subtyping

Imagine that we want to write a parser. To do this, we’ll have a structure that
holds a reference to the string that we’re parsing, and we’ll call that struct
`Context`. We’ll write a parser that will parse this string and return success
or failure. The parser will need to borrow the context to do the parsing.
Implementing this would look like the code in Listing 19-12, which won’t
compile because we’ve left off the lifetime annotations for now:

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

For simplicity’s sake, our `parse` function returns a `Result<(), &str>`. That
is, we don’t do anything on success, and on failure we return the part of the
string slice that didn’t parse correctly. A real implementation would have more
error information than that, and would actually return something created when
parsing succeeds, but we’re leaving those parts of the implementation off since
they aren’t relevant to the lifetimes part of this example. We’re also defining
`parse` to always produce an error after the first byte. Note that this may
panic if the first byte is not on a valid character boundary; again, we’re
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

This compiles fine. Next, in Listing 19-14, let’s write a function that takes
an instance of `Context`, uses a `Parser` to parse that context, and returns
what `parse` returns. This won’t quite work:

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
   |  ________________________________________________________^
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^

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
   |  ________________________________________________________^
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^
```

These errors are saying that both the `Parser` instance we’re creating and the
`context` parameter live from the line that the `Parser` is created until the
end of the `parse_context` function, but they both need to live for the entire
lifetime of the function.

In other words, `Parser` and `context` need to *outlive* the entire function
and be valid before the function starts as well as after it ends in order for
all the references in this code to always be valid. Both the `Parser` we’re
creating and the `context` parameter go out of scope at the end of the
function, though (since `parse_context` takes ownership of `context`).

Let’s look at the definitions in Listing 19-13 again, especially the signature
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
tied to the `Parser` instance’s lifetime (that of `&self` in the `parse` method
signature). That makes sense, as the returned string slice references the
string slice in the `Context` instance that the `Parser` holds, and we’ve
specified in the definition of the `Parser` struct that the lifetime of the
reference to `Context` that `Parser` holds and the lifetime of the string slice
that `Context` holds should be the same.

The problem is that the `parse_context` function returns the value returned
from `parse`, so the lifetime of the return value of `parse_context` is tied to
the lifetime of the `Parser` as well. But the `Parser` instance created in the
`parse_context` function won’t live past the end of the function (it’s
temporary), and the `context` will go out of scope at the end of the function
(`parse_context` takes ownership of it).

We’re not allowed to return a reference to a value that goes out of scope at
the end of the function. Rust thinks that’s what we’re trying to do because we
annotated all the lifetimes with the same lifetime parameter. That told Rust
the lifetime of the string slice that `Context` holds is the same as that of
the lifetime of the reference to `Context` that `Parser` holds.

The `parse_context` function can’t see that within the `parse` function, the
string slice returned will outlive both `Context` and `Parser`, and that the
reference `parse_context` returns refers to the string slice, not to `Context`
or `Parser`.

By knowing what the implementation of `parse` does, we know that the only
reason that the return value of `parse` is tied to the `Parser` is because it’s
referencing the `Parser`’s `Context`, which is referencing the string slice, so
it’s really the lifetime of the string slice that `parse_context` needs to care
about. We need a way to tell Rust that the string slice in `Context` and the
reference to the `Context` in `Parser` have different lifetimes and that the
return value of `parse_context` is tied to the lifetime of the string slice in
`Context`.

We could try only giving `Parser` and `Context` different lifetime parameters
as shown in Listing 19-15. We’ve chosen the lifetime parameter names `'s` and
`'c` here to be clearer about which lifetime goes with the string slice in
`Context` and which goes with the reference to `Context` in `Parser`. Note that
this won’t completely fix the problem, but it’s a start and we’ll look at why
this isn’t sufficient when we try to compile.

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

We’ve annotated the lifetimes of the references in all the same places that we
annotated them in Listing 19-13, but used different parameters depending on
whether the reference goes with the string slice or with `Context`. We’ve also
added an annotation to the string slice part of the return value of `parse` to
indicate that it goes with the lifetime of the string slice in `Context`.

Here’s the error we get now:

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
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```

Rust doesn’t know of any relationship between `'c` and `'s`. In order to be
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
slice in the `Context` have different lifetimes, and we’ve ensured that the
lifetime of the string slice is longer than the reference to the `Context`.

That was a very long-winded example, but as we mentioned at the start of this
chapter, these features are pretty niche. You won’t often need this syntax, but
it can come up in situations like this one, where you need to refer to
something you have a reference to.

### Lifetime Bounds

In Chapter 10, we discussed how to use trait bounds on generic types. We can
also add lifetime parameters as constraints on generic types, which are called
*lifetime bounds*. For example, consider a type that is a wrapper over
references. Recall the `RefCell<T>` type from Chapter 15: its `borrow` and
`borrow_mut` methods return the types `Ref` and `RefMut`, respectively. These
types are wrappers over references that keep track of the borrowing rules at
runtime. The definition of the `Ref` struct is shown in Listing 19-16, without
lifetime bounds for now:

```rust,ignore
struct Ref<'a, T>(&'a T);
```

<span class="caption">Listing 19-16: Defining a struct to wrap a reference to a
generic type; without lifetime bounds to start</span>

Without constraining the lifetime `'a` in relation to the generic parameter
`T`, we get an error because Rust doesn’t know how long the generic type `T`
will live:

```text
error[E0309]: the parameter type `T` may not live long enough
 --> <anon>:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> <anon>:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

Since `T` can be any type, `T` could itself be a reference or a type that holds
one or more references, each of which could have their own lifetimes. Rust
can’t be sure `T` will live as long as `'a`.

Fortunately, Rust gave us helpful advice on how to specify the lifetime bound in
this case:

```text
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at.
```

Listing 19-17 shows how to apply this advice by specifying the lifetime bound
when we declare the generic type `T`. This code now compiles because the `T:
'a` syntax specifies that `T` can be any type, but if it contains any
references, the references must live at least as long as `'a`:

```rust
struct Ref<'a, T: 'a>(&'a T);
```

<span class="caption">Listing 19-17: Adding lifetime bounds on `T` to specify
that any references in `T` live at least as long as `'a`</span>

We could choose to solve this in a different way, shown in the definition of a
`StaticRef` struct in Listing 19-18, by adding the `'static` lifetime bound on
`T`. This means if `T` contains any references, they must have the `'static`
lifetime:

```rust
struct StaticRef<T: 'static>(&'static T);
```

<span class="caption">Listing 19-18: Adding a `'static` lifetime bound to `T`
to constrain `T` to types that have only `'static` references or no
references</span>

Types without any references count as `T: 'static`. Because `'static` means the
reference must live as long as the entire program, a type that contains no
references meets the criteria of all references living as long as the entire
program (since there are no references). Think of it this way: if the borrow
checker is concerned about references living long enough, then there’s no real
distinction between a type that has no references and a type that has
references that live forever; both of them are the same for the purpose of
determining whether or not a reference has a shorter lifetime than what it
refers to.

### Trait Object Lifetimes

In Chapter 17, we learned about trait objects that consist of putting a trait
behind a reference in order to use dynamic dispatch. However, we didn’t discuss
what happens if the type implementing the trait used in the trait object has a
lifetime. Consider Listing 19-19, where we have a trait `Foo` and a struct
`Bar` that holds a reference (and thus has a lifetime parameter) that
implements trait `Foo`, and we want to use an instance of `Bar` as the trait
object `Box<Foo>`:

```rust
trait Foo { }

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> { }

let num = 5;

let obj = Box::new(Bar { x: &num }) as Box<Foo>;
```

<span class="caption">Listing 19-19: Using a type that has a lifetime parameter
with a trait object</span>

This code compiles without any errors, even though we haven’t said anything
about the lifetimes involved in `obj`. This works because there are rules
having to do with lifetimes and trait objects:

* The default lifetime of a trait object is `'static`.
* If we have `&'a X` or `&'a mut X`, then the default is `'a`.
* If we have a single `T: 'a` clause, then the default is `'a`.
* If we have multiple `T: 'a`-like clauses, then there is no default; we must
  be explicit.

When we must be explicit, we can add a lifetime bound on a trait object like
`Box<Foo>` with the syntax `Box<Foo + 'a>` or `Box<Foo + 'static>`, depending
on what’s needed. Just as with the other bounds, this means that any
implementor of the `Foo` trait that has any references inside must have the
lifetime specified in the trait object bounds as those references.

Next, let’s take a look at some other advanced features dealing with traits!
