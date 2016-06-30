# Option

Now that we have had an introduction to enums, let's combine them with a
feature that we talked a little bit about in the previous chapter: generics.

Programming language design is often thought of as which features you include,
but it's also about which features you leave out. Rust does not have a feature
that is in many other languages: 'null'. In languages with this feature,
variables can have two states: null or not-null.

The inventor of this concept has this to say:

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

The problem with null values is twofold: first, a value can be null or not, at
any time. The second is that if you try to use a value that's null, you'll get
an error of some kind, depending on the language. Because this property is
pervasive, it's extremely easy to make this kind of error.

Even with these problems, the concept that null is trying to express is still a
useful one: this is a value which is currently invalid or not present for some
reason. The problem isn't with the concept itself, but with the particular
implementation. As such, Rust does not have the concept of null, but we do have
an enum which can encode the concept of a value being present or not present. We
call this enum `Option<T>`, and it looks like this:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This enum is [provided by the standard library][option], and is so useful that
it's even in the prelude; you don't need to import it explicitly. Furthermore,
so are its variants: you can say `Some` and `None` directly, without prefixing
them with `Option::`.

[option]: ../std/option/enum.Option.html

Here's an example of using `Option<T>`:

```rust
let some_number = Some(5);
let some_string = Some("a string");

// If we only say None, we need to tell Rust what type of Option<T> we have.
let absent_number: Option<i32> = None;
```

Let's dig in. First, you'll notice that we used the `<T>` syntax when defining
`Option<T>`: it's a generic enum. `Option<T>` has two variants: `Some`, which
contains a `T`, and `None`, which has no data associated with it. In some
sense, `None` means 'null', and `Some` means 'not null'. So why is this any
better than null?

In short, because `Option<T>` and `T` are different types. That's a bit too
short though. Here's an example:

```rust,ignore
let x = 5;
let y = Some(5);

let sum = x + y;
```

This will not compile. We get an error message like this:

```text
error: the trait `core::ops::Add<core::option::Option<_>>` is not implemented
for the type `_` [E0277]

let sum = x + y;
          ^~~~~
```

Intense! What this error message is trying to say is that Rust does not
understand how to add an `Option<T>` and a `T`. They're different types! This
shows one of the big advantages of an `Option<T>`: if you have a value that
may or may not exist, you have to deal with that fact before you can assume it
exists. In other words, you have to convert an `Option<T>` to a `T` before you
can do `T` stuff with it. This helps catch one of the most common issues with
null, generally: assuming that something isn't null when it actually is.

So, how _do_ you get a `T` from an `Option<T>`?  The `Option<T>` enum has a
large number of methods that you can check out in [its documentation], and
becoming familiar with them will be extremely useful in your journey with Rust.

[its documentation]: ../std/option/enum.Option.html

But we want a deeper understanding than that. If we didn't have those methods
defined for us already, what would we do? And more generally, how do we get
the inner values out of any enum variant? We need a new feature: `match`.
