## Customizing Builds with Release Profiles

In Rust, _release profiles_ are predefined and customizable profiles with
different configurations that allow a programmer to have more control over
various options for compiling code. Each profile is configured independently of
the others.

Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo
build` and the `release` profile Cargo uses when you run `cargo build
--release`. The `dev` profile is defined with good defaults for development,
and the `release` profile has good defaults for release builds.

These profile names might be familiar from the output of your builds:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

The `dev` and `release` are these different profiles used by the compiler.

Cargo has default settings for each of the profiles that apply when you haven't
explicitly added any `[profile.*]` sections in the project’s _Cargo.toml_ file.
By adding `[profile.*]` sections for any profile you want to customize, you
override any subset of the default settings. For example, here are the default
values for the `opt-level` setting for the `dev` and `release` profiles:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls the number of optimizations Rust will apply to
your code, with a range of 0 to 3. Applying more optimizations extends
compiling time, so if you’re in development and compiling your code often,
you’ll want fewer optimizations to compile faster even if the resulting code
runs slower. The default `opt-level` for `dev` is therefore `0`. When you’re
ready to release your code, it’s best to spend more time compiling. You’ll only
compile in release mode once, but you’ll run the compiled program many times,
so release mode trades longer compile time for code that runs faster. That is
why the default `opt-level` for the `release` profile is `3`.

You can override a default setting by adding a different value for it in
_Cargo.toml_. For example, if we want to use optimization level 1 in the
development profile, we can add these two lines to our project’s _Cargo.toml_
file:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

This code overrides the default setting of `0`. Now when we run `cargo build`,
Cargo will use the defaults for the `dev` profile plus our customization to
`opt-level`. Because we set `opt-level` to `1`, Cargo will apply more
optimizations than the default, but not as many as in a release build.

For the full list of configuration options and defaults for each profile, see
[Cargo’s documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).
