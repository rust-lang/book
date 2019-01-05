<!--## The Module System-->

<!--- Below: earlier, we included crates and packages in the module system, but
they seem to be excluded here. I've tried to make the structure of the chapter
more indicative of connected topics, take a look at the ToC and feel free to
rearrange heading levels if I've misunderstood anything! --->

## Defining Modules

In reality, the module system includes far more features than just modules. In
this section, we’ll talk about modules, but this chapter will cover the other
aspects that are considered part of the module system, namely *paths* that
allow you to name items; the `use` keyword that brings a path into scope; and
the * `pub` keyword to make items public. We'll also cover using the `as`
keyword, external packages, and the glob operator. For now, modules!

<!--I added a quick motivation for using modules, feel free to replace! -->

*Modules* let us organize code into groups for readability and easy re-use. In
Listing 7-1 we've defined a module named `sound` that contains a function named
`guitar`.

Filename: src/main.rs

```
mod sound {
    fn guitar() {
        // Function body code goes here
    }
}
fn main() {

}
```

Listing 7-1: A `sound` module containing a `guitar` function and a `main`
function

We pen with a `mod` block, which defines a module named `sounds`. Inside this
block, We’ve defined the `guitar` function. This organization [fill in with the
result of organizing code like this]

<!---can you open this part with a small motivation for why we'd want to
organize the code into a hierarchy of modules. And also mention main() and it's
purpose here. I think if we do that once here we can forget about it for the
rest of the chapter. --->

To organize code into a hierarchy of modules, you can nest modules inside of
other modules, as shown in Listing 7-2:

Filename: src/main.rs

```
mod sound {
    mod instrument {
        mod woodwind {
            fn clarinet() {
                // Function body code goes here

            }
        }
    }

    mod voice {

    }
}

fn main() {

}
```

Listing 7-2: Modules inside modules

In this example, we defined a `sound` module in the same way as in Listing 7-1.
We then defined two modules within the `sound` module named `instrument` and
`voice`. The `instrument` module has another module defined within it,
`woodwind`, and that module contains a function named `clarinet`. Here, nesting
allows us to [fill in with what we can do with nesting]

<!-- Above: Can you also say why we'd want to do this, what advantage it has?
-->

Now we mentioned in the “Packages and Crates for Making Libraries and
Executables” section that *src/main.rs* and *src/lib.rs* are called *crate
roots*. They are called crate roots because the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the *module tree*.

Listing 7-3 shows the module tree for the structure in Listing 7-2:

```
crate
 └── sound
     └── instrument
        └── woodwind
     └── voice
```

Listing 7-3: The module tree for the code in Listing 7-2

This tree shows how some of the modules nest inside one another (such as
`woodwind` nests inside `instrument`) and how some modules are *siblings* to
each other, meaning they are defined in the same module (`instrument` and
`voice` are both defined within `sound`). To continue the family metaphor, if
module A is contained inside module, we say say that module A is the *child* of
module B, and that module B is the *parent* of module A. Notice that the entire
module tree is rooted under the implicit module named `crate`.

This tree might remind you of the directory tree of the filesystem you have on
your computer; this is a very apt comparison! Just like directories in a
filesystem, you use modules to organize your code. And just like files in a
directory, we need to have a way to find our modules.

## Paths

To show Rust where to find a module in a module tree, we use a *path*, in the
same way that, when navigating a file system, we use a path. If we want to call
a function, we need to know its path.

A *path* can take two forms:

* An *absolute path* starts from a crate root by using a crate name or a
  literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or
  another identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).

Let's return to our example in Listing 7-2. How do we call the `clarinet`
function? This is the same thing as asking, what’s the path of the `clarinet`
function? In Listing 7-4, we simplified our code a bit by removing some of the
modules. We’ll show two ways to call the `clarinet` function from `main`. Note
that this example won’t compile just yet, we’ll explain why in a bit.

Filename: src/main.rs

```
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here

        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}

```

Listing 7-4: Calling the `clarinet` function using absolute and relative paths

The first time we call the `clarinet` function in `main`, we use an absolute
path. The `clarinet` function is defined in the same crate as `main`, which
means we can use the `crate` keyword to start an absolute path.

