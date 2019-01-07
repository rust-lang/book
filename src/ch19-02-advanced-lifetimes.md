## Advanced Lifetimes

In Chapter 10 in the [“Validating References with Lifetimes”]
[validating-references-with-lifetimes]<!-- ignore --> section, you learned how
to annotate references with lifetime parameters to tell Rust how lifetimes of
different references relate. You saw how every reference has a lifetime, but
most of the time, Rust will let you elide lifetimes. Now we’ll look at three
advanced features of lifetimes that we haven’t covered yet:

* Lifetime subtyping: ensures that one lifetime outlives another lifetime
* Lifetime bounds: specifies a lifetime for a reference to a generic type
* Inference of trait object lifetimes: allows the compiler to infer trait
  object lifetimes and when they need to be specified

### Lifetime Bounds on References to Generic Types

In the [“Traits as Parameters”][traits-as-parameters]<!-- ignore --> section in
Chapter 10, we discussed using trait bounds on generic types. We can also add
lifetime parameters as constraints on generic types; these are called *lifetime
bounds*. Lifetime bounds help Rust verify that references in generic types
won’t outlive the data they’re referencing.

As an example, consider a type that is a wrapper over references. Recall the
`RefCell<T>` type from the [“`RefCell<T>` and the Interior Mutability Pattern”]
[refcellt-and-the-interior-mutability-pattern]<!-- ignore --> section in
Chapter 15: its `borrow` and `borrow_mut` methods return the types `Ref` and
`RefMut`, respectively. These types are wrappers over references that keep
track of the borrowing rules at runtime. The definition of the `Ref` struct is
shown in Listing 19-16, without lifetime bounds for now.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
struct Ref<'a, T>(&'a T);
```

<span class="caption">Listing 19-16: Defining a struct to wrap a reference to a
generic type, without lifetime bounds</span>

Without explicitly constraining the lifetime `'a` in relation to the generic
parameter `T`, Rust will error because it doesn’t know how long the generic
type `T` will live:

```text
error[E0309]: the parameter type `T` may not live long enough
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> src/lib.rs:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

Because `T` can be any type, `T` could be a reference or a type that holds one
or more references, each of which could have their own lifetimes. Rust can’t be
sure `T` will live as long as `'a`.

Fortunately, the error provides helpful advice on how to specify the lifetime
bound in this case:

```text
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at
```

Listing 19-17 shows how to apply this advice by specifying the lifetime bound
when we declare the generic type `T`.

```rust
struct Ref<'a, T: 'a>(&'a T);
```

<span class="caption">Listing 19-17: Adding lifetime bounds on `T` to specify
that any references in `T` live at least as long as `'a`</span>

This code now compiles because the `T: 'a` syntax specifies that `T` can be any
type, but if it contains any references, the references must live at least as
long as `'a`.

We could solve this problem in a different way, as shown in the definition of a
`StaticRef` struct in Listing 19-18, by adding the `'static` lifetime bound on
`T`. This means if `T` contains any references, they must have the `'static`
lifetime.

```rust
struct StaticRef<T: 'static>(&'static T);
```

<span class="caption">Listing 19-18: Adding a `'static` lifetime bound to `T`
to constrain `T` to types that have only `'static` references or no
references</span>

Because `'static` means the reference must live as long as the entire program,
a type that contains no references meets the criteria of all references living
as long as the entire program (because there are no references). For the borrow
checker concerned about references living long enough, there is no real
distinction between a type that has no references and a type that has
references that live forever: both are the same for determining whether or not
a reference has a shorter lifetime than what it refers to.

### Inference of Trait Object Lifetimes

In Chapter 17 in the [“Using Trait Objects That Allow for Values of Different
Types”][using-trait-objects-that-allow-for-values-of-different-types]<!--
ignore --> section, we discussed trait objects, consisting of a trait behind a
reference, that allow us to use dynamic dispatch. We haven’t yet discussed what
happens if the type implementing the trait in the trait object has a lifetime
of its own. Consider Listing 19-19 where we have a trait `Red` and a struct
`Ball`. The `Ball` struct holds a reference (and thus has a lifetime parameter)
and also implements trait `Red`. We want to use an instance of `Ball` as the
trait object `Box<dyn Red>`.

<span class="filename">Filename: src/main.rs</span>

```rust
trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}
```

<span class="caption">Listing 19-19: Using a type that has a lifetime parameter
with a trait object</span>

This code compiles without any errors, even though we haven’t explicitly
annotated the lifetimes involved in `obj`. This code works because there are
rules for working with lifetimes and trait objects:

* The default lifetime of a trait object is `'static`.
* With `&'a Trait` or `&'a mut Trait`, the default lifetime of the trait object
  is `'a`.
* With a single `T: 'a` clause, the default lifetime of the trait object is
  `'a`.
* With multiple clauses like `T: 'a`, there is no default lifetime; we must be
  explicit.

When we must be explicit, we can add a lifetime bound on a trait object like
`Box<dyn Red>` using the syntax `Box<dyn Red + 'static>` or `Box<dyn Red +
'a>`, depending on whether the reference lives for the entire program or not.
As with the other bounds, the syntax adding a lifetime bound means that any
implementor of the `Red` trait that has references inside the type must have
the same lifetime specified in the trait object bounds as those references.

Next, let’s look at some other advanced features that manage traits.

[lifetime-annotations-in-struct-definitions]:
ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions
[refcellt-and-the-interior-mutability-pattern]:
ch15-05-interior-mutability.html#refcellt-and-the-interior-mutability-pattern
[traits-as-parameters]: ch10-02-traits.html#traits-as-parameters
[using-trait-objects-that-allow-for-values-of-different-types]:
ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
