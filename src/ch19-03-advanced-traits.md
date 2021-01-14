## Advanced Traits

We first covered traits in the [“Traits: Defining Shared
Behavior”][traits-defining-shared-behavior]<!-- ignore --> section of Chapter
10, but as with lifetimes, we didn’t discuss the more advanced details. Now
that you know more about Rust, we can get into the nitty-gritty.

### Specifying Placeholder Types in Trait Definitions with Associated Types

*Associated types* connect a type placeholder with a trait such that the trait
method definitions can use these placeholder types in their signatures. The
implementor of a trait will specify the concrete type to be used in this type’s
place for the particular implementation. That way, we can define a trait that
uses some types without needing to know exactly what those types are until the
trait is implemented.

We’ve described most of the advanced features in this chapter as being rarely
needed. Associated types are somewhere in the middle: they’re used more rarely
than features explained in the rest of the book but more commonly than many of
the other features discussed in this chapter.

One example of a trait with an associated type is the `Iterator` trait that the
standard library provides. The associated type is named `Item` and stands in
for the type of the values the type implementing the `Iterator` trait is
iterating over. In [“The `Iterator` Trait and the `next`
Method”][the-iterator-trait-and-the-next-method]<!-- ignore --> section of
Chapter 13, we mentioned that the definition of the `Iterator` trait is as
shown in Listing 19-12.

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-12/src/lib.rs}}
```

<span class="caption">Listing 19-12: The definition of the `Iterator` trait
that has an associated type `Item`</span>

The type `Item` is a placeholder type, and the `next` method’s definition shows
that it will return values of type `Option<Self::Item>`. Implementors of the
`Iterator` trait will specify the concrete type for `Item`, and the `next`
method will return an `Option` containing a value of that concrete type.

Associated types might seem like a similar concept to generics, in that the
latter allow us to define a function without specifying what types it can
handle. So why use associated types?

Let’s examine the difference between the two concepts with an example from
Chapter 13 that implements the `Iterator` trait on the `Counter` struct. In
Listing 13-21, we specified that the `Item` type was `u32`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-13-21-reproduced/src/lib.rs:ch19}}
```

This syntax seems comparable to that of generics. So why not just define the
`Iterator` trait with generics, as shown in Listing 19-13?

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-13/src/lib.rs}}
```

<span class="caption">Listing 19-13: A hypothetical definition of the
`Iterator` trait using generics</span>

The difference is that when using generics, as in Listing 19-13, we must
annotate the types in each implementation; because we can also implement
`Iterator<String> for Counter` or any other type, we could have multiple
implementations of `Iterator` for `Counter`. In other words, when a trait has a
generic parameter, it can be implemented for a type multiple times, changing
the concrete types of the generic type parameters each time. When we use the
`next` method on `Counter`, we would have to provide type annotations to
indicate which implementation of `Iterator` we want to use.

With associated types, we don’t need to annotate types because we can’t
implement a trait on a type multiple times. In Listing 19-12 with the
definition that uses associated types, we can only choose what the type of
`Item` will be once, because there can only be one `impl Iterator for Counter`.
We don’t have to specify that we want an iterator of `u32` values everywhere
that we call `next` on `Counter`.

### Default Generic Type Parameters and Operator Overloading

When we use generic type parameters, we can specify a default concrete type for
the generic type. This eliminates the need for implementors of the trait to
specify a concrete type if the default type works. The syntax for specifying a
default type for a generic type is `<PlaceholderType=ConcreteType>` when
declaring the generic type.

A great example of a situation where this technique is useful is with operator
overloading. *Operator overloading* is customizing the behavior of an operator
(such as `+`) in particular situations.

Rust doesn’t allow you to create your own operators or overload arbitrary
operators. But you can overload the operations and corresponding traits listed
in `std::ops` by implementing the traits associated with the operator. For
example, in Listing 19-14 we overload the `+` operator to add two `Point`
instances together. We do this by implementing the `Add` trait on a `Point`
struct:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-14/src/main.rs}}
```

<span class="caption">Listing 19-14: Implementing the `Add` trait to overload
the `+` operator for `Point` instances</span>

The `add` method adds the `x` values of two `Point` instances and the `y`
values of two `Point` instances to create a new `Point`. The `Add` trait has an
associated type named `Output` that determines the type returned from the `add`
method.

The default generic type in this code is within the `Add` trait. Here is its
definition:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

This code should look generally familiar: a trait with one method and an
associated type. The new part is `Rhs=Self`: this syntax is called *default
type parameters*. The `Rhs` generic type parameter (short for “right hand
side”) defines the type of the `rhs` parameter in the `add` method. If we don’t
specify a concrete type for `Rhs` when we implement the `Add` trait, the type
of `Rhs` will default to `Self`, which will be the type we’re implementing
`Add` on.

