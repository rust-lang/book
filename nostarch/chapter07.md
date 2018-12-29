
[TOC]

# Packages, Crates, and Modules

A key question when writing programs is *scope*: what names does the compiler
know about at this location in the code? What functions am I allowed to call?
What does this variable refer to?

Rust has a number of features related to scopes. This is sometimes called
“the module system,” but it encompases more than just modules:

* *Packages* are a Cargo feature that let you build, test, and share crates.
* *Crates* are a tree of modules that produce a library or executable.
* *Modules* and the *use* keyword let you control the scope and privacy of paths.
* A *path* is a way of naming an item such as a struct, function, or module.

This chapter will cover all of these concepts. You’ll be bringing names into
scopes, defining scopes, and exporting names to scopes like a pro soon!

## Packages and Crates for Making Libraries and Executables

Let’s talk about *packages* and *crates*. Here’s a summary:

* A *crate* is a binary or library.
* The *crate root* is a source file that is used to know how to build a crate.
* A *package* has a *Cargo.toml* that describes how to build one or more crates.
  At most one crate in a package can be a library.

So when we type `cargo new`, we’re creating a package:

```
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Because Cargo created a *Cargo.toml*, that means we now have a package. If we
look at the contents of *Cargo.toml*, there’s no mention of *src/main.rs*.
However, Cargo’s conventions are that if you have a *src* directory containing
*main.rs* in the same directory as a package’s *Cargo.toml*, Cargo knows this
package contains a binary crate with the same name as the package, and
*src/main.rs* is its crate root. Another convention of Cargo’s is that if the
package directory contains *src/lib.rs*, the package contains a library crate
with the same name as the package, and *src/lib.rs* is its crate root. The
crate root files are passed by Cargo to `rustc` to actually build the library
or binary.

A package can contain zero or one library crates and as many binary crates as
you’d like. There must be at least one crate (either a library or a binary) in
a package.

If a package contains both *src/main.rs* and *src/lib.rs*, then it has two
crates: a library and a binary, both with the same name. If we only had one of
the two, the package would have either a single library or binary crate. A
package can have multiple binary crates by placing files in the *src/bin*
directory: each file will be a separate binary crate.

Next, let’s talk about modules!

## The Module System to Control Scope and Privacy

Rust has a feature that’s often referred to as “the module system,” but it
encompasses a few more features than modules. In this section, we’ll talk about:

* Modules, a way to organize code and control the privacy of paths
* Paths, a way to name items
* `use` a keyword to bring a path into scope
* `pub`, a keyword to make items public
* Renaming items when bringing them into scope with the `as` keyword
* Using external packages
* Nested paths to clean up large `use` lists
* Using the glob operator to bring everything in a module into scope
* How to split modules into individual files

First up, modules. Modules let us organize code into groups. Listing 7-1 has an
example of some code that defines a module named `sound` that contains a
function named `guitar`.

Filename: src/main.rs

```
mod sound {
    fn guitar() {
        // Function body code goes here
    }
}

fn main() {

}
```

Listing 7-1: A `sound` module containing a `guitar` function and a `main`
function

We’ve defined two functions, `guitar` and `main`. We’ve defined the `guitar`
function within a `mod` block. This block defines a module named `sound`.

To organize code into a hierarchy of modules, you can nest modules inside of
other modules, as shown in Listing 7-2:

Filename: src/main.rs

```
mod sound {
    mod instrument {
        mod woodwind {
            fn clarinet() {
                // Function body code goes here
            }
        }
    }

    mod voice {

    }
}

fn main() {

}
```

Listing 7-2: Modules inside modules

In this example, we defined a `sound` module in the same way as we did in
Listing 7-1. We then defined two modules within the `sound` module named
`instrument` and `voice`. The `instrument` module has another module defined
within it, `woodwind`, and that module contains a function named `clarinet`.

We mentioned in the “Packages and Crates for Making Libraries and Executables”
section that *src/main.rs* and *src/lib.rs* are called *crate roots*. They are
called crate roots because the contents of either of these two files form a
module named `crate` at the root of the crate’s module tree. So in Listing 7-2,
we have a module tree that looks like Listing 7-3:

```text
crate
└── sound
    ├── instrument
    │   └── woodwind
    └── voice
