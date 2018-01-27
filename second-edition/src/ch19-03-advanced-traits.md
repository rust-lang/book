## Advanced Traits

We first covered traits in the “Traits: Defining Shared Behavior” section of
Chapter 10 but, like lifetimes, we didn’t get to some of the more advanced
details. Now that we know more Rust, we can get into the nitty-gritty.

### Associated Types Specify Placeholder Types in Trait Definitions

*Associated types* are a way of associating a type placeholder with a trait
such that the trait method definitions can use these placeholder types in their
signatures. The implementor of a trait will specify the concrete type to be
used in this type’s place for the particular implementation. That way, we can
define a trait that uses some types without needing to know exactly what those
types are until the trait is implemented.

<!-- Can you say what this is useful for -- it seems like a way to not to have
to specify a type prior to use, is that right? -->
<!-- Prior to trait implementation, yes. /Carol -->

We’ve described most of the things in this chapter as being needed very rarely.
Associated types are somewhere in the middle; they’re used more rarely than the
rest of the book, but more commonly than many of the things in this chapter.

One example of a trait with an associated type is the `Iterator` trait provided
by the standard library. This has an associated type named `Item` that stands
in for the type of the values it’s iterating over. In “The `Iterator` Trait and
the `next` Method” section of Chapter 13, we mentioned that the definition of
the `Iterator` trait is as shown in [Listing 19-20][Listing-19-20]:

[Listing-19-20]: #Listing-19-20
<a name="Listing-19-20"></a>

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

<span class="caption">Listing 19-20: The definition of the `Iterator` trait
that has an associated type `Item`</span>

The `Iterator` trait has an associated type named `Item`. This is a placeholder
type, and the `next` method will return values of type `Option<Self::Item>`.
Implementors of this trait will specify the concrete type for `Item`, and the
`next` method will return an `Option` containing a value of that concrete type.

#### Associated Types Versus Generics

This may seem like a similar concept to generics, in that it allows us to
define a function without specifying what types it can deal with. So why use
associated types?

Let’s examine the difference with an example that implements the `Iterator`
trait on the `Counter` struct from Chapter 13. In [Listing 13-21][Listing-13-21], we specified
that the `Item` type was `u32`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

This feels similar to generics. So why not just define the `Iterator` trait
with generics as shown in [Listing 19-21][Listing-19-21]?

[Listing-19-21]: #Listing-19-21
<a name="Listing-19-21"></a>

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

<span class="caption">Listing 19-21: A hypothetical definition of the
`Iterator` trait using generics</span>

The difference lies in the fact that when using generics like in [Listing 19-21][Listing-19-21],
we have to annotate the types in each implementation. This is because we can
also implement `Iterator<String> for Counter`, or any other type, which would
give us multiple implementations of `Iterator` for `Counter`. In other words,
when a trait has a generic parameter, it can be implemented for a type multiple
times, changing the concrete types of the generic type parameters each time.
When we use the `next` method on `Counter`, we’d then have to provide type
annotations to indicate which implementation of `Iterator` we wanted to use.

With associated types, we don’t need to annotate types because we can’t
implement a trait on a type multiple times. With [Listing 19-20][Listing-19-20], we can only
choose once what the type of `Item` will be, because there can only be one `impl
Iterator for Counter`. We don’t have to specify that we want an iterator of
`u32` values everywhere that we call `next` on `Counter`.

### Default Generic Type Parameters and Operator Overloading

When we use generic type parameters, we can specify a default concrete type for
the generic type. This eliminates the need for implementors of the trait to
specify a concrete type if the default type works. The syntax for specifying a
default type for a generic type is to put `<PlaceholderType=ConcreteType>` when
declaring the generic type.

A great example of a situation where this is useful is with operator
overloading. Operator overloading is customizing the behavior of an operator
(like `+`) in particular situations.

<!-- Are we safe in assuming the reader is familiar with operator overloading
and why/when to use it, or is it worth giving a quick definition here? -->
<!-- Added /Carol -->

Rust does not allow you to create your own operators or overload arbitrary
operators, but you *can* overload the operations and corresponding traits
listed in `std::ops` by implementing the traits associated with the operator.
For example, in [Listing 19-22][Listing-19-22] we overload the `+` operator to add two `Point`
instances together. We do this by implementing the `Add` trait on a `Point`
struct:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-22]: #Listing-19-22
<a name="Listing-19-22"></a>

