## Customizing Builds with Release Profiles

In Rust *release profiles* are pre-defined, and customizable, profiles with
different configurations, to allow the programmer more control over various
options for compiling your code. Each profile is configured independently of
the others.

<!-- To be clear, are these release profiles pre-defined profiles that you use
for different things? Can you lay that out more explicitly, give a more
detailed definition? That seems super useful, but I'm not sure I'm following
what they actually are. -->
<!-- They are pre-defined, we've tried to clarify /Carol -->

Cargo has four profiles defined with good default configurations for each use
case. Cargo uses the different profiles based on which command you’re running.
The commands correspond to the profiles as shown in Table 14-1:

<!-- Hm, so these profiles aren't built-in, just supported? and used for what
for cargo build? How do you use a particular profile in a build, is it chosen
by default? Do you have to specify? -->
<!-- They are built in with defaults. We've tried to clarify by changing this
to a table and adding some more explanation, is this better? /Carol -->

| Command                 | Profile   |
|-------------------------|-----------|
| `cargo build`           | `dev`     |
| `cargo build --release` | `release` |
| `cargo test`            | `test`    |
| `cargo doc`             | `doc`     |

<span class="caption">Table 14-1: Which profile is used when you run different
Cargo commands</span>

This may be familiar from the output of your builds, which shows the profile
used in the build:

<!-- Above-is that what you meant here? -->
<!-- Yep! /Carol -->

```text
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ cargo build --release
    Finished release [optimized] target(s) in 0.0 secs
```

The “dev” and “release” notifications here indicate that the compiler is
using different profiles.

<!-- which profile is "debug" associated with? As you can probably tell, I'm
not confident in my interpretation here, I think we need more info -->
<!-- Sorry, this was an inconsistency in cargo that we actually noticed while
writing this section and has since been fixed, but then I think we missed
updating this spot. `debug` should be gone. /Carol -->

### Customizing Release Profiles

<!-- Do we mean that the profiles are all already stored in Cargo.toml, or you
have to add the entire code to cargo.toml? It seems like the former from the
writing, but looking through toml files I've made the latter seems to be true.
If you have multiple profiles in the toml, how do you choose which one to use?
-->
<!-- We've tried to clarify below. Please let me know if this is still unclear,
I'm confused about how you're drawing your conclusions. /Carol -->

Cargo has default settings for each of the profiles that apply when there
aren’t any `[profile.*]` sections in the project’s *Cargo.toml* file. By adding
`[profile.*]` sections for any profile we want to customize, we can choose to
override any subset of the default settings. For example, here are the default
values for the `opt-level` setting for the `dev` and `release` profiles:

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

<!-- So do we choose which profile to use when? How do we do that? Or is that
determined automatically by Rust, and if so, how? I think we need to show that
somewhere around here -->
<!-- Which profile is used is determined by which command you're running, which
we tried to show above. I hope the table added above has clarified this, if
not, please suggest further wording above, but the reader should understand
which profile gets used when by this point and I don't think we should repeat
it again here. /Carol -->

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
[Cargo’s documentation](http://doc.rust-lang.org/cargo/).
