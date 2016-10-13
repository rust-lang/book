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