```

Listing 7-3: The module tree for the code in Listing 7-2

This tree shows how some of the modules nest inside one another (such as
`woodwind` nests inside `instrument`) and how some modules are siblings to
each other (`instrument` and `voice` are both defined within `sound`). The
entire module tree is rooted under the implicit module named `crate`.

This tree might remind you of the directory tree of the filesystem you have on
your computer; this is a very apt comparison! Just like directories in a
filesystem, you place code inside whichever module will create the organization
you’d like. Another similarity is that to refer to an item in a filesystem or a
module tree, you use its *path*.

### Paths for Referring to an Item in the Module Tree

If we want to call a function, we need to know its *path*. “Path” is a synonym
for “name” in a way, but it evokes that filesystem metaphor. Additionally,
functions, structs, and other items may have multiple paths that refer to the
same item, so “name” isn’t quite the right concept.

A *path* can take two forms:

* An *absolute path* starts from a crate root by using a crate name or a
  literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or
  an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).

How do we call the `clarinet` function in the `main` function in Listing 7-2?
That is, what’s the path of the `clarinet` function? In Listing 7-4, let’s
simplify our code a bit by removing some of the modules, and we’ll show two
ways to call the `clarinet` function from `main`. This example won’t compile
just yet, we’ll explain why in a bit.

Filename: src/main.rs

```
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Listing 7-4: Calling the `clarinet` function in a simplified module tree from
the `main` function using absolute and relative paths

The first way we’re calling the `clarinet` function from the `main` function
uses an absolute path. Because `clarinet` is defined within the same crate as
`main`, we use the `crate` keyword to start an absolute path. Then we include
each of the modules until we make our way to `clarinet`. This is similar to
specifying the path `/sound/instrument/clarinet` to run the program at that
location on your computer; using the `crate` name to start from the crate root
is like using `/` to start from the filesystem root in your shell.

The second way we’re calling the `clarinet` function from the `main` function
uses a relative path. The path starts with the name `sound`, a module defined
at the same level of the module tree as the `main` function. This is similar to
specifying the path `sound/instrument/clarinet` to run the program at that
location on your computer; starting with a name means that the path is relative.

We mentioned that Listing 7-4 won’t compile yet, let’s try to compile it and
find out why not! The error we get is shown in Listing 7-5.

```
$ cargo build
   Compiling sampleproject v0.1.0 (file:///projects/sampleproject)
error[E0603]: module `instrument` is private
  --> src/main.rs:11:19
   |
11 |     crate::sound::instrument::clarinet();
   |                   ^^^^^^^^^^

error[E0603]: module `instrument` is private
  --> src/main.rs:14:12
   |
14 |     sound::instrument::clarinet();
   |            ^^^^^^^^^^
```

Listing 7-5: Compiler errors from building the code in Listing 7-4

The error messsages say that module `instrument` is private. We can see that we
have the correct paths for the `instrument` module and the `clarinet` function,
but Rust won’t let us use them because they’re private. It’s time to learn
about the `pub` keyword!

### Modules as the Privacy Boundary

Earlier, we talked about the syntax of modules and that they can be used for
organization. There’s another reason Rust has modules: modules are the *privacy
boundary* in Rust. If you want to make an item like a function or struct
private, you put it in a module. Here are the privacy rules:

* All items (functions, methods, structs, enums, modules, annd constants) are
  private by default.
* You can use the `pub` keyword to make an item public.
* You aren’t allowed to use private code defined in modules that are children
  of the current module.
* You are allowed to use any code defined in ancestor modules or the current
  module.

In other words, items without the `pub` keyword are private as you look “down”
the module tree from the current module, but items without the `pub` keyword
are public as you look “up” the tree from the current module. Again, think of a
filesystem: if you don’t have permissions to a directory, you can’t look into
it from its parent directory. If you do have permissions to a directory, you
can look inside it and any of its ancestor directories.

### Using the `pub` Keyword to Make Items Public

The error in Listing 7-5 said the `instrument` module is private. Let’s mark
the `instrument` module with the `pub` keyword so that we can use it from the
`main` function. This change is shown in Listing 7-6, which still won’t
compile, but we’ll get a different error:

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Listing 7-6: Declaring the `instrument` module as `pub` so that we’re allowed
to use it from `main`

