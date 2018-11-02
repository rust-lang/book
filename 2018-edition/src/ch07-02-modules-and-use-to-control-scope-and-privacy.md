## The Module System to Control Scope and Privacy

Rust has a feature that's often referred to as "the module system," but
it encompasses a few more features than just modules. In this section,
we'll talk about:

* Modules, a way to organize code and control the privacy of paths
* Paths, a way to name items
* `use` a keyword to bring a path into scope
* `pub`, a keyword to make items public
* Re-naming imports with the `as` keyword
* Using external packages
* Nested imports to clean up large import lists
* Glob imports with `*` to bring everything in a module into scope
* How to split modules into individual files

First up, modules. Modules let us organize code into groups. Listing 7-1 has an
example of some code that defines a module named `sound` that contains a
function named `guitar`.

<span class="filename">Filename: src/main.rs</span>

```rust
mod sound {
    fn guitar() {
        // Function body code goes here
    }
}

fn main() {

}
```

<span class="caption">Listing 7-1: A `sound` module containing a `guitar`
function and a `main` function</span>

We've defined two functions, `guitar` and `main`. We've defined the `guitar`
function within a `mod` block. This block defines a module named `sound`.

To organize code into a hierarchy of modules, you can nest modules inside of
other modules, as shown in Listing 7-2:

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 7-2: Modules inside modules</span>

In this example, we defined a `sound` module in the same way as we did in
Listing 7-1. We then defined two modules within the `sound` module named
`instrument` and `voice`. The `instrument` module has another module defined
within it, `woodwind`, and that module contains a function named `clarinet`.

We mentioned in the "Packages and Crates for Making Libraries and Executables"
section that *src/main.rs* and *src/lib.rs* are called *crate roots*. They are
called crate roots because the contents of either of these two files form a
module named `crate` at the root of the crate's module tree. So in Listing 7-2,
we have a module tree that looks like Listing 7-3:

```text
crate
 └── sound
     └── instrument
        └── woodwind
     └── voice
```

<span class="caption">Listing 7-3: The module tree for the code in Listing
7-2</span>

This tree shows how some of the modules nest inside one another (such as
`woodwind` nests inside `instrument`) and how some modules are siblings to
each other (`instrument` and `voice` are both defined within `sound`). The
entire module tree is rooted under the implicit module named `crate`.

This tree might remind you of the directory tree of the filesystem you have on
your computer; this is a very apt comparison! Just like directories in a
filesystem, you place code inside whichever module will create the organization
you'd like. Another similarity is that to refer to an item in a filesystem or a
module tree, you use its *path*.

### Paths for Referring to an Item in the Module Tree

If we want to call a function, we need to know its *path*. "Path" is a synonym
for "name" in a way, but it evokes that filesystem metaphor. Additionally,
functions, structs, and other items may have multiple paths that refer to the
same item, so "name" isn't quite the right concept.

A *path* can take two forms:

* An *absolute path* starts from a crate root by using a crate name or a
  literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or
  an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).

How do we call the `clarinet` function in the `main` function in Listing 7-2?
That is, what's the path of the `clarinet` function? In Listing 7-4, let's
simplify our code a bit by removing some of the modules, and we'll show two
ways to call the `clarinet` function from `main`. This example won't compile
just yet, we'll explain why in a bit.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
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

<span class="caption">Listing 7-4: Calling the `clarinet` function in a
simplified module tree from the `main` function using absolute and relative
paths</span>

The first way we're calling the `clarinet` function from the `main` function
uses an absolute path. Because `clarinet` is defined within the same crate as
`main`, we use the `crate` keyword to start an absolute path. Then we include
each of the modules until we make our way to `clarinet`. This is similar to
specifying the path `/sound/instrument/clarinet` to run the program at that
location on your computer; using the `crate` name to start from the crate root
is like using `/` to start from the filesystem root in your shell.

The second way we're calling the `clarinet` function from the `main` function
uses a relative path. The path starts with the name `sound`, a module defined
at the same level of the module tree as the `main` function. This is similar to
specifying the path `sound/instrument/clarinet` to run the program at that
location on your computer; starting with a name means that the path is relative.

