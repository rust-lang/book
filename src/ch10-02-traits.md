## Traits: Defining Shared Behavior

A *trait* tells the Rust compiler about functionality a particular type has and
can share with other types. We can use traits to define shared behavior in an
abstract way. We can use trait bounds to specify that a generic type can be any
type that has certain behavior.

> Note: Traits are similar to a feature often called *interfaces* in other
> languages, although with some differences.

### Defining a Trait

A type’s behavior consists of the methods we can call on that type. Different
types share the same behavior if we can call the same methods on all of those
types. Trait definitions are a way to group method signatures together to
define a set of behaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and
amounts of text: a `NewsArticle` struct that holds a news story filed in a
particular location and a `Tweet` that can have at most 280 characters along
with metadata that indicates whether it was a new tweet, a retweet, or a reply
to another tweet.

We want to make a media aggregator library that can display summaries of data
that might be stored in a `NewsArticle` or `Tweet` instance. To do this, we
need a summary from each type, and we need to request that summary by calling a
`summarize` method on an instance. Listing 10-12 shows the definition of a
`Summary` trait that expresses this behavior.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

<span class="caption">Listing 10-12: A `Summary` trait that consists of the
behavior provided by a `summarize` method</span>

Here, we declare a trait using the `trait` keyword and then the trait’s name,
which is `Summary` in this case. Inside the curly brackets, we declare the
method signatures that describe the behaviors of the types that implement this
trait, which in this case is `fn summarize(&self) -> String`.

After the method signature, instead of providing an implementation within curly
brackets, we use a semicolon. Each type implementing this trait must provide
its own custom behavior for the body of the method. The compiler will enforce
that any type that has the `Summary` trait will have the method `summarize`
defined with this signature exactly.

A trait can have multiple methods in its body: the method signatures are listed
one per line and each line ends in a semicolon.

### Implementing a Trait on a Type

Now that we’ve defined the desired behavior using the `Summary` trait, we can
implement it on the types in our media aggregator. Listing 10-13 shows an
implementation of the `Summary` trait on the `NewsArticle` struct that uses the
headline, the author, and the location to create the return value of
`summarize`. For the `Tweet` struct, we define `summarize` as the username
followed by the entire text of the tweet, assuming that tweet content is
already limited to 280 characters.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

<span class="caption">Listing 10-13: Implementing the `Summary` trait on the
`NewsArticle` and `Tweet` types</span>

Implementing a trait on a type is similar to implementing regular methods. The
difference is that after `impl`, we put the trait name that we want to
implement, then use the `for` keyword, and then specify the name of the type we
want to implement the trait for. Within the `impl` block, we put the method
signatures that the trait definition has defined. Instead of adding a semicolon
after each signature, we use curly brackets and fill in the method body with
the specific behavior that we want the methods of the trait to have for the
particular type.

