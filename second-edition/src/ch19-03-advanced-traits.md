## Advanced Traits

We covered traits in Chapter 10, but like lifetimes, we didn’t get to all the
details. Now that we know more Rust, we can get into the nitty-gritty.

### Associated Types

*Associated types* are a way of associating a type placeholder with a trait
such that the trait method definitions can use these placeholder types in their
signatures. The implementor of a trait will specify the concrete type to be
used in this type’s place for the particular implementation.

We’ve described most of the things in this chapter as being very rare.
Associated types are somewhere in the middle; they’re more rare than the rest
of the book, but more common than many of the things in this chapter.

An example of a trait with an associated type is the `Iterator` trait provided
by the standard library. It has an associated type named `Item` that stands in
for the type of the values that we’re iterating over. We mentioned in Chapter
13 that the definition of the `Iterator` trait is as shown in Listing 19-20:

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

This says that the `Iterator` trait has an associated type named `Item`. `Item`
is a placeholder type, and the return value of the `next` method will return
values of type `Option<Self::Item>`. Implementors of this trait will specify
the concrete type for `Item`, and the `next` method will return an `Option`
containing a value of whatever type the implementor has specified.

#### Associated Types Versus Generics

When we implemented the `Iterator` trait on the `Counter` struct in Listing 13-6,
we specified that the `Item` type was `u32`:

```rust,ignore
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
```

This feels similar to generics. So why isn’t the `Iterator` trait defined as
shown in Listing 19-21?

[Listing-19-21]: #Listing-19-21
<a name="Listing-19-21"></a>

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

<span class="caption">Listing 19-21: A hypothetical definition of the
`Iterator` trait using generics</span>

The difference is that with the definition in Listing 19-21, we could also
implement `Iterator<String> for Counter`, or any other type as well, so that
we’d have multiple implementations of `Iterator` for `Counter`. In other words,
when a trait has a generic parameter, we can implement that trait for a type
multiple times, changing the generic type parameters’ concrete types each time.
Then when we use the `next` method on `Counter`, we’d have to provide type
annotations to indicate which implementation of `Iterator` we wanted to use.

With associated types, we can’t implement a trait on a type multiple times.
Using the actual definition of `Iterator` from Listing 19-20, we can only
choose once what the type of `Item` will be, since there can only be one `impl
Iterator for Counter`. We don’t have to specify that we want an iterator of
`u32` values everywhere that we call `next` on `Counter`.

The benefit of not having to specify generic type parameters when a trait uses
associated types shows up in another way as well. Consider the two traits
defined in Listing 19-22. Both are defining a trait having to do with a graph
structure that contains nodes of some type and edges of some type. `GGraph` is
defined using generics, and `AGraph` is defined using associated types:

[Listing-19-22]: #Listing-19-22
<a name="Listing-19-22"></a>

```rust
trait GGraph<Node, Edge> {
    // methods would go here
}

trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}
```

<span class="caption">Listing 19-22: Two graph trait definitions, `GGraph`
using generics and `AGraph` using associated types for `Node` and `Edge`</span>

Let’s say we wanted to implement a function that computes the distance between
two nodes in any types that implement the graph trait. With the `GGraph` trait
defined using generics, our `distance` function signature would have to look
like Listing 19-23:

[Listing-19-23]: #Listing-19-23
<a name="Listing-19-23"></a>

```rust
# trait GGraph<Node, Edge> {}
#
fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    // --snip--
#     0
}
```

<span class="caption">Listing 19-23: The signature of a `distance` function
that uses the trait `GGraph` and has to specify all the generic
parameters</span>

Our function would need to specify the generic type parameters `N`, `E`, and
`G`, where `G` is bound by the trait `GGraph` that has type `N` as its `Node`
type and type `E` as its `Edge` type. Even though `distance` doesn’t need to
know the types of the edges, we’re forced to declare an `E` parameter, because
we need to to use the `GGraph` trait and that requires specifying the type for
`Edge`.

Contrast with the definition of `distance` in Listing 19-24 that uses the
`AGraph` trait from Listing 19-22 with associated types:

[Listing-19-24]: #Listing-19-24
<a name="Listing-19-24"></a>

