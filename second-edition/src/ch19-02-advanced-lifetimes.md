## Advanced Lifetimes

Back in Chapter 10 in the “Validating References with Lifetimes” section, we
learned how to annotate references with lifetime parameters to tell Rust how
lifetimes of different references relate. We saw how every reference has a
lifetime but, most of the time, Rust will let you elide lifetimes. Here we’ll
look at three advanced features of lifetimes that we haven’t covered yet:

* Lifetime subtyping, a way to ensure that one lifetime outlives another
  lifetime
* Lifetime bounds, to specify a lifetime for a reference to a generic type
* Trait object lifetimes, how they’re inferred, and when they need to be
  specified

<!-- maybe add a small summary of each here? That would let us launch straight
into examples in the next section -->
<!-- I've switched to bullets and added a small summary /Carol -->

### Lifetime Subtyping Ensures One Lifetime Outlives Another

Lifetime subtyping is a way to specify that one lifetime should outlive another
lifetime. To explore lifetime subtyping, imagine we want to write a parser.
We’ll have a structure called `Context` that holds a reference to the string
we’re parsing. We’ll write a parser that will parse this string and return
success or failure. The parser will need to borrow the context to do the
parsing. Implementing this would look like the code in Listing 19-12, except
this code doesn’t have the required lifetime annotations so it won’t compile:

<span class="filename">Filename: src/lib.rs</span>

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

<span class="caption">Listing 19-12: Defining a parser without lifetime
annotations</span>

Compiling the code results in errors saying that Rust expected lifetime
parameters on the string slice in `Context` and the reference to a `Context` in
`Parser`.

<!-- What will the compile time error be here? I think it'd be worth showing
that to the reader -->
<!-- The errors just say "expected lifetime parameter", they're pretty boring.
We've shown error messages like that before so I've explained in words instead.
/Carol -->

For simplicity’s sake, our `parse` function returns a `Result<(), &str>`. That
is, it will do nothing on success, and on failure will return the part of the
string slice that didn’t parse correctly. A real implementation would have more
error information than that, and would actually return something when parsing
succeeds, but we’ll leave those off because they aren’t relevant to the
lifetimes part of this example.

To keep this code simple, we’re not going to actually write any parsing logic.
It’s very likely that somewhere in parsing logic we’d handle invalid input by
returning an error that references the part of the input that’s invalid, and
this reference is what makes the code example interesting with regards to
lifetimes. So we’re going to pretend that the logic of our parser is that the
input is invalid after the first byte. Note that this code may panic if the
first byte is not on a valid character boundary; again, we’re simplifying the
example in order to concentrate on the lifetimes involved.

<!-- why do we want to always error after the first byte? -->
<!-- For simplicity of the example to avoid cluttering up the code with actual
parsing logic, which isn't the point. I've explained a bit more above /Carol -->

To get this code compiling, we need to fill in the lifetime parameters for the
string slice in `Context` and the reference to the `Context` in `Parser`. The
most straightforward way to do this is to use the same lifetime everywhere, as
shown in Listing 19-13:

<span class="filename">Filename: src/lib.rs</span>

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

This compiles fine, and tells Rust that a `Parser` holds a reference to a
`Context` with lifetime `'a`, and that `Context` holds a string slice that also
lives as long as the reference to the `Context` in `Parser`. Rust’s compiler
error message said lifetime parameters were required for these references, and
we have now added lifetime parameters.

<!-- can you let the reader know they should be taking away from this previous
example? I'm not totally clear on why adding lifetimes here saved the code -->
<!-- Done -->

Next, in Listing 19-14, let’s add a function that takes an instance of
`Context`, uses a `Parser` to parse that context, and returns what `parse`
returns. This won’t quite work:

<span class="filename">Filename: src/lib.rs</span>

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
error[E0597]: borrowed value does not live long enough
  --> src/lib.rs:14:5
   |
14 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
15 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^

error[E0597]: `context` does not live long enough
  --> src/lib.rs:14:24
   |
14 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
15 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 13:1...
  --> src/lib.rs:13:1
   |
13 | / fn parse_context(context: Context) -> Result<(), &str> {
14 | |     Parser { context: &context }.parse()
15 | | }
   | |_^
