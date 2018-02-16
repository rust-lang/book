## Appendix A: Keywords

The following keywords are reserved by the Rust language and may not be used as
identifiers such as names of functions, variables, parameters, struct fields,
modules, crates, constants, macros, static values, attributes, types, traits,
or lifetimes.

### Keywords Currently in Use

* `as` - primitive casting, disambiguating the specific trait containing an
  item, or renaming items in `use` and `extern crate` statements
* `break` - exit a loop immediately
* `const` - constant items and constant raw pointers
* `continue` - continue to the next loop iteration
* `crate` - external crate linkage or a macro variable representing the crate
  in which the macro is defined
* `else` - fallback for `if` and `if let` control flow constructs
* `enum` - defining an enumeration
* `extern` - external crate, function, and variable linkage
* `false` - Boolean false literal
* `fn` - function definition and function pointer type
* `for` - iterator loop, part of trait impl syntax, and higher-ranked lifetime
  syntax
* `if` - conditional branching
* `impl` - inherent and trait implementation block
* `in` - part of `for` loop syntax
* `let` - variable binding
* `loop` - unconditional, infinite loop
* `match` - pattern matching
* `mod` - module declaration
* `move` - makes a closure take ownership of all its captures
* `mut` - denotes mutability in references, raw pointers, and pattern bindings
* `pub` - denotes public visibility in struct fields, `impl` blocks, and modules
* `ref` - by-reference binding
* `return` - return from function
* `Self` - type alias for the type implementing a trait
* `self` - method subject or current module
* `static` - global variable or lifetime lasting the entire program execution
* `struct` - structure definition
* `super` - parent module of the current module
* `trait` - trait definition
* `true` - Boolean true literal
* `type` - type alias and associated type definition
* `unsafe` - denotes unsafe code, functions, traits, and implementations
* `use` - import symbols into scope
* `where` - type constraint clauses
* `while` - conditional loop

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

### Operators

The following lists the operators in Rust, an example of how the operator would
appear in context, a short explanation, and whether that operator is
overloadable. If an operator is overloadable, the relevant trait to use to
overload that operator is listed.

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
* `...` (`...expr`, `expr...expr`) *in an expression*: inclusive range expression.
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
* `<=` (`var <= expr`): less-than or equal-to comparison. Overloadable (`PartialOrd`).
* `=` (`var = expr`, `ident = type`): assignment/equivalence.
* `==` (`var == expr`): equality comparison. Overloadable (`PartialEq`).
* `=>` (`pat => expr`): part of match arm syntax.
* `>` (`expr > expr`): greater-than comparison. Overloadable (`PartialOrd`).
* `>=` (`var >= expr`): greater-than or equal-to comparison. Overloadable (`PartialOrd`).
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

# C - Derivable Traits

In various places in the book, we discussed the `derive` attribute that is
applied to a struct or enum. This attribute generates code that implements a
trait on the annotated type with a default implementation. In this example, the
`#[derive(Debug)]` attribute implements the `Debug` trait for the `Point`
struct:

```
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

The code that the compiler generates for the implementation of `Debug` is
similar to this code:

```
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

The generated code implements sensible default behavior for the `Debug` trait’s
`fmt` function: a `match` expression destructures a `Point` instance into its
field values. Then it builds up a string containing the struct’s name and each
field’s name and value. This means we’re able to use debug formatting on a
`Point` instance to see what value each field has.

The generated code isn’t particularly easy to read because it’s only for the
compiler to consume, rather than for programmers to read! The `derive`
attribute and the default implementation of `Debug` has saved us all of the
work of writing this code for every struct or enum that we want to be able to
print using debug formatting.

The `derive` attribute has default implementations for the following traits
provided by the standard library. If you want different behavior than what the
`derive` attribute provides, consult the standard library documentation for
each trait for the details needed for manual implementation of the traits.

## Standard Library Traits that Can Be Derived

The following sections list all of the traits in the standard library that can
be used with `derive`. Each section covers:

- What operators and methods deriving this trait will enable
- What the implementation of the trait provided by `derive` does
- What implementing the trait signifies about the type
- The conditions in which you’re allowed or not allowed to implement the trait
- Examples of operations that require the trait

### `Debug` for Programmer Output

The `Debug` trait enables debug formatting in format strings, indicated by
adding `:?` within `{}` placeholders.

The `Debug` trait signifies that instances of a type may be printed by
programmers in order to debug their programs by inspecting an instance of a
type at a particular point in a program’s execution.

