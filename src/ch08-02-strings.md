## Strings

We've already talked about strings a bunch in Chapter 4, but let's take a more
in-depth look at them now. Strings are an area that new Rustaceans commonly get
stuck on. This is due to a combination of three things: Rust's propensity for
making sure to expose possible errors, strings being a more complicated data
structure than many programmers give them credit for, and UTF-8. These things
combine in a way that can seem difficult when coming from other languages.

The reason Strings are in the collections chapter is that strings are
implemented as a collection of bytes plus some methods to provide useful
functionality when those bytes are interpreted as text. In this section, we'll
talk about the operations on `String` that every collection type has, like
creating, updating, and reading. We'll also discuss the ways in which `String`
is different than the other collections, namely how indexing into a `String` is
complicated by the differences in which people and computers interpret `String`
data.

### What is a String?

Before we can dig into those aspects, we need to talk about what exactly we
mean by the term 'string'. Rust actually only has one string type in the core
language itself: `str`, the string slice, which is usually seen in its borrowed
form, `&str`. We talked about *string slices* in Chapter 4: these are a
reference to some UTF-8 encoded string data stored elsewhere. String literals,
for example, are stored in the binary output of the program, and are therefore
string slices.

The type called `String` is provided in Rust's standard library rather than
coded into the core language, and is a growable, mutable, owned, UTF-8 encoded
string type. When Rustaceans talk about 'strings' in Rust, they usually mean
both the `String` and the string slice `&str` types, not just one of those.
This section is largely about `String`, but both these types are used heavily
in Rust's standard library. Both `String` and string slices are UTF-8 encoded.

Rust's standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates may provide even
more options for storing string data. Similar to the `*String`/`*Str` naming,
they often provide an owned and borrowed variant, just like `String`/`&str`.
These string types may store different encodings or be represented in memory in
a different way, for example. We won't be talking about these other string
types in this chapter; see their API documentation for more about how to use
them and when each is appropriate.

### Creating a New String

Many of the same operations available with `Vec` are available with `String` as
well, starting with the `new` function to create a string, like so:

```rust
let s = String::new();
```

This creates a new empty string called `s` that we can then load data into.

Often, we'll have some initial data that we'd like to start the string off
with. For that, we use the `to_string` method, which is available on any type
that implements the `Display` trait, which string literals do:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

This creates a string containing `initial contents`.

We can also use the function `String::from` to create a `String` from a string
literal. This is equivalent to using `to_string`:

```rust
let s = String::from("initial contents");
```

Because strings are used for so many things, there are many different generic
APIs that can be used for strings, so there are a lot of options. Some of them
can feel redundant, but they all have their place! In this case, `String::from`
and `.to_string` end up doing the exact same thing, so which you choose is a
matter of style.

Remember that strings are UTF-8 encoded, so we can include any properly encoded
data in them:

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

### Updating a String

A `String` can grow in size and its contents can change just like the
contents of a `Vec`, by pushing more data into it. In addition, `String` has
concatenation operations implemented with the `+` operator for convenience.

#### Appending to a String with Push

We can grow a `String` by using the `push_str` method to append a string slice:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

`s` will contain "foobar" after these two lines. The `push_str` method takes a
string slice because we don't necessarily want to take ownership of the
parameter. For example, it would be unfortunate if we weren't able to use `s2`
after appending its contents to `s1`:

```rust
let mut s1 = String::from("foo");
let s2 = String::from("bar");
s1.push_str(&s2);
```

The `push` method is defined to have a single character as a parameter and add
it to the `String`:

```rust
let mut s = String::from("lo");
s.push('l');
```

After this, `s` will contain "lol".

#### Concatenation with the + Operator or the `format!` Macro

