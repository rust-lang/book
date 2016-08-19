## Slices

So far, we’ve talked about types that have ownership, like `String`, and ones
that don’t, like `&String`. There is another kind of type which does not have
ownership: slices. Slices let you reference a contiguous sequence of elements
in a collection rather than the whole collection itself.

Here’s a small programming problem: write a function which takes a string
and returns the first word you find. If we don’t find a space in the string,
then the whole string is a word, so the whole thing should be returned.

Let’s think about the signature of this function:

```rust,ignore
fn first_word(s: &String) -> ?
```

This function, `first_word`, takes a `&String` as an argument. We don’t want
ownership, so this is fine. But what should we return? We don’t really have a
way to talk about _part_ of a string. We could return the index of the end of
the word, though. Let’s try that:

Filename: src/main.rs

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Let’s break that down a bit:

```rust,ignore
let bytes = s.as_bytes();
```

Since we need to go through the String element by element and
check if a value is a space, we will convert our String to an
array of bytes using the `.as_bytes()` method.

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

We will be discussing iterators in more detail in Chapter XX, but for
now, know that `iter()` is a method that returns each element in a
collection, and `enumerate()` modifies the result of `iter()` and returns
a tuple instead. The first element of the tuple is the index, and the
second element is a reference to the element itself. This is a bit
nicer than calculating the index ourselves.

Since it’s a tuple, we can use patterns, just like elsewhere in Rust. So we
match against the tuple with i for the index and &item for a single byte. Since
we get a reference from `.iter().enumerate()`, we use `&` in the pattern.

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

We search for the byte that represents the space, using the byte literal
syntax. If we find one, we return the position. Otherwise, we return the length
of the string, using `s.len()`.

This works, but there’s a problem. We’re returning a `usize` on its own, but
it’s only a meaningful number in the context of the `&String`. In other
words, because it’s a separate value from the `String`, there’s no guarantee
that it will still be valid in the future. Consider this:

Filename: src/main.rs

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return i;
#         }
#     }
#
#     s.len()
# }
#
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5.

    s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

This is bad! It’s even worse if we wanted to write a `second_word()`
function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking both a start _and_ an ending index. Even more chances for
things to go wrong. We now have three unrelated variable bindings floating
around which need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

## String slices

A string slice looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This looks just like taking a reference to the whole `String`, but with the
extra `[0..5]` bit. Instead of being a reference to the entire `String`, it’s a
reference to an internal position in the `String` and the number of elements
that it refers to.

We can create slices with a range of `[starting_index..ending_index]`, but the
slice data structure actually stores the starting position and the length of the
slice. So in the case of `let world = &s[6..11];`, `world` would be a slice that
contains a pointer to the 6th byte of `s` and a length value of 5.

In other words, it looks like this:

DIAGRAM GOES HERE of s, hello, and world

With Rust’s `..` range syntax, if you want to start at the first index (zero),
you can drop the value before the `..`. In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice should include the last byte of the
`String`, you can drop the trailing number. That means these are
equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these
are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

With this in mind, let’s re-write `first_word()` to return a slice:

Filename: src/main.rs

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Now we have a single value, the `&str`, pronounced "string slice". It stores
both elements that we care about: a reference to the starting point of the
slice and the number of elements in the slice.

This would also work for a `second_word()`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up.

But what about our error condition from before? Slices also fix that. Using
the slice version of `first_word()` will throw an error:

Filename: src/main.rs

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the error:

```bash
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

Remember the borrowing rules? If we have an immutable reference to something,
we cannot also take a mutable reference. Since `clear()` needs to truncate the
`String`, it tries to take a mutable reference, which fails. Not only has Rust
made our API easier to use, but it’s also eliminated an entire class of errors
at compile time!

### String literals are slices

Remember how we talked about string literals being stored inside of the binary
itself? Now that we know about slices, we can now properly understand string
literals.

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: It’s a slice, pointing to that specific point
of the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

### String slices as arguments

Knowing that you can take slices of both literals and `String`s leads us to
one more improvement on `first_word()`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write this one instead:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Why is this? Well, we aren’t trying to modify `s` at all. And we can take
a string slice that’s the full length of a `String`, so we haven’t lost
the ability to talk about full `String`s. And additionally, we can take
string slices of string literals too, so this function is more useful, but
with no loss of functionality:

Filename: src/main.rs

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return &s[0..i];
#         }
#     }
#
#     &s[..]
# }
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

## Other slices

String slices, as you might imagine, are specific to strings. But there’s a more
general slice type, too. Consider arrays:

```rust
let a = [1, 2, 3, 4, 5];
```

Just like we may want to refer to a part of a string, we may want to refer to
part of an array:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the exact same way as string slices
do, by storing a reference to the first element and a length. You’ll use this
kind of slice for all sorts of other collections. We’ll discuss these in detail
when we talk about vectors in Chapter XX.
