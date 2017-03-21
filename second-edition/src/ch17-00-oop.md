# Is Rust an Object-Oriented Programming Language?

Object-Oriented Programming is a way of modeling programs that originated with
Simula in the 1960s and became popular with C++ in the 1990s. There are many
competing definitions for what OOP is: under some definitions, Rust is
object-oriented; under other definitions, Rust is not. In this chapter, we'll
explore some characteristics that are commonly considered to be object-oriented
and how those characteristics translate to idiomatic Rust.

## What Does Object-Oriented Mean?

There isn't consensus in the programming community about the features a
language needs to have in order to be called object-oriented. Rust is
influenced by many different programming paradigms; we explored the features it
has that come from functional programming in Chapter 13. Some of the
characteristics that object-oriented programming languages tend to share are
objects, encapsulation, and inheritance. Let's take a look at what each of
those mean and whether Rust supports them.

### Objects Contain Data and Behavior

The book "Design Patterns: Elements of Reusable Object-Oriented Software,"
colloquially referred to as "The Gang of Four book", is a catalog of
object-oriented design patterns. It defines object-oriented programming in this
way:

> Object-oriented programs are made up of objects. An *object* packages both
> data and the procedures that operate on that data. The procedures are
> typically called *methods* or *operations*.

Under this definition, then, Rust is object-oriented: structs and enums have
data and `impl` blocks provide methods on structs and enums. Even though
structs and enums with methods aren't *called* objects, they provide the same
functionality that objects do, using the Gang of Four's definition of objects.

### Encapsulation that Hides Implementation Details

Another aspect commonly associated with object-oriented programming is the idea
of *encapsulation*: the implementation details of an object aren't accessible
to code using that object. The only way to interact with an object is through
the public API the object offers; code using the object should not be able to
reach into the object's internals and change data or behavior directly.
Encapsulation enables changing and refactoring an object's internals without
needing to change the code that uses the object.

As we discussed in Chapter 7, we can use the `pub` keyword to decide what
modules, types, functions, and methods in our code should be public, and by
default, everything is private. For example, we can define a struct
`CountedCollection` that has a field containing a vector of `i32` values. The
struct can also have a field that knows the number of values in the vector so
that whenever anyone wants to know how many values the struct has in its
vector, we don't have to count the values in the vector again. Listing 17-1 has
the definition of the `CountedCollection` struct:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct CountedCollection {
    list: Vec<i32>,
    count: usize,
}
```

<span class="caption">Listing 17-1: A `CountedCollection` struct that maintains
a list of items and the count of how many items are in the collection.</span>

Note that the struct itself is marked `pub` so that other code may use this
struct, but the fields within the struct remain private. This is important in
this case because we want to ensure that whenever a value is added or removed
from the list, we also update the count. We do this by implementing `add`,
`remove`, and `count` methods on the struct as shown in Listing 17-2:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct CountedCollection {
#     list: Vec<i32>,
#     count: usize,
# }
impl CountedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.count += 1;
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.count -= 1;
                Some(value)
            },
            None => None,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
```

<span class="caption">Listing 17-2: Implementations of the public methods
`add`, `remove`, and `count` on `CountedCollection`</span>

The `add`, `remove`, and `count` methods are public, and are the only way to
modify an instance of a `CountedCollection`. When an item is added to `list`
using the `add` method or removed using the `remove` method, the
implementations of those methods take care of updating the `count` field as
well. Because the `list` and `count` fields are private, there's no way for
external code to add or remove items to the `list` field directly, which could
cause the `count` field to get out of sync. The `count` method returns the
value in the `count` field, which allows external code to read the `count` but
not modify it.

Because we've encapsulated the implementation details of `CountedCollection`,
we could also change aspects like using a different data structure used for the
`list` to use a `HashSet` instead of a `Vec`, for instance. As long as the
signatures of the `add`, `remove`, and `count` public methods stayed the same,
code using `CountedCollection` wouldn't need to change. This wouldn't
necessarily be the case if we exposed `list` to external code: `HashSet` and
`Vec` have different methods for adding and removing items, so the external
code would likely have to change if it was modifying `list` directly.

