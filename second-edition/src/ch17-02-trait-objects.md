## Trait Objects for Using Values of Different Types

In Chapter 8, we said that a limitation of vectors is that vectors can only
store elements of one type. We had an example in Listing 8-1 where we defined a
`SpreadsheetCell` enum that had variants to hold integers, floats, and text so
that we could store different types of data in each cell and still have a
vector represent a row of cells. This works for cases in which the kinds of
things we want to be able to treat interchangeably are a fixed set of types that
we know when our code gets compiled.

<!-- The code example I want to reference did not have a listing number; it's
the one with SpreadsheetCell. I will go back and add Listing 8-1 next time I
get Chapter 8 for editing. /Carol -->

Sometimes we want the set of types that we use to be extensible by the
programmers who use our library. For example, many Graphical User Interface
tools have a concept of a list of items that get drawn on the screen by
iterating through the list and calling a `draw` method on each of the items.
We’re going to create a library crate containing the structure of a GUI library
called `rust_gui`. Our GUI library could include some types for people to use,
such as `Button` or `TextField`. Programmers that use `rust_gui` will want to
create more types that can be drawn on the screen: one programmer might add an
`Image`, while another might add a `SelectBox`. We’re not going to implement a
fully-fledged GUI library in this chapter, but we will show how the pieces
would fit together.

When we’re writing the `rust_gui` library, we don’t know all the types that
other programmers will want to create, so we can’t define an `enum` containing
all the types. What we do know is that `rust_gui` needs to be able to keep
track of a bunch of values of all these different types, and it needs to be
able to call a `draw` method on each of these values. Our GUI library doesn’t
need to know what will happen exactly when we call the `draw` method, just that
the value will have that method available for us to call.

In a language with inheritance, we might define a class named `Component` that
has a method named `draw` on it. The other classes like `Button`, `Image`, and
`SelectBox` would inherit from `Component` and thus inherit the `draw` method.
They could each override the `draw` method to define their custom behavior, but
the framework could treat all of the types as if they were `Component`
instances and call `draw` on them.

### Defining a Trait for the Common Behavior

In Rust, though, we can define a trait that we’ll name `Draw` and that will
have one method named `draw`. Then we can define a vector that takes a *trait
object*, which is a trait behind some sort of pointer, such as a `&` reference
or a `Box<T>` smart pointer. We’ll talk about the reason trait objects have to
be behind a pointer in Chapter 19.

We mentioned that we don’t call structs and enums “objects” to distinguish
structs and enums from other languages’ objects. The data in the struct or enum
fields and the behavior in `impl` blocks is separated, as opposed to other
languages that have data and behavior combined into one concept called an
object. Trait objects *are* more like objects in other languages, in the sense
that they combine the data made up of the pointer to a concrete object with the
behavior of the methods defined in the trait. However, trait objects are
different from objects in other languages because we can’t add data to a trait
object. Trait objects aren’t as generally useful as objects in other languages:
their purpose is to allow abstraction across common behavior.

A trait defines behavior that we need in a given situation. We can then use a
trait as a trait object in places where we would use a concrete type or a
generic type. Rust’s type system will ensure that any value we substitute in
for the trait object will implement the methods of the trait. Then we don’t
need to know all the possible types at compile time, and we can treat all the
instances the same way. Listing 17-3 shows how to define a trait named `Draw`
with one method named `draw`:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Draw {
    fn draw(&self);
}
```

<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

This should look familiar since we talked about how to define traits in
Chapter 10. Next comes something new: Listing 17-4 has the definition of a
struct named `Screen` that holds a vector named `components` that are of type
`Box<Draw>`. That `Box<Draw>` is a trait object: it’s a stand-in for any type
inside a `Box` that implements the `Draw` trait.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

<span class="caption">Listing 17-4: Definition of the `Screen` struct with a
`components` field that holds a vector of trait objects that implement the
`Draw` trait</span>

On the `Screen` struct, we’ll define a method named `run`, which will call the
`draw` method on each of its `components` as shown in Listing 17-5:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
# pub struct Screen {
#     pub components: Vec<Box<Draw>>,
# }
#
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

<span class="caption">Listing 17-5: Implementing a `run` method on `Screen`
that calls the `draw` method on each component</span>

This is different than defining a struct that uses a generic type parameter
with trait bounds. A generic type parameter can only be substituted with one
concrete type at a time, while trait objects allow for multiple concrete types
to fill in for the trait object at runtime. For example, we could have defined
the `Screen` struct using a generic type and a trait bound as in Listing 17-6:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

<span class="caption">Listing 17-6: An alternate implementation of the `Screen`
struct and its `run` method using generics and trait bounds</span>

This only lets us have a `Screen` instance that has a list of components that
are all of type `Button` or all of type `TextField`. If you’ll only ever have
homogeneous collections, using generics and trait bounds is preferable since
the definitions will be monomorphized at compile time to use the concrete types.

With the definition of `Screen` that holds a component list of trait objects in
`Vec<Box<Draw>>` instead, one `Screen` instance can hold a `Vec` that contains
a `Box<Button>` as well as a `Box<TextField>`. Let’s see how that works, and
then talk about the runtime performance implications.

### Implementations of the Trait from Us or Library Users

Now to add some types that implement the `Draw` trait. We’re going to provide
the `Button` type, and again, actually implementing a GUI library is out of
scope of this book, so the `draw` method won’t have any useful implementation
in its body. To imagine what the implementation might look like, a `Button`
struct might have fields for `width`, `height`, and `label`, as shown in
Listing 17-7:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}
```