After `crate`, we include each of the successive modules until we make our way
to `clarinet`. You could imagine a file system with the same structure, and
we'd specify the path `/sound/instrument/clarinet` to run the `clarinet`
program; using the `crate` name to start from the crate root is like using `/`
to start from the filesystem root in your shell.

The second time we call `clarinet` in `main`, we use a relative path. The path
starts with the `sound`, the name of a module defined at the same level of the
module tree as `main`. Here the filesystem equivalent would be using the path
`sound/instrument/clarinet`. Starting with a name means that the path is
relative.

<!--- Can you outline briefly what effect the path being relative or absolyte
has on the code? I'm not totally clear why or when you'd use either --->

We mentioned that Listing 7-4 won’t compile yet, let’s try to compile it and
find out why not! The error we get is shown in Listing 7-5.

```
$ cargo build
   Compiling sampleproject v0.1.0 (file:///projects/sampleproject)
error[E0603]: module `instrument` is private
  --> src/main.rs:11:19
   |
11 |     crate::sound::instrument::clarinet();
   |                   ^^^^^^^^^^

error[E0603]: module `instrument` is private
  --> src/main.rs:14:12
   |

14 |     sound::instrument::clarinet();
   |            ^^^^^^^^^^
```

Listing 7-5: Compiler errors from building the code in Listing 7-4

<!-- Have we covered private and public modules yet? If not can you quickly say
what the module being private means? -->

The error messages say that module `instrument` is *private*. In other words,
we have the correct paths for the `instrument` module and the `clarinet`
function, but Rust won’t let us use them because it doesn't have access to the
private sections.

<!--I'm trying to tidy up the structure of the chapter a little so we are
nesting subjects that relate to each other, rather than having every be a HeadB
level -- I think that'll help the reader navigate. It's a little tricky since
so many topics are so interrelated, bear with me, I have a plan! -->

<!--In the next section, we'll explore how Rust uses privacy.#### The Privacy
Boundary-->

<!-- Below: Can you define privacy boundary and say what it's useful for
briefly here? What does it mean for an item to be private? -->

Modules are not only useful for organizing your code, they also help define
Rust's *privacy boundary*, the [fill in with what a privacy boundary is] that
[fill in with purpose of privacy boundary]. Thus if you want to make an item
like a function or struct private, you put it in a module.

<!--- I didn't think we needed to have this as bullets, and likewise found the
up/down metaphor confusing. What do you think of this rewrite? --->

There are a few privacy rules in Rust. The first is that all items (functions,
methods, structs, enums, modules, annd constants) are private by default.
Normally in Rust, items in a parent module cannot use the items inside that
module's children, while items in child modules can use the items in their
parent modules. *This is because child modules are private but parent modules
are not.*

<!-- above: Is this line I added true? Can you quickly say why the child can
access the parent but not the other way round? -->

However, you can access items in a child module from a parent module if those
items are made *public*. You can use the `pub` keyword to make an item public.

### Gaining Access to Paths with the `pub` Keyword

<!-- I'm not sure this is an accurate heading, can you fix it? I wanted to
expand a little to show its relation to the path topics that surround it-->

With that understanding, let's look back at the error in Listing 7-5, that told
us the `instrument` module is private. We now know to mark the `instrument`
module with the `pub` keyword so that we can use it from the `main` function,
shown in Listing 7-6.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        fn clarinet() {

            // Function body code goes here

        }
    }
}

