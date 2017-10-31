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

The struct itself is marked `pub` so that other code may use it, but the fields
within the struct remain private. This is important in this case because we
want to ensure that whenever a value is added or removed from the list, the
average is also updated. We do this by implementing `add`, `remove`, and
`average` methods on the struct as shown in Listing 17-2:

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
instance of `AveragedCollection`. When an item is added to `list` using the
`add` method or removed using the `remove` method, the implementations of each
call the private `update_average` method that takes care of updating the
`average` field as well.

We leave the `list` and `average` fields private so that there’s no way for
external code to add or remove items to the `list` field directly, otherwise
the `average` field might become out of sync when the `list` changes. The
`average` method returns the value in the `average` field, allowing external
code to read the `average` but not modify it.

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
depending on your reason for reaching for inheritance in the first place.

There are two main reasons to choose inheritance. The first is for re-use of
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
> sometimes called *bounded parametric polymorphism*.

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
