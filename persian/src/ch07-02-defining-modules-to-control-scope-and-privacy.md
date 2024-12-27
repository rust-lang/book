## Defining Modules to Control Scope and Privacy

In this section, we’ll talk about modules and other parts of the module system,
namely _paths_, which allow you to name items; the `use` keyword that brings a
path into scope; and the `pub` keyword to make items public. We’ll also discuss
the `as` keyword, external packages, and the glob operator.

### Modules Cheat Sheet

Before we get to the details of modules and paths, here we provide a quick
reference on how modules, paths, the `use` keyword, and the `pub` keyword work
in the compiler, and how most developers organize their code. We’ll be going
through examples of each of these rules throughout this chapter, but this is a
great place to refer to as a reminder of how modules work.

- **Start from the crate root**: When compiling a crate, the compiler first
  looks in the crate root file (usually _src/lib.rs_ for a library crate or
  _src/main.rs_ for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules;
  say you declare a “garden” module with `mod garden;`. The compiler will look
  for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod
    garden`
  - In the file _src/garden.rs_
  - In the file _src/garden/mod.rs_
- **Declaring submodules**: In any file other than the crate root, you can
  declare submodules. For example, you might declare `mod vegetables;` in
  _src/garden.rs_. The compiler will look for the submodule’s code within the
  directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead
    of the semicolon
  - In the file _src/garden/vegetables.rs_
  - In the file _src/garden/vegetables/mod.rs_
- **Paths to code in modules**: Once a module is part of your crate, you can
  refer to code in that module from anywhere else in that same crate, as long
  as the privacy rules allow, using the path to the code. For example, an
  `Asparagus` type in the garden vegetables module would be found at
  `crate::garden::vegetables::Asparagus`.
- **Private vs. public**: Code within a module is private from its parent
  modules by default. To make a module public, declare it with `pub mod`
  instead of `mod`. To make items within a public module public as well, use
  `pub` before their declarations.
- **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to
  items to reduce repetition of long paths. In any scope that can refer to
  `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
  crate::garden::vegetables::Asparagus;` and from then on you only need to
  write `Asparagus` to make use of that type in the scope.

Here, we create a binary crate named `backyard` that illustrates these rules.
The crate’s directory, also named `backyard`, contains these files and
directories:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case is _src/main.rs_, and it contains:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

The `pub mod garden;` line tells the compiler to include the code it finds in
_src/garden.rs_, which is:

<Listing file-name="src/garden.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

</Listing>

Here, `pub mod vegetables;` means the code in _src/garden/vegetables.rs_ is
included too. That code is:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

Now let’s get into the details of these rules and demonstrate them in action!

### Grouping Related Code in Modules

_Modules_ let us organize code within a crate for readability and easy reuse.
Modules also allow us to control the _privacy_ of items because code within a
module is private by default. Private items are internal implementation details
not available for outside use. We can choose to make modules and the items
within them public, which exposes them to allow external code to use and depend
on them.

As an example, let’s write a library crate that provides the functionality of a
restaurant. We’ll define the signatures of functions but leave their bodies
empty to concentrate on the organization of the code rather than the
implementation of a restaurant.

In the restaurant industry, some parts of a restaurant are referred to as
_front of house_ and others as _back of house_. Front of house is where
customers are; this encompasses where the hosts seat customers, servers take
orders and payment, and bartenders make drinks. Back of house is where the
chefs and cooks work in the kitchen, dishwashers clean up, and managers do
administrative work.

To structure our crate in this way, we can organize its functions into nested
modules. Create a new library named `restaurant` by running `cargo new
restaurant --lib`. Then enter the code in Listing 7-1 into _src/lib.rs_ to
define some modules and function signatures; this code is the front of house
section.

<Listing number="7-1" file-name="src/lib.rs" caption="A `front_of_house` module containing other modules that then contain functions">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

We define a module with the `mod` keyword followed by the name of the module
(in this case, `front_of_house`). The body of the module then goes inside curly
brackets. Inside modules, we can place other modules, as in this case with the
modules `hosting` and `serving`. Modules can also hold definitions for other
items, such as structs, enums, constants, traits, and—as in Listing
7-1—functions.

By using modules, we can group related definitions together and name why
they’re related. Programmers using this code can navigate the code based on the
groups rather than having to read through all the definitions, making it easier
to find the definitions relevant to them. Programmers adding new functionality
to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that _src/main.rs_ and _src/lib.rs_ are called crate
roots. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the _module tree_.

Listing 7-2 shows the module tree for the structure in Listing 7-1.

<Listing number="7-2" caption="The module tree for the code in Listing 7-1">

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

</Listing>

This tree shows how some of the modules nest inside other modules; for example,
`hosting` nests inside `front_of_house`. The tree also shows that some modules
are _siblings_, meaning they’re defined in the same module; `hosting` and
`serving` are siblings defined within `front_of_house`. If module A is
contained inside module B, we say that module A is the _child_ of module B and
that module B is the _parent_ of module A. Notice that the entire module tree
is rooted under the implicit module named `crate`.

The module tree might remind you of the filesystem’s directory tree on your
computer; this is a very apt comparison! Just like directories in a filesystem,
you use modules to organize your code. And just like files in a directory, we
need a way to find our modules.
