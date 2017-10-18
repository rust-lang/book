
[TOC]

# Is Rust an Object-Oriented Programming Language?

Object-Oriented Programming is a way of modeling programs that originated with
Simula in the 1960s and became popular with C++ in the 1990s. There are many
competing definitions for what counts as OOP, and under some definitions, Rust
is object-oriented; under other definitions, it is not. In this chapter, we’ll
explore some characteristics that are commonly considered to be object-oriented
and how those characteristics translate to idiomatic Rust. We’ll then show you
how to implement an object-oriented design pattern in Rust and discuss the
tradeoffs of doing so versus implementing a solution using some of Rust’s
strengths instead.

## What Does Object-Oriented Mean?

There’s no consensus in the programming community about what features a
language needs in order to be called object-oriented. Rust is influenced by
many different programming paradigms including OOP; we explored, for example,
the features that came from functional programming in Chapter 13. Arguably,
object-oriented programming languages do tend to share certain common
characteristics, namely objects, encapsulation, and inheritance. Let’s take a
look at what each of those mean and whether Rust supports them.

### Objects Contain Data and Behavior

<!-- Is there a reason we're using this book as the reference, is it generally
accepted as an authority? -->
<!-- Yes, it is. For example, Martin Fowler (himself regarded as an authority)
had this to say about it https://www.martinfowler.com/bliki/GangOfFour.html:
> In my view the Gang of Four is the best book ever written on object-oriented
> design - possibly of any style of design.
/Carol -->

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
functionality, under the Gang of Four’s definition of objects.

### Encapsulation that Hides Implementation Details

Another aspect commonly associated with object-oriented programming is the idea
of *encapsulation*: that the implementation details of an object aren’t
accessible to code using that object. The only way to interact with an object
therefore is through its public API; code using the object should not be able
to reach into the object’s internals and change data or behavior directly. This
enables the programmer to change and refactor an object’s internals without
needing to change the code that uses the object.

We discussed an example of this in Chapter 7: We can use the `pub` keyword to
decide what modules, types, functions, and methods in our code should be
public, and by default everything else is private. For example, we can define a
struct `AveragedCollection` that has a field containing a vector of `i32`
values. The struct can also have a field that contains the average of the
values in the vector, meaning the average doesn’t have to be computed on-demand
whenever anyone needs it. In other words, `AveragedCollection` will cache the
calculated average for us. Listing 17-1 has the definition of the
`AveragedCollection` struct:

Filename: src/lib.rs

