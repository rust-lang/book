## Characteristics of Object-Oriented Languages

There is no consensus in the programming community about what features a
language must have to be considered object oriented. Rust is influenced by many
programming paradigms, including OOP; for example, we explored the features
that came from functional programming in Chapter 13. Arguably, OOP languages
share certain common characteristics, namely objects, encapsulation, and
inheritance. Let’s look at what each of those characteristics means and whether
Rust supports it.

### Objects Contain Data and Behavior

The book *Design Patterns: Elements of Reusable Object-Oriented Software* by
Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides (Addison-Wesley
Professional, 1994), colloquially referred to as *The Gang of Four* book, is a
catalog of object-oriented design patterns. It defines OOP this way:

> Object-oriented programs are made up of objects. An *object* packages both
> data and the procedures that operate on that data. The procedures are
> typically called *methods* or *operations*.

Using this definition, Rust is object oriented: structs and enums have data,
and `impl` blocks provide methods on structs and enums. Even though structs and
enums with methods aren’t *called* objects, they provide the same
functionality, according to the Gang of Four’s definition of objects.

### Encapsulation that Hides Implementation Details

Another aspect commonly associated with OOP is the idea of *encapsulation*,
which means that the implementation details of an object aren’t accessible to
code using that object. Therefore, the only way to interact with an object is
through its public API; code using the object shouldn’t be able to reach into
the object’s internals and change data or behavior directly. This enables the
programmer to change and refactor an object’s internals without needing to
change the code that uses the object.

We discussed how to control encapsulation in Chapter 7: we can use the `pub`
keyword to decide which modules, types, functions, and methods in our code
should be public, and by default everything else is private. For example, we
can define a struct `AveragedCollection` that has a field containing a vector
of `i32` values. The struct can also have a field that contains the average of
the values in the vector, meaning the average doesn’t have to be computed
on demand whenever anyone needs it. In other words, `AveragedCollection` will
cache the calculated average for us. Listing 17-1 has the definition of the
`AveragedCollection` struct:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-01/src/lib.rs}}
```

<span class="caption">Listing 17-1: An `AveragedCollection` struct that
maintains a list of integers and the average of the items in the
collection</span>

The struct is marked `pub` so that other code can use it, but the fields within
the struct remain private. This is important in this case because we want to
ensure that whenever a value is added or removed from the list, the average is
also updated. We do this by implementing `add`, `remove`, and `average` methods
on the struct, as shown in Listing 17-2:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch17-oop/listing-17-02/src/lib.rs:here}}
```

<span class="caption">Listing 17-2: Implementations of the public methods
`add`, `remove`, and `average` on `AveragedCollection`</span>

The public methods `add`, `remove`, and `average` are the only ways to access
or modify data in an instance of `AveragedCollection`. When an item is added
to `list` using the `add` method or removed using the `remove` method, the
implementations of each call the private `update_average` method that handles
updating the `average` field as well.

We leave the `list` and `average` fields private so there is no way for
external code to add or remove items to the `list` field directly; otherwise,
the `average` field might become out of sync when the `list` changes. The
`average` method returns the value in the `average` field, allowing external
code to read the `average` but not modify it.

Because we’ve encapsulated the implementation details of the struct
`AveragedCollection`, we can easily change aspects, such as the data structure,
in the future. For instance, we could use a `HashSet<i32>` instead of a
`Vec<i32>` for the `list` field. As long as the signatures of the `add`,
`remove`, and `average` public methods stay the same, code using
`AveragedCollection` wouldn’t need to change. If we made `list` public instead,
this wouldn’t necessarily be the case: `HashSet<i32>` and `Vec<i32>` have
different methods for adding and removing items, so the external code would
likely have to change if it were modifying `list` directly.

If encapsulation is a required aspect for a language to be considered object
oriented, then Rust meets that requirement. The option to use `pub` or not for
different parts of code enables encapsulation of implementation details.

### Inheritance as a Type System and as Code Sharing

*Inheritance* is a mechanism whereby an object can inherit from another
object’s definition, thus gaining the parent object’s data and behavior without
you having to define them again.

If a language must have inheritance to be an object-oriented language, then
Rust is not one. There is no way to define a struct that inherits the parent
struct’s fields and method implementations. However, if you’re used to having
inheritance in your programming toolbox, you can use other solutions in Rust,
depending on your reason for reaching for inheritance in the first place.

You choose inheritance for two main reasons. One is for reuse of code: you can
implement particular behavior for one type, and inheritance enables you to
reuse that implementation for a different type. You can share Rust code using
default trait method implementations instead, which you saw in Listing 10-14
when we added a default implementation of the `summarize` method on the
`Summary` trait. Any type implementing the `Summary` trait would have the
`summarize` method available on it without any further code. This is similar to
a parent class having an implementation of a method and an inheriting child
class also having the implementation of the method. We can also override the
default implementation of the `summarize` method when we implement the
`Summary` trait, which is similar to a child class overriding the
implementation of a method inherited from a parent class.

The other reason to use inheritance relates to the type system: to enable a
child type to be used in the same places as the parent type. This is also
called *polymorphism*, which means that you can substitute multiple objects for
each other at runtime if they share certain characteristics.

> ### Polymorphism
>
> To many people, polymorphism is synonymous with inheritance. But it’s
> actually a more general concept that refers to code that can work with data
> of multiple types. For inheritance, those types are generally subclasses.
>
> Rust instead uses generics to abstract over different possible types and
> trait bounds to impose constraints on what those types must provide. This is
> sometimes called *bounded parametric polymorphism*.

Inheritance has recently fallen out of favor as a programming design solution
in many programming languages because it’s often at risk of sharing more code
than necessary. Subclasses shouldn’t always share all characteristics of their
parent class but will do so with inheritance. This can make a program’s design
less flexible. It also introduces the possibility of calling methods on
subclasses that don’t make sense or that cause errors because the methods don’t
apply to the subclass. In addition, some languages will only allow a subclass
to inherit from one class, further restricting the flexibility of a program’s
design.

For these reasons, Rust takes a different approach, using trait objects instead
of inheritance. Let’s look at how trait objects enable polymorphism in Rust.
