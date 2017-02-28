# Is Rust OOP?

Back in Chapter 13, when we talked about functional features in Rust, we said
that we weren't going to worry about exactly what functional programming is. In
this chapter, we're going to talk about object oriented programming, and how
Rust fits into that picture. There are many different definitions of object
oriented programming, and so we're going to, like in Chapter 13, set the
definitions aside for now.

Instead, we're going to talk about things we hear from programmers who are new
to Rust, but worked with object-orieted languages previously. Sometimes, people
come to Rust and get a little stuck, because Rust doesn't support a feature
that they may rely on heavily in other languages.

## classes

Many object oriented programming languages support a construct called
"classes", which are central to their programming model. You can define a class
with some kind of "class" keyword, and then define the data and methods that
that class contains.

Rust does not have classes; instead, we separate our data and our behavior into
two different constructs. For data, some combination of structs and/or enums
can model our information, and for behavior, we have the `impl` keyword to
create methods.

This split is larger than just having two individual things where other
languages have one, however. Given that classes are how you create objects in
these systems, and the paradigm is named "object oriented programming,"
everything in this kind of system revolves around the classes and their
relationships to one another. With Rust, we have two separate axes to deal
with; we can abstract over data, or abstract over behavior, in any combination.
This leads to extra flexibility, but also very different designs, generally.

When modelling a problem in an object oriented way, a programmer asks
themselves "what objects exist, and how do they relate to one another?" When
modelling a problem in Rust, the question is instead, what data do I need to
represent, and what operations are required on that data?

## An object-oriented farm

As an example, consider an application that keeps track of animals on a farm,
and how to feed them. Here's a description of the problem:

> On the farm, we have cows, pigs, and horses.  We want farmers to be able to
> record buying and selling animals. Finally, we can feed horses or cows grass
> or hay, and we can feed pigs slop.

Let's think about how we'd solve this with objects. A classic way to figure out
a design here is to look at all of the nouns we have, and make each into an
object:

* farm
* cow
* pig
* horse
* grass
* hay
* slop

You then think if any of these objects have things in common, and make them
relate to one another. So for example, cows, pigs, and horses are all animals,
so we probably want an animal class. A Farm class represents our whole farm,
and so has a bunch of animals. While we used 'record' as a verb, it's also a
noun: we need to create records of buying and selling animals. Here's our final
list of classes:

* Farm, which has a list of Animals and a list of Feed
* Animal, with Cow, Pig, and Horse sub-classes, they each have a name
* Feed, with Grass, Hay, and Slop sub-classes
* Record, with an Animal and date.

Six classes! We said "sub-classes" here, but we haven't talked about
inheritance at all yet. Inheritance is a major way of re-using code in object
oriented systems. When a class inherits from another class, it gains all of its
methods and data, and can add new ones. It can also choose to override various
aspects of the class it's inheriting from, depending on the specific object
oriented system.

This mechanism allows re-using code. For example, if all of our Animals have
names, we can write functions that take an Animal, and pass any Cow, Pig, or
Horse to it, and it should work.

## A rusty farm

Let's look at this problem from a more... Rustic design perspective. We have
two questions to answer: what data do we need to keep track of, and what do we
need things to do?

Foremost, we need to keep track of records. Given that a record has a number of
different kinds of data, we need a `struct`:

```rust,ignore
struct Record {
   animal: Animal,
   date: i32, // timestamp
}
```

We've created two new types here, so let's figure those out too. For an
`Animal`, we know every speices of animal, so an `enum` is the right choice:

```rust
enum Species {
    Cow,
    Pig,
    Horse,
}
```

But we also need to keep track of the animals themselves; let's do it like
this:

```rust,ignore
struct Animal {
    name: String,
    species: Species,
}
```

Given that every animal has a name, we make an `Animal` struct and have the
name and species in it. If each animal had different attributes, we could have
made an enum, with each variant holding different data.

Feed is an enum:

```rust
enum Feed {
    Grass,
    Hay,
    Slop,
}
```

Finally, we need to keep track of all of our animals, so let's make a `Farm`.

```rust,ignore
struct Farm {
    animals: Vec<Animal>,
}
```

This is entirely focused on what data we have and how to represent it properly,
not on some abstract notion of "objects" or similarities between them. We have
no inheritance or super classes; in some cases, our `enum` kind of works like
one, but not really, as it's focused on data, not data and behavior.

Now we need to figure out what behavior we need. We'll need a `purchase` method
on our `Farm`. Since it's not shared, we'll make it an inherent method:

```rust,ignore
impl Farm {
    fn purchase(&mut self, animal: Animal) -> Record {
       // add the animal to our list of animals and generate a Record
    } 
}
```

Feeding an animal requires another inherent method:

```rust,ignore
impl Animal {
    fn feed(&self, feed: Feed) {
        // implementation goes here
    }
}
```

Finally, we need a way to refer to both Animals and Feed, so we can print out
messages to our users, like "Betsy ate the hay." To do this, we might make a
trait, like, say, `Named`:

```rust
trait Named {
    fn name(&self) -> String;
}
```

However, given that we want to display this to end users, the standard library
has a trait for this: `Display`. We'd implement it for both `Animal` and
`Feed`.

## static vs dynamic dispatch

Another feature of object oriented languages is "late binding" or "dynamic
dispatch." That is, most of the time in Rust, you can tell at compile time
which method is going to be called. However, this is less flexible than
determining the correct method at runtime. In addition, sometimes you just
can't know which method you need.

In these cases, Rust has a feature called "trait objects". Trait objects give
you dynamic dispatch, and they also erase the exact type. Here's an example:

```rust
use std::string::ToString;

let d = Box::new(5) as Box<ToString>;
```

Trait objects have to be behind some kind of pointer: `Box<T>`, `&T`, `Rc<T>`,
etc. You then cast it to the trait object's type, in this case, `Box<ToString>`.

After we've done so, the only method we can call on `d` is `to_string`, the
method defined by the `ToString` trait. We don't know that it's actually an
`i32` anymore, so there's no way to use those methods.

Trait objects are one way around the "everything must be the same type"
restriction that you'll see with things like `Vec<T>`. For example:

```rust
use std::string::ToString;

let d = Box::new(5) as Box<ToString>;
let c = Box::new('a') as Box<ToString>;
let b = Box::new(false) as Box<ToString>;

let v = vec![d, c, b];
```

Here, we have an `i32`, a `char`, and a `bool`. But since all of them implement
`ToString`, we can turn them into the same type, sorta: a trait object. We can
then store them into the vector just fine.

Trait objects aren't used very often in Rust, but sometimes they're the right
call.

### Object safety

Not all traits can be made into objects; only "object safe" traits can. A trait
is object safe as long as:

* It does not require `Self` to be `Sized`, and
* All of its methods are object safe.

What does it mean for a method to be object safe? It is safe if either:

* It requires `Self` to be `Sized` or
* It meets all three of the following:
    * It must not have any type parameters; and,
    * It must have a receiver that has type `Self` or which dereferences to the
Self type;
    * must not use `Self` anywhere but the reciever

Those rules are a bit formal, but think of it this way: if your method requires
`Self`, but an object forgets the exact type that it is, there's no way that it
can work. Same with type parameters; if the type is erased, there's no way to
know what types to fill in the parameters with.
