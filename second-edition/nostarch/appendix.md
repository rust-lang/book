
[TOC]

## Appendix A: Keywords

The following is a list of keywords that are reserved for current or future use
by the Rust language. As such, these may not be used as identifiers, such as
names of functions, variables, parameters, struct fields, modules, crates,
constants, macros, static values, attributes, types, traits, or lifetimes.

### Keywords Currently in Use

* `as` - perform primitive casting, disambiguate the specific trait
  containing an item, or rename items in `use` and `extern crate` statements
* `break` - exit a loop immediately
* `const` - define constant items or constant raw pointers
* `continue` - continue to the next loop iteration
* `crate` - link an external crate or a macro variable representing the crate
  in which the macro is defined
* `else` - fallback for `if` and `if let` control flow constructs
* `enum` - define an enumeration
* `extern` - link an external crate, function, or variable
* `false` - Boolean false literal
* `fn` - define a function or the function pointer type
* `for` - loop over items from an iterator, implement a trait, or specify a
  higher-ranked lifetime
* `if` - branch based on the result of a conditional expression
* `impl` - implement inherent or trait functionality
* `in` - part of `for` loop syntax
* `let` - bind a variable
* `loop` - loop unconditionally
* `match` - match a value to patterns
* `mod` - define a module
* `move` - make a closure take ownership of all its captures
* `mut` - denote mutability in references, raw pointers, or pattern bindings
* `pub` - denote public visibility in struct fields, `impl` blocks, or modules
* `ref` - bind by reference
* `return` - return from function
* `Self` - a type alias for the type implementing a trait
* `self` - method subject or current module
* `static` - global variable or lifetime lasting the entire program execution
* `struct` - define a structure
* `super` - parent module of the current module
* `trait` - define a trait
* `true` - Boolean true literal
* `type` - define a type alias or associated type
* `unsafe` - denote unsafe code, functions, traits, or implementations
* `use` - import symbols into scope
* `where` - denote clauses that constrain a type
* `while` - loop conditionally based on the result of an expression

<!-- we should make sure the definitions for each keyword are consistently
phrased, so for example for enum we say "defining an enumeration" but for fn we
passively call it a "function definition" -- perhaps a good medium would be
"define an enumeration" and "define a function"? Can you go through and make
those consistent? I've attempted it for a few, but am wary of changing meaning.
Also, you may decide to go the passive definition route, which is fine by me,
as long as it's consistent-->
<!-- I've tried, I'm not sure how to be active for keywords that are nouns
though. Please let me know if any still seem inconsistent /Carol -->

### Keywords Reserved for Future Use

These keywords do not have any functionality, but are reserved by Rust for
potential future use.

* `abstract`
* `alignof`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `offsetof`
* `override`
* `priv`
* `proc`
* `pure`
* `sizeof`
* `typeof`
* `unsized`
* `virtual`
* `yield`

## Appendix B: Operators and Symbols

<!-- We try not to stack headings even in the appendix, can you add some intro
text about what this appendix contains? Quick example below -->
<!-- Done! /Carol -->

This appendix is a glossary of Rust's syntax, including operators and other
symbols that appear by themselves or in the context of paths, generics, trait
bounds, macros, attributes, comments, tuples, and brackets.

### Operators

The following lists the operators in Rust, an example of how the operator would
appear in context, a short explanation, and whether that operator is
overloadable. If an operator is overloadable, the relevant trait to use to
overload that operator is listed.

<!-- PROD: I'm not sure how to handle this, would it be too big for a table? I
think some structure with aligned columns would make it a great reference -->

