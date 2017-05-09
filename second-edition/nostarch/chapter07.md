
[TOC]

# Modules

When you start writing programs in Rust, your code might live solely in the
`main` function. As your code grows, you’ll eventually move functionality out
into other functions, both for re-use and for better organization. By splitting
your code up into smaller chunks, each chunk is easier to understand on its
own. But what happens if you find yourself with too many functions? Rust has a
module system that handles the problem of wanting to re-use code while keeping
your code organized.

In the same way that you extract lines of code into a function, you can extract
functions (and other code like structs and enums too) into different modules. A
*module* is a namespace that contains definitions of functions or types, and
you can choose whether those definitions are visible outside their module
(public) or not (private). Here’s an overview of how modules work:

* You declare a new module with the keyword `mod`
* By default, everything is set as private (including modules). You can use the
  `pub` keyword to make a module public and therefore visible outside of its
  namespace.
* The `use` keyword allows you to bring modules, or the definitions inside
  modules, into scope so that it’s easier to refer to them.

We’ll take a look at each of these parts and see how they fit into the whole.

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

Cargo creates an empty test to help us get our library started, rather than the
“Hello, world!” binary that we get with the `--bin` option. We’ll look at the
`#[]` and `mod tests` syntax a little later, but for now just make sure to
leave it in your *src/lib.rs*.

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
the beginning of the *src/lib.rs* file, above the test code:

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
`connect`, we can add it as shown in Listing 7-1:

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

While in this case, we’re building a library, there’s nothing special about
*src/lib.rs*. We could also make use of submodules in *src/main.rs* as well. In
fact, we can also put modules inside of modules. This can be useful as your
modules grow to keep related functionality organized together and separate
functionality apart. The choice of how you organize your code depends on how
you think about the relationship between the parts of your code. For instance,
the `client` code and its `connect` function might make more sense to users of
our library if it was inside the `network` namespace instead, like in Listing
7-2:

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
*src/lib.rs*. For this example, we will start with the code in Listing 7-3:

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

Listing 7-3: Three modules, `client`, `network`, and `network::server`, all
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

If these modules had many functions, and those functions were getting long, it
would be difficult to scroll through this file to find the code we wanted to
work with. Because the functions are nested inside one or more mod blocks, the
lines of code inside the functions will start getting long as well. These would
be good reasons to pull each of the `client`, `network`, and `server` modules
out of *src/lib.rs* and into their own files.

Let’s start by extracting the `client` module into another file. First, replace
the `client` module code in *src/lib.rs* with the following:

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

Notice that we still have a `mod` declaration within this module file; this is
because we still want `server` to be a sub-module of `network`.

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

When we try to `cargo build`, we’ll get the error shown in Listing 7-4:

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

```text
note: maybe move this module `network` to its own directory via `network/mod.rs`
```

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
put the code for `network::server` in the *network* directory in
*src/network/server.rs*, instead of just being able to extract the
`network::server` module into *src/server.rs*? The reason is that Rust wouldn’t
be able to tell that `server` was supposed to be a submodule of `network` if
the *server.rs* file was in the *src* directory. To make it clearer why Rust
can’t tell, let’s consider a different example with the following module
hierarchy, where all the definitions are in *src/lib.rs*:

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

These rules apply recursively, so that if a module named `foo` has a submodule
named `bar` and `bar` does not have submodules, you should have the following
files in your *src* directory:

```text
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

The modules themselves should be declared in their parent module’s file using
the `mod` keyword.

Next, we’ll talk about the `pub` keyword, and get rid of those warnings!

## Controlling Visibility with `pub`

We resolved the error messages shown in Listing 7-4 by moving the `network` and
`network::server` code into the *src/network/mod.rs* and
*src/network/server.rs* files, respectively. At that point, `cargo build` was
able to build our project, but we still get some warning messages about the
`client::connect`, `network::connect`, and `network::server::connect` functions
not being used:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
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

So why are we receiving these warnings? After all, we’re building a library
with functions that are intended to be used by our *users*, and not necessarily
by us within our own project, so it shouldn’t matter that these `connect`
functions go unused. The point of creating them is that they will be used by
another project and not our own.

To understand why this program invokes these warnings, let’s try using the
`connect` library as if we were another project, calling it externally. To do
that, we’ll create a binary crate in the same directory as our library crate,
by making a *src/main.rs* file containing this code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

We use the `extern crate` command to bring the `communicator` library crate
into scope, because our package actually now contains *two* crates. Cargo
treats *src/main.rs* as the root file of a binary crate, which is separate from
the existing library crate whose root file is *src/lib.rs*. This pattern is
quite common for executable projects: most functionality is in a library crate,
and the binary crate uses that library crate. This way, other programs can also
use the library crate, and it’s a nice separation of concerns.

From the point of view of a crate outside of the `communicator` library looking
in, all of the modules we’ve been creating are within a module that has the
same name as the crate, `communicator`. We call the top-level module of a crate
the *root module*.

Also note that even if we’re using an external crate within a submodule of our
project, the `extern crate` should go in our root module (so in *src/main.rs*
or *src/lib.rs*). Then, in our submodules, we can refer to items from external
crates as if the items are top-level modules.

Our binary crate right now just calls our library’s `connect` function from the
`client` module. However, invoking `cargo build` will now give us an error
after the warnings:

```text
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! This tells us that the `client` module is private, and this is the crux
of the warnings. It’s also the first time we’ve run into the concepts of
*public* and *private* in the context of Rust. The default state of all code in
Rust is private: no one else is allowed to use the code. If you don’t use a
private function within your own program, since your own program is the only
code allowed to use that function, Rust will warn you that the function has
gone unused.

