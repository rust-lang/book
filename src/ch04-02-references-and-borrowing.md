## References and Borrowing

The issue with the tuple code at the end of the last section is that we have to
return the `String` back to the calling function so that we can still use the
`String` after the call to `calculate_length`, since the `String` was moved
into `calculate_length`.

Here is how you would define and use a `calculate_length` function that takes a
*reference* to an object as an argument instead of taking ownership of the
argument:

Filename: src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

First, you’ll notice all of the tuple stuff in the variable declaration and the
function return value is gone. Next, note that we pass `&s1` into
`calculate_length`, and in its definition, we take `&String` rather than
`String`.

These `&`s are *references*, and they allow you to refer to some value without
taking ownership of it. Figure 4-5 shows a diagram of this.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<caption>
Figure 4-5: `&String s` pointing at `String s1`
</caption>

Let’s take a closer look at the function call here:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

The `&s1` syntax lets us create a reference which *refers* to the value of `s1`
but does not own it. Because it does not own it, the value it points to will
not be dropped when the reference goes out of scope.

Likewise, the signature of the function uses `&` to indicate that it takes a
reference as an argument. Let’s add some explanatory annotations:

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But since it does not have ownership of what
  // it refers to, nothing happens.
```

It’s the same process as before, but we don’t drop what the reference points to
when it goes out of scope because we don’t have ownership. This lets us write
functions which take references as arguments instead of the values themselves,
so that we won’t need to return them to give back ownership.

We call this process *borrowing*. Just like with real life, if a person owns
something, you can borrow it from them, and when you’re done, you have to give
it back.

So what happens if we try to modify something we’re borrowing? Try this code
out. Spoiler alert: it doesn’t work!

Filename: src/main.rs

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

```bash
error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

Just as variables are immutable by default, so are references. We’re not allowed
to modify something we have a reference to.

### Mutable References

We can fix this error with just a small tweak:

Filename: src/main.rs

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

Mutable references have one big restriction, though: you can only have one
mutable reference to a particular piece of data in a particular scope. This
code will fail:

Filename: src/main.rs

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```bash
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

This restriction allows for mutation but in a very controlled fashion. It is
something that new Rustaceans struggle with, because most languages let you
mutate whenever you’d like. The benefit of having this restriction is that Rust
can prevent data races at compile time.

A *data race* is a particular type of race condition where these three things
occur:

1. Two or more pointers access the same data at the same time
1. At least one of the pointers is being used to write to the data
1. There’s no mechanism being used to synchronize access to the data

Data races cause undefined behavior and can be difficult to diagnose and fix
when trying to track them down at runtime; Rust prevents this problem from
happening since it won’t even compile code with data races!

As always, we can use `{}`s to create a new scope, allowing for multiple
mutable references, just not *simultaneous* ones:

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

```bash
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

Whew! We *also* cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! Multiple immutable references are okay, however, since no one
who is just reading the data has the ability to affect anyone else’s reading of
the data.

Even though these errors may be frustrating at times, remember that it’s the
Rust compiler pointing out a potential bug earlier (at compile time rather than
at runtime) and showing you exactly where the problem is instead of you having
to track down why sometimes your data isn’t what you thought it should be.

### Dangling References

In languages with pointers, it’s easy to make the error of creating a *dangling
pointer*, a pointer referencing a location in memory that may have been given
to someone else, by freeing some memory while keeping around a pointer to that
memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling: if we have a reference to some data, the compiler will
ensure that the data will not go out of scope before the reference to the data
does.

Let’s try to create a dangling reference:

Filename: src/main.rs

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

```bash
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

This error message refers to a feature we haven’t learned about yet:
*lifetimes*. We’ll discuss lifetimes in detail in Chapter 10, but, disregarding
the parts about lifetimes, the message does contain the key to why this code is
a problem:

```bash
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Let’s have a closer look at exactly what’s happening at each stage of our
`dangle` code:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside of `dangle`, when the code of `dangle` is
finished, it will be deallocated. But we tried to return a reference to it.
That means this reference would be pointing to an invalid `String`! That’s no
good. Rust won’t let us do this.

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

1. At any given time, you may have *either*, but not both of:
    1. One mutable reference.
    2. Any number of immutable references.
2. References must always be valid.

Next, let’s look at a different kind of reference: slices.
