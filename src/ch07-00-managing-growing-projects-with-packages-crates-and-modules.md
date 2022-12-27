# Managing Growing Projects with Packages, Crates, and Modules

As you write large programs, organizing your code will become increasingly
important. By grouping related functionality and separating code with distinct
features, you’ll clarify where to find code that implements a particular
feature and where to go to change how a feature works.

The programs we’ve written so far have been in one module in one file. As a
project grows, you should organize code by splitting it into multiple modules
and then multiple files. A package can contain multiple binary crates and
optionally one library crate. As a package grows, you can extract parts into
separate crates that become external dependencies. This chapter covers all
these techniques. For very large projects comprising a set of interrelated
packages that evolve together, Cargo provides *workspaces*, which we’ll cover
in the [“Cargo Workspaces”][workspaces]<!-- ignore --> section in Chapter 14.

We’ll also discuss encapsulating implementation details, which lets you reuse
code at a higher level: once you’ve implemented an operation, other code can
call your code via its public interface without having to know how the
implementation works. The way you write code defines which parts are public for
other code to use and which parts are private implementation details that you
reserve the right to change. This is another way to limit the amount of detail
you have to keep in your head.

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

* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and
  privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module

In this chapter, we’ll cover all these features, discuss how they interact, and
explain how to use them to manage scope. By the end, you should have a solid
understanding of the module system and be able to work with scopes like a pro!

[workspaces]: ch14-03-cargo-workspaces.html