When we implemented `Add` for `Point`, we used the default for `Rhs` because we
wanted to add two `Point` instances. Let’s look at an example of implementing
the `Add` trait where we want to customize the `Rhs` type rather than using the
default.

We have two structs, `Millimeters` and `Meters`, holding values in different
units. We want to add values in millimeters to values in meters and have the
implementation of `Add` do the conversion correctly. We can implement `Add` for
`Millimeters` with `Meters` as the `Rhs`, as shown in Listing 19-15.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-15/src/lib.rs}}
```

<span class="caption">Listing 19-15: Implementing the `Add` trait on
`Millimeters` to add `Millimeters` to `Meters`</span>

To add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the
value of the `Rhs` type parameter instead of using the default of `Self`.

You’ll use default type parameters in two main ways:

* To extend a type without breaking existing code
* To allow customization in specific cases most users won’t need

The standard library’s `Add` trait is an example of the second purpose:
usually, you’ll add two like types, but the `Add` trait provides the ability to
customize beyond that. Using a default type parameter in the `Add` trait
definition means you don’t have to specify the extra parameter most of the
time. In other words, a bit of implementation boilerplate isn’t needed, making
it easier to use the trait.

The first purpose is similar to the second but in reverse: if you want to add a
type parameter to an existing trait, you can give it a default to allow
extension of the functionality of the trait without breaking the existing
implementation code.

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as
another trait’s method, nor does Rust prevent you from implementing both traits
on one type. It’s also possible to implement a method directly on the type with
the same name as methods from traits.

When calling methods with the same name, you’ll need to tell Rust which one you
want to use. Consider the code in Listing 19-16 where we’ve defined two traits,
`Pilot` and `Wizard`, that both have a method called `fly`. We then implement
both traits on a type `Human` that already has a method named `fly` implemented
on it. Each `fly` method does something different.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```

<span class="caption">Listing 19-16: Two traits are defined to have a `fly`
method and are implemented on the `Human` type, and a `fly` method is
implemented on `Human` directly</span>

When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type, as shown in Listing 19-17.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```

<span class="caption">Listing 19-17: Calling `fly` on an instance of
`Human`</span>

Running this code will print `*waving arms furiously*`, showing that Rust
called the `fly` method implemented on `Human` directly.

To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait,
we need to use more explicit syntax to specify which `fly` method we mean.
Listing 19-18 demonstrates this syntax.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```

<span class="caption">Listing 19-18: Specifying which trait’s `fly` method we
want to call</span>

Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call. We could also write
`Human::fly(&person)`, which is equivalent to the `person.fly()` that we used
in Listing 19-18, but this is a bit longer to write if we don’t need to
disambiguate.

Running this code prints the following:

```console
{{#include ../listings/ch19-advanced-features/listing-19-18/output.txt}}
```

Because the `fly` method takes a `self` parameter, if we had two *types* that
both implement one *trait*, Rust could figure out which implementation of a
trait to use based on the type of `self`.

