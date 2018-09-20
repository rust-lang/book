# Modules and use to control scope and privacy

Rust has a feature that's often referred to as "the module system," but
it encompasses a few more features than only modules. In this section,
we'll talk about:

* Modules, a way to control the privacy of paths
* Paths, a way to name things
* `use` a keyword to bring a path into scope
* `pub`, a keyword to make things public
* re-naming imports with `as`
* Using external packages
* Nested imports to clean up large import lists
* "glob imports" with `*` to bring everything into scope
* Splitting modules up into individual files

First up, modules. Here's an example of some code that uses modules:

```rust
mod foo {
    fn bar() {
        // code goes here
    }
}

fn main() {

}
```

As you can see, we've defined two functions, `main` and `bar`. The `bar`
function, however, is inside of a `mod` block. This block defines a module
named `foo`. You can nest modules inside of other modules:

```rust
mod branch1 {
    mod branch2 {
        mod branch4 {
            fn leaf() {
                // code goes here
            }
        }
    }

    mod branch3 {

    }
}

fn main() {

}
```

Remember in the last section, when we said that `main.rs` and `lib.rs` are
considered "crate roots?" This is because the contents of either of these two
files form a module named `crate`, at the root of the crate tree. So in this
example, we have a module tree that looks like this:

```text
crate
 └── branch1
     └── branch2
        └── branch4
     └── branch3
```

This might remind you of the filesystem you have on your computer; this is
a very apt comparison! The module system is similar to a filesystem in many
ways; analogies to filesystems are usually very helpful, and we'll be making
them in this chapter.

Just like directories on a filesystem, you place code inside whichever module
you'd like. How should you split up your code into modules? What should you
name those modules? In order to talk about that, we need to learn about
`pub`. But before we get to `pub`, let's talk about a seemingly simple
question: how can we call the `leaf` function?

## Paths for referring to something

If we want to call a function, we need to know its *path*. It's sort of
a synonym for "name," but evokes that filesystem metaphor. Additionally,
functions, structs, etc may have multiple paths that refer to the same
place, so "name" feels slightly off.

A *path* can take two forms:

* An *absolute path* starts with a crate name, or a literal `crate`, to refer
  to the crate it's in.
* A *relative path* starts with `self`, `super`, or an identifier in the
  current module.
* Both kinds of paths are followed by one or more identifiers, separated by
  double colons (`::`).

What's the path of `leaf`? Let's simplify our code a bit:

```rust
mod branch1 {
    mod branch2 {
        fn leaf() {
            // code goes here
        }
    }
}

fn main() {
    // how do we call leaf?
}
```

If we wanted to call `leaf` from `main`, we can do it two ways:

```rust,ignore,does_not_compile
fn main() {
    // absolute path
    crate::branch1::branch2::leaf();

    // relative path
    branch1::branch2::leaf();
}
```

The former is an absolute path. Because `leaf` is defined in our crate,
we use the `crate` keyword to start an absolute path, and then include
each of the modules until we make our way to leaf. This is kind of like
running `/branch1/branch2/leaf` as a program on your computer; the `crate`
name is like starting the path with `/` in your shell.

The second one is a relative path; it starts with the name of `branch1`,
a module that's at the same level of the module tree that we are. This is
kind of like running `branch1/branch2/leaf` as a program on your computer;
starting with a name means that the path is relative.

You may be thinking "wow, that's a long name. Look at how we had to repeat
all of that `branch1::branch2` stuff just to call `leaf` twice." You're
not wrong. But before we can talk about how to simplify this example,
we have a problem: this example does not compile!

```console
> cargo build
   Compiling sampleproject v0.1.0 (file:///projects/sampleproject)
error[E0603]: module `branch2` is private
  --> src\main.rs:10:5
   |
10 |     crate::branch1::branch2::leaf();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: module `branch2` is private
  --> src\main.rs:12:5
   |
12 |     branch1::branch2::leaf();
   |     ^^^^^^^^^^^^^^^^^^^^^^
```

While we have the correct path for `leaf`, we cannot call it, as it's private.
It's time to learn about `pub`!

## `pub` to make paths public

Earlier, we talked about the syntax of modules, but we didn't really talk about
*why* they exist. Modules are the *privacy boundary* in Rust. In other words,
if you want to make something private, you put it in a module. Here's the
privacy rules:

* Everything is private by default.
* You can use the `pub` keyword to make something public.
* You are not allowed to use private code inside of children modules.
* You are allowed to use any code inside of parent modules or the current module.

In other words, privacy works "down" the module tree, but is public "up" the tree.
Again, think of a filesystem: if a directory is private, you cannot look into it,
but you can look inside the current directory or any parent directories.

Our error said that `branch2` was private. Let's fix that:

```rust,ignore,does_not_compile
mod branch1 {
    pub mod branch2 {
        fn leaf() {
            // code goes here
        }
    }
}

fn main() {
    // absolute path
    crate::branch1::branch2::leaf();

    // relative path
    branch1::branch2::leaf();
}
```

Adding the `pub` keyword in front of `mod branch2` makes the module public.
This means that, if we're allowed to access `branch1`, we can access
`branch2`. The contents of `branch2` are still private; that is, making the
module public does not make its contents public. It purely lets code in its
parent refer to it.

We still have an error, though:

```console
> cargo build
   Compiling sampleproject v0.1.0 (file:///projects/sampleproject)
error[E0603]: function `leaf` is private
  --> src\main.rs:10:5
   |
10 |     crate::branch1::branch2::leaf();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `leaf` is private
  --> src\main.rs:12:5
   |
12 |     branch1::branch2::leaf();
   |     ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
```

You can use `pub` on more than only modules; you can use it on structs,
enums, and functions as well.

Let's make `leaf` public as well:

```rust,ignore
mod branch1 {
    pub mod branch2 {
        pub fn leaf() {
            // code goes here
        }
    }
}

fn main() {
    // absolute path
    crate::branch1::branch2::leaf();

    // relative path
    branch1::branch2::leaf();
}
```

This will now compile! Let's look at both paths and double check why this
works.

In the absolute path case, we start with `crate`, the root of our crate. From
there, we have `branch1`, and it is a module that exists. It's not public,
but because we're in the same module as it's defined, we're allowed to refer
to it. Next is `branch2`, which is `pub`, so that's fine. Finally, `leaf`,
which is also `pub`, so we're good!

In the relative path case, it's the exact same, without the first step.
`branch1` is in the same module as us, so we're fine. `branch2` and `leaf`
are `pub`. Everything checks out!

You can also construct relative paths using `super`. This is like `..` in a
filesytem; that is, it says to start looking in the *parent* module, rather
than the current module.

```rust,ignore
mod foo {
    fn bar() {
        super::baz();
    }
}

fn baz() {
    // code goes here
}
```

`bar` is in the `foo` module, so we can use `super` to go to its parent
module, which in this case is `crate`, the root. From there, we look for
`baz`, and find it. Success!

If you use `pub` on a struct, you can make the struct public, and also its
members on a case-by-case basis:

```rust
// this struct is public...
pub struct Point {
    // ... and so is x ...
    pub x: i32,
    // ... but y is private
    y: i32,
}
```

If you make a public enum, all of its variants are public, so you only need
the `pub` next to `enum`:

```rust
pub enum ThisOrThat {
    This,
    That,
}
```

There's one more way to use `pub` that we haven't covered, and that's using it
along with our last module system feature: `use`.

## `use` to bring paths into scope

If we look at our code, even though we only call `leaf` twice, there's a lot of
duplication by specifying the whole path every time:

```rust,ignore
mod branch1 {
    pub mod branch2 {
        pub fn leaf() {
            // code goes here
        }
    }
}

fn main() {
    // here
    crate::branch1::branch2::leaf();

    // and here
    branch1::branch2::leaf();
}
```

We can use the `use` keyword to fix this:

```rust,ignore

mod branch1 {
    pub mod branch2 {
        pub fn leaf() {
            // code goes here
        }
    }
}

use crate::branch1::branch2;

fn main() {
    // we can now do this!
    branch2::leaf();

    // this still works too
    branch1::branch2::leaf();
}
```

If we say `use` and then a path, it's like creating a symlink in the
filesystem. `branch2` is now a valid name in this module, just like any
other. We can now reach it through the older, full paths, or this new path
that we've created with `use`. `use` also checks privacy, like any other
path.

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

## Making an import public with `pub use`

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

## Re-naming imports with `as`

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

## Using external packages

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

## Nested imports for cleaning up large import lists

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

## Glob imports with `*`

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

## Putting modules in different files

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
