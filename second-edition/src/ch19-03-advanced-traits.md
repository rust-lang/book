# Advanced Traits

We covered traits in Chapter 10, but like lifetimes, we didn't get to all the
details. Now that we know more Rust, we can get into the nitty-gritty.

## Associated Types

We've described most of the things in this chapter as being very rare.
Associated types are somewhere in the middle; they're more rare than the rest
of the book, but more common than many of the things in this chapter.

Associated types look like this:

```rust
trait Foo {
    type Bar;

    fn foo(&self) -> Self::Bar;
}

impl Foo for i32 {
    type Bar = String;

    fn foo(&self) -> Self::Bar {
        self.to_string()
    }
}
```

The trait `Foo` has an associated type called `Bar`. We can then use
`Self::Bar` elsewhere in our trait definition to use that type.

This _feels_ like more generics. For example, this seems similar to
the following code:

```rust
trait Foo<Bar> {
    fn foo(&self) -> Bar;
}

impl Foo<String> for i32 {
    fn foo(&self) -> String {
        self.to_string()
    }
}
```

But there's one big difference: with the second definition, we could also
implement `Foo<i32> for i32`, or anything else. In other words, with a trait
that has a generic parameter, we can implement that trait for a type multiple
times, changing the parameters each time. But with associated types, we can't;
we can only define it one time: it's not actually generic.

There's another benefit to associated types: when using the trait, since there's
only one possible implementation, you end up with a lot less syntax. This is
easier with some code:

```rust
// a generic graph
trait GGraph<Node, Edge> {
    // methods would go here
}

// an associated graph
trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}
```

Let's say we wanted to compute the distance between two nodes in the graph.
With the generic graph, you'd have to write this:

```rust,ignore
fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { ... }
```

Even though `distance` doesn't need to know the types of the edges, we're
forced to declare an `E` parameter, because we need to to use `Graph`. But with
the associated type version:

```rust,ignore
fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 { ... }
```

This is much cleaner.

## Fully qualified syntax

Sometimes, methods can have the same names. Consider this code:

```rust
trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

let b = Baz;
```

If we were to try to call `b.f()`, we’d get an error:

```text
error[E0034]: multiple applicable items in scope
  --> <anon>:21:3
   |
21 | b.f();
   |   ^ multiple `f` found
   |
note: candidate #1 is defined in an impl of the trait `main::Foo` for the type `main::Baz`
  --> <anon>:13:5
   |
13 |     fn f(&self) { println!("Baz’s impl of Foo"); }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `main::Bar` for the type `main::Baz`
  --> <anon>:17:5
   |
17 |     fn f(&self) { println!("Baz’s impl of Bar"); }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

We need a way to disambiguate which method we need. We can do that like this:

```rust
# trait Foo {
#     fn f(&self);
# }
# trait Bar {
#     fn f(&self);
# }
# struct Baz;
# impl Foo for Baz {
#     fn f(&self) { println!("Baz’s impl of Foo"); }
# }
# impl Bar for Baz {
#     fn f(&self) { println!("Baz’s impl of Bar"); }
# }
# let b = Baz;
<Baz as Foo>::f(&b);
<Baz as Bar>::f(&b);
```

In other words, we can turn this:

```rust,ignore
foo.bar(args);
```

Into this:

```rust,ignore
<Foo as Bar>::bar(foo, args);
```

In a more generic sense,

```rust,ignore
<Type as Trait>::method(receiver, args);
```

We only need the `Type as` part if it's ambiguous. And we only need the `<>`
part if we need the `Type as` part. So in some cases, you could write

```rust,ignore
Trait::method(receiver, args);
```

This would have worked above:

```rust
# trait Foo {
#     fn f(&self);
# }
# trait Bar {
#     fn f(&self);
# }
# struct Baz;
# impl Foo for Baz {
#     fn f(&self) { println!("Baz’s impl of Foo"); }
# }
# impl Bar for Baz {
#     fn f(&self) { println!("Baz’s impl of Bar"); }
# }
# let b = Baz;
Foo::f(&b);
Bar::f(&b);
```

Here's an example of where the longer form is needed. We have an inherent
method `foo` and a trait method `foo`:


```rust
trait Foo {
    fn foo() -> i32;
}

struct Bar;

impl Bar {
    fn foo() -> i32 {
        20
    }
}

impl Foo for Bar {
    fn foo() -> i32 {
        10
    }
}

fn main() {
    assert_eq!(10, <Bar as Foo>::foo());
    assert_eq!(20, Bar::foo());
}
```

Using this syntax lets you call the trait method instead of the inherent one.

## Super traits

Sometimes, you may want a trait to be able to rely on another trait existing.
For example, let's say that you have a `Foo` trait and a `Bar` trait, but you
want `Bar`'s methods to be able to call `Foo`'s methods. Let's try it. (It
won't work just yet...)

```rust,ignore
trait Foo {
    fn foo(&self) {
        println!("Foo");
    }
}

trait Bar {
    fn bar(&self) {
        self.foo();
    }
}
```

We get this error:

```text
error: no method named `foo` found for type `&Self` in the current scope
  --> <anon>:10:14
   |
10 |         self.foo();
   |              ^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `foo`, perhaps you need to implement it:
   = help: candidate #1: `main::Foo`
```

In other words, we haven't said that anything that implements `Bar` also
implements `Foo`. We can do that with a `:`, like this:

```rust
trait Foo {
    fn foo(&self) {
        println!("Foo");
    }
}

trait Bar: Foo {
    fn bar(&self) {
        self.foo();
    }
}
```

This works fine.

## Coherence

Finally, traits have a concept called 'coherence'. This governs exactly who is
allowed to implement a trait. In short:

> To implement a type for a trait, you must have defined either the type, the
> trait, or both.

Put another way:

> You cannot implement a trait you didn't define for a type you didn't define.

For example, defining the `Display` trait, which is defined in the standard
library, on a tuple of string slices, which is defined in the standard library,
won't work:

```rust,ignore
use std::fmt;

impl fmt::Display for (&'static str, &'static str) {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
```

gives

```text
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
 --> <anon>:4:1
  |
4 |   impl fmt::Display for (&'static str, &'static str) {
  |  _^ starting here...
5 | |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
6 | |         write!(f, "({}, {})", self.0, self.1)
7 | |     }
8 | | }
  | |_^ ...ending here: impl doesn't use types inside crate
  |
  = note: the impl does not reference any types defined in this crate
```

Why do we have this rule? Allowing this would lead to ambiguity, confusion, and
broken code.  Imagine that we have a crate `foo` that has a type `A` and a
trait `B`. If we could implement `B` for `A` in our code, it would work, but
what if someone else _also_ implemented `B` for `A` in their code? Furthermore,
what if a new release of `foo` comes out and implements `B` for `A` themselves?
These problems are not insurmountable, of course; we could determine some kind
of complex precedent rules to determine which `impl` 'wins' and works. 

## The newtype pattern

There is a way to get around this, though. We call it the 'newtype pattern'.
You create a new type that's a thin wrapper around the type you want to
implement the trait for, and then implement the trait for the wrapper. This
*will* work:

```rust
use std::fmt;

struct Wrapper((&'static str, &'static str));

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", (self.0).0, (self.0).1)
    }
}
```

The downside is that since `Wrapper` is a new type, it has no methods; we'll
have to implement them all. If you want it to have every single method that the
inner type has, implementing `Deref` can help you there. Otherwise, you'll have
to implement the methods yourself.