Adding the `pub` keyword in front of `mod instrument` makes the module public.
With this change, if we’re allowed to access `sound`, we can access
`instrument`. The contents of `instrument` are still private; making the module
public does not make its contents public. The `pub` keyword on a module lets
code in its parent module refer to it.

The code in Listing 7-6 still results in an error, though, as shown in Listing
7-7:

```
$ cargo build
   Compiling sampleproject v0.1.0 (file:///projects/sampleproject)
error[E0603]: function `clarinet` is private
  --> src/main.rs:11:31
   |
11 |     crate::sound::instrument::clarinet();
   |                               ^^^^^^^^

error[E0603]: function `clarinet` is private
  --> src/main.rs:14:24
   |
14 |     sound::instrument::clarinet();
   |                        ^^^^^^^^
```

Listing 7-7: Compiler errors from building the code in Listing 7-6

The errors now say that the `clarinet` function is private. The privacy rules
apply to structs, enums, functions, and methods as well as modules.

Let’s make the `clarinet` function public as well by adding the `pub` keyword
before its definition, as shown in Listing 7-8:

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Listing 7-8: Adding the `pub` keyword to both `mod instrument` and `fn
clarinet` lets us call the function from `main`

This will now compile! Let’s look at both the absolute and the relative path
and double check why adding the `pub` keyword lets us use these paths in `main`.

In the absolute path case, we start with `crate`, the root of our crate. From
there, we have `sound`, and it is a module that is defined in the crate root.
The `sound` module isn’t public, but because the `main` function is defined in
the same module that `sound` is defined, we’re allowed to refer to `sound` from
`main`. Next is `instrument`, which is a module marked with `pub`. We can
access the parent module of `instrument`, so we’re allowed to access
`instrument`. Finally, `clarinet` is a function marked with `pub` and we can
access its parent module, so this function call works!

In the relative path case, the logic is the same as the absolute path except
for the first step. Rather than starting from the crate root, the path starts
from `sound`. The `sound` module is defined within the same module as `main`
is, so the relative path starting from the module in which `main` is defined
works. Then because `instrument` and `clarinet` are marked with `pub`, the rest
of the path works and this function call is valid as well!

### Starting Relative Paths with `super`

You can also construct relative paths beginning with `super`. Doing so is like
starting a filesystem path with `..`: the path starts from the *parent* module,
rather than the current module. This is useful in situations such as the
example in Listing 7-9, where the function `clarinet` calls the function
`breathe_in` by specifying the path to `breathe_in` start with `super`:

Filename: src/lib.rs

```
mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // Function body code goes here
}
```

Listing 7-9: Calling a function using a relative path starting with `super` to
look in the parent module

The `clarinet` function is in the `instrument` module, so we can use `super` to
go to the parent module of `instrument`, which in this case is `crate`, the
root. From there, we look for `breathe_in`, and find it. Success!

The reason you might want to choose a relative path starting with `super`
rather than an absolute path starting with `crate` is that using `super` may
make it easier to update your code to have a different module hierarchy, if the
code defining the item and the code calling the item are moved together. For
example, if we decide to put the `instrument` module and the `breathe_in`
function into a module named `sound`, we would only need to add the `sound`
module, as shown in Listing 7-10.

Filename: src/lib.rs

```
mod sound {
    mod instrument {
        fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {
        // Function body code goes here
    }
}
```

Listing 7-10: Adding a parent module named `sound` doesn’t affect the relative
path `super::breathe_in`

The call to `super::breathe_in` from the `clarinet` function will continue to
work in Listing 7-10 as it did in Listing 7-9, without needing to update the
path. If instead of `super::breathe_in` we had used `crate::breathe_in` in the
`clarinet` function, when we add the parent `sound` module, we would need to
update the `clarinet` function to use the path `crate::sound::breathe_in`
instead. Using a relative path can mean fewer updates are necessary when
rearranging modules.

### Using `pub` with Structs and Enums

You can designate structs and enums to be public in a similar way as we’ve
shown with modules and functions, with a few additional details.

If you use `pub` before a struct definition, you make the struct public.
However, the struct’s fields are still private. You can choose to make each
field public or not on a case-by-case basis. In Listing 7-11, we’ve defined a
public `plant::Vegetable` struct with a public `name` field but a private `id`
field.

