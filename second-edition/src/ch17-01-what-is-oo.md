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

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
```

<span class="caption">Listing 17-1: An `AveragedCollection` struct that
maintains a list of integers and the average of the items in the
collection.</span>

Note that the struct itself is marked `pub` so that other code may use this
struct, but the fields within the struct remain private. This is important in
this case because we want to ensure that whenever a value is added or removed
from the list, we also update the average. We do this by implementing `add`,
`remove`, and `average` methods on the struct as shown in Listing 17-2:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct AveragedCollection {
#     list: Vec<i32>,
#     average: f64,
# }
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

<span class="caption">Listing 17-2: Implementations of the public methods
`add`, `remove`, and `average` on `AveragedCollection`</span>

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
we can easily change aspects like the data structure in the future. For
instance, we could use a `HashSet` instead of a `Vec` for the `list` field. As
long as the signatures of the `add`, `remove`, and `average` public methods
stay the same, code using `AveragedCollection` wouldn’t need to change. This
wouldn’t necessarily be the case if we exposed `list` to external code:
`HashSet` and `Vec` have different methods for adding and removing items, so
the external code would likely have to change if it was modifying `list`
directly.

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
saw in Listing 10-15 when we added a default implementation of the `summary`
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
