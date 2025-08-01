## Advanced Types

The Rust type system has some features that we’ve so far mentioned but haven’t
yet discussed. We’ll start by discussing newtypes in general as we examine why
newtypes are useful as types. Then we’ll move on to type aliases, a feature
similar to newtypes but with slightly different semantics. We’ll also discuss
the `!` type and dynamically sized types.

### Using the Newtype Pattern for Type Safety and Abstraction

This section assumes you’ve read the earlier section [“Using the Newtype Pattern
to Implement External Traits”][using-the-newtype-pattern]<!--
ignore -->. The newtype pattern is also useful for tasks beyond those we’ve
discussed so far, including statically enforcing that values are never confused
and indicating the units of a value. You saw an example of using newtypes to
indicate units in Listing 20-16: recall that the `Millimeters` and `Meters`
structs wrapped `u32` values in a newtype. If we wrote a function with a
parameter of type `Millimeters`, we wouldn’t be able to compile a program that
accidentally tried to call that function with a value of type `Meters` or a
plain `u32`.

We can also use the newtype pattern to abstract away some implementation
details of a type: the new type can expose a public API that is different from
the API of the private inner type.

Newtypes can also hide internal implementation. For example, we could provide a
`People` type to wrap a `HashMap<i32, String>` that stores a person’s ID
associated with their name. Code using `People` would only interact with the
public API we provide, such as a method to add a name string to the `People`
collection; that code wouldn’t need to know that we assign an `i32` ID to names
internally. The newtype pattern is a lightweight way to achieve encapsulation to
hide implementation details, which we discussed in [“Encapsulation that Hides
Implementation Details”][encapsulation-that-hides-implementation-details]<!--
ignore --> in Chapter 18.

### Creating Type Synonyms with Type Aliases

Rust provides the ability to declare a _type alias_ to give an existing type
another name. For this we use the `type` keyword. For example, we can create
the alias `Kilometers` to `i32` like so:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Now the alias `Kilometers` is a _synonym_ for `i32`; unlike the `Millimeters`
and `Meters` types we created in Listing 20-16, `Kilometers` is not a separate,
new type. Values that have the type `Kilometers` will be treated the same as
values of type `i32`:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

Because `Kilometers` and `i32` are the same type, we can add values of both
types and we can pass `Kilometers` values to functions that take `i32`
parameters. However, using this method, we don’t get the type-checking benefits
that we get from the newtype pattern discussed earlier. In other words, if we
mix up `Kilometers` and `i32` values somewhere, the compiler will not give us
an error.

The main use case for type synonyms is to reduce repetition. For example, we
might have a lengthy type like this:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Writing this lengthy type in function signatures and as type annotations all
over the code can be tiresome and error prone. Imagine having a project full of
code like that in Listing 20-25.

<Listing number="20-25" caption="Using a long type in many places">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

A type alias makes this code more manageable by reducing the repetition. In
Listing 20-26, we’ve introduced an alias named `Thunk` for the verbose type and
can replace all uses of the type with the shorter alias `Thunk`.

<Listing number="20-26" caption="Introducing a type alias, `Thunk`, to reduce repetition">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

This code is much easier to read and write! Choosing a meaningful name for a
type alias can help communicate your intent as well (_thunk_ is a word for code
to be evaluated at a later time, so it’s an appropriate name for a closure that
gets stored).

Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition. Consider the `std::io` module in the standard library. I/O
operations often return a `Result<T, E>` to handle situations when operations
fail to work. This library has a `std::io::Error` struct that represents all
possible I/O errors. Many of the functions in `std::io` will be returning
`Result<T, E>` where the `E` is `std::io::Error`, such as these functions in
the `Write` trait:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type
alias declaration:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Because this declaration is in the `std::io` module, we can use the fully
qualified alias `std::io::Result<T>`; that is, a `Result<T, E>` with the `E`
filled in as `std::io::Error`. The `Write` trait function signatures end up
looking like this:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

The type alias helps in two ways: it makes code easier to write _and_ it gives
us a consistent interface across all of `std::io`. Because it’s an alias, it’s
just another `Result<T, E>`, which means we can use any methods that work on
`Result<T, E>` with it, as well as special syntax like the `?` operator.

### The Never Type That Never Returns

Rust has a special type named `!` that’s known in type theory lingo as the
_empty type_ because it has no values. We prefer to call it the _never type_
because it stands in the place of the return type when a function will never
return. Here is an example:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

This code is read as “the function `bar` returns never.” Functions that return
never are called _diverging functions_. We can’t create values of the type `!`,
so `bar` can never possibly return.

But what use is a type you can never create values for? Recall the code from
Listing 2-5, part of the number-guessing game; we’ve reproduced a bit of it
here in Listing 20-27.

