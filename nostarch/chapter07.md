
[TOC]

# Modules

When you start writing programs in Rust, your code might live solely in the
`main` function. As your code grows, you'll eventually move functionality out
into other functions, both for re-use and for better organization. By splitting
your code up into smaller chunks, each chunk is easier to understand on its
own. But what happens if find yourself with too many functions? Rust has a
module system that handles the problem of wanting to to re-use code while
keeping your code organized.

In the same way that you extract lines of code into a function, you can extract
functions (and other code like structs and enums too) into different modules. A
*module* is a namespace that contains definitions of functions or types, and
you can choose whether those definitions are visible outside their module
(public) or not (private). Here's an overview of how modules work in Rust:

* You declare a new module with the keyword `mod`
* By default, everything is set as private, but you can use the `pub` keyword
  to make the module public, and therefore visible outside of the namespace.
* The `use` keyword allows you to bring modules, or the definitions inside
  modules, into scope so that it's easier to refer to them.

We'll take a look at each of these parts and see how they fit into the whole.

## mod and the Filesystem

We'll start our modules example by making a new project with Cargo, but instead
of creating a binary file, we're going to make a library: a project that other
people can pull into their projects as a dependency. We saw this with the
`rand` crate in Chapter 2.

<!-- do we generally refer to libraries as crates in Rust, or can you have both
crates and libraries? If the former, we should make that known, something like
"we're going to make a library, known as a crate in Rust" -->

We'll create a library that provides some general networking functionality, and
we'll call our library `communicator`. By default, cargo will create a library
unless another type of file is specified, so if we leave off the `--bin` option
that we've been using so far our file will be a library:

```bash
$ cargo new communicator
$ cd communicator
```

Notice that Cargo generated `src/lib.rs` instead of `src/main.rs`, and inside
it we'll find this:

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

Rust creates an empty test created to help us get our library started, rather
than the "Hello, world!" binary that we get with the `--bin` option. We'll look
at the `#[]` and `mod tests` syntax a little later, but for now just make sure
to leave it in your `src/lib.rs`.

We're going to look at the different options for organizing your library's
code, suitable for the different intentions you have for your code.

### Module Definitions

Every module definition in Rust starts with the `mod` keyword. Add this code to
the beginning of the *lib.rs* file, above the test code:

<!-- What does "cryptic!" mean? Intriguing! -->

<!--cryptic!-->

Filename: src/lib.rs

```rust
mod network {
    fn connect() {
    }
}
```

This is our first module declaration. We use the `mod` keyword, followed by the
name of the module `network`, and then a block of code in curly braces.
Everything inside this block is inside the namespace `network`. In this case,
we have a single function, `connect`. If we wanted to call this function from a
script outside the `network` module, we would need to specify the module and
use the namespace syntax `::`, like so `network::connect()`, rather than just
`connect()`.

We can also have multiple modules, side-by-side, in the same *lib.rs* file. For
example, if we wanted a `client` module too, we would add:

Filename: src/lib.rs

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

Listing 7-1:

<!---I'm adding listing numbers to those examples we later reference, could you
add captions? If you feel other listings warrant numbers too, feel free to add.
We can update the numbering at copyedit, no need to worry about it now -->

Now we have a `network::connect` function and a `client::connect` function.

<!-- Would these `connect` functions do the same thing, or can they be totally
different? -->

And we can put modules inside of modules. If we wanted to have a `client` module
within `network`, we can do so like this:

<!-- In what situation would you want to put a module inside another module?
Can we give some clues as to when each organization style would be suitable? -->

Filename: src/lib.rs

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```
Listing 7-2:

Place this in your *lib.rs* file. This gives us `network::connect` and
`network::client::connect`.

<!-- What would the difference between this and the example in 7-1 be? -->

In this way, modules form a tree. The contents of `src/lib.rs` are at the root
of the project's tree, and the submodules form the leaves. Here's what our
example from Listing 7-1 looks like when thought of this way:

<!-- Above --- would the submodules not be the branches? That seems like the
more common metaphor, though I may well be wrong!-->

```text
communicator
 ├── network
 └── client
