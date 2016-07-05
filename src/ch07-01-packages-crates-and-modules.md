# Packages, Crates, & Modules

Now that we've got a better understanding of some of the fundamentals, let's
change gears a bit. Let's talk about the structure of larger Rust programs
and libraries. This will also help you understand the standard library and
its organization, as it is itself a large Rust library!

Rust's system works a bit differently than package management of other
languages that you may have used in the past.It's worth reading this section
carefully; sometimes new Rustaceans have incorrect expectations based on
previous experience.

## Some terminology

First, let's talk about some vocabulary:

* **Crates** are what we call a library or executable
* **Packages** are a collection of crates
* **Modules** allow you to oraganize your code within a crate

A package contains one or more crates, and crates contain one or more modules.

We'll start in the middle, though, as in some sense, crates are what's
fundamental here. We can't talk about a collection of crates without knowing
what a crate is! Let's talk about crates.

## Crates

A 'crate' is Rust's name for a library or binary. We mean 'binary' in the sense
of 'executable program' here: both libraries and binaries are compiled into
machine code.

We've already made a number of crates as we've worked through the book! Each of
the examples we've made is a crate. The simplest crates have only one file: our
`lib.rs` or `main.rs`, depending on if we're making a library or an executable.

Let's make this a bit more concrete. We'll make a simple crate, and then, over
the rest of this section, explore the crate system by changing it. More
specifically, we'll make a crate that does basic math on some numbers: adds one
or subtracts one.

Let's call our new crate `math`. Generate it with Cargo:

```bash
$ cargo new --bin math
$ cd math
```

If you recall from our previous uses of `cargo new --bin`, we have a Hello
World crate already generated for us:

```bash
$ cargo run
$ cargo run
   Compiling math v0.1.0 (file:///path/to/projects/math)
     Running `target/debug/math`
Hello, world!
```

This program is stored in `src/main.rs`. This is, in fact, our first crate.

Let's implement some basic functionality, rather than hello world. Put this in
`src/main.rs`:

```rust
fn main() {
    let x = 5;

    println!("x is: {}", x);

    let result = x - 1;

    println!("x - 1 is: {}", result);

    let result = x + 1;

    println!("x + 1 is: {}", result);
}   
```

I told you we'd be keeping it easy! Give that a `cargo run` and see it print
some numbers out.

Before we move forward, let's extract our addition and subtraction into
functions. It's overkill for now, but will be useful for the example. Change
your code to look like this:

```rust
fn main() {
    let x = 5;

    println!("x is: {}", x);

    let result = subtract_one(x);

    println!("x - 1 is: {}", result);

    let result = add_one(x);

    println!("x + 1 is: {}", result);
}

fn subtract_one(x: i32) -> i32 {
    x - 1
}

fn add_one(x: i32) -> i32 {
    x + 1
}
```

Two very small functions. Let's take them and learn about modules.

## Modules

Our binary crate is very short, and so its code is just fine as it is. However,
real programs are much longer than a few lines. As your code grows, it's
important to give it some kind of organization to make things more clear.
Rust's way of organizing your code is called 'modules'. Each crate is organized
into a tree of modules, with the crate itself as the root of the tree. That's
the real explanation for why it's called the 'crate root'.

We can declare a module with the `mod` keyword. Let's move our two functions
into an 'ops' module, short for 'operations'. Here's the first step, which
won't _quite_ work yet:

```rust,ignore
fn main() {
    let x = 5;

    println!("x is: {}", x);

    let result = ops::subtract_one(x);

    println!("x - 1 is: {}", result);

    let result = ops::add_one(x);

    println!("x + 1 is: {}", result);
}

mod ops {
    pub fn subtract_one(x: i32) -> i32 {
        x - 1
    }
    
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
}
```

This gives us an error:

```text
error: function `subtract_one` is private
let result = ops::subtract_one(x);
             ^~~~~~~~~~~~~~~~~
error: function `add_one` is private
 let result = ops::add_one(x);
              ^~~~~~~~~~~~
```

Before we get into details about this error, I'd like to talk about what we've
changed. First, we used `mod ops {}` to declare a module, and moved our
functions into it. Then, when we called these functions, we needed to update
their names: they went from `add_one()` and `subtract_one()` to
`ops::add_one()` and `ops::subtract_one()`. Now that we've put them into a
module, they're inside its namespace, and so we need to call the functions
by their full name. This full name is sometimes called a 'path'. Modules
work kind of like a filesystem: you can think of modules as directories or
folders.

Now, let's talk about this error.

### Privacy

We got an error that said "function `subtract_one` is private." What's up with
that? Rust allows you to demarcate which portions of your code are for internal
and external use, or in other words, public or private. Everything starts off
as being private by default. So when we tried to compile our program, Rust
complained. We were trying to call a private function in a different module.

We can use the `pub` keyword to make our functions public. Change the function
declarations to look like this:

```rust
# fn main() {
#     let x = 5;
# 
#     println!("x is: {}", x);
# 
#     let result = ops::subtract_one(x);
# 
#     println!("x - 1 is: {}", result);
# 
#     let result = ops::add_one(x);
# 
#     println!("x + 1 is: {}", result);
# }
# 
# mod ops {
     pub fn subtract_one(x: i32) -> i32 {
#         x - 1
#     }
#     
     pub fn add_one(x: i32) -> i32 {
#         x + 1
#     }
# }
```

