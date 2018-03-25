## C - Derivable Traits

In various places in the book, we’ve discussed the `derive` attribute
that you can apply to a struct or enum definition.

<!-- Above -- I wasn't clear throughout whether the derive attribute is
something passively applied to structs and enums by Rust, or something the
reader applies. I've experimented with making the tone more active, but may
have misinterpreted -- can you make it clear here? Should this be "we've
discussed the `derive` attribute you can apply to a struct or enum"? -->
<!-- Rust never edits your source code file for you. I'm curious to know what
parts of the book have given you that impression... I've tried to clarify here
but now I'm worried about other places in the book... /Carol -->

<!-- Below -- Can you lay out what it is we're showing them about derivable
traits in this appendix, just showing them some common ones and how to use
them? -->
<!-- No, we're showing *all* of the derivable traits provided by the standard
library. I guess explaining what we mean by "derivable" was too much of a
tangent for the beginning of this section? I'm not sure where that would fit
instead, so I took it out. So now the text that we had under the "standard
library traits that can be derived" section is here where it seems like you
were expecting it to be /Carol -->

The `derive` attribute generates code that will implement a trait with its own
default implementation, on the type you have annotated with the `derive`
syntax. In this appendix, we provide a reference of all of the traits in the
standard library that can be used with `derive`. Each section covers:

- What operators and methods deriving this trait will enable
- What the implementation of the trait provided by `derive` does
- What implementing the trait signifies about the type
- The conditions in which you’re allowed or not allowed to implement the trait
- Examples of operations that require the trait

If you want different behavior than that provided by the `derive` attribute,
consult the standard library documentation for each trait for details of how to
manually implement them.

<!-- Liz: I've incorporated the small sections that were after the list of
traits here and then moved the section headings out a level, what do you think?
/Carol -->

The rest of the traits defined in the standard library can’t be implemented on
your types using `derive`. These traits don’t have sensible default behavior,
so it’s up to you to implement them in the way that makes sense for what you’re
trying to accomplish.

An example of a trait that can’t be derived is `Display`, which handles
formatting for end users. You should always put thought into the appropriate
way to display a type to an end user: what parts of the type should an end user
be allowed to see? What parts would they find relevant? What format of the data
would be most relevant to them? The Rust compiler doesn’t have this insight and
so can’t provide appropriate default behavior for you.

The list of derivable traits provided in this appendix is not comprehensive:
libraries can implement `derive` for their own traits! In this way, the list of
traits you can use `derive` with is truly open-ended. Implementing `derive`
involves using a procedural macro, which is covered in Appendix D, “Macros.”

### `Debug` for Programmer Output

The `Debug` trait enables debug formatting in format strings, indicated by
adding `:?` within `{}` placeholders.

The `Debug` trait allows you to print instances of a type for debugging
purposes, so you and other programmers using your type can inspect an instance
at a particular point in a program’s execution.

`Debug` is required, for example, in use of the `assert_eq!` macro, which
prints the values of instances given as arguments if the equality assertion
fails so programmers can see why the two instances weren’t equal.

### `PartialEq` and `Eq` for Equality Comparisons

<!-- I've tried to phrase these definitions in a more active way, it seems like
we're saying using these traits gives us this capabilities --- apologies if
I've misunderstood, feel free to change the phrasing back to the "signifies
that..." version -->
<!-- More active is fine. I feel like it lost a tiny bit of meaning-- not only
can we use these capabilities on our own types, but other programmers using our
types can use these capabilities too. I've tried to reinsert that sentiment
occasionally. /Carol -->

The `PartialEq` trait allows you to compare instances of a type to check for
equality, and enables use of the `==` and `!=` operators.

Deriving `PartialEq` implements the `eq` method. When `PartialEq` is derived on
structs, two instances are equal only if *all* fields are equal, and not equal
if any fields are not equal. When derived on enums, each variant is equal to
itself and not equal to the other variants.

`PartialEq` is required, for example, with the use of the `assert_eq!` macro,
which needs to be able to compare two instances of a type for equality.

The `Eq` trait has no methods. Its purpose is to signal that for every value of
the annotated type, the value is equal to itself. The `Eq` trait can only be
applied to types that also implement `PartialEq`, though not all types that
implement `PartialEq` can implement `Eq`. One example of this is floating point
number types: the implementation of floating point numbers says that two
instances of the not-a-number value, `NaN`, are not equal to each other.

An example of when `Eq` is required is for keys in a `HashMap` so that the
`HashMap` can tell whether two keys are the same.

### `PartialOrd` and `Ord` for Ordering Comparisons

The `PartialOrd` trait allows you to compare instances of a type for sorting
purposes. A type that implements `PartialOrd` may be used with the `<`, `>`,
`<=`, and `>=` operators. The `PartialOrd` trait can only be applied to types
that also implement `PartialEq`.

Deriving `PartialOrd` implements the `partial_cmp` method, which returns an
`Option<Ordering>` that will be `None` when the values given do not produce an
ordering. An example of a value that doesn’t produce an ordering, even though
most values of that type can be compared, is the not-a-number (`NaN`) floating
point value. Calling `partial_cmp` with any floating point number and the `NaN`
floating point value will return `None`.