```rust
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

<span class="caption">Listing 19-22: Implementing the `Add` trait to overload
the `+` operator for `Point` instances</span>

The `add` method adds the `x` values of two `Point` instances together and the
`y` values of two `Point` instances together to create a new `Point`. The `Add`
trait has an associated type named `Output` that determines the type returned
from the `add` method.

The default generic type here is within the `Add` trait. Here’s its definition:

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

This should look generally familiar, as a trait with one method and an
associated type. The new part here is the `RHS=Self` in the angle brackets:
this syntax is called *default type parameters*. The `RHS` generic type
parameter---short for “right hand side”---that’s used to define the type of the
`rhs` parameter in the `add` method. If we don’t specify a concrete type for
`RHS` when we implement the `Add` trait, the type of `RHS` will default to
`Self`, which will be the type we’re implementing `Add` on.

<!-- Can you say what we're looking out for in this next trait -->
<!-- Done/reworked to be less repetitive with the Point example /Carol -->

When we implemented `Add` for `Point`, we made use of the default for `RHS`
because we wanted to add two `Point` instances together. Let’s look at an
example of implementing the `Add` trait where we want to customize the `RHS`
type rather than using the default.

We have two structs holding values in different units, `Millimeters` and
`Meters`. We want to be able to add values in millimeters to values in meters,
and have the implementation of `Add` do the conversion correctly. We can
implement `Add` for `Millimeters` with `Meters` as the right hand side as shown
in [Listing 19-23][Listing-19-23]:

<span class="filename">Filename: src/lib.rs</span>

[Listing-19-23]: #Listing-19-23
<a name="Listing-19-23"></a>

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

<span class="caption">Listing 19-23: Implementing the `Add` trait on
`Millimeters` to be able to add `Millimeters` to `Meters`</span>

To be able to add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to
set the value of the `RHS` type parameter instead of using the default of
`Self`.

Default type parameters are used in two main ways:

1. To extend a type without breaking existing code.
2. To allow customization in specific cases most users won’t need.

<!-- Above, in 2., do you mean customization used in corner cases? -->
<!-- Yes, I'm not sure how that's different than what we've stated here or how
it could be clearer /Carol-->

The standard library’s `Add` trait is an example of the second purpose: most of
the time, you’re adding two like types together, but it gives the ability for
customizing beyond that. Using a default type parameter in the `Add` trait
definition means you don’t have to specify the extra parameter most of the
time. In other words, a little bit of implementation boilerplate isn’t needed,
making it easier to use the trait.

The first purpose is similar, but in reverse: if we want to add a type
parameter to an existing trait, we can give it a default to let us extend the
functionality of the trait without breaking the existing implementation code.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as
another trait’s method, nor can it prevent us from implementing both of these
traits on one type. It’s also possible to have a method implemented directly on
the type with the same name as methods from traits as well!

<!-- Same name as the type, you mean? -->
<!-- No, the same name as methods implemented from traits. I've tried to
clarify /Carol -->

When calling methods with the same name, then, we need to tell Rust which one
we want to use. Consider the code in [Listing 19-24][Listing-19-24] where we’ve defined two
traits, `Pilot` and `Wizard`, that both have a method called `fly`. We then
implement both traits on a type `Human` that itself already has a method named
`fly` implemented on it. Each `fly` method does something different:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-24]: #Listing-19-24
<a name="Listing-19-24"></a>

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

<span class="caption">Listing 19-24: Two traits defined to have a `fly` method,
and implementations of those traits on the `Human` type in addition to a `fly`
method on `Human` directly</span>

When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in [Listing 19-25][Listing-19-25]:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-25]: #Listing-19-25
<a name="Listing-19-25"></a>

```rust
# trait Pilot {
#     fn fly(&self);
# }
#
# trait Wizard {
#     fn fly(&self);
# }
#
# struct Human;
#
# impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
# }
#
# impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
# }
#
# impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
# }
#
fn main() {
    let person = Human;
    person.fly();
}
```

<span class="caption">Listing 19-25: Calling `fly` on an instance of
`Human`</span>

Running this will print out `*waving arms furiously*`, which shows that Rust
called the `fly` method implemented on `Human` directly.

In order to call the `fly` methods from either the `Pilot` trait or the
`Wizard` trait, we need to use more explicit syntax in order to specify which
`fly` method we mean. This syntax is demonstrated in [Listing 19-26][Listing-19-26]:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-26]: #Listing-19-26
<a name="Listing-19-26"></a>

```rust
# trait Pilot {
#     fn fly(&self);
# }
#
# trait Wizard {
#     fn fly(&self);
# }
#
# struct Human;
#
# impl Pilot for Human {
#     fn fly(&self) {
#         println!("This is your captain speaking.");
#     }
# }
#
# impl Wizard for Human {
#     fn fly(&self) {
#         println!("Up!");
#     }
# }
#
# impl Human {
#     fn fly(&self) {
#         println!("*waving arms furiously*");
#     }
# }
#
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