* `!` (`ident!(…)`, `ident!{…}`, `ident![…]`): denotes macro expansion.
* `!` (`!expr`): bitwise or logical complement. Overloadable (`Not`).
* `!=` (`var != expr`): nonequality comparison. Overloadable (`PartialEq`).
* `%` (`expr % expr`): arithmetic remainder. Overloadable (`Rem`).
* `%=` (`var %= expr`): arithmetic remainder and assignment. Overloadable (`RemAssign`).
* `&` (`&expr`, `&mut expr`): borrow.
* `&` (`&type`, `&mut type`, `&'a type`, `&'a mut type`): borrowed pointer type.
* `&` (`expr & expr`): bitwise AND. Overloadable (`BitAnd`).
* `&=` (`var &= expr`): bitwise AND and assignment. Overloadable (`BitAndAssign`).
* `&&` (`expr && expr`): logical AND.
* `*` (`expr * expr`): arithmetic multiplication. Overloadable (`Mul`).
* `*` (`*expr`): dereference.
* `*` (`*const type`, `*mut type`): raw pointer.
* `*=` (`var *= expr`): arithmetic multiplication and assignment. Overloadable (`MulAssign`).
* `+` (`trait + trait`, `'a + trait`): compound type constraint.
* `+` (`expr + expr`): arithmetic addition. Overloadable (`Add`).
* `+=` (`var += expr`): arithmetic addition and assignment. Overloadable (`AddAssign`).
* `,`: argument and element separator.
* `-` (`- expr`): arithmetic negation. Overloadable (`Neg`).
* `-` (`expr - expr`): arithmetic subtraction. Overloadable (`Sub`).
* `-=` (`var -= expr`): arithmetic subtraction and assignment. Overloadable (`SubAssign`).
* `->` (`fn(…) -> type`, `|…| -> type`): function and closure return type.
* `.` (`expr.ident`): member access.
* `..` (`..`, `expr..`, `..expr`, `expr..expr`): right-exclusive range literal.
* `..` (`..expr`): struct literal update syntax.
* `..` (`variant(x, ..)`, `struct_type { x, .. }`): “and the rest” pattern binding.
* `...` (`expr...expr`) *in a pattern*: inclusive range pattern.
* `/` (`expr / expr`): arithmetic division. Overloadable (`Div`).
* `/=` (`var /= expr`): arithmetic division and assignment. Overloadable (`DivAssign`).
* `:` (`pat: type`, `ident: type`): constraints.
* `:` (`ident: expr`): struct field initializer.
* `:` (`'a: loop {…}`): loop label.
* `;`: statement and item terminator.
* `;` (`[…; len]`): part of fixed-size array syntax
* `<<` (`expr << expr`): left-shift. Overloadable (`Shl`).
* `<<=` (`var <<= expr`): left-shift and assignment. Overloadable (`ShlAssign`).
* `<` (`expr < expr`): less-than comparison. Overloadable (`PartialOrd`).
* `<=` (`expr <= expr`): less-than or equal-to comparison. Overloadable (`PartialOrd`).
* `=` (`var = expr`, `ident = type`): assignment/equivalence.
* `==` (`expr == expr`): equality comparison. Overloadable (`PartialEq`).
* `=>` (`pat => expr`): part of match arm syntax.
* `>` (`expr > expr`): greater-than comparison. Overloadable (`PartialOrd`).
* `>=` (`expr >= expr`): greater-than or equal-to comparison. Overloadable (`PartialOrd`).
* `>>` (`expr >> expr`): right-shift. Overloadable (`Shr`).
* `>>=` (`var >>= expr`): right-shift and assignment. Overloadable (`ShrAssign`).
* `@` (`ident @ pat`): pattern binding.
* `^` (`expr ^ expr`): bitwise exclusive OR. Overloadable (`BitXor`).
* `^=` (`var ^= expr`): bitwise exclusive OR and assignment. Overloadable (`BitXorAssign`).
* `|` (`pat | pat`): pattern alternatives.
* `|` (`|…| expr`): closures.
* `|` (`expr | expr`): bitwise OR. Overloadable (`BitOr`).
* `|=` (`var |= expr`): bitwise OR and assignment. Overloadable (`BitOrAssign`).
* `||` (`expr || expr`): logical OR.
* `_`: “ignored” pattern binding. Also used to make integer-literals readable.
* `?` (`expr?`): Error propagation.

### Non-operator Symbols

<!-- And maybe a quick explanation of what you mean by non-operator
symbols/what counts as a non-operator symbol? -->
<!-- I've tried but it's hard to explain, it's the kind of thing you know when
you see it? /Carol -->

