# Importing names with `use`

We've seen how we can call functions defined within a module by using the
module name as part of the call, like this:

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

## Glob imports with `*`

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

For example, in this program, imagine that `TrafficLight` is actually coming
from an external `traffic_light` crate we're depending on. We could also happen
to define our own enum named `Blue` that looks like:

```rust
enum Blue {
    Cyan,
    Navy,
    Royal,
}
```

If the traffic_light crate released a new major version and we decided to
upgrade, the crate could have added `Blue` to its `TrafficLight` variants:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
    Blue,
}
```

Since we have imported all the names in `TrafficLight` using `*`, we would now
have a name conflict in our code and just upgrading would cause our crate to no
longer compile:

```bash
src/main.rs:10:1: 14:2 error: a type named `Blue` has already been imported in this module [E0255]
src/main.rs:10 enum Blue {
               ^
src/main.rs:8:5: 8:21 note: previous import of `Blue` here
src/main.rs:8 use TrafficLight::*;
                  ^~~~~~~~~~~~~~~~
```

If, instead, we were explicit about the variants we wanted to import, our code
would not have stopped compiling when we upgraded the `traffic_light` crate.


## Using `super` to access a parent module

Remember when we created our crate that Cargo made a `tests` module for us? Let's talk about that now. It was in `src/lib.rs`:

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
and contains one function named `it_works()`. Even though there are special
annotations, the `tests` module is just another module!

Since tests are for exercising the code within our library, let's try to call
our `client::connect()` function from this `it_works()` function, even though
we're not going to be checking any functionality right now:

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
   Compiling modules v0.1.0 (file:///projects/modules)
src/lib.rs:9:9: 9:24 error: failed to resolve. Use of undeclared type or module
`client` [E0433]
src/lib.rs:9         client::connect();
                     ^~~~~~~~~~~~~~~
src/lib.rs:9:9: 9:24 help: run `rustc --explain E0433` to see a detailed
explanation
```

Why doesn't this work? It's not because we don't have `modules::` in front of
the function like we had in `src/main.rs`: we are definitely within the
`modules` library crate here. It's because we have to be explicit about the
names we want to `use` in scope, even with sibling modules in the same library.
We need to bring `client` in scope.

`use` is relative to the current module, `tests`. We can move up a module level
using `super` to refer to the parent module of `tests`, and from there we can
access the sibling `client` module:

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
   Compiling modules v0.1.0 (file:///projects/modules)
     Running target/debug/modules-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

`super` is generally only used with tests because...?
