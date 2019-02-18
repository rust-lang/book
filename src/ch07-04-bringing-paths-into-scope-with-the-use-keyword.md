## Bringing Paths into Scope with the `use` Keyword

<!-- Liz: All the subsections of this section have to do with `use`, so they
should be HeadC rather than HeadB as you had them. /Carol -->

It may seem like the paths we’ve written to call functions so far are
inconveniently long and repetitive. For example, in Listing 7-7, whether we
chose the absolute or relative path to the `add_to_waitlist` function, every
time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and
`hosting` too. Luckily, there’s a way to simplify this process. We can bring a
path into a scope once and then call the items in that path as if they’re local
items with the `use` keyword.

In Listing 7-11, we bring the `crate::front_of_house::hosting` module into the
scope of the `eat_at_restaurant` function so that we only have to specify
`hosting::add_to_waitlist` to call the `add_to_waitlist` function in
`eat_at_restaurant`.

<span class="filename">Filename: src/lib.rs</span>

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
# fn main() {}
```

<span class="caption">Listing 7-11: Bringing a module into scope with
`use`</span>

Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::front_of_house::hosting` in the crate
root, `hosting` is now a valid name in that scope, just as if the `hosting`
module had been defined in the crate root. Paths brought into scope with `use`
also check privacy, like any other paths.

Specifying a relative path with `use` is slightly different. Instead of
starting from a name in the current scope, you must start the path given to
`use` with the keyword `self`. Listing 7-12 shows how to specify a relative
path to get the same behavior as Listing 7-11.

<span class="filename">Filename: src/lib.rs</span>

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
# fn main() {}
```

<span class="caption">Listing 7-12: Bringing a module into scope with `use` and
a relative path starting with `self`</span>

Note that using `self` in this way might not be necessary in the future; it’s
an inconsistency in the language that Rust developers are working on
eliminating.

### Creating Idiomatic `use` Paths

In Listing 7-11, you may have wondered why we specified `use
crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in
`eat_at_restaurant`, rather than specifying the `use` path all the way out to
the add_to_waitlist function to achieve the same result, as in Listing 7-13:

<!-- Can you specify what we're doing differently here? -->
<!-- Done /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
# fn main() {}
```

<span class="caption">Listing 7-13: Bringing the `add_to_waitlist` function
into scope with `use`, which is unidiomatic</span>

While both Listing 7-11 and 7-13 accomplish the same thing, Listing 7-11 is the
idiomatic way to bring a function into scope with `use`. Bringing the
function’s parent module into scope with `use` so we have to specify the parent
module when calling the function makes it clear that the function isn’t locally
defined, while still minimizing repetition of the full path. The code in
Listing 7-13 is unclear as to where `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`,
it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way
to bring the standard library’s `HashMap` struct into the scope of a binary
crate.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

<span class="caption">Listing 7-14: Bringing `HashMap` into scope in an
idiomatic way</span>

There’s no strong reason behind this idiom; this is just the convention that
has emerged, and folks have gotten used to reading and writing Rust code this
way.

The exception to this idiom is if you are bringing two items with the same name
into scope with `use` statements, as Rust doesn’t allow that. Listing 7-15
shows how to bring two `Result` types into scope that have the same name but
different parent modules, and how to refer to them.

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
#     Ok(())
}

fn function2() -> io::Result<()> {
#     Ok(())
}
```

<span class="caption">Listing 7-15: Bringing two types with the same name into
the same scope requires using their parent modules</span>

As you can see, using the parent modules distinguishes the two `Result` types.
If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d
have two `Result` types in the same scope and Rust wouldn’t know which one we
meant when we used `Result`. Try it and see what compiler error you get!

### Providing New Names with the `as` Keyword

There’s another solution to the problem of bringing two types of the same name
into the same scope with `use`: after the path, we can specify `as` and a new
local name, or alias, for the type. Listing 7-16 shows another way to write the
code from Listing 7-15 by renaming one of the two `Result` types using `as`.

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
#     Ok(())
}

fn function2() -> IoResult<()> {
#     Ok(())
}
```

<span class="caption">Listing 7-16: Renaming a type when it’s brought into
scope with the `as` keyword</span>

In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won’t conflict with the `Result` from `std::fmt`
that we’ve also brought into scope. Both Listing 7-15 and Listing 7-16 are
considered idiomatic, the choice is up to you!

### Re-exporting Names with `pub use`

<!--- I found this paragraph a little difficult to follow and have tried to
clarify -- can you please check my edits and correct anywhere I may have
changed meaning? --->
<!-- Looks fine! /Carol -->

When you bring a name into scope with the `use` keyword, the name available in
the new scope is private. To enable the code that calls your code to refer to
that name as if it had been defined in that code’s scope, you can combine `pub`
and `use`. This technique is called *re-exporting* because you’re bringing an
item into scope but also making that item available for others to bring into
their scope.

Listing 7-17 shows the code from Listing 7-11 with the `use` in the
root module changed to `pub use`.

<span class="filename">Filename: src/lib.rs</span>

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
# fn main() {}
```