fn main() {

    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();

}
```

Listing 7-6: Declaring the `instrument` module as `pub` so that we’re allowed
to use it from `main`

Unfortunately, as you may have just found out, the code in Listing 7-6 still
results in an error (Listing 7-7):

```
$ cargo build

   Compiling sampleproject v0.1.0 (file:///projects/sampleproject)
error[E0603]: function `clarinet` is private
  --> src/main.rs:11:31
   |
11 |     crate::sound::instrument::clarinet();
   |                               ^^^^^^^^

error[E0603]: function `clarinet` is private
  --> src/main.rs:14:24
   |
14 |     sound::instrument::clarinet();
   |                        ^^^^^^^^
```

Listing 7-7: Compiler errors from building the code in Listing 7-6

What happened? Adding the `pub` keyword in front of `mod instrument` makes the
module public. With this change, if we’re allowed to access `sound`, we can
access `instrument`. But the *contents* of `instrument` are still private;
making the module public does not make its contents public. The `pub` keyword
on a module only lets code in its parent module refer to it.

The errors in Listing 7-7 now say that the `clarinet` function is private. The
privacy rules apply to structs, enums, functions, and methods as well as
modules.

Let’s also make the `clarinet` function public by adding the `pub` keyword
before its definition, like in Listing 7-8:

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Listing 7-8: Adding the `pub` keyword to both `mod instrument` and `fn
clarinet` lets us call the function from `main`

Now it'll compile! Let’s look at both the absolute and the relative path and
double check why adding the `pub` keyword lets us use these paths in `main`.

In the absolute path case, we start with `crate`, the root of our crate <!---
is a crate the same as a "module tree"? what does it mean to be the root of the
crate. --->

From here, we have the `sound` module defined in the crate root. The `sound`
module isn’t public, but because the `main` function is defined in the same
module that `sound` is defined (`main` and `sound` are siblings), we’re allowed
to refer to `sound` from `main`. Next is the `instrument` module marked with
`pub`. We can access the parent module of `instrument`, so we’re allowed to
access `instrument`. Finally, the `clarinet` function is marked with `pub` and
we can access its parent module, so this function call works!

In the relative path case, the logic is the same as the absolute path except
for the first step: rather than starting from the crate root, the path starts
from `sound`. The `sound` module is defined within the same module as `main`,
so the relative path starting from the module in which `main` is defined works.
Then because `instrument` and `clarinet` are marked with `pub`, the rest of the
path works and this function call is valid!

### Starting Relative Paths with `super`

You can also construct relative paths that begin in the parent module by using
`super` at the start of the path. This is like starting a filesystem path with
`..`. Why would we want to do this?

Consider the situation in Listing 7-9, where the function `clarinet` calls the
function `breathe_in` by specifying the path to `breathe_in` starting with
`super`:

Filename: src/lib.rs

```
mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // Function body code goes here
}
```

Listing 7-9: Calling a function using a relative path starting with `super`

The `clarinet` function is in the `instrument` module, so we can use `super` to
go to the parent module of `instrument`, which in this case is `crate`, the
root. From there, we look for `breathe_in`, and find it. Success! Here, we used
`super` because[fill in with why we used super as opposed to something else]

<!-- Can you broaden this out and let them know the general reason we would use
super over the other methods here? -->

Another reason you might want to choose to use `super` is that it can simplify
your code updates when you want to change the module hierarchy, as long as the
code defining the item and the code calling the item are moved together. For
example, if we decide to put the `instrument` module and the `breathe_in`
function into a module named `sound`, we would only need to add the `sound`
module, as shown in Listing 7-10.

Filename: src/lib.rs

```
mod sound {
    mod instrument {
        fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {
        // Function body code goes here

    }
}
```

Listing 7-10: Adding a parent module named `sound` doesn’t affect the relative
path `super::breathe_in`

The call to `super::breathe_in` from the `clarinet` function will continue to
work in Listing 7-10 as it did in Listing 7-9, without needing be updated. If
instead of `super::breathe_in` we had used `crate::breathe_in` in the
`clarinet` function, when we add the parent `sound` module, we would need to
update the `clarinet` function to use the path `crate::sound::breathe_in`
instead. Using a relative path can mean fewer updates as you rearrange modules.

### Making Structs and Enums Public

You can also use `pub` to designate structs and enums as public, but there are
a few extra details. If you use `pub` before a struct definition, you make the
struct public, but the struct’s fields will still be private. You can choose to
make each field public or not on a case-by-case basis. In Listing 7-11, we’ve
defined a public `plant::Vegetable` struct with a public `name` field but a
private `id` field.

Filename: src/main.rs

```
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
```

Listing 7-11: A struct with some public fields and some private fields

Because the `name` field of the `plant::Vegetable` struct is public, in `main`
we can write and read to the `name` field using dot notation. Notice, though,
that we’re not allowed to use the `id` field in `main` because `id` is private.
Try uncommenting the line printing the `id` field value to see what error you
get!

Also note that because `plant::Vegetable` has a private field, the struct needs
to provide a public associated function that constructs an instance of
`Vegetable` (we’ve used the conventional name `new` here). If `Vegetable`
didn’t have such a function, we wouldn’t be able to create an instance of
`Vegetable` in `main` because we’re not allowed to set the value of the private
`id` field in `main`.

In contrast, if you make an enum public, all of its variants are then public.
You only need the `pub` before the `enum` keyword, as shown in Listing 7-12.

Filename: src/main.rs

```
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
```

Listing 7-12: Designating an enum as public makes all its variants public

Because we made the `Appetizer` enum public, we’re able to use the `Soup` and
`Salad` variants in `main`.

<!-- Is there a reason rust developers chose to have enum arms public but the
content of structs not, using pub? -->

There’s one more situation involving `pub` that we haven’t covered, and that
concerns our last module system feature: the `use` keyword. Let’s cover `use`
by itself first, and then we’ll show how `pub` and `use` can be combined.

### Bringing Paths Into Scope with the `use` Keyword

It may seem like the paths we’ve written to call functions so far are
inconveniently long and repetitive. For example, in Listing 7-8, whether we
chose the absolute or relative path to the `clarinet` function, every time we
wanted to call `clarinet` we had to specify `sound` and `instrument` too.
Luckily, there’s a way to simplify this process. We can bring a path into a
scope once and then call the items in that path as if they’re local items. To
do this, we use the `use` keyword.

In Listing 7-13, we bring the `crate::sound::instrument` module into the scope
of the `main` function so that we only have to specify `instrument::clarinet`
to call the `clarinet` function in `main`.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Listing 7-13: Bringing a module into scope with `use`

Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::sound::instrument` in the crate root,
`instrument` is now a valid name in that scope, just as if the `instrument`
module had been defined in the crate root. Paths brought into scope with `use`
also check privacy, like any other paths.

Using `use` with a relative path is slightly different. Instead of starting
from a name in the current scope, you must start the path given to `use` with
the keyword `self`. Listing 7-14 shows how to specify a relative path to get
the same behavior as Listing 7-13.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here

        }
    }
}

