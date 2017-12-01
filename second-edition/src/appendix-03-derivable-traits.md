# C - Derivable Traits

In various places in the book, we discussed the `derive` attribute that is
applied to a struct or enum. This attribute generates code that implements a
trait on the annotated type with a default implementation. In this example, the
`#[derive(Debug)]` attribute implements the `Debug` trait for the `Point`
struct:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

The code that the compiler generates for the implementation of `Debug` is
similar to this code:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
impl ::std::fmt::Debug for Point {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Point { x: ref __self_0_0, y: ref __self_0_1 } => {
                let mut builder = __arg_0.debug_struct("Point");
                let _ = builder.field("x", &&(*__self_0_0));
                let _ = builder.field("y", &&(*__self_0_1));
                builder.finish()
            }
        }
    }
}
```

The generated code implements sensible default behavior for the `Debug` trait's
`fmt` function: a `match` expression destructures a `Point` instance into its
field values. Then it builds up a string containing the struct's name and each
field's name and value. This means we're able to use debug formatting on a
`Point` instance to see what value each field has.

The generated code isn't particularly easy to read because it's only for the
compiler to consume, rather than for programmers to read! The `derive`
attribute and the default implementation of `Debug` has saved us all of the
work of writing this code for every struct or enum that we want to be able to
print using debug formatting.

The `derive` attribute has default implementations for the following traits
provided by the standard library. If you want different behavior than what the
`derive` attribute provides, consult the standard library documentation for
each trait for the details needed for manual implementation of the traits.

## What might make you derive/what do errors look like


## Standard Library Traits that Can Be Derived

The following sections list all of the traits in the standard library that can
be used with `derive`. Each section covers:

- What operators and methods deriving this trait will enable
- What the implementation of the trait provided by `derive` does
- What implementing the trait signifies about the type
- The conditions in which you're allowed or not allowed to implement the trait
- Examples of operations that require the trait

### `Debug` for Programmer Output

The `Debug` trait enables debug formatting in format strings, indicated by
adding `:?` within `{}` placeholders.

The `Debug` trait signifies that instances of a type may be printed by
programmers in order to debug their programs by inspecting an instance of a
type at a particular point in a program's execution.

An example of when `Debug` is required is the `assert_eq!` macro, which prints
the values of the instances given as arguments if the equality assertion fails
so that programmers can see why the two instances weren't equal.

### `PartialEq` and `Eq` for Equality Comparisons

The `PartialEq` trait signifies that instances of a type can be compared to
each other for equality, and enables use of the `==` and `!=` operators.

Deriving `PartialEq` implements the `eq` method. When derived on structs, two
instances are equal if all fields are equal, and not equal if any fields are
not equal. When derived on enums, each variant is equal to itself and not equal
to the other variants.

An example of when `PartialEq` is required is the `assert_eq!` macro, which
needs to be able to compare two instances of a type for equality.

The `Eq` trait doesn't have any methods. It only signals that for every value
of the annotated type, the value is equal to itself. The `Eq` trait can only be
applied to types that also implement `PartialEq`. An example of types that
implements `PartialEq` but that cannot implement `Eq` are floating point number
types: the implementation of floating point numbers says that two instances of
the not-a-number value, `NaN`, are not equal to each other.

An example of when `Eq` is required is for keys in a `HashMap` so that the
`HashMap` can tell whether two keys are the same.

### `PartialOrd` and `Ord` for Ordering Comparisons

The `PartialOrd` trait signifies that instances of a type can be compared to
each other to see which is larger than the other for sorting purposes. A type
that implements `PartialOrd` may be used with the `<`, `>`, `<=`, and `>=`
operators. The `PartialOrd` trait can only be applied to types that also
implement `PartialEq`.

Deriving `PartialOrd` implements the `partial_cmp` method, which returns an
`Option<Ordering>` that may be `None` if comparing the given values does not
produce an ordering. When derived on structs, two instances of the struct are
compared by comparing the value in each field in the order in which the fields
appear in the struct defintion. When derived on enums, variants of the enum
declared earlier in the enum defintion are greater than the variants listed
later.

An example of when `PartialOrd` is required is the `gen_range` method in the
`rand` crate that generates a random value in the range specified by a low
value and a high value.

The `Ord` trait signifies that for any two value of the annotated type, a valid
ordering exists. The `Ord` trait implements the `cmp` method, which returns an
`Ordering` rather than an `Option<Ordering>` because a valid ordering will
always be possible. The `Ord` trait can only be applied to types that also
implement `PartialOrd` and `Eq` (and `Eq` requires `PartialEq`). When derived
on structs and enums, `cmp` behaves the same way as the derived implementation
for `partial_cmp` does with `PartialOrd`.

An example of when `Ord` is required is when storing values in a `BTreeSet<T>`,
a data structure that stores data based on the sort order of the values.

### `Clone` and `Copy` for Duplicating Values

The `Clone` trait signifies there is a way to explicitly create a duplicate of
a value, and the duplication process might involve running arbitrary code.
Deriving `Clone` implements the `clone` method. When derived, the
implementation of `clone` for the whole type calls `clone` on each of the parts
of the type, so all of the fields or values in the type must also implement
`Clone` to derive `Clone`.

An example of when `Clone` is required is when calling the `to_vec` method on a
slice containing instances of some type. The slice doesn't own the instances
but the vector returned from `to_vec` will need to own its instances, so the
implementation of `to_vec` calls `clone` on each item. Thus, the type stored in
the slice must implement `Clone`.

The `Copy` trait signifies that a value can be duplicated by only copying bits;
no other code is necessary. The `Copy` trait does not define any methods to
prevent programmers from overloading those methods violating the assumption
that no arbitrary code is being run. You can derive `Copy` on any type whose
parts all implement `Copy`. The `Copy` trait can only be applied to types that
also implement `Clone`.

An example of when `Copy` is required is when storing values of that type in a
`Cell<T>`, a data structure that provides interior mutability by moving values
into and out of the cell.

### `Hash` for Mapping a Value to a Value of Fixed Size

The `Hash` trait signifies there is a way to take an instance of a type that
takes up an arbitrary amount of size and map that instance to a value of fixed
size by using a hash function. Deriving `Hash` implements the `hash` method.
When derived, the implementation of `hash` for the whole type combines the
result of calling `hash` on each of the parts of the type, so all of the fields
or values in the type must also implement `Hash` to derive `Hash`.

An example of when `Hash` is required is for keys in a `HashMap` so that the
`HashMap` can store data efficiently.

### `Default` for Default Values

The `Default` trait signifies there is a way to create a default value for a
type. Deriving `Default` implements the `default` method. When derived, the
implementation of `Default` for the whole type calls the `default` method on
each of the parts of the type, so all of the fields or values in the type must
also implement `Default` to derive `Default.`

An example of when `Default` is required is the `unwrap_or_default` method on
`Option<T>` instances. If the `Option<T>` is `None`, the `unwrap_or_default`
method will return the result of `Default::default` for the type `T` stored in
the `Option<T>`.

## Standard Library Traits that Can't Be Derived

The rest of the traits defined in the standard library can't be implemented on
your types using `derive`. These traits don't have a sensible default behavior
they could have, so you are required to implement them in the way that makes
sense for what you are trying to accomplish with your code.

An example of a trait that can't be derived is `Display`, which handles
formatting of a type for end users of your programs. You should put thought
into the appropriate way to display a type to an end user: what parts of the
type should an end user be allowed to see? What parts would they find relevant?
What format of the data would be most relevant to them? The Rust compiler
doesn't have this insight into your application, so you must provide it.

## Making Custom Traits Derivable

The above list is not comprehensive, however: libraries can implement `derive`
for their own types! In this way, the list of traits you can use `derive` with
is truly open-ended. Implementing `derive` involves using a procedural macro,
which is covered in the next appendix, "Macros."
