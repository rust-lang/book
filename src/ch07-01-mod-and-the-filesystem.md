## `mod` and the Filesystem

We’ll start our module example by making a new project with Cargo, but instead
of creating a binary crate, we’re going to make a library crate: a project that
other people can pull into their projects as a dependency. We saw this with the
`rand` crate in Chapter 2.

We’ll create a skeleton of a library that provides some general networking
functionality; we’re going to concentrate on the organization of the modules
and functions, but not worry about what code goes in the function bodies. We’ll
call our library `communicator`. By default, cargo will create a library unless
another type of project is specified, so if we leave off the `--bin` option
that we’ve been using so far our project will be a library:

```text
$ cargo new communicator
$ cd communicator
```

Notice that Cargo generated *src/lib.rs* instead of *src/main.rs*. Inside
*src/lib.rs* we’ll find this:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

Cargo creates an empty test to help us get our library started, rather
than the “Hello, world!” binary that we get with the `--bin` option. We’ll look
at the `#[]` and `mod tests` syntax a little later, but for now just make sure
to leave it in your *src/lib.rs*.

Since we don’t have a *src/main.rs*, there’s nothing for Cargo to execute with
the `cargo run` command. Therefore, we will be using the `cargo build` command
to only compile our library crate’s code.

We’re going to look at different options for organizing your library’s code
which will be suitable in a variety of situations, depending on the intentions
you have for your code.

### Module Definitions

For our `communicator` networking library, we’re first going to define a module
named `network` that contains the definition of a function called `connect`.
Every module definition in Rust starts with the `mod` keyword. Add this code to
the beginning of the *lib.rs* file, above the test code:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}
```

After the `mod` keyword, we put the name of the module, `network`, then a block
of code in curly braces. Everything inside this block is inside the namespace
`network`. In this case, we have a single function, `connect`. If we wanted to
call this function from a script outside the `network` module, we would need to
specify the module and use the namespace syntax `::`, like so:
`network::connect()`, rather than just `connect()`.

We can also have multiple modules, side-by-side, in the same *src/lib.rs* file.
For example, to have a `client` module too, that also has a function named
`connect`, we can add:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 7-1: The `network` module and the `client` module defined side-by-side
in *src/lib.rs*

</figcaption>
</figure>

Now we have a `network::connect` function and a `client::connect` function.
These can have completely different functionality, and the function names do
not conflict with each other since they’re in different modules.

While in this case, we’re building a library, there's nothing special about
*lib.rs*. We could also make use of submodules in a *main.rs* as well. In fact,
we can also put modules inside of modules. This can be useful as your modules
grow to keep related functionality organized together and separate
functionality apart. The choice of how you organize your code depends on how
you think about the relationship between the parts of your code. For instance,
the `client` code and its `connect` function might make more sense to users of
our library if it was inside the `network` namespace instead, like so:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 7-2: Moving the `client` module inside of the `network` module

</figcaption>
</figure>

In your *src/lib.rs* file, replace the existing `mod network` and `mod client`
definitions with this one that has the `client` module as an inner module of
`network`. Now we have the functions `network::connect` and
`network::client::connect`: again, the two functions named `connect` don’t
conflict with each other since they’re in different namespaces.

In this way, modules form a hierarchy. The contents of *src/lib.rs* are at the
topmost level, and the submodules are at lower levels. Here’s what the
organization of our example from Listing 7-1 looks like when thought of this
way:

```text
communicator
 ├── network
 └── client
```

And here’s the example from Listing 7-2:

```text
communicator
 └── network
     └── client
```

You can see that in Listing 7-2, `client` is a child of the `network` module,
rather than a sibling. More complicated projects can have a lot of modules, and
they’ll need to be organized logically in order to keep track of them. What
“logically” means in your project is up to you and depends on how you and users
of your library think about your project’s domain. Use the techniques we’ve
shown here to create side-by-side modules and nested modules in whatever
structure you would like.

### Moving Modules to Other Files

Modules form a hierarchical structure, much like another structure in computing
that you’re used to: file systems! We can use Rust’s module system along with
multiple files to split Rust projects up so that not everything lives in
*src/lib.rs*. For this example, we will start with this code in *src/lib.rs*:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 7-3: Three modules, `client`, `network`, and `network::server` all
defined in *src/lib.rs*

</figcaption>
</figure>

which has this module hierarchy:

```text
communicator
 ├── client
 └── network
     └── server
```

If these modules had many functions, and each function was getting long, we
would have to scroll through this file to find the code we wanted to work with.
This would be a good reason to pull each of the `client`, `network`, and
`server` modules out of *src/lib.rs* and into their own files. Let’s start by
extracting the `client` module into another file. First, replace the `client`
module code in *src/lib.rs* with the following:

<span class="filename">Filename: src/lib.rs</span>

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

<!-- I will add wingdings/ghosting in libreoffice /Carol -->

We’re still *defining* the `client` module here, but by removing the curly
braces and definitions inside the `client` module and replacing them with a
semicolon, we’re letting Rust know to look in another location for the code
defined inside that module.

So now we need to create the external file with that module name. Create a
*client.rs* file in your *src/* directory, then open it up and enter the
following, which is the `connect` function in the `client` module that we
removed in the previous step:

<span class="filename">Filename: src/client.rs</span>

```rust
fn connect() {
}
```

Note that we don’t need a `mod` declaration in this file; that’s because we
already declared the `client` module with `mod` in *src/lib.rs*. This file just
provides the *contents* of the `client` module. If we put a `mod client` here,
we’d be giving the `client` module its own submodule named `client`!

Rust only knows to look in *src/lib.rs* by default. If we want to add more
files to our project, we need to tell Rust in *src/lib.rs* to look in other
files; this is why `mod client` needs to be defined in *src/lib.rs* and can’t
be defined in *src/client.rs*.

Now, everything should compile successfully, though you’ll get a few warnings.
Remember to use `cargo build` instead of `cargo run` since we have a library
crate rather than a binary crate:

```text
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