We mentioned that Listing 7-4 won't compile yet, let's try to compile it and
find out why not! The error we get is shown in Listing 7-5.

```text
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

<span class="caption">Listing 7-5: Compiler errors from building the code in
Listing 7-4</span>

The error messsages say that module `instrument` is private. We can see that we
have the correct paths for the `instrument` module and the `clarinet` function,
but Rust won't let us use them because they're private. It's time to learn
about the `pub` keyword!

### Modules as the Privacy Boundary

Earlier, we talked about the syntax of modules and that they can be used for
organization. There's another reason Rust has modules: modules are the *privacy
boundary* in Rust. If you want to make an item like a function or struct
private, you put it in a module. Here are the privacy rules:

* All items (functions, methods, structs, enums, modules, annd constants) are
  private by default.
* You can use the `pub` keyword to make an item public.
* You aren't allowed to use private code defined in modules that are children
  of the current module.
* You are allowed to use any code defined in ancestor modules or the current
  module.

In other words, items without the `pub` keyword are private as you look "down"
the module tree from the current module, but items without the `pub` keyword
are public as you look "up" the tree from the current module. Again, think of a
filesystem: if you don't have permissions to a directory, you can't look into
it from its parent directory. If you do have permissions to a directory, you
can look inside it and any of its ancestor directories.

#### Using the `pub` Keyword to Make Items Public

The error in Listing 7-5 said the `instrument` module is private. Let's mark
the `instrument` module with the `pub` keyword so that we can use it from the
`main` function. This change is shown in Listing 7-6, which still won't
compile, but we'll get a different error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
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

<span class="caption">Listing 7-6: Declaring the `instrument` module as `pub`
so that we're allowed to use it from `main`</span>

Adding the `pub` keyword in front of `mod instrument` makes the module public.
With this change, if we're allowed to access `sound`, we can access
`instrument`. The contents of `instrument` are still private; making the module
public does not make its contents public. The `pub` keyword on a module lets
code in its parent module refer to it.

The code in Listing 7-6 still results in an error, though, as shown in Listing
7-7:

```text
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

<span class="caption">Listing 7-7: Compiler errors from building the code in
Listing 7-6</span>

The errors now say that the `clarinet` function is private. The privacy rules
apply to structs, enums, functions, and methods as well as modules.

Let's make the `clarinet` function public as well by adding the `pub` keyword
before its definition, as shown in Listing 7-8:

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 7-8: Adding the `pub` keyword to both `mod
instrument` and `fn clarinet` lets us call the function from `main`</span>

This will now compile! Let's look at both the absolute and the relative path
and double check why adding the `pub` keyword lets us use these paths in `main`.

In the absolute path case, we start with `crate`, the root of our crate. From
there, we have `sound`, and it is a module that is defined in the crate root.
The `sound` module isn't public, but because the `main` function is defined in
the same module that `sound` is defined, we're allowed to refer to `sound` from
`main`. Next is `instrument`, which is a module marked with `pub`. We can
access the parent module of `instrument`, so we're allowed to access
`instrument`. Finally, `clarinet` is a function marked with `pub` and we can
access its parent module, so this function call works!

In the relative path case, the logic is the same as the absolute path except
for the first step. Rather than starting from the crate root, the path starts
from `sound`. The `sound` module is defined within the same module as `main`
is, so the relative path starting from the module in which `main` is defined
works. Then because `instrument` and `clarinet` are marked with `pub`, the rest
of the path works and this function call is valid as well!

#### Starting Relative Paths with `super`

You can also construct relative paths beginning with `super`. Doing so is like
starting a filesystem path with `..`: the path starts from the *parent* module,
rather than the current module. This is useful in situations such as the
example in Listing 7-9, where the function `clarinet` calls the function
`breathe_in` by specifying the path to `breathe_in` start with `super`:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // Function body code goes here
}
```

<span class="caption">Listing 7-9: Calling a function using a relative path
starting with `super` to look in the parent module</span>

The `clarinet` function is in the `instrument` module, so we can use `super` to
go to the parent module of `instrument`, which in this case is `crate`, the
root. From there, we look for `breathe_in`, and find it. Success!

The reason you might want to choose a relative path starting with `super`
rather than an absolute path starting with `crate` is that using `super` may
make it easier to update your code to have a different module hierarchy. For
example, if we decide to put the `instrument` module and the `breathe_in`
function into a module named `sound`, we would only need to add the `sound`
module, as shown in Listing 7-10.

<span class="filename">Filename: src/lib.rs</span>

```rust
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