An example of when `Debug` is required is the `assert_eq!` macro, which prints
the values of the instances given as arguments if the equality assertion fails
so that programmers can see why the two instances weren’t equal.

### `PartialEq` and `Eq` for Equality Comparisons

The `PartialEq` trait signifies that instances of a type can be compared to
each other for equality, and enables use of the `==` and `!=` operators.

Deriving `PartialEq` implements the `eq` method. When derived on structs, two
instances are equal if all fields are equal, and not equal if any fields are
not equal. When derived on enums, each variant is equal to itself and not equal
to the other variants.

An example of when `PartialEq` is required is the `assert_eq!` macro, which
needs to be able to compare two instances of a type for equality.

The `Eq` trait doesn’t have any methods. It only signals that for every value
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
appear in the struct definition. When derived on enums, variants of the enum
declared earlier in the enum definition are greater than the variants listed
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
slice containing instances of some type. The slice doesn’t own the instances
but the vector returned from `to_vec` will need to own its instances, so the
implementation of `to_vec` calls `clone` on each item. Thus, the type stored in
the slice must implement `Clone`.

The `Copy` trait signifies that a value can be duplicated by only copying bits;
no other code is necessary. The `Copy` trait does not define any methods to
prevent programmers from overloading those methods violating the assumption
that no arbitrary code is being run. You can derive `Copy` on any type whose
parts all implement `Copy`. The `Copy` trait can only be applied to types that
also implement `Clone`, as a type that implements `Copy` has a trivial
implementation of `Clone`, doing the same thing as `Copy`.

`Copy` is rarely required; when types implement `Copy`, there are optimizations
that can be applied and the code becomes nicer because you don’t have to call
`clone`. Everything possible with `Copy` can also be accomplished with `Clone`,
but the code might be slower or have to use `clone` in places.

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

A common use of `Default::default` is in combination with the struct update
syntax discussed in the “Creating Instances From Other Instances With Struct
Update Syntax” section in Chapter 5. You can customize a few fields of a struct
and then use the default values for the rest by using `..Default::default()`.

An example of when `Default` is required is the `unwrap_or_default` method on
`Option<T>` instances. If the `Option<T>` is `None`, the `unwrap_or_default`
method will return the result of `Default::default` for the type `T` stored in
the `Option<T>`.

## Standard Library Traits that Can’t Be Derived

The rest of the traits defined in the standard library can’t be implemented on
your types using `derive`. These traits don’t have a sensible default behavior
they could have, so you are required to implement them in the way that makes
sense for what you are trying to accomplish with your code.

An example of a trait that can’t be derived is `Display`, which handles
formatting of a type for end users of your programs. You should put thought
into the appropriate way to display a type to an end user: what parts of the
type should an end user be allowed to see? What parts would they find relevant?
What format of the data would be most relevant to them? The Rust compiler
doesn’t have this insight into your application, so you must provide it.

## Making Custom Traits Derivable

The above list is not comprehensive, however: libraries can implement `derive`
for their own types! In this way, the list of traits you can use `derive` with
is truly open-ended. Implementing `derive` involves using a procedural macro,
which is covered in the next appendix, “Macros.”

# D - Macros

We’ve used macros, such as `println!`, throughout this book. This appendix will
explain:

- What macros are and how they differ from functions
- How to define a declarative macro to do metaprogramming
- How to define a procedural macro to create custom `derive` traits

Macros are covered in an appendix because they’re still evolving. They have
changed and will change more than the rest of the language and standard library
since Rust 1.0, so this section will likely get out of date more than the rest
of this book. The code shown here will still continue to work due to Rust’s
stability guarantees, but there may be additional capabilities or easier ways
to write macros that aren’t available at the time of this publication.

## Macros are More Flexible and Complex than Functions

Fundamentally, macros are a way of writing code that writes other code, which
is known as *metaprogramming*. In the previous appendix, we discussed the
`derive` attribute, which generates an implementation of various traits for
you. We’ve also used the `println!` and `vec!` macros. All of these macros
*expand* to produce more code than what you’ve written in your source code.

Metaprogramming is useful to reduce the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don’t have, as we discussed in Chapter 1.
A function signature has to declare the number and type of parameters the
function has. Macros can take a variable number of parameters: we can call
`println!("hello")` with one argument, or `println!("hello {}", name)` with two
arguments. Also, macros are expanded before the compiler interprets the meaning
of the code, so a macro can, for example, implement a trait on a given type,
whereas a function can’t because a function gets called at runtime and a trait
needs to be implemented at compile time.

