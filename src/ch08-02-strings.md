## Strings

We've already talked about strings a bunch in Chapter 4, but let's take a more
in-depth look at them now. Strings are an area that new Rustaceans commonly get stuck on. This is due to a
combination of three things: Rust's propensity for making sure to expose
possible errors, strings being a more complicated data structure than many
programmers give them credit for, and UTF-8. These things combine in a way that
can seem difficult when coming from other languages.

<!---Can you connect this up more concretely to vectors/collections, let the reader know why this is relevant here? Also, maybe briefly outline what new aspects of strings we're looking at here --->

### What is a String?

Before we can dig into those aspects, we need to talk about what exactly we
mean by the term 'string'. Rust actually only has one string type in the
core language itself: `&str`. We talked about *string slices* in Chapter 4:
these are a reference to some UTF-8 encoded string data stored elsewhere.
String literals, for example, are stored in the binary output of the program,
and are therefore string slices.

The type called `String` is provided in Rust's standard library rather than coded into the core language, and is a
growable, mutable, owned, UTF-8 encoded string type. When Rustaceans talk about
'strings' in Rust, they usually mean the string object "`String`" and a string slice "`&str`".
<!-- As opposed to what, above? Also, is this right, we mean the string object and string slice? If not, can you correct those definitions?-->

This chapter is
largely about `String`, and these two types are used heavily in Rust's standard
library. Both `String` and string slices are UTF-8 encoded.

Rust's standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates may provide even
more options for storing string data. Similar to the `*String`/`*Str` naming,
they often provide an owned and borrowed variant, just like `String`/`&str`.
These string types may store different encodings or be represented in memory in
a different way, for example. We won't be talking about these other string
types in this chapter; see their API documentation for more about how to use
them and when each is appropriate.

### Creating a New String

Many of the same operations can be done on `String` as can be done on `Vec`,
starting with the`new` function to create a string, like so:

```rust
let s = String::new();
```
This creates a new empty string called `s` that we can then load data into.

Often, we'll have some initial data that we'd like to start the string off with.
For that, we use the `to_string` method:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

This creates a string containing "initial contents".

<!--Could you add a little text explaining what this next method (below)is doing, just to make sure it's clear?-->

We can also namespace .... This form is equivalent to using `to_string`:

```rust
let s = String::from("Initial contents");
```

Because strings are used for so many things, there are many different generic
APIs that can be used for strings, and so there are a lot of options. Some of them
can feel redundant, but they all have their place! In this
case, `String::from` and `.to_string` end up doing the exact same thing, so
which you choose is a matter of style. Some people use `String::from` for
literals, and `.to_string` for variable bindings to differentiate between the two in the code.
<!---Is this right, this is why they use one for literals and one for variables? (above) -->

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

A `String` can can grow in size and its contents changes just like a `Vec`, using concatenation and the `push` method.

#### Appending a String with Push

We can grow a `String` by using the `push_str` method to append another
string:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

`s` will contain "foobar" after these two lines.

The `push` method without the `_str` syntax will add a single `char`:

<!--- Above--is this what we're saying, if we leave off _str it adds just a single char?-->

```rust
let mut s = String::from("lo");
s.push('l');
```

After this, `s` will contain "lol".

<!-- I'm not sure this bit below belongs in this section, it seems to apply to more than just push, right? Maybe mvoe it to the end of "updating a string"?-->

We can make any `String` contain an empty string using the `clear` method:

```rust
let mut s = String::from("Noooooooooooooooooooooo!");
s.clear();
```

Now `s` will be the empty string, "".

#### Concatenation with the + Operator

Often, we'll want to combine two existing strings together. One way is to use the `+`
operator:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

<!--- do we need to mention *why* the second string needs to be a reference here, or will that be clear by now? -->

After this code the String `s3` will contain "Hello, world!" There's some tricky bits here,
though, that come from the type signature of `+` for `String`. The `+` operator uses the `add method`, whose signature looks something like this:

```rust,ignore
fn add(self, s: &str) -> String {
```

This isn't the exact signature that's in the standard library; there
`add` is defined using generics, but here, we're
defining it specifically for
`String`. This signature gives us the clues we need to understand the
tricky bits of the `+` operator.

First of all, `s2` has an `&`, meaning that we are adding a *reference* of the second string to the first string. This is because of the `s` argument in the `add`
function: we can only add a `&str` to a `String`, we can't add two `String`s
together. Remember back in Chapter 4 when we talked about how `&String` will
coerce to `&str`: we write `&s2` so that the `String` will coerce to the proper
type, `&str`.

<!-- why can we only add a &str to a String and not a String to a String?  -->

Secondly, we can see in the signature that `add` takes ownership of `self`, because `self`
does *not* have an `&`. This means `s1` in the above example
will be moved into the `add` call and no longer be a valid binding after that.
So while `let s3 = s1 + &s2;` looks like it will copy both strings and create a
new one, this statement actually takes ownership of `s1`, appends a copy of
`s2`'s contents, then returns ownership of the result. In other words, it looks
like it's making a lot of copies, but isn't: the implementation is more
efficient than copying.

<!-- Ah, good explanation! So s2 will be valid after the code? -->

If we need to concatenate multiple strings, the behavior of `+` gets
unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

