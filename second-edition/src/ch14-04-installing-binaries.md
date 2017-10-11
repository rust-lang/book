## Installing Binaries from Crates.io with `cargo install`

The `cargo install` command allows you to install and use binary crates
locally. This isn’t intended to replace system packages; it’s meant to be a
convenient way for Rust developers to install tools that others have shared on
crates.io. Only packages that have binary targets can be installed. A binary
target is the runnable program that gets created if the crate has a
*src/main.rs* or another file specified as a binary, as opposed to a library
target that isn’t runnable on its own but is suitable for including within
other programs. Usually, crates have information in the *README* file about
whether a crate is a library, has a binary target, or both.

All binaries from `cargo install` are put into the installation root’s *bin*
folder. If you installed Rust using *rustup.rs* and don’t have any custom
configurations, this will be `$HOME/.cargo/bin`. Ensure that directory is in
your `$PATH` to be able to run programs you’ve gotten through `cargo install`.

For example, we mentioned in Chapter 12 that there’s a Rust implementation of
the `grep` tool for searching files called `ripgrep`. If we want to install
`ripgrep`, we can run:

```text
$ cargo install ripgrep
Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading ripgrep v0.3.2
 ...snip...
   Compiling ripgrep v0.3.2
    Finished release [optimized + debuginfo] target(s) in 97.91 secs
  Installing ~/.cargo/bin/rg
```

The last line of the output shows the location and the name of the installed
binary, which in the case of `ripgrep` is `rg`. As long as the installation
directory is in your `$PATH` as mentioned above, you can then run `rg --help`
and start using a faster, rustier tool for searching files!
