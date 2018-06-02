## Referring to Names in Different Modules

We’ve covered how to call functions defined within a module using the module
name as part of the call, as in the call to the `nested_modules` function shown
here in Listing 7-7.

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

<span class="caption">Listing 7-7: Calling a function by fully specifying its
enclosing module’s path</span>

As you can see, referring to the fully qualified name can get quite lengthy.
Fortunately, Rust has a keyword to make these calls more concise.

### Bringing Names into Scope with the `use` Keyword

Rust’s `use` keyword shortens lengthy function calls by bringing the modules of
the function you want to call into scope. Here’s an example of bringing the
`a::series::of` module into a binary crate’s root scope:

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

The `use` keyword brings only what we’ve specified into scope: it does not
bring children of modules into scope. That’s why we still have to use
`of::nested_modules` when we want to call the `nested_modules` function.

We could have chosen to bring the function into scope by instead specifying the
function in the `use` as follows:

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

Doing so allows us to exclude all the modules and reference the function
directly.

Because enums also form a sort of namespace like modules, we can bring an
enum’s variants into scope with `use` as well. For any kind of `use` statement,
if you’re bringing multiple items from one namespace into scope, you can list
them using curly brackets and commas in the last position, like so:

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
    let green = TrafficLight::Green;
}
```

We’re still specifying the `TrafficLight` namespace for the `Green` variant
because we didn’t include `Green` in the `use` statement.

### Bringing All Names into Scope with a Glob

To bring all the items in a namespace into scope at once, we can use the `*`
syntax, which is called the *glob operator*. This example brings all the
variants of an enum into scope without having to list each specifically:

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

The `*` operator will bring into scope all the visible items in the
`TrafficLight` namespace. You should use globs sparingly: they are convenient,
but a glob might also pull in more items than you expected and cause naming
conflicts.

### Using `super` to Access a Parent Module

As you saw at the beginning of this chapter, when you create a library crate,
Cargo makes a `tests` module for you. Let’s go into more detail about that now.
In your `communicator` project, open *src/lib.rs*:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Chapter 11 explains more about testing, but parts of this example should make
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
`client::connect` function from this `it_works` function, even though we won’t
be checking any functionality right now. This won’t work yet:

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
  |         ^^^^^^ Use of undeclared type or module `client`
```

The compilation failed, but why? We don’t need to place `communicator::` in
front of the function, as we did in *src/main.rs*, because we are definitely
within the `communicator` library crate here. The reason is that paths are
always relative to the current module, which here is `tests`. The only
exception is in a `use` statement, where paths are relative to the crate root
by default. Our `tests` module needs the `client` module in its scope!

So how do we get back up one module in the module hierarchy to call the
`client::connect` function in the `tests` module? In the `tests` module, we can
either use leading colons to let Rust know that we want to start from the root
and list the whole path, like this:

```rust,ignore
::client::connect();
```

Or, we can use `super` to move up one module in the hierarchy from our current
module, like this:

```rust,ignore
super::client::connect();
```

These two options don’t look that different in this example, but if you’re
deeper in a module hierarchy, starting from the root every time would make your
code lengthy. In those cases, using `super` to get from the current module to
sibling modules is a good shortcut. Plus, if you’ve specified the path from the
root in many places in your code and then rearrange your modules by moving a
subtree to another place, you’ll end up needing to update the path in several
places, which would be tedious.

It would also be annoying to have to type `super::` in each test, but you’ve
already seen the tool for that solution: `use`! The `super::` functionality
changes the path you give to `use` so it is relative to the parent module
instead of to the root module.

For these reasons, in the `tests` module especially, `use super::something` is
usually the best solution. So now our test looks like this:

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

When we run `cargo test` again, the test will pass, and the first part of the
test result output will be the following:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Summary

Now you know some new techniques for organizing your code! Use these techniques
to group related functionality together, keep files from becoming too long, and
present a tidy public API to your library users.

Next, we’ll look at some collection data structures in the standard library
that you can use in your nice, neat code.