The following lists all non-letters that don't function as operators; that is,
they don't behave like a function or method call.

#### Standalone Syntax

* `'ident`: named lifetime or loop label
* `…u8`, `…i32`, `…f64`, `…usize`, *etc.*: numeric literal of specific type.
* `"…"`: string literal.
* `r"…"`, `r#"…"#`, `r##"…"##`, *etc.*: raw string literal, escape characters are not processed.
* `b"…"`: byte string literal, constructs a `[u8]` instead of a string.
* `br"…"`, `br#"…"#`, `br##"…"##`, *etc.*: raw byte string literal, combination of raw and byte string literal.
* `'…'`: character literal.
* `b'…'`: ASCII byte literal.
* `|…| expr`: closure.
* `!`: always empty bottom type for diverging functions.

#### Path-related Syntax

* `ident::ident`: namespace path.
* `::path`: path relative to the crate root (*i.e.* an explicitly absolute path).
* `self::path`: path relative to the current module (*i.e.* an explicitly relative path).
* `super::path`: path relative to the parent of the current module.
* `type::ident`, `<type as trait>::ident`: associated constants, functions, and types.
* `<type>::…`: associated item for a type which cannot be directly named (*e.g.* `<&T>::…`, `<[T]>::…`, *etc.*).
* `trait::method(…)`: disambiguating a method call by naming the trait which defines it.
* `type::method(…)`: disambiguating a method call by naming the type for which it’s defined.
* `<type as trait>::method(…)`: disambiguating a method call by naming the trait *and* type.

#### Generics

* `path<…>` (*e.g.* `Vec<u8>`): specifies parameters to generic type *in a type*.
* `path::<…>`, `method::<…>` (*e.g.* `"42".parse::<i32>()`): specifies parameters to generic type, function, or method *in an expression*. Often referred to as *turbofish*.
* `fn ident<…> …`: define generic function.
* `struct ident<…> …`: define generic structure.
* `enum ident<…> …`: define generic enumeration.
* `impl<…> …`: define generic implementation.
* `for<…> type`: higher-ranked lifetime bounds.
* `type<ident=type>` (*e.g.* `Iterator<Item=T>`): a generic type where one or more associated types have specific assignments.

#### Trait Bound Constraints

* `T: U`: generic parameter `T` constrained to types that implement `U`.
* `T: 'a`: generic type `T` must outlive lifetime `'a`. When we say that a type ‘outlives’ the lifetime, we mean that it cannot transitively contain any references with lifetimes shorter than `'a`.
* `T : 'static`: The generic type `T` contains no borrowed references other than `'static` ones.
* `'b: 'a`: generic lifetime `'b` must outlive lifetime `'a`.
* `T: ?Sized`: allow generic type parameter to be a dynamically-sized type.
* `'a + trait`, `trait + trait`: compound type constraint.

#### Macros and Attributes

* `#[meta]`: outer attribute.
* `#![meta]`: inner attribute.
* `$ident`: macro substitution.
* `$ident:kind`: macro capture.
* `$(…)…`: macro repetition.

#### Comments

* `//`: line comment.
* `//!`: inner line doc comment.
* `///`: outer line doc comment.
* `/*…*/`: block comment.
* `/*!…*/`: inner block doc comment.
* `/**…*/`: outer block doc comment.

#### Tuples

* `()`: empty tuple (*a.k.a.* unit), both literal and type.
* `(expr)`: parenthesized expression.
* `(expr,)`: single-element tuple expression.
* `(type,)`: single-element tuple type.
* `(expr, …)`: tuple expression.
* `(type, …)`: tuple type.
* `expr(expr, …)`: function call expression. Also used to initialize tuple `struct`s and tuple `enum` variants.
* `ident!(…)`, `ident!{…}`, `ident![…]`: macro invocation.
* `expr.0`, `expr.1`, …: tuple indexing.

#### Curly Brackets

* `{…}`: block expression.
* `Type {…}`: `struct` literal.

#### Square Brackets