Once we specify that a function like `client::connect` is public, not only will
our call to that function from our binary crate be allowed, the warning that
the function is unused will go away. Marking something public lets Rust know
that we intend for the function to be used by code outside of our program. Rust
considers the theoretical external usage that’s now possible as the function
“being used.” Thus, when something is marked as public, Rust will not require
that it’s used in our own program and will stop warning that the item is unused.

### Making a Function Public

To tell Rust to make something public, we add the `pub` keyword to the start of
the declaration of the item we want to make public. We’ll focus on fixing the
warning that tells us that `client::connect` has gone unused for now, as well
as the “module `client` is private” error from our binary crate. Modify
*src/lib.rs* to make the `client` module public, like so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

mod network;
```

The `pub` goes right before `mod`. Let’s try building again:

```text
<warnings>
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Hooray! We have a different error! Yes, different error messages are a cause
for celebration. The new error says “function `connect` is private”, so let’s
edit *src/client.rs* to make `client::connect` public too:

<span class="filename">Filename: src/client.rs</span>

```rust
pub fn connect() {
}
```

And run `cargo build` again:

```text
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

Unused code warnings don’t always indicate that something needs to be made
public: if you *didn’t* want these functions to be part of your public API,
unused code warnings could be alerting you to code you no longer needed and can
safely delete. They could also be alerting you to a bug, if you had just
accidentally removed all places within your library where this function is
called.

In our case though, we *do* want the other two functions to be part of our
crate’s public API, so let’s mark them as `pub` as well to try to get rid of
the remaining warnings. Modify *src/network/mod.rs* to be:

<span class="filename">Filename: src/network/mod.rs</span>

```rust,ignore
pub fn connect() {
}

mod server;
```

And compile:

```text
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

