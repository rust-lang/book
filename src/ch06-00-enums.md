<!-- Hi Steve, Carol. I like this chapter, we're getting into some powerful
tools here! I saw that the copyeditor has suggested listing numbers, which I
think is a good idea. If you agree, could you suggest captions and add
numbering to those listings we reference again in the chapter? Also, if we are
going to include any of these in the source files can you add file names?
Thanks! /Liz -->
<!-- I added some listing numbers where the code examples were lengthy or
referred to again. I haven't added any file names-- the code in this chapter is
little snippets that would be useful in larger programs, but they could appear
anywhere and don't have to be in any particular file. /Carol -->

# Enums

In this chapter we'll look at *enumerations*, also referred to as *enums*.
Enums allow you to define a type by enumerating its possible values. First
we'll define and use an enum to show how an enum can encode meaning along with
data. Then we'll explore a particularly useful enum, `Option`, which expresses
that a value can be either something or nothing. Next we'll look at how pattern
matching in the `match` statement makes it easy to run different code for
different values of an enum. Finally, we'll cover how the `if let` construct is
another convenient and concise idiom you have available to handle enums in your
code.

Enums are a feature in many languages, but their capabilities differ
per-language. Rust’s enums are most similar to "algebraic data types" in
functional languages like F#, OCaml, or Haskell.

## Defining an Enum

<!-- I'm not sure what you meant by "looking inside it" when you said "I wasn't
clear throughout this section whether we were defining the IpAddrKind enum or
looking inside it", but I've tried to clarify. Please elaborate on what you
meant by that and why it's confusing if I haven't resolved the issue. /Carol -->

Let's look at a situation we might want to express in code and see why enums
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
enum, you use an instance of the enum that is one of the variants the enum
allows.

<!-- Option 2: -->
When you want to use a struct, you create an instance of the *struct* itself.
When you want to use an enum, you create an instance of one of its *variants*.
Each variant is defined like a struct, and you instantiate both using the same
syntax.

<!-- end options -->

We can create instances of each of the two variants of `IpAddrKind` like this:

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we
use the double colon to separate the two. The reason this is useful is that now
both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type:
`IpAddrKind`. We can then, for instance, define a function that takes any
`IpAddrKind` as an argument:

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
fn route(ip_type: IpAddrKind) { }
```

And we can call this function with either variant:

```rust
# enum IpAddrKind {
#     V4,
#     V6,
# }
#
# fn route(ip_type: IpAddrKind) { }
#
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Enums have more tricks up their sleeves, too. Thinking more about our IP
address type, at the moment we don’t have a way to store the actual *data* of
the IP address; we only know what *kind* it is. Given that you just learned
about structs, you might tackle this problem as in Listing 6-1:

<figure>

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

<figcaption>

Listing 6-1: Storing the data and type of an IP address using a `struct`

</figcaption>
</figure>

<!-- I will add wingdings here in libreoffice /Carol -->

Here, we've defined a struct `IpAddr` that has two fields: a `kind` field that
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

There's another advantage to using an enum over a struct: each variant can
store *different kinds* of data. Version four type IP addresses will always
have four numeric components that will have values between 0 and 255. If we
wanted to store `V4` addresses as four `u8`s but still express `V6` addresses
as `String`s, we wouldn't be able to with a `struct`. Enums handle this case
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
they are is so common that [the standard library has a definition we can
use!][IpAddr]<!-- ignore --> Let's look at how the standard libary defines
`IpAddr`: it has the exact enum and variants that we've defined and used, but
it chose to embed the address data inside the variants in the form of two
different structs, which are defined differently for each variant:

[IpAddr]: ../std/net/enum.IpAddr.html

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
we haven't brought the standard library's definition into our scope. We'll talk
more about importing types in Chapter 7.

Let's look at another example: here’s an enum with a wide variety of types
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

But if we used the different structs, we wouldn't be able to as easily define a
function that could take any of these kinds of messages as we could with the
`Message` enum defined above.

One more similarity between enums and structs: just as we are able to define
methods on structs using `impl`, we are also able to define methods on enums.
Here's a method, `call`, that we could define on our `Message` enum:

```rust
# enum Message {
#     Quit,
#     Move { x: i32, y: i32 },
#     Write(String),
#     ChangeColor(i32, i32, i32),
# }
#
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
`Message::Write("hello")`, and that is what `self` will be in the body of
the `call` method when `m.call()` runs.

Let's look at another enum in the standard library that is very common and
useful: `Option`.
