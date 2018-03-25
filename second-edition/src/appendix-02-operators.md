## Appendix B: Operators and Symbols

<!-- We try not to stack headings even in the appendix, can you add some intro
text about what this appendix contains? Quick example below -->
<!-- Done! /Carol -->

This appendix is a glossary of Rust’s syntax, including operators and other
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

The following lists all non-letters that don’t function as operators; that is,
they don’t behave like a function or method call.

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
