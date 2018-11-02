## Appendix A: Keywords

The following list contains keywords that are reserved for current or future
use by the Rust language. As such, they cannot be used as identifiers (except as
[raw identifiers][raw-identifiers]), including names of functions, variables,
parameters, struct fields, modules, crates, constants, macros, static values,
attributes, types, traits, or lifetimes.

### Keywords Currently in Use

The following keywords currently have the functionality described.

* `as` - perform primitive casting, disambiguate the specific trait containing
  an item, or rename items in `use` and `extern crate` statements
* `break` - exit a loop immediately
* `const` - define constant items or constant raw pointers
* `continue` - continue to the next loop iteration
* `crate` - link an external crate or a macro variable representing the crate in
  which the macro is defined
* `dyn` - dynamic dispatch to a trait object
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

### Keywords Reserved for Future Use

The following keywords do not have any functionality but are reserved by Rust
for potential future use.

* `abstract`
* `async`
* `become`
* `box`
* `do`
* `final`
* `macro`
* `override`
* `priv`
* `try`
* `typeof`
* `unsized`
* `virtual`
* `yield`

### Raw identifiers
[raw-identifiers]: #raw-identifiers

Raw identifiers let you use keywords where they would not normally be allowed by
prefixing them with `r#`.

For example, `match` is a keyword. If you try to compile this function:

```rust,ignore
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

You’ll get this error:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

You can write this with a raw identifier:

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Note the `r#` prefix on both the function name as well as the call.

#### Motivation

This feature is useful for a few reasons, but the primary motivation was
inter-edition situations. For example, `try` is not a keyword in the 2015
edition, but is in the 2018 edition. So if you have a library that is written
in Rust 2015 and has a `try` function, to call it in Rust 2018, you’ll need
to use the raw identifier.