```

And here's the example from Listing 7-2:

```text
communicator
 └── network
     └── client
```

You can see that the `client` branch is a sub-branch of `network`, rather than
running parralel to it.

<!---What does this mean for the project, how would this kind of branching
effect the program? -->

More complicated projects can have a lot of modules, and they'll need to be
orgnaized logically in order to keep track of them.

### Moving Modules to Other Files

Modules form a hierarchical, tree-like structure, much like another structure
in Rust: file systems! We use Rust's module system to split Rust projects up
into multiple files. As an example, replace the modules in your *lib.rs* file
with the following (remembering to leave the `#[cfg:(test)]` block at the
bottom):

File: src/lib.rs

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<!-- Can you give an example of when we'd want to split the client module into
another file? -->

Let's extract the `client` module into another file. First, we need to change
our code in `src/lib.rs` to the following:

File: src/lib.rs

```rust,ignore
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<!--- Since we're using the mod keyword, are we still defining client here or
is this just referencing it? Reading it on, it looks like we are defining it
here, that might be worth mentioning here -->

Here we still reference the module `client`, but but rather than following `mod
client` with curly braces, we have a semicolon, and we delete the `connect`
function namespaced within it. This lets Rust know that this module exists, but
that it's in an external file with that module's name.

So now we need to create the external file with that module name. Create a
`client.rs` file in your *src/* directory, then open it up and enter the
following:

File: src/client.rs

```rust
fn connect() {
}
```

Note that we don't need a `mod` declaration in this file; that's because we
already declared the `client` module with `mod` in `src/lib.rs`. This file just
provides the _contents_ of the `client` module. If we put a `mod client` here,
we'd be giving the `client` module its own submodule named `client`!

<!-- so you declare the module in the file you want to reference the module in,
but hold that module's contents in another file, is that right? Why do it that
way, and not have the entire module with declaration in one file and just
reference it? Seems interesting/useful to know. That may be a silly question,
coming from a non-programmer perspective, apologies if so! -->

Now, everything should compile successfully, though you'll get a few warnings:

<!-- why do we use `build` and not `run` here? I just tried `run` out of habit
and got an error requesting a bin file --- you might want to point out that
we're only building and not running, so reader don't make the same mistake, and
say why -->

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:4:5
  |
4 |     fn connect() {
  |     ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:8:9
  |
8 |         fn connect() {
  |         ^
```

These warnings tell us that we have a function that we never use, much like the
unused variable warnings we got in Chapter XX. Don't worry about those warnings
for now; we'll clear them later in the chapter . The good news us that they're
just warnings, but our project was built successfully!

Let's extract the `network` module into its own file next, using the same
pattern. In `src/lib.rs` delete your `network` branches and add a semicolon to
the declaration, like so:

Filename: src/lib.rs

```rust,ignore
mod client;

mod network;
```

And then create a new `src/network.rs` file and enter the following:

Filename: src/network.rs

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

Notice that here you *have* added a `mod` declaration within this module file;
this is because you want `server` to be a sub-module of `network`.

<!-- I think it's worth drawing attention to this, above, feel free to
re-word/add -->

Now run `cargo build` again. Success! We have one more module to extract:
`server`. Because it's a sub-module---that is, a module within a module---our
current tactic of extracting a module into a file named after that module won't
work. If we try in now, by modifying `src/network.rs` to look like this, we'll
get an error:

Filename: src/network.rs

```rust,ignore
fn connect() {
}

mod server;
```

Now create a `src/server.rs` file and enter the following:

Filename: src/server.rs

```rust
fn connect() {
}
```

When we try to `cargo build`, we'll get this error:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```
Listing 7-4:

<!-- I got a bit lost with the hypothetical tone of the original text here,
below --- are we saying that this **is** what happened, what caused the error?
Or just something that can happen? Is there a way to make this more clear, I'm
stil not quite following -->

