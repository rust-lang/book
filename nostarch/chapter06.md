
[TOC]

# Enums

Next, let’s look at *enumerations*, which allow you to define a type by
enumerating its possible values. Commonly called "enums", these unlock a lot of
power in Rust when combined with pattern matching. Enums are a feature that are
in many languages, but what they can do is different per-language. Rust’s enums
are most similar to "algebraic data types" in functional languages like F#,
OCaml, or Haskell.

Here’s an example of an enum definition:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

This enum represents the kind of an IP address. There are two major standards
used for IP addresses: version four and version six. Any IP address can be
either a version four address or a version six address, but it cannot be both
kinds at the same time. This is where enums get their name: they allow us to
enumerate all of the possible kinds that our value can have.

We can create values of `IpAddrKind` like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its name, and we use
the double colon to separate the two.

Enums have more tricks up their sleeves, however. Thinking more about our IP
address type, we don’t have a way to store the actual data of the IP address;
we only know what kind it is. Given that you just learned about structs, you
might tackle this problem like this:

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

We’ve used a struct to bundle the two values together: now we keep the kind
with the value itself. We can represent the same thing in a different way with
just an enum:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

We can attach data to each variant of the enum directly. No need for an extra
struct. But beyond that, this approach is better than using a struct alongside
our enum because we can attach different kinds of data to each variant.
Imagine that instead of a `String`, we would prefer to store a `V4` as its four
individual components while leaving the `V6` variant as a `String`. With our
struct, we’d be stuck. But enums deal with this case with ease:

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

Here’s an enum with a variety of types embedded in its variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

* `Quit` has no data associated with it at all.
* `Move` includes an anonymous struct inside of it.
* `Write` includes a single `String`.
* `ChangeColor` includes three `i32`s.

This might seem overwhelming, but another way to look at the different enum
possibilities is that they are just like different kinds of struct definitions
that you already know, except without the `struct` keyword and they are grouped
together under the `Message` type. These structs could hold the same data that
these enum variants hold:

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

## Option

Now that we have had an introduction to enums, let's combine them with a
feature that we talked a little bit about in the previous chapter: generics.

Programming language design is often thought of as which features you include,
but it's also about which features you leave out. Rust does not have a feature
that is in many other languages: *null*. In languages with this feature,
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

This enum is provided by the standard library, and is so useful that
it's even in the prelude; you don't need to import it explicitly. Furthermore,
so are its variants: you can say `Some` and `None` directly, without prefixing
them with `Option::`.

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
sense, `None` means "null", and `Some` means "not null". So why is this any
better than null?

In short, because `Option<T>` and `T` are different types. That's a bit too
short though. Here's an example:

```rust,ignore
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

This will not compile. We get an error message like this:

```bash
error: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not
satisfied [E0277]

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

This is pretty powerful: in order to have a value that can possibly be null,
you have to explicitly opt in by making the type of that value an `Option<T>`.
Then, when you use that value, you are required to explicitly handle the case
when the value is null. Everywhere that a value has a type that isn't an
`Option<T>`, you *can* safely assume that the value isn't null. This was a
deliberate design decision for Rust to limit null's pervasiveness and increase
the safety of Rust code.

So, how _do_ you get a `T` from an `Option<T>`?  The `Option<T>` enum has a
large number of methods that you can check out in its documentation, and
becoming familiar with them will be extremely useful in your journey with Rust.

But we want a deeper understanding than that. If we didn't have those methods
defined for us already, what would we do? And more generally, how do we get
the inner values out of any enum variant? We need a new feature: `match`.

## Match

Rust has an extremely powerful control-flow operator: `match`. It allows us to
compare a value against a series of patterns and then execute code based on
how they compare.

Think of a `match` expression kind of like a coin sorting machine. Coins slide
down a track that has variously sized holes along it, and each coin falls
through the first hole it encounters that it fits into. In the same way, values
go through each pattern in a `match`, and for the first pattern that the value
"fits", the value will fall into the associated code block to be used during
execution.

Since we're already talking about coins, let's use them for an example using
`match`! We can write a function that can take an unknown American coin and, in
a similar way as the coin counting machine, determine which coin it is and
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

Let's break down the `match`! At a high-level, using `match` looks like this:

```text
match expression {
    pattern => code,
}
```

First, we have the `match` keyword. Next, we have an expression. This feels
very similar to an expression used with `if`, but there's a big difference:
with `if`, the condition needs to return a boolean value. Here, it can be any
type.