<span class="caption">Listing 19-26: Specifying which trait’s `fly` method we
want to call</span>

Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also choose to write
`Human::fly(&person)`, which is equivalent to `person.fly()` that we had in
[Listing 19-26][Listing-19-26], but is a bit longer to write if we don’t need to disambiguate.

Running this code will print:

```text
This is your captain speaking.
Up!
*waving arms furiously*
```

Because the `fly` method takes a `self` parameter, if we had two *types* that
both implement one *trait*, Rust can figure out which implementation of a trait
to use based on the type of `self`.

However, associated functions that are part of traits don’t have a `self`
parameter. When two types in the same scope implement that trait, Rust can’t
figure out which type we mean unless we use *fully qualified syntax*. For
example, take the `Animal` trait in [Listing 19-27][Listing-19-27] that has the associated
function `baby_name`, the implementation of `Animal` for the struct `Dog`, and
the associated function `baby_name` defined on `Dog` directly:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-27]: #Listing-19-27
<a name="Listing-19-27"></a>

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

<span class="caption">Listing 19-27: A trait with an associated function and a
type that has an associated function with the same name that also implements
the trait</span>

This code is for an animal shelter where they want to give all puppies the name
Spot, which is implemented in the `baby_name` associated function that is
defined on `Dog`. The `Dog` type also implements the trait `Animal`, which
describes characteristics that all animals have. Baby dogs are called puppies,
and that is expressed in the implementation of the `Animal` trait on `Dog` in
the `baby_name` function associated with the `Animal` trait.

In `main`, we’re calling the `Dog::baby_name` function, which calls the
associated function defined on `Dog` directly. This code prints:

```text
A baby dog is called a Spot
```

This isn’t what we wanted. We want to call the `baby_name` function that’s part
of the `Animal` trait that we implemented on `Dog` so that we print `A baby dog
is called a puppy`. The technique we used in [Listing 19-26][Listing-19-26] doesn’t help here;
if we change `main` to be the code in [Listing 19-28][Listing-19-28], we’ll get a compilation
error:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-28]: #Listing-19-28
<a name="Listing-19-28"></a>

```rust,ignore
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

<span class="caption">Listing 19-28: Attempting to call the `baby_name`
function from the `Animal` trait, but Rust doesn’t know which implementation to
use</span>

Because `Animal::baby_name` is an associated function rather than a method, and
thus doesn’t have a `self` parameter, Rust has no way to figure out which
implementation of `Animal::baby_name` we want. We’ll get this compiler error:

```text
error[E0283]: type annotations required: cannot resolve `_: Animal`
  --> src/main.rs:20:43
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^
   |
   = note: required by `Animal::baby_name`
```

To disambiguate and tell Rust that we want to use the implementation of
`Animal` for `Dog`, we need to use *fully qualified syntax*, which is the most
specific we can be when calling a function. [Listing 19-29][Listing-19-29] demonstrates how to
use fully qualified syntax:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-29]: #Listing-19-29
<a name="Listing-19-29"></a>

```rust
# trait Animal {
#     fn baby_name() -> String;
# }
#
# struct Dog;
#
# impl Dog {
#     fn baby_name() -> String {
#         String::from("Spot")
#     }
# }
#
# impl Animal for Dog {
#     fn baby_name() -> String {
#         String::from("puppy")
#     }
# }
#
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

<span class="caption">Listing 19-29: Using fully qualified syntax to specify
that we want to call the `baby_name` function from the `Animal` trait as
implemented on `Dog`</span>

We’re providing Rust with a type annotation within the angle brackets, and
we’re specifying that we want to call the `baby_name` method from the `Animal`
trait as implemented on `Dog` by saying that we want to treat the `Dog` type as
an `Animal` for this function call. This code will now print what we want:

```text
A baby dog is called a puppy
```

In general, fully qualified syntax is defined as:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

For associated functions, there would not be a `receiver`, there would only be
the list of other arguments. We could choose to use fully qualified syntax
everywhere that we call functions or methods. However, we’re allowed to leave
out any part of this syntax that Rust is able to figure out from other
information in the program. We only need to use this more verbose syntax in
cases where there are multiple implementations that use the same name and Rust
needs help in order to know which implementation we want to call.

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