These warnings tell us that we have functions that are never used. Don’t worry
about those warnings for now; we’ll address them later in the chapter. The good
news is that they’re just warnings; our project was built successfully!

Let’s extract the `network` module into its own file next, using the same
pattern. In *src/lib.rs*, delete the body of the `network` module and add a
semicolon to the declaration, like so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network;
```

Then create a new *src/network.rs* file and enter the following:

<span class="filename">Filename: src/network.rs</span>

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

Notice that we still have a `mod` declaration within this module file;
this is because we still want `server` to be a sub-module of `network`.

Now run `cargo build` again. Success! We have one more module to extract:
`server`. Because it’s a sub-module—that is, a module within a module—our
current tactic of extracting a module into a file named after that module won’t
work. We’re going to try anyway so that we can see the error. First change
*src/network.rs* to have `mod server;` instead of the `server` module’s
contents:

<span class="filename">Filename: src/network.rs</span>

```rust,ignore
fn connect() {
}

mod server;
```

Then create a *src/server.rs* file and enter the contents of the `server`
module that we extracted:

<span class="filename">Filename: src/server.rs</span>

```rust
fn connect() {
}
```

When we try to `cargo build`, we’ll get this error:

<figure>

```text
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

<figcaption>

Listing 7-4: Error when trying to extract the `server` submodule into
*src/server.rs*

</figcaption>
</figure>

The error says we `cannot declare a new module at this location` and is
pointing to the `mod server;` line in *src/network.rs*. So *src/network.rs* is
different than *src/lib.rs* somehow; let’s keep reading to understand why.

The note in the middle of Listing 7-4 is actually pretty helpful, as it points
out something we haven’t yet talked about doing:

> note: maybe move this module `network` to its own directory via
`network/mod.rs`

Instead of continuing to follow the same file naming pattern we used
previously, we can do what the note suggests:

1. Make a new *directory* named *network*, the parent module’s name
2. Move the *src/network.rs* file into the new *network* directory and rename
   it so that it is now *src/network/mod.rs*
3. Move the submodule file *src/server.rs* into the *network* directory

Here are commands to carry out these steps:

```text
$ mkdir src/network
$ mv src/network.rs src/network/mod.rs
$ mv src/server.rs src/network
```

Now if we try to `cargo build`, compilation will work (we’ll still have
warnings though). Our module layout still looks like this, which is exactly the
same as it did when we had all the code in *src/lib.rs* in Listing 7-3:

```text
communicator
 ├── client
 └── network
     └── server
```

The corresponding file layout now looks like this:

```text
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```

So when we wanted to extract the `network::server` module, why did we have to
also change the *src/network.rs* file into the *src/network/mod.rs* file, and
also put the code for `network::server` in the `network` directory in
*src/network/server.rs*, instead of just being able to extract the
`network::server` into *src/server.rs*? The reason is that Rust wouldn’t be
able to tell that `server` was supposed to be a submodule of `network` if the
*server.rs* file was in the *src* directory. To make it clearer why Rust can’t
tell, let’s consider a different example where we have this module hierarchy
with all the definitions in *src/lib.rs*:

```text
communicator
 ├── client
 └── network
     └── client
```

In this example, we have three modules again, `client`, `network`, and
`network::client`. If we follow the same steps we originally did above for
extracting modules into files, for the `client` module we would create
*src/client.rs*. For the `network` module, we would create *src/network.rs*.
Then we wouldn’t be able to extract the `network::client` module into a
*src/client.rs* file, because that already exists for the top-level `client`
module! If we put the code in both the `client` and `network::client` modules
in the *src/client.rs* file, Rust would not have any way to know whether the
code was for `client` or for `network::client`.

Therefore, once we wanted to extract a file for the `network::client` submodule
of the `network` module, we needed to create a directory for the `network`
module instead of a *src/network.rs* file. The code that is in the `network`
module then goes into the *src/network/mod.rs* file, and the submodule
`network::client` can have its own *src/network/client.rs* file. Now the
top-level *src/client.rs* is unambiguously the code that belongs to the
`client` module.

### Rules of Module File Systems

In summary, these are the rules of modules with regards to files:

* If a module named `foo` has no submodules, you should put the declarations
  for `foo` in a file named *foo.rs*.
* If a module named `foo` does have submodules, you should put the declarations
  for `foo` in a file named *foo/mod.rs*.
* The first two rules apply recursively, so that if a module named `foo` has a
  submodule named `bar` and `bar` does not have submodules, you should have the
  following files in your *src* directory:

  ```text
  ├── foo
  │   ├── bar.rs (contains the declarations in `foo::bar`)
  │   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
  ```

* The modules themselves should be declared in their parent module’s file using
  the `mod` keyword.

Next, we’ll talk about the `pub` keyword, and get rid of those warnings!
