<!-- Hi Steve, Carol. I like this chapter, we're getting into some powerful
tools here! I saw that the copyeditor has suggested listing numbers, which I
think is a good idea. If you agree, could you suggest captions and add
numbering to those listings we reference again in the chapter? Also, if we are
going to include any of these in the source files can you add file names?
Thanks! /Liz -->

[TOC]

# Enums

In this chapter we'll look at *enumerations*, or enums. Enums allow you to
define a type by enumerating its possible values, and unlock a lot of power in
Rust when combined with pattern matching.

Enums are a feature
in many languages, but their capabilities differ per-language. Rust’s enums
are most similar to "algebraic data types" in functional languages like F#,
OCaml, or Haskell.

## Defining an Enum

<!--This chapter doesn't have many heading separators, I've suggested a few
places we could add some to help organize it, but feel free to change the
headings, or space them more logically if you think they'd be better placed
elswhere -->

<!-- I wasn't clear throughout this section whether we were defining the
IpAddrKind enum or looking inside it --- could you read carefully, correct
anything I've misunderstood, make sure it's clear early on? -->

We'll start with an example of an enum definition the enumerates the possible
types an IP address can take:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

We define an enum named `IpAddrKind` and list its members, known as *variants*,
as `V4` and `V6`. These are the two major standards used for IP addresses:
version four and version six, listed here as `V4` and `V6`. Any IP address can
be either a version four or a version six address, but not both at the same
time. This is where enums get their name: they allow us to enumerate all of the
possible kinds that our value can have.

<!-- so if it *could* be both V4 and V6 at the same time, would the list have
included "v4,v6" or something similar? -->

### Storing Values and Data

Enum variants can optionally have associated values. We can create values of
`IpAddrKind` like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we
use the double colon to separate the two.

Enums have more tricks up their sleeves, too. Thinking more about our IP
address type, at the moment we don’t have a way to store the actual data of the
IP address; we only know what kind it is. Given that you just learned about
structs, you might tackle this problem like this:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

<!-- I think this could use a more thorough explanation, so `kind` and
`address` are the values of the struct, is that right? What does this actually
result in, a home variable containing an ipaddr enum that's V4 with the address
127.0.0.1?-->

We’ve used a struct to bundle the `kind` and `address` values together: now we
keep the kind with the value itself.

We can represent the same thing in a more efficient way with
just an enum:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

We attach data to each variant of the enum directly, no need for an extra
struct. And beyond that, this approach allows us to attach different kinds of
data to each variant, which the tuple approach does not. Imagine that we would
prefer to store a `V4` as its four individual components rather than a string,
while leaving the `V6` variant as a string. With our struct, we’d be stuck, but
enums deal with this case with ease:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

You can put any kind of data inside of an enum variant, including another enum!


The `IpAddr` enum is in the standard library, but it embeds two
different structs inside of its variants:

<!--- Hm, do you mean the IpAddr enum is stored in the standard library, once
its created? I read this as IpAddr is an in-built enum included is the standard
library --- if so, are we not defining it with the code above? -->

```rust
struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

<!-- I'm not sure what we're saying here, I think we could flesh this out a
little, add some explanation to the examples -->

Here’s an enum with a variety of types embedded in its variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enum has four variants with different types:

* `Quit` has no data associated with it at all.
* `Move` includes an anonymous struct inside of it.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32`s.

You might see this as like different kinds of struct definitions, except
without the `struct` keyword and all grouped together under the `Message` type.
The following structs could hold the same data that the enum variants above
hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

Let's look at another enum in the standard library that is very common and
useful: `Option`.

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

## Match

Rust has an extremely powerful control-flow operator, `match`, that allows us to
compare a value against a series of patterns and then execute code based on
which pattern matches.

Think of a `match` expression kind of like a coin sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls
through the first hole it encounters that it fits into. In the same way, values
go through each pattern in a `match`, and at the first pattern the value
"fits", the value will fall into the associated code block to be used during
execution.

Since we're already talking about coins, let's use them for an example using
`match`! We can write a function that can take an unknown American coin and, in
a similar way as the counting machine, determine which coin it is and
return its value in cents:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Let's break down `match`. At a high-level, using `match` looks like this:

```text
match expression {
    pattern => code,
    pattern => code,
}
```
<!--- Flagging as a place to possibly put wingding numbers -- would it work to
put two arms in this example? I think that would illustrate the control flow
well -->

First, we list the `match` keyword followed by an expression. This feels
very similar to an expression used with `if`, but there's a big difference:
with `if`, the expression needs to return a boolean value. Here, it can be any
type.

Next, we have the *match arms*. An arm has two parts: a pattern and some code.
When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn't match the
value, execution continues to the next arm, much like a coin sorting machine.
We can have as many arms as we need: our `match` above has four arms.

The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets
returned for the entire `match` expression.

Curly braces typically aren't used if the match arm code is short, as it is in
the above example where each arm just returns a value. If we wanted to run
multiple lines of code in a match arm, we can use curly braces. This code would
print out "Lucky penny!" every time the method was called with a `Coin::Penny`,
but would still return the last value of the block, `1`:

```rust
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
### Matching for Attributes

Another useful feature of match arms is that they can create bindings to parts
of the values that match the pattern. This is useful for

<!--- Above, maybe give an explicit example of what we'd use this for -->

From 1999 through 2008, the U.S. printed quarters with different designs for
each of the 50 states on one side. No other coins got state designs, so only
quarters have this extra attribute. We can add this information to our `enum`
by changing the `Quarter` variant to include a `State` value as an argument:

```rust
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Let's imagine that a friend of ours is trying to collect all 50 state quarters.
While we sort our loose change by coin type, we're also going to call out the
name of the state associated with each quarter so that if it's one our friend
doesn't have they can add it to their collection.

In the match statement for this, we add a binding, `state`, to the quarter
variant that contains the value of that quarter's state. The binding will only
be created if the coin matches the `Quarter` pattern. Then we can use the
binding in the code for that arm like so:

```rust
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
will be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.

### Matching with Option<T>

In the previous section we wanted to get the inner `T` value out of the `Some`
case when using Option<T>; we can so this in a very similar way! Instead of
comparing coins we will be comparing patterns, but the way that the `match`
expression works remains the same.

Let's say we want to write a function that takes an `Option<i32>`, and if
there's a value inside, adds one to that value. If there isn't a value inside,
it should return the `None` value and not attempt to perform any operations.

This function is very easy to write, thanks to `match`, and will look like this:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```
<!-- Flagging for wingding numbers -->

#### A Some Match

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. We compare that against each arm:

```text
None => None,
```

The `Some(5)` pattern doesn't match the variant `None`, so we continue.

```text
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. In
that case, the `i` binds to the value contained in `Some`, so `i` takes the
value `5`. The code in the match arm is then executed, so we add one to the new
value of `i` and create a new `Some` value with our total `6` inside.

#### A None Match

Now let's consider the second call of `plus_one()` where `x` is
`None`. We enter the `match`, and compare to the first arm:

```text
None => None,
```

It matches! There's no value to add to, so the program stops and
returns the `None` value on the right side of `=>`. Since
the first arm matched, no other arms are compared.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, bind to the data
inside, and then execute code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in all languages.
It's consistently a user favorite.

### Matches are Exhaustive

There's one other aspect of `match` we haven't discussed. Consider this version
of our `plus_one()` program:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn't handle the `None` case, so this will cause a bug. Luckily, it's a bug
Rust knows how to catch. If we try to compile this code, we'll get this error:

```bash
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => Some(i + 1),
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! Enums in Rust are *exhaustive*: we must exhaust
every last option possible in order to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null and thus making the billion-dollar mistake discussed earlier.

### The _ Placeholder

Rust also has an enum tool for dealing with situations when we don't want to
list all possible values. When there are a lot of possible values for a
type---for example, a `u8` can have valid values of zero through 255---we don't
want to list out 0, 2, 4, 6, 8, 9 all the way up to 255 if we only care about
1, 3, 5, and 7. We can use the special pattern `_` instead:

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The `_` pattern is a placeholder for any value, so by putting it after our
other arms it will match all the possible cases that aren't specified before
it. The `()` syntax is the unit value and will do nothing. This way, we can say
that we want to do nothing for all of the possible values that we don't list
before the `_` placeholder.

## if let With match

<!-- Do we use if let with match, is that why we include it in this chapter?
That would be worth specifying -->

There's one more advanced control flow structure we haven't discussed that's
often used with the `match` enum: `if let`.

Take the following program:

```rust
match some_option {
    Some(x) => {
        // do something with x
    },
    None => {},
}
```

We want to do something with the `Some` match, but do nothing with the `None`
case. We can do this with an `Option`, but with a more complex enum,
adding `_ => {}` after processing just one variant doesn't feel great.
<!-- Could you be more specific about why that's bad, say it explicitly? -->

We have this boilerplate arm and an extra level of indentation for the code
that does something with `x`. We really want a construct that says "Do
something with this one case; do nothing with anything else."

<!-- I'm not totally clear how this is different to the last case--I liked this
line below from the original documentation and added it in, what do you think?
-->

The `if let` syntax lets you combine `if` and `let` to reduce the overhead for
certain kinds of pattern matching, so rather than using `match` we can do the
following:

```rust
if let Some(x) = some_option {
    // do something with x
}
```

<!--- So would we only use this if let pattern when searching for one case and
discarding all others, rather than searching for multiple cases? If so, can you
make that clear early on. What is the advantage of using if let over match,
here, it's more efficient for this one-case situation? -->

`if let` takes a pattern and an expression separated by an `=`. It works
exactly like a `match`, where the expression is given to the `match` and the
pattern is its first arm.

In other words, you can think of `if let` as syntax
sugar:

```rust,ignore
if let pattern = expression {
    body
}

match expression {
   pattern => body,
   _ => {}
}
```

<!-- Can you elaborate on this? -->

If we include an `else` and it becomes the body of the `_`
case:

```rust,ignore
if let pattern = expression {
    body
} else {
    else_body
}

match expression {
   pattern => body,
   _ => else_body,
}
```

<!--- Can you talk this through a little, and perhaps add comments to label one
as mathc and one as if let, just to make it clear you're comparing the two? Why
is the if let else better than match here? -->

In other words, it's the high-level construct we were originally looking for:
do something special with only one pattern, and treat all others the same way.

<!-- Could you add a chapter summary? -->
