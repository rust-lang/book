
[TOC]

<!-- Hi Steve, Carol. I like this chapter, we're getting into some powerful
tools here! I saw that the copyeditor has suggested listing numbers, which I
think is a good idea. If you agree, could you suggest captions and add
numbering to those listings we reference again in the chapter? Also, if we are
going to include any of these in the source files can you add file names?
Thanks! /Liz -->
<!-- I added some listing numbers where the code examples were lengthy or
referred to again. I haven’t added any file names-- the code in this chapter is
little snippets that would be useful in larger programs, but they could appear
anywhere and don’t have to be in any particular file. /Carol -->

# Enums

In this chapter we’ll look at *enumerations*, also referred to as *enums*.
Enums allow you to define a type by enumerating its possible values. First
we’ll define and use an enum to show how an enum can encode meaning along with
data. Then we’ll explore a particularly useful enum, `Option`, which expresses
that a value can be either something or nothing. Next we’ll look at how pattern
matching in the `match` statement makes it easy to run different code for
different values of an enum. Finally, we’ll cover how the `if let` construct is
another convenient and concise idiom you have available to handle enums in your
code.

Enums are a feature in many languages, but their capabilities differ
per-language. Rust’s enums are most similar to “algebraic data types” in
functional languages like F#, OCaml, or Haskell.

## Defining an Enum

<!-- I'm not sure what you meant by "looking inside it" when you said "I wasn't
clear throughout this section whether we were defining the IpAddrKind enum or
looking inside it”, but I’ve tried to clarify. Please elaborate on what you
meant by that and why it’s confusing if I haven’t resolved the issue. /Carol -->

Let’s look at a situation we might want to express in code and see why enums
are useful and more appropriate than structs in this case. Say we need to work
with IP addresses. There are two major standards used for IP addresses today:
version four and version six. These are the only possibilities for an IP
address that our program will come across: we can *enumerate* all possible
values, which is where *enumeration* gets its name.

Any IP address can be either a version four or a version six address, but not
both at the same time. That property of IP addresses makes the enum data
structure appropriate for this case, since enum values can only be one of the
variants. Both version four and version six addresses are still fundamentally
IP addresses, though, so they should be treated as the same type when the code
is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an `IpAddrKind` enumeration and
listing the possible kinds an IP address can be, `V4` and `V6`. These are known
as the *variants* of the enum:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

This is now a custom data type that we can use elsewhere in our code.

### Enum Values

<!-- Liz: You seemed confused at this point about the differences between an
enum's definition, which includes its valid variants, and using the values of
the enum. You had changed this text to be:

"Enum variants can optionally have associated values. We can create values of
`IpAddrKind` like this:"

While it's strictly true that enum values are "optional", there wouldn't be any
point in defining the enum unless you were going to use values of that type.
Also, "associated" has other meanings in Rust that we don't want to conflate
with.

We've tried to clear up the confusion here by relating enum definition and
instantiation to struct definition and instantiation, assuming the reader
understands structs at this point. We're having trouble figuring out just the
right wording here, though, so we have two options for you. Please let us
know which is clearest, or a combination of the two, or if you have any
suggestions in a totally different direction! /Carol -->

<!-- Option 1: -->
An `enum` definition is similar to a `struct` definition: it defines a new type
and a template of what instances of that new type will be like. When you want to
use a struct, you create an instance of the struct. When you want to use an
enum, you use an instannce of the enum that is one of the variants the enum
allows.

<!-- Option 2: -->
When you want to use a struct, you create an instance of the *struct* itself.
When you want to use an enum, you create an instance of one of its *variants*.
Each variant is defined like a struct, and you instantiate both using the same
syntax.

<!-- end options -->

We can create instances of each of the two variants of `IpAddrKind` like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we
use the double colon to separate the two. The reason this is useful is that now
both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type:
`IpAddrKind`. We can then, for instance, define a function that takes any
`IpAddrKind` as an argument:

```rust
fn route(ip_type: IpAddrKind) { }
```

And we can call this function with either variant:

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Enums have more tricks up their sleeves, too. Thinking more about our IP
address type, at the moment we don’t have a way to store the actual *data* of
the IP address; we only know what *kind* it is. Given that you just learned
about structs, you might tackle this problem as in Listing 6-1:

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

<caption>
Listing 6-1: Storing the data and type of an IP address using a `struct`
</caption>