```
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

Listing 17-1: An `AveragedCollection` struct that maintains a list of integers
and the average of the items in the collection.

The struct itself is marked `pub` so that other code may use it, but the fields
within the struct remain private. This is important in this case because we
want to ensure that whenever a value is added or removed from the list, the
average is also updated. We do this by implementing `add`, `remove`, and
`average` methods on the struct as shown in Listing 17-2:

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

Listing 17-2: Implementations of the public methods `add`, `remove`, and
`average` on `AveragedCollection`

The public methods `add`, `remove`, and `average` are the only way to modify an
instance of `AveragedCollection`. When an item is added to `list` using the
`add` method or removed using the `remove` method, the implementations of each
call the private `update_average` method that takes care of updating the
`average` field as well.

We leave the `list` and `average` fields private so that there’s no way for
external code to add or remove items to the `list` field directly, otherwise
the `average` field might become out of sync. The `average` method returns the
value in the `average` field, allowing external code to read the `average` but
not modify it.

Because we’ve encapsulated the implementation details of `AveragedCollection`,
we can easily change aspects like the data structure in the future. For
instance, we could use a `HashSet` instead of a `Vec` for the `list` field. As
long as the signatures of the `add`, `remove`, and `average` public methods
stay the same, code using `AveragedCollection` wouldn’t need to change. If we
made `list` public instead, this wouldn’t necessarily be the case: `HashSet`
and `Vec` have different methods for adding and removing items, so the external
code would likely have to change if it was modifying `list` directly.

If encapsulation is a required aspect for a language to be considered
object-oriented, then Rust meets that requirement. The option to use `pub` or
not for different parts of code enables encapsulation of implementation details.

### Inheritance as a Type System and as Code Sharing

*Inheritance* is a mechanism whereby an object can inherit from another
object’s definition, thus gaining the parent object’s data and behavior without
you having to define them again.

If a language must have inheritance to be an object-oriented language, then
Rust is not. There is no way to define a struct that inherits the parent
struct’s fields and method implementations. However, if you’re used to having
inheritance in your programming toolbox, there are other solutions in Rust
depending on your reasons for using inheritance.

There are two main reasons to reach for inheritance. The first is for re-use of
code: you can implement particular behavior for one type, and inheritance
enables you to re-use that implementation for a different type. Rust code can
be shared using default trait method implementations instead, which we saw in
Listing 10-15 when we added a default implementation of the `summary` method on
the `Summarizable` trait. Any type implementing the `Summarizable` trait would
have the `summary` method available on it without any further code. This is
similar to a parent class having an implementation of a method, and an
inheriting child class then also having the implementation of the method. We
can also choose to override the default implementation of the `summary` method
when we implement the `Summarizable` trait, similar to a child class overriding
the implementation of a method inherited from a parent class.

The second reason to use inheritance relates to the type system: to enable a
child type to be used in the same places as the parent type. This is also
called *polymorphism*, which means that multiple objects can be substituted for
each other at runtime if they share certain characteristics.

<!-- What does it mean for objects to have the same shape? -->
<!-- The use of "shape" in this context has to do with the roots of "morph" in
"polymorphism", but it's not very well defined so I've reworded. /Carol -->

<!-- PROD: START BOX -->

> Polymorphism
>
> To many people, polymorphism is synonymous with inheritance. But it’s
> actually a more general concept that refers to code that can work with data
> of multiple types. For inheritance, those types are generally subclasses.
> Rust instead uses generics to abstract over different possible types, and
> trait bounds to impose constraints on what those types must provide. This is
> sometimes called bounded parametric polymorphism.

<!-- PROD: END BOX -->

Inheritance has recently fallen out of favor as a programming design solution
in many programming languages because it’s often at risk of sharing more code
than needs be. Subclasses shouldn’t always share all characteristics of their
parent class, but will do so with inheritance. This can make a program’s design
less flexible, and introduces the possibility of calling methods on subclasses
that don’t make sense or that cause errors because the methods don’t actually
apply to the subclass. Some languages will also only allow a subclass to
inherit from one class, further restricting the flexibility of a program’s
design.

For these reasons, Rust chose to take a different approach, using trait objects
instead of inheritance. Let’s take a look at how trait objects enable
polymorphism in Rust.

## Using Trait Objects that Allow for Values of Different Types

In Chapter 8, we mentioned that one limitation of vectors is that they can only
store elements of one type. We created a workaround in Listing 8-10 where we
defined a `SpreadsheetCell` enum that had variants to hold integers, floats,
and text. This meant we could store different types of data in each cell and
still have a vector that represented a row of cells. This is a perfectly good
solution when our interchangeable items are a fixed set of types that we know
when our code gets compiled.

Sometimes, however, we want the user of our library to be able to extend the
set of types that are valid in a particular situation. To show how we might
achieve this, we’ll create an example Graphical User Interface tool that
iterates through a list of items, calling a `draw` method on each one to drawn
it to the screen; a common technique for GUI tools. We’re going to create a
library crate containing the structure of a GUI library called `rust_gui`. This
crate might include some types for people to use, such as `Button` or
`TextField`. On top of these, users of `rust_gui` will want to create their own
types that can be drawn on the screen: for instance, one programmer might add
an `Image`, another might add a `SelectBox`.

We won’t implement a fully-fledged GUI library for this example, but will show
how the pieces would fit together. At the time of writing the library, we can’t
know and define all the types other programmers will want to create. What we do
know is that `rust_gui` needs to keep track of a bunch of values that are of
different types, and it needs to be able to call a `draw` method on each of
these differently-typed values. It doesn’t need to know exactly what will
happen when we call the `draw` method, just that the value will have that
method available for us to call.

To do this in a language with inheritance, we might define a class named
`Component` that has a method named `draw` on it. The other classes like
`Button`, `Image`, and `SelectBox` would inherit from `Component` and thus
inherit the `draw` method. They could each override the `draw` method to define
their custom behavior, but the framework could treat all of the types as if
they were `Component` instances and call `draw` on them. But Rust doesn’t have
inheritance, so we need another way.

### Defining a Trait for Common Behavior

To implement the behavior we want `rust_gui` to have, we’ll define a trait
named `Draw` that will have one method named `draw`. Then we can define a
vector that takes a *trait object*. A trait object points to an instance of a
type that implements the trait we specify. We create a trait object by
specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart
pointer, and then specifying the relevant trait (we’ll talk about the reason
trait objects have to use a pointer in Chapter 19 in the section on Dynamically
Sized Types). We can use trait objects in place of a generic or concrete type.
Wherever we use a trait object, Rust’s type system will ensure at compile-time
that any value used in that context will implement the trait object’s trait.
This way we don’t need to know all the possible types at compile time.

<!-- What will the trait object do in this case? I've taken this last part of
the line from below, but I'm not 100% on that -->
<!-- I've moved up more and reworded a bit, hope that clarifies /Carol -->

We’ve mentioned that in Rust we refrain from calling structs and enums
“objects” to distinguish them from other languages’ objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks is
separated, whereas in other languages the data and behavior combined into one
concept is often labeled an object. Trait objects, though, *are* more like
objects in other languages, in the sense that they combine both data and
behavior. However, trait objects differ from traditional objects in that we
can’t add data to a trait object. Trait objects aren’t as generally useful as
objects in other languages: their specific purpose is to allow abstraction
across common behavior.

Listing 17-3 shows how to define a trait named `Draw` with one method named
`draw`:

Filename: src/lib.rs

```
pub trait Draw {
    fn draw(&self);
}
```

Listing 17-3: Definition of the `Draw` trait

This should look familiar from our discussions on how to define traits in
Chapter 10. Next comes something new: Listing 17-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<Draw>`, which is a trait object: it’s a stand-in for any type inside a
`Box` that implements the `Draw` trait.