<Listing number="20-27" caption="A `match` with an arm that ends in `continue`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

At the time, we skipped over some details in this code. In [“The `match`
Control Flow Construct”][the-match-control-flow-construct]<!-- ignore --> in
Chapter 6, we discussed that `match` arms must all return the same type. So,
for example, the following code doesn’t work:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

The type of `guess` in this code would have to be an integer _and_ a string,
and Rust requires that `guess` have only one type. So what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 20-27?

As you might have guessed, `continue` has a `!` value. That is, when Rust
computes the type of `guess`, it looks at both match arms, the former with a
value of `u32` and the latter with a `!` value. Because `!` can never have a
value, Rust decides that the type of `guess` is `u32`.

The formal way of describing this behavior is that expressions of type `!` can
be coerced into any other type. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t return a value; instead, it moves control
back to the top of the loop, so in the `Err` case, we never assign a value to
`guess`.

The never type is useful with the `panic!` macro as well. Recall the `unwrap`
function that we call on `Option<T>` values to produce a value or panic with
this definition:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

In this code, the same thing happens as in the `match` in Listing 20-27: Rust
sees that `val` has the type `T` and `panic!` has the type `!`, so the result
of the overall `match` expression is `T`. This code works because `panic!`
doesn’t produce a value; it ends the program. In the `None` case, we won’t be
returning a value from `unwrap`, so this code is valid.

One final expression that has the type `!` is a `loop`:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Here, the loop never ends, so `!` is the value of the expression. However, this
wouldn’t be true if we included a `break`, because the loop would terminate
when it got to the `break`.

### Dynamically Sized Types and the `Sized` Trait

Rust needs to know certain details about its types, such as how much space to
allocate for a value of a particular type. This leaves one corner of its type
system a little confusing at first: the concept of _dynamically sized types_.
Sometimes referred to as _DSTs_ or _unsized types_, these types let us write
code using values whose size we can know only at runtime.

Let’s dig into the details of a dynamically sized type called `str`, which
we’ve been using throughout the book. That’s right, not `&str`, but `str` on
its own, is a DST. In many cases, such as when storing text entered by a user,
we can’t know how long the string is until runtime. That means we can’t create
a variable of type `str`, nor can we take an argument of type `str`. Consider
the following code, which does not work:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory. If Rust
allowed us to write this code, these two `str` values would need to take up the
same amount of space. But they have different lengths: `s1` needs 12 bytes of
storage and `s2` needs 15. This is why it’s not possible to create a variable
holding a dynamically sized type.

So what do we do? In this case, you already know the answer: we make the types
of `s1` and `s2` a `&str` rather than a `str`. Recall from [“String
Slices”][string-slices]<!-- ignore --> in Chapter 4 that the slice data
structure just stores the starting position and the length of the slice. So,
although a `&T` is a single value that stores the memory address of where the
`T` is located, a `&str` is _two_ values: the address of the `str` and its
length. As such, we can know the size of a `&str` value at compile time: it’s
twice the length of a `usize`. That is, we always know the size of a `&str`, no
matter how long the string it refers to is. In general, this is the way in which
dynamically sized types are used in Rust: they have an extra bit of metadata
that stores the size of the dynamic information. The golden rule of dynamically
sized types is that we must always put values of dynamically sized types behind
a pointer of some kind.

We can combine `str` with all kinds of pointers: for example, `Box<str>` or
`Rc<str>`. In fact, you’ve seen this before but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In [“Using Trait Objects to Abstract over Shared
Behavior”][using-trait-objects-to-abstract-over-shared-behavior]<!-- ignore -->
in Chapter 18, we mentioned that to use traits as trait objects, we must put
them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>` (`Rc<dyn
Trait>` would work too).

To work with DSTs, Rust provides the `Sized` trait to determine whether or not
a type’s size is known at compile time. This trait is automatically implemented
for everything whose size is known at compile time. In addition, Rust
implicitly adds a bound on `Sized` to every generic function. That is, a
generic function definition like this:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

is actually treated as though we had written this:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

By default, generic functions will work only on types that have a known size at
compile time. However, you can use the following special syntax to relax this
restriction:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

A trait bound on `?Sized` means “`T` may or may not be `Sized`” and this
notation overrides the default that generic types must have a known size at
compile time. The `?Trait` syntax with this meaning is only available for
`Sized`, not any other traits.

Also note that we switched the type of the `t` parameter from `T` to `&T`.
Because the type might not be `Sized`, we need to use it behind some kind of
pointer. In this case, we’ve chosen a reference.

Next, we’ll talk about functions and closures!

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-03-slices.html#string-slices
[the-match-control-flow-construct]: ch06-02-match.html#the-match-control-flow-construct
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[using-the-newtype-pattern]: ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits
