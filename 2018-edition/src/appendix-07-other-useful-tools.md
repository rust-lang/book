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

## Built-in lints

When the compiler run, it includes a collection of checks, called
*lints*, for potential problems in the source text. Examples of such
problems include blocks of code that are unreachable, or violations of
the recommended programming style.

The problems caught by lints be symptoms of bugs in logic, or they may
be stylistic differences that can impede programmer understanding;
either of these can lead to unexpected behavior in the program.

However, the patterns (or anti-patterns) caught by lints are not
examples of unsoundness nor undefined behavior. Therefore, lints
usually only *warn* about the problematic code, rather than signalling
an error and refusing to finish the compilation.

In other words, lints provide an opinion about your code once you have
managed to satisfy all of the other checks imposed by the Rust
language.

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

Here, we’re calling `do_something` a hundred times. But we never use the
variable `i`. And so Rust warns:

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

The note is referring to the `unused_variables` lint. The note tells
us that the compiler, by default, is set up to warn the developer when
it encounters an unused variable.

Running `rustc -W help` prints out a list of all the lints that are
built into the compiler. You can control the set of lints that are
enabled for your program by changing the settings of the lints on that
list.

Every normal lint has four possible settings, which are ordered
according to how severe an impact it has on the compilation:

 * `allow`:  silently ignores cases that would otherwise trigger the lint,
 * `warn`:   if the lint is triggered, reports a warning but allows the compilation to continue,
 * `deny`:   if the lint is triggered, reports an error and halts the compilation, and
 * `forbid`: just like `deny`, but also reports an error if the annotated code attempts
             to override with a less severe setting for the lint.

Each of the above settings has both a corresponding source code
attribute (e.g. `#[allow(unused_warnings)]`) and `rustc` command-line
option (e.g. `--deny unused_warnings` or `-D unused_warnings`); these
allow you to override the compiler's default setting for any standard
lint. Using a command-line option will apply the change to all of the
source code given to that invocation of `rustc`, while using an
attribute allows you to restrict the scope of the change to small
portions of your code.

For example:

```rust
#![deny(unused_variables)]

fn main() {
    let a = 1;

    #[allow(unused_variables)]
    {
        let b = 3;
    }

    let c = 2;
}
```

will cause the compiler to print:

```text
error: unused variable: `a`
 --> src\main.rs:4:9
  |
4 |     let a = 1;
  |         ^ help: consider using `_a` instead
  |
note: lint level defined here
 --> src\main.rs:1:9
  |
1 | #![deny(unused_variables)]
  |         ^^^^^^^^^^^^^^^^

error: unused variable: `c`
  --> src\main.rs:11:9
   |
11 |     let c = 2;
   |         ^ help: consider using `_c` instead
```

Due to the use of `deny`, the compilation aborted after the lint was
triggered (rather than merely warning like our earlier example).

Note that there was no error about the variable `b`, even though it
was also unused: the nested use of the `allow` attribute overrode
`deny` attribute in the block surrounding it.

Compare that with this example:

```rust
#![forbid(unused_variables)]

fn main() {

    {
        let a = 1;

        #[allow(unused_variables)]
        {
            let b = 3;
        }

        let c = 2;
    }
}
```

which causes the compiler to print:

```text
error[E0453]: allow(unused_variables) overruled by outer forbid(unused_variables)
 --> src\main.rs:6:13
  |
1 | #![forbid(unused_variables)]
  |           ---------------- `forbid` level set here
...
6 |     #[allow(unused_variables)]
  |             ^^^^^^^^^^^^^^^^ overruled by previous forbid
```

Thus, one can use `forbid` to ensure that all of the code within your
crate (or, if you prefer, a specific subtree of the module hierarchy)
does not trigger a particular lint.


## Fix up your code with `rustfix`

Let us revisit the diagnostic from the `do_something` example above.

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
of Rust. Editions are covered in Appendix H.

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