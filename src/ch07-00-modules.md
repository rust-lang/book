# Modules

When you write a program in Rust, your code might start off living solely in
the `main` function. As your code grows, you eventually move functionality out
into functions, both for re-use and for nicer organization. By splitting your
code up into smaller chunks, each chunk is easier to understand on its own. So
what happens when you start having too many functions? Rust has a module system
that tackles both of these problems. In the same way that you extract some lines
of code into a function, you can extract some functions (and other things too)
into different modules. A *module* is a namespace that contains definitions of
functions or types, and those definitions can be visible outside their module
or not. Here's an overview of how the bits fit together:

* `mod` declares a new module.
* Everything starts off as private, but the `pub` keyword makes it public.
* The `use` keyword allows you to bring modules, or definitions inside of them,
  into scope so that it's easier to refer to them.

We'll take a look at each of these parts and see how they fit into the whole.
