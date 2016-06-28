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
#
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