The problem here is that, in our case, we have different names for our modules:
`client` and `network::server`. But when we make a separate module file for the
submodule, we have `client` and `network::client`, or `server` and
`network::server`, meaning we have two modules with the same name at different
places in the module hierarchy. This is completely valid in Rust, but then the
program doesn't know which module the files `src/client.rs` and `src/server.rs`
are for.

The error we get in Listing 7-4 is actually pretty helpful, as it points out
something you probably didn't know you could do:

> note: maybe move this module `network` to its own directory via
`network/mod.rs`

Instead of continuing to follow the same file naming pattern we used
previously, we can do what the error suggests, and make a new _directory_ with
the module branch's name, move the submodule file `src/server.rs` into it, and
change

<!-- I'm not sure what you mean by "change `src/network.rs` to
`src/network/mod.rs`." That we need to make a new directory, move the
network.rs file into it, and rename it as mod.rs? That's how I'm reading it,
but it doesn't seem right, can you clarify that? -->

`src/network.rs` to `src/network/mod.rs`. If you do that now and try to build
it:

```bash
$ mkdir src/network
$ mv src/server.rs src/network
$ mv src/network.rs src/network/mod.rs
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
<warnings>
$
```

It works! So now our module layout looks like this:

<!-- can you explain a bit about why this works, where the previous method
failed? -->

```text
communicator
 ├── client
 └── network
     └── server
```

And the corresponding file layout looks like this:

```text
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```

### Rules of Module File Systems

In summary, these are the rules of modules with regards to files:

* If a module named `foo` has no submodules, you should put the declarations
  for `foo` in a file named `foo.rs`.
* If a module named `foo` does have submodules, you should put the declarations
  for `foo` in a file named `foo/mod.rs`.
* The first two rules apply recursively, so that if a module named `foo` has a
  submodule named `bar` and `bar` does not have submodules, you should have the
  following files in your `src` directory:

  ```text
  ├── foo
  │   ├── bar.rs (contains the declarations in `foo::bar`)
  │   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
  ```

* The modules themselves should be declared in their parent module's file using
  the `mod` keyword.

Next, we'll talk about the `pub` keyword, and get rid of those warnings!

## Controlling Visibility with `pub`

Listing 7-4 showed the error message we received when we built our
`communicator` as it was then, warning us that the `connect` function is never
used.

<!--
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 -->
```bash
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

So why are we receiving these errors? After all, we're building a library with
functions that are intended to be used by our *users*, and not necessarily by
us within our own project, so it shouldn't matter that `connect` goes unused.
The point of creating them is that they will be used by another project and not
our own.

To understand why this program invokes these warnings, let's try using the
`connect` library as if we were another project, calling it externally. Create
a `src/main.rs` file and fille it with with this code:

<!--- do they need to create a new binary/cargo project? Or is this within the
communcator library? -->

Filename: src/main.rs

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

<!-- I'm not sure what the phrase "as the crate root of a binary crate" means
or refers to here, I didn't can you expand on that? What is the binary crate
here, I can't see where the binary came from? And what is the exisiting library
create, you mean the `communicator` crate?-->

We use the `extern crate` command to bring the `communicator` library crate
into scope, because our package actually now contains *two* crates: Cargo
treats src/main.rs as the crate root of a binary crate, and the existing
library crate. This pattern is quite common for executable crates: most
functionality is in a library crate, and the executable crate uses that
library. This way, other programs can also use the library crate, and it’s a
nice separation of concerns.

Our binary crate right now just calls our library's `connect` function from the
`client` module. However, invoking `cargo build` will now give us an error
after the warnings:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! This tells us that the `client` module is private, and this is the crux
of the warnings. It's also the first time you've run into the concepts of
'public' and 'private' in the context of Rust. The default state of all
programs in Rust is private, where no one else can possibly use the code. That
menas that so if you don't use a function within your own program, Rust will
warn you that it's gone unused. Once we specify that the function is public,
Rust knows that you intend the functions for external use and considers the
theoretical external usage that's now possible as "being used". Thus, when
something is marked as public, Rust will not require that it's ussed in your
own program and will stop warning that the item is unused.

### Making a Function Public

To tell Rust to make something public, you add the `pub` keyword to the start
of the declaration of the item you want to make public. We'll focus on fixing
that tells us that `client::connect` has gone unused for now. To fix the error,
modify `src/lib.rs` to make the `client` module public, like so:

Filename: src/lib.rs

```rust,ignore
pub mod client;

