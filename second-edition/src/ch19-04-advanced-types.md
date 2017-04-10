# Advanced Types

There's a few aspects of Rust's type system we haven't gone over. Write a better
intro that isn't literally the same as every other section here :frown:

## Type Aliases

Rust provides the ability to declare a 'type alias' with the `type` keyword:

```rust
type Foo = i32;
```

This means that `Foo` is a _synonym_ for `i32`; it's not its own, new type. Which
means you can do this:

```rust
type Foo = i32;

let x: i32 = 5;
let y: Foo = 5;

println!("x + y = {}", x + y);
```

Since `Foo` is an alias for `i32`, they're the same type, and we can add them together.
If you want a distinct type for `Foo`, you'd use the newtype pattern from Chapter XX.

The main use-case for type synonyms is to reduce repitition. For example, you may have
a type like this:

```rust,ignore
Box<FnOnce() + Send + 'static>
```

Typing this out all over the place can be tiresome and error-prone:

```rust,ignore
let f: Box<FnOnce() + Send + 'static> = |x| x + 1;

fn takes_long_type(f: Box<FnOnce() + Send + 'static>) {
    // ...
}

fn returns_long_type() -> Box<FnOnce() + Send + 'static> {
    // ...
}
```

An alias makes this more manageable:

```rust,ignore
type Thunk = Box<FnOnce() + Send + 'static>;

let f: Thunk = |x| x + 1;

fn takes_long_type(f: Thunk) {
    // ...
}

fn returns_long_type() -> Thunk {
    // ...
}
```

Much easier. A related case is with the `Result<T, E>` type. Consider the `std::io`
module in the standard library. I/O operations often return a `Result<T, E>`, as they
may fail to work. So, there's a struct, `std::io::Error`, that represents all of these
different possible errors. Many of the functions in `std::io` will be returning a
`Result<T, E>` where the `E` is an `std::io::Error`. For example, the `Write` trait:

```rust,ignore
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> { ... }
    fn write_fmt(&mut self, fmt: Arguments) -> Result<(), Error> { ... }
}
```

We're writing `Result<..., Error>` a lot. As such, `std::io` has this
declaration:

```rust,ignore
type Result<T> = Result<T, std::io::Error>;
```

Because this is in the `std::io` module, it's now `std::io::Result<T>`; that is,
a `Result<T, E>` with the `E` filled in as `std::io::Error`. This helps in two
ways: first, the `Write` trait ends up looking like this:

```rust,ignore
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()> { ... }
}
```

This is easier to write *and* gives us a consistent interface across all
of `std::io`. But because it's an alias, it is just another `Result<T, E>`,
which means we can use any methods that work on `Result<T, E>` with it,
and special syntax like `?`.

## The 'never' type, `!`

Rust has a special type named `!`. In type theory lingo, it's called the 'bottom type',
but we prefer the name 'never'. The name describes what it does:

```rust,ignore
fn bar() -> ! {
```

This is read as "the function `bar` returns never." And in this case, that's what
it means! You cannot create values of the type `!`, and so `bar` can never possibly
return. How could it, if it can't create a value to return?

What use is a type you can never create values for? If you think all the way back
to Chapter 2, we had some code that looked like this:

