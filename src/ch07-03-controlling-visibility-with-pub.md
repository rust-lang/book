# Controlling visibility with `pub`

At the end of the last section, we had a project, `modules`, and when we compiled it, we got some strange warnings:

```bash
   Compiling modules v0.1.0 (file:///projects/modules)
src/client.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/client.rs:1 fn connect() {
                ^
src/network/mod.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/mod.rs:1 fn connect() {
                     ^
src/network/server.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/server.rs:1 fn connect() {
                        ^
```

Why does this happen? After all, we're building a library. What if these three
functions are the public interface that we want our *users* to use? We won't
necessarily be using them within our own crate, but the point of creating them
is that they *will* be used by another project. Let's try using them as if we
were another project using our library to see what happens and understand why
we're getting these unused function warnings. Create a `src/main.rs` file with
this code:

Filename: src/main.rs

```rust,ignore
extern crate modules;

fn main() {
    modules::client::connect();
}
```

We need the `extern crate` line to bring our `modules` library crate into
scope, because our package actually now contains *two* crates. Cargo treats
src/main.rs as the crate root of a binary crate, and we also have our existing
library crate. This pattern is quite common for executable crates: most
functionality is in a library crate, and the executable crate uses that
library. This way, other programs can also use the library crate, and it’s also
a nice separation of concerns.

Our binary crate right now just calls our library's `connect()` function from
the `client` module; we picked that one since it's the first warning in our
build output above. Invoking `cargo build` will now give us an error after the
warnings:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
<warnings>
src/main.rs:4:5: 4:29 error: module `client` is private
src/main.rs:4     modules::client::connect();
                  ^~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
error: Could not compile `modules`.
```

Ah ha! The `client` module is private. This is the first time we've run into
the concepts of 'public' and 'private' in the context of Rust. There's no
keyword to make something private; that's the default state. In this default
state, no one else could possibly use it, so if we don't use it within our
library crate, Rust will warn us that it's unused. Once we tell Rust something
is public, Rust knows that we intend for code external to our crate to use it,
and Rust considers theoretical external usage that is now possible to count as
being used. Thus, when something is marked as public, Rust will stop warning us
that it is unused.

To tell Rust we want to make something public, we add the `pub` keyword. This
keyword goes before the declaration of the item we want to make public. Let's
modify `src/lib.rs` to make the `client` module public to fix the error we got:

Filename: src/lib.rs

```rust,ignore
pub mod client;

mod network;
```

The `pub` goes right before `mod`. Let's try building again:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
<warnings>
src/main.rs:4:5: 4:29 error: function `connect` is private
src/main.rs:4     modules::client::connect();
                  ^~~~~~~~~~~~~~~~~~~~~~~~
```

Hooray! We have a different error! Yes, different error messages are a cause
for celebration. The new error says "function `connect` is private", so let's
edit `src/client.rs` to make `client::connect()` public:

Filename: src/client.rs

```rust
pub fn connect() {
}
```

And run `cargo build` again:

```bash
 cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/network/mod.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/mod.rs:1 fn connect() {
                     ^
src/network/server.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/server.rs:1 fn connect() {
                        ^
```

It compiled! And the warning about `client::connect()` not being used is gone!

Making functions public isn't the only way to fix unused code warnings: if
we *didn't* want these functions to be part of our public API and we got these
warnings, the warnings could be alerting us to code we no longer needed and
could safely delete. They could also be alerting us to a bug, if we
had just accidentally removed all places within our library where we called
this function.

However, we *do* want the other two functions to be part of our crate's public
API, so let's mark them as `pub` as well to get rid of the remaining warnings.
Modify `src/network/mod.rs` to be:

Filename: src/network/mod.rs

```rust,ignore
pub fn connect() {
}

mod server;
```

And compile:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/network/mod.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/mod.rs:1 pub fn connect() {
                     ^
src/network/server.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/server.rs:1 fn connect() {
                        ^
```

Hmmm, it says this is still dead, even though it's `pub`. While the function is
public within the module, the `network` module it's in is not public. We're
working from the interior of the library out, this time, as opposed to with
`client` where we worked from the outside in. Let's change `src/lib.rs` to add
the same fix though, by making `network` public like `client` is:

Filename: src/lib.rs

```rust,ignore
pub mod client;

pub mod network;
```

Now if we compile, that warning is gone:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/network/server.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/server.rs:1 fn connect() {
                        ^
```

Only one last warning! Try to fix this one on your own!

## Privacy rules

FIXME: these are the rules:

* If an item is public, then it can be used externally through any of its
  parent modules.
* If an item is private, it may be accessed by the current module and its
  submodules.


but describing them in a human way is very hard.