mod network;
```

The `pub` goes right before `mod`. Let's try building again:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
<warnings>
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Hooray! We have a different error! Yes, different error messages are a cause
for celebration. The new error says "function `connect` is private", so let's
edit `src/client.rs` to make `client::connect` public too:

Filename: src/client.rs

```rust
pub fn connect() {
}
```

And run `cargo build` again:

```bash
 cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

It compiled, and the warning about `client::connect` not being used is gone!

> Note: Unused code warnings don't always indicate that something needs to be
> made public: if you *didn't* want these functions to be part of your public
> API, unused code warnings could be alerting you to code you no longer needed
> and can safely delete. They could also be alerting you to a bug, if you had
> just accidentally removed all places within your library where this function
> is called.

In our case though, we *do* want the other two functions to be part of our
crate's public API, so let's mark them as `pub` as well to try to get rid of
the remaining warnings. Modify `src/network/mod.rs` to be:

Filename: src/network/mod.rs

```rust,ignore
pub fn connect() {
}

mod server;
```

And compile:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | pub fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Hmmm, we're still getting an unused warning even though `connect` is set to
`pub`. This is, because while the function is public within the module, the
`network` module the function resides in is not public. We're working from the
interior of the library out this time, where with `client` where we worked from
the outside in. We need to change `src/lib.rs` to make `network` public too:

Filename: src/lib.rs

```rust,ignore
pub mod client;

pub mod network;
```

Now if we compile, that warning is gone:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Only one warning left! Try to fix this one on your own!

### Privacy Rules

Overall, these are the rules for item visibility:

1. If an item is public, it can be accessed through any of its
  parent modules.
2. If an item is private, it may be accessed only by the current module and its
  child modules.

### A Privacy Example

Let's look at a few more examples to get some practice. Create a new libary
project and enter the code in Listing 7-5 into your new project's `src/lib.rs`:

Filename: src/lib.rs

```rust,ignore
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

Before you try to compile this code, make a guess about which lines in
`try_me` function will have errors.

When you've made some educated guesses, read on and we'll talk through them!

#### A Privacy Example: Looking at the Errors

The `try_me` function is in the root module of your project. The module named
`outermost` is private, but the second rule says you, the compiler, are allowed
to access it since `outermost` is in your current root module.

The function call `outermost::middle_function()` will work. This is because
`middle_function` is public, and you are accessing it through its parent
module, `outermost`, which we just determined you can access in the previous
paragraph.

`outermost::middle_secret_function()` will cause a compilation error.
`middle_secret_function` is private, so the second rule applies. Your current
root module is neither the current module of `middle_secret_function`
(`outermost` is), nor is it a child module of the current module of
`middle_secret_function`.

The module named `inside` is private and has no child modules, so it can only
be accessed by its current module, `outermost`. That means the `try_me`
function is not allowed to call `outermost::inside::inner_function()` or
`outermost::inside::secret_function()`.

#### A Privacy Example: Fixing the Errors

Here we provide you with some suggestions for fixing the code. Before you try
each one, make a guess as to whether it will fix the errors, then compile to
see if you're right and use the privacy rules to understand why.

* What if the `inside` module was public?
* What if `outside` was public and `inside` was private?
* What if, in the body of `inner_function`, you called
  `::outermost::middle_secret_function()`? (The two colons at the beginning
  mean that we want to refer to the namespaces starting from the root
  namespace.)

Feel free to design more experiments and try them out!

Next, let's talk about bringing items into a scope with the `use` keyword.

## Importing Names

You've seen how you can call functions defined within a module using the
module name as part of the call, as in Listing 7-6.

Filename: src/main.rs

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn namespaces() {}
        }
    }
}

fn main() {
    a::series::of::namespaces();
}
```