```rust
# trait AGraph {
#     type Node;
#     type Edge;
# }
#
fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
    // --snip--
#     0
}
```

<span class="caption">Listing 19-24: The signature of a `distance` function
that uses the trait `AGraph` and the associated type `Node`</span>

This is much cleaner. We only need to have one generic type parameter, `G`,
with the trait bound `AGraph`. Since `distance` doesn’t use the `Edge` type at
all, it doesn’t need to be specified anywhere. To use the `Node` type
associated with `AGraph`, we can specify `G::Node`.

#### Trait Objects with Associated Types

You may have been wondering why we didn’t use a trait object in the `distance`
functions in Listing 19-23 and Listing 19-24. The signature for the `distance`
function using the generic `GGraph` trait does get a bit more concise using a
trait object:

```rust
# trait GGraph<Node, Edge> {}
#
fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
    // --snip--
#     0
}
```

This might be a more fair comparison to Listing 19-24. Specifying the `Edge`
type is still required, though, which means Listing 19-24 is still preferable
since we don’t have to specify something we don’t use.

It’s not possible to change Listing 19-24 to use a trait object for the graph,
since then there would be no way to refer to the `AGraph` trait’s associated
type.

It is possible in general to use trait objects of traits that have associated
types, though; Listing 19-25 shows a function named `traverse` that doesn’t
need to use the trait’s associated types in other arguments. We do, however,
have to specify the concrete types for the associated types in this case. Here,
we’ve chosen to accept types that implement the `AGraph` trait with the
concrete type of `usize` as their `Node` type and a tuple of two `usize` values
for their `Edge` type:

```rust
# trait AGraph {
#     type Node;
#     type Edge;
# }
#
fn traverse(graph: &AGraph<Node=usize, Edge=(usize, usize)>) {
    // --snip--
}
```

While trait objects mean that we don’t need to know the concrete type of the
`graph` parameter at compile time, we do need to constrain the use of the
`AGraph` trait in the `traverse` function by the concrete types of the
associated types. If we didn’t provide this constraint, Rust wouldn’t be able
to figure out which `impl` to match this trait object to.

### Operator Overloading and Default Type Parameters

The `<PlaceholderType=ConcreteType>` syntax is used in another way as well: to
specify the default type for a generic type. A great example of a situation
where this is useful is operator overloading.

Rust does not allow you to create your own operators or overload arbitrary
operators, but the operations and corresponding traits listed in `std::ops` can
be overloaded by implementing the traits associated with the operator. For
example, Listing 19-25 shows how to overload the `+` operator by implementing
the `Add` trait on a `Point` struct so that we can add two `Point` instances
together:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-25]: #Listing-19-25
<a name="Listing-19-25"></a>

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

<span class="caption">Listing 19-25: Implementing the `Add` trait to overload
the `+` operator for `Point` instances</span>

We’ve implemented the `add` method to add the `x` values of two `Point`
instances together and the `y` values of two `Point` instances together to
create a new `Point`. The `Add` trait has an associated type named `Output`
that’s used to determine the type returned from the `add` method.

Let’s look at the `Add` trait in a bit more detail. Here’s its definition:

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

This should look familiar; it’s a trait with one method and an associated type.
The new part is the `RHS=Self` in the angle brackets: this syntax is called
*default type parameters*. `RHS` is a generic type parameter (short for “right
hand side”) that’s used for the type of the `rhs` parameter in the `add`
method. If we don’t specify a concrete type for `RHS` when we implement the
`Add` trait, the type of `RHS` will default to the type of `Self` (the type
that we’re implementing `Add` on).

Let’s look at another example of implementing the `Add` trait. Imagine we have
two structs holding values in different units, `Millimeters` and `Meters`. We
can implement `Add` for `Millimeters` in different ways as shown in Listing 19-26:

[Listing-19-26]: #Listing-19-26
<a name="Listing-19-26"></a>

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

<span class="caption">Listing 19-26: Implementing the `Add` trait on
`Millimeters` to be able to add `Millimeters` to `Millimeters` and
`Millimeters` to `Meters`</span>