<span class="caption">Listing 17-7: A `Button` struct that implements the
`Draw` trait</span>

The `width`, `height`, and `label` fields on `Button` will differ from other
components, such as a `TextField` type that might have `width`, `height`,
`label`, and `placeholder` fields instead. Each of the types that we want to be
able to draw on the screen will implement the `Draw` trait with different code
in the `draw` method that defines how to draw that type like `Button` has here
(without any actual GUI code that’s out of scope of this chapter). In addition
to implementing the `Draw` trait, `Button` might also have another `impl` block
containing methods having to do with what happens if the button is clicked.
These kinds of methods won’t apply to types like `TextField`.

Someone using our library has decided to implement a `SelectBox` struct that
has `width`, `height`, and `options` fields. They implement the `Draw` trait on
the `SelectBox` type as well, as shown in Listing 17-8:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rust_gui;
use rust_gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
    }
}
```

<span class="caption">Listing 17-8: Another crate using `rust_gui` and
implementing the `Draw` trait on a `SelectBox` struct</span>

The user of our library can now write their `main` function to create a
`Screen` instance and add a `SelectBox` and a `Button` to the screen by putting
each in a `Box<T>` to become a trait object. They can then call the `run`
method on the `Screen` instance, which will call `draw` on each of the
components. Listing 17-9 shows this implementation:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use rust_gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

<span class="caption">Listing 17-9: Using trait objects to store values of
different types that implement the same trait</span>

Even though we didn’t know that someone would add the `SelectBox` type someday,
our `Screen` implementation was able to operate on the `SelectBox` and draw it
because `SelectBox` implements the `Draw` type, which means it implements the
`draw` method.

Only being concerned with the messages a value responds to, rather than the
value’s concrete type, is similar to a concept called *duck typing* in
dynamically typed languages: if it walks like a duck, and quacks like a duck,
then it must be a duck! In the implementation of `run` on `Screen` in Listing
17-5, `run` doesn’t need to know what the concrete type of each component is.
It doesn’t check to see if a component is an instance of a `Button` or a
`SelectBox`, it just calls the `draw` method on the component. By specifying
`Box<Draw>` as the type of the values in the `components` vector, we’ve defined
that `Screen` needs values that we can call the `draw` method on.

The advantage with using trait objects and Rust’s type system to do duck typing
is that we never have to check that a value implements a particular method at
runtime or worry about getting errors if a value doesn’t implement a method but
we call it. Rust won’t compile our code if the values don’t implement the
traits that the trait objects need.

For example, Listing 17-10 shows what happens if we try to create a `Screen`
with a `String` as a component:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rust_gui;
use rust_gui::Draw;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}
```

<span class="caption">Listing 17-10: Attempting to use a type that doesn’t
implement the trait object’s trait</span>

We’ll get this error because `String` doesn’t implement the `Draw` trait:

```text
error[E0277]: the trait bound `std::string::String: Draw` is not satisfied
  -->
   |
 4 |             Box::new(String::from("Hi")),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not
   implemented for `std::string::String`
   |
   = note: required for the cast to the object type `Draw`
```

This lets us know that either we’re passing something we didn’t mean to pass to
`Screen` and we should pass a different type, or we should implement `Draw` on
`String` so that `Screen` is able to call `draw` on it.

### Trait Objects Perform Dynamic Dispatch

Recall in Chapter 10 when we discussed the process of monomorphization that the
compiler performs when we use trait bounds on generics: the compiler generates
non-generic implementations of functions and methods for each concrete type
that we use in place of a generic type parameter. The code that results from
monomorphization is doing *static dispatch*: when the method is called, the
code that goes with that method call has been determined at compile time, and
looking up that code is very fast.

When we use trait objects, the compiler can’t perform monomorphization because
we don’t know all the types that might be used with the code. Instead, Rust
keeps track of the code that might be used when a method is called and figures
out at runtime which code needs to be used for a particular method call. This
is known as *dynamic dispatch*, and there’s a runtime cost when this lookup
happens. Dynamic dispatch also prevents the compiler from choosing to inline a
method’s code, which prevents some optimizations. We did get extra flexibility
in the code that we wrote and were able to support, though, so it’s a tradeoff
to consider.

