# Appendix D - Useful Development Tools

In this appendix, we’ll talk about tools provided by the Rust project that are
useful when developing Rust code.

## Automatic Formatting with `rustfmt`

The tool `rustfmt` reformats your code according to the community code style.
Many projects use `rustfmt` to prevent arguments about which style to use when
writing Rust: everyone formats their code with the tool!

The `rustfmt` tool is not yet at the quality of a version 1.0 release, but
a preview is available for you to use in the meantime. Please give it a try and
let us know how it goes!

To install `rustfmt`:

```
$ rustup component add rustfmt-preview
```

This will give you both `rustfmt` and `cargo-fmt`, similar to how Rust gives
you both `rustc` and `cargo`. To take any Cargo project and format it:

```
$ cargo fmt
```

Running this command will reformat all of the Rust code in the current crate.
This should only change the code style, not the code semantics. For more
information on `rustfmt`, see its documentation at
*https://github.com/rust-lang-nursery/rustfmt*.

## Fix Up Your Code with `rustfix`

If you’ve written code in Rust, you’ve probably seen compiler warnings. For
example, consider this code:

Filename: src/main.rs

```
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

Here, we’re calling the `do_something` function 100 times, but we never use the
variable `i` in the body of the `for` loop. Rust warns us about that:

```
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

The warning suggests that we use `_i` as a name instead: the underscore
indicates that we intend for this variable to be unused. We can automatically
apply that suggestion using the `rustfix` tool by running the command `cargo
fix`:

```
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

If we look at *src/main.rs* again, we’ll see that `cargo fix` has changed the
code:

Filename: src/main.rs

```
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

The `for` loop variable is now named `_i`, and the warning will no longer
appear.

The `cargo fix` command can also be used to transition your code between
different editions of Rust. Editions are covered in Appendix E.

## More Lints with `clippy`

The `clippy` tool is a collection of lints to catch common mistakes and improve
your Rust code.

The `clippy` tool is not yet at the quality of a version 1.0 release, but a
preview is available for you to use in the meantime. Please give it a try and
let us know how it goes!

To install `clippy`:

```
$ rustup component add clippy-preview
```

To take any Cargo project and run clippy’s lints on it:

```
$ cargo clippy
```

For example, if you write a program that uses an approximation of a
mathematical constant such as pi, as this program does:

Filename: src/main.rs

```
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Running `cargo clippy` on this project will result in this error:

```
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.212/index.html#approx_constant
```

This lets you know that Rust has this constant defined more precisely, and that
your program would be more correct if you used the constant instead. This code
doesn’t result in any errors or warnings from `clippy`:

Filename: src/main.rs

```
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

For more information on `clippy`, see its documentation at
*https://github.com/rust-lang-nursery/rust-clippy*.

## IDE Integration Using the Rust Language Server

To help IDE integration, the Rust project distributes the `rls`, which stands
for the Rust Language Server. This tool speaks the Language Server Protocol
described at *http://langserver.org/*, which is a specification for IDEs and
programming languages to communicate with each other. The `rls` can be used by
different clients, such as the Rust plugin for Visual Studio: Code at
*https://marketplace.visualstudio.com/items?itemName=rust-lang.rust*.

The `rls` is not yet at the quality of a version 1.0 release, but a preview is
available for you to use in the meantime. Please give it a try and let us know
how it goes!

To install the `rls`:

```
$ rustup component add rls-preview
```

Then install the language server support in your particular IDE, and you will
gain abilities such as autocompletion, jump to definition, and inline errors.

For more information on the `rls`, see its documentation at
*https://github.com/rust-lang-nursery/rls*.

# Appendix E - Editions

Way back in Chapter 1, we saw that `cargo new` adds a bit of metadata to your
*Cargo.toml* about an `edition`. This appendix talks about what that means!

The Rust language and compiler have a six-week release cycle. This means users
get a constant stream of new features. Other programming languages release
larger changes less often; Rust chooses to release smaller updates more
frequently. After a while, all of those tiny changes add up. But from release
to release, it can be hard to look back and say “Wow, between Rust 1.10 and
Rust 1.31, Rust has changed a lot!”

Every two or three years, the Rust team produces a new *edition* of Rust.
Each edition brings together the features that have landed into a clear
package with fully updated documentation and tooling. New editions ship
as part of the usual six-week release process.

This serves different purposes for different people:

* For active Rust users, it brings together incremental changes into an
  easy-to-understand package.
* For non-users, it signals that some major advancements have landed, which
  might make Rust worth another look.
* For those developing Rust itself, it provides a rallying point for the
  project as a whole.

At the time of writing, there are two editions: Rust 2015 and Rust 2018.
This book is written using Rust 2018 edition idioms.

The `edition` key in *Cargo.toml* indicates which edition your code should be
compiled under. If the key does not exist, it defaults to `2015` for backwards
compatibility reasons.

Each project can choose to opt in to an edition other than the default 2015
edition. By doing so, editions can contain incompatible changes, such as adding
a new keyword that might conflict with identifiers in code or turning warnings
into errors. But unless you opt in to those changes, your code will continue to
compile even as you upgrade the version of the Rust compiler that you use. All
Rust compiler versions support any edition that existed prior to that
compiler’s release, and they can link crates of any supported editions
together. Edition changes only affect the way the compiler initially parses
code. Therefore, if you’re using Rust 2015 and one of your dependencies uses
Rust 2018, your project will compile and be able to use that dependency. The
opposite situation, where your project uses Rust 2018 and a dependency uses
Rust 2015, works as well.

To be clear: most features will be available on all editions. Developers using
any edition of Rust will continue to see improvements as new stable releases
are made. In some cases, however, mainly when new keywords are added, there may
be new features that are only available in later editions. You only need to
switch editions if you want to take advantage of such features.

For more details, the Edition
Guide at *https://rust-lang-nursery.github.io/edition-guide/* is a complete
book about editions, including how to automatically upgrade your code to
a new edition via `cargo fix`.
