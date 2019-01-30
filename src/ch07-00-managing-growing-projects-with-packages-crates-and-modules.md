# Managing Growing Projects with Packages, Crates, and Modules

As you write larger programs, organizing your code is important because it’ll
become impossible to keep track of your entire program in your head at one
time. By grouping related functionality together and separating code with
distinct features apart, you’ll make it clear where to find code that
implements a particular feature and you’ll know where to go to change how a
feature works.

The programs we’ve written so far have been written in one module in one file.
As a project grows, you can organize code by splitting into multiple modules,
then multiple files. A package can contain multiple binary crates and
optionally one library crate. As a package grows, you can extract parts into
separate crates that become external dependencies. We’ll be covering all these
techniques in this chapter. For really large projects of a set of interrelated
packages that evolve together, Cargo provides the concept of *workspaces* that
we’ll cover in the [“Cargo Workspaces”][workspaces]<!-- ignore --> section of
Chapter 14.

In addition to grouping functionality, encapsulating implementation details
lets you reuse code at a higher level: once you’ve implemented an operation,
other code can call that code without knowing exactly how the implementation
works via the code’s public interface. The way you choose to write the code
defines what parts are public for other code to use and what parts are private
implementation details that you reserve the right to change and that other code
shouldn’t have to worry about. This is another way to limit the amount of
detail you have to keep in your head.

A related aspect to organization and encapsulation is *scope*: the nested
context code is written in that has a set of names that are defined as “in
scope.” When reading, writing, and compiling code, programmers and compilers
need to know whether a particular name at a particular spot refers to a
variable, function, struct, enum, module, constant, or other item, and what
that item means. We can create scopes and change what names are in or out of
scope. We’re not allowed to have two items with the same name in the same
scope; there are tools to resolve name conflicts.

Rust has a number of features that allow you to manage the organization of your
code, which details are exposed and which details are private, and what names
are in each scope in your programs. These features are sometimes collectively
referred to as “the module system.” They are:

<!-- Liz: these bullet points were in order from most general to least general,
so I've put them back in that order. Was there a reason you reordered them?
/Carol -->

* *Packages*, a Cargo feature that lets you build, test, and share crates.
* *Crates*, a tree of modules that produces a library or executable.
* *Modules* and *use*, which let you control the organization, scope, and
  privacy of paths.
* *Paths*, a way of naming an item such as a struct, function, or module.

In this chapter we’ll cover all of these features and discuss how they interact
and how they’re used to manage scope. By the end, you should have a solid
understanding of the module system, and be able to work with scopes like a pro!

[workspaces]: ch14-03-cargo-workspaces.html