After implementing the trait, we can call the methods on instances of
`NewsArticle` and `Tweet` in the same way we call regular methods, like this:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs:here}}
```

This code prints `1 new tweet: horse_ebooks: of course, as you probably already
know, people`.

Note that because we defined the `Summary` trait and the `NewsArticle` and
`Tweet` types in the same *lib.rs* in Listing 10-13, they’re all in the same
scope. Let’s say this *lib.rs* is for a crate we’ve called `aggregator` and
someone else wants to use our crate’s functionality to implement the `Summary`
trait on a struct defined within their library’s scope. They would need to
bring the trait into their scope first. They would do so by specifying `use
aggregator::Summary;`, which then would enable them to implement `Summary` for
their type. The `Summary` trait would also need to be a public trait for
another crate to implement it, which it is because we put the `pub` keyword
before `trait` in Listing 10-12.

One restriction to note with trait implementations is that we can implement a
trait on a type only if either the trait or the type is local to our crate.
For example, we can implement standard library traits like `Display` on a
custom type like `Tweet` as part of our `aggregator` crate functionality,
because the type `Tweet` is local to our `aggregator` crate. We can also
implement `Summary` on `Vec<T>` in our `aggregator` crate, because the
trait `Summary` is local to our `aggregator` crate.

But we can’t implement external traits on external types. For example, we can’t
implement the `Display` trait on `Vec<T>` within our `aggregator` crate,
because `Display` and `Vec<T>` are defined in the standard library and aren’t
local to our `aggregator` crate. This restriction is part of a property of
programs called *coherence*, and more specifically the *orphan rule*, so named
because the parent type is not present. This rule ensures that other people’s
code can’t break your code and vice versa. Without the rule, two crates could
implement the same trait for the same type, and Rust wouldn’t know which
implementation to use.

### Default Implementations

Sometimes it’s useful to have default behavior for some or all of the methods
in a trait instead of requiring implementations for all methods on every type.
Then, as we implement the trait on a particular type, we can keep or override
each method’s default behavior.

Listing 10-14 shows how to specify a default string for the `summarize` method
of the `Summary` trait instead of only defining the method signature, as we did
in Listing 10-12.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

<span class="caption">Listing 10-14: Definition of a `Summary` trait with a
default implementation of the `summarize` method</span>

To use a default implementation to summarize instances of `NewsArticle` instead
of defining a custom implementation, we specify an empty `impl` block with
`impl Summary for NewsArticle {}`.

Even though we’re no longer defining the `summarize` method on `NewsArticle`
directly, we’ve provided a default implementation and specified that
`NewsArticle` implements the `Summary` trait. As a result, we can still call
the `summarize` method on an instance of `NewsArticle`, like this:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

This code prints `New article available! (Read more...)`.

Creating a default implementation for `summarize` doesn’t require us to change
anything about the implementation of `Summary` on `Tweet` in Listing 10-13. The
reason is that the syntax for overriding a default implementation is the same
as the syntax for implementing a trait method that doesn’t have a default
implementation.

Default implementations can call other methods in the same trait, even if those
other methods don’t have a default implementation. In this way, a trait can
provide a lot of useful functionality and only require implementors to specify
a small part of it. For example, we could define the `Summary` trait to have a
`summarize_author` method whose implementation is required, and then define a
`summarize` method that has a default implementation that calls the
`summarize_author` method:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

To use this version of `Summary`, we only need to define `summarize_author`
when we implement the trait on a type:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

After we define `summarize_author`, we can call `summarize` on instances of the
`Tweet` struct, and the default implementation of `summarize` will call the
definition of `summarize_author` that we’ve provided. Because we’ve implemented
`summarize_author`, the `Summary` trait has given us the behavior of the
`summarize` method without requiring us to write any more code.

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

This code prints `1 new tweet: (Read more from @horse_ebooks...)`.

Note that it isn’t possible to call the default implementation from an
overriding implementation of that same method.

### Traits as Parameters

Now that you know how to define and implement traits, we can explore how to use
traits to define functions that accept many different types.

For example, in Listing 10-13, we implemented the `Summary` trait on the
`NewsArticle` and `Tweet` types. We can define a `notify` function that calls
the `summarize` method on its `item` parameter, which is of some type that
implements the `Summary` trait. To do this, we can use the `impl Trait`
syntax, like this:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

Instead of a concrete type for the `item` parameter, we specify the `impl`
keyword and the trait name. This parameter accepts any type that implements the
specified trait. In the body of `notify`, we can call any methods on `item`
that come from the `Summary` trait, such as `summarize`. We can call `notify`
and pass in any instance of `NewsArticle` or `Tweet`. Code that calls the
function with any other type, such as a `String` or an `i32`, won’t compile
because those types don’t implement `Summary`.

#### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is actually
syntax sugar for a longer form, which is called a *trait bound*; it looks like
this:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous section but is
more verbose. We place trait bounds with the declaration of the generic type
parameter after a colon and inside angle brackets.

The `impl Trait` syntax is convenient and makes for more concise code in simple
cases. The trait bound syntax can express more complexity in other cases. For
example, we can have two parameters that implement `Summary`. Using the `impl
Trait` syntax looks like this:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

If we wanted this function to allow `item1` and `item2` to have different
types, using `impl Trait` would be appropriate (as long as both types implement
`Summary`). If we wanted to force both parameters to have the same type, that’s
only possible to express using a trait bound, like this:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

The generic type `T` specified as the type of the `item1` and `item2`
parameters constrains the function such that the concrete type of the value
passed as an argument for `item1` and `item2` must be the same.

#### Specifying Multiple Trait Bounds with the `+` Syntax

We can also specify more than one trait bound. Say we wanted `notify` to use
display formatting on `item` as well as the `summarize` method: we specify in
the `notify` definition that `item` must implement both `Display` and
`Summary`. We can do so using the `+` syntax:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

The `+` syntax is also valid with trait bounds on generic types:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

With the two trait bounds specified, the body of `notify` can call `summarize`
and use `{}` to format `item`.

#### Clearer Trait Bounds with `where` Clauses

Using too many trait bounds has its downsides. Each generic has its own trait
bounds, so functions with multiple generic type parameters can contain lots of
trait bound information between the function’s name and its parameter list,
making the function signature hard to read. For this reason, Rust has alternate
syntax for specifying trait bounds inside a `where` clause after the function
signature. So instead of writing this:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a `where` clause, like this:

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

This function’s signature is less cluttered: the function name, parameter list,
and return type are close together, similar to a function without lots of trait
bounds.

### Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a
value of some type that implements a trait, as shown here:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

By using `impl Summary` for the return type, we specify that the
`returns_summarizable` function returns some type that implements the `Summary`
trait without naming the concrete type. In this case, `returns_summarizable`
returns a `Tweet`, but the code calling this function doesn’t know that.

The ability to return a type that is only specified by the trait it implements
is especially useful in the context of closures and iterators, which we cover
in Chapter 13. Closures and iterators create types that only the compiler knows
or types that are very long to specify. The `impl Trait` syntax lets you
concisely specify that a function returns some type that implements the
`Iterator` trait without needing to write out a very long type.

However, you can only use `impl Trait` if you’re returning a single type. For
example, this code that returns either a `NewsArticle` or a `Tweet` with the
return type specified as `impl Summary` wouldn’t work:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions
around how the `impl Trait` syntax is implemented in the compiler. We’ll cover
how to write a function with this behavior in the [“Using Trait Objects That
Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> section of Chapter 17.

### Fixing the `largest` Function with Trait Bounds

Now that you know how to specify the behavior you want to use using the generic
type parameter’s bounds, let’s return to Listing 10-5 to fix the definition of
the `largest` function that uses a generic type parameter! Last time we tried
to run that code, we received this error:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

In the body of `largest` we wanted to compare two values of type `T` using the
greater than (`>`) operator. Because that operator is defined as a default
method on the standard library trait `std::cmp::PartialOrd`, we need to specify
`PartialOrd` in the trait bounds for `T` so the `largest` function can work on
slices of any type that we can compare. We don’t need to bring `PartialOrd`
into scope because it’s in the prelude. Change the signature of `largest` to
look like this:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/src/main.rs:here}}
```

