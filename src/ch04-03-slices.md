## Slices

There is another data type which does not have ownership: slices. Slices let
you reference a contiguous sequence of elements in a collection rather than the
whole collection itself.

Here’s a small programming problem: write a function which takes a string and
returns the first word it finds in that string. If it doesn’t find a space in
the string, it means the whole string is one word, so the whole thing should be
returned.

Let’s think about the signature of this function:

```rust,ignore
fn first_word(s: &String) -> ?
```

This function, `first_word`, takes a `&String` as an argument. We don’t want
ownership, so this is fine. But what should we return? We don’t really have a
way to talk about *part* of a string. We could return the index of the end of
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
array of bytes using the `as_bytes` method.

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

We will be discussing iterators in more detail in Chapter XX, but for now, know
that `iter` is a method that returns each element in a collection, and
`enumerate` modifies the result of `iter` and returns each element as part
of a tuple instead, where the first element of the tuple is the index, and the
second element is a reference to the element itself. This is a bit nicer than
calculating the index ourselves.

Since it’s a tuple, we can use patterns, just like elsewhere in Rust. So we
match against the tuple with `i` for the index and `&item` for a single byte.
Since we get a reference from `.iter().enumerate()`, we use `&` in the pattern.

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

We search for the byte that represents the space, using the byte literal
syntax. If we find a space, we return the position. Otherwise, we return the
length of the string, using `s.len()`.

We now have a way to find out the index of the end of the first word in the
string, but there’s a problem. We’re returning a `usize` on its own, but it’s
only a meaningful number in the context of the `&String`. In other words,
because it’s a separate value from the `String`, there’s no guarantee that it
will still be valid in the future. Consider this program that uses this
`first_word` function:

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

    // word still has the value 5 here, but there’s no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

This program compiles without any errors, and also would if we used `word`
after calling `s.clear()`. `word` isn’t connected to the state of `s` at all,
so `word` still contains the value `5`. We could use that `5` with `s` to try
to extract the first word out, but this would be a bug since the contents of
`s` have changed since we saved `5` in `word`.

This is bad! It’s even worse if we wanted to write a `second_word`
function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking both a start *and* an ending index, and we have even more
values that were calculated from data in a particular state but aren’t tied to
that state at all. We now have three unrelated variables floating
around which need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

### String Slices

A string slice is a reference to part of a `String`, and looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This is similar to taking a reference to the whole `String`, but with the
extra `[0..5]` bit. Rather than a reference to the entire `String`, it’s a
reference to an internal position in the `String` and the number of elements
that it refers to.

We create slices with a range of `[starting_index..ending_index]`, but the
slice data structure actually stores the starting position and the length of the
slice. So in the case of `let world = &s[6..11];`, `world` would be a slice that
contains a pointer to the 6th byte of `s` and a length value of 5.

Figure 4-6 shows this in a diagram:

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

Figure 4-6: String slice referring to part of a `String`

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

With this in mind, let’s re-write `first_word` to return a slice. The type
that signifies “string slice” is written as `&str`:

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

We get the index for the end of the word in the same way as before, by looking
for the first occurrence of a space. When we find a space, we return a string
slice using the start of the string and the index of the space as the starting
and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up. Remember our
bug from before, when we got the first word but then cleared the string so that
our first word was invalid? That code was logically incorrect but didn’t show
any immediate errors. The problems would show up later, if we kept trying to
use the first word index with an emptied string. Slices make this bug
impossible, and let us know we have a problem with our code much sooner. Using
the slice version of `first_word` will throw a compile time error:

Filename: src/main.rs

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the compiler error:

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

Remember from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Since `clear` needs to
truncate the `String`, it tries to take a mutable reference, which fails. Not
only has Rust made our API easier to use, but it’s also eliminated an entire
class of errors at compile time!

#### String Literals are Slices

Remember how we talked about string literals being stored inside of the binary
itself? Now that we know about slices, we can now properly understand string
literals.

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: It’s a slice, pointing to that specific point
of the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

#### String Slices as Arguments

Knowing that you can take slices of both literals and `String`s leads us to
one more improvement on `first_word`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write this one instead because it allows us
to use the same function on both `String`s and `&str`s:

```rust,ignore
fn first_word(s: &str) -> &str {
```

If we have a string slice, we can pass that as the argument directly. If we
have a `String`, we can pass a slice of the entire `String`. This makes our API
more general and useful without losing any functionality:

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

### Other Slices

String slices, as you might imagine, are specific to strings. But there’s a more
general slice type, too. Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

Just like we may want to refer to a part of a string, we may want to refer to
part of an array, and would do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the exact same way as string slices
do, by storing a reference to the first element and a length. You’ll use this
kind of slice for all sorts of other collections. We’ll discuss these in detail
when we talk about vectors in Chapter XX.

## Summary

The concepts of ownership, borrowing, and slices are what ensure memory safety
in Rust programs at compile time. Rust is a language that gives you control
over your memory usage like other systems programming languages, but having the
owner of data automatically clean up that data when the owner goes out of scope
means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we will be talking
about these concepts further throughout the rest of the book. Let’s move on to
the next chapter where we’ll look at grouping pieces of data together in a
`struct`.