The `pub` keyword before `fn` will make these functions public, rather than
private.  With this change, our code compiles and runs just fine.

Privacy applies to all kinds of things in Rust: structs and enums, for example,
are also private by default. Not only that, but you can control which parts of
a struct are public and which are private:

```rust
// This struct is public, but its fields are not. Others can have variable
// bindings with `Person`s in them, but cannot access `name` or `age`.
pub struct Person {
    name: String,
    age: i32,
}

// This struct is public, and its fields are too. Others can see it all,
// including the fields.
pub struct Pet {
    pub name: String,
    pub age: i32,
}
```

### Moving modules into an external file

While we've now given our code a bit of organization, our file has actually
gotten a bit larger. At some point, our code will grow to where a single file
is too small. Currently, we've defined our modules in the main file, but we
can also move the contents of a module to another file. Create a new file,
`src/ops.rs`, and put the functions into it:

```rust
pub fn subtract_one(x: i32) -> i32 {
    x - 1
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Note that we no longer have the `mod ops` in this file. Next, open up your
`src/main.rs` and make it look like this:

```rust,ignore
mod ops;

fn main() {
    let x = 5;

    println!("x is: {}", x);

    let result = ops::subtract_one(x);

    println!("x - 1 is: {}", result);

    let result = ops::add_one(x);

    println!("x + 1 is: {}", result);
}
```

We've moved the `mod ops` to the top of the file, and instead of using curly
braces, we have a semicolon. This form of `mod` says "this module's code will
be defined in another file." Which file? There are actually two forms. We've
used the first, the name of the module. `mod ops;` will look for a file named
`ops.rs`.

We have two forms for a reason, though. Imagine a module structure that looked
like this:

```rust,ignore
mod foo {
    mod ops;
}

mod bar {
    mod ops;
}
```

Which `ops.rs` is `foo::ops`, and which one is `bar::ops`? Well, remember how
we said that the module system is sort of similar to a file system before?
The solution is the same: folders. When we say `mod ops;`, we can put our code
in `ops.rs` or `ops/mod.rs`. This second form is required when your module has
sub-modules of its own, in order to disambiguate. So a more complicated example
would look like this:

```rust,ignore
// in main.rs
mod foo;
mod bar;

// in foo/mod.rs
mod ops;

// in bar/mod.rs
mod ops;

// and the code would be in foo/ops.rs and bar/ops.rs
```

You can nest modules to your heart's content!

## Packages

We've learned that crates are made up of modules, but you can also put multiple
crates into a 'package'. In fact, this is really what `cargo new` does: it
generates a package with a single crate. Let's extract our library
functionality into a separate crate. Create a file, `src/lib.rs`, and put this
in it:

```rust,ignore
mod ops;
```

Then, modify your `src/main.rs` to look like this:

```rust,ignore
extern crate math;

fn main() {
    let x = 5;

    println!("x is: {}", x);

    let result = math::ops::subtract_one(x);

    println!("x - 1 is: {}", result);

    let result = math::ops::add_one(x);

    println!("x + 1 is: {}", result);
}
```

This won't quite compile yet. We get an error when typing `cargo build`:

```text
src/ops.rs:1:1: 3:2 warning: function is never used: `subtract_one`, #[warn(dead_code)] on by default
src/ops.rs:1 pub fn subtract_one(x: i32) -> i32 {
src/ops.rs:2     x - 1
src/ops.rs:3 }
src/ops.rs:5:1: 7:2 warning: function is never used: `add_one`, #[warn(dead_code)] on by default
src/ops.rs:5 pub fn add_one(x: i32) -> i32 {
src/ops.rs:6     x + 1
src/ops.rs:7 }
src/main.rs:8:18: 8:41 error: module `ops` is private
src/main.rs:8     let result = math::ops::subtract_one(x);
                               ^~~~~~~~~~~~~~~~~~~~~~~
src/main.rs:12:18: 12:36 error: module `ops` is private
src/main.rs:12     let result = math::ops::add_one(x);
                                ^~~~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
```

It says that `ops` is private! But it wasn't before, and that was okay. Privacy has
two rules:

* If an item is public, then it can be used externally through any of its
  parent modules.
* If an item is private, it may be accessed by the current module and its
  submodules.

So when we had our `ops` mod in `src/main.rs`, we were taking advantage of rule
two: our `ops` module was private. And since it was declared in our crate root,
our crate root is allowed to access it. We were also taking advantage of rule one:
our `subtract_one()` and `add_one()` functions were public, and so was allowed to
be accessed through their parent module, `ops`. These two things in tandem made
calling `ops::subtract_one()` from `src/main.rs` okay.

But now that we've moved our `ops` module into another crate, we are no longer
declaring `ops` ourselves, and therefore, we cannot access it. The solution is
to make `ops` public in `src/lib.rs`:

```rust,ignore
pub mod ops;
```

With this change, everything will run just fine.

This combination of a library crate and a binary crate is common for many
projects. Often, the binary crate will parse and validate command-line
arguments, and then call functions in the library crate to do most of the work.
This nicely separates the logic from the interaction with the outside world,
and as a bonus, others can re-use the library crate if they need to do similar
things.

## Recap

Here's a review of Rust's package, crates, and modules system: Each Rust
program or library is called a crate. A crate is made up of a tree of modules.
You can combine crates into a package. A package can contain at most one
library crate, but as many binary crates as you'd like.

Whew! We've come a long way. Next, let's talk about how to properly handle
errors in Rust.
