## Organizing your Public API

Here, we will describe the properties that a good public API in Rust has in
order to be pleasant for others to use.

### Re-exports with `pub use`

If your internal code organization doesn't match your desired public API
organization, you can re-export items in the way you want them to be organized.

To re-export a name, combine the `pub` keyword with `use`:

```rust
# fn main() {} // this ex needs to be outside main
mod a {
    pub mod namespace {
        pub fn function() {}
    }
}

pub use a::namespace::function;
```

Here, the `a` module is not public to users of our library, so neither are its
children, even though `namespace` and `function` are public *within* our
library. So users of our library couldn't call `a::namespace::function()`
themselves. However, since we've re-exported `function()` with `pub use`,
`function()` will be public. Users can just call `function()` themselves,
directly. This allows us to organize our code internally however we'd like,
while presenting a different external interface.
