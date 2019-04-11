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
