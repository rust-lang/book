
[TOC]

# Is Rust an Object-Oriented Programming Language?

Object-Oriented Programming is a way of modeling programs that originated with
Simula in the 1960s and became popular with C++ in the 1990s. There are many
competing definitions for what OOP is: under some definitions, Rust is
object-oriented; under other definitions, Rust is not. In this chapter, we’ll
explore some characteristics that are commonly considered to be object-oriented
and how those characteristics translate to idiomatic Rust.

## What Does Object-Oriented Mean?

There isn’t consensus in the programming community about the features a
language needs to have in order to be called object-oriented. Rust is
influenced by many different programming paradigms; we explored the features it
has that come from functional programming in Chapter 13. Some of the
characteristics that object-oriented programming languages tend to share are
objects, encapsulation, and inheritance. Let’s take a look at what each of
those mean and whether Rust supports them.

### Objects Contain Data and Behavior

The book “Design Patterns: Elements of Reusable Object-Oriented Software,”
colloquially referred to as “The Gang of Four book,” is a catalog of
object-oriented design patterns. It defines object-oriented programming in this
way:

> Object-oriented programs are made up of objects. An *object* packages both
> data and the procedures that operate on that data. The procedures are
> typically called *methods* or *operations*.

Under this definition, then, Rust is object-oriented: structs and enums have
data and `impl` blocks provide methods on structs and enums. Even though
structs and enums with methods aren’t *called* objects, they provide the same
functionality that objects do, using the Gang of Four’s definition of objects.

### Encapsulation that Hides Implementation Details

Another aspect commonly associated with object-oriented programming is the idea
of *encapsulation*: the implementation details of an object aren’t accessible
to code using that object. The only way to interact with an object is through
the public API the object offers; code using the object should not be able to
reach into the object’s internals and change data or behavior directly.
Encapsulation enables changing and refactoring an object’s internals without
needing to change the code that uses the object.

As we discussed in Chapter 7, we can use the `pub` keyword to decide what
modules, types, functions, and methods in our code should be public, and by
default, everything is private. For example, we can define a struct
`AveragedCollection` that has a field containing a vector of `i32` values. The
struct can also have a field that knows the average of the values in the vector
so that whenever anyone wants to know the average of the values that the struct
has in its vector, we don’t have to compute it on-demand. `AveragedCollection`
will cache the calculated average for us. Listing 17-1 has the definition of
the `AveragedCollection` struct:

Filename: src/lib.rs

```
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

Listing 17-1: An `AveragedCollection` struct that maintains a list of integers
and the average of the items in the collection.

Note that the struct itself is marked `pub` so that other code may use this
struct, but the fields within the struct remain private. This is important in
this case because we want to ensure that whenever a value is added or removed
from the list, we also update the average. We do this by implementing `add`,
`remove`, and `average` methods on the struct as shown in Listing 17-2:

Filename: src/lib.rs

```
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

Listing 17-2: Implementations of the public methods
`add`, `remove`, and `average` on `AveragedCollection`

The public methods `add`, `remove`, and `average` are the only way to modify an
instance of a `AveragedCollection`. When an item is added to `list` using the
`add` method or removed using the `remove` method, the implementations of those
methods call the private `update_average` method that takes care of updating
the `average` field as well. Because the `list` and `average` fields are
private, there’s no way for external code to add or remove items to the `list`
field directly, which could cause the `average` field to get out of sync. The
`average` method returns the value in the `average` field, which allows
external code to read the `average` but not modify it.

Because we’ve encapsulated the implementation details of `AveragedCollection`,
we could also change aspects like using a different data structure used for the
`list` to use a `HashSet` instead of a `Vec`, for instance. As long as the
signatures of the `add`, `remove`, and `average` public methods stayed the same,
code using `AveragedCollection` wouldn’t need to change. This wouldn’t
necessarily be the case if we exposed `list` to external code: `HashSet` and
`Vec` have different methods for adding and removing items, so the external
code would likely have to change if it was modifying `list` directly.

If encapsulation is a required aspect for a language to be considered
object-oriented, then Rust meets that requirement. Using `pub` or not for
different parts of code enables encapsulation of implementation details.

### Inheritance as a Type System and as Code Sharing

*Inheritance* is a mechanism that some programming languages provide whereby an
object can be defined to inherit from another object’s definition, thus gaining
the parent object’s data and behavior without having to define those again.
Inheritance is a characteristic that is part of some people’s definitions of
what an OOP language is.

