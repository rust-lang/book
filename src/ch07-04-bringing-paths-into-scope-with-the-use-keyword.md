## Bringing Paths into Scope with the `use` Keyword

It might seem like the paths we’ve written to call functions so far are
inconveniently long and repetitive. For example, in Listing 7-7, whether we
chose the absolute or relative path to the `add_to_waitlist` function, every
time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and
`hosting` too. Fortunately, there’s a way to simplify this process. We can
bring a path into a scope once and then call the items in that path as if
they’re local items with the `use` keyword.

In Listing 7-11, we bring the `crate::front_of_house::hosting` module into the
scope of the `eat_at_restaurant` function so we only have to specify
`hosting::add_to_waitlist` to call the `add_to_waitlist` function in
`eat_at_restaurant`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Listing 7-11: Bringing a module into scope with
`use`</span>

Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::front_of_house::hosting` in the crate
root, `hosting` is now a valid name in that scope, just as though the `hosting`
module had been defined in the crate root. Paths brought into scope with `use`
also check privacy, like any other paths.

You can also bring an item into scope with `use` and a relative path. Listing
7-12 shows how to specify a relative path to get the same behavior as in
Listing 7-11.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Listing 7-12: Bringing a module into scope with `use` and
a relative path</span>

### Creating Idiomatic `use` Paths

In Listing 7-11, you might have wondered why we specified `use
crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in
`eat_at_restaurant` rather than specifying the `use` path all the way out to
the `add_to_waitlist` function to achieve the same result, as in Listing 7-13.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Listing 7-13: Bringing the `add_to_waitlist` function
into scope with `use`, which is unidiomatic</span>

Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is
the idiomatic way to bring a function into scope with `use`. Bringing the
function’s parent module into scope with `use` so we have to specify the parent
module when calling the function makes it clear that the function isn’t locally
defined while still minimizing repetition of the full path. The code in Listing
7-13 is unclear as to where `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`,
it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way
to bring the standard library’s `HashMap` struct into the scope of a binary
crate.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Listing 7-14: Bringing `HashMap` into scope in an
idiomatic way</span>

There’s no strong reason behind this idiom: it’s just the convention that has
emerged, and folks have gotten used to reading and writing Rust code this way.

The exception to this idiom is if we’re bringing two items with the same name
into scope with `use` statements, because Rust doesn’t allow that. Listing 7-15
shows how to bring two `Result` types into scope that have the same name but
different parent modules and how to refer to them.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Listing 7-15: Bringing two types with the same name into
the same scope requires using their parent modules.</span>

As you can see, using the parent modules distinguishes the two `Result` types.
If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d
have two `Result` types in the same scope and Rust wouldn’t know which one we
meant when we used `Result`.

### Providing New Names with the `as` Keyword

There’s another solution to the problem of bringing two types of the same name
into the same scope with `use`: after the path, we can specify `as` and a new
local name, or alias, for the type. Listing 7-16 shows another way to write the
code in Listing 7-15 by renaming one of the two `Result` types using `as`.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Listing 7-16: Renaming a type when it’s brought into
scope with the `as` keyword</span>

In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won’t conflict with the `Result` from `std::fmt`
that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are
considered idiomatic, so the choice is up to you!

### Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name available in
the new scope is private. To enable the code that calls our code to refer to
that name as if it had been defined in that code’s scope, we can combine `pub`
and `use`. This technique is called *re-exporting* because we’re bringing
an item into scope but also making that item available for others to bring into
their scope.

Listing 7-17 shows the code in Listing 7-11 with `use` in the root module
changed to `pub use`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: Making a name available for any code to use
from a new scope with `pub use`</span>

By using `pub use`, external code can now call the `add_to_waitlist` function
using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the
`eat_at_restaurant` function could call `hosting::add_to_waitlist` in its
scope, but external code couldn’t take advantage of this new path.

Re-exporting is useful when the internal structure of your code is different
from how programmers calling your code would think about the domain. For
example, in this restaurant metaphor, the people running the restaurant think
about “front of house” and “back of house.” But customers visiting a restaurant
probably won’t think about the parts of the restaurant in those terms. With
`pub use`, we can write our code with one structure but expose a different
structure. Doing so makes our library well organized for programmers working on
the library and programmers calling the library.

### Using External Packages

In Chapter 2, we programmed a guessing game project that used an external
package called `rand` to get random numbers. To use `rand` in our project, we
added this line to *Cargo.toml*:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and any dependencies from [crates.io](https://crates.io/) and
make `rand` available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the crate, `rand`, and listed the items
we wanted to bring into scope. Recall that in the [“Generating a Random
Number”][rand]<!-- ignore --> section in Chapter 2, we brought the `Rng` trait
into scope and called the `rand::thread_rng` function:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Members of the Rust community have made many packages available at
[crates.io](https://crates.io/), and pulling any of them into your package
involves these same steps: listing them in your package’s *Cargo.toml* file and
using `use` to bring items from their crates into scope.

Note that the standard library (`std`) is also a crate that’s external to our
package. Because the standard library is shipped with the Rust language, we
don’t need to change *Cargo.toml* to include `std`. But we do need to refer to
it with `use` to bring items from there into our package’s scope. For example,
with `HashMap` we would use this line:

```rust
use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library
crate.

### Using Nested Paths to Clean Up Large `use` Lists

If we’re using multiple items defined in the same crate or same module,
listing each item on its own line can take up a lot of vertical space in our
files. For example, these two `use` statements we had in the Guessing Game in
Listing 2-4 bring items from `std` into scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Instead, we can use nested paths to bring the same items into scope in one
line. We do this by specifying the common part of the path, followed by two
colons, and then curly brackets around a list of the parts of the paths that
differ, as shown in Listing 7-18.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listing 7-18: Specifying a nested path to bring multiple
items with the same prefix into scope</span>

In bigger programs, bringing many items into scope from the same crate or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!

We can use a nested path at any level in a path, which is useful when combining
two `use` statements that share a subpath. For example, Listing 7-19 shows two
`use` statements: one that brings `std::io` into scope and one that brings
`std::io::Write` into scope.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listing 7-19: Two `use` statements where one is a subpath
of the other</span>

The common part of these two paths is `std::io`, and that’s the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path, as shown in Listing 7-20.

<span class="filename">Filename: src/lib.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Combining the paths in Listing 7-19 into
one `use` statement</span>

This line brings `std::io` and `std::io::Write` into scope.

### The Glob Operator

If we want to bring *all* public items defined in a path into scope, we can
specify that path followed by `*`, the glob operator:

```rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into
the current scope. Be careful when using the glob operator! Glob can make it
harder to tell what names are in scope and where a name used in your program
was defined.

The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the [“How to Write
Tests”][writing-tests]<!-- ignore --> section in Chapter 11. The glob operator
is also sometimes used as part of the prelude pattern: see [the standard
library documentation](../std/prelude/index.html#other-preludes)<!-- ignore -->
for more information on that pattern.

[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
