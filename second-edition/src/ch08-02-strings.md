## Strings Store UTF-8 Encoded Text

We talked about strings in Chapter 4, but we’ll look at them in more depth now.
New Rustaceans commonly get stuck on strings due to a combination of three
concepts: Rust’s propensity for exposing possible errors, strings being a more
complicated data structure than many programmers give them credit for, and
UTF-8. These concepts combine in a way that can seem difficult when you’re
coming from other programming languages.

This discussion of strings is in the collections chapter because strings are
implemented as a collection of bytes plus some methods to provide useful
functionality when those bytes are interpreted as text. In this section, we’ll
talk about the operations on `String` that every collection type has, such as
creating, updating, and reading. We’ll also discuss the ways in which `String`
is different than the other collections, namely how indexing into a `String` is
complicated by the differences between how people and computers interpret
`String` data.

### What Is a String?

We’ll first define what we mean by the term *string*. Rust has only one string
type in the core language, which is the string slice `str` that is usually seen
in its borrowed form `&str`. In Chapter 4, we talked about *string slices*,
which are references to some UTF-8 encoded string data stored elsewhere. String
literals, for example, are stored in the binary output of the program and are
therefore string slices.

The `String` type is provided in Rust’s standard library rather than coded into
the core language and is a growable, mutable, owned, UTF-8 encoded string type.
When Rustaceans refer to “strings” in Rust, they usually mean the `String` and
the string slice `&str` types, not just one of those types. Although this
section is largely about `String`, both types are used heavily in Rust’s
standard library and both `String` and string slices are UTF-8 encoded.

Rust’s standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates can provide even
more options for storing string data. Similar to the `*String`/`*Str` naming,
they often provide an owned and borrowed variant, just like `String`/`&str`.
These string types can store text in different encodings or be represented in
memory in a different way, for example. We won’t discuss these other string
types in this chapter; see their API documentation for more about how to use
them and when each is appropriate.

### Creating a New String

Many of the same operations available with `Vec<T>` are available with `String`
as well, starting with the `new` function to create a string, shown in [Listing 8-11][Listing-8-11]:

[Listing-8-11]: #Listing-8-11
<a name="Listing-8-11"></a>

```rust
let mut s = String::new();
```

<span class="caption">Listing 8-11: Creating a new, empty `String`</span>

This line creates a new empty string called `s` that we can then load data
into. Often, we’ll have some initial data that we want to start the string
with. For that, we use the `to_string` method, which is available on any type
that implements the `Display` trait, which string literals do. [Listing 8-12][Listing-8-12]
shows two examples:

[Listing-8-12]: #Listing-8-12
<a name="Listing-8-12"></a>

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

<span class="caption">Listing 8-12: Using the `to_string` method to create a
`String` from a string literal</span>

This code creates a string containing `initial contents`.

We can also use the function `String::from` to create a `String` from a string
literal. The code in [Listing 8-13][Listing-8-13] is equivalent to the code from Listing 8-12
that uses `to_string`:

[Listing-8-13]: #Listing-8-13
<a name="Listing-8-13"></a>

```rust
let s = String::from("initial contents");
```

<span class="caption">Listing 8-13: Using the `String::from` function to create
a `String` from a string literal</span>

Because strings are used for so many things, we can use many different generic
APIs for strings, providing us with a lot of options. Some of them can seem
redundant, but they all have their place! In this case, `String::from` and
`to_string` do the same thing, so which you choose is a matter of style.

Remember that strings are UTF-8 encoded, so we can include any properly encoded
data in them, as shown in [Listing 8-14][Listing-8-14]:

[Listing-8-14]: #Listing-8-14
<a name="Listing-8-14"></a>

```rust
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

<span class="caption">Listing 8-14: Storing greetings in different languages in
strings</span>

All of these are valid `String` values.

### Updating a String

A `String` can grow in size and its contents can change, just like the contents
of a `Vec<T>`, by pushing more data into it. In addition, we can conveniently
use the `+` operator or the `format!` macro to concatenate `String` values
together.

#### Appending to a String with `push_str` and `push`

We can grow a `String` by using the `push_str` method to append a string slice,
as shown in [Listing 8-15][Listing-8-15]:

[Listing-8-15]: #Listing-8-15
<a name="Listing-8-15"></a>

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

<span class="caption">Listing 8-15: Appending a string slice to a `String`
using the `push_str` method</span>

After these two lines, `s` will contain `foobar`. The `push_str` method takes a
string slice because we don’t necessarily want to take ownership of the
parameter. For example, the code in [Listing 8-16][Listing-8-16] shows that it would be
unfortunate if we weren’t able to use `s2` after appending its contents to `s1`:

[Listing-8-16]: #Listing-8-16
<a name="Listing-8-16"></a>

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(&s2);
println!("s2 is {}", s2);
```