```

These errors are saying that both the `Parser` instance that’s created and the
`context` parameter live only from when the `Parser` is created until the end
of the `parse_context` function, but they both need to live for the entire
lifetime of the function.

In other words, `Parser` and `context` need to *outlive* the entire function
and be valid before the function starts as well as after it ends in order for
all the references in this code to always be valid. Both the `Parser` we’re
creating and the `context` parameter go out of scope at the end of the
function, though (because `parse_context` takes ownership of `context`).

<!-- Oh interesting, why do they need to outlive the function, simply to
absolutely ensure they will live for as long as the function? -->
<!-- Yes, which is what I think we've said in the first sentence of the
previous paragraph. Is there something that's unclear? /Carol -->

To figure out why we’re getting these errors, let’s look at the definitions in
Listing 19-13 again, specifically the references in the signature of the
`parse` method:

```rust,ignore
    fn parse(&self) -> Result<(), &str> {
```

<!-- What exactly is it the reader should be looking at in this signature? -->
<!-- Added above /Carol -->

Remember the elision rules? If we annotate the lifetimes of the references
rather than eliding, the signature would be:

```rust,ignore
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

That is, the error part of the return value of `parse` has a lifetime that is
tied to the lifetime of the `Parser` instance (that of `&self` in the `parse`
method signature). That makes sense: the returned string slice references the
string slice in the `Context` instance held by the `Parser`, and the definition
of the `Parser` struct specifies that the lifetime of the reference to
`Context` and the lifetime of the string slice that `Context` holds should be
the same.

The problem is that the `parse_context` function returns the value returned
from `parse`, so the lifetime of the return value of `parse_context` is tied to
the lifetime of the `Parser` as well. But the `Parser` instance created in the
`parse_context` function won’t live past the end of the function (it’s
temporary), and `context` will go out of scope at the end of the function
(`parse_context` takes ownership of it).

Rust thinks we’re trying to return a reference to a value that goes out of
scope at the end of the function, because we annotated all the lifetimes with
the same lifetime parameter. That told Rust the lifetime of the string slice
that `Context` holds is the same as that of the lifetime of the reference to
`Context` that `Parser` holds.

The `parse_context` function can’t see that within the `parse` function, the
string slice returned will outlive both `Context` and `Parser`, and that the
reference `parse_context` returns refers to the string slice, not to `Context`
or `Parser`.

By knowing what the implementation of `parse` does, we know that the only
reason the return value of `parse` is tied to the `Parser` is because it’s
referencing the `Parser`’s `Context`, which is referencing the string slice, so
it’s really the lifetime of the string slice that `parse_context` needs to care
about. We need a way to tell Rust that the string slice in `Context` and the
reference to the `Context` in `Parser` have different lifetimes and that the
return value of `parse_context` is tied to the lifetime of the string slice in
`Context`.

First we’ll try giving `Parser` and `Context` different lifetime parameters as
shown in Listing 19-15. We’ll use `'s` and `'c` as lifetime parameter names to
be clear about which lifetime goes with the string slice in `Context` and which
goes with the reference to `Context` in `Parser`. Note that this won’t
completely fix the problem, but it’s a start and we’ll look at why this isn’t
sufficient when we try to compile.

<span class="filename">Filename: src/lib.rs</span>

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

The following is the error we get now when we try to compile:

```text
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
 --> src/lib.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:1
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:1
 --> src/lib.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```

Rust doesn’t know of any relationship between `'c` and `'s`. In order to be
valid, the referenced data in `Context` with lifetime `'s` needs to be
constrained, to guarantee that it lives longer than the reference with lifetime
`'c`. If `'s` is not longer than `'c`, the reference to `Context` might not be
valid.

Which gets us to the point of this section: the Rust feature *lifetime
subtyping* is a way to specify that one lifetime parameter lives at least as
long as another one. In the angle brackets where we declare lifetime
parameters, we can declare a lifetime `'a` as usual, and declare a lifetime
`'b` that lives at least as long as `'a` by declaring `'b` with the syntax `'b:
'a`.

In our definition of `Parser`, in order to say that `'s` (the lifetime of the
string slice) is guaranteed to live at least as long as `'c` (the lifetime of
the reference to `Context`), we change the lifetime declarations to look like
this:

<span class="filename">Filename: src/lib.rs</span>

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

### Lifetime Bounds on References to Generic Types

In the “Trait Bounds” section of Chapter 10, we discussed using trait bounds on
generic types. We can also add lifetime parameters as constraints on generic
types, and these are called *lifetime bounds*. Lifetime bounds help Rust verify
that references in generic types won’t outlive the data they’re referencing.

<!-- Can you say up front why/when we use these? -->
<!-- Done -->

For an example, consider a type that is a wrapper over references. Recall the
`RefCell<T>` type from the “`RefCell<T>` and the Interior Mutability Pattern”
section of Chapter 15: its `borrow` and `borrow_mut` methods return the types
`Ref` and `RefMut`, respectively. These types are wrappers over references that
keep track of the borrowing rules at runtime. The definition of the `Ref`
struct is shown in Listing 19-16, without lifetime bounds for now:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
struct Ref<'a, T>(&'a T);
```

<span class="caption">Listing 19-16: Defining a struct to wrap a reference to a
generic type; without lifetime bounds to start</span>

Without explicitly constraining the lifetime `'a` in relation to the generic
parameter `T`, Rust will error because it doesn’t know how long the generic
type `T` will live:

```text
error[E0309]: the parameter type `T` may not live long enough
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

Because `T` can be any type, `T` could itself be a reference or a type that
holds one or more references, each of which could have their own lifetimes.
Rust can’t be sure `T` will live as long as `'a`.

Fortunately, that error gave us helpful advice on how to specify the lifetime
bound in this case:

```text
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at
```

Listing 19-17 shows how to apply this advice by specifying the lifetime bound
when we declare the generic type `T`.

```rust
struct Ref<'a, T: 'a>(&'a T);
```

<span class="caption">Listing 19-17: Adding lifetime bounds on `T` to specify
that any references in `T` live at least as long as `'a`</span>

This code now compiles because the `T: 'a` syntax specifies that `T` can be any
type, but if it contains any references, the references must live at least as
long as `'a`.

We could solve this in a different way, shown in the definition of a
`StaticRef` struct in Listing 19-18, by adding the `'static` lifetime bound on
`T`. This means if `T` contains any references, they must have the `'static`
lifetime:

```rust
struct StaticRef<T: 'static>(&'static T);
```

<span class="caption">Listing 19-18: Adding a `'static` lifetime bound to `T`
to constrain `T` to types that have only `'static` references or no
references</span>

Because `'static` means the reference must live as long as the entire program,
a type that contains no references meets the criteria of all references living
as long as the entire program (because there are no references). For the borrow
checker concerned about references living long enough, there’s no real
distinction between a type that has no references and a type that has
references that live forever; both of them are the same for the purpose of
determining whether or not a reference has a shorter lifetime than what it
refers to.

### Inference of Trait Object Lifetimes

In Chapter 17 in the “Using Trait Objects that Allow for Values of Different
Types” section, we discussed trait objects, consisting of a trait behind a
reference, that allow us to use dynamic dispatch. We haven’t yet discussed what
happens if the type implementing the trait in the trait object has a lifetime
of its own. Consider Listing 19-19, where we have a trait `Red` and a struct
`Ball`. `Ball` holds a reference (and thus has a lifetime parameter) and also
implements trait `Red`. We want to use an instance of `Ball` as the trait
object `Box<Red>`:

<span class="filename">Filename: src/main.rs</span>

```rust
trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}
```

<span class="caption">Listing 19-19: Using a type that has a lifetime parameter
with a trait object</span>

This code compiles without any errors, even though we haven’t said anything
explicit about the lifetimes involved in `obj`. This works because there are
rules having to do with lifetimes and trait objects:

* The default lifetime of a trait object is `'static`.
* With `&'a Trait` or `&'a mut Trait`, the default lifetime is `'a`.
* With a single `T: 'a` clause, the default lifetime is `'a`.
* With multiple `T: 'a`-like clauses, there is no default; we must
  be explicit.

When we must be explicit, we can add a lifetime bound on a trait object like
`Box<Red>` with the syntax `Box<Red + 'a>` or `Box<Red + 'static>`, depending
on what’s needed. Just as with the other bounds, this means that any
implementor of the `Red` trait that has references inside must have the
lifetime specified in the trait object bounds as those references.

Next, let’s take a look at some other advanced features dealing with traits!