If we’re adding `Millimeters` to other `Millimeters`, we don’t need to
parameterize the `RHS` type for `Add` since the default `Self` type is what we
want. If we want to implement adding `Millimeters` and `Meters`, then we need
to say `impl Add<Meters>` to set the value of the `RHS` type parameter.

Default type parameters are used in two main ways:

1. To extend a type without breaking existing code.
2. To allow customization in a way most users don’t want.

The `Add` trait is an example of the second purpose: most of the time, you’re
adding two like types together. Using a default type parameter in the `Add`
trait definition makes it easier to implement the trait since you don’t have to
specify the extra parameter most of the time. In other words, we’ve removed a
little bit of implementation boilerplate.

The first purpose is similar, but in reverse: since existing implementations of
a trait won’t have specified a type parameter, if we want to add a type
parameter to an existing trait, giving it a default will let us extend the
functionality of the trait without breaking the existing implementation code.

### Fully Qualified Syntax for Disambiguation

Rust cannot prevent a trait from having a method with the same name as another
trait’s method, nor can it prevent us from implementing both of these traits on
one type. We can also have a method implemented directly on the type with the
same name as well! In order to be able to call each of the methods with the
same name, then, we need to tell Rust which one we want to use.

Consider the code in Listing 19-27 where we’ve defined two traits, `Pilot` and
`Wizard`, that both have a method called `fly`. We then implement both traits
on a type `Human` that itself already has a method named `fly` implemented on
it. Each `fly` method does something different:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-27]: #Listing-19-27
<a name="Listing-19-27"></a>

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

<span class="caption">Listing 19-27: Two traits defined to have a `fly` method,
and implementations of those traits on the `Human` type in addition to a `fly`
method on `Human` directly</span>

When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in Listing 19-28:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-28]: #Listing-19-28
<a name="Listing-19-28"></a>

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

<span class="caption">Listing 19-28: Calling `fly` on an instance of
`Human`</span>

Running this will print out `*waving arms furiously*`, which shows that Rust
called the `fly` method implemented on `Human` directly.

In order to call the `fly` methods from either the `Pilot` trait or the
`Wizard` trait, we need to use more explicit syntax in order to specify which
`fly` method we mean. This syntax is demonstrated in Listing 19-29:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-29]: #Listing-19-29
<a name="Listing-19-29"></a>

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

<span class="caption">Listing 19-29: Specifying which trait’s `fly` method we
want to call</span>

Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also choose to write
`Human::fly(&person)`, which is equivalent to `person.fly()` that we had in
Listing 19-28, but is a bit longer to write if we don’t need to disambiguate.

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
example, take the `Animal` trait in Listing 19-30 that has the associated
function `baby_name`, the implementation of `Animal` for the struct `Dog`, and
the associated function `baby_name` defined on `Dog` directly:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-30]: #Listing-19-30
<a name="Listing-19-30"></a>

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

<span class="caption">Listing 19-30: A trait with an associated function and a
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

This isn’t really what we wanted, in this case we want to call the `baby_name`
function that’s part of the `Animal` trait that we implemented on `Dog`, so
that we can print `A baby dog is called a puppy`. The technique we used in
Listing 19-29 doesn’t help here; if we change `main` to be the code in Listing 19-31:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-31]: #Listing-19-31
<a name="Listing-19-31"></a>

```rust,ignore
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

<span class="caption">Listing 19-31: Attempting to call the `baby_name`
function from the `Animal` trait, but Rust doesn’t know which implementation to
use</span>

Because `Animal::baby_name` is an associated function rather than a method, and
thus doesn’t have a `self` parameter, Rust has no way to figure out which
implementation of `Animal::baby_name` we want. We’ll get this compiler error:

```text
error[E0283]: type annotations required: cannot resolve `_: Animal`
  --> src/main.rs
   |
20 |     println!("A baby dog is called a {}", Animal::baby_name());
   |                                           ^^^^^^^^^^^^^^^^^
   |
   = note: required by `Animal::baby_name`