If a language must have inheritance to be an object-oriented language, then
Rust is not object-oriented. There is not a way to define a struct that
inherits from another struct in order to gain the parent struct’s fields and
method implementations. However, if you’re used to having inheritance in your
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
substituted for each other at runtime if they have the same shape.

<!-- PROD: START BOX -->

> While many people use “polymorphism” to describe inheritance, it’s actually
> a specific kind of polymorphism, called “sub-type polymorphism.” There are
> other forms as well; a generic parameter with a trait bound in Rust is
> also polymorphism, more specifically “parametric polymorphism.” The exact
> details between the different kinds of polymorphism aren’t crucial here,
> so don’t worry too much about the details: just know that Rust has multiple
> polymorphism-related features, unlike many OOP languages.

<!-- PROD: END BOX -->

To support this sort of pattern, Rust has *trait objects* so that we can
specify that we would like values of any type, as long as the values implement
a particular trait.

Inheritance has recently fallen out of favor as a programming design solution
in many programming languages. Using inheritance to re-use some code can
require more code to be shared than you actually need. Subclasses shouldn’t
always share all characteristics of their parent class, but inheritance means
the subclass gets all of its parent’s data and behavior. This can make a
program’s design less flexible, and creates the possibility of calling methods
on subclasses that don’t make sense or cause errors since the methods don’t
apply to the subclass but must be inherited from the parent class. In addition,
some languages only allow a subclass to inherit from one class, further
restricting the flexibility of a program’s design.

For these reasons, Rust chose to take a different approach with trait objects
instead of inheritance. Let’s take a look at how trait objects enable
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
or a `Box<T>` smart pointer.

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

Filename: src/lib.rs

```
pub trait Draw {
    fn draw(&self);
}
```

Listing 17-3: Definition of the `Draw` trait

This should look familiar since we talked about how to define traits in
Chapter 10. Next comes something new: Listing 17-4 has the definition of a
struct named `Screen` that holds a vector named `components` that are of type
`Box<Draw>`. That `Box<Draw>` is a trait object: it’s a stand-in for any type
inside a `Box` that implements the `Draw` trait.

Filename: src/lib.rs

```
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

Listing 17-4: Definition of the `Screen` struct with a `components` field that
holds a vector of trait objects that implement the `Draw` trait

On the `Screen` struct, we’ll define a method named `run`, which will call the
`draw` method on each of its `components` as shown in Listing 17-5:

Filename: src/lib.rs

```
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Listing 17-5: Implementing a `run` method on `Screen` that calls the `draw`
method on each component

This is different than defining a struct that uses a generic type parameter
with trait bounds. A generic type parameter can only be substituted with one
concrete type at a time, while trait objects allow for multiple concrete types
to fill in for the trait object at runtime. For example, we could have defined
the `Screen` struct using a generic type and a trait bound as in Listing 17-6:

Filename: src/lib.rs

```
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

Listing 17-6: An alternate implementation of the `Screen` struct and its `run`
method using generics and trait bounds

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

Filename: src/lib.rs

```
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

Listing 17-7: A `Button` struct that implements the `Draw` trait

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

Filename: src/main.rs

```
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

Listing 17-8: Another crate using `rust_gui` and implementing the `Draw` trait
on a `SelectBox` struct

The user of our library can now write their `main` function to create a
`Screen` instance and add a `SelectBox` and a `Button` to the screen by putting
each in a `Box<T>` to become a trait object. They can then call the `run`
method on the `Screen` instance, which will call `draw` on each of the
components. Listing 17-9 shows this implementation:

Filename: src/main.rs

```
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

Listing 17-9: Using trait objects to store values of different types that
implement the same trait

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

Filename: src/main.rs

```
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

Listing 17-10: Attempting to use a type that doesn’t implement the trait
object’s trait

We’ll get this error because `String` doesn’t implement the `Draw` trait:

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

```
trait Foo: Sized {
    fn some_method(&self);
}
```

The trait `Sized` is now a *super trait* of trait `Foo`, which means trait
`Foo` requires types that implement `Foo` (that is, `Self`) to be `Sized`.
We’re going to talk about super traits in more detail in Chapter 19.

The reason a trait like `Foo` that requires `Self` to be `Sized` is not allowed
to be a trait object is that it would be impossible to implement the trait
`Foo` for the trait object `Foo`: trait objects aren’t sized, but `Foo`
requires `Self` to be `Sized`. A type can’t be both sized and unsized at the
same time!

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

```
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