The downside to implementing a macro rather than a function is that macro
definitions are more complex than function definitions. You’re writing Rust
code that writes Rust code, and macro definitions are generally more difficult
to read, understand, and maintain than function definitions.

Another difference between macros and functions is that macro definitions
aren’t namespaced within modules like function definitions are. In order to
prevent unexpected name clashes when using a crate, when bringing an external
crate into the scope of your project, you have to explicitly bring the macros
into the scope of your project as well with the `#[macro_use]` annotation. This
example would bring all the macros defined in the `serde` crate into the scope
of the current crate:

```
#[macro_use]
extern crate serde;
```

If `extern crate` also brought macros into scope by default, you wouldn’t be
allowed to use two crates that happened to define macros with the same name. In
practice this conflict doesn’t come up much, but the more crates you use, the
more likely it is.

One last important difference between macros and functions: macros must be
defined or brought into scope before they’re called in a file. Unlike
functions, where we can define a function at the bottom of a file yet call it
at the top, we always have to define macros before we’re able to call them.

## Declarative Macros with `macro_rules!` for General Metaprogramming

The first form of macros in Rust, and the one that’s most widely used, is
called *declarative macros*. These are also sometimes referred to as *macros by
example*, *`macro_rules!` macros*, or just plain *macros*. At their core,
declarative macros allow you to write something similar to a Rust `match`
expression. As discussed in Chapter 6, `match` expressions are control
structures that take an expression, compare the resulting value of the
expression to patterns, and then choose the code specified with the matching
pattern when the program runs. Macros also have a value that is compared to
patterns that have code associated with them, but the value is the literal Rust
code passed to the macro, the patterns match the structure of that source code,
and the code associated with each pattern is the code that is generated to
replace the code passed to the macro. This all happens during compilation.

To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by taking a look at how the `vec!` macro is defined. Chapter
8 covered how we can use the `vec!` macro to create a new vector that holds
particular values. For example, this macro creates a new vector with three
integers inside:

```
let v: Vec<u32> = vec![1, 2, 3];
```

We can also use `vec!` to make a vector of two integers or a vector of five
string slices. Because we don’t know the number or type of values, we can’t
define a function that is able to create a new vector with the given elements
like `vec!` can.

Let’s take a look at a slightly simplified definition of the `vec!` macro:

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

> Note: the actual definition of the `vec!` macro in the standard library also
> has code to pre-allocate the correct amount of memory up-front. That code
> is an optimization that we’ve chosen not to include here for simplicity.

The `#[macro_export]` annotation indicates that this macro should be made
available when other crates import the crate in which we’re defining this
macro. Without this annotation, even if someone depending on this crate uses
the `#[macro_use]` annotation, this macro would not be brought into scope.

Macro definitions start with `macro_rules!` and the name of the macro we’re
defining without the exclamation mark, which in this case is `vec`. This is
followed by curly brackets denoting the body of the macro definition.

Inside the body is a structure similar to the structure of a `match`
expression. This macro definition has one arm with the pattern `( $( $x:expr
),* )`, followed by `=>` and the block of code associated with this pattern. If
this pattern matches, then the block of code will be emitted. Given that this
is the only pattern in this macro, there’s only one valid way to match; any
other will be an error. More complex macros will have more than one arm.

The pattern syntax valid in macro definitions is different than the pattern
syntax covered in Chapter 18 because the patterns are for matching against Rust
code structure rather than values. Let’s walk through what the pieces of the
pattern used here mean; for the full macro pattern syntax, see the reference at
*https://doc.rust-lang.org/stable/reference/macros.html*.

The `$x:expr` part of the pattern matches any Rust expression and gives the
expression the name `$x`. The `*` specifies that the pattern matches zero or
more of whatever precedes the `*`. In this case, `*` is preceded by `$(),` so
this pattern matches zero or more of whatever is inside the parentheses,
delimited by a comma. When we call this macro with `vec![1, 2, 3];`, the
pattern matches the three expressions `1`, `2`, and `3`.

In the body of the code associated with this arm, the `$()*` part is generated
for each part that matches `$()` in the pattern, zero or more times depending
on how many times the pattern matches. The `$x` in the code associated with the
arm is replaced with each expression matched. When we call this macro with
`vec![1, 2, 3];`, the code generated that replaces this macro call will be:

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

## Procedural Macros for Custom `derive`

