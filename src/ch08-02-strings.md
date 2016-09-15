# Strings

We've already talked about strings a bunch in chapter four, but let's take a
more in-depth look at them now.

## Six(?) kinds of strings

Strings are a common place for new Rustaceans to get stuck. This is due to a
combination of three things: Rust's propensity for making sure to expose
possible errors, that strings are a more complicated data structure than many
programmers give them credit for, and UTF-8. These things combine in a way that
can seem difficult when you're used to other languages.

Before we can dig into those things, we need to talk about what exactly we even
mean by the word 'string'. Rust-the-language has only one string type: `&str`.
We talked about these string slices in chapter four: they're a reference to some
UTF-8 encoded string data stored somewhere else. String literals, for example,
are stored in the binary output of your program, and are therefore string
slices.

Rust's standard library also provides a type called `String`. This is a growable,
mutable, UTF-8 encoded string type. When Rustaceans talk about 'strings' in Rust,
they usually mean "`String` and `&str`." This chapter is largely about `String`,
and these two types are used heavily in Rust's standard library. This is what
Rustaceans mean when they say "Rust strings are UTF-8," since both `String` and
string slices are UTF-8 encoded.

Rust's standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates may provide even
more options for string string data. As you can see from the `*String`/`*Str`
naming, they often provide an owned and borrowed variant, just like
`String`/`&str`. These string types may store different encodings, be
represented in memory in a different way, or all kinds of other things. We
won't be talking about them in this chapter, see their API documentation for
more about how to use them, and when each is appropriate.

Many options! As I said, strings are surprisingly complex. Most of the time,
we'll be using `String` as an owned string type, though. So let's talk about
how to use it.

## Creating

You can create an empty string with `new`:

```rust
let s = String::new();
```

Often, you have some initial data that you'd like to start the string off with.
For that, there's the `.to_string()` method:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

You'll also see this form sometimes:

```rust
let s = String::from("Initial contents");
```

Because strings are used for so many things, there are many different generic
APIs that make sense for strings. There are a lot of options, and some of them
can feel redundant because of this, but they all have their place! In this
case, `String::from` and `.to_string` end up doing the exact same thing, so
which you choose is a matter of style. Some people use `String::from` for
literals, and `.to_string` for variables bindings. Most Rust style is pretty
uniform, but this specific question is one of the most-debated.

Don't forget that strings are UTF-8 encoded, and so you can include any
properly encoded data in them:

```rust
let hello = "السلام عليكم";
let hello = "Dobrý den";
let hello = "Hello";
let hello = "שָׁלוֹם";
let hello = "नमस्ते";
let hello = "こんにちは";
let hello = "안녕하세요";
let hello = "你好";
let hello = "Olá";
let hello = "Здравствуйте";
let hello = "Hola";
```

## Updating

You can grow a `String` by using the `push_str` method to append another
string:

```rust
let mut s = String::from("foo");

s.push_str("bar");

// s will be "foobar" here
```

And `push` will add a `char`:

```rust
let mut s = String::from("lo");

s.push('l');

// s will be "lol" here
```

You can make any string into the empty string with the `clear` method:

```rust
let mut s = String::from("Noooooooooooooooooooooo!");

s.clear();

// s will be "" here
```

### Concatenation

Often, you'll want to combine two strings together. One way is to use the `+`
operator:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2;

