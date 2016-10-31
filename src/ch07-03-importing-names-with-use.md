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
