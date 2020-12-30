## Using Trait Objects That Allow for Values of Different Types

In Chapter 8, we mentioned that one limitation of vectors is that they can
store elements of only one type. We created a workaround in Listing 8-10 where
we defined a `SpreadsheetCell` enum that had variants to hold integers, floats,
and text. This meant we could store different types of data in each cell and
still have a vector that represented a row of cells. This is a perfectly good
solution when our interchangeable items are a fixed set of types that we know
when our code is compiled.

However, sometimes we want our library user to be able to extend the set of
types that are valid in a particular situation. To show how we might achieve
this, we’ll create an example graphical user interface (GUI) tool that iterates
through a list of items, calling a `draw` method on each one to draw it to the
screen—a common technique for GUI tools. We’ll create a library crate called
`gui` that contains the structure of a GUI library. This crate might include
some types for people to use, such as `Button` or `TextField`. In addition,
`gui` users will want to create their own types that can be drawn: for
instance, one programmer might add an `Image` and another might add a
`SelectBox`.

We won’t implement a fully fledged GUI library for this example but will show
how the pieces would fit together. At the time of writing the library, we can’t
know and define all the types other programmers might want to create. But we do
know that `gui` needs to keep track of many values of different types, and it
needs to call a `draw` method on each of these differently typed values. It
doesn’t need to know exactly what will happen when we call the `draw` method,
just that the value will have that method available for us to call.

To do this in a language with inheritance, we might define a class named
`Component` that has a method named `draw` on it. The other classes, such as
`Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus
inherit the `draw` method. They could each override the `draw` method to define
their custom behavior, but the framework could treat all of the types as if
they were `Component` instances and call `draw` on them. But because Rust
doesn’t have inheritance, we need another way to structure the `gui` library to
allow users to extend it with new types.

### Defining a Trait for Common Behavior

To implement the behavior we want `gui` to have, we’ll define a trait named
`Draw` that will have one method named `draw`. Then we can define a vector that
takes a *trait object*. A trait object points to both an instance of a type
implementing our specified trait as well as a table used to look up trait
methods on that type at runtime. We create a trait object by specifying some
sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the
`dyn` keyword, and then specifying the relevant trait. (We’ll talk about the
reason trait objects must use a pointer in Chapter 19 in the section
[“Dynamically Sized Types and the `Sized` Trait.”][dynamically-sized]<!--
ignore -->) We can use trait objects in place of a generic or concrete type.
Wherever we use a trait object, Rust’s type system will ensure at compile time
that any value used in that context will implement the trait object’s trait.
Consequently, we don’t need to know all the possible types at compile time.

We’ve mentioned that in Rust, we refrain from calling structs and enums
“objects” to distinguish them from other languages’ objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks are
separated, whereas in other languages, the data and behavior combined into one
concept is often labeled an object. However, trait objects *are* more like
objects in other languages in the sense that they combine data and behavior.
But trait objects differ from traditional objects in that we can’t add data to
a trait object. Trait objects aren’t as generally useful as objects in other
languages: their specific purpose is to allow abstraction across common
behavior.

Listing 17-3 shows how to define a trait named `Draw` with one method named
`draw`:

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-03/src/lib.rs}}
```

<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>

This syntax should look familiar from our discussions on how to define traits
in Chapter 10. Next comes some new syntax: Listing 17-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<dyn Draw>`, which is a trait object; it’s a stand-in for any type inside
a `Box` that implements the `Draw` trait.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-04/src/lib.rs:here}}
```

<span class="caption">Listing 17-4: Definition of the `Screen` struct with a
`components` field holding a vector of trait objects that implement the `Draw`
trait</span>

On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 17-5:

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-05/src/lib.rs:here}}
```

<span class="caption">Listing 17-5: A `run` method on `Screen` that calls the
`draw` method on each component</span>

This works differently from defining a struct that uses a generic type
parameter with trait bounds. A generic type parameter can only be substituted
with one concrete type at a time, whereas trait objects allow for multiple
concrete types to fill in for the trait object at runtime. For example, we
could have defined the `Screen` struct using a generic type and a trait bound
as in Listing 17-6:

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch17-oop/listing-17-06/src/lib.rs:here}}
```

<span class="caption">Listing 17-6: An alternate implementation of the `Screen`
struct and its `run` method using generics and trait bounds</span>

This restricts us to a `Screen` instance that has a list of components all of
type `Button` or all of type `TextField`. If you’ll only ever have homogeneous
collections, using generics and trait bounds is preferable because the
definitions will be monomorphized at compile time to use the concrete types.

On the other hand, with the method using trait objects, one `Screen` instance
can hold a `Vec<T>` that contains a `Box<Button>` as well as a
`Box<TextField>`. Let’s look at how this works, and then we’ll talk about the
runtime performance implications.

### Implementing the Trait

Now we’ll add some types that implement the `Draw` trait. We’ll provide the
`Button` type. Again, actually implementing a GUI library is beyond the scope
of this book, so the `draw` method won’t have any useful implementation in its
body. To imagine what the implementation might look like, a `Button` struct
might have fields for `width`, `height`, and `label`, as shown in Listing 17-7:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-07/src/lib.rs:here}}
```

<span class="caption">Listing 17-7: A `Button` struct that implements the
`Draw` trait</span>

The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components, such as a `TextField` type, that might have those
fields plus a `placeholder` field instead. Each of the types we want to draw on
the screen will implement the `Draw` trait but will use different code in the
`draw` method to define how to draw that particular type, as `Button` has here
(without the actual GUI code, which is beyond the scope of this chapter). The
`Button` type, for instance, might have an additional `impl` block containing
methods related to what happens when a user clicks the button. These kinds of
methods won’t apply to types like `TextField`.

If someone using our library decides to implement a `SelectBox` struct that has
`width`, `height`, and `options` fields, they implement the `Draw` trait on the
`SelectBox` type as well, as shown in Listing 17-8:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-08/src/main.rs:here}}
```

