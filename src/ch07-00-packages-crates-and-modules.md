<!--- I liked this chapter. My only major note would be that the chapter opens
talking about scope, so I expected to see more references to scope throughout,
especially early on, though it only showed up in the latter half of the
chapter. I would go through some of the early sections and see whether they
mention scope, and if not, add some references to it to motivate the things
they are learning and keep that thread clear through the chapter. Good work!
--->

# Packages, Crates, and Modules

When writing programs, we have to think about *scope*: [definition of scope] So
when we look at a line of code, we should be thinking: what does the compiler
know here? What names does it recognize? What functions are we allowed to call?
What does this variable refer to?

<!-- I'm not sure what you mean by "features related to scope" -- don't all
features relate to scope in some way or another? Can you expand? Is "features
that allow you to manage scope" accurate? -->

Rust has a number of features related to scope, which are sometimes
collectively referred to as "the module system." They are:

* *Crates*, a tree of modules that produce a library or executable.
* *Packages*, a Cargo feature that let you build, test, and share crates.
* *Modules* and *use*, two keywords which let you control the scope and privacy
  of paths.
* *Paths*, a way of naming an item such as a struct, function, or module.

In this chapter we'll cover all of these features and discuss how they interact
and how they're used to manage scope. By the end, you should have a solid
understanding of the module system, and be able to work with scopes like a pro!