This time when we compile the code, we get a different set of errors:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-fixing-listing-10-05/output.txt}}
```

The key line in this error is `cannot move out of type [T], a non-copy slice`.
With our non-generic versions of the `largest` function, we were only trying to
find the largest `i32` or `char`. As discussed in the [“Stack-Only Data:
Copy”][stack-only-data-copy]<!-- ignore --> section in Chapter 4, types like
`i32` and `char` that have a known size can be stored on the stack, so they
implement the `Copy` trait. But when we made the `largest` function generic,
it became possible for the `list` parameter to have types in it that don’t
implement the `Copy` trait. Consequently, we wouldn’t be able to move the
value out of `list[0]` and into the `largest` variable, resulting in this
error.

To call this code with only those types that implement the `Copy` trait, we can
add `Copy` to the trait bounds of `T`! Listing 10-15 shows the complete code of
a generic `largest` function that will compile as long as the types of the
values in the slice that we pass into the function implement the `PartialOrd`
*and* `Copy` traits, like `i32` and `char` do.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/main.rs}}
```

<span class="caption">Listing 10-15: A working definition of the `largest`
function that works on any generic type that implements the `PartialOrd` and
`Copy` traits</span>

If we don’t want to restrict the `largest` function to the types that implement
the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead
of `Copy`. Then we could clone each value in the slice when we want the
`largest` function to have ownership. Using the `clone` function means we’re
potentially making more heap allocations in the case of types that own heap
data like `String`, and heap allocations can be slow if we’re working with
large amounts of data.

Another way we could implement `largest` is for the function to return a
reference to a `T` value in the slice. If we change the return type to `&T`
instead of `T`, thereby changing the body of the function to return a
reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could
avoid heap allocations. Try implementing these alternate solutions on your own!
If you get stuck with errors having to do with lifetimes, keep reading: the
“Validating References with Lifetimes” section coming up will explain, but
lifetimes aren't required to solve these challenges.

### Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an `impl` block that uses generic type parameters,
we can implement methods conditionally for types that implement the specified
traits. For example, the type `Pair<T>` in Listing 10-16 always implements the
`new` function. But `Pair<T>` only implements the `cmp_display` method if its
inner type `T` implements the `PartialOrd` trait that enables comparison *and*
the `Display` trait that enables printing.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/lib.rs}}
```

<span class="caption">Listing 10-16: Conditionally implement methods on a
generic type depending on trait bounds</span>

We can also conditionally implement a trait for any type that implements
another trait. Implementations of a trait on any type that satisfies the trait
bounds are called *blanket implementations* and are extensively used in the
Rust standard library. For example, the standard library implements the
`ToString` trait on any type that implements the `Display` trait. The `impl`
block in the standard library looks similar to this code:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the
`to_string` method defined by the `ToString` trait on any type that implements
the `Display` trait. For example, we can turn integers into their corresponding
`String` values like this because integers implement `Display`:

```rust
let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the
“Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters to
reduce duplication but also specify to the compiler that we want the generic
type to have particular behavior. The compiler can then use the trait bound
information to check that all the concrete types used with our code provide the
correct behavior. In dynamically typed languages, we would get an error at
runtime if we called a method on a type which didn’t define the method. But Rust
moves these errors to compile time so we’re forced to fix the problems before
our code is even able to run. Additionally, we don’t have to write code that
checks for behavior at runtime because we’ve already checked at compile time.
Doing so improves performance without having to give up the flexibility of
generics.

Another kind of generic that we’ve already been using is called *lifetimes*.
Rather than ensuring that a type has the behavior we want, lifetimes ensure
that references are valid as long as we need them to be. Let’s look at how
lifetimes do that.

[stack-only-data-copy]:
ch04-01-what-is-ownership.html#stack-only-data-copy
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
