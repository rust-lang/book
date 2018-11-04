# Packages, Crates, and Modules

A key question when writing programs is scope: what names do I know about?
What functions am I allowed to call? What does this variable refer to?

Rust has a number of features related to scopes. This is sometimes called
"the module system", but it encompases more than just modules:

* A *path* is a way of naming something.
* *Modules* and the *use keyword* let you control the scope and privacy of paths.
* *Crates* are a tree of modules that produce a library or executable.
* *Packages* are a Cargo feature that let you build, test, and share crates.

This chapter will cover all of these concepts. You'll be importing and
exporting things like a pro soon!