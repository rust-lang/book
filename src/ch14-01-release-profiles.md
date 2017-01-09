## Release profiles

Cargo supports a notion of *release profiles*. These profiles control various
options for compiling your code and let you configure each profile
independently of the others. You've seen a hint of this feature in the output
of your builds:

```text
$ cargo build
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

The "debug" and "release" notifications here indicate that the compiler is
using different profiles. Cargo supports four profiles:

* `dev`: used for `cargo build`
* `release` used for `cargo build --release`
* `test` used for `cargo test`
* `doc` used for `cargo doc`

We can customize our `Cargo.toml` file with `[profile.*]` sections to tweak
various compiler options for these profiles. For example, here's one of the
default options for the `dev` and `release` profiles:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls how many optimizations Rust will apply to your
code. The setting goes from zero to three. Applying more optimizations takes
more time. When you're compiling very often in development, you'd usually want
compiling to be fast at the expense of the resulting code running slower. When
you're ready to release, it's better to spend more time compiling the one time
that you build your code to trade off for code that will run faster every time
you use that compiled code.

We could override these defaults by changing them in `Cargo.toml`. For example,
if we wanted to use optimization level 1 in development:

```toml
[profile.dev]
opt-level = 1
```

This overrides the default setting of `0`, and now our development builds will
use more optimizations. Not as much as a release build, but a little bit more.

For the full list of settings and the defaults for each profile, see [Cargo's
documentation.][cargodoc]

[cargodoc]: http://doc.crates.io/