```

In order to tell Rust that we want to use the implementation of `Animal` for
`Dog`, we need to use *fully qualified syntax*, which is the most specific we
can be when calling a function. Listing 19-32 demonstrates how to use fully
qualified syntax in this case:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-32]: #Listing-19-32
<a name="Listing-19-32"></a>

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

<span class="caption">Listing 19-32: Using fully qualified syntax to specify
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

### Supertraits to Use One Trait’s Functionality Within Another Trait

Sometimes, we may want a trait to be able to rely on another trait also being
implemented wherever our trait is implemented, so that our trait can use the
other trait’s functionality. The required trait is a *supertrait* of the trait
we’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print out a value outlined in asterisks. That
is, if our `Point` struct implements `Display` to result in `(x, y)`, calling
`outline_print` on a `Point` instance that has 1 for `x` and 3 for `y` would
look like:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, since we want to be able to use the
`Display` trait’s functionality, we need to be able to say that the
`OutlinePrint` trait will only work for types that also implement `Display` and
provide the functionality that `OutlinePrint` needs. We can do that in the
trait definition by specifying `OutlinePrint: Display`. It’s like adding a
trait bound to the trait. Listing 19-33 shows an implementation of the
`OutlinePrint` trait:

[Listing-19-33]: #Listing-19-33
<a name="Listing-19-33"></a>

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

<span class="caption">Listing 19-33: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use `to_string` in `outline_print` (`to_string` is automatically
implemented for any type that implements `Display`). If we hadn’t added the `:
Display` after the trait name and we tried to use `to_string` in
`outline_print`, we’d get an error that no method named `to_string` was found
for the type `&Self` in the current scope.

If we try to implement `OutlinePrint` on a type that doesn’t implement
`Display`, such as the `Point` struct:

```rust
# trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We’ll get an error that `Display` isn’t implemented and that `Display` is
required by `OutlinePrint`:

```text
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ the trait `std::fmt::Display` is not implemented for
   `Point`
   |
   = note: `Point` cannot be formatted with the default formatter; try using
   `:?` instead if you are using a format string
   = note: required by `OutlinePrint`
```

Once we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:

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

then implementing the `OutlinePrint` trait on `Point` will compile successfully
and we can call `outline_print` on a `Point` instance to display it within an
outline of asterisks.

### The Newtype Pattern to Implement External Traits on External Types

In Chapter 10, we mentioned the orphan rule, which says we’re allowed to
implement a trait on a type as long as either the trait or the type are local
to our crate. One way to get around this restriction is to use the *newtype
pattern*, which involves creating a new type using a tuple struct with one
field as a thin wrapper around the type we want to implement a trait for. Then
the wrapper type is local to our crate, and we can implement the trait on the
wrapper. “Newtype” is a term originating from the Haskell programming language.
There’s no runtime performance penalty for using this pattern. The wrapper type
is elided at compile time.

For example, if we wanted to implement `Display` on `Vec`, we can make a
`Wrapper` struct that holds an instance of `Vec`. Then we can implement
`Display` on `Wrapper` and use the `Vec` value as shown in Listing 19-34:

<span class="filename">Filename: src/main.rs</span>

[Listing-19-34]: #Listing-19-34
<a name="Listing-19-34"></a>

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

<span class="caption">Listing 19-34: Creating a `Wrapper` type around
`Vec<String>` to be able to implement `Display`</span>

The implementation of `Display` uses `self.0` to access the inner `Vec`, and
then we can use the functionality of the `Display` type on `Wrapper`.

The downside is that since `Wrapper` is a new type, it doesn’t have the methods
of the value it’s holding; we’d have to implement all the methods of `Vec` like
`push`, `pop`, and all the rest directly on `Wrapper` to delegate to `self.0`
in order to be able to treat `Wrapper` exactly like a `Vec`. If we wanted the
new type to have every single method that the inner type has, implementing the
`Deref` trait that we discussed in Chapter 15 on the wrapper to return the
inner type can be a solution. If we don’t want the wrapper type to have all the
methods of the inner type, in order to restrict the wrapper type’s behavior for
example, we’d have to implement just the methods we do want ourselves.

That’s how the newtype pattern is used in relation to traits; it’s also a
useful pattern without having traits involved. Let’s switch focus now to talk
about some advanced ways to interact with Rust’s type system.