However, associated functions that are part of traits don’t have a `self`
parameter. When two types in the same scope implement that trait, Rust can’t
figure out which type you mean unless you use *fully qualified syntax*. For
example, the `Animal` trait in Listing 19-19 has the associated function
`baby_name`, the implementation of `Animal` for the struct `Dog`, and the
associated function `baby_name` defined on `Dog` directly.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-19/src/main.rs}}
```

<span class="caption">Listing 19-19: A trait with an associated function and a
type with an associated function of the same name that also implements the
trait</span>

This code is for an animal shelter that wants to name all puppies Spot, which
is implemented in the `baby_name` associated function that is defined on `Dog`.
The `Dog` type also implements the trait `Animal`, which describes
characteristics that all animals have. Baby dogs are called puppies, and that
is expressed in the implementation of the `Animal` trait on `Dog` in the
`baby_name` function associated with the `Animal` trait.

In `main`, we call the `Dog::baby_name` function, which calls the associated
function defined on `Dog` directly. This code prints the following:

```console
{{#include ../listings/ch19-advanced-features/listing-19-19/output.txt}}
```

This output isn’t what we wanted. We want to call the `baby_name` function that
is part of the `Animal` trait that we implemented on `Dog` so the code prints
`A baby dog is called a puppy`. The technique of specifying the trait name that
we used in Listing 19-18 doesn’t help here; if we change `main` to the code in
Listing 19-20, we’ll get a compilation error.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```

<span class="caption">Listing 19-20: Attempting to call the `baby_name`
function from the `Animal` trait, but Rust doesn’t know which implementation to
use</span>

Because `Animal::baby_name` is an associated function rather than a method, and
thus doesn’t have a `self` parameter, Rust can’t figure out which
implementation of `Animal::baby_name` we want. We’ll get this compiler error:

```console
{{#include ../listings/ch19-advanced-features/listing-19-20/output.txt}}
```

To disambiguate and tell Rust that we want to use the implementation of
`Animal` for `Dog`, we need to use fully qualified syntax. Listing 19-21
demonstrates how to use fully qualified syntax.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```

<span class="caption">Listing 19-21: Using fully qualified syntax to specify
that we want to call the `baby_name` function from the `Animal` trait as
implemented on `Dog`</span>

We’re providing Rust with a type annotation within the angle brackets, which
indicates we want to call the `baby_name` method from the `Animal` trait as
implemented on `Dog` by saying that we want to treat the `Dog` type as an
`Animal` for this function call. This code will now print what we want:

```console
{{#include ../listings/ch19-advanced-features/listing-19-21/output.txt}}
```

In general, fully qualified syntax is defined as follows:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

For associated functions, there would not be a `receiver`: there would only be
the list of other arguments. You could use fully qualified syntax everywhere
that you call functions or methods. However, you’re allowed to omit any part of
this syntax that Rust can figure out from other information in the program. You
only need to use this more verbose syntax in cases where there are multiple
implementations that use the same name and Rust needs help to identify which
implementation you want to call.

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

Sometimes, you might need one trait to use another trait’s functionality. In
this case, you need to rely on the dependent trait also being implemented.
The trait you rely on is a *supertrait* of the trait you’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print a value framed in asterisks. That is,
given a `Point` struct that implements `Display` to result in `(x, y)`, when we
call `outline_print` on a `Point` instance that has `1` for `x` and `3` for
`y`, it should print the following:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, we want to use the `Display` trait’s
functionality. Therefore, we need to specify that the `OutlinePrint` trait will
work only for types that also implement `Display` and provide the functionality
that `OutlinePrint` needs. We can do that in the trait definition by specifying
`OutlinePrint: Display`. This technique is similar to adding a trait bound to
the trait. Listing 19-22 shows an implementation of the `OutlinePrint` trait.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```

<span class="caption">Listing 19-22: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that is automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding a
colon and specifying the `Display` trait after the trait name, we’d get an
error saying that no method named `to_string` was found for the type `&Self` in
the current scope.

Let’s see what happens when we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

We get an error saying that `Display` is required but not implemented:

```console
{{#include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

To fix this, we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

Then implementing the `OutlinePrint` trait on `Point` will compile
successfully, and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.

### Using the Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the [“Implementing a Trait on a
Type”][implementing-a-trait-on-a-type]<!-- ignore --> section, we mentioned
the orphan rule that states we’re allowed to implement a trait on a type as
long as either the trait or the type are local to our crate. It’s possible to
get around this restriction using the *newtype pattern*, which involves
creating a new type in a tuple struct. (We covered tuple structs in the
[“Using Tuple Structs without Named Fields to Create Different
Types”][tuple-structs]<!-- ignore --> section of Chapter 5.) The tuple struct
will have one field and be a thin wrapper around the type we want to implement
a trait for. Then the wrapper type is local to our crate, and we can implement
the trait on the wrapper. *Newtype* is a term that originates from the Haskell
programming language. There is no runtime performance penalty for using this
pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement `Display` on `Vec<T>`, which the
orphan rule prevents us from doing directly because the `Display` trait and the
`Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct
that holds an instance of `Vec<T>`; then we can implement `Display` on
`Wrapper` and use the `Vec<T>` value, as shown in Listing 19-23.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-23/src/main.rs}}
```

<span class="caption">Listing 19-23: Creating a `Wrapper` type around
`Vec<String>` to implement `Display`</span>

The implementation of `Display` uses `self.0` to access the inner `Vec<T>`,
because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the
tuple. Then we can use the functionality of the `Display` type on `Wrapper`.

The downside of using this technique is that `Wrapper` is a new type, so it
doesn’t have the methods of the value it’s holding. We would have to implement
all the methods of `Vec<T>` directly on `Wrapper` such that the methods
delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a
`Vec<T>`. If we wanted the new type to have every method the inner type has,
implementing the `Deref` trait (discussed in Chapter 15 in the [“Treating Smart
Pointers Like Regular References with the `Deref`
Trait”][smart-pointer-deref]<!-- ignore --> section) on the `Wrapper` to return
the inner type would be a solution. If we don’t want the `Wrapper` type to have
all the methods of the inner type—for example, to restrict the `Wrapper` type’s
behavior—we would have to implement just the methods we do want manually.

Now you know how the newtype pattern is used in relation to traits; it’s also a
useful pattern even when traits are not involved. Let’s switch focus and look
at some advanced ways to interact with Rust’s type system.

[implementing-a-trait-on-a-type]:
ch10-02-traits.html#implementing-a-trait-on-a-type
[the-iterator-trait-and-the-next-method]:
ch13-02-iterators.html#the-iterator-trait-and-the-next-method
[traits-defining-shared-behavior]:
ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
