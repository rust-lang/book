# Modules

When a project starts getting large, it’s considered good software engineering
practice to split it up into a bunch of smaller pieces and then fit them
together. It’s also important to have a well-defined interface, so that some of
your functionality is private and some is public. Rust has a module system that
tackles both of these problems. A *module* is a namespace that contains
definitions of functions or types, and those definitions can be visible outside
their module or not. Here's an overview of how the bits fit together:

* `mod` declares a new module.
* Everything starts off as private, but the `pub` keyword makes it public.
* The `use` keyword allows you to bring modules, or definitions inside of them,
  into scope so that it's easier to refer to them.

We'll take a look at each of these parts and see how they fit into the whole.
Then, we'll take a look at the standard library and its module layout.
