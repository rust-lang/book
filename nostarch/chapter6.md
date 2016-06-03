# Enums

Next, let’s look at a feature of Rust that’s similar to structs, but also
different. Enumerations, or ‘enums’ as they’re more commonly referred to,
are an extremely powerful feature of Rust. Enums are a feature that are in many
languages, but what they can do is different per-language. Rust’s enums are
most similar to enums in functional languages.

Here’s an example of an enum:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

This enum represents the kind of an IP address. There are two major standards
used for IP addresses: version four, and version six. Any IP address can be either
a version four address, or a version six address. But it cannot be both kinds at
the same time. This is where enums get their name: they allow us to enumerate all
of the possible kinds that our value can have.

We can create values of `IpAddrKind` like this:

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its name, and we use
the double colon to separate the two.

Enums have more tricks up their sleeves, however. Thinking more about our IP
address type, we don’t have a way to store the actual data of the IP address,
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
with the value itself. This design isn’t bad, exactly, but it wouldn’t be
considered idiomatic Rust. We can represent the same thing with just an enum:

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
our enum because we can attatch different kinds of data to each variant.
Imagine that instead of a `String`, we would prefer to store a `V4` as its four
individual components, while leaving the `V6` variant as a `String`. With our
struct, we’d be stuck. But enums deal with this case with ease:

```rust
enum IpAddr {
    V4(u32, u32, u32, u32),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

You can put any kind of data inside of an enum variant, including another enum!
The `IpAddr` enum is [in the standard library][IpAddr], but it embeds two different
structs inside of its variants:

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

[IpAddr]: http://doc.rust-lang.org/std/net/enum.IpAddr.html

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

We haven’t talked a lot about how to access the data inside an enum variant,
however. To do that, let’s move on to some new Rust syntax that’s especially
useful with enums: `match`.










# Option

Now that we have a handle on enums, let's combine them with a feature that we
talked a little bit about in the previous chapter: generics.

Programming language design is often though of as which features you include,
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
a type which can encode the concept of a value being present. We call this type
`Option<T>`, and it looks like this:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

This type is [provided by the standard library][option], and is so useful that
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
shows one of the big advantages of an `Option<T>` type: if you have a type that
may or may not exist, you have to deal with that fact before you can assume it
exists. In other words, you have to convert an `Option<T>` to a `T` before you
can do `T` stuff with it. This helps catch one of the most common issues with
null, generally: assuming that something isn't null, when it actually is.

So, how _do_ you get a `T` from an `Option<T>`?  The option type has a large
number of methods that you can check out in [its documentation], and becoming
familiar with them will be extremely useful in your journey with Rust.

[its documentation]: ../std/option/enum.Option.html

But we want a deeper understanding than that. If we didn't have those methods
defined for us already, what would we do? For that, we need a new feature: `match`.
# Match

Rust has an extremely powerful control-flow operator: `match`. It allows us to
compare a value against a series of patterns, and then execute code based on
how they compare. Remember the `Option<T>` type from the previous section?
Let's say that we want to write a function that takes an `Option<i32>`, and
if there's a value inside, add one to it.

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

Let's break down the `match`! At a high-level, the `match` expression looks
like this:

```text
match condition {
    pattern => code,
}
```

First, we have the `match` keyword. Next, we have a condition. This feels very
similar to an `if` expression, but there's a big difference: with `if`, the
condition needs to be a boolean. Here, it can be any type.

Next, we have a "match arm". That's the part that looks like `pattern =>
code,`.  We can have as many arms as we need to: our `match` above has two
arms. An arm has two parts: a pattern, and some code. When the `match`
expression executes, it compares the condition against the pattern of each arm,
in turn. If the pattern matches the condition, the associated code is executed,
and the rest of the patterns are not checked. If it doesn't match, execution
continues to the next arm.

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. Let's compare that against each arm:

```text
None => None,
```

Does `Some(5)` match `None`? No, it's the wrong variant. So let's continue.

```text
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. But
what about `i`? In a pattern like this, we can declare new bindings, similarly
to what we did with `let`. So in this case, the code part of the match arm will
have a binding, `i`, which corresponds to the `5`.

With this arm, the code portion is `Some(i + 1)`. So we do exactly that: we
take `i`, which is `5`, add one to it, and create a new `Some` value with our
sum inside.

Because `match` is an expression, the value of the overall expression becomes
the value of the arm that executed. So the value of this `match` expression
will be `Some(6)`. And since our `match` is the only expression in the
function, the value of the `match` will be the value of the function, and so
`Some(6)` is our return value as well, which is exactly what we were shooting
for.

Now let's consider the second call. In this case, `x` is `None`. We enter the
`match`, and compare to the first arm:

```text
None => None,
```

Does `None` match `None`? Yup! And so we return `None`. There's no value to add
to.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, binding to the data
inside, and then executing code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in languages that don't support
it. It's consistently a user favorite.