`s` will contain "tic-tac-toe" at this point. With all of the `+` and `"`
characters, it gets hard to see what's going on. For more complicated string
combining, we can use the `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

<!-- Are we going to discuss the format macro elsewhere at all? If not, some more info here might be good, this seems like a really useful tool. Is it only used on strings? -->

This code will also set `s` to "tic-tac-toe". The `format!` macro works in the
same way as `println!`, but instead of printing the output to the screen, it
returns a `String` with the contents. This version is much easier to read.

### Indexing into Strings

In many other languages, accessing individual characters in a string by
referencing them by index is a valid and common operation. In Rust,
however, if we try to access parts of a `String` using indexing syntax, we'll
get an error. That is, this code:

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

#### Internal Representation of Strings

A `String` is a wrapper over a `Vec<u8>`. Let's take a look at some of our
properly-encoded UTF-8 example strings from before. First, this one:

```rust
let len = "Hola".len();
```

In this case, `len` will be four, which means the `Vec` storing the string
"Hola" is four bytes long: each of these letters takes one byte when encoded in
UTF-8. What about this example, though?

```rust
let len = "Здравствуйте".len();
```

A person counting how long the string is might say 12.
However, Rust's answer is 24. This is the
number of bytes that it takes to encode "Здравствуйте" in UTF-8, because each
character takes two bytes of storage. Therefore, the index of the string will not actually correlate to any one character, and can't be used to access elements.
<!-- Could you sum this up, make it easier to apply it below? I've added a sample sentence above -->

By the same token, imagine this invalid Rust code:

```rust,ignore
let hello = "Здравствуйте";
let answer = &h[0];
```
<!-- should &h above be &hello? -->

What should the value of `answer` be? Should it be `З`, the first letter? When
encoded in UTF-8, the first byte of `З` is `208`, and the second is `151`, so
 `answer` should in fact be `208`, but `208` is not a valid character on its own.

<!-- Are we saying that it would be 208, or none of the above? -->

Plus, for latin letters, this would not return the answer most people would
expect: `&"hello"[0]` would then return `104`, not `h`.

<!-- I'm afraid I couldn't follow this last line at all! Why would it return 104, are you saying that even when indexing standard latin letters what you get back is the UTF code? If so, could you say that in so many words? -->

#### Bytes and Scalar Values and Grapheme Clusters! Oh my!

This leads to another point about UTF-8: there are really three relevant ways
to look at strings, from Rust's perspective: as bytes, scalar values, and grapheme
clusters.

If we look at the string "नमस्ते", it is ultimately stored as a `Vec`
of `u8` values that looks like this:

<!--- what is a grapheme cluster? -->

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```

That's 18 bytes. But if we look at them as Unicode scalar values, which are
what Rust's `char` type is, those bytes look like this:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here. Finally, if we look at them as grapheme
clusters, which is the closest thing to what humans would call 'letters', we'd
get this:

```text
["न", "म", "स्", "ते"]
```

Four elements! It turns out that even within 'grapheme cluster', there are
multiple ways of grouping things. Convinced that strings are actually really
complicated yet?

<!--- why does Rust look at strings in these different ways, what's the reasoning for this in the language? Maybe give some rust justification (rustification, if you will...) here -->

A final reason Rust does not allow you to index into a `String` to get a character
is that indexing operations are expected to always be fast, but this isn't possible
with a `String`, since Rust would have to walk through the contents from the
beginning to the index to determine how many valid characters there were, no
matter how we define "character".

All of these problems mean that Rust does not implement `[]` for `String`, so
we cannot directly do this.

### Slicing Strings

However, indexing the *bytes* of a string is very useful, and is not expected to
be fast. While we can't use `[]` with a single number, we _can_ use `[]` with
a range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first four bytes of the string.
Earlier, we mentioned that each of these characters was two bytes, so that means
that `s` will be "Зд".

<!-- so you would  need to know how many bytes each character within the string was in order to do this, is that right? That seems quite complicated---but below it looks like Rust warns you if you are outside character boundries, so is it a case of trial and error? Could you give some advice on using this practically? -->

What would happen if we did `&hello[0..1]`? The answer: it will panic at
runtime, in the same way that accessing an invalid index in a vector does:

```bash
thread 'main' panicked at 'index 0 and/or 1 in `Здравствуйте` do not lie on
character boundary', ../src/libcore/str/mod.rs:1694
```

Because the range `[0..1]` does not contain one or more full characters but falls between chracter boundaries, you cannot call it from the String.

### Methods for Iterating Over Strings

Luckily, there are other ways we can access elements in a String.

If we need to perform operations on individual characters, the best way to
do so is to use the `chars` method. Calling `chars` on "नमस्ते" separates out and returns  the six
Rust `char` values:

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

This code will print:

```bash
न
म
स
्
त
े
```

<!--- and then how would you access a single element, from here? -->

The `bytes` method returns each raw byte, which might be appropriate for your
domain:

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

This code will print the 18 bytes that make up this `String`, starting with:

```bash
224
164
168
224
// ... etc
```
But make sure to remember that valid UTF-8 characters may be made up of more than
one byte.

There are crates available on crates.io for getting grapheme clusters from `String`s.

<!-- Can you recommend some, or maybe just say why we aren't outlining the method here, ie it's complicated and therefore best to use a crate? -->

### Strings are Not so Simple

To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which does mean programmers have to put more thought
into handling UTF-8 data upfront. This tradeoff exposes the user to more of the
complexity of strings than they have to handle in other languages, but will
prevent you from having to handle errors involving non-ASCII characters later in
your development lifecycle.

Let's switch to something a bit less complex: Hash Map!
