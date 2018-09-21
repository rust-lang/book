# G - Other useful tools

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