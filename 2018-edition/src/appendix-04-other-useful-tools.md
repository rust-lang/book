# D - Other useful tools

In this appendix, we'll talk about some additional tools that are provided by
the Rust project, and are useful when developing Rust code.

## Automatic formatting with `rustfmt`

`rustfmt` is a tool that can re-format your code according to community
norms. Many projects use `rustfmt` to prevent arguments about which style to
use when writing Rust: just do what the tool does!

`rustfmt` is not at 1.0 yet, but a preview is available for you to use in
the meantime. Please give it a try and let us know how it goes!

To install `rustfmt`:

```shell
$ rustup component add rustfmt-preview
```

This will give you both `rustfmt` and `cargo-fmt`, similar to how Rust gives
you both `rustc` and `cargo`. To take any Cargo project and format it:

```shell
$ cargo fmt
```

## Fix up your code with `rustfix`

If you’ve written code in Rust before, you’ve probably seen a compiler
warning before. For example, consider this code:

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

Here, we’re calling do_something a hundred times. But we never use the
variable i. And so Rust warns:

```text
> cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src\main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

See how it suggests that we use `_i` as a name instead? We can automatically
apply that suggestion with cargo fix:

```console
> cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src\main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

If we look at `src\main.rs` again, we’ll see that the code has changed:

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

We’re now using `_i`, and the warning will no longer appear.

`cargo fix` can also be used to transition your code between different editions
of Rust. Editions are covered in Appendix E.

## More lints with `clippy`

`clippy` is a bunch of lints to catch common mistakes and improve your Rust
code.

`clippy` is not at 1.0 yet, but a preview is available for you to use in the
meantime. Please give it a try and let us know how it goes!

To install `clippy`:

```shell
$ rustup component add clippy-preview
```

To take any Cargo project and run clippy's lints on it:

```shell
$ cargo clippy
```

## IDE integration with the Rust Language Server

To help IDE integration, the Rust project distributes `rls`, the Rust
Language Server, as in <http://langserver.org/>. This can be used by
different clients, such as [the Rust plugin for Visual Studio:
Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust).

The `rls` is not at 1.0 yet, but a preview is available for you to use in the
meantime. Please give it a try and let us know how it goes!

To install the `rls`:

```shell
$ rustup component add rls-preview
```

Then, install the language server support in your particular IDE, and it
should all work.