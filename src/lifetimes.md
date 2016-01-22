# Advanced: Lifetimes

There is a third part to Rust’s ownership system: ‘lifetimes’. We’ve just
started our journey with Rust, and so digging into the details of how lifetimes
work is not important just yet. However, you may see the _syntax_ of lifetimes
in documentation, and so we do need to talk about them at some level. The first
part of this chapter will give you an understanding of the basics, and you can
skip the second part if you wish for now. Come back to it later when you want
the deep dive.

## Lifetime Syntax

In the last section, we discussed references, and how they must always be
valid. We call this scope a ‘lifetime’.

We wrote a function that looked like this:

```rust
fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}
```

`s` is in scope for the entire duration of this function. This is currently
implicit: there’s no special syntax describing exactly how long the reference
is valid.

We can give this scope a name by introducing a lifetime:

```rust
fn calculate_length<'a>(s: &'a String) -> usize {
    let length = s.len();

    length
}
```

There are two new annotations here: the first is the `<'a>` bit after the
function name. This defines a particular lifetime, and gives it a name, `'a`.
All liftime names start with `'`. You can give a lifetime any name that you
wish, but most people name them `'a`, `'b`, `'c`, and so on. 

The second change is using the name that we’ve defined: `&'a String`. This is
read as “a reference to a `String` with the lifetime ‘a’.”

Rust doesn’t require us to write out this lifetime because we’re not doing
anything complex. The ‘in depth’ section below describes exactly when you
must give a reference’s lifetime a name, and when you can leave it out. For
now, all you need to know is:

* Every reference has a lifetime.
* Most of the time, you will not need to name these lifetimes.
* In advanced code, giving a lifetime a name allows you to do more complex
  things with the validity of a reference.
* Lifetime names start with `'`.

### The `'static` lifetime

There is one special lifetime: `'static`. The `'static` lifetime is the
longest possible lifetime: a reference with the `'static` lifetime is
always valid.

We’ve already seen something with the `'static` lifetime: string literals.

```rust
fn message() -> &'static str {
    "Hello"
}
```

Because string literals are baked into the final binary, they will always
be valid, no matter what part of your program accesses them: a perfect fit
for `'static`.

At this point, we’ve got enough understanding of references to move forward
learning Rust. Feel free to skip the rest of this section and move on to
the next chapter, ‘Structs’, if you’d like.

## In Depth

Ready for a more in-depth look at lifetimes? Let’s dig into the details.