* `[…]`: array literal.
* `[expr; len]`: array literal containing `len` copies of `expr`.
* `[type; len]`: array type containing `len` instances of `type`.
* `expr[expr]`: collection indexing. Overloadable (`Index`, `IndexMut`).
* `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]`: collection indexing pretending to be collection slicing, using `Range`, `RangeFrom`, `RangeTo`, `RangeFull` as the “index”.

## C - Derivable Traits

In various places in the book, we've discussed the `derive` attribute
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
so it's up to you to implement them in the way that makes sense for what you're
trying to accomplish.

An example of a trait that can’t be derived is `Display`, which handles
formatting for end users. You should always put thought into the appropriate
way to display a type to an end user: what parts of the type should an end user
be allowed to see? What parts would they find relevant? What format of the data
would be most relevant to them? The Rust compiler doesn’t have this insight and
so can't provide appropriate default behavior for you.

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
ordering. An example of a value that doesn't produce an ordering, even though
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
data. See the "Ways Variables and Data Interact: Clone" section in Chapter 4
for more information on `Clone`.

Deriving `Clone` implements the `clone` method which, when implemented for the
whole type, calls `clone` on each of the parts of the type. This means all of
the fields or values in the type must also implement `Clone` to derive `Clone`.

An example of when `Clone` is required is when calling the `to_vec` method on a
slice. The slice doesn’t own the type instances it contains, but the vector
returned from `to_vec` will need to own its instances, so `to_vec` calls
`clone` on each item. Thus, the type stored in the slice must implement `Clone`.

The `Copy` trait allows you to duplicate a value by only copying bits stored on
the stack; no arbitrary code is necessary. See the "Stack-Only Data: Copy"
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
mean you don't have to call `clone`, making the code more concise.

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

## Appendix D: Macros

We’ve used macros like `println!` throughout this book, but haven't fully
explored what a macro is and how it works. This appendix will explain:

- What macros are and how they differ from functions
- How to define a declarative macro to do metaprogramming
- How to define a procedural macro to create custom `derive` traits

We're covering the details of macros in an appendix because they’re still
evolving in Rust. Macros have changed and, in the near future, will change at a
quicker rate than the rest of the language and standard library since Rust 1.0,
so this section is more likely to date than the rest of the book. The code
shown here will still continue to work with future versions, due to Rust’s
stability guarantees, but there may be additional capabilities or easier ways
to write macros that weren't available at the time of this publication. Bear
that in mind if you try to implement anything from this appendix.

### The Difference Between Macros and Functions

Fundamentally, macros are a way of writing code that writes other code, known
as *metaprogramming*. In Appendix C, we discussed the `derive` attribute, which
generates an implementation of various traits for you. We’ve also used the
`println!` and `vec!` macros throughout the book. All of these macros *expand*
to produce more code than the code you’ve written yourself.

Metaprogramming is useful for reducing the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don’t have.

A function signature has to declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: we can call `println!("hello")` with one argument, or
`println!("hello {}", name)` with two arguments. Also, macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can’t, because it gets
called at runtime and a trait needs to be implemented at compile time.

The downside to implementing a macro over a function is that macro definitions
are more complex than function definitions because you’re writing Rust code
that writes Rust code. Due to this indirection, macro definitions are generally
more difficult to read, understand, and maintain than function definitions.

Another difference between macros and functions is that macro definitions
aren’t namespaced within modules like function definitions are. In order to
prevent unexpected name clashes when using external crates, you have to
explicitly bring the macros into the scope of your project at the same time as
bringing the external crate into scope, using the `#[macro_use]` annotation.
The following example would bring all the macros defined in the `serde` crate
into the scope of the current crate:

```
#[macro_use]
extern crate serde;
```

If `extern crate` was able to bring macros into scope by default without this
explicit annotation, you would be prevented from using two crates that happened
to define macros with the same name. In practice this conflict doesn’t come up
much, but the more crates you use, the more likely it is.

One last important difference between macros and functions: macros must be
defined or brought into scope *before* they’re called in a file, whereas
functions can be defined anywhere and called anywhere.

### Declarative Macros with `macro_rules!` for General Metaprogramming