### Object Safety is Required for Trait Objects

<!-- Liz: we're conflicted on including this section. Not being able to use a
trait as a trait object because of object safety is something that
beginner/intermediate Rust developers run into sometimes, but explaining it
fully is long and complicated. Should we just cut this whole section? Leave it
(and finish the explanation of how to fix the error at the end)? Shorten it to
a quick caveat, that just says something like "Some traits can't be trait
objects. Clone is an example of one. You'll get errors that will let you know
if a trait can't be a trait object, look up object safety if you're interested
in the details"? Thanks! /Carol -->

Not all traits can be made into trait objects; only *object safe* traits can. A
trait is object safe as long as both of the following are true:

* The trait does not require `Self` to be `Sized`
* All of the trait’s methods are object safe.

`Self` is a keyword that is an alias for the type that we’re implementing
traits or methods on. `Sized` is a marker trait like the `Send` and `Sync`
traits that we talked about in Chapter 16. `Sized` is automatically implemented
on types that have a known size at compile time, such as `i32` and references.
Types that do not have a known size include slices (`[T]`) and trait objects.

`Sized` is an implicit trait bound on all generic type parameters by default.
Most useful operations in Rust require a type to be `Sized`, so making `Sized`
a default requirement on trait bounds means we don’t have to write `T: Sized`
with most every use of generics. If we want to be able to use a trait on
slices, however, we need to opt out of the `Sized` trait bound, and we can do
that by specifying `T: ?Sized` as a trait bound.

Traits have a default bound of `Self: ?Sized`, which means that they can be
implemented on types that may or may not be `Sized`. If we create a trait `Foo`
that opts out of the `Self: ?Sized` bound, that would look like the following:

```rust
trait Foo: Sized {
    fn some_method(&self);
}
```

The trait `Sized` is now a *supertrait* of trait `Foo`, which means trait `Foo`
requires types that implement `Foo` (that is, `Self`) to be `Sized`. We’re
going to talk about supertraits in more detail in Chapter 19.

`Foo` requires `Self` to be `Sized`, and therefore is not allowed to be used in 
a trait object like `Box<Foo>`. This is because it would be impossible to implement
the trait `Foo` for a trait object like `Box<Foo>`: trait objects aren’t sized, 
but `Foo` requires `Self` to be `Sized`. A type can’t be both sized and unsized
at the same time!

For the second object safety requirement that says all of a trait’s methods
must be object safe, a method is object safe if either:

* It requires `Self` to be `Sized` or
* It meets all three of the following:
    * It must not have any generic type parameters
    * Its first argument must be of type `Self` or a type that dereferences to
      the Self type (that is, it must be a method rather than an associated
      function and have `self`, `&self`, or `&mut self` as the first argument)
    * It must not use `Self` anywhere else in the signature except for the
      first argument

Those rules are a bit formal, but think of it this way: if your method requires
the concrete `Self` type somewhere in its signature, but an object forgets the
exact type that it is, there’s no way that the method can use the original
concrete type that it’s forgotten. Same with generic type parameters that are
filled in with concrete type parameters when the trait is used: the concrete
types become part of the type that implements the trait. When the type is
erased by the use of a trait object, there’s no way to know what types to fill
in the generic type parameters with.

An example of a trait whose methods are not object safe is the standard
library’s `Clone` trait. The signature for the `clone` method in the `Clone`
trait looks like this:

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

`String` implements the `Clone` trait, and when we call the `clone` method on
an instance of `String` we get back an instance of `String`. Similarly, if we
call `clone` on an instance of `Vec`, we get back an instance of `Vec`. The
signature of `clone` needs to know what type will stand in for `Self`, since
that’s the return type.

If we try to implement `Clone` on a trait like the `Draw` trait from Listing
17-3, we wouldn’t know whether `Self` would end up being a `Button`, a
`SelectBox`, or some other type that will implement the `Draw` trait in the
future.

The compiler will tell you if you’re trying to do something that violates the
rules of object safety in regards to trait objects. For example, if we had
tried to implement the `Screen` struct in Listing 17-4 to hold types that
implement the `Clone` trait instead of the `Draw` trait, like this:

```rust,ignore
pub struct Screen {
    pub components: Vec<Box<Clone>>,
}
```

We’ll get this error:

```text
error[E0038]: the trait `std::clone::Clone` cannot be made into an object
 -->
  |
2 |     pub components: Vec<Box<Clone>>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` cannot be
  made into an object
  |
  = note: the trait cannot require that `Self : Sized`
```

<!-- If we are including this section, we would explain how to fix this
problem. It involves adding another trait and implementing Clone manually for
that trait. Because this section is getting long, I stopped because it feels
like we're off in the weeds with an esoteric detail that not everyone will need
to know about. /Carol -->