use self::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

Listing 7-14: Bringing a module into scope with `use` and a relative path
starting with `self`

Note that using `self` in this way might not be necessary in the future; it’s
an inconsistency in the language that Ruse developers are working on
eliminating.

Using `use` with absolute paths make updates easier if you move the code
calling the item to a different place in the module tree but the code defining
the item stays where it is. For example, if we decide to take the code from
Listing 7-13 and move the behavior in the `main` function to a function called
`clarinet_trio`, and then move that function into a module named
`performance_group`, the path specified in `use` wouldn’t need to change, as
shown in Listing 7-15.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here

        }
    }
}

mod performance_group {
    use crate::sound::instrument;
    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();
}
```

Listing 7-15: The absolute path doesn’t need to be updated when moving the code
that calls the item

In contrast, if we made the same change to the code in Listing 7-14 but
specified a relative path, we would need to change `use
self::sound::instrument` to `use super::sound::instrument`.

Choosing whether to use a relative or absolute paths is always a decision
you'll have to make based on your specific project. That said, we tend to
specify absolute paths starting with `crate` as we've found that it's more
likely to move code definitions and item calls independently of each other,
rather than together as we saw in Listing 7-10.

<!--- Above: did this preserve the meaning? --->

### Creating Idiomatic `use` Paths

In Listing 7-13, you may have wondered why we specified `use
crate::sound::instrument` and then called `instrument::clarinet` in `main`,
rather than [explain what you're doing in Listing 7-16] to achieve the same
result, like in Listing 7-16:

<!-- Can you specify what we're doing differently here? -->

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument::clarinet;

fn main() {
    clarinet();
    clarinet();
    clarinet();
}
```

Listing 7-16: Bringing the `clarinet` function into scope with `use`, which is
unidiomatic