Sometimes, we may need one trait to use another trait’s functionality. In this
case, we need to be able to rely on the dependent trait also being implemented.
The trait we’re relying on is a *supertrait* of the trait we’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print out a value framed in asterisks. That
is, given a `Point` struct that implements `Display` to result in `(x, y)`,
when we call `outline_print` on a `Point` instance that has 1 for `x` and 3 for
`y`, it should print the following:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, we want to use the `Display` trait’s
functionality. We therefore need to specify that the `OutlinePrint` trait will
only work for types that also implement `Display` and therefore provide the
functionality that `OutlinePrint` needs. We can do that in the trait definition
by specifying `OutlinePrint: Display`. This is similar to adding a trait bound
to the trait. [Listing 19-30][Listing-19-30] shows an implementation of the `OutlinePrint` trait:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-30]: #Listing-19-30
<a name="Listing-19-30"></a>

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

<span class="caption">Listing 19-30: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that’s automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding`:
Display` after the trait name we’d get an error saying that no method named
`to_string` was found for the type `&Self` in the current scope.

Let’s see what happens if we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:

<span class="filename">Filename: src/main.rs</span>

```rust
# trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We’ll get an error saying that `Display` is required but not implemented:

```text
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter;
   try using `:?` instead if you are using a format string
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`
```

Once we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Then, implementing the `OutlinePrint` trait on `Point` will compile
successfully and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.

### The Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned the
orphan rule that says we’re allowed to implement a trait on a type as long as
either the trait or the type are local to our crate. It is possible to get
around this restriction using the *newtype pattern*, which involves creating a
new type in a tuple struct (we covered tuple structs in the “Tuple Structs
without Named Fields to Create Different Types” section of Chapter 5). The
tuple struct will have one field and will be a thin wrapper around the type we
want to implement a trait for. Then the wrapper type is local to our crate, and
we can implement the trait on the wrapper. “Newtype” is a term originating from
the Haskell programming language. There’s no runtime performance penalty for
using this pattern, and the wrapper type is elided at compile time.

As an example, we want to implement `Display` on `Vec`, which the orphan rule
prevents us from doing directly because the `Display` trait and the `Vec` type
are both defined outside of our crate. We can make a `Wrapper` struct that
holds an instance of `Vec`, then we can implement `Display` on `Wrapper` and
use the `Vec` value as shown in [Listing 19-31][Listing-19-31]:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-31]: #Listing-19-31
<a name="Listing-19-31"></a>

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

<span class="caption">Listing 19-31: Creating a `Wrapper` type around
`Vec<String>` to be able to implement `Display`</span>

The implementation of `Display` uses `self.0` to access the inner `Vec`,
because `Wrapper` is a tuple struct and the `Vec` is the item at index 0 in the
tuple. Then we can use the functionality of the `Display` type on `Wrapper`.

<!-- What is self.0? I think the syntax here might need a bit more talking
through -->
<!-- `Wrapper` is a tuple struct; we covered those in chapter 5, added a back
reference to that section in the first paragraph of this section but we've used
the `.0` syntax in multiple places before here /Carol -->

The downside of this method is that, because `Wrapper` is a new type, it
doesn’t have the methods of the value it’s holding; we’d have to implement all
the methods of `Vec` directly on `Wrapper`, so that it can delegate to
`self.0`--- this allows us to treat `Wrapper` exactly like a `Vec`. If we
wanted the new type to have every single method that the inner type has,
implementing the `Deref` trait (discussed in Chapter 15 in the “Treating Smart
Pointers like Regular References with the `Deref` Trait” section) on the
wrapper to return the inner type can be a solution. If we don’t want the
wrapper type to have all the methods of the inner type, in order to restrict
the wrapper type’s behavior for example, we’d have to implement just the
methods we do want ourselves.

That’s how the newtype pattern is used in relation to traits; it’s also a
useful pattern without having traits involved. Let’s switch focus now to talk
about some advanced ways to interact with Rust’s type system.

[Listing-19-20]: ch19-03-advanced-traits.html#Listing-19-20
[Listing-13-21]: ch13-02-iterators.html#Listing-13-21
[Listing-19-21]: ch19-03-advanced-traits.html#Listing-19-21
[Listing-19-22]: ch19-03-advanced-traits.html#Listing-19-22
[Listing-19-23]: ch19-03-advanced-traits.html#Listing-19-23
[Listing-19-24]: ch19-03-advanced-traits.html#Listing-19-24
[Listing-19-25]: ch19-03-advanced-traits.html#Listing-19-25
[Listing-19-26]: ch19-03-advanced-traits.html#Listing-19-26
[Listing-19-27]: ch19-03-advanced-traits.html#Listing-19-27
[Listing-19-28]: ch19-03-advanced-traits.html#Listing-19-28
[Listing-19-29]: ch19-03-advanced-traits.html#Listing-19-29
[Listing-19-30]: ch19-03-advanced-traits.html#Listing-19-30
[Listing-19-31]: ch19-03-advanced-traits.html#Listing-19-31