Filename: src/main.rs

```
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
```

Listing 7-11: A struct with some public fields and some private fields

Because the `name` field of the `plant::Vegetable` struct is public, in `main`
we can write and read to the `name` field by using dot notation. We’re not
allowed to use the `id` field in `main` because it’s private. Try uncommenting
the line printing the `id` field value to see what error you get! Also note
that because `plant::Vegetable` has a private field, the struct needs to
provide a public associated function that constructs an instance of `Vegetable`
(we’ve used the conventional name `new` here). If `Vegetable` didn’t have such
a function, we wouldn’t be able to create an instance of `Vegetable` in `main`
because we’re not allowed to set the value of the private `id` field in `main`.

In contrast, if you make a public enum, all of its variants are public. You
only need the `pub` before the `enum` keyword, as shown in Listing 7-12.

Filename: src/main.rs

```
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
```

Listing 7-12: Designating an enum as public makes all its variants public

Because we made the `Appetizer` enum public, we’re able to use the `Soup` and
`Salad` variants in `main`.

There’s one more situation involving `pub` that we haven’t covered, and that’s
with our last module system feature: the `use` keyword. Let’s cover `use` by
itself, and then we’ll show how `pub` and `use` can be combined.

### The `use` Keyword to Bring Paths into a Scope

You may have been thinking that many of the paths we’ve written to call
functions in the listings in this chapter are long and repetitive. For example,
in Listing 7-8, whether we chose the absolute or relative path to the
`clarinet` function, every time we wanted to call `clarinet` we had to specify
`sound` and `instrument` too. Luckily, there’s a way to bring a path into a
scope once and then call the items in that path as if they’re local items: with
the `use` keyword. In Listing 7-13, we bring the `crate::sound::instrument`
module into the scope of the `main` function so that we only have to specify
`instrument::clarinet` to call the `clarinet` function in `main`.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Listing 7-13: Bringing a module into scope with `use` and an absolute path to
shorten the path we have to specify to call an item within that module

Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::sound::instrument` in the crate root,
`instrument` is now a valid name in that scope as if the `instrument` module
had been defined in the crate root. We can now reach items in the `instrument`
module through the older, full paths, or we can reach items through the new,
shorter path that we’ve created with `use`. Paths brought into scope with `use`
also check privacy, like any other paths.

If you want to bring an item into scope with `use` and a relative path, there’s
a small difference from directly calling the item using a relative path:
instead of starting from a name in the current scope, you must start the path
given to `use` with `self`. Listing 7-14 shows how to specify a relative path
to get the same behavior as Listing 7-13 that used an absolute path.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use self::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Listing 7-14: Bringing a module into scope with `use` and a relative path
starting with `self`

Starting relative paths with `self` when specified after `use` might not be
neccesary in the future; it’s an inconsistency in the language that people are
working on eliminating.

Choosing to specify absolute paths with `use` can make updates easier if the
code calling the items moves to a different place in the module tree but the
code defining the items does not, as opposed to when they moved together in the
changes we made in Listing 7-10. For example, if we decide to take the code
from Listing 7-13, extract the behavior in the `main` function to a function
called `clarinet_trio`, and move that function into a module named
`performance_group`, the path specified in `use` wouldn’t need to change, as
shown in Listing 7-15.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

mod performance_group {
    use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();
}
```

Listing 7-15: The absolute path doesn’t need to be updated when moving the code
that calls the item

In contrast, if we made the same change to the code in Listing 7-14 that
specifies a relative path, we would need to change `use
self::sound::instrument` to `use super::sound::instrument`. Choosing whether
relative or absolute paths will result in fewer updates can be a guess if
you’re not sure how your module tree will change in the future, but your
authors tend to specify absolute paths starting with `crate` because code
defining and calling items is more likely to be moved around the module tree
independently of each other, rather than together as we saw in Listing 7-10.

### Idiomatic `use` Paths for Functions vs. Other Items

In Listing 7-13, you may have wondered why we specified `use
crate::sound::instrument` and then called `instrument::clarinet` in `main`,
rather than the code shown in Listing 7-16 that has the same behavior:

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument::clarinet;

