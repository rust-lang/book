## Using Trait Objects that Allow for Values of Different Types

In Chapter 8, we mentioned that one limitation of vectors is that they can only
store elements of one type. We created a workaround in [Listing 8-10][Listing-8-10] where we
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

[Listing 17-3][Listing-17-3] shows how to define a trait named `Draw` with one method named
`draw`:

<span class="filename">Filename: src/lib.rs</span>

[Listing-17-3]: #Listing-17-3
<a name="Listing-17-3"></a>

```rust
pub trait Draw {
    fn draw(&self);
}
```

<span class="caption">Listing 17-3: Definition of the `Draw` trait</span>

This should look familiar from our discussions on how to define traits in
Chapter 10. Next comes something new: [Listing 17-4][Listing-17-4] defines a struct named
`Screen` that holds a vector named `components`. This vector is of type
`Box<Draw>`, which is a trait object: it’s a stand-in for any type inside a
`Box` that implements the `Draw` trait.

<!-- Would it be useful to let the reader know why we need a box here, or will
that be clear at this point? -->
<!-- We get into this in chapter 19; I've added a reference to the start of
this section where we talk about needing a `&` or a `Box` to be a trait object.
/Carol -->

<span class="filename">Filename: src/lib.rs</span>

[Listing-17-4]: #Listing-17-4
<a name="Listing-17-4"></a>

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
`components` field holding a vector of trait objects that implement the `Draw`
trait</span>

On the `Screen` struct, we’ll define a method named `run` that will call the
`draw` method on each of its `components`, as shown in [Listing 17-5][Listing-17-5]:

<span class="filename">Filename: src/lib.rs</span>

[Listing-17-5]: #Listing-17-5
<a name="Listing-17-5"></a>

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

This works differently to defining a struct that uses a generic type parameter
with trait bounds. A generic type parameter can only be substituted with one
concrete type at a time, while trait objects allow for multiple concrete types
to fill in for the trait object at runtime. For example, we could have defined
the `Screen` struct using a generic type and a trait bound as in [Listing 17-6][Listing-17-6]:

<span class="filename">Filename: src/lib.rs</span>

[Listing-17-6]: #Listing-17-6
<a name="Listing-17-6"></a>

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
[Listing 17-7][Listing-17-7]:

<span class="filename">Filename: src/lib.rs</span>

[Listing-17-7]: #Listing-17-7
<a name="Listing-17-7"></a>

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
the `SelectBox` type as well, as shown in [Listing 17-8][Listing-17-8]:

<span class="filename">Filename: src/main.rs</span>

[Listing-17-8]: #Listing-17-8
<a name="Listing-17-8"></a>

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
`Screen` instance. To this they can add a `SelectBox` and a `Button` by putting
each in a `Box<T>` to become a trait object. They can then call the `run`
method on the `Screen` instance, which will call `draw` on each of the
components. [Listing 17-9][Listing-17-9] shows this implementation:

<span class="filename">Filename: src/main.rs</span>

[Listing-17-9]: #Listing-17-9
<a name="Listing-17-9"></a>

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

When we wrote the library, we didn’t know that someone would add the
`SelectBox` type someday, but our `Screen` implementation was able to operate
on the new type and draw it because `SelectBox` implements the `Draw` type,
which means it implements the `draw` method.

This concept---of being concerned only with the messages a value responds to,
rather than the value’s concrete type---is similar to a concept in dynamically
typed languages called *duck typing*: if it walks like a duck, and quacks like
a duck, then it must be a duck! In the implementation of `run` on `Screen` in
[Listing 17-5][Listing-17-5], `run` doesn’t need to know what the concrete type of each
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

For example, [Listing 17-10][Listing-17-10] shows what happens if we try to create a `Screen`
with a `String` as a component:

<span class="filename">Filename: src/main.rs</span>

[Listing-17-10]: #Listing-17-10
<a name="Listing-17-10"></a>

```rust,ignore
extern crate rust_gui;
use rust_gui::Screen;

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

We’ll get this error because `String` doesn’t implement the `rust_gui::Draw` trait:

```text
error[E0277]: the trait bound `std::string::String: rust_gui::Draw` is not satisfied
  -->
   |
 4 |             Box::new(String::from("Hi")),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rust_gui::Draw` is not
   implemented for `std::string::String`
   |
   = note: required for the cast to the object type `rust_gui::Draw`
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
<!-- Ok, I've cut section way down to the practical pieces, but still explained
a little bit /Carol -->

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

The compiler will tell you if you’re trying to do something that violates the
rules of object safety in regards to trait objects. For example, if we had
tried to implement the `Screen` struct in [Listing 17-4][Listing-17-4] to hold types that
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

This means you can’t use this trait as a trait object in this way. If you’re
interested in more details on object safety, see [Rust RFC 255].

[Rust RFC 255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md

[Listing-8-10]: ch08-01-vectors.html#Listing-8-10
[Listing-17-3]: ch17-02-trait-objects.html#Listing-17-3
[Listing-17-4]: ch17-02-trait-objects.html#Listing-17-4
[Listing-17-5]: ch17-02-trait-objects.html#Listing-17-5
[Listing-17-6]: ch17-02-trait-objects.html#Listing-17-6
[Listing-17-7]: ch17-02-trait-objects.html#Listing-17-7
[Listing-17-8]: ch17-02-trait-objects.html#Listing-17-8
[Listing-17-9]: ch17-02-trait-objects.html#Listing-17-9
[Listing-17-10]: ch17-02-trait-objects.html#Listing-17-10