The most widely used form of macros in Rust are *declarative macros*. These are
also sometimes referred to as *macros by example*, *`macro_rules!` macros*, or
just plain *macros*. At their core, declarative macros allow you to write
something similar to a Rust `match` expression. As discussed in Chapter 6,
`match` expressions are control structures that take an expression, compare the
resulting value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that have
code associated with them, but in this case the value is the literal Rust
source code passed to the macro, the patterns are compared with the structure
of that source code, and the code associated with each pattern is the code that
replaces the code passed to the macro. This all happens during compilation.

To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by taking a look at how the `vec!` macro is defined. Chapter
8 covered how we can use the `vec!` macro to create a new vector with
particular values. For example, this macro creates a new vector with three
integers inside:

```
let v: Vec<u32> = vec![1, 2, 3];
```

We could also use the `vec!` macro to make a vector of two integers or a vector
of five string slices---we wouldn't be able to use a function to do the same
because we wouldn’t know the number or type of values up front.

Let’s take a look at a slightly simplified definition of the `vec!` macro in
Listing AD-1:

```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

Listing AD-1: A simplified version of the `vec!` macro definition

> Note: the actual definition of the `vec!` macro in the standard library
> includes code to pre-allocate the correct amount of memory up-front. That
> code is an optimization that we’ve chosen not to include here to make the
> example simpler.

The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which we’re defining the macro is imported.
Without this annotation, even if someone depending on this crate uses the
`#[macro_use]` annotation, the macro would not be brought into scope.

We then start the macro definition with `macro_rules!` and the name of the
macro we’re defining *without* the exclamation mark. The name---in this case
`vec`---is followed by curly brackets denoting the body of the macro definition.

The structure in the `vec!` body is similar to the structure of a `match`
expression. Here we have one arm with the pattern `( $( $x:expr ),* )`,
followed by `=>` and the block of code associated with this pattern. If the
pattern matches, the associated block of code will be emitted. Given that this
is the only pattern in this macro, there’s only one valid way to match; any
other will be an error. More complex macros will have more than one arm.

Valid pattern syntax in macro definitions is different than the pattern syntax
covered in Chapter 18 because macro patterns are matched against Rust code
structure rather than values. Let’s walk through what the pieces of the pattern
in Listing AD-1 mean; for the full macro pattern syntax, see the reference at
*https://doc.rust-lang.org/stable/reference/macros.html*.

First, a set of parentheses encompasses the whole pattern. Next comes a dollar
sign (`$`) followed by a set of parentheses, which captures values that match
the pattern within the parentheses for use in the replacement code. Within
`$()` is `$x:expr`, which matches any Rust expression and gives the expression
the name `$x`.

The comma following `$()` indicates that a literal comma separator character
could optionally appear after the code that matches the code captured in `$()`.
The `*` following the comma specifies that the pattern matches zero or more of
whatever precedes the `*`.

When we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three
times with the three expressions `1`, `2`, and `3`.

Now let's look at the pattern in the body of the code associated with this arm:
The `temp_vec.push()` code within the `$()*` part is generated for each part
that matches `$()` in the pattern, zero or more times depending on how many
times the pattern matches. The `$x` is replaced with each expression matched.
When we call this macro with `vec![1, 2, 3];`, the code generated that replaces
this macro call will be:

<!-- Above What about temp_vec.push, do you want to quickly mention that? Or do
you mean "The `$()*` part and the content of the parentheses is generated for
each part that matches `$()` in the pattern"-->
<!-- The latter, I've tried to clarify /Carol -->

```
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

We’ve defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.

Given that most Rust programmers will *use* macros more than *write* macros,
that’s all we’ll discuss about `macro_rules!` in this book. To learn more about
how to write macros, consult the online documentation or other resources such
as The Little Book of Rust Macros at
*https://danielkeep.github.io/tlborm/book/index.html*.

### Procedural Macros for Custom `derive`

The second form of macros is called *procedural macros* because they’re more
like functions (which are a type of procedure). Procedural macros accept some
Rust code as an input, operate on that code, and produce some Rust code as an
output, rather than matching against patterns and replacing the code with other
code as declarative macros do. At the time of writing, you can only really
define procedural macros to allow your traits to be implemented on a type by
specifying the trait name in a `derive` annotation.

We're going to create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making users of our crate implement the `HelloMacro` trait for each of their
types, we’ll provide a procedural macro so users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined.

In other words, we’re going to write a crate that enables another programmer to
write code like Listing AD-2 using our crate:

Filename: src/main.rs

```
extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