<span class="caption">Listing 7-17: Making a name available for any code to use
from a new scope with `pub use`</span>

By using `pub use`, external code can now call the `add_to_waitlist` function
using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the
`eat_at_restaurant` function can call `hosting::add_to_waitlist` in its scope
but external code wouldn’t be allowed to take advantage of this new path.

This is useful when the internal structure of your code is different than the
way programmers calling your code would think about the domain. For example, in
this restaurant metaphor, the people running the restaurant think about “front
of house” and “back of house” but customers visiting a restaurant probably
won’t think about the parts of the restaurant in those terms. With `pub use`,
we can write our code with one structure but expose a different structure to
make our library well organized both for programmers working on the library and
programmers calling the library.

### Using External Packages

In Chapter 2 we programmed a guessing game project that used an external
package, `rand`, to get random numbers. To use `rand` in our project, we added
this line to *Cargo.toml*:

<span class="filename">Filename: Cargo.toml</span>

```toml
[dependencies]
rand = "0.5.5"
```

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and any dependencies from *https://crates.io* and make `rand`
available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the package, `rand`, and listing the items
we wanted to bring into scope. Recall that in the [“Generating a Random
Number”][rand]<!-- ignore --> section in Chapter 2, we brought the `Rng` trait
into scope and called the `rand::thread_rng` function:

```rust,ignore
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

Members of the Rust community have made many packages available at
*https://crates.io*, and pulling any of them in to your package involves these
same steps: listing them in your package’s *Cargo.toml* and using `use` to
bring items into scope.

Note that the standard library (`std`) is also a crate that’s external to your
package. Because the standard library is shipped with the Rust language, you
don’t need to change *Cargo.toml* to include `std`, but you do need to refer to
it with `use` to bring items from there into your package’s scope. For example,
with `HashMap`:

```rust
use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library
crate.

### Using Nested Paths to Clean Up Large `use` Lists

If you’re using multiple items defined in the same package or same module,
listing each item on its own line can take up a lot of vertical space in your
files. For example, these two `use` statements we had in Listing 2-4 in the
Guessing Game both bring items from `std` into scope:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::cmp::Ordering;
use std::io;
// ---snip---
```

Instead, we can use nested paths to bring the same items into scope in one line.
We do this by specifying the common part of the path, then two colons, then
curly brackets around a list of the parts of the paths that differ, as shown in
Listing 7-18.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::{cmp::Ordering, io};
// ---snip---
```

<span class="caption">Listing 7-18: Specifying a nested path to bring multiple
items with the same prefix into scope</span>

In bigger programs, bringing many items into scope from the same package or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!

<!--- Below: is this what you meant? I found deduplicate to be a confusing
word, if that's a technical term, please revert these changes. --->
<!-- This is fine /Carol -->

You can use a nested path at any level in a path, which is useful when
combining two `use` statements that share a subpath. For example, Listing 7-19
shows two `use` statements: one that brings `std::io` into scope and one that
brings `std::io::Write` into scope:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::io;
use std::io::Write;
```

<span class="caption">Listing 7-19: Two `use` statements where one is a
sub-path of the other</span>

The common part of these two paths is `std::io`, and that’s the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path as shown in Listing 7-20.

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::io::{self, Write};
```

<span class="caption">Listing 7-20: Combining the paths from Listing 7-19 into
one `use` statement</span>

This brings both `std::io` and `std::io::Write` into scope.

### The Glob Operator

If you’d like to bring *all* public items defined in a path into scope, you can
specify that path followed by `*`, the glob operator:

```rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections`
into the current scope. Be careful when using the glob operator! Glob can make
it harder to tell what names are in scope and where a name used in your program
was defined.

The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the [“How to Write
Tests”][writing-tests]<!-- ignore --> section of Chapter 11. The glob operator
is also sometimes used as part of the prelude pattern; see [the standard
library documentation](../std/prelude/index.html#other-preludes)<!-- ignore -->
for more information on that pattern.

[packages]: ch07-01-packages-and-crates-for-making-libraries-and-executables.html#packages-and-crates-for-making-libraries-and-executables
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