<span class="caption">Listing 8-16: Using a string slice after appending its
contents to a `String`</span>

If the `push_str` method took ownership of `s2`, we wouldn’t be able to print
out its value on the last line. However, this code works as we’d expect!

The `push` method takes a single character as a parameter and adds it to the
`String`. [Listing 8-17][Listing-8-17] shows code that adds the letter l character to a
`String` using the `push` method:

[Listing-8-17]: #Listing-8-17
<a name="Listing-8-17"></a>

```rust
let mut s = String::from("lo");
s.push('l');
```

<span class="caption">Listing 8-17: Adding one character to a `String` value
using `push`</span>

As a result of this code, `s` will contain `lol`.

#### Concatenation with the `+` Operator or the `format!` Macro

Often, we’ll want to combine two existing strings. One way is to use the `+`
operator, as shown in [Listing 8-18][Listing-8-18]:

[Listing-8-18]: #Listing-8-18
<a name="Listing-8-18"></a>

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
```

<span class="caption">Listing 8-18: Using the `+` operator to combine two
`String` values into a new `String` value</span>

The string `s3` will contain `Hello, world!` as a result of this code. The
reason `s1` is no longer valid after the addition and the reason we used a
reference to `s2` has to do with the signature of the method that gets called
when we use the `+` operator. The `+` operator uses the `add` method, whose
signature looks something like this:

```rust,ignore
fn add(self, s: &str) -> String {
```

This isn’t the exact signature that’s in the standard library: in the standard
library, `add` is defined using generics. Here, we’re looking at the signature
of `add` with concrete types substituted for the generic ones, which is what
happens when we call this method with `String` values. We’ll discuss generics
in Chapter 10. This signature gives us the clues we need to understand the
tricky bits of the `+` operator.

First, `s2` has an `&`, meaning that we’re adding a *reference* of the second
string to the first string because of the `s` parameter in the `add` function:
we can only add a `&str` to a `String`; we can’t add two `String` values
together. But wait - the type of `&s2` is `&String`, not `&str`, as specified
in the second parameter to `add`. So why does [Listing 8-18][Listing-8-18] compile?

The reason we’re able to use `&s2` in the call to `add` is that the compiler
can *coerce* the `&String` argument into a `&str`. When we call the `add`
method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`.
We’ll discuss deref coercion in more depth in Chapter 15. Because `add` does
not take ownership of the `s` parameter, `s2` will still be a valid `String`
after this operation.

Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does *not* have an `&`. This means `s1` in [Listing 8-18][Listing-8-18] will be
moved into the `add` call and no longer be valid after that. So although `let
s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this
statement actually takes ownership of `s1`, appends a copy of the contents of
`s2`, and then returns ownership of the result. In other words, it looks like
it’s making a lot of copies but isn’t: the implementation is more efficient
than copying.

If we need to concatenate multiple strings, the behavior of `+` gets unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"`
characters, it’s difficult to see what’s going on. For more complicated string
combining, we can use the `format!` macro:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

This code also sets `s` to `tic-tac-toe`. The `format!` macro works in the same
way as `println!`, but instead of printing the output to the screen, it returns
a `String` with the contents. The version of the code using `format!` is much
easier to read and also doesn’t take ownership of any of its parameters.

### Indexing into Strings

In many other programming languages, accessing individual characters in a
string by referencing them by index is a valid and common operation. However,
if we try to access parts of a `String` using indexing syntax in Rust, we’ll
get an error. Consider the invalid code in [Listing 8-19][Listing-8-19]:

[Listing-8-19]: #Listing-8-19
<a name="Listing-8-19"></a>

```rust,ignore
let s1 = String::from("hello");
let h = s1[0];
```

<span class="caption">Listing 8-19: Attempting to use indexing syntax with a
String</span>

This code will result in the following error:

```text
error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
 -->
  |
3 |     let h = s1[0];
  |             ^^^^^ the type `std::string::String` cannot be indexed by `{integer}`
  |
  = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
```

The error and the note tell the story: Rust strings don’t support indexing. But
why not? To answer that question, we need to discuss how Rust stores strings in
memory.

#### Internal Representation

A `String` is a wrapper over a `Vec<u8>`. Let’s look at some of our properly
encoded UTF-8 example strings from [Listing 8-14][Listing-8-14]. First, this one:

```rust
let len = String::from("Hola").len();
```

In this case, `len` will be four, which means the `Vec` storing the string
“Hola” is four bytes long. Each of these letters takes one byte when encoded in
UTF-8. But what about the following line?

```rust
let len = String::from("Здравствуйте").len();
```

Note that this string begins with the capital Cyrillic letter Ze, not the
Arabic number 3. Asked how long the string is, you might say 12. However,
Rust’s answer is 24: that’s the number of bytes it takes to encode
“Здравствуйте” in UTF-8, because each Unicode scalar value takes two bytes of
storage. Therefore, an index into the string’s bytes will not always correlate
to a valid Unicode scalar value. To demonstrate, consider this invalid Rust
code:

```rust,ignore
let hello = "Здравствуйте";
let answer = &hello[0];
```

What should the value of `answer` be? Should it be `З`, the first letter? When
encoded in UTF-8, the first byte of `З` is `208`, and the second is `151`, so
`answer` should in fact be `208`, but `208` is not a valid character on its
own. Returning `208` is likely not what a user would want if they asked for the
first letter of this string; however, that’s the only data that Rust has at
byte index 0. Returning the byte value is probably not what users want, even if
the string contains only Latin letters: if `&"hello"[0]` was valid code that
returned the byte value, it would return `104`, not `h`. To avoid returning an
unexpected value and causing bugs that might not be discovered immediately,
Rust doesn’t compile this code at all and prevents misunderstandings earlier in
the development process.

#### Bytes and Scalar Values and Grapheme Clusters! Oh My!

Another point about UTF-8 is that there are actually three relevant ways to
look at strings from Rust’s perspective: as bytes, scalar values, and grapheme
clusters (the closest thing to what we would call *letters*).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is
ultimately stored as a `Vec` of `u8` values that looks like this:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

That’s 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust’s `char` type is, those
bytes look like this:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters:
they’re diacritics that don’t make sense on their own. Finally, if we look at
them as grapheme clusters, we’d get what a person would call the four letters
that make up the Hindi word:

```text
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.

A final reason Rust doesn’t allow us to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). But it isn’t possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.

### Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the
return type of the string indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. Therefore, Rust asks you to
be more specific if you really need to use indices to create string slices. To
be more specific in your indexing and indicate that you want a string slice,
rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first four bytes of the string.
Earlier, we mentioned that each of these characters was two bytes, which means
`s` will be `Зд`.

What would happen if we used `&hello[0..1]`? The answer: Rust will panic at
runtime in the same way that accessing an invalid index in a vector does:

```text
thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2188:4
```

You should use ranges to create string slices with caution, because it can
crash your program.

### Methods for Iterating Over Strings

Fortunately, we can access elements in a string in other ways.

If we need to perform operations on individual Unicode scalar values, the best
way to do so is to use the `chars` method. Calling `chars` on “नमस्ते” separates
out and returns six values of type `char`, and we can iterate over the result
in order to access each element:

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

This code will print the following:

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

But be sure to remember that valid Unicode scalar values may be made up of more
than one byte.

Getting grapheme clusters from strings is complex, so this functionality is not
provided by the standard library. Crates are available on
[crates.io](https://crates.io) if this is the functionality you need.

### Strings Are Not So Simple

To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which means programmers have to put more thought into
handling UTF-8 data upfront. This trade-off exposes more of the complexity of
strings than other programming languages do but prevents you from having to
handle errors involving non-ASCII characters later in your development life
cycle.

Let’s switch to something a bit less complex: hash maps!

[Listing-8-11]: ch08-02-strings.html#Listing-8-11
[Listing-8-12]: ch08-02-strings.html#Listing-8-12
[Listing-8-13]: ch08-02-strings.html#Listing-8-13
[Listing-8-14]: ch08-02-strings.html#Listing-8-14
[Listing-8-15]: ch08-02-strings.html#Listing-8-15
[Listing-8-16]: ch08-02-strings.html#Listing-8-16
[Listing-8-17]: ch08-02-strings.html#Listing-8-17
[Listing-8-18]: ch08-02-strings.html#Listing-8-18
[Listing-8-19]: ch08-02-strings.html#Listing-8-19