Hmmm, we’re still getting an unused function warning even though
`network::connect` is set to `pub`. This is because the function is public
within the module, but the `network` module that the function resides in is not
public. We’re working from the interior of the library out this time, where
with `client::connect` we worked from the outside in. We need to change
*src/lib.rs* to make `network` public too:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;
```

Now if we compile, that warning is gone:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Only one warning left! Try to fix this one on your own!

### Privacy Rules

Overall, these are the rules for item visibility:

1. If an item is public, it can be accessed through any of its parent modules.
2. If an item is private, it may be accessed only by the current module and its
  child modules.

### Privacy Examples

Let’s look at a few more examples to get some practice. Create a new library
project and enter the code in Listing 7-5 into your new project’s *src/lib.rs*:

<figure>
<span class="filename">Filename: src/lib.rs</span>

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

<figcaption>

Listing 7-5: Examples of private and public functions, some of which are
incorrect

</figcaption>
</figure>

Before you try to compile this code, make a guess about which lines in `try_me`
function will have errors. Then try compiling to see if you were right, and
read on for discussion of the errors!

#### Looking at the Errors

The `try_me` function is in the root module of our project. The module named
`outermost` is private, but the second privacy rule says the `try_me` function
is allowed to access the `outermost` module since `outermost` is in the current
(root) module, as is `try_me`.

The call to `outermost::middle_function` will work. This is because
`middle_function` is public, and `try_me` is accessing `middle_function`
through its parent module, `outermost`. We determined in the previous paragraph
that this module is accessible.

The call to `outermost::middle_secret_function` will cause a compilation error.
`middle_secret_function` is private, so the second rule applies. The root
module is neither the current module of `middle_secret_function` (`outermost`
is), nor is it a child module of the current module of `middle_secret_function`.

The module named `inside` is private and has no child modules, so it can only
be accessed by its current module, `outermost`. That means the `try_me`
function is not allowed to call `outermost::inside::inner_function` or
`outermost::inside::secret_function` either.

#### Fixing the Errors

Here are some suggestions for changing the code in an attempt to fix the
errors. Before you try each one, make a guess as to whether it will fix the
errors, then compile to see if you’re right and use the privacy rules to
understand why.

* What if the `inside` module was public?
* What if `outermost` was public and `inside` was private?
* What if, in the body of `inner_function`, you called
  `::outermost::middle_secret_function()`? (The two colons at the beginning mean
  that we want to refer to the modules starting from the root module.)

Feel free to design more experiments and try them out!

Next, let’s talk about bringing items into a scope with the `use` keyword.

## Importing Names

We’ve covered how to call functions defined within a module using the module
name as part of the call, as in the call to the `nested_modules` function shown
here in Listing 7-6.

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

<figcaption>

Listing 7-6: Calling a function by fully specifying its enclosing module’s
namespaces

</figcaption>
</figure>

As you can see, referring to the fully qualified name can get quite lengthy.
Luckily, Rust has a keyword to make these calls more concise.

### Concise Imports with `use`

Rust’s `use` keyword works to shorten lengthy function calls by bringing the
modules of the function you want to call into a scope. Here’s an example of
bringing the `a::series::of` module into a binary crate’s root scope:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

The line `use a::series::of;` means that rather than using the full
`a::series::of` path wherever we want to refer to the `of` module, we can use
`of`.

The `use` keyword brings only what we have specified into scope; it does not
bring children of modules into scope. That’s why we still have to say
`of::nested_modules` when we want to call the `nested_modules` function.

We could have chosen to bring the function itself into scope, by instead
specifying the function in the `use` as follows:

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

This allows us to exclude all of the modules and reference the function
directly.

Since enums also form a sort of namespace like modules, we can import an enum’s
variants with `use` as well. For any kind of `use` statement, if you’re
importing multiple items from one namespace, you can list them using curly
braces and commas in the last position, like so:

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
    let green = TrafficLight::Green; // because we didn’t `use` TrafficLight::Green
}
```

### Glob Imports with `*`

To import all the items in a namespace at once, we can use the `*` syntax. For
example:

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

The `*` is called a *glob*, and it will import everything that’s visible inside
of the namespace. Globs should be used sparingly: they are convenient, but you
might also pull in more things than you expected and cause naming conflicts.

### Using `super` to Access a Parent Module

As you now know, when you create a library crate, Cargo makes a `tests` module
for you. Let’s go into more detail about that now. In your `communicator`
project, open *src/lib.rs*.

<span class="filename">Filename: src/lib.rs</span>

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

We’ll explain more about testing in Chapter 12, but parts of this should make
sense now: we have a module named `tests` that lives next to our other modules
and contains one function named `it_works`. Even though there are special
annotations, the `tests` module is just another module! So our module hierarchy
looks like this:

```text
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

Tests are for exercising the code within our library, so let’s try to call our
`client::connect` function from this `it_works` function, even though we’re not
going to be checking any functionality right now:

<span class="filename">Filename: src/lib.rs</span>

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

```text
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

The compilation failed, but why? We don’t need to place `communicator::` in
front of the function like we did in *src/main.rs* because we are definitely
within the `communicator` library crate here. The reason is that paths are
always relative to the current module, which here is `tests`. The only
exception is in a `use` statement, where paths are relative to the crate root
by default. Our `tests` module needs the `client` module in its scope!

So how do we get back up one module in the module hierarchy to be able to call
the `client::connect` function in the `tests` module? In the `tests` module, we
can either use leading colons to let Rust know that we want to start from the
root and list the whole path:

```rust,ignore
::client::connect();
```

Or we can use `super` to move up one module in the hierarchy from our current
module:

```rust,ignore
super::client::connect();
```

These two options don’t look all that different in this example, but if you’re
deeper in a module hierarchy, starting from the root every time would get long.
In those cases, using `super` to get from the current module to sibling modules
is a good shortcut. Plus, if you’ve specified the path from the root in many
places in your code and then you rearrange your modules by moving a subtree to
another place, you’d end up needing to update the path in a lot of places,
which would be tedious.

It would also be annoying to have to type `super::` all the time in each test,
but you’ve already seen the tool for that solution: `use`! The `super::`
functionality changes the path you give to `use` so that it is relative to the
parent module instead of to the root module.

For these reasons, in the `tests` module especially, `use super::something` is
usually the way to go. So now our test looks like this:

<span class="filename">Filename: src/lib.rs</span>

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

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Summary

Now you know techniques for organizing your code! Use these to group related
functionality together, keep files from getting too long, and present a tidy
public API to users of your library.

Next, let’s look at some collection data structures in the standard library
that you can make use of in your nice, neat code!
