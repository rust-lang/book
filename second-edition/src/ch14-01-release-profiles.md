## Customizing Builds with Release Profiles

In Rust *release profiles* are pre-defined, and customizable, profiles with
different configurations, to allow the programmer more control over various
options for compiling your code. Each profile is configured independently of
the others.

Cargo has two main profiles you should know about: the `dev` profile Cargo uses
when you run `cargo build`, and the `release` profile Cargo uses when you run
`cargo build --release`. The `dev` profile is defined with good defaults for
developing, and likewise the `release` profile has good defaults for release
builds.

These names may be familiar from the output of your builds, which shows the
profile used in the build:

```text
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

The “dev” and “release” notifications here indicate that the compiler is using
different profiles.

### Customizing Release Profiles

Cargo has default settings for each of the profiles that apply when there
aren’t any `[profile.*]` sections in the project’s *Cargo.toml* file. By adding
`[profile.*]` sections for any profile we want to customize, we can choose to
override any subset of the default settings. For example, here are the default
values for the `opt-level` setting for the `dev` and `release` profiles:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls how many optimizations Rust will apply to your
code, with a range of zero to three. Applying more optimizations makes
compilation take longer, so if you’re in development and compiling very often,
you’d want compiling to be fast at the expense of the resulting code running
slower. That’s why the default `opt-level` for `dev` is `0`. When you’re ready
to release, it’s better to spend more time compiling. You’ll only be compiling
in release mode once, and running the compiled program many times, so release
mode trades longer compile time for code that runs faster. That’s why the
default `opt-level` for the `release` profile is `3`.

We can choose to override any default setting by adding a different value for
them in *Cargo.toml*. If we wanted to use optimization level 1 in the
development profile, for example, we can add these two lines to our project’s
*Cargo.toml*:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

This overrides the default setting of `0`. Now when we run `cargo build`, Cargo
will use the defaults for the `dev` profile plus our customization to
`opt-level`. Because we set `opt-level` to `1`, Cargo will apply more
optimizations than the default, but not as many as a release build.

For the full list of configuration options and defaults for each profile, see
[Cargo’s documentation](https://doc.rust-lang.org/cargo/).
