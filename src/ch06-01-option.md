## Options and "Null" Values

The `Option` enum combines enums with a feature we talked a little bit about in
the previous chapter: generics.

<!-- Did we take generics out of Ch 5? I couldn't spot them there, that may
have been cut. In that case, do you want to add it here/put it back in there?
-->

<!-- Also, can you connect up generics/enums with nulls here? I worry we're
launching into it without saying why we start talking about nulls -->

Programming language design is often thought of in terms of which features you
include, but the features you leave out are important too. Rust does not have
the *null* feature that many other languages contain.

<!-- Do we want to go into what it means for a variable to be null, or are we
safe assuming everyone has the same definition? A quick line on it might be
useful, to make sure everyone's on the same page, something like below, but I
defer to you! -->

A variable with a null value is one that has a value but whose value is
unspecified, so rather than a variable being empty or containing a zero it
contains a null.

In languages with this feature,
variables can have two states: null or not-null.

The inventor of the null concept has this to say:

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

The problem with null values is that <!-- a value can be null or not at any
time, but --> if you try to actually use a value that's null, you'll get an
error of some kind. Because this property is pervasive, it's extremely easy to
make this kind of error.

The concept that null is trying to express is still a useful one, however: a
null is a value which is currently invalid or absent for some reason.

The problem isn't with the concept itself, but with the particular
implementation. As such, Rust does not have nulls, but it does have
an enum that can encode the concept of a value being present or absent. We
call this enum `Option<T>`, and it looks like this:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This enum is provided by the standard library, and is so useful that
it's even in the prelude; you don't need to import it explicitly. Furthermore,
so are its variants: you can use `Some` and `None` directly, without prefixing
them with `Option::`.

<!-- We haven't spoken about the prelude so far in the book, I think I made a
note of that in a previous chapter---we should tell the reader what it is
before mentioning it so they know what significance it has here -->

<!-- What's the difference between Some and None? If we're going to cover that
later, maybe just say so here -->

Here's an example program using `Option<T>`:

```rust
let some_number = Some(5);
let some_string = Some("a string");

// If we use None rather than Some, we need to tell Rust what type of Option<T> we have.
let absent_number: Option<i32> = None;
```

<!--Below -- hm, it does seem imporant that we talk about generics at some
point first then -->

Let's dig in. First, you'll notice that we used the `<T>` syntax when defining
`Option<T>`: this is because it's a generic enum, so we use the generix syntanx
`<T>`.

`Option<T>` has two variants: `Some`, which contains a `T`, and `None`, which
has no data associated with it. In some sense, `None` means "null", and `Some`
means "not null". So why is this any better than null?

In short, because `Option<T>` and `T` are different types. That's a bit too
short though, we'll illustrate it with an example that won't compile, because
it's trying to compare those two different types:

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

We get an error message like this:

```bash
error: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not
satisfied [E0277]

let sum = x + y;
          ^~~~~
```

Intense! What this error message is trying to say is that Rust does not
understand how to add an `Option<T>` and a `T`. They're different types!

<!-- I think this could be more clearly stated, are we saying they are
different types *because* one is assumed to exist and the other is not? -->

This shows one of the big advantages of an `Option<T>`: if you have a value
that may or may not exist, you have to deal with that fact before you can
assume it exists. In other words, you have to convert an `Option<T>` to a `T`
before you can do `T` stuff with it. This helps catch one of the most common
issues with null, generally: assuming that something isn't null when it
actually is.

This is pretty powerful: in order to have a value that can possibly be null,
you have to explicitly opt in by making the type of that value `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn't an
`Option<T>`, you *can* safely assume that the value isn't null. This was a
deliberate design decision for Rust to limit null's pervasiveness and increase
the safety of Rust code.

<!-- So does None count as an option<T>? I lost the None thread a bit here -->

So, how _do_ you get a `T` from an `Option<T>`?  The `Option<T>` enum has a
large number of methods that you can check out in its documentation, and
becoming familiar with them will be extremely useful in your journey with Rust.

<!-- But we want a deeper understanding than that. If we didn't have those methods
defined for us already, what would we do? And more generally, how do we get
the inner values out of any enum variant? We need a new feature: `match`. -->
<!-- I'm not sure about this connecting paragraph, it doesn't seem like match
and option are actually that much connected, at least not at first. That's
fine, this is all under the enum heading, but it might confuse if we imply
otherwise --- unless I'm just missing the connection -->
