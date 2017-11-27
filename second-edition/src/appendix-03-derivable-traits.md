# C - Derivable Traits

In various places in the book, we discussed the "derive" feature, which
looks like this:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

More specifically, `derive` is an attribute that is applied to a struct or
enum, and generates code that implements the `Debug` trait for `Point`.

The code it generates looks something like this:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl ::std::fmt::Debug for Point {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Point { x: ref __self_0_0, y: ref __self_0_1 } => {
                let mut builder = __arg_0.debug_struct("Point");
                let _ = builder.field("x", &&(*__self_0_0));
                let _ = builder.field("y", &&(*__self_0_1));
                builder.finish()
            }
        }
    }
}
```

As you can see, the generated code doesn't look that great! The compiler doesn't
care, however. But the `derive` attribute has saved us all of the work of writing
this code.

This works with the following traits provided by the standard library:

* `Eq`, `PartialEq`, the traits for the `==` operator.
* `Ord`, `PartialOrd`, the traits for the `<` and `>` operators.
* `Copy` and `Clone`, which control how to make copies of your structs and enums.
* `Hash`, which is used by `HashMap` for its keys.
* `Default` and `Zero`, which provide default or zero values.
* `Debug` and notably, *not* `Display`, the formatting traits.

> If you remember from Chapter 5, `Display` is for end-users, and so is specific
> to your application. As such, we don't provide a way to derive `Display`, as
> there's no way to understand what the correct output should be.

Of course, the code that's generated is specific to each trait; the example above
is only for `Debug`, the code for `Clone` would look quite different! If you'd
like to see the exact code generated, the [`cargo-expand`] package on Crates.io
will show your code after the generation occurs. This requires
a nightly version of Rust.

[`cargo-expand`]: https://crates.io/crates/cargo-expand

## Custom `derive`

The above list is not comprehensive, however: libraries can implement `derive`
for their own types! In this way, the list of traits you can use `derive` with
is truly open-ended. To learn how this is possible, please read the next appendix,
"Macros."