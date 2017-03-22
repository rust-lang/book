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
* `false` - boolean false literal
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
* `true` - boolean true literal
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
