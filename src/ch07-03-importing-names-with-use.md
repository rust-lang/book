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
