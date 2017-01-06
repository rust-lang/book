## The `Option` Enum and its Advantages Over Null Values

In the previous section, we looked at how the `IpAddr` enum let us use Rust's
type system to encode more information than just the data into our program.
This section is a case study of `Option`, which is another enum defined by the
standard library. The `Option` type is used in many places because it encodes
the very common scenario in which a value could be *something* or it could be
*nothing*. Expressing this concept in terms of the type system means the
compiler can check that you've handled all the cases you should be handling,
which can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in terms of which features you
include, but the features you leave out are important too. Rust does not have
the *null* feature that many other languages have. Null is a value that means
there is no value there. In languages with null, variables can always be in one
of two states: null or not-null.

The inventor of null has this to say:

> I call it my billion-dollar mistake. At that time, I was designing the first
> comprehensive type system for references in an object-oriented language. My
> goal was to ensure that all use of references should be absolutely safe, with
> checking performed automatically by the compiler. But I couldn't resist the
> temptation to put in a null reference, simply because it was so easy to
> implement. This has led to innumerable errors, vulnerabilities, and system
> crashes, which have probably caused a billion dollars of pain and damage in
> the last forty years.
>
> - Tony Hoare "Null References: The Billion Dollar Mistake"

The problem with null values is that if you try to actually use a value that's
null as if it is a not-null value, you'll get an error of some kind. Because
this null or not-null property is pervasive, it's extremely easy to make this
kind of error.

The concept that null is trying to express is still a useful one, however: a
null is a value which is currently invalid or absent for some reason.

The problem isn't with the concept itself, but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is [defined by the standard library][option]<!-- ignore -->
as follows:

[option]: ../std/option/enum.Option.html

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is so useful that it's even included in the prelude; you
don't need to import it explicitly. Furthermore, so are its variants: you can
use `Some` and `None` directly, without prefixing them with `Option::`.
`Option<T>` is still just a regular enum, however, and `Some(T)` and `None` are
still values of type `Option<T>`.

<!-- We haven't spoken about the prelude so far in the book, I think I made a
note of that in a previous chapter---we should tell the reader what it is
before mentioning it so they know what significance it has here -->

<!-- We did speak about the prelude previously, in chapter 2, the Processing a
Guess section. I don't have any comments from you about it there... /Carol -->

The `<T>` syntax is a feature of Rust we haven't talked about yet. It's a
generic type parameter, and we'll cover generics in more detail in Chapter 10.
For now, all you need to know is that this means the `Some` variant of the
`Option` enum can hold one piece of data of any type. Here are some examples of
using `Option` values to hold number types and string types:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

If we use `None` rather than `Some`, we need to tell Rust what type of
`Option<T>` we have, because the compiler can't infer the type that the `Some`
variant will hold by looking at the `None` variant.

When we have a `Some` value, we know that there is a value present, and the
value is held within the `Some`. When we have a `None` value, in some sense,
that means the same thing that null does: we do not have a valid value. So why
is this any better than null?

In short, because `Option<T>` and `T` (where `T` can be any type) are different
types from each other, so the compiler won't let us use an `Option` value as if
it was definitely a valid value. For example, this code won't compile because
it's trying to compare an `Option<i8>` to an `i8`:

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

If we run this code, we get an error message like this:

```text
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not satisfied
 -->
  |
7 | let sum = x + y;
  |           ^^^^^
  |
```

Intense! What this error message is trying to say is that Rust does not
understand how to add an `Option<i8>` and an `i8`, since they're different
types. When we have a value of a type like `i8` in Rust, the compiler will
ensure that we always have a valid value. We can proceed confidently without
having to check for null before using that value. Only when we have an
`Option<i8>` (or whatever type of value we're working with) do we have to
worry about possibly not having a value, and the compiler will make sure we
handle that case before using the value.

In other words, you have to convert an `Option<T>` to a `T` before you can do
`T` stuff with it. This helps catch one of the most common issues with null,
generally: assuming that something isn't null when it actually is.

This is pretty powerful: in order to have a value that can possibly be null,
you have to explicitly opt in by making the type of that value `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn't an
`Option<T>`, you *can* safely assume that the value isn't null. This was a
deliberate design decision for Rust to limit null's pervasiveness and increase
the safety of Rust code.

<!-- So does None count as an option<T>? I lost the None thread a bit here -->
<!-- Yes, since `Option<T>` is an enum and `None` is a variant of this enum,
`None`'s type is `Option<T>`. I hope with the clarifications I added in the
previous section that this will be clear by this point. /Carol -->

So, how *do* you get the `T` value out of a `Some` variant when you have a
value of type `Option<T>` so that you can use that value? The `Option<T>` enum
has a large number of methods useful in a variety of situations that you can
check out in [its documentation][docs]<!-- ignore -->, and becoming familiar
with them will be extremely useful in your journey with Rust.

[docs]: ../std/option/enum.Option.html

What we generally want to do in order to use an `Option<T>` value is to have
code that will handle each variant. We want some code that will run only in the
case that we have a `Some(T)` value, and this code *is* allowed to use the
inner `T`. We want some *other* code to run if we have a `None` value, and that
code doesn't have a `T` value available. The `match` expression is a control
flow construct that does just this, when used with enums: it will run different
code depending on which variant of the enum it has, and that code can use the
data inside the matching value.

<!-- I'm not sure about this connecting paragraph, it doesn't seem like match
and option are actually that much connected, at least not at first. That's
fine, this is all under the enum heading, but it might confuse if we imply
otherwise --- unless I'm just missing the connection -->
<!-- I've tried to make the connection more explicit, is this better? /Carol -->
