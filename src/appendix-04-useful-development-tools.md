## Appendix D - Useful Development Tools

In this appendix, we talk about some useful development tools that the Rust
project provides. We’ll look at automatic formatting, quick ways to apply
warning fixes, a linter, and integrating with IDEs.

### Automatic Formatting with `rustfmt`

The `rustfmt` tool reformats your code according to the community code style.
Many collaborative projects use `rustfmt` to prevent arguments about which
style to use when writing Rust: everyone formats their code using the tool.

To install `rustfmt`, enter the following:

```console
$ rustup component add rustfmt
```

This command gives you `rustfmt` and `cargo-fmt`, similar to how Rust gives you
both `rustc` and `cargo`. To format any Cargo project, enter the following:

```console
$ cargo fmt
```

Running this command reformats all the Rust code in the current crate. This
should only change the code style, not the code semantics. For more information
on `rustfmt`, see [its documentation][rustfmt].

[rustfmt]: https://github.com/rust-lang/rustfmt

### Fix Your Code with `rustfix`

The rustfix tool is included with Rust installations and can automatically fix
some compiler warnings. If you’ve written code in Rust, you’ve probably seen
compiler warnings. For example, consider this code:

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

Here, we’re calling the `do_something` function 100 times, but we never use the
variable `i` in the body of the `for` loop. Rust warns us about that:

```console
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

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

When we look at *src/main.rs* again, we’ll see that `cargo fix` has changed the
code:

<span class="filename">Filename: src/main.rs</span>

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

The `for` loop variable is now named `_i`, and the warning no longer appears.

You can also use the `cargo fix` command to transition your code between
different Rust editions. Editions are covered in Appendix E.

### More Lints with Clippy

The Clippy tool is a collection of lints to analyze your code so you can catch
common mistakes and improve your Rust code.

To install Clippy, enter the following:

```console
$ rustup component add clippy
```

To run Clippy’s lints on any Cargo project, enter the following:

```console
$ cargo clippy
```

For example, say you write a program that uses an approximation of a
mathematical constant, such as pi, as this program does:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

Running `cargo clippy` on this project results in this error:

```text
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/master/index.html#approx_constant
```

This error lets you know that Rust has this constant defined more precisely and
that your program would be more correct if you used the constant instead. You
would then change your code to use the `PI` constant. The following code
doesn’t result in any errors or warnings from Clippy:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

For more information on Clippy, see [its documentation][clippy].

[clippy]: https://github.com/rust-lang/rust-clippy

### IDE Integration Using the Rust Language Server

To help IDE integration, the Rust project distributes the *Rust Language
Server* (`rls`). This tool speaks the [Language Server
Protocol][lsp], which is a specification for IDEs and programming
languages to communicate with each other. Different clients can use the `rls`,
such as [the Rust plug-in for Visual Studio Code][vscode].

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

To install the `rls`, enter the following:

```console
$ rustup component add rls
```

Then install the language server support in your particular IDE; you’ll gain
abilities such as autocompletion, jump to definition, and inline errors.

For more information on the `rls`, see [its documentation][rls].

[rls]: https://github.com/rust-lang/rls
