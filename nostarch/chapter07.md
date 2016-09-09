
[TOC]

# Modules

When you write a program in Rust, your code might start off living solely in
the `main` function. As your code grows, you eventually move functionality out
into functions, both for re-use and for nicer organization. By splitting your
code up into smaller chunks, each chunk is easier to understand on its own. So
what happens when you start having too many functions? Rust has a module system
that tackles both the problem of wanting to be able to re-use code and the
problem of keeping your code organized.

In the same way that you extract lines of code into a function, you can extract
functions (and other code like structs and enums too) into different modules. A
*module* is a namespace that contains definitions of functions or types, and
those definitions can be visible outside their module or not. Here's an
overview of how the bits fit together:

* `mod` declares a new module.
* Everything starts off as private, but the `pub` keyword makes it public.
* The `use` keyword allows you to bring modules, or definitions inside of them,
  into scope so that it's easier to refer to them.

We'll take a look at each of these parts and see how they fit into the whole.

## `mod` and the Filesystem

Every module in Rust starts with the `mod` keyword. In this next example, we'll
start again by making a new project with Cargo. This time, instead of a binary,
we're going to make a library: a project that other people would pull into their
projects as a dependency. We saw this with the `rand` crate in Chapter 2.

Imagine that we're creating a library to provide some general networking
functionality, and we decide to call our library `communicator`. To create this
library, we won't use the `--bin` option like we have before. This is because
by default cargo will create a library:

```bash
$ cargo new communicator
$ cd communicator
```

Notice that Cargo generated `src/lib.rs` instead of `src/main.rs` for us, and
inside it we'll find this:

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

This is an empty test to help us get our library started, instead of the binary
that says "Hello, world!" that we get with the `--bin` option. Let's ignore the
`#[]` stuff and `mod tests` for a little bit, but we'll make sure to leave it
in `src/lib.rs` for later.

We're going to look at different ways we could choose to organize our library's
code, any of which could make sense depending on exactly what we were trying to
do. To start, add this code at the beginning of the file:

Filename: src/lib.rs

```rust
mod network {
    fn connect() {
    }
}
```

This is our first module declaration. We use the `mod` keyword, followed by the
name of the module, and then a block of code in curly braces. Everything inside
this block is inside the namespace `network`. In this case, we have a single
function, `connect`. If we wanted to try and call this function from outside
the `network` module, we would say `network::connect()` rather than `connect()`.

We could have multiple modules, side-by-side. For example, if we wanted a
`client` module:

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

Now we have a `network::connect` function and a `client::connect` function.

And we can put modules inside of modules. If we wanted to have `client` be
within `network`:

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

This gives us `network::connect` and `network::client::connect`.

In this way, modules form a tree. The contents of `src/lib.rs` are at the root
of the project's tree, and the submodules form the leaves. Here's what our
first example looks like when thought of this way:

```text
communicator
 ├── network
 └── client
```

And here's the second:

```text
communicator
 └── network
     └── client
```

More complicated projects can have a lot of modules.

### Putting Modules in Another File

Modules form a hierarchical, tree-like structure. So does another thing:
file systems! The module system is the way that we split larger Rust projects up
into multiple files. Let's imagine we have a module layout like this:

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

Let's extract the `client` module into another file. First, we need to change
our code in `src/lib.rs`:

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

We still say `mod client`, but instead of curly braces, we have a semicolon.
This lets Rust know that we have a module, but it's in another file with that
module's name. Open up `src/client.rs` and put this in it:

File: src/client.rs

```rust
fn connect() {
}
```

Note that we don't need a `mod` declaration in this file. `mod` is for
declaring a new module, and we've already declared this module in `src/lib.rs`.
This file provides the _contents_ of the `client` module. If we put a `mod
client` here, we'd be giving the `client` module its own submodule named
`client`!

Now, everything should compile successfully, but with a few warnings:

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

