# `mod` and the filesystem

Every module in Rust starts with the `mod` keyword. Let's give it a try by
making a new project with Cargo called "modules". This time, instead of a
binary, we're going to make a library: a project that other people would pull
into their projects as a dependency to get the functionality we provided, like
we used the `rand` crate in Chapter 2. So we're not going to use the `--bin`
option like we have before, instead run:

```bash
$ cargo new modules
$ cd modules
```

You'll notice that Cargo generated `src/lib.rs` instead of `src/main.rs` for
us, and inside it we'll find this:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

This is an empty test to help us get our library started, instead of the binary
that says "Hello, world!" that we get with a new binary. We haven't talked
about the `#[]` stuff yet, so let's ignore those for now:

```rust
mod tests {
    fn it_works() {
    }
}
```

This is our first module declaration. As you can see, you use the `mod`
keyword, followed by the name of the module, and then a block of code in curly
braces. Everything inside this block is inside the namespace `tests`. In this
case, we have a single function, `it_works`. If we wanted to try and call this
function from outside the `tests` module, we would say `tests::it_works` rather
than `it_works`.

You could have multiple modules, side-by-side:

```rust
mod tests {
    fn it_works() {
    }
}

mod foo {
    fn it_works() {
    }
}
```

Now we have a `tests::it_works` function and a `foo::it_works` function.

And you can put modules inside of modules:

```rust
mod tests {
    fn it_works() {
    }

    mod foo {
        fn it_works() {
        }
    }
}
```

This gives us `tests::it_works` and `tests::foo::it_works`.

In this way, modules form a tree. The contents of `src/lib.rs` are at the root
of the project's tree, and the submodules form the leaves. Here's what our
first example looks like when thought of this way:

```text
modules
 ├── foo
 └── tests
```

And here's the second:

```text
modules
 └── tests
     └── foo
```

More complicated projects can have a lot of modules.

## Putting modules in another file

Modules form a hierarchical, tree-like structure. So does another thing:
file systems! The module system is the way that we split larger Rust projects up
into multiple files. Let's imagine we have a module layout like this:

```rust
mod foo {
    fn it_works() {
    }
}

mod tests {
    fn it_works() {
    }

    mod bar {
        fn it_works() {
        }
    }
}
```

Let's extract the `foo` module into another file. First, we need to change our
code in `src/lib.rs`:

```rust,ignore
mod foo;

mod tests {
    fn it_works() {
    }

    mod bar {
        fn it_works() {
        }
    }
}
```

We still say `mod foo`, but instead of curly braces, we have a semicolon. This
lets Rust know that we have a module, but it's in another file. Which file is
it in? Open up `src/foo.rs` and put this in it:

```rust
fn it_works() {
}
```

Note that we don't need a `mod` declaration in this file. `mod` is for declaring
a new module, and we've already declared this module in `src/lib.rs`. If we put
a `mod foo` here, we'd be giving the `foo` module its own submodule named `foo`!

Now, everything should compile:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///home/steve/tmp/modules)
src/foo.rs:1:1: 2:2 warning: function is never used: `it_works`,
#[warn(dead_code)] on by default
src/foo.rs:1 fn it_works() {
             ^
src/lib.rs:4:5: 5:6 warning: function is never used: `it_works`,
#[warn(dead_code)] on by default
src/lib.rs:4     fn it_works() {
                 ^
src/lib.rs:8:9: 9:10 warning: function is never used: `it_works`,
#[warn(dead_code)] on by default
src/lib.rs:8         fn it_works() {
                     ^
```

Don't worry about those warnings for now; we'll clear them up in a future
section. They're just warnings, we've built things successfully!

Let's convert the `tests` module next. Change `src/lib.rs` to look like this:

```rust,ignore
mod foo;

mod tests;
```

And then put this in `src/tests.rs`

```rust
fn it_works() {
}

mod bar {
    fn it_works() {
    }
}
```

And then run `cargo build` again. Success! We have one more module to extract:
`bar`. Unfortunately, our current tactic won't work. Let's try it anyway. Modify
`src/tests.rs` to look like this:

```rust,ignore
fn it_works() {
}

mod bar;
```

Put this in `src/bar.rs`

```rust
fn it_works() {
}
```

When we try to `cargo build`, we'll get an error:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///home/steve/tmp/modules)
src/tests.rs:4:5: 4:8 error: cannot declare a new module at this location
src/tests.rs:4 mod bar;
                   ^~~
src/tests.rs:4:5: 4:8 note: maybe move this module `tests` to its own directory
via `tests/mod.rs`
src/tests.rs:4 mod bar;
                   ^~~
src/tests.rs:4:5: 4:8 note: ... or maybe `use` the module `bar` instead of
possibly redeclaring it
src/tests.rs:4 mod bar;
                   ^~~
error: aborting due to previous error
error: Could not compile `modules`.
```

This error is actually pretty helpful. It points out something we didn't know
that we could do yet:

> note: maybe move this module `tests` to its own directory via `tests/mod.rs`

Here's the problem: in our case, we have different names for our modules: `foo`
and `tests::bar`. But what if we had `foo` and `tests::foo`? That's completely
valid, but then which module is `src/foo.rs` for?

So instead, we can do what the error suggests. We'll make a new directory,
move `src/bar.rs` into it, and change `src/tests.rs` to `src/tests/mod.rs`.
Then, we try to build:

```bash
$ mkdir src/tests
$ mv src/bar.rs src/tests
$ mv src/tests.rs src/tests/mod.rs
$ cargo build
   Compiling modules v0.1.0 (file:///home/steve/tmp/modules)
<warnings>
$
```

It works! In summary, these are the two rules of modules with regards to files:

* If a module named `foo` has no submodules, it will be named `foo.rs`.
* If a module named `foo` does have submodules, it will be named `foo/mod.rs`.

Next, we'll talk about the `pub` keyword, and get rid of those warnings!