The second form of macros is called *procedural macros* because they’re more
like functions (which are a type of procedure). Procedural macros accept some
Rust code as an input, operate on that code, and produce some Rust code as an
output, rather than matching against patterns and replacing the code with other
code as declarative macros do. Today, the only thing you can define procedural
macros for is to allow your traits to be implemented on a type by specifying
the trait name in a `derive` annotation.

Let’s create a crate named `hello-world` that defines a trait named
`HelloWorld` with one associated function named `hello_world`. Rather than
making users of our crate implement the `HelloWorld` trait for each of their
types, we’d like users to be able to annotate their type with
`#[derive(HelloWorld)]` to get a default implementation of the `hello_world`
function associated with their type. The default implementation will print
`Hello world, my name is TypeName!` where `TypeName` is the name of the type on
which this trait has been defined.

In other words, we’re going to write a crate that enables another programmer to
write code that looks like Listing A4-1 using our crate:

Filename: src/main.rs

```
extern crate hello_world;
#[macro_use]
extern crate hello_world_derive;

use hello_world::HelloWorld;

#[derive(HelloWorld)]
struct Pancakes;

fn main() {
    Pancakes::hello_world();
}
```

Listing A4-1: The code a user of our crate will be able to write when we’ve
written the procedural macro

This code will print `Hello world, my name is Pancakes!` when we’re done. Let’s
get started!

Let’s make a new library crate:

```
$ cargo new hello-world
```

First, we’ll define the `HelloWorld` trait and associated function:

Filename: src/lib.rs

```
pub trait HelloWorld {
    fn hello_world();
}
```

At this point, a user of our crate could implement the trait themselves to
achieve the functionality we wanted to enable, like so:

```
extern crate hello_world;

use hello_world::HelloWorld;

struct Pancakes;

impl HelloWorld for Pancakes {
    fn hello_world() {
        println!("Hello world, my name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_world();
}
```

However, they would need to write out the implementation block for each type
they wanted to be able to use with `hello_world`; we’d like to make using our
trait more convenient for other programmers by saving them this work.

Additionally, we can’t provide a default implementation for the `hello_world`
function that has the behavior we want of printing out the name of the type the
trait is implemented on: Rust doesn’t have reflection capabilities, so we can’t
look up the type’s name at runtime. We need a macro to generate code at compile
time.

### Defining Procedural Macros Requires a Separate Crate

The next step is to define the procedural macro. At the moment, procedural
macros need to be in their own crate. Eventually, this restriction may be
lifted, but for now, it’s required. As such, there’s a convention: for a crate
named `foo`, a custom derive procedural macro crate is called `foo-derive`.
Let’s start a new crate called `hello-world-derive` inside our `hello-world`
project:

```
$ cargo new hello-world-derive
```

We’ve chosen to create the procedural macro crate within the directory of our
`hello-world` crate because the two crates are tightly related: if we change
the trait definition in `hello-world`, we’ll have to change the implementation
of the procedural macro in `hello-world-derive` as well. The two crates will
need to be published separately, and programmers using these crates will need
to add both as dependencies and bring them both into scope. It’s possible to
have the `hello-world` crate use `hello-world-derive` as a dependency and
re-export the procedural macro code, but structuring the project this way makes
it possible for programmers to easily decide they only want to use
`hello-world` if they don’t want the `derive` functionality.

We need to declare that the `hello-world-derive` crate is a procedural macro
crate. We also need to add dependencies on the `syn` and `quote` crates to get
useful functionality for operating on Rust code. To do these two things, add
the following to the *Cargo.toml* for `hello-world-derive`:

Filename: hello-world-derive/Cargo.toml

```
[lib]
proc-macro = true

[dependencies]
syn = "0.11.11"
quote = "0.3.15"
```

To start defining the procedural macro, place the code from Listing A4-2 in
*src/lib.rs* for the `hello-world-derive` crate. Note that this won’t compile
until we add a definition for the `impl_hello_world` function. We’ve split the
code into functions in this way because the code in Listing A4-2 will be the
same for almost every procedural macro crate; it’s code that makes writing a
procedural macro more convenient. What you choose to do in the place where the
`impl_hello_world` function is called will be different and depend on the
purpose of your procedural macro.

Filename: hello-world-derive/src/lib.rs

```
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
```

Listing A4-2: Code that most procedural macro crates will need to have for
processing Rust code

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

The `hello_world_derive` function is the code that will get called when a user
of our library specifies the `#[derive(HelloWorld)]` annotation on a type
because we’ve annotated the `hello_world_derive` function here with
`proc_macro_derive` and specified the same name, `HelloWorld`. This name
matches our trait named `HelloWorld`; that’s the convention most procedural
macros follow.