Listing AD-2: The code a user of our crate will be able
to write with use of our procedural macro

This code will print `Hello, Macro! My name is Pancakes!` when we’re done. Let’s
get started! First we need to make a new library crate:

```
$ cargo new hello_macro --lib
```

Now we’ll define the `HelloMacro` trait and its associated function:

Filename: src/lib.rs

```
pub trait HelloMacro {
    fn hello_macro();
}
```

We have a trait and its function. At this point, a user of our crate would be
able to implement the trait themselves to achieve the desired functionality,
like so:

```
extern crate hello_macro;

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

However, they would need to write out the implementation block for each type
they wanted to use with `hello_macro`; we’d like to save them this work.

Additionally, we can’t yet provide a default implementation for the
`hello_macro` function that will print out the name of the type the trait is
implemented on: Rust doesn’t have reflection capabilities, so can’t look up the
type’s name at runtime. We need a macro to generate code at compile time.

<!--Defining Procedural Macros Requires a Separate Crate-->
<!-- Since this is a lone subheading, okay to merge with the general procedural
macros section? -->
<!-- Sure /Carol -->

The next step is to define the procedural macro. At the time of writing,
procedural macros need to be in their own crate. Eventually, this restriction
may be lifted. The convention for structuring crates and macro crates is as
such: for a crate named `foo`, a custom derive procedural macro crate is called
`foo-derive`. Let’s start a new crate called `hello_macro_derive` inside our
`hello_macro` project:

```
$ cargo new hello_macro_derive --lib
```

Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we’ll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, though, and programmers using these crates will need
to add both as dependencies and bring them both into scope. We could instead
have the `hello_macro` crate use `hello_macro_derive` as a dependency and
re-export the procedural macro code, but the way we've structured the project
makes it possible for programmers to use `hello_macro` even if they don’t want
the `derive` functionality.

We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We'll also need functionality from the `syn` and `quote` crates, as we'll see
in a moment, so we need to add them as dependencies. Add the following to the
*Cargo.toml* file for `hello_macro_derive`:

Filename: hello_macro_derive/Cargo.toml

```
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

To start defining the procedural macro, place the code from Listing AD-3 in
your *src/lib.rs* for the `hello_macro_derive` crate. Note that this won’t
compile until we add a definition for the `impl_hello_macro` function.

Note the way we've split the functions in AD-3; this will be the same for
almost every procedural macro crate you see or create, as it makes writing a
procedural macro more convenient. What you choose to do in the place where the
`impl_hello_macro` function is called will be different depending on the
purpose of your procedural macro.

Filename: hello_macro_derive/src/lib.rs

```
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_macro(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```

Listing AD-3: Code that most procedural macro crates will
need to have for processing Rust code