If encapsulation is a required aspect for a language to be considered
object-oriented, then Rust meets that requirement. Using `pub` or not for
different parts of code enables encapsulation of implementation details.

### Inheritance as a Type System and as Code Sharing

*Inheritance* is a mechanism that some programming languages provide whereby an
object can be defined to inherit from another object's definition, thus gaining
the parent object's data and behavior without having to define those again.
Inheritance is a characteristic that is part of some people's definitions of
what an OOP language is.

If a language must have inheritance to be an object-oriented language, then
Rust is not object-oriented. There is not a way to define a struct that
inherits from another struct in order to gain the parent struct's fields and
method implementations. However, if you're used to having inheritance in your
programming toolbox, there are other solutions in Rust depending on the reason
you want to use inheritance.

There are two main reasons to reach for inheritance. The first is to be able to
re-use code: once a particular behavior is implemented for one type,
inheritance can enable re-using that implementation for a different type. Rust
code can be shared using default trait method implementations instead, which we
saw in Listing 10-14 when we added a default implementation of the `summary`
method on the `Summarizable` trait. Any type implementing the `Summarizable`
trait would have the `summary` method available on it without any further code.
This is similar to a parent class having an implementation of a method, and a
child class inheriting from the parent class also having the implementation of
the method due to the inheritance. We can also choose to override the default
implementation of the `summary` method when we implement the `Summarizable`
trait, which is similar to a child class overriding the implementation of a
method inherited from a parent class.

The second reason to use inheritance is with the type system: to express that a
child type can be used in the same places that the parent type can be used.
This is also called *polymorphism*, which means that multiple objects can be
substituted for each other at runtime if they have the same shape. To support
polymorphism, Rust has *trait objects* so that we can specify that we would
like values of any type, as long as the values implement a particular trait.

Inheritance has recently fallen out of favor as a programming design solution
in many programming languages. Using inheritance to re-use some code can
require more code to be shared than you actually need. Subclasses don't always
share all characteristics of their parent class, but inheritance means the
subclass gets all of its parent's data and behavior. This can make a program's
design less flexible, and create the possibility of calling methods on
subclasses that don't make sense or cause errors since the methods don't apply
to the subclass but must be inherited from the parent class. In addition, some
languages only allow a subclass to inherit from one class, further restricting
the flexibility of a program's design.

For these reasons, Rust chose to take a different approach with trait objects
instead of inheritance. Let's take a look at how trait objects enable
polymorphism in Rust.

## Trait Objects for Using Values of Different Types

In Chapter 8, we talked about a limitation of vectors is that vectors can only
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
We're going to create a library crate conatining the structure of a GUI library
called `rust_gui`. Our GUI library could include some types for people to use,
such as `Button` or `TextField`. Programmers that use `rust_gui` will want to
create more types that can be drawn on the screen: one programmer might add an
`Image`, while another might add a `SelectBox`. We're not going to implement a
fully-fledged GUI library in this chapter, but we will show how the pieces
would fit together.

When we're writing the `rust_gui` library, we don't know all the types that
other programmers will want to create, so we can't define an `enum` containing
all the types. What we do know is that `rust_gui` needs to be able to keep
track of a bunch of values of all these different types, and it needs to be
able to call a `draw` method on each of these values. Our GUI library doesn't
need to know what will happen exactly when we call the `draw` method, just that
the value will have that method available for us to call.

In a language with inheritance, we might define a class named `Component` that
has a method named `draw` on it. The other classes like `Button`, `Image`, and
`SelectBox` would inherit from `Component` and thus inherit the `draw` method.
They could each override the `draw` method to define their custom behavior, but
the framework could treat all of the types as if they were `Component`
instances and call `draw` on them.

### Defining a Trait for the Common Behavior

