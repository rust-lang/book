## Advanced Types

The Rust type system has some features that we’ve mentioned or used without
discussing. We started talking about the newtype pattern in regards to traits;
we’ll start with a more general discussion about why newtypes are useful as
types. We’ll then move to type aliases, a feature that is similar to newtypes
but has slightly different semantics. We’ll also discuss the `!` type and
dynamically sized types.

### Using the Newtype Pattern for Type Safety and Abstraction

The newtype pattern that we started discussing at the end of the “Advanced
Traits” section, where we create a new type as a tuple struct with one field
that wraps a type can also be useful for statically enforcing that values are
never confused, and is often used to indicate the units of a value. We actually
had an example of this in Listing 19-26: the `Millimeters` and `Meters` structs
both wrap `u32` values in a new type. If we write a function with a parameter
of type `Millimeters`, we won’t be able to compile a program that accidentally
tries to call that function with a value of type `Meters` or a plain `u32`.

Another reason to use the newtype pattern is to abstract away some
implementation details of a type: the wrapper type can expose a different
public API than the private inner type would if we used it directly in order to
restrict the functionality that is available, for example. New types can also
hide internal generic types. For example, we could provide a `People` type that
wraps a `HashMap<i32, String>` that stores a person’s ID associated with their
name. Code using `People` would only interact with the public API we provide,
such as a method to add a name string to the `People` collection, and that code
wouldn’t need to know that we assign an `i32` ID to names internally. The
newtype pattern is a lightweight way to achieve encapsulation to hide
implementation details that we discussed in Chapter 17.

### Type Aliases Create Type Synonyms

The newtype pattern involves creating a new struct to be a new, separate type.
Rust also provides the ability to declare a *type alias* with the `type`
keyword to give an existing type another name. For example, we can create the
alias `Kilometers` to `i32` like so:

```rust
type Kilometers = i32;
```

This means `Kilometers` is a *synonym* for `i32`; unlike the `Millimeters` and
`Meters` types we created in Listing 19-26, `Kilometers` is not a separate, new
type. Values that have the type `Kilometers` will be treated exactly the same
as values of type `i32`:

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

Since `Kilometers` is an alias for `i32`, they’re the same type. We can add
values of type `i32` and `Kilometers` together, and we can pass `Kilometers`
values to functions that take `i32` parameters. We don’t get the type checking
benefits that we get from the newtype pattern that we discussed in the previous
section.

The main use case for type synonyms is to reduce repetition. For example, we
may have a lengthy type like this:

```rust,ignore
Box<Fn() + Send + 'static>
```

Writing this out in function signatures and as type annotations all over the
place can be tiresome and error-prone. Imagine having a project full of code
like that in Listing 19-31:

```rust
let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // ...snip...
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // ...snip...
#     Box::new(|| ())
}
```

<span class="caption">Listing 19-31: Using a long type in many places</span>

A type alias makes this code more manageable by reducing the amount of
repetition this project has. Here, we’ve introduced an alias named `Thunk` for
the verbose type, and we can replace all uses of the type with the shorter
`Thunk` as shown in Listing 19-32:

```rust
type Thunk = Box<Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // ...snip...
}

fn returns_long_type() -> Thunk {
    // ...snip...
#     Box::new(|| ())
}
```

<span class="caption">Listing 19-32: Introducing a type alias `Thunk` to reduce
repetition</span>

Much easier to read and write! Choosing a good name for a type alias can help
communicate your intent as well (*thunk* is a word for code to be evaluated at
a later time, so it’s an appropriate name for a closure that gets stored).

Another common use of type aliases is with the `Result<T, E>` type. Consider
the `std::io` module in the standard library. I/O operations often return a
`Result<T, E>`, since their operations may fail to work. There’s a
`std::io::Error` struct that represents all of the possible I/O errors. Many of
the functions in `std::io` will be returning `Result<T, E>` where the `E` is
`std::io::Error`, such as these functions in the `Write` trait:

