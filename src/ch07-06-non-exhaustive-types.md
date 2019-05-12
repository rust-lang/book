## Non-exhaustive Types
Earlier in the chapter, we saw how to make [struct fields and enums public][making_public],
this is one way to allow users of your crate to access the data in your types.

However, there's one problem with this approach, if we add a field or variant, that can cause the
code written by our users to stop compiling, let's take a look at an example.

### What can go wrong with public fields/variants?
Imagine you've expanded on the *rectangles* library that we started in an earlier chapter, and
have made an enum that represents lots of different shapes, as shown in Listing 7-23 below.

```rust
pub enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: u32, },
}
```

<span class="caption">Listing 7-23: A `Shape` enum definition</span>

In the newest version of the library, we extend `Shape` to have a `Pentagon` variant, like in
Listing 7-24.

```rust
pub enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: u32, },
    Pentagon { size_of_edge: u32 },
}
```

<span class="caption">Listing 7-24: A `Shape` enum definition with `Pentagon` variant</span>

But after releasing the new version, we receive a bug report from a user whose code doesn't
compile with the new version. They've included a snippet of their code, shown in Listing 7-25,
and the error they get, shown in Listing 7-26.

```rust,ignore,does_not_compile
use rectangles::{Shape, random_shape};

fn main() {
    match random_shape() {
        Shape::Rectangle { width, height } =>
          println!("we have a rectangle with width {} and height {}", width, height),
        Shape::Circle { radius } =>
          println!("we have a circle with radius {}", radius),
    }
}
```

<span class="caption">Listing 7-25: Existing code using `Shape` that now fails to compile</span>

```text
error[E0004]: non-exhaustive patterns: `Pentagon { .. }` not covered
  --> src/main.rs:12:11
   |
12 |     match random_shape() {
   |           ^^^^^^^^^^^^^^ pattern `Pentagon { .. }` not covered
```

<span class="caption">Listing 7-26: Error from code using `Shape` that now fails to compile</span>

As it turns out, adding a new variant to a public enum is a breaking change for everyone who hasn't
included a `_` arm on their `match` statement. The same is true for fields of structs and enum
variants.

[making_public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md#making-structs-and-enums-public

### Introducing the `#[non_exhaustive]` attribute
Luckily, there's an attribute we can apply to structs, enums and enum variants that tell the
compiler we intend to add new fields or variants in future. We can then rely on the compiler to
enforce that none of the users of our crate use the type in a way that would break if new fields or
variants were added. Let's revisit our `Shape` example, in Listing 7-27, we can see what `Shape`
looks like once we've added the `#[non_exhaustive]` attribute.

```rust
#[non_exhaustive]
pub enum Shape {
    Rectangle { width: u32, height: u32 },
    Circle { radius: u32, },
}
```

<span class="caption">Listing 7-27: A `Shape` enum definition with `#[non_exhaustive]`</span>

After adding the attribute, the compiler will make sure that any users of our crate that match
on `Shape` include a wildcard arm. Let's see what happens when we use the `Shape` from Listing 7-27
with the code from our user in Listing 7-25.

```text
error[E0004]: non-exhaustive patterns: `_` not covered
  --> src/main.rs:12:11
   |
12 |     match random_shape() {
   |           ^^^^^^^^^^^^^^ pattern `_` not covered
   |
= help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
```

<span class="caption">Listing 7-28: Error when exhaustively matching `Shape` defined with `#[non_exhaustive]`</span>

Now that our type is annotated with `#[non_exhaustive]`, the user's code would never have compiled
without a wildcard arm, so when we added the `Pentagon` variant, it would have continued to work!

### Non-exhaustive structs and enum variants
We can also annotate structs and enum variants with `#[non_exhaustive]`. When a struct or enum
variant is marked non-exhaustive, then users of the struct from another crate won't be able to
instantiate the struct. If they could, then their code would fail to compile when a new field is
added and they aren't giving it a value! For example, in Listing 7-29 we define a non-exhaustive
struct `User`.

```rust
#[non_exhaustive]
pub struct User {
    pub name: String,
    pub age: u32,
}
```

<span class="caption">Listing 7-29: A `User` struct definition with `#[non_exhaustive]`</span>

If we try to instantiate a `User` from another crate, like in Listing 7-30, then we get the error
shown in Listing 7-31.

```rust,ignore,does_not_compile
use users::User;

fn main() {
    let u = User { name: "David".to_string(), age: 32 };
}
```

<span class="caption">Listing 7-30: Incorrect example of instantiating a `User` struct with `#[non_exhaustive]`</span>

```text
error[E0639]: cannot create non-exhaustive struct using struct expression
  --> src/main.rs:12:14
   |
12 |     let u = User { name: "David".to_string(), age: 32 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

<span class="caption">Listing 7-31: Error from instantiating a `User` struct with `#[non_exhaustive]`</span>

Just like enums, we also can't exhaustively match on a struct or enum variant marked as
`#[non_exhaustive]` as shown in Listing 7-32 and Listing 7-33.

```rust,ignore,does_not_compile
use users::{User, get_random_user};

fn main() {
    let User { name, age } = get_random_user();
}
```

<span class="caption">Listing 7-32: Incorrect example exhaustively matching `User` struct with `#[non_exhaustive]`</span>

```text
error[E0638]: `..` required with struct marked as non-exhaustive
  --> $DIR/struct.rs:26:9
   |
LL |     let User { name, age } = get_random_user();
   |         ^^^^^^^^^^^^^^^^^^
```

<span class="caption">Listing 7-33: Error from exhaustively matching `User` struct with `#[non_exhaustive]`</span>

## Summary

Rust lets you organize your packages into crates and your crates into modules
so you can refer to items defined in one module from another module. You can do
this by specifying absolute or relative paths. These paths can be brought into
scope with a `use` statement so you can use a shorter path for multiple uses of
the item in that scope. Module code is private by default, but you can make
definitions public by adding the `pub` keyword and backwards compatible with the
`#[non_exhaustive]` attribute.

In the next chapter, we’ll look at some collection data structures in the
standard library that you can use in your neatly organized code.
