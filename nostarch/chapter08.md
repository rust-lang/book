
[TOC]

# Fundamental Collections

Rust's standard library includes a number of really useful data structures
called *collections*. Most other types represent one specific value, but
collections can contain multiple values inside of them. Each collection has
different capabilities and costs, and choosing an appropriate one for the
situation you're in is a skill you'll develop over time. In this chapter, we'll
go over three collections which are used very often in Rust programs:

* A *vector* allows us to store a variable number of values next to each other.
* A *string* is a collection of characters. We've seen the `String` type
  before, but we'll talk about it in depth now.
* A *hash map* allows us to associate a value with a particular key.

There are more specialized variants of each of these data structures for
particular situations, but these are the most fundamental and common. We're
going to discuss how to create and update each of the collections, as well as
what makes each special.

## Vectors

The first type we'll look at is `Vec<T>`, also known as a *vector*. Vectors
allow us to store more than one value in a single data structure that puts all
the values next to each other in memory.

### Creating a New Vector

To create a new vector, we can call the `new` function:

```rust
let v: Vec<i32> = Vec::new();
```

Note that we added a type annotation here. Since we don't actually do
anything with the vector, Rust doesn't know what kind of elements we intend to
store. This is an important point. Vectors are homogeneous: they may store many
values, but those values must all be the same type. Vectors are generic over
the type stored inside them (we'll talk about Generics more thoroughly in
Chapter 10), and the angle brackets here tell Rust that this vector will hold
elements of the `i32` type.

That said, in real code, we very rarely need to do this type annotation since
Rust can infer the type of value we want to store once we insert values. Let's
look at how to modify a vector next.

### Updating a Vector

To put elements in the vector, we can use the `push` method:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Since these numbers are `i32`s, Rust infers the type of data we want to store
in the vector, so we don't need the `<i32>` annotation.

We can improve this code even further. Creating a vector with some initial
values like this is very common, so there's a macro to do it for us:

```rust
let v = vec![5, 6, 7, 8];
```

This macro does a similar thing to our previous example, but it's much more
convenient.

### Dropping a Vector Drops its Elements

Like any other `struct`, a vector will be freed when it goes out of scope:

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```

When the vector gets dropped, it will also drop all of its contents, so those
integers are going to be cleaned up as well. This may seem like a
straightforward point, but can get a little more complicated once we start to
introduce references to the elements of the vector. Let's tackle that next!

### Reading Elements of Vectors

Now that we know how creating and destroying vectors works, knowing how to read
their contents is a good next step. There are two ways to reference a value
stored in a vector. In the following examples of these two ways, we've
annotated the types of the values that are returned from these functions for
extra clarity:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

First, note that we use the index value of `2` to get the third element:
vectors are indexed by number, starting at zero. Secondly, the two different
ways to get the third element are using `&` and `[]`s and using the `get`
method. The square brackets give us a reference, and `get` gives us an
`Option<&T>`. The reason we have two ways to reference an element is so that we
can choose the behavior we'd like to have if we try to use an index value that
the vector doesn't have an element for:

```rust,should_panic
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

With the `[]`s, Rust will cause a `panic!`. With the `get` method, it will
instead return `None` without `panic!`ing. Deciding which way to access
elements in a vector depends on whether we consider an attempted access past
the end of the vector to be an error, in which case we'd want the `panic!`
behavior, or whether this will happen occasionally under normal circumstances
and our code will have logic to handle getting `Some(&element)` or `None`.

Once we have a valid reference, the borrow checker will enforce the ownership
and borrowing rules we covered in Chapter 4 in order to ensure this and other
references to the contents of the vector stay valid. This means in a function
that owns a `Vec`, we can't return a reference to an element since the `Vec`
will be cleaned up at the end of the function:

```rust,ignore
fn element() -> String {
    let list = vec![String::from("hi"), String::from("bye")];
    list[1]
}
```

Trying to compile this will result in the following error:

```bash
error: cannot move out of indexed content [--explain E0507]
  |>
4 |>     list[1]
  |>     ^^^^^^^ cannot move out of indexed content
```

