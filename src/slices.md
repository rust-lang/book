# Slices

So far, we’ve talked about types that have ownership, like `String`, and ones
that don’t, like `&String`. There is a second kind of type which does not have
ownership: slices. Slices let you take a reference to a particular series of
elements in a collection, rather than the whole collection itself.

Here’s a small programming problem: write a function which takes a string,
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

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == 32 {
            return i;
        }
    }

    s.len()
}
```

Let’s break that down a bit:

```rust
fn first_word(s: &String) -> usize {

    // Since we need to go through the String element by element, and
    // check if a value is a space, we will convert our String to an
    // array of bytes, using the `.as_bytes()` method.
    let bytes = s.as_bytes();

    // We discussed using the iter() method with for in Chapter 3.7. Here,
    // we’re adding another method: enumerate(). While iter() returns each
    // element, enumerate() modifies the result of iter(), and returns a
    // tuple instead. The first element of the tuple is the index, and the
    // second element is a reference to the element itself. This is a bit
    // nicer than calculating the index ourselves.
    //
    // Since it’s a tuple, we can use patterns, just like elsewhere in Rust.
    // So we match against the tuple with i for the index, and &byte for
    // the byte itself.
    for (i, &byte) in bytes.iter().enumerate() {

        // 32 is the value of a space in UTF-8
        if byte == 32 {

            // We found a space! Return this position.
            return i;
        }
    }

    // If we got here, we didn’t find a space, so this whole thing must be a
    // word. So return the length.
    s.len()
}
```

This works, but there’s a problem. We’re returning a `usize` on its own, but
it’s only a meaningful number in the context of the `&String` itself. In other
words, because it’s a separate value from the `String`, there’s no guarantee
that it will still be valid in the future. Consider this:

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
# 
#     for (i, &byte) in bytes.iter().enumerate() {
#         if byte == 32 {
#             return i;
#         }
#     }
# 
#     s.len()
# }

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // This empties the String, making it equal to "".

    // word is now totally invalid! There’s no more word here.
}
```

This is bad! It’s even worse if we wanted to write a `second_word()`
function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking both a start _and_ and ending index. Even more chances for
things to go wrong. We now have three unrelated variable bindings floating
around which need to be kept in sync.

Luckily, Rust has a solution to this probem: string slices.

# String slices

A string slice looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[5..9];
```

This looks just like taking a reference to the whole `String`, but with the
extra `[0..5]` bit. Instead of being a reference to the entire `String`,
it’s a reference to an internal position in the `String`, but it also keeps
track of the number of elements that it refers to as well. In other words,
it looks like this:

DIAGRAM GOES HERE of s, hello, and world

With Rust’s `..` syntax, if you want to start at zero, you can drop the zero.
In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if you want to go to the maximum value, which for slices is
the last element, you can drop the trailing number. In other words, these are
equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[1..len];
let slice = &s[1..];
```

With this in mind, let’s re-write `first_word()` to return a slice:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == 32 {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Now, we have a single value, the `&str`. It contains both elements that we care
about: a reference to the starting point, and the number of elements.
This would also work for a `second_word()`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

Same deal. We now have a straightforward API, that’s much harder to mess up.

But what about our error condition from before? Slices also fix that. Using
the slice version of `first_word()` will throw an error:

```rust,ignore
# fn first_word(s: &String) -> &str {
#     let bytes = s.as_bytes();
# 
#     for (i, &byte) in bytes.iter().enumerate() {
#         if byte == 32 {
#             return &s[0..i];
#         }
#     }
# 
#     &s[..]
# }
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the error:

```text
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

## String slices as arguments

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

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
# 
#     for (i, &byte) in bytes.iter().enumerate() {
#         if byte == 32 {
#             return &s[0..i];
#         }
#     }
# 
#     &s[..]
# }
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);

    let s = "hello world";
    let word = first_word(&s[..]);

    let word = first_word(s); // since literals are &strs, this works too!
}
```

# Other slices

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
do, with a reference to the first element, and a length. You’ll use this kind
of slice for all sorts of other collections. We’ll discuss these other slices
in detail when we talk about vectors, in Chapter 9.1.