In Rust, though, we can define a trait that we'll name `Draw` and that will
have one method named `draw`. Then we can define a vector that takes a *trait
object*, which is a trait behind some sort of pointer, such as a `&` reference
or a `Box<T>` smart pointer.

We mentioned that we don't call structs and enums "objects" to distinguish
structs and enums from other languages' objects. The data in the struct or enum
fields and the behavior in `impl` blocks is separated, as opposed to other
languages that have data and behavior combined into one concept called an
object. Trait objects *are* more like objects in other languages, in the sense
that they combine the data made up of the pointer to a concrete object with the
behavior of the methods defined in the trait. Keep in mind that trait objects
are different from objects in other languages; we can't customize the kind of
data held in a trait object like we can by definining struct fields, for
example. Trait objects aren't as generally useful as objects in other
languages: their purpose is to allow abstraction across common behavior.

A trait defines behavior that we need in a given situation. We can then use a
trait as a trait object in places where we would use a concrete type or a
generic type. Rust's type system will ensure that any value we substitute in
for the trait object will implement the methods of the trait. Then we don't
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

This should look familiar since we talked about how to define traits in Chapter
10. Next comes something new: Listing 17-4 has the definition of a struct named
`Screen` that holds a vector named `components` that are of type `Box<Draw>`.
That `Box<Draw>` is a trait object: it's a stand-in for any type inside a `Box`
that implements the `Draw` trait.

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

On the `Screen` struct, we'll define a method named `run`, which will call the
`draw` method on each of its `components` as shown in Liting 17-5:

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
are all of type `Button` or all of type `TextField`. If you'll only ever have
homogeneous collections, using generics and trait bounds is preferable since
the definitions will be monomorphized at compile time to use the concrete types.

With the definition of `Screen` that holds a component list of trait objects in
`Vec<Box<Draw>>` instead, one `Screen` instance can hold a `Vec` that contains
a `Box<Button>` as well as a `Box<TextField>`. Let's see how that works, and
then talk about the runtime performance implications.

### Implementations of the Trait from Us or Library Users