Don't worry about those warnings for now; we'll clear them up in a future
section. They're just warnings, we've built things successfully!

Let's extract the `network` module into its own file next, using the same
pattern. Change `src/lib.rs` to look like this:

Filename: src/lib.rs

```rust,ignore
mod client;

mod network;
```

And then put this in `src/network.rs`

Filename: src/network.rs

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

And then run `cargo build` again. Success! We have one more module to extract:
`server`. Unfortunately, our current tactic of extracting a module into a file
named after that module won't work. Let's try it anyway. Modify
`src/network.rs` to look like this:

Filename: src/network.rs

```rust,ignore
fn connect() {
}

mod server;
```

Put this in `src/server.rs`

Filename: src/server.rs

```rust
fn connect() {
}
```

When we try to `cargo build`, we'll get an error:

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

This error is actually pretty helpful. It points out something we didn't know
that we could do yet:

> note: maybe move this module `network` to its own directory via
`network/mod.rs`

Here's the problem: in our case, we have different names for our modules:
`client` and `network::server`. But what if we had `client` and
`network::client`, or `server` and `network::server`? Having two modules at
different places in the module hierarchy have the same name is completely
valid, but then which module would the files `src/client.rs` and
`src/server.rs`, respectively, be for?

Instead of continuing to follow the same file naming pattern we used
previously, we can do what the error suggests. We'll make a new _directory_,
move `src/server.rs` into it, and change `src/network.rs` to
`src/network/mod.rs`. Then, when we try to build:

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

In summary, these are the rules of modules with regards to files:

* If a module named `foo` has no submodules, you should put the declarations in
  the `foo` module in a file named `foo.rs`.
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

At the end of the last section, we had a project, `communicator`, and when we compiled it, we got some strange warnings:

```bash
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/client.rs:1:1
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

Why does this happen? After all, we're building a library. What if these three
functions are the public interface that we want our *users* to use? We won't
necessarily be using them within our own project, but the point of creating them
is that they *will* be used by another project. Let's try using them as if we
were another project using our library to see what happens and understand why
we're getting these unused function warnings. Create a `src/main.rs` file with
this code:

Filename: src/main.rs

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

We need the `extern crate` line to bring our `communicator` library crate into
scope, because our package actually now contains *two* crates. Cargo treats
src/main.rs as the crate root of a binary crate, and we also have our existing
library crate. This pattern is quite common for executable crates: most
functionality is in a library crate, and the executable crate uses that
library. This way, other programs can also use the library crate, and it’s a
nice separation of concerns.

Our binary crate right now just calls our library's `connect` function from
the `client` module; we picked that one since it's the first warning in our
build output above. Invoking `cargo build` will now give us an error after the
warnings:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! The `client` module is private. This is the first time we've run into
the concepts of 'public' and 'private' in the context of Rust. There's no
keyword to make something private; that's the default state. In this default
state, no one else could possibly use it, so if we don't use it within our
library crate, Rust will warn us that it's unused. Once we tell Rust something
is public, Rust knows that we intend for code external to our crate to use it,
and Rust considers theoretical external usage that is now possible to count as
being used. Thus, when something is marked as public, Rust will stop warning us
that it is unused.

To tell Rust we want to make something public, we add the `pub` keyword. This
keyword goes before the declaration of the item we want to make public. Let's
modify `src/lib.rs` to make the `client` module public to fix the error we got:

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
edit `src/client.rs` to make `client::connect` public:

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

It compiled! And the warning about `client::connect` not being used is gone!

Making functions public isn't the only way to fix unused code warnings: if
we *didn't* want these functions to be part of our public API and we got these
warnings, the warnings could be alerting us to code we no longer needed and
could safely delete. They could also be alerting us to a bug, if we
had just accidentally removed all places within our library where we called
this function.

However, we *do* want the other two functions to be part of our crate's public
API, so let's mark them as `pub` as well to get rid of the remaining warnings.
Modify `src/network/mod.rs` to be:

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

Hmmm, it says this is still dead, even though it's `pub`. While the function is
public within the module, the `network` module it's in is not public. We're
working from the interior of the library out this time, as opposed to with
`client` where we worked from the outside in. Let's change `src/lib.rs` to add
the same fix though, by making `network` public like `client` is:

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

Only one last warning! Try to fix this one on your own!

### Privacy Rules

Overall, these are the rules for item visibility:

1. If an item is public, then it can be accessed through any of its
  parent modules.
2. If an item is private, it may be accessed by the current module and its
  child modules.

Let's look at a few more examples to get some practice. What if we had this
code in a new project's `src/lib.rs`:

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
`try_me` will have errors.

Ready? Let's talk through them!

The `try_me` function is in the root module of our project. The module named
`outermost` is private, but the second rule says we're allowed to access it
since `outermost` is in our current, root module.

The function call `outermost::middle_function()` will work. `middle_function`
is public, and we are accessing it through its parent module, `outermost`,
which we just determined we can access in the previous paragraph.

`outermost::middle_secret_function()` will cause a compilation error.
`middle_secret_function` is private, so the second rule applies. Our current
root module is neither the current module of `middle_secret_function`
(`outermost` is), nor is it a child module of the current module of
`middle_secret_function`.

The module named `inside` is private and has no child modules, so it can only
be accessed by its current module, `outermost`. That means the `try_me`
function is not allowed to call `outermost::inside::inner_function()` or
`outermost::inside::secret_function()`.

Here are some changes to try making with this code. Try each one, make a guess
about what will be allowed or not, compile to see if you're right, and use the
rules to understand why.

* What if the `inside` module was public?
* What if `outside` was public and `inside` was private?
* What if, in the body of `inner_function`, we called
  `::outermost::middle_secret_function()`? (The two colons at the beginning
  mean that we want to refer to the namespaces starting from the root
  namespace.)

Feel free to design more experiments and try them out!

Next, let's talk about bringing items into a scope with the `use` keyword.

## Importing Names with `use`

We've seen how we can call functions defined within a module by using the
module name as part of the call, like this:

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

However, referring to the fully qualified name can get quite lengthy, as we see
in that example. To solve this issue, Rust has a keyword, `use`. It works like
this:

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

We can `use` a module, and that will bring its name into scope. This allows us
to shorten our function call, only requiring us to type the final module name,
not the entire chain of them. `use` is quite powerful and can bring all kinds
of things into scope. For example, we could `use` the function itself:

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

Enums also form this kind of namespace; we can import an enum's variants with
`use` as well. For any kind of `use` statement, if you are importing multiple
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

If you'd like to import all the items in a namespace at once, you can use `*`:

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

Remember when we created our crate that Cargo made a `tests` module for us?
Let's talk about that now. It was in `src/lib.rs`:

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

Since tests are for exercising the code within our library, let's try to call
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

Why doesn't this compile? It's not because we don't have `communicator::` in
front of the function like we had in `src/main.rs`: we are definitely within
the `communicator` library crate here. The reason is that paths anywhere except
in a `use` statement are relative to the current module (In a `use` statement,
they're relative to the crate root by default). Our `tests` module doesn't have
a `client` module in its scope!

So how do we get back up one module? We can either use leading colons to say
that we want to start from the root and list the whole path:

```rust,ignore
::client::connect();
```

Or we can use `super` to move up one module in the hierarchy:

```rust,ignore
super::client::connect();
```

If we were deep in the module hierarchy, starting from the root every time
would get long. Plus, if we rearrange our modules by moving a subtree to
another place, there might be a lot of places the path would need to be updated
if we always used the path from the root.

It would also be annoying to have to type `super::` all the time in each test,
but we now have a tool for that solution: `use`! `super::` is special and
changes the path we give to `use` so that it is relative to the parent module
instead of to the root module.

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