<!-- I will add wingdings here in libreoffice /Carol -->

Here, we've defined a struct `IPAddr` that has two fields: a `kind` field that
is of type `IpAddrKind` (the enum we defined previously), and an `address`
field of type `String`. We have two instances of this struct. The first,
`home`, has the value `IpAddrKind::V4` as its `kind`, with associated address
data of `127.0.0.1`. The second instance, `loopback`, has the other variant of
`IpAddrKind` as its `kind` value, `V6`, and has address `::1` associated with
it. We’ve used a struct to bundle the `kind` and `address` values together, so
that now the kind is associated with the value itself.

We can represent the same concept in a more concise way using just an enum
rather than an enum as part of a struct by putting data directly into each enum
variant. This new definition of the `IpAddr` enum says that both `V4` and `V6`
variants will have associated `String` values:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

We attach data to each variant of the enum directly, no need for an extra
struct.

There’s another advantage to using an enum over a struct: each variant can
store *different kinds* of data. Version four type IP addresses will always
have four numeric components that will have values between 0 and 255. If we
wanted to store `V4` addresses as four `u8`s but still express `V6` addresses
as `String`s, we wouldn’t be able to with a `struct`. Enums handle this case
with ease:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

We've been showing a bunch of different possibilities that we could define in
our code for storing IP addresses of the two different kinds using an enum. It
turns out, though, that wanting to store IP addresses and encode which kind
they are is so common that the standard library has a definition we can use!
Let's look at how the standard library defines `IpAddr`: it has the exact enum
and variants that we've defined and used, but it chose to embed the address
data inside the variants in the form of two different structs, which are
defined differently for each variant:

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

This illustrates you can put any kind of data inside of an enum variant:
strings, numeric types, structs, and you could even include another enum! Also,
standard library types are often not much more complicated than what you might
come up with.

Note that even though the standard library contains a definition for `IpAddr`,
we can still choose to create and use our own definition without conflict since
we haven’t brought the standard library’s definition into our scope. We’ll talk
more about importing types in Chapter 7.

Let’s look at another example: here’s an enum with a wide variety of types
embedded in its variants:

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

This is similar to different kinds of struct definitions, except without the
`struct` keyword and all grouped together under the `Message` type. The
following structs could hold the same data that the enum variants above hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, we wouldn’t be able to as easily define a
function that could take any of these kinds of messages as we could with the
`Message` enum defined above.

One more similarity between enums and structs: just as we are able to define
methods on structs using `impl`, we are also able to define methods on enums.
Here’s a method, `call`, that we could define on our `Message` enum:

```rust
impl Message {
    fn call(&self) {
        // body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

<!-- I will add wingdings here /Carol -->

The body of the method would use `self` to get the value that we called the
method on. In this example, we've created a variable `m` that has the value
`Message::Write(“hello”)`, and that is what `self` will be in the body of
the `call` method when `m.call()` runs.

Let's look at another enum in the standard library that is very common and
useful: `Option`.

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
`Option<T>`, and it is defined by the standard library
as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you
don’t need to import it explicitly. Furthermore, so are its variants: you can
use `Some` and `None` directly, without prefixing them with `Option::`. This is
still just a regular enum, however, `Some(T)` and `None` are still values of
type `Option<T>`.

<!-- We haven't spoken about the prelude so far in the book, I think I made a
note of that in a previous chapter---we should tell the reader what it is
before mentioning it so they know what significance it has here -->

<!-- We did speak about the prelude previously, in chapter 2, the Processing a
Guess section. I don’t have any comments from you about it there... /Carol -->

The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a
generic type parameter, and we’ll cover generics in more detail in Chapter 10.
For now, all you need to know is that this means the `Some` variant of the
`Option` enum can hold one piece of data of any type. Here are some examples of
using `Option` values to hold number types and string types:

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

If we use `None` rather than `Some`, we need to tell Rust what type of
`Option<T>` we have.

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

```bash
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

So, how _do_ you get the `T` value out of a `Some` variant when you have a
value of type `Option<T>` so that you can use that value? The `Option<T>` enum
has a large number of methods useful in a variety of situations that you can
check out in its documentation, and becoming familiar
with them will be extremely useful in your journey with Rust.

