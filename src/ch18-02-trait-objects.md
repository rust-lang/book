## Using Trait Objects to Abstract over Shared Behavior

<!-- Old headings. Do not remove or links may break. -->

<a id="using-trait-objects-that-allow-for-values-of-different-types"></a>

In Chapter 8, we mentioned that one limitation of vectors is that they can
store elements of only one type. We created a workaround in Listing 8-9 where
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

At the time of writing the library, we can’t know and define all the types
other programmers might want to create. But we do know that `gui` needs to keep
track of many values of different types, and it needs to call a `draw` method
on each of these differently typed values. It doesn’t need to know exactly what
will happen when we call the `draw` method, just that the value will have that
method available for us to call.

To do this in a language with inheritance, we might define a class named
`Component` that has a method named `draw` on it. The other classes, such as
`Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus
inherit the `draw` method. They could each override the `draw` method to define
their custom behavior, but the framework could treat all of the types as if
they were `Component` instances and call `draw` on them. But because Rust
doesn’t have inheritance, we need another way to structure the `gui` library to
allow users to create new types compatible with the library.

### Defining a Trait for Common Behavior

To implement the behavior we want `gui` to have, we’ll define a trait named
`Draw` that will have one method named `draw`. Then we can define a vector that
takes a trait object. A _trait object_ points to both an instance of a type
implementing our specified trait and a table used to look up trait methods on
that type at runtime. We create a trait object by specifying some sort of
pointer, such as an `&` reference or a `Box<T>` smart pointer, then the `dyn`
keyword, and then specifying the relevant trait. (We’ll talk about the reason
trait objects must use a pointer in [“Dynamically Sized Types and the `Sized`
Trait”][dynamically-sized]<!-- ignore --> in Chapter 20.) We can use trait
objects in place of a generic or concrete type. Wherever we use a trait object,
Rust’s type system will ensure at compile time that any value used in that
context will implement the trait object’s trait. Consequently, we don’t need to
know all the possible types at compile time.

We’ve mentioned that, in Rust, we refrain from calling structs and enums
“objects” to distinguish them from other languages’ objects. In a struct or
enum, the data in the struct fields and the behavior in `impl` blocks are
separated, whereas in other languages, the data and behavior combined into one
concept is often labeled an object. Trait objects differ from objects in other
languages in that we can’t add data to a trait object. Trait objects aren’t as
generally useful as objects in other languages: their specific purpose is to
allow abstraction across common behavior.

Listing 18-3 shows how to define a trait named `Draw` with one method named
`draw`.

<Listing number="18-3" file-name="src/lib.rs" caption="Definition of the `Draw` trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

This syntax should look familiar from our discussions on how to define traits
in Chapter 10. Next comes some new syntax: Listing 18-4 defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<dyn Draw>`, which is a trait object; it’s a stand-in for any type inside a
`Box` that implements the `Draw` trait.

<Listing number="18-4" file-name="src/lib.rs" caption="Definition of the `Screen` struct with a `components` field holding a vector of trait objects that implement the `Draw` trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in Listing 18-5.

<Listing number="18-5" file-name="src/lib.rs" caption="A `run` method on `Screen` that calls the `draw` method on each component">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

This works differently from defining a struct that uses a generic type
parameter with trait bounds. A generic type parameter can be substituted with
only one concrete type at a time, whereas trait objects allow for multiple
concrete types to fill in for the trait object at runtime. For example, we
could have defined the `Screen` struct using a generic type and a trait bound,
as in Listing 18-6.

<Listing number="18-6" file-name="src/lib.rs" caption="An alternate implementation of the `Screen` struct and its `run` method using generics and trait bounds">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

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
might have fields for `width`, `height`, and `label`, as shown in Listing 18-7.

<Listing number="18-7" file-name="src/lib.rs" caption="A `Button` struct that implements the `Draw` trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

The `width`, `height`, and `label` fields on `Button` will differ from the
fields on other components; for example, a `TextField` type might have those
same fields plus a `placeholder` field. Each of the types we want to draw on
the screen will implement the `Draw` trait but will use different code in the
`draw` method to define how to draw that particular type, as `Button` has here
(without the actual GUI code, as mentioned). The `Button` type, for instance,
might have an additional `impl` block containing methods related to what
happens when a user clicks the button. These kinds of methods won’t apply to
types like `TextField`.

If someone using our library decides to implement a `SelectBox` struct that has
`width`, `height`, and `options` fields, they would implement the `Draw` trait
on the `SelectBox` type as well, as shown in Listing 18-8.

<Listing number="18-8" file-name="src/main.rs" caption="Another crate using `gui` and implementing the `Draw` trait on a `SelectBox` struct">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

Our library’s user can now write their `main` function to create a `Screen`
instance. To the `Screen` instance, they can add a `SelectBox` and a `Button`
by putting each in a `Box<T>` to become a trait object. They can then call the
`run` method on the `Screen` instance, which will call `draw` on each of the
components. Listing 18-9 shows this implementation.

<Listing number="18-9" file-name="src/main.rs" caption="Using trait objects to store values of different types that implement the same trait">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

When we wrote the library, we didn’t know that someone might add the
`SelectBox` type, but our `Screen` implementation was able to operate on the
new type and draw it because `SelectBox` implements the `Draw` trait, which
means it implements the `draw` method.

This concept—of being concerned only with the messages a value responds to
rather than the value’s concrete type—is similar to the concept of _duck
typing_ in dynamically typed languages: if it walks like a duck and quacks like
a duck, then it must be a duck! In the implementation of `run` on `Screen` in
Listing 18-5, `run` doesn’t need to know what the concrete type of each
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

For example, Listing 18-10 shows what happens if we try to create a `Screen`
with a `String` as a component.

<Listing number="18-10" file-name="src/main.rs" caption="Attempting to use a type that doesn’t implement the trait object’s trait">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

We’ll get this error because `String` doesn’t implement the `Draw` trait:

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

This error lets us know that either we’re passing something to `Screen` that we
didn’t mean to pass and so should pass a different type, or we should implement
`Draw` on `String` so that `Screen` is able to call `draw` on it.

### Trait Objects Perform Dynamic Dispatch

Recall in [“Performance of Code Using
Generics”][performance-of-code-using-generics]<!-- ignore --> in Chapter 10 our
discussion on the monomorphization process performed on generics by the
compiler: the compiler generates nongeneric implementations of functions and
methods for each concrete type that we use in place of a generic type
parameter. The code that results from monomorphization is doing _static
dispatch_, which is when the compiler knows what method you’re calling at
compile time. This is opposed to _dynamic dispatch_, which is when the compiler
can’t tell at compile time which method you’re calling. In dynamic dispatch
cases, the compiler emits code that at runtime will know which method to call.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t
know all the types that might be used with the code that’s using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at
runtime, Rust uses the pointers inside the trait object to know which method to
call. This lookup incurs a runtime cost that doesn’t occur with static dispatch.
Dynamic dispatch also prevents the compiler from choosing to inline a method’s
code, which in turn prevents some optimizations, and Rust has some rules about
where you can and cannot use dynamic dispatch, called _dyn compatibility_. Those
rules are beyond the scope of this discussion, but you can read more about them
[in the reference][dyn-compatibility]<!-- ignore -->. However, we did get extra
flexibility in the code that we wrote in Listing 18-5 and were able to support
in Listing 18-9, so it’s a trade-off to consider.

[performance-of-code-using-generics]: ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