Since `list` goes out of scope and gets cleaned up at the end of the function,
the reference `list[1]` cannot be returned because it would outlive `list`.

Here's another example of code that looks like it should be allowed, but it
won't compile because the references actually aren't valid anymore:

```rust,ignore
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

Compiling this will give us this error:

```bash
error: cannot borrow `v` as mutable because it is also borrowed as immutable
[--explain E0502]
  |>
5 |> let first = &v[0];
  |>              - immutable borrow occurs here
7 |> v.push(6);
  |> ^ mutable borrow occurs here
9 |> }
  |> - immutable borrow ends here
```

This violates one of the ownership rules we covered in Chapter 4: the `push`
method needs to have a mutable borrow to the `Vec`, and we aren't allowed to
have any immutable borrows while we have a mutable borrow.

Why is it an error to have a reference to the first element in a vector while
we try to add a new item to the end, though? Due to the way vectors work,
adding a new element onto the end might require allocating new memory and
copying the old elements over to the new space if there wasn't enough room to
put all the elements next to each other where the vector was. If this happened,
our reference would be pointing to deallocated memory. For more on this, see
The Nomicon at *https://doc.rust-lang.org/stable/nomicon/vec.html*.

### Using an Enum to Store Multiple Types

Let's put vectors together with what we learned about enums in Chapter 6. At
the beginning of this section, we said that vectors will only store values that
are all the same type. This can be inconvenient; there are definitely use cases
for needing to store a list of things that might be different types. Luckily,
the variants of an enum are all the same type as each other, so when we're in
this scenario, we can define and use an enum!

For example, let's say we're going to be getting values for a row in a
spreadsheet. Some of the columns contain integers, some floating point numbers,
and some strings. We can define an enum whose variants will hold the different
value types. All of the enum variants will then be the same type, that of the
enum. Then we can create a vector that, ultimately, holds different types:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

This has the advantage of being explicit about what types are allowed in this
vector. If we allowed any type to be in a vector, there would be a chance that
the vector would hold a type that would cause errors with the operations we
performed on the vector. Using an enum plus a `match` where we access elements
in a vector like this means that Rust will ensure at compile time that we
always handle every possible case.

Using an enum for storing different types in a vector does imply that we need
to know the set of types we'll want to store at compile time. If that's not the
case, instead of an enum, we can use a trait object. We'll learn about those in
Chapter XX.

Now that we've gone over some of the most common ways to use vectors, be sure
to take a look at the API documentation for other useful methods defined on
`Vec` by the standard library. For example, in addition to `push` there's a
`pop` method that will remove and return the last element. Let's move on to the
next collection type: `String`!

## Strings

We've already talked about strings a bunch in Chapter 4, but let's take a more
in-depth look at them now.

### Many Kinds of Strings

Strings are a common place for new Rustaceans to get stuck. This is due to a
combination of three things: Rust's propensity for making sure to expose
possible errors, strings being a more complicated data structure than many
programmers give them credit for, and UTF-8. These things combine in a way that
can seem difficult coming from other languages.

Before we can dig into those aspects, we need to talk about what exactly we
even mean by the word 'string'. Rust actually only has one string type in the
core language itself: `&str`. We talked about *string slices* in Chapter 4:
they're a reference to some UTF-8 encoded string data stored somewhere else.
String literals, for example, are stored in the binary output of the program,
and are therefore string slices.

Rust's standard library is what provides the type called `String`. This is a
growable, mutable, owned, UTF-8 encoded string type. When Rustaceans talk about
'strings' in Rust, they usually mean "`String` and `&str`". This chapter is
largely about `String`, and these two types are used heavily in Rust's standard
library. Both `String` and string slices are UTF-8 encoded.

Rust's standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates may provide even
more options for storing string data. Similarly to the `*String`/`*Str` naming,
they often provide an owned and borrowed variant, just like `String`/`&str`.
These string types may store different encodings or be represented in memory in
a different way, for example. We won't be talking about these other string
types in this chapter; see their API documentation for more about how to use
them and when each is appropriate.

### Creating a New String

Let's look at how to do the same operations on `String` as we did with `Vec`,
starting with creating one. Similarly, `String` has `new`:

```rust
let s = String::new();
```

Often, we'll have some initial data that we'd like to start the string off with.
For that, there's the `to_string` method:

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

This form is equivalent to using `to_string`:

```rust
let s = String::from("Initial contents");
```

Since strings are used for so many things, there are many different generic
APIs that make sense for strings. There are a lot of options, and some of them
can feel redundant because of this, but they all have their place! In this
case, `String::from` and `.to_string` end up doing the exact same thing, so
which you choose is a matter of style. Some people use `String::from` for
literals, and `.to_string` for variable bindings. Most Rust style is pretty
uniform, but this specific question is one of the most debated.

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

A `String` can be changed and can grow in size, just like a `Vec` can.

#### Push

We can grow a `String` by using the `push_str` method to append another
string:

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

`s` will contain "foobar" after these two lines.

The `push` method will add a `char`:

```rust
let mut s = String::from("lo");
s.push('l');
```

`s` will contain "lol" after this point.

We can make any `String` contain the empty string with the `clear` method:

```rust
let mut s = String::from("Noooooooooooooooooooooo!");
s.clear();
```

Now `s` will be the empty string, "".

#### Concatenation

Often, we'll want to combine two strings together. One way is to use the `+`
operator:

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

This code will make `s3` contain "Hello, world!" There's some tricky bits here,
though, that come from the type signature of `+` for `String`. The signature
for the `add` method that the `+` operator uses looks something like this:

```rust,ignore
fn add(self, s: &str) -> String {
```

This isn't exactly what the actual signature is in the standard library because
`add` is defined using generics there. Here, we're just looking at what the
signature of the method would be if `add` was defined specifically for
`String`. This signature gives us the clues we need in order to understand the
tricky bits of `+`.

First of all, `s2` has an `&`. This is because of the `s` argument in the `add`
function: we can only add a `&str` to a `String`, we can't add two `String`s
together. Remember back in Chapter 4 when we talked about how `&String` will
coerce to `&str`: we write `&s2` so that the `String` will coerce to the proper
type, `&str`.

Secondly, `add` takes ownership of `self`, which we can tell because `self`
does *not* have an `&` in the signature. This means `s1` in the above example
will be moved into the `add` call and no longer be a valid binding after that.
So while `let s3 = s1 + &s2;` looks like it will copy both strings and create a
new one, this statement actually takes ownership of `s1`, appends a copy of
`s2`'s contents, then returns ownership of the result. In other words, it looks
like it's making a lot of copies, but isn't: the implementation is more
efficient than copying.

If we need to concatenate multiple strings, this behavior of `+` gets
unwieldy:

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

This code will also set `s` to "tic-tac-toe". The `format!` macro works in the
same way as `println!`, but instead of printing the output to the screen, it
returns a `String` with the contents. This version is much easier to read than
all of the `+`s.

### Indexing into Strings

In many other languages, accessing individual characters in a string by
referencing the characters by index is a valid and common operation. In Rust,
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

#### Internal Representation

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

There are two answers that potentially make sense here: the first is 12, which
is the number of letters that a person would count if we asked someone how long
this string was. The second, though, is what Rust's answer is: 24. This is the
number of bytes that it takes to encode "Здравствуйте" in UTF-8, because each
character takes two bytes of storage.

By the same token, imagine this invalid Rust code:

```rust,ignore
let hello = "Здравствуйте";
let answer = &h[0];
```

What should the value of `answer` be? Should it be `З`, the first letter? When
encoded in UTF-8, the first byte of `З` is `208`, and the second is `151`. So
should `answer` be `208`? `208` is not a valid character on its own, though.
Plus, for Latin letters, this would not return the answer most people would
expect: `&"hello"[0]` would then return `104`, not `h`.

#### Bytes and Scalar Values and Grapheme Clusters! Oh my!

This leads to another point about UTF-8: there are really three relevant ways
to look at strings, from Rust's perspective: bytes, scalar values, and grapheme
clusters. If we look at the string "नमस्ते", it is ultimately stored as a `Vec`
of `u8` values that looks like this:

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

Another reason that indexing into a `String` to get a character is not available
is that indexing operations are expected to always be fast. This isn't possible
with a `String`, since Rust would have to walk through the contents from the
beginning to the index to determine how many valid characters there were, no
matter how we define "character".

All of these problems mean that Rust does not implement `[]` for `String`, so
we cannot directly do this.

### Slicing Strings

However, indexing the bytes of a string is very useful, and is not expected to
be fast. While we can't use `[]` with a single number, we _can_ use `[]` with
a range to create a string slice from particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first four bytes of the string.
Earlier, we mentioned that each of these characters was two bytes, so that means
that `s` will be "Зд".

What would happen if we did `&hello[0..1]`? The answer: it will panic at
runtime, in the same way that accessing an invalid index in a vector does:

```bash
thread 'main' panicked at 'index 0 and/or 1 in `Здравствуйте` do not lie on
character boundary', ../src/libcore/str/mod.rs:1694
```

### Methods for Iterating Over Strings

If we do need to perform operations on individual characters, the best way to
do that is using the `chars` method. Calling `chars` on "नमस्ते" gives us the six
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

The `bytes` method returns each raw byte, which might be appropriate for your
domain, but remember that valid UTF-8 characters may be made up of more than
one byte:

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

There are crates available on crates.io to get grapheme clusters from `String`s.

To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to attempt to make correct handling of `String` data be the default
for all Rust programs, which does mean programmers have to put more thought
into handling UTF-8 data upfront. This tradeoff exposes us to more of the
complexity of strings than we have to handle in other languages, but will
prevent us from having to handle errors involving non-ASCII characters later in
our development lifecycle.

Let's switch to something a bit less complex: Hash Map!

## Hash Maps

The last of our fundamental collections is the *hash map*. The type `HashMap<K,
V>` stores a mapping of keys of type `K` to values of type `V`. It does this
via a *hashing function*, which determines how it places these keys and values
into memory. Many different programming languages support this kind of data
structure, but often with a different name: hash, map, object, hash table, or
associative array, just to name a few.

We'll go over the basic API in this chapter, but there are many more goodies
hiding in the functions defined on `HashMap` by the standard library. As always,
check the standard library documentation for more information.

### Creating a New Hash Map

We can create an empty `HashMap` with `new`, and add elements with `insert`:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");
```

Note that we need to `use` the `HashMap` from the collections portion of the
standard library. Of our three fundamental collections, this one is the least
often used, so it has a bit less support from the language. There's no built-in
macro to construct them, for example, and they're not in the prelude, so we
need to add a `use` statement for them.

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `i32` and values of type `&str`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values must
have the same type.

If we have a vector of tuples, we can convert it into a hash map with the
`collect` method. The first element in each tuple will be the key, and the
second element will be the value:

```rust
use std::collections::HashMap;

let data = vec![(1, "hello"), (2, "world")];

let map: HashMap<_, _> = data.into_iter().collect();
```

The type annotation `HashMap<_, _>` is needed here because it's possible to
`collect` into many different data structures, so Rust doesn't know which we
want. For the type parameters for the key and value types, however, we can use
underscores and Rust can infer the types that the hash map contains based on the
types of the data in our vector.

For types that implement the `Copy` trait like `i32` does, the values are
copied into the hash map. If we insert owned values like `String`, the values
will be moved and the hash map will be the owner of those values:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point
```

We would not be able to use the bindings `field_name` and `field_value` after
they have been moved into the hash map with the call to `insert`.

If we insert references to values, the values themselves will not be moved into
the hash map. The values that the references point to must be valid for at least
as long as the hash map is valid, though. We will talk more about these issues
in the Lifetimes section of Chapter 10.

### Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the `get` method:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");

let value = map.get(&2);
```

Here, `value` will have the value `Some("world")`, since that's the value
associated with the `2` key. "world" is wrapped in `Some` because `get` returns
an `Option<V>`. If there's no value for that key in the hash map, `get` will
return `None`.

We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");

for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

This will print:

```bash
1: hello
2: world
```

### Updating a Hash Map

Since each key can only have one value, when we want to change the data in a
hash map, we have to decide how to handle the case when a key already has a
value assigned. We could choose to replace the old value with the new value. We
could choose to keep the old value and ignore the new value, and only add the
new value if the key *doesn't* already have a value. Or we could change the
existing value. Let's look at how to do each of these!

#### Overwriting a Value

If we insert a key and a value, then insert that key with a different value,
the value associated with that key will be replaced. Even though this code
calls `insert` twice, the hash map will only contain one key/value pair, since
we're inserting with the key `1` both times:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(1, "Hi There");

println!("{:?}", map);
```

This will print `{1: "Hi There"}`.

#### Only Insert If the Key Has No Value

It's common to want to see if there's some sort of value already stored in the
hash map for a particular key, and if not, insert a value. hash maps have a
special API for this, called `entry`, that takes the key we want to check as an
argument:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(1, "hello");

let e = map.entry(2);
```

Here, the value bound to `e` is a special enum, `Entry`. An `Entry` represents a
value that might or might not exist. Let's say that we want to see if the key
`2` has a value associated with it. If it doesn't, we want to insert the value
"world". In both cases, we want to return the resulting value that now goes
with `2`. With the entry API, it looks like this:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");

map.entry(2).or_insert("world");
map.entry(1).or_insert("Hi There");

println!("{:?}", map);
```

The `or_insert` method on `Entry` does exactly this: returns the value for the
`Entry`'s key if it exists, and if not, inserts its argument as the new value
for the `Entry`'s key and returns that. This is much cleaner than writing the
logic ourselves, and in addition, plays more nicely with the borrow checker.

This code will print `{1: "hello", 2: "world"}`. The first call to `entry` will
insert the key `2` with the value "world", since `2` doesn't have a value
already. The second call to `entry` will not change the hash map since `1`
already has the value "hello".

#### Update a Value Based on the Old Value

Another common use case for hash maps is to look up a key's value then update
it, using the old value. For instance, if we wanted to count how many times
each word appeared in some text, we could use a hash map with the words as keys
and increment the value to keep track of how many times we've seen that word.
If this is the first time we've seen a word, we'll first insert the value `0`.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

This will print `{"world": 2, "hello": 1, "wonderful": 1}`. The `or_insert`
method actually returns a mutable reference (`&mut V`) to the value in the
hash map for this key. Here we store that mutable reference in the `count`
variable binding, so in order to assign to that value we must first dereference
`count` using the asterisk (`*`). The mutable reference goes out of scope at
the end of the `for` loop, so all of these changes are safe and allowed by the
borrowing rules.

### Hashing Function

By default, `HashMap` uses a cryptographically secure hashing function that can
provide resistance to Denial of Service (DoS) attacks. This is not the fastest
hashing algorithm out there, but the tradeoff for better security that comes
with the drop in performance is a good default tradeoff to make. If you profile
your code and find that the default hash function is too slow for your
purposes, you can switch to another function by specifying a different
*hasher*. A hasher is an object that implements the `BuildHasher` trait. We'll
be talking about traits and how to implement them in Chapter 10.

## Summary

Vectors, strings, and hash maps will take you far in programs where you need to
store, access, and modify data. Some programs you are now equipped to write and
might want to try include:

* Given a list of integers, use a vector and return their mean (average),
  median (when sorted, the value in the middle position), and mode (the value
  that occurs most often; a hash map will be helpful here).
* Convert strings to Pig Latin, where the first consonant of each word gets
  moved to the end with an added "ay", so "first" becomes "irst-fay". Words that
  start with a vowel get an h instead ("apple" becomes "apple-hay"). Remember
  about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in the company. For example, "Add Sally to
  Engineering" or "Add Ron to Sales". Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.

The standard library API documentation describes methods these types have that
will be helpful for these exercises!

We're getting into more complex programs where operations can fail, which means
it's a perfect time to go over error handling next!