## Matches are exhaustive

There's one other aspect of `match` we didn't talk about. Consider this version
of `plus_one()`:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

A bug! We didn't handle the `None` case. Luckily, it's a bug Rust knows how to catch.
If we try to compile this code, we'll get an error:

```text
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => Some(i + 1),
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! This is referred to as being "exhaustive", we must exhaust
every last option possible in order to be valid!

This analysis isn't perfect, however. This will also error:

```rust,ignore
# let some_u8_value = 0u8;
match some_u8_value {
    0 => println!("zero"),
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    6 => println!("six"),
    7 => println!("seven"),
    // We won't write out all of the arms here, but imagine that there are more
    // arms corresponding to the rest of the numbers.
    254 => println!("two-hundred and fifty-four"),
    255 => println!("two-hundred and fifty-five"),
}
```

Even though a `u8` can only have valid values of zero through 255, Rust isn't
quite smart enough to understand we've covered all the cases. In order to fix
this, we can use a special pattern, `_`:

```rust
# let some_u8_value = 0u8;
match some_u8_value {
    0 => println!("zero"),
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    6 => println!("six"),
    7 => println!("seven"),
    // ...
    254 => println!("two-hundred and fifty-four"),
    255 => println!("two-hundred and fifty-five"),
    _ => panic!("can't ever happen"),
}
```

The `_` pattern matches anything at all, and so with it as the final pattern,
Rust can understand that we have all our bases covered. It's not only used for
this sort of exhastiveness issue, though. It's useful any time we don't want to
deal with a number of cases. Consider this scenario: if we wanted to print out
something one one, three, five, and seven:

```rust
# let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The `_` pattern will match all the other cases, and `()` will do nothing, it's
the unit value.

## More about patterns

As we've just seen, patterns are powerful, yet complex. Let's take a whole
section to cover all of the things that they can do.
# if let

There's one more advanced control flow structure we haven't discussed: `if
let`. Imagine we're in a situation like this:

```rust
# let some_option = Some(5);
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
this boilerplate arm, and we have an extra level of indentation: the code that
does something with `x` is indented twice, rather than just once. We really want
a construct that says "Do something with this one case, I don't care about the
others."

Enter `if let`:

```rust
# let some_option = Some(5);
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
do something with a single pattern.
# Patterns

We've mentioned 'patterns' a few times so far: they're used in `let` bindings,
in function arguments, and in the `match` expression. Patterns have a lot of
abilities, so in this section, we'll cover all of the different things they can
do. Any of these abilities work in any place where a pattern is used.

## Literals & _

You can match against literals directly, and `_` acts as an any case:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one`.

# Multiple patterns

You can match multiple patterns with `|`:

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one or two`.

## ref and ref mut

Usually, when you match against a pattern, bindings are bound by value.
This means you'll end up moving the value out:

```rust,ignore
let name = Some(String::from("Bors"));

match name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

// name is moved here. This line will fail to compile:
println!("name is: {:?}", name);
```

If you'd prefer to bind `name` by reference, use the `ref` keyword:

```rust
let name = Some(String::from("Bors"));

match name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

// name is not moved here; the match only took a reference to its data rather
// than moving it. This will work:
println!("name is: {:?}", name);
```

And for a mutable reference, `ref mut`:

```rust
let mut name = Some(String::from("Bors"));

match name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

// name is not moved here; the match only took a reference to its data rather
// than moving it
```

## Destructuring

Patterns can be used to destructure structs and enums:

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

let Point { x, y } = origin;
```

This brings an `x` and `y` binding into scope, matching the `x` and `y` of
`origin`. While it can be unusual in `let`, this is the same principle of
patterns in `match`:

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x, y } => { }, // x and y are bound here
}
```

## Shadowing

As with all bindings, anything bound by a pattern will shadow bindings
outside of the binding construct:

```rust
let x = Some(5);

match x {
    Some(x) => { }, // x is an i32 here, not an Option<i32>
    None => (),
}
```

## Ignoring bindings

We discussed using `_` as a whole pattern to ignore it above, but you can
also use `_` inside of another pattern to ignore just part of it:

```rust
let x = Some(5);

match x {
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

Or like this:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
}
```

If you want, you can use `..` to ignore all of the parts you haven't defined:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => { }, // y and z are ignored
}
```

## Ranges

You can match a range of values with `...`:

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

Ranges are usually used with integers or `char`s:

```rust
fn main() {
    let x = 'c';
    
    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
```

## Guards

You can introduce match guards with `if`:

```rust
let x = Some(5);

match x {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

If youre using if with multiple patterns, the if applies to both sides:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
}
```

This prints `no`, because the if applies to the whole of `4 | 5`, and not to only
the `5`. In other words, the precedence of if behaves like this:

```text
(4 | 5) if y => ...
```

not this:

```text
4 | (5 if y) => ...
```

## Bindings

You can bind values to names with `@`:
