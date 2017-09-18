## Treating Smart Pointers like Regular References with the `Deref` Trait

Implementing `Deref` trait allows us to customize the behavior of the
*dereference operator* `*`(as opposed to the multiplication or glob operator).
By implementing `Deref` in such a way that a smart pointer can be treated like
a regular reference, we can write code that operates on references and use that
code with smart pointers too.

<!-- Why would we want to override the dereference operator? Can you lay that
out? -->
<!-- Attempted above. /Carol -->

<!-- I'd suggest introducing what you mean by "convenient" here, if we are
using it as the reason we want to use Deref -->
<!-- I've removed convenient from the first paragraph and foreshadowed in a
different way in the below paragraph /Carol -->

Let's first take a look at how `*` works with regular references, then try and
define our own type like `Box<T>` and see why `*` doesn't work like a
reference. We'll explore how implementing the `Deref` trait makes it possible
for smart pointers to work in a similar way as references. Finally, we'll look
at the *deref coercion* feature of Rust and how that lets us work with either
references or smart pointers.

### Following the Pointer to the Value with `*`

<!-- I want to avoid too much cross referencing, I think it can be distracting,
make the reader feel they need to flip back but they don't really, here -->
<!-- Ok, guess we went too far then! I've been adding *more* cross referencing
so that the reader can go back if they've forgotten something we've already
covered. /Carol -->

<!--Oh! I see, de-reference, meaning we cut the tie between the data and the
reference? I've assumed so above, please correct if not! -->
<!-- I wouldn't describe it as "cutting the tie"; the tie is still there. It's
more like we're following an arrow (the pointer) to find the value. Let us know
if this explanation is still unclear. /Carol -->

A regular reference is a type of pointer, and one way to think of a pointer is
that it's an arrow to a value stored somewhere else. In Listing 15-8, let's
create a reference to an `i32` value then use the dereference operator to
follow the reference to the data:

<!-- We'll start with an example of dereferencing and re-allocating references
to `i32` values: -->
<!-- Is this what this is an example of? -->
<!-- No, there isn't any re-allocation happening here; allocation is a term
that means asking for more space in order to hold data (as we covered in
chapter 4). What were you trying to convey with "re-allocating", exactly? Have
we addressed whatever was confusing here before? /Carol -->

<!-- We've reworked the following sections in this chapter heavily because the
`Mp3` example seemed to be confusing with the metadata that was involved.
Interested to see if this breakdown works better or not. /Carol -->

<span class="filename">Filename: src/main.rs

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-8: Using the dereference operator to follow a
reference to an `i32` value</span>

The variable `x` holds an `i32` value, `5`. We set `y` equal to a reference to
`x`. We can assert that `x` is equal to `5`. However, if we want to make an
assertion about the value in `y`, we have to use `*y` to follow the reference
to the value that the reference is pointing to (hence *de-reference*). Once we
de-reference `y`, we have access to the integer value `y` is pointing to that
we can compare with `5`.

If we try to write `assert_eq!(5, y);` instead, we'll get this compilation
error:

```text
error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<&{integer}>` is
not satisfied
 --> <assert_eq macros>:5:19
  |
5 | if ! ( * left_val == * right_val ) {
  |                   ^^ can't compare `{integer}` with `&{integer}`
  |
  = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for
  `{integer}`
```

Comparing a reference to a number with a number isn't allowed because they're
different types. We have to use `*` to follow the reference to the value it's
pointing to.

### Using `Box<T>` Like a Reference

We can rewrite the code in Listing 15-8 to use a `Box<T>` instead of a
reference, and the de-reference operator will work the same way as shown in
Listing 15-9:

<span class="filename">Filename: src/main.rs

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-9: Using the dereference operator on a
`Box<i32>`</span>

The only part of Listing 15-8 that we changed was to set `y` to be an instance
of a box pointing to the value in `x` rather than a reference pointing to the
value of `x`. In the last assertion, we can use the dereference operator to
follow the box's pointer in the same way that we did when `y` was a reference.
Let's explore what is special about `Box<T>` that enables us to do this by
defining our own box type.

### Defining Our Own Smart Pointer

Let's build a smart pointer similar to the `Box<T>` type that the standard
library has provided for us, in order to experience that smart pointers don't
behave like references by default. Then we'll learn about how to add the
ability to use the dereference operator.

`Box<T>` is ultimately defined as a tuple struct with one element, so Listing
15-10 defines a `MyBox<T>` type in the same way. We'll also define a `new`
function to match the `new` function defined on `Box<T>`:

<span class="filename">Filename: src/main.rs</span>

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

<span class="caption">Listing 15-10: Defining a `MyBox<T>` type</span>

We define a struct named `MyBox` and declare a generic parameter `T`, since we
want our type to be able to hold values of any type. `MyBox` is a tuple struct
with one element of type `T`. The `MyBox::new` function takes one parameter of
type `T` and returns a `MyBox` instance that holds the value passed in.

Let's try adding the code from Listing 15-9 to the code in Listing 15-10 and
changing `main` to use the `MyBox<T>` type we've defined instead of `Box<T>`.
The code in Listing 15-11 won't compile because Rust doesn't know how to
dereference `MyBox`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

<span class="caption">Listing 15-11: Attempting to use `MyBox<T>` in the same
way we were able to use references and `Box<T>`</span>

The compilation error we get is:

```text
error: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^
```

Our `MyBox<T>` type can't be dereferenced because we haven't implemented that
ability on our type. To enable dereferencing with the `*` operator, we can
implement the `Deref` trait.

### Implementing the `Deref` Trait Defines How To Treat a Type Like a Reference

As we discussed in Chapter 10, in order to implement a trait, we need to
provide implementations for the trait's required methods. The `Deref` trait,
provided by the standard library, requires implementing one method named
`deref` that borrows `self` and returns a reference to the inner data. Listing
15-12 contains an implementation of `Deref` to add to the definition of `MyBox`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::ops::Deref;

# struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

<span class="caption">Listing 15-12: Implementing `Deref` on `MyBox<T>`</span>

The `type Target = T;` syntax defines an associated type for this trait to use.
Associated types are a slightly different way of declaring a generic parameter
that you don't need to worry about too much for now; we'll cover it in more
detail in Chapter 19.

<!-- Is it possible to just use a method for declaring a generic parameter we
have seen before, so we can focus on the deref trait here? -->
<!-- No, this is how the `Deref` trait is defined in the standard library, so
this is what you have to specify in order to implement it. /Carol -->

We filled in the body of the `deref` method with `&self.0` so that `deref`
returns a reference to the value we want to access with the `*` operator. The
`main` function from Listing 15-11 that calls `*` on the `MyBox<T>` value now
compiles and the assertions pass!

Without the `Deref` trait, the compiler can only dereference `&` references.
The `Deref` trait's `deref` method gives the compiler the ability to take a
value of any type that implements `Deref` and call the `deref` method in order
to get a `&` reference that it knows how to dereference.

When we typed `*y` in Listing 15-11, what Rust actually ran behind the scenes
was this code:

```rust,ignore
*(y.deref())
```

<!-- why is that happening behind the scenes, rather than us just calling this
up front? -->
<!-- we've tried to clarify below /Carol -->

Rust substitutes the `*` operator with a call to the `deref` method and then a
plain dereference so that we don't have to think about when we have to call the
`deref` method or not. This feature of Rust lets us write code that functions
identically whether we have a regular reference or a type that implements
`Deref`.

The reason the `deref` method returns a reference to a value, and why the plain
dereference outside the parentheses in `*(y.deref())` is still necessary, is
because of ownership. If the `deref` method returned the value directly instead
of a reference to the value, the value would be moved out of `self`. We don’t
want to take ownership of the inner value inside `MyBox<T>` in this case and in
most cases where we use the dereference operator.

Note that replacing `*` with a call to the `deref` method and then a call to
`*` happens once, each time we type a `*` in our code. The substitution of `*`
does not recurse infinitely. That’s how we end up with data of type `i32`,
which matches the `5` in the `assert_eq!` in Listing 15-11.

### Implicit Deref Coercions with Functions and Methods

<!--Below -- "A deref coercion happens when..." So this isn't something the
reader is making happen, but something that just happens behind the scene? If
not, can you change this to an active tone? -->
<!-- Yes, it is something that happens behind the scenes, which is why we
describe it as implicit. /Carol -->

*Deref coercion* is a convenience that Rust performs on arguments to functions
and methods. Deref coercion converts a reference to a type that implements
`Deref` into a reference to a type that `Deref` can convert the original type
into. Deref coercion happens automatically when we pass a reference to a value
of a particular type as an argument to a function or method that doesn't match
the type of the parameter in the function or method definition, and there's a
sequence of calls to the `deref` method that will convert the type we provided
into the type that the parameter needs.

Deref coercion was added to Rust so that programmers writing function and
method calls don't need to add as many explicit references and dereferences
with `&` and `*`. This feature also lets us write more code that can work for
either references or smart pointers.

To illustrate deref coercion in action, let's use the `MyBox<T>` type we
defined in Listing 15-10 as well as the implementation of `Deref` that we added
in Listing 15-12. Listing 15-13 shows the definition of a function that has a
string slice parameter:

<span class="filename">Filename: src/main.rs</span>

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

<span class="caption">Listing 15-13: A `hello` function that has the parameter
`name` of type `&str`</span>

We can call the `hello` function with a string slice as an argument, like
`hello("Rust");` for example. Deref coercion makes it possible for us to call
`hello` with a reference to a value of type `MyBox<String>`, as shown in
Listing 15-14:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

<span class="caption">Listing 15-14: Calling `hello` with a reference to a
`MyBox<String>`, which works because of deref coercion</span>

Here we're calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-12, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, which we can see in the API
documentation for `Deref`. Rust calls `deref` again to turn the `&String` into
`&str`, which matches the `hello` function's definition.

If Rust didn't implement deref coercion, in order to call `hello` with a value
of type `&MyBox<String>`, we'd have to write the code in Listing 15-15 instead
of the code in Listing 15-14:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::ops::Deref;
#
# struct MyBox<T>(T);
#
# impl<T> MyBox<T> {
#     fn new(x: T) -> MyBox<T> {
#         MyBox(x)
#     }
# }
#
# impl<T> Deref for MyBox<T> {
#     type Target = T;
#
#     fn deref(&self) -> &T {
#         &self.0
#     }
# }
#
# fn hello(name: &str) {
#     println!("Hello, {}!", name);
# }
#
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

<span class="caption">Listing 15-15: The code we'd have to write if Rust didn't
have deref coercion</span>

The `(*m)` is dereferencing the `MyBox<String>` into a `String`. Then the `&`
and `[..]` are taking a string slice of the `String` that is equal to the whole
string to match the signature of `hello`. The code without deref coercions is
harder to read, write, and understand with all of these symbols involved. Deref
coercion makes it so that Rust takes care of these conversions for us
automatically.

When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as it needs in order to get a
reference to match the parameter's type. This is resolved at compile time, so
there is no run-time penalty for taking advantage of deref coercion!

### How Deref Coercion Interacts with Mutability

<!-- below: are we talking about any mutable references, or are we talking
about mutable generic types, below? Can you make sure it's clear throughout, I
wasn't 100% -->
<!-- I'm not sure what you're asking, *types* don't have the property of
mutability or immutability, it's the variables or references to *instances* of
those types that are mutable or immutable. Also the way to say "any mutable
reference" is with `&mut` and a generic type parameter. Is that what's
confusing? /Carol -->

Similar to how we use the `Deref` trait to override `*` on immutable
references, Rust provides a `DerefMut` trait for overriding `*` on mutable
references.

Rust does deref coercion when it finds types and trait implementations in three
cases:

<!-- Would it make sense to move this list to the start of the deref section?
-->
<!-- I don't think this list makes very much sense until you understand what
deref coercion *is*. Can you elaborate on why you think it should be moved to
the beginning? /Carol -->

* From `&T` to `&U` when `T: Deref<Target=U>`.
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`.
* From `&mut T` to `&U` when `T: Deref<Target=U>`.

The first two cases are the same except for mutability. The first case says
that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can
get a `&U` transparently. The second case states that the same deref coercion
happens for mutable references.

The last case is trickier: Rust will also coerce a mutable reference to an
immutable one. The reverse is *not* possible though: immutable references will
never coerce to mutable ones. Because of the borrowing rules, if you have a
mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn't compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that
there was only one immutable reference to that data, and the borrowing rules
don't guarantee that. Therefore, Rust can't make the assumption that converting
an immutable reference to a mutable reference is possible.

<!-- Why does it coerce to an immutable reference, and why cant it go the other
way?-->
<!-- Elaborated above /Carol-->
