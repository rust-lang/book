## References and Borrowing

At the end of the last section, we had some example Rust that wasn’t very
good. Here it is again:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

The issue here is that we have to return the `String` back to the calling
function so that we can still use it there, since it was moved when we called
`calculate_length()`.

There is a better way. It looks like this:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}
```

First, you’ll notice all of the tuple stuff in the binding declaration and the
function return value is gone. Next, note that we pass `&s1` into
`calculate_length()`, and in its definition, we take `&String` rather than
`String`.

These `&`s are called ‘references’, and they allow you to refer to some value
without taking ownership of it. Here’s a diagram:

DIAGRAM GOES HERE of a &String pointing at a String, with (ptr, len, capacity)

Let’s take a closer look at the function call here:

```rust
# fn calculate_length(s: &String) -> usize {
#     let length = s.len();
#
#     length
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

The `&s1` syntax lets us create a reference with `s1`. This reference _refers_
to the value of `s1` but does not own it. Because it does not own it, the
value it points to will not be dropped when the reference goes out of scope.

Likewise, the signature of the function uses `&` to indicate that it takes
a reference as an argument. Let’s add some explanatory annotations:

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    let length = s.len();

    length
} // Here, s goes out of scope. But since it does not have ownership of what
  // it refers to, nothing happens.
```

It’s the same process as before, except that because we don’t have ownership,
we don’t drop what a reference points to when the reference goes out of scope.
This lets us write functions which take references as arguments instead of the
values themselves, so that we won’t need to return them to give back ownership.

There’s another word for what references do, and that’s ‘borrowing’. Just like
with real life, if a person owns something, you can borrow it from them. When
you’re done, you have to give it back.

Speaking of which, what if you try to modify something you borrow from me? Try
this code out. Spoiler alert: it doesn’t work!

```rust,ignore
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Here’s the error:

```text
error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

Just like bindings are immutable by default, so are references. We’re not
allowed to modify something we have a reference to.

### Mutable references

We can fix this bug! Just a small tweak:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First, we had to change `s` to be `mut`. Then we had to create a mutable
reference with `&mut s` and accept a mutable reference with `some_string: &mut
String`.

Mutable references have one big restriction, though. This code fails:

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

The error is what it says: you cannot borrow something mutably more than once
at a time. This restriction allows for mutation but in a very controlled
fashion. It is something that new Rustaceans struggle with, because most
languages let you mutate whenever you’d like.

As always, we can use `{}`s to create a new scope, allowing for multiple mutable
references, just not _simultaneous_ ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

There is a similar rule for combining mutable and immutable references. This
code errors:

```rust,ignore
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

Here’s the error:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Whew! We _also_ cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! Multiple immutable references are okay, however.

### Dangling references

In languages with pointers, it’s easy to create a “dangling pointer” by freeing
some memory while keeping around a pointer to that memory. In Rust, by
contrast, the compiler guarantees that references will never be dangling: if we
have a reference to something, the compiler will ensure that it will not go
out of scope before the reference does.

Let’s try to create a dangling reference:

```rust,ignore
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```text
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^^^^^^^
  |
  = help: this function's return type contains a borrowed value, but there is no
    value for it to be borrowed from
  = help: consider giving it a 'static lifetime

error: aborting due to previous error
```

This error message refers to a feature we haven’t learned about yet,
‘lifetimes’. The message does contain the key to why this code is a problem,
though:

```text
this function’s return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Let’s examine exactly what happens with `dangle()`:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside of `dangle()`, when the code of `dangle()` is
finished, it will be deallocated. But we tried to return a reference to it.
That means this reference would be pointing to an invalid `String`! That’s
no good. Rust won’t let us do this.

The correct code here is to return the `String` directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works, no problem. Ownership is moved out, nothing is deallocated.

### The Rules of References

Here’s a recap of what we’ve talked about:

1. At any given time, you may have _either_, but not both of:
    1. One mutable reference.
    2. Any number of immutable references.
2. References must always be valid.

Next, let's look at a different kind of reference: slices.
