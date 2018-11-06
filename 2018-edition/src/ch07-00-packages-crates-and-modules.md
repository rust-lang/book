# Packages, Crates, and Modules

A key question when writing programs is *scope*: what names does the compiler
know about at this location in the code? What functions am I allowed to call?
What does this variable refer to?

Rust has a number of features related to scopes. This is sometimes called
“the module system”, but it encompases more than just modules:

* *Packages* are a Cargo feature that let you build, test, and share crates.
* *Crates* are a tree of modules that produce a library or executable.
* *Modules* and the *use* keyword let you control the scope and privacy of paths.
* A *path* is a way of naming an item such as a struct, function, or module.

This chapter will cover all of these concepts. You’ll be bringing names into
scopes, defining scopes, and exporting names to scopes like a pro soon!
