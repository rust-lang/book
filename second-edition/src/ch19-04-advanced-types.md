## Advanced Types

The Rust type system has some features that we’ve mentioned in this book but
haven’t yet discussed. We’ll start our discussion on advanced types with a more
general discussion about why newtypes are useful as types. We’ll then move to
type aliases, a feature similar to newtypes but with slightly different
semantics. We’ll also discuss the `!` type and dynamically sized types.

### Using the Newtype Pattern for Type Safety and Abstraction

> This section assumes you’ve read the newtype pattern section in the “Advanced
> Traits” section.

The newtype pattern is useful for other things beyond what we’ve discussed so
far, including statically enforcing that values are never confused, and as
indication of the units of a value. We actually had an example of this in
Listing 19-26: the `Millimeters` and `Meters` structs both wrap `u32` values in
a newtype. If we write a function with a parameter of type `Millimeters`, we
won’t be able to compile a program that accidentally tries to call that
function with a value of type `Meters` or a plain `u32`.

Another use is in abstracting away some implementation details of a type: the
wrapper type can expose a public API that’s different to the API of the private
inner type, if we used it directly to restrict the available functionality, for
example.

Newtypes can also hide internal generic types. For example, we could provide a
`People` type to wrap a `HashMap<i32, String>` that stores a person’s ID
associated with their name. Code using `People` would only interact with the
public API we provide, such as a method to add a name string to the `People`
collection, and that code wouldn’t need to know that we assign an `i32` ID to
names internally. The newtype pattern is a lightweight way to achieve
encapsulation to hide implementation details that we discussed in the
“Encapsulation that Hides Implementation Details” section of Chapter 17.

### Type Aliases Create Type Synonyms

Alongside the newtype pattern, Rust provides the ability to declare a *type
alias* to give an existing type another name. For this we use the `type`
keyword. For example, we can create the alias `Kilometers` to `i32` like so:

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

Because `Kilometers` and `i32`, are the same type, we can add values of both
types, and we can also pass `Kilometers` values to functions that take `i32`
parameters. With this method, though, we don’t get the type checking benefits
that we get from the newtype pattern discussed in the previous section.

The main use case for type synonyms is to reduce repetition. For example, we
may have a lengthy type like this:

```rust,ignore
Box<Fn() + Send + 'static>
```

Writing this out in function signatures and as type annotations all over the
place can be tiresome and error-prone. Imagine having a project full of code
like that in Listing 19-35:

```rust
let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // --snip--
#     Box::new(|| ())
}
```

<span class="caption">Listing 19-35: Using a long type in many places</span>

A type alias makes this code more manageable by reducing the repetition. Here,
we’ve introduced an alias named `Thunk` for the verbose type, and can replace
all uses of the type with the shorter `Thunk` as shown in Listing 19-36:

```rust
type Thunk = Box<Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
#     Box::new(|| ())
}
```

<span class="caption">Listing 19-36: Introducing a type alias `Thunk` to reduce
repetition</span>

Much easier to read and write! Choosing a good name for a type alias can help
communicate your intent as well (*thunk* is a word for code to be evaluated at
a later time, so it’s an appropriate name for a closure that gets stored).

Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition. Consider the `std::io` module in the standard library. I/O
operations often return a `Result<T, E>` to handle situations when operations
fail to work. This library has a `std::io::Error` struct that represents all
possible I/O errors. Many of the functions in `std::io` will be returning
`Result<T, E>` where the `E` is `std::io::Error`, such as these functions in
the `Write` trait:

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

We have `Result<..., Error>` repeated a lot. As such, `std::io` has this type
alias declaration:

```rust,ignore
type Result<T> = Result<T, std::io::Error>;
```

Because this is in the `std::io` module, we can use the fully qualified alias
`std::io::Result<T>`; that is, a `Result<T, E>` with the `E` filled in as
`std::io::Error`. The `Write` trait function signatures end up looking like
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
`Result<T, E>` with it, as well as special syntax like `?`.

### The `!` Never Type that Never Returns

Rust has a special type named `!` that’s known in type theory lingo as the
*empty type*, because it has no values. We prefer to call it the *never type*,
because it stands in the place of the return type when a function will never
return. For example:

```rust,ignore
fn bar() -> ! {
    // --snip--
}
```

This is read as “the function `bar` returns never.” Functions that return never
are called *diverging functions*. We can’t create values of the type `!`, so
`bar` can never possibly return.

But what use is a type you can never create values for? If you think all the
way back to Chapter 2, we had some code that looked like the code we’ve
reproduced here in Listing 19-37:

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

<span class="caption">Listing 19-37: A `match` with an arm that ends in
`continue`</span>

At the time, we skipped over some details in this code. In Chapter 6 in “The
`match` Control Flow Operator” section, we covered that `match` arms must all
return the same type. This, for example, doesn’t work:

```rust,ignore
let guess = match guess.trim().parse()  {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

The type of `guess` here would have to be both an integer and a string, and
Rust requires that `guess` can only have one type. So what does `continue`
return? How were we allowed to return a `u32` from one arm and have another arm
that ends with `continue` in Listing 19-37?

As you may have guessed, `continue` has a value of `!`. That is, when Rust goes
to compute the type of `guess`, it looks at both of the match arms, the former
with a value of `u32`, and the latter a value of `!`. Because `!` can never
have a value, Rust decides that the type of `guess` is `u32`.

The formal way of describing this behavior is that expressions of type `!` can
be coerced into any other type. We’re allowed to end this `match` arm with
`continue` because `continue` doesn’t actually return a value; it instead moves
control back to the top of the loop, so in the `Err` case, we never actually
assign a value to `guess`.

<!-- I'm not sure I'm following what would then occur in the event of an error,
literally nothing? -->
<!-- The block returns control to the enclosing loop; I'm not sure how to
clarify this other than what we already have here, do you have any suggestions?
I wouldn't say it's "literally nothing" because it does do something, it
returns control to the loop and the next iteration of the loop happens...
/Carol -->

The never type is also useful with `panic!`. Remember the `unwrap` function
that we call on `Option<T>` values to produce a value or panic? Here’s its
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
got to the `break`.

### Dynamically Sized Types & `Sized`

Due to Rust’s need to know things like how much space to allocate for a value
of a particular type, there’s a corner of its type system that can be
confusing: the concept of *dynamically sized types*. Sometimes referred to as
‘DSTs’ or ‘unsized types’, these types let us talk about types whose size we
can only know at runtime.

Let’s dig into the details of a dynamically sized type that we’ve been using
this whole book: `str`. That’s right, not `&str`, but `str` on its own, is a
DST. We can’t know how long the string is until runtime, meaning we can’t
create a variable of type `str`, nor can we take an argument of type `str`.
Consider this code, which does not work:

```rust,ignore
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

<!-- Why do they need to have the same memory layout? Perhaps I'm not
understanding fully what is meant by the memory layout, is it worth explaining
that a little in this section? -->
<!-- I've reworded /Carol -->

Rust needs to know how much memory to allocate for any value of a particular
type, and all values of a type must use the same amount of memory. If we were
allowed to write this code, that would mean these two `str` values would need
to take up the exact same amount of space, but they have different lengths:
`s1` needs 12 bytes of storage, and `s2` needs 15. This is why it’s not
possible to create a variable holding a dynamically sized type.

So what to do? You already know the answer in this case: we make the types of
`s1` and `s2` a `&str` rather than `str`. If you think back to the “String
Slices” section of Chapter 4, we said that the slice data structure stores the
starting position and the length of the slice.

So while a `&T` is a single value that stores the memory address of where the
`T` is located, a `&str` is *two* values: the address of the `str` and its
length. As such, a `&str` has a size we can know at compile time: it’s two
times the size of a `usize` in length. That is, we always know the size of a
`&str`, no matter how long the string it refers to is. This is the general way
in which dynamically sized types are used in Rust; they have an extra bit of
metadata that stores the size of the dynamic information. This leads us to the
golden rule of dynamically sized types: we must always put values of
dynamically sized types behind a pointer of some kind.

We can combine `str` with all kinds of pointers: `Box<str>`, for example, or
`Rc<str>`. In fact, you’ve seen this before, but with a different dynamically
sized type: traits. Every trait is a dynamically sized type we can refer to by
using the name of the trait. In Chapter 17 in the “Using Trait Objects that
Allow for Values of Different Types” section, we mentioned that in order to use
traits as trait objects, we have to put them behind a pointer like `&Trait` or
`Box<Trait>` (`Rc<Trait>` would work too). Traits being dynamically sized is
the reason we have to do that!

#### The `Sized` Trait

<!-- If we end up keeping the section on object safety in ch 17, we should add
a back reference here. /Carol -->

<!-- I think we dropped that one, right? -->
<!-- We cut a large portion of it, including the part about `Sized`, so I
didn't add a back reference. /Carol -->

To work with DSTs, Rust has a particular trait to determine if a type’s size is
known at compile time or not: the `Sized` trait. This trait is automatically
implemented for everything whose size is known at compile time. In addition,
Rust implicitly adds a bound on `Sized` to every generic function. That is, a
generic function definition like this:

```rust,ignore
fn generic<T>(t: T) {
    // --snip--
}
```

is actually treated as if we had written this:

```rust,ignore
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will only work on types that have a known size at
compile time. There is, however, special syntax you can use to relax this
restriction:

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

A trait bound on `?Sized` is the opposite of a trait bound on `Sized`; that is,
we would read this as “`T` may or may not be `Sized`”. This syntax is only
available for `Sized`, no other traits.

Also note we switched the type of the `t` parameter from `T` to `&T`: because
the type might not be `Sized`, we need to use it behind some kind of pointer.
In this case, we’ve chosen a reference.

Next let’s talk about functions and closures!
