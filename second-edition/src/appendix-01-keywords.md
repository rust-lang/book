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
