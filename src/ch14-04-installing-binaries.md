<!-- Old link, do not remove -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## Installing Binaries with `cargo install`

The `cargo install` command allows you to install and use binary crates
locally. This isn’t intended to replace system packages; it’s meant to be a
convenient way for Rust developers to install tools that others have shared on
[crates.io](https://crates.io/)<!-- ignore -->. Note that you can only install
packages that have binary targets. A _binary target_ is the runnable program
that is created if the crate has a _src/main.rs_ file or another file specified
as a binary, as opposed to a library target that isn’t runnable on its own but
is suitable for including within other programs. Usually, crates have
information in the _README_ file about whether a crate is a library, has a
binary target, or both.

All binaries installed with `cargo install` are stored in the installation
root’s _bin_ folder. If you installed Rust using _rustup.rs_ and don’t have any
custom configurations, this directory will be *$HOME/.cargo/bin*. Ensure that
directory is in your `$PATH` to be able to run programs you’ve installed with
`cargo install`.

For example, in Chapter 12 we mentioned that there’s a Rust implementation of
the `grep` tool called `ripgrep` for searching files. To install `ripgrep`, we
can run the following:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

The second-to-last line of the output shows the location and the name of the
installed binary, which in the case of `ripgrep` is `rg`. As long as the
installation directory is in your `$PATH`, as mentioned previously, you can
then run `rg --help` and start using a faster, Rustier tool for searching files!