Now to add some types that implement the `Draw` trait. We're going to provide
the `Button` type, and again, actually implementing a GUI library is out of
scope of this book, so the `draw` method won't have any useful implementation
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
(without any actual GUI code that's out of scope of this chapter). In addition
to implementing the `Draw` trait, `Button` might also have another `impl` block
containing methods having to do with what happens if the button is clicked.
These kinds of methods won't apply to types like `TextField`.

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

Even though we didn't know that someone would add the `SelectBox` type someday,
our `Screen` implementation was able to operate on the `SelectBox` and draw it
because `SelectBox` implements the `Draw` type, which means it implements the
`draw` method.

Only being concerned with the messages a value responds to, rather than the
value's concrete type, is sometimes called *duck typing*: if it walks like a
duck, and quacks like a duck, then it must be a duck! In the implementation of
`run` on `Screen` in Listing 17-5, `run` doesn't need to know what the
concrete type of each component is. It doesn't check to see if a component is
an instance of a `Button` or a `SelectBox`, it just calls the `draw` method on
the component. By specifying `Box<Draw>` as the type of the values in the
`components` vector, we've defined that `Screen` needs values that we can call
the `draw` method on.

The advantage with using trait objects and Rust's type system to do duck typing
is that we never have to check that a value implements a particular method at
runtime or worry about getting errors if a value doesn't implement a method but
we call it. Rust won't compile our code if the values don't implement the
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

<span class="caption">Listing 17-10: Attempting to use a type that doesn't
implement the trait object's trait</span>

We'll get this error because `String` doesn't implement the `Draw` trait:

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

This lets us know that either we're passing something we didn't mean to pass to
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

When we use trait objects, the compiler can't perform monomorphization because
we don't know all the types that might be used with the code. Instead, Rust
keeps track of the code that might be used when a method is called and figures
out at runtime which code needs to be used for a particular method call. This
is known as *dynamic dispatch*, and there's a runtime cost when this lookup
happens. We did get extra flexibility in the code that we wrote and were able
to support, though, so it's a tradeoff to consider.

### Object Safety is Required for Trait Objects

Not all traits can be made into trait objects; only *object safe* traits can. A
trait is object safe as long as both of the following are true:

* The trait does not require `Self` to be `Sized`
* All of the trait's methods are object safe.

`Self` is a keyword that is an alias for the type implementing a trait. `Sized`
is a marker trait like the `Send` and `Sync` traits that we talked about in
Chapter 16. `Sized` is automatically implemented on types that have a known
size at compile time, such as `i32` and references. Types that do not have a
known size include slices and traits.

`Sized` is an implicit trait bound on all generic type parameters by default.
Most useful operations in Rust require a type to be `Sized`, so making `Sized`
a default requirement on trait bounds means we don't have to write `T: Sized`
with most every use of generics. If we want to be able to use a trait on
slices, however, we need to opt out of the `Sized` trait bound, and we can do
that by specifying `T: ?Sized` as a trait bound.

Traits have a default bound of `Self: ?Sized`, which means that they can be
impelmented on types that may or may not be `Sized`. If we create a trait `Foo`
that opts out of the `Self: ?Sized` bound, that would look like the following:

```rust
trait Foo: Sized {
    fn some_method(&self);
}
```

The trait `Sized` is now a *super trait* of trait `Foo`, which means trait
`Foo` requires types that implement `Foo` (that is, `Self`) to be `Sized`.
We're going to talk about super traits in more detail in Chapter 19.

The reason a trait like `Foo` that requires `Self` to be `Sized` is not allowed
to be a trait object is that it would be impossible to implement the trait
`Foo` for the trait object `Foo`: trait objects aren't sized, but `Foo`
requires `Self` to be `Sized`. A type can't be both sized and unsized at the
same time!

For the second object safety requirement that says all of a trait's methods
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
exact type that it is, there's no way that the method can use the original
concrete type that it's forgotten. Same with generic type parameters that are
filled in with concrete type parameters when the trait is used: the concrete
types become part of the type that implements the trait. When the type is
erased by the use of a trait object, there's no way to know what types to fill
in the generic type parameters with.

An example of a trait whose methods are not object safe is the standard
library's `Clone` trait. The signature for the `clone` method in the `Clone`
trait looks like this:

```
pub trait Clone {
    fn clone(&self) -> Self;
}
```

`String` implements the `Clone` trait, and when we call the `clone` method on
an instance of `String` we get back an instance of `String`. Similarly, if we
call `clone` on an instance of `Vec`, we get back an instance of `Vec`. The
signature of `clone` needs to know what type will stand in for `Self`, since
that's the return type.

If we try to implement `Clone` on a trait like the `Draw` trait from Listing
17-3, we wouldn't know whether `Self` would end up being a `Button`, a
`SelectBox`, or some other type that will implement the `Draw` trait in the
future.

The compiler will tell you if you're trying to do something that violates the
rules of object safety in regards to trait objects. For example, if we had
tried to implement the `Screen` struct in Listing 17-4 to hold types that
implement the `Clone` trait instead of the `Draw` trait, like this:

```rust,ignore
pub struct Screen {
    pub components: Vec<Box<Clone>>,
}
```

We'll get this error:

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

TODO: what do you do instead? See http://play.integer32.com/?gist=6d92a25725b556be2ac77315f1b50bbd&version=stable

UGH this is making me want to cut this section, we have to be able to explain
this fully but it's so complicated to explain fully

## Object-Oriented Design Pattern Implementations

### Creational Patterns: Builder?

### Structural Patterns: Proxy?

### Behavioral Patterns: Visitor? Command?

## Summary

TODO: Trait objects provide dynamic dispatch, which enables OOP-like patterns,
whether that means you want to call it object oriented or not, make up your own
mind. Reiterate tradeoffs of trait objects