```
pub struct Screen {
    pub components: Vec<Box<Clone>>,
}
```

We’ll get this error:

```
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

## Object-Oriented Design Pattern Implementation

Let’s look at an example of the state design pattern and how to use it in Rust.
The *state pattern* is when a value has some internal state, and the value’s
behavior changes based on the internal state. The internal state is represented
by a set of objects that inherit shared functionality (we’ll use structs and
traits since Rust doesn’t have objects and inheritance). Each state object is
responsible for its own behavior and the rules for when it should change into
another state. The value that holds one of these state objects doesn’t know
anything about the different behavior of the states or when to transition
between states. In the future when requirements change, we won’t need to change
the code of the value holding the state or the code that uses the value. We’ll
only need to update the code inside one of the state objects to change its
rules, or perhaps add more state objects.

In order to explore this idea, we’re going to implement a blog post workflow in
an incremental way. The workflow that we want our blog posts to follow, once
we’re done with the implementation, is:

1. A blog post starts as an empty draft.
2. Once the draft is done, we request a review of the post.
3. Once the post is approved, it gets published.
4. Only published blog posts return content to print so that we can’t
   accidentally print the text of a post that hasn’t been approved.

Any other changes attempted on a post should have no effect. For example, if we
try to approve a draft blog post before we’ve requested a review, the post
should stay an unpublished draft.

Listing 17-11 shows this workflow in code form. This is an example usage of the
API we’re going to implement in a library crate named `blog`:

Filename: src/main.rs

```
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

Listing 17-11: Code that demonstrates the desired behavior we want our `blog`
crate to have

We want to be able to create a new draft blog post with `Post::new`. Then, we
want to add some text to the blog post while we’re in the draft state. If we
try to print out the post’s content immediately, though, we shouldn’t get any
text, since the post is still a draft. We’ve added an `assert_eq!` here for
demonstration purposes. Asserting that a draft blog post returns an empty
string from the `content` method would make an excellent unit test in our
library, but we’re not going to write tests for this example.

Next, we want to be able to request a review of our post, and `content` should
still return an empty string while waiting for a review. Lastly, when we
approve the blog post, it should get published, which means the text we added
will be returned when we call `content`.

Notice that the only type we’re interacting with from the crate is the `Post`
type. The various states a post can be in (draft, waiting for review,
published) are managed internally to the `Post` type. The states change due to
the methods we call on the `Post` instance, but we don’t have to manage the
state changes directly. This also means we won’t make a mistake with the
states, like forgetting to request a review before publishing.

### Defining `Post` and Creating a New Instance in the Draft State

Let’s get started on the implementation of the library! We know we want to have
a public `Post` struct that holds some content, so let’s start with the
definition of the struct and an associated public `new` function to create an
instance of `Post` as shown in Listing 17-12. We’re also going to have a
private trait `State`. `Post` will hold a trait object of `Box<State>` inside
an `Option` in a private field named `state`. We’ll see why the `Option` is
necessary in a bit. The `State` trait defines all the behavior different post
states share, and the `Draft`, `PendingReview`, and `Published` states will all
implement the `State` trait. For now, the trait does not have any methods, and
we’re going to start by defining just the `Draft` state since that’s the state
we want to start in:

Filename: src/lib.rs

