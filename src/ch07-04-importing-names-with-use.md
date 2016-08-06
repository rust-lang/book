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

Enums also form this kind of namespace; we can use `use` on those as well:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::Red;
use TrafficLight::Yellow;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green; // because we didn't use TrafficLight::Green
}
```

## Glob imports with `*`

If you'd like to import many names at once, you can use `*` to do so:

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

The `*` is called a 'glob', and it will import everything that's public inside
of the module. Globs should be used sparingly: they are convenient, but you
might also pull in more things than you expected, causing naming conflicts.

## Re-exports with `pub use`

Finally, you can combine the `pub` keyword with `use` to 're-export' something:

```rust
mod a {
    mod namespace {
        pub fn function() {}
    }
}

pub use a::namespace::function;
```

Here, the `a` and `namespace` modules are not public, so users of our library
couldn't call `a::namespace` themselves. However, since we've `pub use`'d
`function`, it will be public. Users can just call `function` themselves,
directly. This allows us to organize our code internally however we'd like,
while presenting a different external interface.