Listing 7-6:

As you can see, referring to the fully qualified name can get quite lengthy.
Luckily, Rust has a keyword to make these calls more efficient.

### Efficient Imports with `use`

Rust's `use` keyword works to shorten lengthy function calls by bringing the
modules of the function you want to call into scope. To use `use`, replace your
*src/main.rs* code with the following:

Filename: src/main.rs

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn namespaces() {}
        }
    }
}

use a::series::of;

fn main() {
    of::namespaces();
}
```

<!--- I had assumed the line "of::namespaces();" line would just need to be
"namespaces();" since we include "of" in the `use`, could you say explicitly
that we need to repeat "of" and why? -->

When you `use` a module, it brings the module's name into scope, allowing you
to shorten the function call; you only need type the final module name,
not the entire chain of modules.

The `use` keyword is quite powerful and can bring all kinds of things into
scope. For example, we could `use` the function itself:

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn namespaces() {}
        }
    }
}

use a::series::of::namespaces;

fn main() {
    namespaces();
}
```

<!--- Ah, this is what I thought we were doing above. So why would you ever
namespace the module over the function, if this is more efficient --- if the
module had more than one function? -->

This allows us to exclude any of the modules and just reference the function.

Since Enums form this kind of namespace, we can import an enum's variants with
`use` as well. For any kind of `use` statement, if you're importing multiple
items from one namespace, you can list them using curly braces and commas in
the last position, like so:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green; // because we didn't use TrafficLight::Green
}
```

### Glob Imports with `*`

If you'd like to import all the items in a namespace at once, you can use the
`*` syntax. For example:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

The `*` is called a 'glob', and it will import everything that's visible inside
of the namespace. Globs should be used sparingly: they are convenient, but you
might also pull in more things than you expected and cause naming conflicts.

### Using `super` to Access a Parent Module

As you now know, when you create a crate Cargo makes a `tests` module for you.
Let's go into more detail about that now. Open your `communicator` project, and
open `src/lib.rs`.

Filename: src/lib.rs

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

We'll explain more about testing in Chapter XX, but parts of this should make
sense now: we have a module named `tests` that lives next to our other modules
and contains one function named `it_works`. Even though there are special
annotations, the `tests` module is just another module!

Tests are for exercising the code within our library, so let's try to call
our `client::connect` function from this `it_works` function, even though
we're not going to be checking any functionality right now:

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Run the tests by invoking the `cargo test` command:

```bash
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `client`

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

The compilation failed, but why? We don't need to place `communicator::` in
front of the function like we did in `src/main.rs` because we are definitely
within the `communicator` library crate here. The reason is that paths are
always relative to the current module; the only exception being in a `use`
statement, where paths are relative to the crate root by default. Our `tests`
module needs a `client` module in its scope!

So how do we get back up one module? We can either use leading colons to let
Rust know that we want to start from the root and list the whole path:

```rust,ignore
::client::connect();
```

Or we can use `super` to move up one module in the hierarchy:

```rust,ignore
super::client::connect();
```

<!-- I can't really see what the different is with the examples above since
super uses the root path but also adds super? It doesn't seem more convenient,
is this just not a great example of its use? -->

If you're deep in the module hierarchy, starting from the root every time would
get long, so using `super` is a good shortcut. Plus, if you've used the path
from the root in many places in your code and then you rearrange your modules
by moving a subtree to another place, you'd end up needing to update the path
in a lot of places and could easily miss some.

It would also be annoying to have to type `super::` all the time in each test,
but you've already seen the tool for that solution: `use`! The `super::`
functionality changes the path you give to `use` so that it is relative to the
parent module instead of to the root module.

For these reasons, in the `tests` module especially, `use super::something` is
usually the way to go. So now our test looks like this:

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

If we run `cargo test` again, the test will pass and the first part of the test
result output will be:

```bash
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Now you know techniques for organizing your code! Use these to group related
functionality together, keep files from getting too long, and present a tidy
public API to users of your library.

<!-- Could you add the summary? -->