Often, we'll want to combine two existing strings together. One way is to use
the `+` operator like this:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
```

After this code the String `s3` will contain `Hello, world!`. The reason that
`s1` is no longer valid after the addition and the reason that we used a
reference to `s2` has to do with the signature of the method that gets called
when we use the `+` operator. The `+` operator uses the `add` method, whose
signature looks something like this:

```rust,ignore
fn add(self, s: &str) -> String {
```

This isn't the exact signature that's in the standard library; there `add` is
defined using generics. Here, we're looking at the signature of `add` with
concrete types substituted for the generic ones, which is what happens when we
call this method with `String` values. This signature gives us the clues we
need to understand the tricky bits of the `+` operator.

First of all, `s2` has an `&`, meaning that we are adding a *reference* of the
second string to the first string. This is because of the `s` parameter in the
`add` function: we can only add a `&str` to a `String`, we can't add two
`String`s together. Remember back in Chapter 4 when we talked about how
`&String` will coerce to `&str`: we write `&s2` so that the `String` will
coerce to the proper type, `&str`. Because this method does not take ownership
of the parameter, `s2` will still be valid after this operation.

Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does *not* have an `&`. This means `s1` in the above example
will be moved into the `add` call and no longer be valid after that. So while
`let s3 = s1 + &s2;` looks like it will copy both strings and create a new one,
this statement actually takes ownership of `s1`, appends a copy of `s2`'s
contents, then returns ownership of the result. In other words, it looks like
it's making a lot of copies, but isn't: the implementation is more efficient
than copying.

If we need to concatenate multiple strings, the behavior of `+` gets unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

`s` will be "tic-tac-toe" at this point. With all of the `+` and `"`
characters, it gets hard to see what's going on. For more complicated string
combining, we can use the `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

<!-- Are we going to discuss the format macro elsewhere at all? If not, some
more info here might be good, this seems like a really useful tool. Is it only
used on strings? -->

<!-- No, we weren't planning on it. We thought it would be sufficient to
mention that it works the same way as `println!` since we've covered how
`println!` works in Ch 2, "Printing Values with `println!` Placeholders" and Ch
5, Ch 5, "Adding Useful Functionality with Derived Traits". `format!` can be
used on anything that `println!` can; using `{}` in the format string works
with anything that implements the `Display` trait and `{:?}` works with
anything that implements the `Debug` trait. Do you have any thoughts on how we
could make the similarities with `format!` and `println!` clearer than what we
have in the next paragraph without repeating the `println!` content too much?
/Carol -->

This code will also set `s` to "tic-tac-toe". The `format!` macro works in the
same way as `println!`, but instead of printing the output to the screen, it
returns a `String` with the contents. This version is much easier to read, and
also does not take ownership of any of its parameters.

### Indexing into Strings

In many other languages, accessing individual characters in a string by
referencing them by index is a valid and common operation. In Rust, however, if
we try to access parts of a `String` using indexing syntax, we'll get an error.
That is, this code:

```rust,ignore
let s1 = String::from("hello");
let h = s1[0];
```

will result in this error:

```text
error: the trait bound `std::string::String: std::ops::Index<_>` is not
satisfied [--explain E0277]
  |>
  |>     let h = s1[0];
  |>             ^^^^^
note: the type `std::string::String` cannot be indexed by `_`
```

The error and the note tell the story: Rust strings don't support indexing. So
the follow-up question is, why not? In order to answer that, we have to talk a
bit about how Rust stores strings in memory.

#### Internal Representation

A `String` is a wrapper over a `Vec<u8>`. Let's take a look at some of our
properly-encoded UTF-8 example strings from before. First, this one:

```rust
let len = String::from("Hola").len();
```

In this case, `len` will be four, which means the `Vec` storing the string
"Hola" is four bytes long: each of these letters takes one byte when encoded in
UTF-8. What about this example, though?

```rust
let len = String::from("Здравствуйте").len();
```

A person asked how long the string is might say 12. However, Rust's answer
is 24. This is the number of bytes that it takes to encode "Здравствуйте" in
UTF-8, since each character takes two bytes of storage. Therefore, an index
into the string's bytes will not always correlate to a valid character.

To demonstrate, consider this invalid Rust code:

```rust,ignore
let hello = "Здравствуйте";
let answer = &hello[0];
```

What should the value of `answer` be? Should it be `З`, the first letter? When
encoded in UTF-8, the first byte of `З` is `208`, and the second is `151`, so
`answer` should in fact be `208`, but `208` is not a valid character on its
own. Returning `208` is likely not what a person would want if they asked for
the first letter of this string, but that's the only data that Rust has at byte
index 0. Returning the byte value is probably not what people want, even with
only Latin letters: `&"hello"[0]` would return `104`, not `h`. To avoid
returning an unexpected value and causing bugs that might not be discovered
immediately, Rust chooses to not compile this code at all and prevent
misunderstandings earlier.

#### Bytes and Scalar Values and Grapheme Clusters! Oh my!

This leads to another point about UTF-8: there are really three relevant ways
to look at strings, from Rust's perspective: as bytes, scalar values, and
grapheme clusters (the closest thing to what people would call 'letters').

If we look at the Hindi word "नमस्ते" written in the Devanagari script, it is
ultimately stored as a `Vec` of `u8` values that looks like this:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

That's 18 bytes, and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust's `char` type is, those
bytes look like this:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters,
they're diacritics that don't make sense on their own. Finally, if we look at
them as grapheme clusters, we'd get what a person would call the four letters
that make up this word:

```text
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.

A final reason Rust does not allow you to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). It isn't possible to guarantee that performance with a `String`,
though, since Rust would have to walk through the contents from the beginning
to the index to determine how many valid characters there were.

All of these problems mean that Rust does not implement `[]` for `String`, so
we cannot directly do this.

### Slicing Strings

However, indexing the *bytes* of a string is very useful, and is not expected
to be fast. While we can't use `[]` with a single number, we _can_ use `[]`
with a range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first four bytes of the string.
Earlier, we mentioned that each of these characters was two bytes, so that
means that `s` will be "Зд".

What would happen if we did `&hello[0..1]`? The answer: it will panic at
runtime, in the same way that accessing an invalid index in a vector does:

```text
thread 'main' panicked at 'index 0 and/or 1 in `Здравствуйте` do not lie on
character boundary', ../src/libcore/str/mod.rs:1694
```

You should use this with caution, since it can cause your program to crash.

### Methods for Iterating Over Strings

Luckily, there are other ways we can access elements in a String.

If we need to perform operations on individual characters, the best way to do
so is to use the `chars` method. Calling `chars` on "नमस्ते" separates out and
returns six values of type `char`, and you can iterate over the result in order
to access each element:

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

This code will print:

```text
न
म
स
्
त
े
```

The `bytes` method returns each raw byte, which might be appropriate for your
domain:

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

This code will print the 18 bytes that make up this `String`, starting with:

```text
224
164
168
224
// ... etc
```

But make sure to remember that valid UTF-8 characters may be made up of more
than one byte.

Getting grapheme clusters from `String`s is complex, so this functionality is
not provided by the standard library. There are crates available on crates.io
if this is the functionality you need.

<!-- Can you recommend some, or maybe just say why we aren't outlining the
method here, ie it's complicated and therefore best to use a crate? -->

<!-- We're trying not to mention too many crates in the book. Most crates are
provided by the community, so we don't want to mention some and not others and
seem biased towards certain crates, plus crates can change more quickly (and
new crates can be created) than the language and this book will. /Carol -->

### Strings are Not so Simple

To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which does mean programmers have to put more thought
into handling UTF-8 data upfront. This tradeoff exposes more of the complexity
of strings than other programming languages do, but this will prevent you from
having to handle errors involving non-ASCII characters later in your
development lifecycle.

Let's switch to something a bit less complex: hash map!