```
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

Listing 17-12: Definition of a `Post` struct and a `new` function that creates
a new `Post` instance, a `State` trait, and a `Draft` struct that implements
`State`

When we create a new `Post`, we set its `state` field to a `Some` value holding
a `Box` pointing to a new instance of the `Draft` struct. This ensures whenever
we create a new instance of `Post`, it’ll start out as a draft. Because the
`state` field of `Post` is private, there’s no way to create a `Post` in any
other state!

### Storing the Text of the Post Content

In the `Post::new` function, we set the `content` field to a new, empty
`String`. In Listing 17-11, we showed that we want to be able to call a method
named `add_text` and pass a `&str` to it to add that text to the content of the
blog post. We’re choosing to implement this as a method rather than exposing
the `content` field as `pub` because we want to be able to control how the
`content` field’s data is read by implementing a method later. The `add_text`
method is pretty straightforward though, let’s add the implementation in
Listing 17-13 to the `impl Post` block:

Filename: src/lib.rs

```
impl Post {
    // ...snip...
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Listing 17-13: Implementing the `add_text` method to add
text to a post’s `content`

`add_text` takes a mutable reference to `self`, since we’re changing the `Post`
instance that we’re calling `add_text` on. We then call `push_str` on the
`String` in `content` and pass the `text` argument to add to the saved
`content`. This isn’t part of the state pattern since its behavior doesn’t
depend on the state that the post is in. The `add_text` method doesn’t interact
with the `state` field at all, but it is part of the behavior we want to
support.

### Content of a Draft Post is Empty

After we’ve called `add_text` and added some content to our post, we still want
the `content` method to return an empty string slice since the post is still in
the draft state, as shown on line 8 of Listing 17-11. For now, let’s implement
the `content` method with the simplest thing that will fulfill this requirement:
always returning an empty string slice. We’re going to change this later once
we implement the ability to change a post’s state to be published. With what we
have so far, though, posts can only be in the draft state, which means the post
content should always be empty. Listing 17-14 shows this placeholder
implementation:

Filename: src/lib.rs

```
impl Post {
    // ...snip...
    pub fn content(&self) -> &str {
        ""
    }
}
```

Listing 17-14: Adding a placeholder implementation for
the `content` method on `Post` that always returns an empty string slice

With this added `content` method, everything in Listing 17-11 up to line 8
works as we intend.

### Requesting a Review of the Post Changes its State

Next up is requesting a review of a post, which should change its state from
`Draft` to `PendingReview`. We want `post` to have a public method named
`request_review` that will take a mutable reference to `self`. Then we’re going
to call a `request_review` method on the state that we’re holding, and that
`request_review` method will consume the current state and return a new state.
In order to be able to consume the old state, the state `request_review` method
needs to take ownership of the state value. This is where the `Option` comes
in: we’re going to take the `Some` value out of the `state` field and leave a
`None` in its place since Rust doesn’t let us have unpopulated fields in
structs. Then we’ll set the post’s `state` value to the result of this
operation. Listing 17-15 shows this code:

Filename: src/lib.rs

```
impl Post {
    // ...snip...
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```

Listing 17-15: Implementing `request_review` methods on `Post` and the `State`
trait

We’ve added the `request_review` method to the `State` trait; all types that
implement the trait will now need to implement the `request_review` method.
Note that rather than having `self`, `&self`, or `&mut self` as the first
parameter of the method, we have `self: Box<Self>`. This syntax means the
method is only valid when called on a `Box` holding the type. This syntax takes
ownership of `Box<Self>`, which is what we want because we’re transforming the
old state into a new state, and we want the old state to no longer be valid.

The implementation for the `request_review` method on `Draft` is to return a
new, boxed instance of the `PendingReview` struct, which is a new type we’ve
introduced that represents the state when a post is waiting for a review. The
`PendingReview` struct also implements the `request_review` method, but it
doesn’t do any transformations. It returns itself since requesting a review on
a post that’s already in the `PendingReview` state should stay in the
`PendingReview` state.

Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter what its `state` value
is. Each state is responsible for its own rules.

We’re going to leave the `content` method on `Post` as it is, returning an
empty string slice. We can now have a `Post` in the `PendingReview` state, not
just the `Draft` state, but we want the same behavior in the `PendingReview`
state. Listing 17-11 now works up until line 11!

### Approving a Post Changes the Behavior of `content`

The `approve` method on `Post` will be similar to that of the `request_review`
method: it will set the `state` to the value that the current state says it
should have when that state is approved. We’ll need to add the `approve` method
to the `State` trait, and we’ll add a new struct that implements `State`, the
`Published` state. Listing 17-16 shows the new code:

Filename: src/lib.rs

```
impl Post {
    // ...snip...
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    // ...snip...
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // ...snip...
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}
```

Listing 17-16: Implementing the `approve` method on `Post` and the `State` trait

Similarly to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect since it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself since the
post should stay in the `Published` state in those cases.

Now for updating the `content` method on `Post`: we want to return the value in
the post’s `content` field if its state is `Published`, otherwise we want to
return an empty string slice. Because the goal is to keep all the rules like
this in the structs that implement `State`, we’re going to call a `content`
method on the value in `state` and pass the post instance (that is, `self`) as
an argument. Then we’ll return the value returned from the `content` method on
the `state` value as shown in Listing 17-17:

Filename: src/lib.rs

```
impl Post {
    // ...snip...
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    // ...snip...
}
```

Listing 17-17: Updating the `content` method on `Post` to delegate to a
`content` method on `State`

We’re calling the `as_ref` method on the `Option` because we want a reference
to the value inside the `Option`. We’re then calling the `unwrap` method, which
we know will never panic because all the methods on `Post` ensure that the
`state` value will have a `Some` value in it when those methods are done. This
is one of the cases we talked about in Chapter 12 where we know that a `None`
value is never possible even though the compiler isn’t able to understand that.

The `content` method on the `State` trait is where the logic for what content
to return will be. We’re going to add a default implementation for the
`content` method that returns an empty string slice. That lets us not need to
implement `content` on the `Draft` and `PendingReview` structs. The `Published`
struct will override the `content` method and will return the value in
`post.content`, as shown in Listing 17-18:

Filename: src/lib.rs

```
trait State {
    // ...snip...
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// ...snip...
struct Published {}

impl State for Published {
    // ...snip...
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

Listing 17-18: Adding the `content` method to the `State` trait

Note that we need lifetime annotations on this method, like we discussed in
Chapter 10. We’re taking a reference to a `post` as an argument, and we’re
returning a reference to a part of that `post`, so the lifetime of the returned
reference is related to the lifetime of the `post` argument.

### Tradeoffs of the State Pattern

We’ve shown that Rust is capable of implementing the object-oriented state
pattern in order to encapsulate the different kinds of behavior that a post
should have that depends on the state that the post is in. The methods on
`Post` don’t know anything about the different kinds of behavior. The way this
code is organized, we have one place to look in order to find out all the
different ways that a published post behaves: the implementation of the `State`
trait on the `Published` struct.

An alternative implementation that didn’t use the state pattern might have
`match` statements in the methods on `Post` or even in the code that uses
`Post` (`main` in our case) that checks what the state of the post is and
changes behavior in those places instead. That would mean we’d have a lot of
places to look in order to understand all the implications of a post being in
the published state! This would get worse the more states we added: each of
those `match` statements would need another arm. With the state pattern, the
`Post` methods and the places we use `Post` don’t need `match` statements and
adding a new state only involves adding a new `struct` and implementing the
trait methods on that one struct.

This implementation is easy to extend to add more functionality. Here are some
changes you can try making to the code in this section to see for yourself what
it’s like to maintain code using this pattern over time:

- Only allow adding text content when a post is in the `Draft` state
- Add a `reject` method that changes the post’s state from `PendingReview` back
  to `Draft`
- Require two calls to `approve` before changing the state to `Published`

A downside of the state pattern is that since the states implement the
transitions between the states, some of the states are coupled to each other.
If we add another state between `PendingReview` and `Published`, such as
`Scheduled`, we would have to change the code in `PendingReview` to transition
to `Scheduled` instead. It would be nicer if `PendingReview` wouldn’t need to
change because of the addition of a new state, but that would mean switching to
another design pattern.

There are a few bits of duplicated logic that are a downside of this
implementation in Rust. It would be nice if we could make default
implementations for the `request_review` and `approve` methods on the `State`
trait that return `self`, but this would violate object safety since the trait
doesn’t know what the concrete `self` will be exactly. We want to be able to
use `State` as a trait object, so we need its methods to be object safe.

The other duplication that would be nice to get rid of is the similar
implementations of the `request_review` and `approve` methods on `Post`. They
both delegate to the implementation of the same method on the value in the
`Option` in the `state` field, and set the new value of the `state` field to
the result. If we had a lot of methods on `Post` that followed this pattern, we
might consider defining a macro to eliminate the repetition (see Appendix E on
macros).

A downside of implementing this object-oriented pattern exactly as it’s defined
for object-oriented languages is that we’re not taking advantage of Rust’s
strengths as much as we could be. Let’s take a look at some changes we can make
to this code that can make invalid states and transitions into compile time
errors.

#### Encoding States and Behavior as Types

We’re going to show how to rethink the state pattern a bit in order to get a
different set of tradeoffs. Rather than encapsulating the states and
transitions completely so that outside code has no knowledge of them, we’re
going to encode the states into different types. When the states are types,
Rust’s type checking will make any attempt to use a draft post where we should
only use published posts into a compiler error.

Let’s consider the first part of `main` from Listing 17-11:

Filename: src/main.rs

```
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

We still want to create a new post in the draft state using `Post::new`, and we
still want to be able to add text to the post’s content. But instead of having
a `content` method on a draft post that returns an empty string, we’re going to
make it so that draft posts don’t have the `content` method at all. That way,
if we try to get a draft post’s content, we’ll get a compiler error that the
method doesn’t exist. This will make it impossible for us to accidentally
display draft post content in production, since that code won’t even compile.
Listing 17-19 shows the definition of a `Post` struct, a `DraftPost` struct,
and methods on each:

Filename: src/lib.rs

```
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
       &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Listing 17-19: A `Post` with a `content` method and a `DraftPost` without a
`content` method

Both the `Post` and `DraftPost` structs have a private `content` field that
stores the blog post text. The structs no longer have the `state` field since
we’re moving the encoding of the state to the types of the structs. `Post` will
represent a published post, and it has a `content` method that returns the
`content`.

We still have a `Post::new` function, but instead of returning an instance of
`Post`, it returns an instance of `DraftPost`. It’s not possible to create an
instance of `Post` right now since `content` is private and there aren’t any
functions that return `Post`. `DraftPost` has an `add_text` method defined on
it so that we can add text to `content` as before, but note that `DraftPost`
does not have a `content` method defined! So we’ve enforced that all posts
start as draft posts, and draft posts don’t have their content available for
display. Any attempt to get around these constraints will be a compiler error.

#### Implementing Transitions as Transformations into Different Types

So how do we get a published post then? The rule we want to enforce is that a
draft post has to be reviewed and approved before it can be published. A post
in the pending review state should still not display any content. Let’s
implement these constraints by adding another struct, `PendingReviewPost`,
defining the `request_review` method on `DraftPost` to return a
`PendingReviewPost`, and defining an `approve` method on `PendingReviewPost` to
return a `Post` as shown in Listing 17-20:

Filename: src/lib.rs

```
impl DraftPost {
    // ...snip...

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

Listing 17-20: A `PendingReviewPost` that gets created by
calling `request_review` on `DraftPost`, and an `approve` method that turns a
`PendingReviewPost` into a published `Post`

The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won’t have any `DraftPost` instances lingering around after we’ve called
`request_review` on them, and so forth. `PendingReviewPost` doesn’t have a
`content` method defined on it, so attempting to read its content is a compiler
error like it is with `DraftPost`. Because the only way to get a published
`Post` instance that does have a `content` method defined is to call the
`approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we’ve now encoded the blog post workflow into the type system.

This does mean we have to make some small changes to `main`. Because
`request_review` and `approve` return new instances rather than modifying the
struct they’re called on, we need to add more `let post = ` shadowing
assignments to save the returned instances. We also can’t have the assertions
about the draft and pending review post’s contents being empty string anymore,
nor do we need them: we can’t compile code that tries to use the content of
posts in those states any longer. The updated code in `main` is shown in
Listing 17-21:

Filename: src/main.rs

```
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

Listing 17-21: Modifications to `main` to use the new implementation of the
blog post workflow

Having to change `main` to reassign `post` is what makes this implementation
not quite following the object-oriented state pattern anymore: the
transformations between the states are no longer encapsulated entirely within
the `Post` implementation. However, we’ve gained the property of having invalid
states be impossible because of the type system and type checking that happens
at compile time! This ensures that certain bugs, such as displaying the content
of an unpublished post, will be discovered before they make it to production.

Try the tasks suggested that add additional requirements that we mentioned at
the start of this section to see how working with this version of the code
feels.

Even though Rust is capable of implementing object-oriented design patterns,
there are other patterns like encoding state into the type system that are
available in Rust. These patterns have different tradeoffs than the
object-oriented patterns do. While you may be very familiar with
object-oriented patterns, rethinking the problem in order to take advantage of
Rust’s features can give benefits like preventing some bugs at compile-time.
Object-oriented patterns won’t always be the best solution in Rust, since Rust
has features like ownership that object-oriented languages don’t have.

## Summary

No matter whether you think Rust is an object-oriented language or not after
reading this chapter, you’ve now seen that trait objects are a way to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. This flexibility can
be used to implement object-oriented patterns that can help with the
maintainability of your code. Rust also has different features, like ownership,
than object-oriented languages. An object-oriented pattern won’t always be the
best way to take advantage of Rust’s strengths.

Next, let’s look at another feature of Rust that enables lots of flexibility:
patterns. We’ve looked at them briefly throughout the book, but haven’t seen
everything they’re capable of yet. Let’s go!