<span class="caption">Listing 17-8: Another crate using `gui` and implementing
the `Draw` trait on a `SelectBox` struct</span>

Our library’s user can now write their `main` function to create a `Screen`
instance. To the `Screen` instance, they can add a `SelectBox` and a `Button`
by putting each in a `Box<T>` to become a trait object. They can then call the
`run` method on the `Screen` instance, which will call `draw` on each of the
components. Listing 17-9 shows this implementation:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch17-oop/listing-17-09/src/main.rs:here}}
```

<span class="caption">Listing 17-9: Using trait objects to store values of
different types that implement the same trait</span>

When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.

This concept—of being concerned only with the messages a value responds to
rather than the value’s concrete type—is similar to the concept of *duck
typing* in dynamically typed languages: if it walks like a duck and quacks
like a duck, then it must be a duck! In the implementation of `run` on `Screen`
in Listing 17-5, `run` doesn’t need to know what the concrete type of each
component is. It doesn’t check whether a component is an instance of a `Button`
or a `SelectBox`, it just calls the `draw` method on the component. By
specifying `Box<dyn Draw>` as the type of the values in the `components`
vector, we’ve defined `Screen` to need values that we can call the `draw`
method on.

The advantage of using trait objects and Rust’s type system to write code
similar to code using duck typing is that we never have to check whether a
value implements a particular method at runtime or worry about getting errors
if a value doesn’t implement a method but we call it anyway. Rust won’t compile
our code if the values don’t implement the traits that the trait objects need.

For example, Listing 17-10 shows what happens if we try to create a `Screen`
with a `String` as a component:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/listing-17-10/src/main.rs}}
```

<span class="caption">Listing 17-10: Attempting to use a type that doesn’t
implement the trait object’s trait</span>

We’ll get this error because `String` doesn’t implement the `Draw` trait:

```console
{{#include ../listings/ch17-oop/listing-17-10/output.txt}}
```

This error lets us know that either we’re passing something to `Screen` we
didn’t mean to pass and we should pass a different type or we should implement
`Draw` on `String` so that `Screen` is able to call `draw` on it.

### Trait Objects Perform Dynamic Dispatch

Recall in the [“Performance of Code Using
Generics”][performance-of-code-using-generics]<!-- ignore --> section in
Chapter 10 our discussion on the monomorphization process performed by the
compiler when we use trait bounds on generics: the compiler generates
nongeneric implementations of functions and methods for each concrete type
that we use in place of a generic type parameter. The code that results from
monomorphization is doing *static dispatch*, which is when the compiler knows
what method you’re calling at compile time. This is opposed to *dynamic
dispatch*, which is when the compiler can’t tell at compile time which method
you’re calling. In dynamic dispatch cases, the compiler emits code that at
runtime will figure out which method to call.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t
know all the types that might be used with the code that is using trait
objects, so it doesn’t know which method implemented on which type to call.
Instead, at runtime, Rust uses the pointers inside the trait object to know
which method to call. There is a runtime cost when this lookup happens that
doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler
from choosing to inline a method’s code, which in turn prevents some
optimizations. However, we did get extra flexibility in the code that we wrote
in Listing 17-5 and were able to support in Listing 17-9, so it’s a trade-off
to consider.

### Object Safety Is Required for Trait Objects

You can only make *object-safe* traits into trait objects. Some complex rules
govern all the properties that make a trait object safe, but in practice, only
two rules are relevant. A trait is object safe if all the methods defined in
the trait have the following properties:

* The return type isn’t `Self`.
* There are no generic type parameters.

The `Self` keyword is an alias for the type we’re implementing the traits or
methods on. Trait objects must be object safe because once you’ve used a trait
object, Rust no longer knows the concrete type that’s implementing that trait.
If a trait method returns the concrete `Self` type, but a trait object forgets
the exact type that `Self` is, there is no way the method can use the original
concrete type. The same is true of generic type parameters that are filled in
with concrete type parameters when the trait is used: the concrete types become
part of the type that implements the trait. When the type is forgotten through
the use of a trait object, there is no way to know what types to fill in the
generic type parameters with.

An example of a trait whose methods are not object safe is the standard
library’s `Clone` trait. The signature for the `clone` method in the `Clone`
trait looks like this:

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

The `String` type implements the `Clone` trait, and when we call the `clone`
method on an instance of `String` we get back an instance of `String`.
Similarly, if we call `clone` on an instance of `Vec<T>`, we get back an
instance of `Vec<T>`. The signature of `clone` needs to know what type will
stand in for `Self`, because that’s the return type.

The compiler will indicate when you’re trying to do something that violates the
rules of object safety in regard to trait objects. For example, let’s say we
tried to implement the `Screen` struct in Listing 17-4 to hold types that
implement the `Clone` trait instead of the `Draw` trait, like this:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-oop/no-listing-01-trait-object-of-clone/src/lib.rs}}
```

We would get this error:

```console
{{#include ../listings/ch17-oop/no-listing-01-trait-object-of-clone/output.txt}}
```

This error means you can’t use this trait as a trait object in this way. If
you’re interested in more details on object safety, see [Rust RFC 255].

[Rust RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md

[performance-of-code-using-generics]:
ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