What we generally want to do in order to use an `Option<T>` value is to have
code that will handle each variant. We want some code that will run only in the
case that we have a `Some(T)` value, and this code _is_ allowed to use the
inner `T`. We want some _other_ code to run if we have a `None` value, and that
code doesn't have a `T` value available. The `match` expression is a control
flow construct that does just this, when used with enums: it will run different
code depending on which variant of the enum it has, and that code can use the
data inside the matching value.

<!-- I'm not sure about this connecting paragraph, it doesn't seem like match
and option are actually that much connected, at least not at first. That's
fine, this is all under the enum heading, but it might confuse if we imply
otherwise --- unless I'm just missing the connection -->
<!-- I've tried to make the connection more explicit, is this better? /Carol -->

## Match

Rust has an extremely powerful control-flow operator, `match`, that allows us to
compare a value against a series of patterns and then execute code based on
which pattern matches. The power comes from the expressiveness of the patterns
and the compiler checks that make sure all possible cases are handled.

Think of a `match` expression kind of like a coin sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls
through the first hole it encounters that it fits into. In the same way, values
go through each pattern in a `match`, and at the first pattern the value
"fits", the value will fall into the associated code block to be used during
execution.

Since we're already talking about coins, let's use them for an example using
`match`! We can write a function that can take an unknown American coin and, in
a similar way as the counting machine, determine which coin it is and
return its value in cents, shown here in Listing 6-2:

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

<caption>
Listing 6-2: An enum and a `match` expression that has the variants of the enum
as its patterns.
</caption>

<!--- Flagging as a place to possibly put wingding numbers -- would it work to
put two arms in this example? I think that would illustrate the control flow
well -->
<!-- I think we're moving away from using generic examples like this and talking
about concrete examples instead. I’ve changed the text to reflect that, and I’m
happy to add wingdings once we’re in libreoffice. /Carol -->

Let’s break down the `match` in the `value_in_cents` function. First, we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This feels very similar to an expression used with `if`, but there’s a
big difference: with `if`, the expression needs to return a boolean value.
Here, it can be any type. The type of `coin` in this example is the `Coin` enum
that we have defined above.

Next, we have the *match arms*. An arm has two parts: a pattern and some code.
The first arm here has a pattern that is the value `Coin::Penny`, then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.

When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much like a coin sorting machine.
We can have as many arms as we need: our `match` above has four arms.

The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.

Curly braces typically aren’t used if the match arm code is short, as it is in
the above example where each arm just returns a value. If you wanted to run
multiple lines of code in a match arm, you can use curly braces. For example,
this code would print out “Lucky penny!“ every time the method was called with
a `Coin::Penny`, but would still return the last value of the block, `1`:

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

### Patterns that Bind to Values

Another useful feature of match arms is that they can bind to parts of the
values that match the pattern. This is how we can extract values out of enum
variants.

As an example, let's change one of our enum variants to hold data inside it.
From 1999 through 2008, the U.S. printed quarters with different designs for
each of the 50 states on one side. No other coins got state designs, so only
quarters have this extra value. We can add this information to our `enum`
by changing the `Quarter` variant to include a `State` value stored inside it
as we've done here in Listing 6-3:

```rust
#[derive(Debug)] // So we can inspect the state in a minute
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

<caption>
Listing 6-3: A `Coin` enum where the `Quarter` variant also holds a `UsState`
value
</caption>

Let’s imagine that a friend of ours is trying to collect all 50 state quarters.
While we sort our loose change by coin type, we’re also going to call out the
name of the state associated with each quarter so that if it’s one our friend
doesn’t have they can add it to their collection.

In the match expression for this, we add a variable, `state`, to the pattern
that matches values of the variant `Coin::Quarter`. When a `Coin::Quarter`
matches, the `state` variable will bind to the value of that quarter’s state.
Then we can use `state` in the code for that arm like so:

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
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.

### Matching with Option<T>

In the previous section we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can do this in a very similar way! Instead of
comparing coins we will be comparing the variants of `Option<T>`, but the way
that the `match` expression works remains the same.

Let's say we want to write a function that takes an `Option<i32>` and if
there's a value inside, adds one to that value. If there isn't a value inside,
the function should return the `None` value and not attempt to perform any
operations.

This function is very easy to write, thanks to `match`, and will look like
Listing 6-4:

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

<caption>
Listing 6-4: A function that uses a `match` expression on an `Option<i32>`
</caption>

<!-- Flagging for wingding numbers -->

#### Matching `Some(T)`

Let’s examine the first execution of `plus_one` in more detail. In the above
example when we call `plus_one(five)`, the variable `x` in the body of
`plus_one` will have the value `Some(5)`. We compare that against each match
arm:

```rust,ignore
None => None,
```

The `Some(5)` value doesn't match the pattern `None`, so we continue.

```rust,ignore
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value contained in `Some`, so `i` takes the value `5`. The
code in the match arm is then executed, so we add one to the value of `i`
and create a new `Some` value with our total `6` inside.