While both Listing 7-13 and 7-16 accomplish the same thing, Listing 7-13 is the
idiomatic way to bring a function into scope using `use`. We want to specify
the function’s parent module with `use` so we have to specify the parent module
when calling the function, making it clear that the function isn’t locally
defined, while still minimizing repetition of the full path. The code in
Listing 7-16 is unclear as to where `clarinet` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`,
it's idiomatic to specify the full path. Listing 7-17 shows the idiomatic way
to bring the standard library’s `HashMap` struct into scope.

Filename: src/main.rs

```
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Listing 7-17: Bringing `HashMap` into scope in an idiomatic way

There’s no strong reason behind this idiom; this is just the convention that
has emerged, and folks have gotten used to reading and writing Rust code this
way.

<!--- deleted the example here because it seemed unnecessary. If you want to
keep it, feel free to add it back in! If you do, please update the listing nums
below. --->

The exception to this idiom is if you are using the `use` statements to bring
two items with the same name into scope, as Rust does not allow that. Listing
7-18 shows how to bring two `Result` types that have the same name but
different parent modules into scope, and how to refer to them.

Filename: src/lib.rs

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()> {

}
```

Listing 7-18: Bringing two types with the same name into the same scope
requires using their parent modules

As you can see, using the parent modules distinguishes the two `Result` types.
If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d
have two `Result` types in the same scope and Rust wouldn’t know which one we
meant when we used `Result`. Try it and see what compiler error you get!

## Providing New Names with the `as` Keyword

There’s another solution to the problem of bringing two types of the same name
into the same scope: we can use `as` after the `use` to specify a new local
name, or alias, for the type. Listing 7-19 shows another way to write the code
from Listing 7-18 by renaming one of the two `Result` types using `as`.

Filename: src/lib.rs

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {

}

fn function2() -> IoResult<()> {

}
```

Listing 7-19: Renaming a type when it’s brought into scope with the `as` keyword

In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won’t conflict with the `Result` from `std::fmt`
that we’ve also brought into scope. Both Listing 7-19 and Listing 7-20 are
considered idiomatic, the choice is up to you!

## Re-exporting Names with `pub use`

<!--- I found this paragrah a little difficult to follow and have tried to
clarify -- can you please check my edits and correct anywhere I may have
changed meaning? --->

When you bring a name into scope with the `use` keyword, the name available in
the new scope is private. To enable the code that calls your code to refer to
that name as if it had been defined in that code's scope, you can combine `pub`
and `use`. This technique is called *re-exporting* because you’re bringing an
item into scope but also making that item available for others to bring into
their scope.

Listing 7-20 shows the code from Listing 7-15 with the `use` in the
`performance_group` module changed to `pub use`.

Filename: src/main.rs

```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here

        }
    }

}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }

}

fn main() {
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
```

Listing 7-20: Making a name available for any code to use from a new scope with
`pub use`

By using `pub use`, the `main` function can now call the `clarinet` function
using `performance_group::instrument::clarinet`. If we hadn’t specified `pub
use`, the `clarinet_trio` function can call `instrument::clarinet` in its scope
but `main` wouldn’t be allowed to take advantage of this new path.

## Using External Packages

In Chapter 2 we programmed a guessing game project that used an external
package, `rand`, to get random numbers. To use `rand` in our project, we added
this line to *Cargo.toml*:

Filename: Cargo.toml

```
[dependencies]
rand = "0.5.5"
```

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and any of its dependencies from *https://crates.io* and make it
available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the package, `rand`, and listing the items
we wanted to bring into scope. Recall that in the “Generating a Random Number”
section in Chapter 2, we brought the `Rng` trait into scope and called the
`rand::thread_rng` function:

```
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

Member of the Rust have made many packages that available at
*https://crates.io*, and pulling any of them in to your package involves these
same steps: listing them in your package’s *Cargo.toml* and using `use` to
bring items into scope.

Note that the standard library (`std`) is also a crate that’s external to your
package. Because the standard library is shipped with the Rust language, you
don’t need to change *Cargo.toml* to include `std`, but you do need to refer to
it with `use` to bring items from there into your package’s scope. For example,
with `HashMap`:

```
use std::collections::HashMap;
```

This is an absolute path starting with `std`, the name of the standard library
crate.

### Using Nested Paths to Clean Up Large `use` Lists

If you're using multiples items defined in the same package or same module,
listing each item on its own line can take up a lot of vertical space in your
files. For example, these two `use` statements we had in Listing 2-4 in the
Guessing Game both bring items from `std` into scope:

Filename: src/main.rs

```
use std::cmp::Ordering;
use std::io;
// ---snip---
```

We can instead use nested paths to bring the same items into scope in one line.
We do this by specifying the common part of the path, then two colons, then
curly brackets around a list of the parts of the paths that differ, as shown in
Listing 7-21.

Filename: src/main.rs

```
use std::{cmp::Ordering, io};
// ---snip---
```

Listing 7-21: Specifying a nested path to bring multiple items with the same
prefix into scope

In bigger programs, bringing many items into scope from the same package or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!

<!--- Below: is this what you meant? I found deduplicate to be a confusing
word, if that's a technical term, please revert these changes. --->

You can do use a nested path at any level in a path, which is useful when
combining two `use` statements that share a subpath. For example, Listing 7-22
shows two `use` statements: one that brings `std::io` into scope, and one that
brings `std::io::Write` into scope:

Filename: src/lib.rs

```
use std::io;
use std::io::Write;
```

Listing 7-22: Two `use` statements where one is a sub-path of the other

The common part of these two paths is `std::io`, and that’s the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path as shown in Listing 7-23.

Filename: src/lib.rs

```
use std::io::{self, Write};
```

Listing 7-23: Combining the paths from Listing 7-22 into one `use` statement

This brings both `std::io` and `std::io::Write` into scope.

### The Glob Operator

If you’d like to bring *all* public items defined in a path into scope, you can
specify that path followed by `*`, the glob operator:

```
use std::collections::*;
```

This `use` statements brings all public items defined in `std::collections`
into the current scope. Be careful when using the glob operator! Glob can make
it harder to tell what names are in scope and where a name used in your program
was defined.

The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the “How to Write Tests”
section of Chapter 11. The glob operator is also sometimes used as part of the
prelude pattern; see the standard library documentation at
*../../std/prelude/index.html#other-preludes* for more information on that
pattern.

## Separating Modules into Different Files

So far, all of the examples in this chapter defined multiple modules in one
file. When modules get large, you may want to move their definitions to a
separate file to make the code easier to navigate.

For example, lets take the code in Listing 7-8 and move the `sound` module to
its own file *src/sound.rs* by changing the crate root file (in this case,
*src/main.rs*) so that it contain the code shown in Listing 7-24.

Filename: src/main.rs

```
mod sound;
fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Listing 7-24: Declaring the `sound` module whose body will be in *src/sound.rs*

And *src/sound.rs* gets the definitions from the body of the `sound` module,
shown in Listing 7-25.

Filename: src/sound.rs

```
pub mod instrument {
    pub fn clarinet() {
        // Function body code goes here
    }
}
```

Listing 7-25: Definitions inside the `sound` module in *src/sound.rs*

Using a semicolon after `mod sound`, rather than using a block, tells Rust to
load the contents of the module from another file with the same name as the
module. To continue with our example and extract the `instrument` module to its
own file as well, we change *src/sound.rs* to contain only the declaration of
the `instrument` module:

Filename: src/sound.rs

```
pub mod instrument;
```

Then we create a *src/sound* directory and a file *src/sound/instrument.rs* to
contain the definitions made in the `instrument` module:

Filename: src/sound/instrument.rs

```
pub fn clarinet() {
    // Function body code goes here
}
```

The module tree remains the same and the function calls in `main` will work
without any modification, even though the definitions live in different files.
This lets you move modules to new files as they grow in size.

## Summary

Rust lets you organize your packages into crates and your crates into modules
so you can refer to items defined in one module from another module. You can do
this by specifying absolute or relative paths. These paths can be brought into
scope with a `use` statement so that you can use a shorter path for multiple
uses of the item in that scope. Modules code is private by default, but you can
choose to make definitions public by adding the `pub` keyword.

Next chapter, we’ll look at some collection data structures in the standard
library that you can use in your nice, neatly organized code.
