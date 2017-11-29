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

## `PartialEq` and `Eq` for Equality Comparisons

The `Eq` and `PartialEq` traits enable the `==` and `!=` operators.

The `PartialEq` trait signifies that instances of a type have a *partial
equivalence relation*, which means that for any instances of that type:

* If `a == b`, then `b == a`. The equality relationship is symmetric.
* If `a == b` and `b == c`, then `a == c`. The equality relationshisp is
  transitive.

The `PartialEq` trait defines the `eq` method. When derived on structs, two
instances are equal if all fields are equal, and not equal if any fields are
not equal. When derived on enums, each variant is equal to itself and not equal
to the other variants.

An example of when `PartialEq` is required is the `assert_eq!` macro, which
needs to be able to compare two instances of a type for equality.

The `Eq` trait doesn't have any methods. It only signals that a type has a
*full equivalence relation*, which means in addition to the equality
relationship being symmetric and transitive, it is also reflexive:

* For all instances `a`, `a == a` must be true.

The `Eq` trait can only be applied to types that also implement `PartialEq`. An
example of types that implements `PartialEq` but that cannot implement `Eq` are
floating point number types: the implementation of floating point numbers says
that two instances of the not-a-number type, `NaN`, are not equal to each other.

An example of when `Eq` is needed is for keys in a `HashMap` so that the
`HashMap` can tell whether two keys are the same.

* `Ord`, `PartialOrd`, the traits for the `<` and `>` operators.
* `Copy` and `Clone`, which control how to make copies of your structs and enums.
* `Hash`, which is used by `HashMap` for its keys.
* `Default` and `Zero`, which provide default or zero values.
* `Debug` and notably, *not* `Display`, the formatting traits.

> If you remember from Chapter 5, `Display` is for end-users, and so is specific
> to your application. As such, we don't provide a way to derive `Display`, as
> there's no way to understand what the correct output should be.

## Custom `derive`

The above list is not comprehensive, however: libraries can implement `derive`
for their own types! In this way, the list of traits you can use `derive` with
is truly open-ended. To learn how this is possible, please read the next appendix,
"Macros."