```rust,ignore
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

At the time, we skipped over some details. For example, you've learned that
`match` arms must have the same value. This doesn't work:

```rust,ignore
let guess = match guess.trim().parse()  {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

What would the type of `guess` be here? It'd have to be both an integer and a string,
and that doesn't work. So why does `continue`?

As you may have guessed, `continue` has a value of `!`. That is, when Rust goes to
compute the type of `guess`, it looks at both of the match arms. The former has a
value of `u32`, and the latter has a value of `!`. Since `!` can never have a value,
Rust is okay with this, and decides that the type of `guess` is `u32`. The fancy way
of saying this is that "never unifies with all other types". This works becuase
`continue` doesn't actually return a value; it instead moves control back to the top
of the loop. In the `Err` case, we never actually assign a value to `guess`. So
this is fine.

Another example of the never type is `panic!`. Remember the `unwrap` function that
we call on `Option<T>` values to produce a value or panic? Here's its definition:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Here, the same thing happens: We know that `val` has the type `T`, and `panic!` has
the type `!`. So the result of the overall `match` expression is `T`. This works
because `panic!` doesn't produce a value; it panics. In the `None` case, we won't be
returning a value from `unwrap`, and so it all works out.

One final expression that has the type `!` is a `loop`:

```rust,ignore
print!("forever ");

loop {
    print!("and ever ");
}
```

Here, the loop never ends, and so the value of the expression is `!`. This
wouldn't be true if we included a `break`, however, as the loop would terminate.

## Dynamically Sized Types & `Sized`

Because Rust needs to know things like memory layout, there's a particular corner
of its type system that can be confusing, and that's the concept of 'dynamically
sized types.' Sometimes referred to as 'DSTs' or 'unsized types', these types let
us talk about things that we only know the size of at runtime.

That's extremely abstract, so let's dig into the details of a dynamically sized
type that we've been using this whole book: `str`. That's right, not `&str`, but
`str`, on its own. `str` is a DST; we can't know how long the string is until
runtime. Since we can't know that, we can't create a variable of type `str`;
nor can we take an argument of type `str`. Consider this code, which does not
work:

```rust,ignore
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

These two `str`s would need to have the exact same memory layout, but they have
different lengths: `s1` needs 12 bytes of storage, and `s2` needs 15. This is
why it's not possible to create a variable holding a dynamically sized type.

So what to do? Well, you already know the answer in this case: `s1` and `s2`
aren't just `str`s, but `&str`s, and more specifically, `&'static str`s, though
the static bit isn't particularly relevant here. If you think back to Chapter 4,
we said this about `&str`:

> ... it’s a reference to an internal position in the String and the number of
> elements that it refers to.

So while a `&T` is a single value, storing the memory address of where the `T`
is located, a `&str` is _two_ values: the address of the `str`, and how long
it is. As such, a `&str` has a size we can know at compile time: it's two
`usizes` in length. That is, we always know the size of a `&str`, no matter
how long the string it refers to is. This is the general way in which dynamically
sized types are used in Rust; they have an extra bit of metadata that stores
the dynamic information. This leads us to the golden rule of dynamically sized
types:

You must always put values of dynamically sized types behind a pointer of some
kind.

While we've talked a lot about `&str`, we can combine `str` with all kinds of
pointers: `Box<str>`, for example, or `Rc<str>`. In fact, you've already seen
this before, but with a different dynamically sized type: `Trait`. That is,
the name of a trait, without any sort of qualifications. In Chapter 17,
we only talked about `Box<Trait>` as a trait object, but given that
just `Trait` on its own is a dynamically sized type, `Rc<Trait>` or
`&Trait` work too.

<!-- steve: we should probably have a forward ref in 17 to this -->

### The Sized trait

To work with DSTs, Rust has a trait that determines if a type's size is known
at compile time or not: `Sized`. This trait is automatically implemented for
everything the compiler knows the size of at compile time. In addition, Rust
sneaks in a bound on `Sized` to every generic function. That is,

```rust,ignore
fn generic<T>(t: T) {
```

is actually

```rust,ignore
fn generic<T: Sized>(t: T) {
```

That is, by default, everything can only work on types that are sized at compile
time. There is, however, special syntax you can use to relax this restriction:

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
```

There's two differences here: `?Sized` is the opposite of `Sized`, that is, this
reads as '`T` may or may not be `Sized`. This syntax is only available for `Sized`,
and not other traits.

Secondly, you'll note we switched to `&T`; because the argument may not be `Sized`,
we need to use it behind some kind of pointer, in this case, a reference.