<!-- Above -- you mean when the values cannot be ordered, for example if they
are of types that can't be compared? -->
<!-- No, if they're *types* that can't be compared, then the PartialOrd trait
doesn't apply at all. I've tried to clarify and added an example /Carol-->

When derived on structs, `PartialOrd` compares two instances by comparing the
value in each field in the order in which the fields appear in the struct
definition. When derived on enums, variants of the enum declared earlier in the
enum definition are considered greater than the variants listed later.

`PartialOrd` is required, for example, for the `gen_range` method from the
`rand` crate that generates a random value in the range specified by a low
value and a high value.

The `Ord` trait allows you to know that for any two values of the annotated
type, a valid ordering will exist. The `Ord` trait implements the `cmp` method,
which returns an `Ordering` rather than an `Option<Ordering>` because a valid
ordering will always be possible. The `Ord` trait can only be applied to types
that also implement `PartialOrd` and `Eq` (and `Eq` requires `PartialEq`). When
derived on structs and enums, `cmp` behaves the same way as the derived
implementation for `partial_cmp` does with `PartialOrd`.

An example of when `Ord` is required is when storing values in a `BTreeSet<T>`,
a data structure that stores data based on the sort order of the values.

### `Clone` and `Copy` for Duplicating Values

<!-- Below -- I wasn't clear on the arbitrary code section of this explanation.
Are we saying using Clone (as opposed to copy) risks bringing it arbitrary
code? Why use Clone over copy? (I think we might have covered this in an
earlier chapter, so feel free to cross ref there too if that's an easier
explanation) -->
<!-- Yes, we covered this in chapter 4 and I've added a cross reference. /Carol
-->

The `Clone` trait allows you to explicitly create a deep copy of a value, and
the duplication process might involve running arbitrary code and copying heap
data. See the “Ways Variables and Data Interact: Clone” section in Chapter 4
for more information on `Clone`.

Deriving `Clone` implements the `clone` method which, when implemented for the
whole type, calls `clone` on each of the parts of the type. This means all of
the fields or values in the type must also implement `Clone` to derive `Clone`.

An example of when `Clone` is required is when calling the `to_vec` method on a
slice. The slice doesn’t own the type instances it contains, but the vector
returned from `to_vec` will need to own its instances, so `to_vec` calls
`clone` on each item. Thus, the type stored in the slice must implement `Clone`.

The `Copy` trait allows you to duplicate a value by only copying bits stored on
the stack; no arbitrary code is necessary. See the “Stack-Only Data: Copy”
section in Chapter 4 for more information on `Copy`.

<!-- I'm not clear on why the clone trait uses arbitrary code but copy doesn't
-- is this important to make clear? -->
<!-- We discussed this in chapter 4; I've added a cross ref. /Carol -->

The `Copy` trait does not define any methods to prevent programmers from
overloading those methods violating the assumption that no arbitrary code is
being run. That way, all programmers can assume that copying a value will be
very fast.

<!-- above -- I couldn't follow this either, what does that mean practically
for the programmer? What does overloading methods that violate the assumption
mean? -->
<!-- I added a sentence at the end of the paragraph, does that clear it up?
/Carol -->

You can derive `Copy` on any type whose parts all implement `Copy`. The `Copy`
trait can only be applied to types that also implement `Clone`, as a type that
implements `Copy` has a trivial implementation of `Clone`, doing the same thing
as `Copy`.

`Copy` is rarely required; types implement `Copy` have optimizations available
mean you don’t have to call `clone`, making the code more concise.

<!-- By "nicer" do you mean more efficient and understandable? -->
<!-- concise, I've changed /Carol -->

Everything possible with `Copy` can also be accomplished with `Clone`, but the
code might be slower or have to use `clone` in places.

### `Hash` for Mapping a Value to a Value of Fixed Size

The `Hash` trait allows you to take an instance of a type of arbitrary size and
map that instance to a value of fixed size, using a hash function. Deriving
`Hash` implements the `hash` method. The derived implementation of the `hash`
method combines the result of calling `hash` on each of the parts of the type,
meaning all fields or values must also implement `Hash` to derive `Hash`.

An example of when `Hash` is required is in storing keys in a `HashMap`, in
order to store data efficiently.

### `Default` for Default Values

The `Default` trait allows you to create a default value for a type. Deriving
`Default` implements the `default` method. The derived implementation of the
`default` method calls the `default` method on each part of the type, meaning
all fields or values in the type must also implement `Default` to derive
`Default.`

`Default::default` is commonly used in combination with the struct update
syntax discussed in the “Creating Instances From Other Instances With Struct
Update Syntax” section in Chapter 5. You can customize a few fields of a struct
and then set and use a default value for the rest of the fields by using
`..Default::default()`.

`Default` is required when, for example, you use the `unwrap_or_default` method
on `Option<T>` instances. If the `Option<T>` is `None`, the `unwrap_or_default`
method will return the result of `Default::default` for the type `T` stored in
the `Option<T>`.