The first thing this function does is convert the `input` from a `TokenStream`
to a `String` by calling `to_string`. This `String` is a string representation
of the Rust code for which we are deriving `HelloWorld`. In the example in
Listing A4-1, `s` will have the `String` value `struct Pancakes;` because
that’s the Rust code we added the `#[derive(HelloWorld)]` annotation to.

At the moment, the only thing you can do with a `TokenStream` is convert it to
a string. A richer API will exist in the future.

What we really need is to be able to parse the Rust code `String` into a data
structure that we can then interpret and perform operations on. This is where
`syn` comes to play. The `parse_derive_input` function in `syn` takes a
`String` and returns a `DeriveInput` struct representing the parsed Rust code.
Here’s the relevant parts of the `DeriveInput` struct we get from parsing the
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

We haven’t defined the `impl_hello_world` function; that’s where we’ll build
the new Rust code we want to include. Before we get to that, the last part of
this `hello_world_derive` function is using the `quote` crate’s `parse`
function to turn the output of the `impl_hello_world` function back into a
`TokenStream`. The returned `TokenStream` is added to the code that users of
our crate write so that when they compile their crate, they get extra
functionality we provide.

You may have noticed that we’re calling `unwrap` to panic if the calls to the
`parse_derive_input` or `parse` functions fail because they’re unable to parse
the `TokenStream` or generate a `TokenStream`. Panicking on errors is necessary
in procedural macro code because `proc_macro_derive` functions must return
`TokenStream` rather than `Result` in order to conform to the procedural macro
API. We’ve chosen to keep this example simple by using `unwrap`; in production
code you should provide more specific error messages about what went wrong by
using `expect` or `panic!`.

Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `String` and into a `DeriveInput` instance, let’s write the code that
will generate the code implementing the `HelloWorld` trait on the annotated
type:

Filename: hello-world-derive/src/lib.rs

```
fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}
```

We are able to get an `Ident` struct instance containing the name (identifier)
of the annotated type using `ast.ident`. With the code from Listing A4-1,
`name` will be `Ident("Pancakes")`.

The `quote!` macro from the `quote` crate lets us write up the Rust code that
we wish to return and convert it into `quote::Tokens`. The `quote!` macro lets
us use some really cool templating mechanics; we can write `#name` and `quote!`
will replace it with the value in the variable named `name`. You can even do
some repetition similar to the way regular macros work. Check out the `quote`
crate’s docs at *https://docs.rs/quote* for a thorough introduction.

What we want to do for our procedural macro is generate an implementation of
our `HelloWorld` trait for the type the user of our crate has annotated, which
we can get by using `#name`. The trait implementation has one function,
`hello_world`, and the function body contains the functionality we want to
provide: printing `Hello, World! My name is` and then the name of the type the
user of our crate has annotated. The `stringify!` macro used here is built into
Rust. It takes a Rust expression, such as `1 + 2`, and at compile time turns
the expression into a string literal, such as `"1 + 2"`. This is different than
`format!` or `println!`, which evaluate the expression and then turn the result
into a `String`. There’s a possibility that `#name` would be an expression that
we would want to print out literally, and `stringify!` also saves an allocation
by converting `#name` to a string literal at compile time.

At this point, `cargo build` should complete successfully in both `hello-world`
and `hello-world-derive`. Let’s hook these crates up to the code in Listing
A4-1 to see it in action! Create a new binary project in your `projects`
directory with `cargo new --bin pancakes`. We need to add both `hello-world`
and `hello-world-derive` as dependencies in the `pancakes` crate’s
*Cargo.toml*. If you’ve chosen to publish your versions of `hello-world` and
`hello-world-derive` to *https://crates.io* they would be regular dependencies;
if not, you can specify them as `path` dependencies as follows:

```
[dependencies]
hello_world = { path = "../hello-world" }
hello_world_derive = { path = "../hello-world/hello-world-derive" }
```

Put the code from Listing A4-1 into *src/main.rs*, and executing `cargo run`
should print `Hello, World! My name is Pancakes`! The implementation of the
`HelloWorld` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloWorld)]` took care
of adding the trait implementation.

## The Future of Macros

In the future, we’ll be expanding both declarative and procedural macros. A
better declarative macro system will be used with the `macro` keyword, and
we’ll add more types of procedural macros, for more powerful tasks than only
`derive`. These systems are still under development at the time of publication;
please consult the online Rust documentation for the latest information.