#### Matching `None`

Now let’s consider the second call of `plus_one` where `x` is `None`. We
enter the `match`, and compare to the first arm:

```rust,ignore
None => None,
```

It matches! There's no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Since the first arm matched, no other
arms are compared.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, then execute code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in all languages. It's
consistently a user favorite.

### Matches are Exhaustive

There's one other aspect of `match` we haven't discussed. Consider this version
of our `plus_one` function:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn’t handle the `None` case, so this will cause a bug. Luckily, it’s a bug
Rust knows how to catch. If we try to compile this code, we’ll get this error:

```bash
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! Enums in Rust are *exhaustive*: we must exhaust every last
option possible in order to be valid. Especially in the case of `Option<T>`,
when Rust prevents us from forgetting to explicitly handle the `None` case, it
protects us from assuming that we have a value when we might have null and thus
making the billion-dollar mistake discussed earlier.

### The _ Placeholder

Rust also has a pattern we can use in situations when we don't want to list all
possible values. For example, a `u8` can have valid values of zero through 255.
If we only care about the values 1, 3, 5, and 7, we don't want to have to list
out 0, 2, 4, 6, 8, 9 all the way up to 255. Thankfully, we don't have to: we
can use the special pattern `_` instead.

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

The `_` pattern will match any value. By putting it after our other arms, the
`_` will match all the possible cases that aren’t specified before it. The `()`
is just the unit value, so nothing will happen in the `_` case. This way, we
can say that we want to do nothing for all of the possible values that we don’t
list before the `_` placeholder.

The `match` expression can be a little wordy for the case where we only care
about *one* of the cases, though. For that case, Rust provides `if let`.

## Concise control flow with `if let`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to
handle values that match one pattern and ignoring the rest. Take the following
program:

```rust
match some_option {
    Some(x) => {
        // do something with x
    },
    None => (),
}
```

We want to do something with the `Some` match, but do nothing with the `None`
case. We can do this with an `Option`, but with a more complex enum,
adding `_ => ()` after processing just one variant is a lot of boilerplate code
that we have to add to satisfy the `match` expression.

Instead, we could write this in a shorter way with `if let`. This code behaves
exactly the same as the `match` above:

```rust
if let Some(x) = some_option {
    // do something with x
}
```

`if let` takes a pattern and an expression separated by an `=`. It works
just like a `match`, where the expression is given to the `match` and the
pattern is its first arm.

Using `if let` means you have less to type, less indentation, and less
boilerplate. However, we’ve lost the exhaustiveness checking that `match`
enforces. Choosing between `match` and `if let` depends on what you’re doing in
your particular case, and if gaining conciseness is an appropriate tradeoff for
losing exhaustiveness checking.

In other words, you can think of `if let` as syntax sugar for a `match` that
runs code when the value matches one pattern and then ignores all other values.

We can include an `else` that goes with an `if let`. The block of code that
goes with the `else` is the same as the block of code that would go with the
`_` case in the `match` expression that is equivalent to the `if let` and
`else`. Recall the `Coin` enum definition in Listing 6-3, where the `Quarter`
variant also held a `UsState` value. If we wanted to count all non-quarter
coins we see while also announcing the state of the quarters, we could do that
with a `match` expression like this:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Or we could choose to use an `if let` and `else` expression like this:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

If you find yourself in a situation where your program has logic that is
verbose to express using a `match`, remember that `if let` is in your Rust
toolbox as well.

## Summary

We’ve now covered how to use enums to create custom types that can be one of a
set of enumerated values. We’ve shown how the standard library’s `Option<T>`
type helps you use the type system to prevent errors. When enum values have data
inside them, you can use `match` or `if let` to extract and use those values,
depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and
enums. Creating custom types to use in your API ensures type safety: the
compiler will make certain your functions only get values of the type each
function expects.

In order to provide a well-organized API to your users that is straightforward
to use and only exposes exactly what your users will need, let’s now turn to
Rust’s *modules*.