Next, we have a "match arm". That's the part that looks like `pattern =>
code,`.  We can have as many arms as we need to: our `match` above has four
arms. An arm has two parts: a pattern and some code. When the `match`
expression executes, it compares the resulting value against the pattern of
each arm, in order. If a pattern matches the value, the code associated
with that pattern is executed. If that pattern doesn't match the value,
execution continues to the next arm, much like a coin sorting machine.

The code associated with each arm is an expression, and the resulting value of
the code with the matching arm that gets executed is the value that gets
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

Another useful feature of match arms is that they can create bindings to parts
of the values that match the pattern. From 1999 through 2008, the U.S. printed
quarters with different designs for each of the 50 states on one side. The other
coins did not get state designs, so only quarters have this extra attribute. We
can add this information to our `enum` by changing the `Quarter` variant to have
a `State` value:

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
While we sort our loose change by coin type in order to count it, we're going
to call out the name of the state so that if it's one our friend doesn't have
yet, they can add it to their collection.

In the match statement to do this, the quarter case now has a binding, `state`,
that contains the value of the state of that quarter. The binding will only get
created if the coin matches the `Quarter` pattern. Then we can use the binding
in the code for that arm:

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

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` will
be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each of the
match arms, none of them match until we reach `Coin::Quarter(state)`. At that
point, the binding for `state` will be the value `UsState::Alaska`. We can then
use that binding in the `println!`, thus getting the inner state value out of
the `Coin` enum variant for `Quarter`.

Remember the `Option<T>` type from the previous section, and that we wanted to
be able to get the inner `T` value out of the `Some` case? This will be very
similar! Instead of coins, we will be comparing to other patterns, but the way
that the `match` expression works remains the same as a coin sorting machine in
the way that we look for the first pattern that fits the value.

Let's say that we want to write a function that takes an `Option<i32>`, and
if there's a value inside, add one to it. If there isn't a value inside, we
want to return the `None` value and not attempt to add.

This function is very easy to write, thanks to `match`. It looks like this:

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

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. Let's compare that against each arm:

```text
None => None,
```

Does `Some(5)` match `None`? No, it's the wrong variant. So let's continue.

```text
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value inside of the `Some`, so `i` has the value `5`. Then we
execute the code in that match arm: take `i`, which is `5`, add one to it, and
create a new `Some` value with our total inside.

Now let's consider the second call of `plus_one()`. In this case, `x` is
`None`. We enter the `match`, and compare to the first arm:

```text
None => None,
```

Does `None` match `None`? Yup! There's no value to add to. So we stop and
return the `None` value that is on the right side of the `=>`. We don't
check any other arms since we found one that matched.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, bind to the data
inside, and then execute code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in languages that don't support
it. It's consistently a user favorite.

### Matches are exhaustive

There's one other aspect of `match` we didn't talk about. Consider this version
of `plus_one()`:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

A bug! We didn't handle the `None` case. Luckily, it's a bug Rust knows how to
catch. If we try to compile this code, we'll get an error:

```bash
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => Some(i + 1),
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! This is referred to as being "exhaustive": we must exhaust
every last option possible in order to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null and thus making the billion-dollar mistake we discussed in the
previous section.

### The _ placeholder

What if we don't care about all of the possible values, though? Especially when
there are a lot of possible values for a type: a `u8` can have valid values of
zero through 255-- if we only care about 1, 3, 5, and 7, does this mean we must
list out 0, 2, 4, 6, 8, 9, all the way up through 255? Thankfully, no! We can
use a special pattern, `_`:

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

The `_` pattern will match all the other cases, and `()` will do nothing, it's
the unit value. This way, we don't have to list individual match arms for all
the other possible values in order to say that we want to do nothing for all of
those-- the `_` is a placeholder for any value.

## if let

There's one more advanced control flow structure we haven't discussed: `if
let`. Imagine we're in a situation like this:

```rust
match some_option {
    Some(x) => {
        // do something with x
    },
    None => {},
}
```

We care about the `Some` case, but don't want to do anything with the `None`
case. With an `Option`, this isn't _too_ bad, but with a more complex enum,
adding `_ => {}` after processing just one variant doesn't feel great. We have
this boilerplate arm and an extra level of indentation (the code that
does something with `x` is indented twice, rather than just once). We really want
a construct that says "Do something with this one case; I don't care about the
others."

Enter `if let`:

```rust
if let Some(x) = some_option {
    // do something with x
}
```

`if let` takes a pattern and an expression, separated by an `=`. It works
exactly like a `match`, where the expression is given to the `match` and the
pattern is its first arm. In other words, you can think of `if let` as syntax
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

And in fact, we can include an `else` and it becomes the body of the `_`
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

In other words, it's the high-level construct we were originally looking for:
do something special with only one pattern.
