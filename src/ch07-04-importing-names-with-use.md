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
