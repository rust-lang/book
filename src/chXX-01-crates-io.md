# Crates.io

[crates.io] is the Rust community's central repository that serves
as a location to discover and download packages. `cargo` is configured to use
it by default to find requested packages.

To depend on a library hosted on [crates.io], add it to your `Cargo.toml`.

[crates.io]: https://crates.io/

### Adding a dependency

If your `Cargo.toml` doesn't already have a `[dependencies]` section, add that,
then list the crate name and version that you would like to use. This example
adds a dependency of the `time` crate:

```toml
[dependencies]
time = "0.1.12"
```

The version string is a [semver] version requirement. The [specifying
dependencies](specifying-dependencies.html) docs have more information about
the options you have here.

[semver]: https://github.com/steveklabnik/semver#requirements

If we also wanted to add a dependency on the `regex` crate, we would not need
to add `[dependencies]` for each crate listed. Here's what your whole
`Cargo.toml` file would look like with dependencies on the `time` and `regex`
crates:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
time = "0.1.12"
regex = "0.1.41"
```

Re-run `cargo build`, and Cargo will fetch the new dependencies and all of
their dependencies, compile them all, and update the `Cargo.lock`:

<pre><code class="language-shell"><span class="gp">$</span> cargo build
<span style="font-weight: bold" class="s1">    Updating</span> registry `https://github.com/rust-lang/crates.io-index`
<span style="font-weight: bold" class="s1"> Downloading</span> memchr v0.1.5
<span style="font-weight: bold" class="s1"> Downloading</span> libc v0.1.10
<span style="font-weight: bold" class="s1"> Downloading</span> regex-syntax v0.2.1
<span style="font-weight: bold" class="s1"> Downloading</span> memchr v0.1.5
<span style="font-weight: bold" class="s1"> Downloading</span> aho-corasick v0.3.0
<span style="font-weight: bold" class="s1"> Downloading</span> regex v0.1.41
<span style="font-weight: bold" class="s1">   Compiling</span> memchr v0.1.5
<span style="font-weight: bold" class="s1">   Compiling</span> libc v0.1.10
<span style="font-weight: bold" class="s1">   Compiling</span> regex-syntax v0.2.1
<span style="font-weight: bold" class="s1">   Compiling</span> memchr v0.1.5
<span style="font-weight: bold" class="s1">   Compiling</span> aho-corasick v0.3.0
<span style="font-weight: bold" class="s1">   Compiling</span> regex v0.1.41
<span style="font-weight: bold" class="s1">   Compiling</span> hello_world v0.1.0 (file:///path/to/project/hello_world)</code></pre>

Our `Cargo.lock` contains the exact information about which revision of all of
these dependencies we used.

Now, if `regex` gets updated, we will still build with the same revision until
we choose to `cargo update`.

You can now use the `regex` library using `extern crate` in `main.rs`.

```
extern crate regex;

use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
```

Running it will show:

<pre><code class="language-shell"><span class="gp">$</span> cargo run
<span style="font-weight: bold" class="s1">     Running</span> `target/hello_world`
Did our date match? true</code></pre>

