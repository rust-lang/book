## Importing names with `use`

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

### Glob imports with `*`

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

### Using `super` to access a parent module

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
   Compiling communicator v0.1.0 (file:///projects/communicator)
src/lib.rs:9:9: 9:24 error: failed to resolve. Use of undeclared type or module
`client` [E0433]
src/lib.rs:9         client::connect();
                     ^~~~~~~~~~~~~~~
src/lib.rs:9:9: 9:24 help: run `rustc --explain E0433` to see a detailed
explanation
```

Why doesn't this work? It's not because we don't have `communicator::` in front
of the function like we had in `src/main.rs`: we are definitely within the
`communicator` library crate here. The reason is that paths anywhere except in
a `use` statement are relative to the current module. Our `tests` module
doesn't have a `client` module in its scope!

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
