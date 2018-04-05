# Appendix F - Newest Features

This appendix documents features that have been added to stable Rust since the
main part of the book was completed.


## Field init shorthand

We can initialize a data structure (struct, enum, union) with named
fields, by writing `fieldname` as a shorthand for `fieldname: fieldname`.
This allows a compact syntax for initialization, with less duplication:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    // Using full syntax:
    let peter = Person { name: name, age: age };

    let name = String::from("Portia");
    let age = 27;

    // Using field init shorthand:
    let portia = Person { name, age };

    println!("{:?}", portia);
}
```


## Returning from loops

One of the uses of a `loop` is to retry an operation you know can fail, such as
checking if a thread completed its job. However, you might need to pass the
result of that operation to the rest of your code. If you add it to the `break`
expression you use to stop the loop, it will be returned by the broken loop:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

## Nested groups in `use` declarations

If you have a complex module tree with many different submodules and you need
to import a few items from each one, it might be useful to group all the
imports in the same declaration to keep your code clean and avoid repeating the
base modules’ name.

The `use` declaration supports nesting to help you in those cases, both with
simple imports and glob ones. For example this snippets imports `bar`, `Foo`,
all the items in `baz` and `Bar`:

```rust
# #![allow(unused_imports, dead_code)]
#
# mod foo {
#     pub mod bar {
#         pub type Foo = ();
#     }
#     pub mod baz {
#         pub mod quux {
#             pub type Bar = ();
#         }
#     }
# }
#
use foo::{
    bar::{self, Foo},
    baz::{*, quux::Bar},
};
#
# fn main() {}
```

## Inclusive ranges

Previously, when a range (`..` or `...`) was used as an expression, it had to be
`..`, which is exclusive of the upper bound, while patterns had to use `...`,
which is inclusive of the upper bound. Now, `..=` is accepted as syntax for
inclusive ranges in both expression and range context:

```rust
fn main() {
    for i in 0 ..= 10 {
        match i {
            0 ..= 5 => println!("{}: low", i),
            6 ..= 10 => println!("{}: high", i),
            _ => println!("{}: out of range", i),
        }
    }
}
```

The `...` syntax is still accepted in matches, but it is not accepted in
expressions. `..=` should be preferred.

## 128-bit integers

Rust 1.26.0 added 128-bit integer primitives:

- `u128`: A 128-bit unsigned integer with range [0, 2^128 - 1]
- `i128`: A 128-bit signed integer with range [-(2^127), 2^127 - 1]

These primitives are implemented efficiently via LLVM support. They are
available even on platforms that don’t natively support 128-bit integers and
can be used like the other integer types.

These primitives can be very useful for algorithms that need to use very large
integers efficiently, such as certain cryptographic algorithms.
