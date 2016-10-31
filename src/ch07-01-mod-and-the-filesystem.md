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