// s3 is "Hello, world!"
```

There's some tricky bits here, though! They come through the type signature
of `+` for strings. It looks something like this:

```rust,ignore
fn add(self, s: &str) -> String {
```

I say 'something' because `add` is generic, and so this is what the signature
would be if it isn't. But this signature gives us all the clues we need about
the tricky bits of `+`.

First of all, you'll notice that `s2` has an `&`. This is because of the `s`
argument in the function: you can only add a `&str` to a `String`, you can't
add two `String`s together. Remember back in chpater four when we talked about
how `&String` will coerce to `&str`? That's why it's `&s2`: so that it will
coerce to the proper type.

Secondly, `add` takes ownership of `self`. This means that `s1` in the above
example will move. So while `let s3 = s1 + &s2;` looks like it will copy the
two strings and create a new one, it actually takes ownership of `s1`, appends
a copy of `s2`'s contents, and then returns ownership back. In other words, it
looks like it's making a lot of copies, but isn't.

If you need to concatenate multiple strings, this behavior of `+` gets
unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// s will be 'tic-tac-toe' here
```

With all of these `+`s and `"`s, it gets hard to see what's going on. For more
complicated string combining, we can use the `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);

// s will be 'tic-tac-toe' here
```

The `format!` macro works in the same way as `println!`, but instead of
printing the output to the screen, it returns a `String` with the contents.
This version is much easier to read than all of the `+`s.

## Indexing strings

If you try to access a string with the indexing syntax, you'll get an error. In
other words, this:

```rust,ignore
let s1 = String::from("hello");

let h = s1[0];
```

will give you this:

```text
error: the trait bound `std::string::String: std::ops::Index<_>` is not satisfied [--explain E0277]
 --> <anon>:4:14
  |>
4 |>     let s3 = s1[0];
  |>              ^^^^^
note: the type `std::string::String` cannot be indexed by `_`
```

That note tells the story: Rust strings don't support indexing like this. So
the follow-up question is, why not? In order to answer that, we have to talk a
bit about how Rust stores strings in memory.

### Internal representation

A `String` is a wrapper over a `Vec<u8>`. Let's take a look at some of our
examples from before. First, this one:

```rust
let len = "Hola".len();
```

In this case, `len` will be four bytes long: each of these letters takes one
byte when encoded in UTF-8. What about this one, though?

```rust
let len = "Здравствуйте".len();
```

There are two answers that make sense here: the first is 12, which is the number
of letters that it would be if you asked someone. The second, though, is the
real answer here: 24. This is the number of bytes that it takes to encode
"Здравствуйте" in UTF-8: each character is two bytes.

By the same token, imagine this invalid Rust code:

```rust,ignore
let hello = "Здравствуйте";

let answer = &h[0];
```

What should the value of `answer` be? Should it be `З`, the first letter? Or
should it be `208`? When you encode `З` in UTF-8, the first byte is `208`, and
the second is `151`. If it's `208`, well, that's not a valid character on its
own. So... do we make `[]` return an integer? For latin letters, this would
then not return the answer you'd expect: `&"hello"[0]` would then give `104`,
not `h`.

This leads to another point about UTF-8: there are really three relevant ways
to look at strings, from Rust's perspective: bytes, scalar values, and grapheme
clusters. If we look at "नमस्ते ", it ultimately boils down to:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135, 32]
```

That's 19 bytes. But if you look at them as Unicode scalar values, which are what
Rust's `char` type is, those bytes look like this:

```text
['न', 'म', 'स', '्', 'त', 'े', ' ']
```

There are seven of them, and the last one isn't even visible! Finally, if you
look at them as grapheme clusters, which is the closest thing to what humans
would call 'letters', you'd get this:

```text
["न", "म", "स्", "ते", " "]
```

Five elemtns, and there's still that empty character on the end. It turns out
that even within 'grapheme cluster', there are multiple ways of grouping
things. Have we convinced you strings are actually really complicated yet?

Furthermore, `[]` implies *O(n)* access time. But because UTF-8 is a
variable-length encoding, implementing `[]` on strings would be *O(n)* instead,
which would make it significantly worse-performing than what people expect.

All of these problems means that we decided to not implement `[]` for strings, so
you cannot directly do this.

However.

Sometimes, indexing the bytes of a string is useful. So while you can't use `[]`
with a single number, you _can_ use `[]` with a range:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first four bytes of the string.
Earlier, we mentioned that each of these characters was two bytes, so that means
that `s` will be 'Зд'.

What would happen if we did `&hello[0..1]`? We said each of these characters
required two bytes. The answer: it will panic, in the same way that accessing
an invalid index in a vector does.
