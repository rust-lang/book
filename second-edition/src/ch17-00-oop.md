# Is Rust an Object-Oriented Programming Language?

Object-Oriented Programming is a way of modeling programs that originated with
Simula in the 1960s and became popular with C++ in the 1990s. There are many
competing definitions for what OOP is: under some definitions, Rust is
object-oriented; under other definitions, Rust is not. In this chapter, we'll
explore some characteristics that are commonly considered to be object-oriented
and how those characteristics translate to idiomatic Rust.

## Objects Contain Data and Behavior

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

## Inheritance as a Type System and as Code Sharing

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

TODO: should we expound more on inheritance's downsides, like inheriting too
much, not being able to have multiple inheritance in some languages, the
diamond problem?

There are two main reasons to reach for inheritance. The first is as a type
system: to express that a child type can be used in the same places that the
parent type can be used. For this, Rust has *trait objects* so that we can
specify that we would like values of any type, as long as the values implement
a particular trait. The second reason to use inheritance is to be able to
re-use code: once a particular behavior is implemented for one type, enable
re-using that implementation for a different type. Rust code can be shared
using default trait method implementations or composition instead. Let's
explore these situations and idiomatic Rust code solutions.

### Trait Objects for Using Types Interchangeably

In Chapter 8, we talked about a limitation of vectors is that vectors can only
store elements of one type. We had an example in Listing 8-1 where we defined a
`SpreadsheetCell` enum that had variants to hold integers, floats, and text so
that we could store different types of data in each cell and still have a
vector represent a row of cells. This works for cases in which the kinds of
things we want to be able to treat interchangably are a fixed set of types that
we know when our code gets compiled.

<!-- The code example I want to reference did not have a listing number; it's
the one with SpreadsheetCell. I will go back and add Listing 8-1 next time I
get Chapter 8 for editing. /Carol -->

Sometimes we want the set of types that we use to be extendable by the
programmers who use our library. For example, many Graphical User Interface
tools have a concept of a list of items that get drawn on the screen by
iterating through the list and calling a `draw` method on each of the items. A
GUI library might include some types for people to use, such as `Button` or
`TextField`. Programmers that use the GUI library will want to create more
types that can be drawn on the screen: one programmer might add an `Image`,
while another might add a `SelectBox`. We're not going to implement a
fully-fledged GUI library in this chapter, but we will show how the pieces
would fit together.

When we're writing the GUI library, we don't know all the types that other
programmers will want to create, so we can't define an `enum` containing all
the types. What we do know is that our GUI library needs to be able to keep
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

#### Defining a Trait for the Common Behavior

In Rust, though, we can define a trait that we'll name `Draw` and that will
have one method named `draw`. Then we can define a vector that takes a *trait
object*, which is a trait behind some sort of pointer. Rust's type system will
ensure that any value we substitute in for the trait object will implement the
methods of the trait. Then we don't need to know all the possible types at
compile time, and we can treat all the instances the same way. Here's how we'd
define a trait named `Draw` with one method named `draw`:

```rust
trait Draw {
    fn draw(&self);
}
```

This should look familiar since we talked about how to define traits in Chapter
10. Next comes something new: a definition of a struct named `Screen` that
holds a vector named `components` that are of type `Box<Draw>`. That
`Box<Draw>` is a trait object: it's a stand-in for any type inside a `Box` that
implements the `Draw` trait:

```rust
struct Screen {
    components: Vec<Box<Draw>>,
}
```

On the `Screen` struct, we'll define a method named `run`, which will call the
`draw` method on each of its `components`:

```rust
impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

#### Implementations of the Trait from Us or Library Users

Now to add some types that implement the `Draw` trait. We're going to provide
the `Button` type, and again, actually implementing a GUI library is out of
scope of this book, so the `draw` method won't have any useful implementation
in its body. To imagine what the implementation might look like, a `Button`
struct might have fields for `width`, `height`, and `label`, as well as methods
having to do with what happens if the button is clicked. A `Button`
implementation will differ from other components, such as a `TextField` type
that might have `width`, `height`, `label`, and `placeholder` fields. Each of
the types that we want to be able to draw on the screen will implement the
`Draw` trait with different code in the `draw` method that defines how to draw
that type.

```
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}
```

Someone using our library has decided to implement a `SelectBox` struct that
has `width`, `height`, and `options` fields. They implement the `Draw` trait on
the `SelectBox` type as well:

```rust
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

