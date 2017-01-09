# Modules

When you start writing programs in Rust, your code might live solely in the
`main` function. As your code grows, you’ll eventually move functionality out
into other functions, both for re-use and for better organization. By splitting
your code up into smaller chunks, each chunk is easier to understand on its
own. But what happens if you find yourself with too many functions? Rust has a
module system that handles the problem of wanting to re-use code while
keeping your code organized.

In the same way that you extract lines of code into a function, you can extract
functions (and other code like structs and enums too) into different modules. A
*module* is a namespace that contains definitions of functions or types, and
you can choose whether those definitions are visible outside their module
(public) or not (private). Here’s an overview of how modules work:

* You declare a new module with the keyword `mod`
* By default, everything is set as private, but you can use the `pub` keyword
  to make the module public, and therefore visible outside of the namespace.
* The `use` keyword allows you to bring modules, or the definitions inside
  modules, into scope so that it’s easier to refer to them.

We’ll take a look at each of these parts and see how they fit into the whole.