fn main() {
    clarinet();
    clarinet();
    clarinet();
}
```

Listing 7-16: Bringing the `clarinet` function into scoope with `use`, which is
unidiomatic

For functions, it’s considered idiomatic to specify the function’s parent
module with `use`, and then specify the parent module when calling the
function. Doing so rather than specifying the path to the function with `use`,
as Listing 7-16 does, makes it clear that the function isn’t locally defined,
while still minimizing repetition of the full path.

For structs, enums, and other items, specifying the full path to the item with
`use` is idiomatic. For example, Listing 7-17 shows the idiomatic way to bring
the standard library’s `HashMap` struct into scope.

Filename: src/main.rs

```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Listing 7-17: Bringing `HashMap` into scope in an idiomatic way

In contrast, the code in Listing 7-18 that brings the parent module of
`HashMap` into scope would not be considered idiomatic. There’s not a strong
reason for this idiom; this is the convention that has emerged and folks have
gotten used to reading and writing.

Filename: src/main.rs

```
use std::collections;

fn main() {
    let mut map = collections::HashMap::new();
    map.insert(1, 2);
}
```

Listing 7-18: Bringing `HashMap` into scope in an unidiomatic way

The exception to this idiom is if the `use` statements would bring two items
with the same name into scope, which isn’t allowed. Listing 7-19 shows how to
bring two `Result` types that have different parent modules into scope and
refer to them.

Filename: src/lib.rs

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
}
fn function2() -> io::Result<()> {
}
```

Listing 7-19: Bringing two types with the same name into the same scope
requires using their parent modules

If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d
have two `Result` types in the same scope and Rust wouldn’t know which one we
meant when we used `Result`. Try it and see what compiler error you get!

### Renaming Types Brought Into Scope with the `as` Keyword

There’s another solution to the problem of bringing two types of the same name
into the same scope: we can specify a new local name for the type by adding
`as` and a new name after the `use`. Listing 7-20 shows another way to write
the code from Listing 7-19 by renaming one of the two `Result` types using `as`.

Filename: src/lib.rs

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
}
fn function2() -> IoResult<()> {
}
```

Listing 7-20: Renaming a type when it’s brought into scope with the `as` keyword

In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won’t conflict with the `Result` from `std::fmt`
that we’ve also brought into scope. This is also considered idiomatic; choosing
between the code in Listing 7-19 and Listing 7-20 is up to you.

### Re-exporting Names with `pub use`

When you bring a name into scope with the `use` keyword, the name being
available in the new scope is private. If you want to enable code calling your
code to be able to refer to the type as if it was defined in that scope just as
your code does, you can combine `pub` and `use`. This technique is called
*re-exporting* because you’re bringing an item into scope but also making that
item available for others to bring into their scope.

For example, Listing 7-21 shows the code from Listing 7-15 with the `use`
within the `performance_group` module changed to `pub use`.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
```

Listing 7-21: Making a name available for any code to use from a new scope with
`pub use`

By using `pub use`, the `main` function can now call the `clarinet` function
through this new path with `performance_group::instrument::clarinet`. If we
hadn’t specified `pub use`, the `clarinet_trio` function can call
`instrument::clarinet` in its scope but `main` wouldn’t be allowed to take
advantage of this new path.

### Using External Packages

In Chapter 2, we programmed a guessing game. That project used an external
package, `rand`, to get random numbers. To use `rand` in our project, we added
this line to *Cargo.toml*:

Filename: Cargo.toml

```
[dependencies]
rand = "0.5.5"
```

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and its dependencies from *https://crates.io* and make its code
available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the package, `rand`, and listing the items
we wanted to bring into scope. Recall that in the “Generating a Random Number”
section in Chapter 2, we brought the `Rng` trait into scope and called the
`rand::thread_rng` function:

```
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

There are many packages that members of the community have published on
*https://crates.io*, and pulling any of them in to your package involves these
same steps: listing them in your package’s *Cargo.toml* and bringing items
defined in them into a scope in your package with `use`.

Note that the standard library (`std`) is also a crate that’s external to your
package. Because the standard library is shipped with the Rust language, you
don’t need to change *Cargo.toml* to include `std`, but you refer to it in
`use` to bring items the standard library defines into your package’s scope,
such as with `HashMap`:

```
use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library
crate.

### Nested Paths for Cleaning Up Large `use` Lists

When you use many items defined by the same package or in the same module,
listing each item on its own line can take up a lot of vertical space in your
files. For example, these two `use` statements we had in Listing 2-4 in the
Guessing Game both bring items from `std` into scope:

Filename: src/main.rs

```
use std::cmp::Ordering;
use std::io;
// ---snip---
```

We can use nested paths to bring the same items into scope in one line instead
of two, by specifying the common part of the path, then two colons, then curly
brackets around a list of the parts of the paths that differ, as shown in
Listing 7-22.

Filename: src/main.rs

```
use std::{cmp::Ordering, io};
// ---snip---
```

Listing 7-22: Specifying a nested path to bring multiple
items with the same prefix into scope in one line instead of two

In programs bringing many items into scope from the same package or module,
using nested paths can reduce the number of separate `use` statements needed by
a lot!

We can also deduplicate paths where one path is completely shared with part of
another path. For example, Listing 7-23 shows two `use` statements: one that
brings `std::io` into scope, and one that brings `std::io::Write` into scope:

Filename: src/lib.rs

```
use std::io;
use std::io::Write;
```

Listing 7-23: Bringing two paths into scope in two `use` statements where one
is a sub-path of the other

The common part between these two paths is `std::io`, and that’s the complete
first path. To deduplicate these two paths into one `use` statement, we can use
`self` in the nested path as shown in Listing 7-24.

Filename: src/lib.rs

```
use std::io::{self, Write};
```

Listing 7-24: Deduplicating the paths from Listing 7-23 into one `use` statement

This brings both `std::io` and `std::io::Write` into scope.

### Bringing All Public Definitions into Scope with the Glob Operator

If you’d like to bring *all* public items defined in a path into scope, you can
use specify that path followed by `*`, the glob operator:

```
use std::collections::*;
```

This `use` statements brings all public items defined in `std::collections`
into the current scope.

Be careful with using the glob operator! It makes it harder to tell what names
are in scope and where a name your program uses was defined.

The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the “How to Write Tests”
section of Chapter 11. The glob operator is also sometimes used as part of the
prelude pattern; see the standard library
documentation at *../../std/prelude/index.html#other-preludes* for more
information on that pattern.

### Separating Modules into Different Files

All of the examples in this chapter so far defined multiple modules in one
file. When modules get large, you may want to move their definitions to a
separate file to make the code easier to navigate.

For example, if we started from the code in Listing 7-8, we can move the
`sound` module to its own file *src/sound.rs* by changing the crate root file
(in this case, *src/main.rs*) to contain the code shown in Listing 7-25.

Filename: src/main.rs

```
mod sound;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Listing 7-25: Declaring the `sound` module whose body will be in *src/sound.rs*

And *src/sound.rs* gets the definitions from the body of the `sound` module,
shown in Listing 7-26.

Filename: src/sound.rs

```
pub mod instrument {
    pub fn clarinet() {
        // Function body code goes here
    }
}
```

Listing 7-26: Definitions inside the `sound` module in *src/sound.rs*

Using a semicolon after `mod sound` instead of a block tells Rust to load the
contents of the module from another file with the same name as the module.

To continue with our example and extract the `instrument` module to its own
file as well, we change *src/sound.rs* to contain only the declaration of the
`instrument` module:

Filename: src/sound.rs

```
pub mod instrument;
```

Then we create a *src/sound* directory and a file *src/sound/instrument.rs* to
contain the definitions made in the `instrument` module:

Filename: src/sound/instrument.rs

```
pub fn clarinet() {
    // Function body code goes here
}
```

The module tree remains the same and the function calls in `main` continue to
work without any modification, even though the definitions live in different
files. This lets you move modules to new files as they grow in size.

## Summary

Rust provides ways to organize your packages into crates, your crates into
modules, and to refer to items defined in one module from another by specifying
absolute or relative paths. These paths can be brought into a scope with a
`use` statement so that you can use a shorter path for multiple uses of the
item in that scope. Modules define code that’s private by default, but you can
choose to make definitions public by adding the `pub` keyword.

Next, we’ll look at some collection data structures in the standard library
that you can use in your nice, neat code.
