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
