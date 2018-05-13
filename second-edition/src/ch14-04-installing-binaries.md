## Installing Binaries from Crates.io with `cargo install`

The `cargo install` command allows you to install and use binary crates
locally. This isn’t intended to replace system packages; it’s meant to be a
convenient way for Rust developers to install tools that others have shared on
[crates.io](https://crates.io)<!-- ignore -->. Note that you can only install
packages that have binary targets. A *binary target* is the runnable program
that is created if the crate has a *src/main.rs* file or another file specified
as a binary, as opposed to a library target that isn’t runnable on its own but
is suitable for including within other programs. Usually, crates have
information in the *README* file about whether a crate is a library, has a
binary target, or both.

All binaries installed with `cargo install` are stored in the installation
root’s *bin* folder. If you installed Rust using `rustup` and don’t have any
custom configurations, this directory will be *$HOME/.cargo/bin*. Ensure that
directory is in your `$PATH` to be able to run programs you’ve installed with
`cargo install`.

For example, in Chapter 12 we mentioned that there’s a Rust implementation of
the `grep` tool called `ripgrep` for searching files. If we want to install
`ripgrep`, we can run the following:

```text
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 --snip--
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg
```

The last line of the output shows the location and the name of the installed
binary, which in the case of `ripgrep` is `rg`. As long as the installation
directory is in your `$PATH`, as mentioned previously, you can then run `rg
--help` and start using a faster, rustier tool for searching files!