<span class="caption">Listing 7-10: Adding a parent module named `sound`
doesn't affect the relative path `super::breathe_in`</span>

The call to `super::breathe_in` from the `clarinet` function will continue to
work in Listing 7-10 as it did in Listing 7-9, without needing to update the
path. If instead of `super::breathe_in` we had used `crate::breathe_in` in the
`clarinet` function, when we add the parent `sound` module, we would need to
update the `clarinet` function to use the path `crate::sound::breathe_in`
instead. Using a relative path can mean fewer updates are necessary when
rearranging modules.

#### Using `pub` with Structs and Enums

You can designate structs and enums to be public in a similar way as we've
shown with modules and functions, with a few additional details.

If you use `pub` before a struct definition, you make the struct public.
However, the struct's fields are still private. You can choose to make each
field public or not on a case-by-case basis. In Listing 7-11, we've defined a
public `plant::Vegetable` struct with a public `name` field but a private `id`
field.

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 7-11: A struct with some public fields and some
private fields</span>

Because the `name` field of the `plant::Vegetable` struct is public, in `main`
we can write and read to the `name` field by using dot notation. We're not
allowed to use the `id` field in `main` because it's private. Try uncommenting
the line printing the `id` field value to see what error you get! Also note
that because `plant::Vegetable` has a private field, the struct needs to
provide a public associated function that constructs an instance of `Vegetable`
(we've used the conventional name `new` here). If `Vegetable` didn't have such
a function, we wouldn't be able to create an instance of `Vegetable` in `main`
because we're not allowed to set the value of the private `id` field in `main`.

In contrast, if you make a public enum, all of its variants are public. You
only need the `pub` before the `enum` keyword, as shown in Listing 7-12.

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 7-12: Designating an enum as public makes all its
variants public</span>

Because we made the `Appetizer` enum public, we're able to use the `Soup` and
`Salad` variants in `main`.

There's one more situation involving `pub` that we haven't covered, and that's
with our last module system feature: the `use` keyword. Let's cover `use` by
itself, and then we'll show how `pub` and `use` can be combined.

### The `use` Keyword to Bring Paths into a Scope

You may have been thinking that many of the paths we've written to call
functions in the listings in this chapter are long and repetitive. For example,
in Listing 7-8, whether we chose the absolute or relative path to the
`clarinet` function, every time we wanted to call `clarinet` we had to specify
`sound` and `instrument` too. Luckily, there's a way to bring a path into a
scope once and then call the items in that path as if they're local items: with
the `use` keyword. In Listing 7-13, we bring the `crate::sound::instrument`
module into the scope of the `main` function so that we only have to specify
`instrument::clarinet` to call the `clarinet` function in `main`.

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 7-13: Bringing a module into scope to shorten the
path we have to specify to call an item within that module</span>

Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::sound::instrument` in the crate root,
`instrument` is now a valid name in that scope as if the `instrument` module
had been defined in the crate root. We can now reach items in the `instrument`
module through the older, full paths, or we can reach items through the new,
shorter path that we've created with `use`. Paths brought into scope with `use`
also check privacy, like any other paths.

<!-- Carol has edited up to here so far -->

If you want to use `use` with a relative path, there's a small wart: instead
of being able to use a name in the current scope, you must prefix it with
`self`:

```rust,ignore
use self::branch1::branch2;
```

This may not be neccesary in the future, but it's something to keep in mind
currently. Your authors rarely use `self`, preferring to always use `crate`
and absolute paths. This way, when you move code around, the imports it needs
don't change. Up to you!

A brief note about idioms:

```rust,ignore
// idiomatic import
use crate::branch1::branch2;

// idiomatic call
branch2::leaf();

// unidiomatic import
use crate::branch1::branch2::leaf;

// unidiomatic call
leaf();
```

For functions, it's considered idiomatic to `use` the parent module, and
use it to call the function that way. This makes it clear that it's not
locally defined, while still minimizing boilerplate.

For structs, enums, and other things, importing them directly is idiomatic
For example:

```rust,ignore
// idiomatic
use std::collections::HashMap;
let map = HashMap::new();

// not idiomatic
use std::collections;
let map = collections::HashMap::new();
```

The exception is if the names would clash:

```rust,ignore
use std::fmt;
use std::io;

fn foo() -> fmt::Result<()> {
fn foo() -> io::Result<()> {
```

We couldn't bring both `Result`s into the same scope, or their names would
clash.

### Making an Import Public with `pub use`

When you `use` something, it brings that name into scope, but it's private.
If you want it to be public, you can combine `pub` and `use`:

```rust,ignore
mod branch1 {
    use self::branch2::leaf;

    mod branch2 {
        pub fn leaf() {
            // code goes here
        }
    }
}

// this won't work
use branch1::leaf;
```

Here, while we can access `branch1` because it's in the same module, and
`leaf` does exist inside of `branch1` thanks to `use`, it's private.

If we change it to `pub use self::branch2::leaf`, it would now be public
and that line works!

`pub use` is sometimes nicknamed a "re-export", since you're both bringing
something into scope, but also making it available for others to bring into
their scope.

### Re-naming Imports with `as`

Speaking of clashing names, we *could* solve this another way:

```rust,ignore
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn foo() -> FmtResult<()> {
fn foo() -> IoResult<()> {
```

In other words, `as` lets us pick a differnet final name for this path. It
will still refer to the original definition, but under a different name.
Sometimes this can be a good way to avoid conflicts.

### Using External Packages

If you read Chapter 2, you programmed a guessing game. That project used an
external package, `rand`, to get random numbers. To use `rand` in your own
project, you add this to your `Cargo.toml`:

```toml
[dependencies]
rand = "0.5.5"
```

And now, you can use `use` with the name of the crate, `rand`, to bring stuff into
scope:

```rust,ignore
use rand::Rng;

// Rng can now be used.
```

It's that easy!

Note that the standard library is a crate, and that means it's external to your crate.
You don't need to change `Cargo.toml` to include `std`, but you can refer to it in
`use`:

```rust
use std::collections::HashMap;
```

This is an absolute path, starting with the name of the crate: `std`.

### Nested Imports for Cleaning Up Large Import Lists

The guessing game project also had multiple imports with a common
prefix, like this:

```rust
use std::io;
use std::cmp::Ordering;
```

We can use 'nested paths' to make this a bit shorter:

```rust
use std::{
    io,
    cmp::Ordering,
};
```

Additionally, if we want to say, de-duplicate this:

```rust
use std::io;
use std::io::Write;
```

We can use `self` in the nested path:

```rust
use std::io::{self, Write};
```

This brings both `std::io` and `std::io::Write` into scope.

### Glob Imports with `*`

If you'd like to bring *all* public items into scope, you can use a glob
import:

```rust
use std::collections::*;
```

Be careful with this! This makes it a little harder to tell what names are in
scope.

Glob imports are often used when testing; we'll talk about that in Chapter
11. They're also sometimes used as part of the "prelude pattern", see [the
standard library documentation](../../std/prelude/index.html#other-preludes)
for more.

### Putting Modules in Different Files

Finally, you don't have to write all of your modules in the same file!
Instead of writing this:

```rust
mod branch1 {
    fn leaf() {
        // code goes here
    }
}
```

You can create a new file, `src/branch1.rs`, with this in it:

```rust
fn leaf() {
    // code goes here
}
```

And then modify your `lib.rs` or `main.rs` like this:

```rust,ignore
mod branch1;
```

Using a `;` instead of a block tells Rust to load the contents of the module
from another file. If we wanted to continue with our example, and put a
sub-module inside of `src/branch1.rs`:

```rust,ignore
mod branch2;
```

We would need to create a sub-folder, and a file inside of it. They would be
named `src/branch1/branch2.rs`. If `branch2` has any `mod` declarations inside
of it, you'd keep going, making sub-folders as appropriate.

## Summary