<!-- Would it be useful to let the reader know why we need a box here, or will
that be clear at this point? -->
<!-- We get into this in chapter 19; I've added a reference to the start of
this section where we talk about needing a `&` or a `Box` to be a trait object.
/Carol -->

Filename: src/lib.rs

```
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

Listing 17-4: Definition of the `Screen` struct with a `components` field
holding a vector of trait objects that implement the `Draw` trait

On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 17-5:

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

This works differently to defining a struct that uses a generic type parameter
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

This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you’ll only ever have homogeneous
collections, using generics and trait bounds is preferable since the
definitions will be monomorphized at compile time to use the concrete types.

With the the method using trait objects, on the other hand, one `Screen`
instance can hold a `Vec` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let’s see how that works, and then talk about the runtime
performance implications.

### Implementing the Trait

Now we’ll add some types that implement the `Draw` trait. We’re going to
provide the `Button` type. Again, actually implementing a GUI library is out of
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

The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components, such as a `TextField` type that might have those
plus a `placeholder` field instead. Each of the types we want to draw on the
screen will implement the `Draw` trait, with different code in the `draw`
method to define how to draw that particular type, like `Button` has here
(without the actual GUI code that’s out of scope of this chapter). `Button`,
for instance, might have an additional `impl` block containing methods related
to what happens if the button is clicked. These kinds of methods won’t apply to
types like `TextField`.

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
`Screen` instance. To this they can add a `SelectBox` and a `Button` by putting
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

When we wrote the library, we didn’t know that someone would add the
`SelectBox` type someday, but our `Screen` implementation was able to operate
on the new type and draw it because `SelectBox` implements the `Draw` type,
which means it implements the `draw` method.

This concept---of being concerned only with the messages a value responds to,
rather than the value’s concrete type---is similar to a concept in dynamically
typed languages called *duck typing*: if it walks like a duck, and quacks like
a duck, then it must be a duck! In the implementation of `run` on `Screen` in
Listing 17-5, `run` doesn’t need to know what the concrete type of each
component is. It doesn’t check to see if a component is an instance of a
`Button` or a `SelectBox`, it just calls the `draw` method on the component. By
specifying `Box<Draw>` as the type of the values in the `components` vector,
we’ve defined `Screen` to need values that we can call the `draw` method on.

<!-- I may be slow on the uptake here, but it seems like we're saying that
responsibility for how the type trait object behaves with the draw method is
called on it belongs to the trait object, and not to the draw method itself. Is
that an accurate summary? I want to make sure I'm clearly following the
argument! -->
<!-- Each type (like `Button` or `SelectBox`) that implements the `Draw` trait
can customize what happens in the body of the `draw` method. The trait object
is just responsible for making sure that the only things that are usable in
that context are things that implement the `Draw` trait. Does this clear it up
at all? Is there something we should clarify in the text? /Carol -->

The advantage of using trait objects and Rust’s type system to do something
similar to duck typing is that we never have to check that a value implements a
particular method at runtime or worry about getting errors if a value doesn’t
implement a method but we call it anyway. Rust won’t compile our code if the
values don’t implement the traits that the trait objects need.

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

This lets us know that either we’re passing something to `Screen` we didn’t
mean to pass, and we should pass a different type, or implement `Draw` on
`String` so that `Screen` is able to call `draw` on it.

### Trait Objects Perform Dynamic Dispatch

Recall from Chapter 10 our discussion on the monomorphization process performed
by the compiler when we use trait bounds on generics: the compiler generates
non-generic implementations of functions and methods for each concrete type
that we use in place of a generic type parameter. The code that results from
monomorphization is doing *static dispatch*. Static dispatch is when the
compiler knows what method you’re calling at compile time. This is opposed to
*dynamic dispatch*, when the compiler can’t tell at compile time which method
you’re calling. In these cases, the compiler emits code that will figure out at
runtime which method to call.

<!--I'm struggling to follow the static dispatch definition, can you expand
that a little? Which part of that is the static dispatch, pre-determining the
code called with a method and storing it? -->
<!-- Yes, in a way. We've expanded and moved the definitions of static and
dynamic dispatch together to better contrast, hopefully this helps? /Carol -->

When we use trait objects, Rust has to use dynamic dispatch. The compiler
doesn’t know all the types that might be used with the code using trait
objects, so it doesn’t know which method implemented on which type to call.
Instead, Rust uses the pointers inside of the trait object at runtime to know
which specific method to call. There’s a runtime cost when this lookup happens,
compared to static dispatch. Dynamic dispatch also prevents the compiler from
choosing to inline a method’s code which in turn prevents some optimizations.
We did get extra flexibility in the code that we wrote and were able to
support, though, so it’s a tradeoff to consider.

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
<!-- That sounds like a good solution, since the compiler will warn them in any
case. I read through, editing a little, and I agree we could afford to cut it,
I'm not sure it brings practical skills to the user -->
<!-- Ok, I've cut section way down to the practical pieces, but still explained a little bit /Carol -->

Only *object safe* traits can be made into trait objects. There are some
complex rules around all the properties that make a trait object safe, but in
practice, there are only two rules that are relevant. A trait is object safe if
all of the methods defined in the trait have the following properties:

- The return type isn’t `Self`
- There aren’t any generic type parameters

The `Self` keyword is an alias for the type we’re implementing traits or
methods on. Object safety is required for trait objects because once you have a
trait object, you no longer know what the concrete type implementing that trait
is. If a trait method returns the concrete `Self` type, but a trait object
forgets the exact type that it is, there’s no way that the method can use the
original concrete type that it’s forgotten. Same with generic type parameters
that are filled in with concrete type parameters when the trait is used: the
concrete types become part of the type that implements the trait. When the type
is erased by the use of a trait object, there’s no way to know what types to
fill in the generic type parameters with.

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

This means you can’t use this trait as a trait object in this way. If you’re
interested in more details on object safety, see Rust RFC 255 at
*https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md*.

## Implementing an Object-Oriented Design Pattern

The *state pattern* is an object-oriented design pattern. The crux of the
pattern is that a value has some internal state, represented by a set of *state
objects*, and the value’s behavior changes based on the internal state. The
state objects share functionality--in Rust, of course, we use structs and
traits rather than objects and inheritance. Each state object representing the
state is responsible for its own behavior and for governing when it should
change into another state. The value that holds a state object knows nothing
about the different behavior of the states or when to transition between states.

<!-- Below -- requirements for what, for what we need the value for? -->
<!-- I've clarified /Carol -->

Using the state pattern means when the business requirements of the program
change, we won’t need to change the code of the value holding the state or the
code that uses the value. We’ll only need to update the code inside one of the
state objects to change its rules, or perhaps add more state objects. Let’s
look at an example of the state design pattern and how to use it in Rust.

To explore this idea, we’ll implement a blog post workflow in an incremental
way. The blog’s final functionality will look like this:

1. A blog post starts as an empty draft.
2. Once the draft is done, a review of the post is requested.
3. Once the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts can’t
   accidentally get published.

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

We want to allow the user to create a new draft blog post with `Post::new`.
Then, we want to allow text to be added to the blog post while it’s in the
draft state. If we try to get the post’s content immediately, before
approval, nothing should happen because the post is still a draft. We’ve added
an `assert_eq!` here for demonstration purposes. An excellent unit test for
this would be to assert that a draft blog post returns an empty string from the
`content` method, but we’re not going to write tests for this example.

Next, we want to enable a request for a review of the post, and we want
`content` to return an empty string while waiting for the review. Lastly, when
the post receives approval, it should get published, meaning the text of the
post will be returned when `content` is called.

<!-- Below -- so this is where we'll implement the state pattern? If so, can
you make that explicit, just to be clear! I've added some text to the second
line, not sure if that's accurate though -->
<!-- Yes, the state pattern will be implemented within the `Post` type. I've
tweaked the wording a bit but you've pretty much got it! /Carol-->

Notice that the only type we’re interacting with from the crate is the `Post`
type. This type will use the state pattern and will hold a value that will be
one of three state objects representing the various states a post can be
in---draft, waiting for review, or published. Changing from one state to
another will be managed internally within the `Post` type. The states change in
response to the methods users of our library call on the `Post` instance, but
they don’t have to manage the state changes directly. This also means users
can’t make a mistake with the states, like publishing a post before it is
reviewed.

### Defining `Post` and Creating a New Instance in the Draft State

Let’s get started on the implementation of the library! We know we need a
public `Post` struct that holds some content, so let’s start with the
definition of the struct and an associated public `new` function to create an
instance of `Post`, as shown in Listing 17-12. We’ll also make a private
`State` trait. Then `Post` will hold a trait object of `Box<State>` inside an
`Option` in a private field named `state`. We’ll see why the `Option` is
necessary in a bit.

The `State` trait defines the behavior shared by different post states, and the
`Draft`, `PendingReview`, and `Published` states will all implement the `State`
trait. For now, the trait does not have any methods, and we’re going to start
by defining just the `Draft` state since that’s the state we want a post to
start in:

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
a new `Post` instance, a `State` trait, and a `Draft` struct

When we create a new `Post`, we set its `state` field to a `Some` value that
holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This
ensures whenever we create a new instance of `Post`, it’ll start out as a
draft. Because the `state` field of `Post` is private, there’s no way to create
a `Post` in any other state!

### Storing the Text of the Post Content

In the `Post::new` function, we set the `content` field to a new, empty
`String`. Listing 17-11 showed that we want to be able to call a method named
`add_text` and pass it a `&str` that’s then added to the text content of the
blog post. We implement this as a method rather than exposing the `content`
field as `pub`. This means we can implement a method later that will control
how the `content` field’s data is read. The `add_text` method is pretty
straightforward, so let’s add the implementation in Listing 17-13 to the `impl
Post` block:

Filename: src/lib.rs

```
impl Post {
    // ...snip...
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Listing 17-13: Implementing the `add_text` method to add text to a post’s
`content`

`add_text` takes a mutable reference to `self`, since we’re changing the `Post`
instance that we’re calling `add_text` on. We then call `push_str` on the
`String` in `content` and pass the `text` argument to add to the saved
`content`. This behavior doesn’t depend on the state the post is in so it’s not
part of the state pattern. The `add_text` method doesn’t interact with the
`state` field at all, but it is part of the behavior we want to support.

### Ensuring the Content of a Draft Post is Empty

Even after we’ve called `add_text` and added some content to our post, we still
want the `content` method to return an empty string slice since the post is
still in the draft state, as shown on line 8 of Listing 17-11. For now, let’s
implement the `content` method with the simplest thing that will fulfill this
requirement: always returning an empty string slice. We’re going to change this
later once we implement the ability to change a post’s state so it can be
published. So far, though, posts can only be in the draft state, so the post
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

Listing 17-14: Adding a placeholder implementation for the `content` method on
`Post` that always returns an empty string slice

With this added `content` method, everything in Listing 17-11 up to line 8
works as we intend.

### Requesting a Review of the Post Changes its State

Next up we need to add functionality to request a review of a post, which
should change its state from `Draft` to `PendingReview`. We want to give `Post`
a public method named `request_review` that will take a mutable reference to
`self`. Then we’re going to call an internal `request_review` method on the
current state of `Post`, and this second `request_review` method will consume
the current state and return a new state. To consume the old state, the second
`request_review` method needs to take ownership of the state value. This is
where the `Option` comes in: we’re going to take the `Some` value out of the
`state` field and leave a `None` in its place, since Rust doesn’t let us have
unpopulated fields in structs. Then we’ll set the post’s `state` value to the
result of this operation. Listing 17-15 shows this code:

<!-- NOTE TO DE/AU: We might want to move this explanation to after the code if
you want to add wingdings, we can see once we transfer it to Word -->

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
ownership of `Box<Self>`, invalidating the old state so that the state value of
the `Post` can transform itself into a new state.

<!-- Above -- so Post can transform, or so Draft can transform? -->
<!-- Technically it's so the Draft value can transform into another value,
which changes the state of Post-- I've tried to clarify. /Carol -->

The `request_review` method on `Draft` needs to return a new, boxed instance of
a new `PendingReview` struct, which represents the state when a post is waiting
for a review. The `PendingReview` struct also implements the `request_review`
method, but doesn’t do any transformations. Rather, it returns itself, since
when we request a review on a post already in the `PendingReview` state, it
should stay in the `PendingReview` state.

Now we can start seeing the advantages of the state pattern: the
`request_review` method on `Post` is the same no matter its `state` value. Each
state is responsible for its own rules.

We’re going to leave the `content` method on `Post` as it is, returning an
empty string slice. We can now have a `Post` in the `PendingReview` state as
well as the `Draft` state, but we want the same behavior in the `PendingReview`
state. Listing 17-11 now works up until line 11!

### Adding the `approve` Method that Changes the Behavior of `content`

The `approve` method will be similar to the `request_review` method: it will
set `state` to the value that the current state says it should have when that
state is approved, shown in Listing 17-16.

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

We add the `approve` method to the `State` trait, and add a new struct that
implements `State`, the `Published` state.

Similar to `request_review`, if we call the `approve` method on a `Draft`, it
will have no effect since it will return `self`. When we call `approve` on
`PendingReview`, it returns a new, boxed instance of the `Published` struct.
The `Published` struct implements the `State` trait, and for both the
`request_review` method and the `approve` method, it returns itself, since the
post should stay in the `Published` state in those cases.

Now to update the `content` method on `Post`: if the state is `Published` we
want to return the value in the post’s `content` field; otherwise we want to
return an empty string slice, as shown in Listing 17-17:

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

Because the goal is to keep all these rules inside the structs that implement
`State`, we call a `content` method on the value in `state` and pass the post
instance (that is, `self`) as an argument. Then we return the value that’s
returned from using the `content` method on the `state` value.

We call the `as_ref` method on the `Option` so we get a reference to the value
inside the `Option` rather than ownership of it. We’re then calling the
`unwrap` method, which we know will never panic, because we know the methods on
`Post` ensure that `state` will always contain a `Some` value when those
methods are done. This is one of the cases we talked about in Chapter 12 when
we know that a `None` value is never possible, even though the compiler isn’t
able to understand that.

We’ll put the logic for the content to return in the `content` method on the
`State` trait, as shown in Listing 17-18:

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

We add a default implementation for the `content` method that returns an empty
string slice. That means we don’t need to implement `content` on the `Draft`
and `PendingReview` structs. The `Published` struct will override the `content`
method and return the value in `post.content`.

Note that we need lifetime annotations on this method, like we discussed in
Chapter 10. We’re taking a reference to a `post` as an argument, and returning
a reference to part of that `post`, so the lifetime of the returned reference
is related to the lifetime of the `post` argument.

<!-- Is this it finished, without the touch up we make to get rid of the empty
string? That's pretty awesome coding, maybe give it some ceremony here. Does
all of 17-11 now work? -->
<!-- Yep! Good point, so added! /Carol -->

And we’re done-- all of Listing 17-11 now works! We’ve implemented the state
pattern with the rules of the blog post workflow. The logic around the rules
lives in the state objects rather than scattered throughout `Post`.

### Tradeoffs of the State Pattern

We’ve shown that Rust is capable of implementing the object-oriented state
pattern to encapsulate the different kinds of behavior a post should have in
each state. The methods on `Post` know nothing about the different kinds of
behavior. The way this code is organized, we only have to look in one place to
know the different ways a published post can behave: the implementation of the
`State` trait on the `Published` struct.

If we were to create an alternative implementation that didn’t use the state
pattern we might use `match` statements in the methods on `Post`, or even in
the `main` code that checks the state of the post and changes behavior in those
places instead. That would mean we’d have to look in a lot of places to
understand all the implications of a post being in the published state! This
would only increase the more states we added: each of those `match` statements
would need another arm.

With the state pattern, the `Post` methods and the places we use `Post` don’t
need `match` statements, and to add a new state we would only need to add a new
`struct` and implement the trait methods on that one struct.

This implementation is easy to extend to add more functionality. To see the
simplicity of maintaining code that uses this patterns, try out a few of these
suggestions:

- Allow users to add text content only when a post is in the `Draft` state
- Add a `reject` method that changes the post’s state from `PendingReview` back
  to `Draft`
- Require two calls to `approve` before the state can be changed to `Published`

One downside of the state pattern is that, because the states implement the
transitions between states, some of the states are coupled to each other. If we
add another state between `PendingReview` and `Published`, such as `Scheduled`,
we would have to change the code in `PendingReview` to transition to
`Scheduled` instead. It would be less work if `PendingReview` wouldn’t need to
change with the addition of a new state, but that would mean switching to
another design pattern.

Another downside is that we find ourselves with a few bits of duplicated logic.
To eliminate this, we might try to make default implementations for the
`request_review` and `approve` methods on the `State` trait that return `self`,
but this would violate object safety, since the trait doesn’t know what the
concrete `self` will be exactly. We want to be able to use `State` as a trait
object, so we need its methods to be object safe.

The other duplication is the similar implementations of the `request_review`
and `approve` methods on `Post`. Both methods delegate to the implementation of
the same method on the value in the `state` field of `Option`, and set the new
value of the `state` field to the result. If we had a lot of methods on `Post`
that followed this pattern, we might consider defining a macro to eliminate the
repetition (see Appendix E on macros).

By implementing this pattern exactly as it’s defined for object-oriented
languages, we’re not taking full advantage of Rust’s strengths as much as we
could. Let’s take a look at some changes we can make to this code that can make
invalid states and transitions into compile time errors.

#### Encoding States and Behavior as Types

We’re going to show how to rethink the state pattern to get a different set of
tradeoffs. Rather than encapsulating the states and transitions completely so
that outside code has no knowledge of them, we’re going to encode the states
into different types. Like this, Rust’s type checking system will make attempts
to use draft posts where only published posts are allowed into a compiler error.

Let’s consider the first part of `main` from Listing 17-11:

Filename: src/main.rs

```
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

We still enable the creation of new posts in the draft state using `Post::new`,
and the ability to add text to the post’s content. But instead of having a
`content` method on a draft post that returns an empty string, we’ll make it so
that draft posts don’t have the `content` method at all. That way, if we try to
get a draft post’s content, we’ll get a compiler error telling us the method
doesn’t exist. This will make it impossible for us to accidentally display
draft post content in production, since that code won’t even compile. Listing
17-19 shows the definition of a `Post` struct, a `DraftPost` struct, and
methods on each:

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
`Post`, it returns an instance of `DraftPost`. Because `content` is private,
and there aren’t any functions that return `Post`, it’s not possible to create
an instance of `Post` right now.

`DraftPost` has an `add_text` method so we can add text to `content` as before,
but note that `DraftPost` does not have a `content` method defined! So now the
program ensures all posts start as draft posts, and draft posts don’t have
their content available for display. Any attempt to get around these
constraints will result in a compiler error.

#### Implementing Transitions as Transformations into Different Types

So how do we get a published post then? We want to enforce the rule that a
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

Listing 17-20: A `PendingReviewPost` that gets created by calling
`request_review` on `DraftPost`, and an `approve` method that turns a
`PendingReviewPost` into a published `Post`

The `request_review` and `approve` methods take ownership of `self`, thus
consuming the `DraftPost` and `PendingReviewPost` instances and transforming
them into a `PendingReviewPost` and a published `Post`, respectively. This way,
we won’t have any `DraftPost` instances lingering around after we’ve called
`request_review` on them, and so forth. `PendingReviewPost` doesn’t have a
`content` method defined on it, so attempting to read its content results in a
compiler error, as with `DraftPost`. Because the only way to get a published
`Post` instance that does have a `content` method defined is to call the
`approve` method on a `PendingReviewPost`, and the only way to get a
`PendingReviewPost` is to call the `request_review` method on a `DraftPost`,
we’ve now encoded the blog post workflow into the type system.

This does mean we have to make some small changes to `main`. The
`request_review` and `approve` methods return new instances rather than
modifying the struct they’re called on, so we need to add more `let post = `
shadowing assignments to save the returned instances. We also can’t have the
assertions about the draft and pending review post’s contents being empty
strings, nor do we need them: we can’t compile code that tries to use the
content of posts in those states any longer. The updated code in `main` is
shown in Listing 17-21:

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

These changes we need to make to `main` to reassign `post` means this
implementation doesn’t quite follow the object-oriented state pattern anymore:
the transformations between the states are no longer encapsulated entirely
within the `Post` implementation. However, our gain is that invalid states are
now impossible because of the type system and the type checking that happens at
compile time! This ensures that certain bugs, such as the content of an
unpublished post being displayed, will be discovered before they make it to
production.

Try the tasks suggested for additional requirements that we mentioned at the
start of this section on this code, to see how working with this version of the
code feels.

We’ve seen that even though Rust is capable of implementing object-oriented
design patterns, other patterns like encoding state into the type system are
also available in Rust. These patterns have different tradeoffs. While you may
be very familiar with object-oriented patterns, rethinking the problem in order
to take advantage of Rust’s features can provide benefits like preventing some
bugs at compile-time. Object-oriented patterns won’t always be the best
solution in Rust, because of the features like ownership that object-oriented
languages don’t have.

## Summary

No matter whether you think Rust is an object-oriented language or not after
reading this chapter, you’ve now seen that trait objects are a way to get some
object-oriented features in Rust. Dynamic dispatch can give your code some
flexibility in exchange for a bit of runtime performance. This flexibility can
be used to implement object-oriented patterns that can help with the
maintainability of your code. Rust also has other different features, like
ownership, that object-oriented languages don’t have. An object-oriented
pattern won’t always be the best way to take advantage of Rust’s strengths, but
is an available option.

Next, let’s look at another feature of Rust that enables lots of flexibility:
patterns. We’ve looked at them briefly throughout the book, but haven’t seen
everything they’re capable of yet. Let’s go!
