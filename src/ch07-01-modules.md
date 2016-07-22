# Modules

When a project starts getting large, it’s considered good software engineering
practice to split it up into a bunch of smaller pieces, and then fit them
together. It’s also important to have a well-defined interface, so that some of
your functionality is private, and some is public. To facilitate these kinds of
things, Rust has a module system. It tackles both of these problems. Here's an
overview of how the bits fit together:

* `mod` declares a new module.
* Everything starts off as private, but the `pub` keyword makes it public.
* The `use` keyword allows you to bring modules into scope so that it's easier
  to refer to them.

We'll take a look at each of these parts and see how they fit into the whole.
Then, we'll take a look at the standard library and its module layout.