The user of our library can now write their `main` function to create a
`Screen` instance and add a `SelectBox` and a `Button` to the screen by putting
each in a `Box<T>` to become a trait object. They can then call the `run`
method on the `Screen` instance, which will call `draw` on each of the
components.

```rust
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

Even though we didn't know that someone would add the `SelectBox` type someday,
our `Screen` implementation was able to operate on the `SelectBox` and draw it
because `SelectBox` implements the `Draw` type, which means it implements the
`draw` method.

Only being concerned with the messages a value responds to, rather than the
value's concrete type, is sometimes called *duck typing*: if it walks like a
duck, and quacks like a duck, then it must be a duck! The advantage with using
trait objects and Rust's type system is that we never have to check that a
value implements a particular method at runtime or worry about getting errors
if a value doesn't implement a method but we call it. Rust won't compile our
code if the values don't implement the traits that the trait objects need.

For example, here's what happens if we try to create a `Screen` with a `String`
as a component like so:

```rust,ignore
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}
```

We'll get this error because `String` doesn't implement the `Draw` trait:

```
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

#### Trait Objects Perform Dynamic Dispatch

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

#### Object Safety is Required for Trait Objects

Not all traits can be made into objects; only "object safe" traits can. A trait
is object safe as long as:

* It does not require `Self` to be `Sized`, and
* All of its methods are object safe.

What does it mean for a method to be object safe? It is safe if either:

* It requires `Self` to be `Sized` or
* It meets all three of the following:
    * It must not have any type parameters; and,
    * It must have a receiver that has type `Self` or which dereferences to the
Self type;
    * must not use `Self` anywhere but the receiver

Those rules are a bit formal, but think of it this way: if your method requires
`Self`, but an object forgets the exact type that it is, there's no way that it
can work. Same with type parameters; if the type is erased, there's no way to
know what types to fill in the parameters with.

### Default Implementations of Trait Methods for Code Sharing

TODO: demonstrate a trait with some methods with default implementations,
possibly just call back to the example around Listing 10-14

## Composition and Delegation

TODO: is this worth demonstrating? to me, this is kind of a deficiency of Rust,
see below

Show how this is another alternative to inheritance that is now encouraged in
many languages

```
struct Person {
    name: String,
    address: String,
}

struct Employee {
    person: Person,
    position: String,
    salary: u32,
}

impl Employee {
    fn name(&self) -> &str {
        &self.person.name
    }
}
```

The deficiency is that you can't just say "`Employee` responds to all messages
that `Person` does and forwards them along to `self.person`", or "Employee
delegates the messages `name`, `address`, etc to `self.person` and the messages
`logo` to `self.employer`", etc

OR would this code be better demonstrated as:

```
trait Named {
    fn name(&self) -> &str;
}

struct Person {
    name: String,
    address: String,
}

impl Named for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

struct Employee {
    person: Person,
    position: String,
    salary: u32,
}

impl Named for Employee {
    fn name(&self) -> &str {
        &self.person.name
    }
}
```

so that you could use `Employee` everywhere you can use `Person`?

## Trait inheritance

TODO: Should we mention this?

## Specialization

TODO: Should we mention this?

## Encapsulation

TODO: is it worthwhile to point this out? Seems pretty trivial/obvious

If encapsulation defines OOP, then Rust is OOP because it has:

* `pub` and non-`pub`:
  * fields on structs
  * functions, methods
  * modules

## Summary

TODO: Trait objects provide dynamic dispatch, which enables OOP-like patterns,
whether that means you want to call it object oriented or not, make up your own
mind. Reiterate tradeoffs of trait objects