```rust
use std::io::Error;
use std::fmt;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

We’re writing `Result<..., Error>` a lot. As such, `std::io` has this type
alias declaration:

```rust,ignore
type Result<T> = Result<T, std::io::Error>;
```

Because this is in the `std::io` module, the fully qualified alias that we can
use is `std::io::Result<T>`; that is, a `Result<T, E>` with the `E` filled in
as `std::io::Error`. The `Write` trait function signatures end up looking like
this:

```rust,ignore
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}
```

The type alias helps in two ways: this is easier to write *and* it gives us a
consistent interface across all of `std::io`. Because it’s an alias, it is just
another `Result<T, E>`, which means we can use any methods that work on
`Result<T, E>` with it, and special syntax like `?`.

### The Never Type, `!`, that Never Returns

Rust has a special type named `!`. In type theory lingo, it’s called the *empty
type*, because it has no values. We prefer to call it the *never type*. The name
describes what it does: it stands in the place of the return type when a
function will never return. For example:

```rust,ignore
fn bar() -> ! {
    // ...snip...
}
```

This is read as “the function `bar` returns never,” and functions that return
never are called *diverging functions*. We can’t create values of the type `!`,
so `bar` can never possibly return. What use is a type you can never create
values for? If you think all the way back to Chapter 2, we had some code that
looked like this, reproduced here in Listing 19-33:

```rust
# let guess = "3";
# loop {
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
# break;
# }
```

<span class="caption">Listing 19-33: A `match` with an arm that ends in
`continue`</span>

At the time, we skipped over some details in this code. In Chapter 6, we
learned that `match` arms must return the same type. This doesn’t work:

```rust,ignore
let guess = match guess.trim().parse()  {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

What would the type of `guess` be here? It’d have to be both an integer and a
string, and Rust requires that `guess` can only have one type. So what does
`continue` return? Why are we allowed to return a `u32` from one arm in Listing
19-33 and have another arm that ends with `continue`?

As you may have guessed, `continue` has a value of `!`. That is, when Rust goes
to compute the type of `guess`, it looks at both of the match arms. The former
has a value of `u32`, and the latter has a value of `!`. Since `!` can never
have a value, Rust is okay with this, and decides that the type of `guess` is
`u32`. The formal way of describing this behavior of `!` is that the never type
unifies with all other types. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t actually return a value; it instead moves
control back to the top of the loop, so in the `Err` case, we never actually
assign a value to `guess`.

Another use of the never type is `panic!`. Remember the `unwrap` function that
we call on `Option<T>` values to produce a value or panic? Here’s its
definition:

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

Here, the same thing happens as in the `match` in Listing 19-33: we know that
`val` has the type `T`, and `panic!` has the type `!`, so the result of the
overall `match` expression is `T`. This works because `panic!` doesn’t produce
a value; it ends the program. In the `None` case, we won’t be returning a value
from `unwrap`, so this code is valid.

One final expression that has the type `!` is a `loop`:

```rust,ignore
print!("forever ");

loop {
    print!("and ever ");
}
```

Here, the loop never ends, so the value of the expression is `!`. This wouldn’t
be true if we included a `break`, however, as the loop would terminate when it
gets to the `break`.

### Dynamically Sized Types & `Sized`

Because Rust needs to know things like memory layout, there’s a particular
corner of its type system that can be confusing, and that’s the concept of
*dynamically sized types*. Sometimes referred to as ‘DSTs’ or ‘unsized types’,
these types let us talk about types whose size we can only know at runtime.

Let’s dig into the details of a dynamically sized type that we’ve been using
this whole book: `str`. That’s right, not `&str`, but `str` on its own. `str`
is a DST; we can’t know how long the string is until runtime. Since we can’t
know that, we can’t create a variable of type `str`, nor can we take an
argument of type `str`. Consider this code, which does not work:

```rust,ignore
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

These two `str` values would need to have the exact same memory layout, but
they have different lengths: `s1` needs 12 bytes of storage, and `s2` needs 15.
This is why it’s not possible to create a variable holding a dynamically sized
type.

So what to do? Well, you already know the answer in this case: the types of
`s1` and `s2` are `&str` rather than `str`. If you think back to Chapter 4, we
said this about `&str`:

> ... it’s a reference to an internal position in the String and the number of
> elements that it refers to.

So while a `&T` is a single value that stores the memory address of where the
`T` is located, a `&str` is *two* values: the address of the `str` and how long
it is. As such, a `&str` has a size we can know at compile time: it’s two times
the size of a `usize` in length. That is, we always know the size of a `&str`,
no matter how long the string it refers to is. This is the general way in which
dynamically sized types are used in Rust; they have an extra bit of metadata
that stores the size of the dynamic information. This leads us to the golden
rule of dynamically sized types: we must always put values of dynamically sized
types behind a pointer of some kind.

<!-- Note for Carol: `Rc<str>` is only in an accepted RFC right now, check on
its progress and pull this out if it's not going to be stable by Oct -->

While we’ve talked a lot about `&str`, we can combine `str` with all kinds of
pointers: `Box<str>`, for example, or `Rc<str>`. In fact, you’ve already seen
this before, but with a different dynamically sized type: traits. Every trait
is a dynamically sized type we can refer to by using the name of the trait. In
Chapter 17, we mentioned that in order to use traits as trait objects, we have
to put them behind a pointer like `&Trait` or `Box<Trait>` (`Rc<Trait>` would
work too). Traits being dynamically sized is the reason we have to do that!

#### The `Sized` Trait

<!-- If we end up keeping the section on object safety in ch 17, we should add
a back reference here. /Carol -->

To work with DSTs, Rust has a trait that determines if a type’s size is known
at compile time or not, which is `Sized`. This trait is automatically
implemented for everything the compiler knows the size of at compile time. In
addition, Rust implicitly adds a bound on `Sized` to every generic function.
That is, a generic function definition like this:

```rust,ignore
fn generic<T>(t: T) {
    // ...snip...
}
```

is actually treated as if we had written this:

```rust,ignore
fn generic<T: Sized>(t: T) {
    // ...snip...
}
```

By default, generic functions will only work on types that have a known size at
compile time. There is, however, special syntax you can use to relax this
restriction:

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
    // ...snip...
}
```

A trait bound on `?Sized` is the opposite of a trait bound on `Sized`; that is,
we would read this as “`T` may or may not be `Sized`”. This syntax is only
available for `Sized`, no other traits.

Also note we switched the type of the `t` parameter from `T` to `&T`: since the
type might not be `Sized`, we need to use it behind some kind of pointer. In
this case, we’ve chosen a reference.

Next let’s talk about functions and closures!
