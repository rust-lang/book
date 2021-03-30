# Managing Growing Projects with Packages, Crates, and Modules

As you write large programs, organizing your code will be important because
keeping track of your entire program in your head will become impossible. By
grouping related functionality and separating code with distinct features,
you’ll clarify where to find code that implements a particular feature and
where to go to change how a feature works.

The programs we’ve written so far have been in one module in one file. As a
project grows, you can organize code by splitting it into multiple modules and
then multiple files. A package can contain multiple binary crates and
optionally one library crate. As a package grows, you can extract parts into
separate crates that become external dependencies. This chapter covers all
these techniques. For very large projects of a set of interrelated packages
that evolve together, Cargo provides workspaces, which we’ll cover in the
[“Cargo Workspaces”][workspaces]<!-- ignore --> section in Chapter 14.

In addition to grouping functionality, encapsulating implementation details
lets you reuse code at a higher level: once you’ve implemented an operation,
other code can call that code via the code’s public interface without knowing
how the implementation works. The way you write code defines which parts are
public for other code to use and which parts are private implementation details
that you reserve the right to change. This is another way to limit the amount
of detail you have to keep in your head.

A related concept is scope: the nested context in which code is written has a
set of names that are defined as “in scope.” When reading, writing, and
compiling code, programmers and compilers need to know whether a particular
name at a particular spot refers to a variable, function, struct, enum, module,
constant, or other item and what that item means. You can create scopes and
change which names are in or out of scope. You can’t have two items with the
same name in the same scope; tools are available to resolve name conflicts.

Rust has a number of features that allow you to manage your code’s
organization, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the *module system*, include:

* Packages and crates, features provided by Cargo that organize the Rust
  ecosystem into buildable, testable, and sharable units:
  * **Package:** A collection of functionally related crates
  * **Crate:** A tree of source files that comprises a single source library or
  a single executable
* Modules that are a part of the Rust language used to organize code and
  encapsulate implementation detail:
  * **Module**: A unit of related code with a specific scope path (a way of
    naming an item, such as a struct, function, or module), internal privacy,
    and external permission

In this chapter, we’ll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!

[workspaces]: ch14-03-cargo-workspaces.html

