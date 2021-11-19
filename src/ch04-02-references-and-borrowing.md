## References and Borrowing

The issue with the tuple code in Listing 4-5 is that we have to return the
`String` to the calling function so we can still use the `String` after the
call to `calculate_length`, because the `String` was moved into
`calculate_length`. Instead, we can provide a reference to the `String` value.
A *reference* is like a pointer in that it’s an address we can follow to access
data stored at that address that is owned by some other variable. Unlike a
pointer, a reference is guaranteed to point to a valid value of a particular
type. Here is how you would define and use a `calculate_length` function that
has a reference to an object as a parameter instead of taking ownership of the
value:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

First, notice that all the tuple code in the variable declaration and the
function return value is gone. Second, note that we pass `&s1` into
`calculate_length` and, in its definition, we take `&String` rather than
`String`. These ampersands represent *references*, and they allow you to refer
to some value without taking ownership of it. Figure 4-5 depicts this concept.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figure 4-5: A diagram of `&String s` pointing at `String
s1`</span>

> Note: The opposite of referencing by using `&` is *dereferencing*, which is
> accomplished with the dereference operator, `*`. We’ll see some uses of the
> dereference operator in Chapter 8 and discuss details of dereferencing in
> Chapter 15.

Let’s take a closer look at the function call here:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

The `&s1` syntax lets us create a reference that *refers* to the value of `s1`
but does not own it. Because it does not own it, the value it points to will
not be dropped when the reference stops being used.

Likewise, the signature of the function uses `&` to indicate that the type of
the parameter `s` is a reference. Let’s add some explanatory annotations:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

The scope in which the variable `s` is valid is the same as any function
parameter’s scope, but the value pointed to by the reference is not dropped
when `s` stops being used because `s` doesn’t have ownership. When functions
have references as parameters instead of the actual values, we won’t need to
return the values in order to give back ownership, because we never had
ownership.

We call the action of creating a reference *borrowing*. As in real life, if a
person owns something, you can borrow it from them. When you’re done, you have
to give it back. You don’t own it.

So what happens if we try to modify something we’re borrowing? Try the code in
Listing 4-6. Spoiler alert: it doesn’t work!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">Listing 4-6: Attempting to modify a borrowed value</span>

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Just as variables are immutable by default, so are references. We’re not
allowed to modify something we have a reference to.

### Mutable References

We can fix the code from Listing 4-6 to allow us to modify a borrowed value
with just a few small tweaks that use, instead, a *mutable reference*:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

First, we change `s` to be `mut`. Then we create a mutable reference with `&mut
s` where we call the `change` function, and update the function signature to
accept a mutable reference with `some_string: &mut String`. This makes it very
clear that the `change` function will mutate the value it borrows.

Mutable references have one big restriction: you can have only one mutable
reference to a particular piece of data at a time. This code that attempts to
create two mutable references to `s` will fail:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

This error says that this code is invalid because we cannot borrow `s` as
mutable more than once at a time. The first mutable borrow is in `r1` and must
last until it’s used in the `println!`, but between the creation of that
mutable reference and its usage, we tried to create another mutable reference
in `r2` that borrows the same data as `r1`.

The restriction preventing multiple mutable references to the same data at the
same time allows for mutation but in a very controlled fashion. It’s something
that new Rustaceans struggle with, because most languages let you mutate
whenever you’d like. The benefit of having this restriction is that Rust can
prevent data races at compile time. A *data race* is similar to a race
condition and happens when these three behaviors occur:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix
when you’re trying to track them down at runtime; Rust prevents this problem
by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for
multiple mutable references, just not *simultaneous* ones:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust enforces a similar rule for combining mutable and immutable references.
This code results in an error:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Whew! We *also* cannot have a mutable reference while we have an immutable one
to the same value. Users of an immutable reference don’t expect the value to
suddenly change out from under them! However, multiple immutable references are
allowed because no one who is just reading the data has the ability to affect
anyone else’s reading of the data.

Note that a reference’s scope starts from where it is introduced and continues
through the last time that reference is used. For instance, this code will
compile because the last usage of the immutable references, the `println!`,
occurs before the mutable reference is introduced:

```rust,edition2018
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

The scopes of the immutable references `r1` and `r2` end after the `println!`
where they are last used, which is before the mutable reference `r3` is
created. These scopes don’t overlap, so this code is allowed. The ability of
the compiler to tell that a reference is no longer being used at a point before
the end of the scope is called *Non-Lexical Lifetimes* (NLL for short), and you
can read more about it in [The Edition Guide][nll].

Even though borrowing errors may be frustrating at times, remember that it’s
the Rust compiler pointing out a potential bug early (at compile time rather
than at runtime) and showing you exactly where the problem is. Then you don’t
have to track down why your data isn’t what you thought it was.

### Dangling References

In languages with pointers, it’s easy to erroneously create a *dangling
pointer*--a pointer that references a location in memory that may have been
given to someone else--by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.

Let’s try to create a dangling reference to see how Rust prevents them with a
compile-time error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

This error message refers to a feature we haven’t covered yet: lifetimes. We’ll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That’s no good! Rust
won’t let us do this.

The solution here is to return the `String` directly:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

This works without any problems. Ownership is moved out, and nothing is
deallocated.

### The Rules of References

Let’s recap what we’ve discussed about references:

* At any given time, you can have *either* one mutable reference *or* any
  number of immutable references.
* References must always be valid.

Next, we’ll look at a different kind of reference: slices.

[nll]: https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html