We have introduced three new crates: `proc_macro`, `syn` (available from
*https://crates.io/crates/syn*), and `quote` (available from
*https://crates.io/crates/quote*). The `proc_macro` crate comes with Rust, so
we didn’t need to add that to the dependencies in *Cargo.toml*. The
`proc_macro` crate allows us to convert Rust code into a string containing that
Rust code. The `syn` crate parses Rust code from a string into a data structure
that we can perform operations on. The `quote` crate takes `syn` data
structures and turns them back into Rust code. These crates make it much
simpler to parse any sort of Rust code we might want to handle: writing a full
parser for Rust code is no simple task.

The `hello_macro_derive` function will get called when a user of our library
specifies `#[derive(HelloMacro)]` on a type, because we’ve annotated the
`hello_macro_derive` function here with `proc_macro_derive` and specified the
name, `HelloMacro`, which matches our trait name; that’s the convention most
procedural macros follow.

This function first converts the `input` from a `TokenStream` to a `String` by
calling `to_string`. This `String` is a string representation of the Rust code
for which we are deriving `HelloMacro`. In the example in Listing AD-2, `s`
will have the `String` value `struct Pancakes;` because that’s the Rust code we
added the `#[derive(HelloMacro)]` annotation to.

<!-- I'm not sure why we convert to a string then to a structure we can use,
will that be clear to the reader here? -->
<!-- This is just how procedural macros work and what you have to do, which is
why we have the next note. /Carol -->

> Note: At the time of writing, the only thing you can do with a `TokenStream`
> is convert it to a string. A richer API will exist in the future.

Now we need to be able to parse the Rust code `String` into a data structure
that we can then interpret and perform operations on. This is where `syn` comes
to play. The `parse_derive_input` function in `syn` takes a `String` and
returns a `DeriveInput` struct representing the parsed Rust code. The following
shows the relevant parts of the `DeriveInput` struct we get from parsing the
string `struct Pancakes;`:

```
DeriveInput {
    // --snip--

    ident: Ident(
        "Pancakes"
    ),
    body: Struct(
        Unit
    )
}
```

The fields of this struct show that the Rust code we’ve parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the `syn`
API docs for `DeriveInput` at
*https://docs.rs/syn/0.11.11/syn/struct.DeriveInput.html* for more information.

At this point we haven’t defined the `impl_hello_macro` function, which is
where we’ll build the new Rust code we want to include. Before we get to that,
the last part of this `hello_macro_derive` function is using the `parse`
function from the `quote` crate to turn the output of the `impl_hello_macro`
function back into a `TokenStream`. The returned `TokenStream` is added to the
code that users of our crate write so that when they compile their crate, they
get extra functionality we provide.

You may have noticed that we’re calling `unwrap` to panic if the calls to the
`parse_derive_input` or `parse` functions fail here. Panicking on errors is
necessary in procedural macro code because `proc_macro_derive` functions must
return `TokenStream` rather than `Result` in order to conform to the procedural
macro API. We’ve chosen to keep this example simple by using `unwrap`; in
production code you should provide more specific error messages about what went
wrong by using `expect` or `panic!`.

Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `String` and a `DeriveInput` instance, let’s generate the code
implementing the `HelloMacro` trait on the annotated type:

Filename: hello_macro_derive/src/lib.rs

```
fn impl_hello_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    }
}
```

We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. With the code from Listing AD-2, `name` will
be `Ident("Pancakes")`.

The `quote!` macro lets us write up the Rust code that we want to return and
convert it into `quote::Tokens`. This macro also provides some really cool
templating mechanics; we can write `#name` and `quote!` will replace it with
the value in the variable named `name`. You can even do some repetition similar
to the way regular macros work. Check out the `quote` crate’s docs at
*https://docs.rs/quote* for a thorough introduction.

We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has one function, `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.

The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different than `format!` or
`println!`, which evaluate the expression and then turn the result into a
`String`. There’s a possibility that the `#name` input might be an expression
to print out literally so we use `stringify!`. Using `stringify!` also saves an
allocation by converting `#name` to a string literal at compile time.

At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let’s hook these crates up to the code in Listing
AD-2 to see it in action! Create a new binary project in your `projects`
directory with `cargo new --bin pancakes`. We need to add both `hello_macro`
and `hello_macro_derive` as dependencies in the `pancakes` crate’s
*Cargo.toml*. If you’ve chosen to publish your versions of `hello_macro` and
`hello_macro_derive` to *https://crates.io* they would be regular dependencies;
if not, you can specify them as `path` dependencies as follows:

```
[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

Put the code from Listing AD-2 into *src/main.rs*, and when you run `cargo run`
it should print `Hello, Macro! My name is Pancakes!` The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` took care
of adding the trait implementation.

### The Future of Macros

In the future, we’ll be expanding both declarative and procedural macros. Rust
will use better declarative macro system with the `macro` keyword, and we’ll
add more types of procedural macros, for more powerful tasks than just
`derive`. These systems are still under development at the time of publication;
please consult the online Rust documentation for the latest information.
