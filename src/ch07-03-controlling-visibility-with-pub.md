# Controlling visibility with `pub`

At the end of the last section, we had a project, `modules`, with a layout that
looks like this:

```text
modules
 ├── client
 └── network
     └── server
```

When we compiled it, we got some strange warnings:

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
functions are the public interface that we want our users to use? Well, let's
try using them. Modify `src/lib.rs` to look like this:

Filename: src/lib.rs

```rust,ignore
fn try_me() {
    client::connect();
}

mod client;

mod network;
```

Here, we make a new `try_me()` function, which calls the `connect()` function
from the `client` module, which is the first warning in our build output above.
Invoking `cargo build` will give us an error:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/lib.rs:2:5: 2:20 error: function `connect` is private
src/lib.rs:2     client::connect();
                 ^~~~~~~~~~~~~~~
error: aborting due to previous error
error: Could not compile `modules`.
```

Ah ha! This is why we're getting the warnings: because our functions were not
public, and none of our public functions called them, they weren't being used at
all.

This is the first time we've run into the concepts of 'public' and 'private' in
the context of Rust. There's no keyword to make something private; that's the
default state. To make something public, we need to introduce the `pub` keyword.
It goes before the declaration itself. Let's modify `src/client.rs` to make
`connect()` public:

Filename: src/client.rs

```rust
pub fn connect() {
}
```

The `pub` goes right before `fn`. Let's try building again:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/lib.rs:1:1: 3:2 warning: function is never used: `try_me`,
#[warn(dead_code)] on by default
src/lib.rs:1 fn try_me() {
             ^
src/client.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/client.rs:1 pub fn connect() {
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

It worked! But, funny enough, we forgot to make `try_me()` public, so we've just
added a new warning. Let's make `try_me()` public in `src/lib.rs`:

Filename: src/lib.rs

```rust,ignore
pub fn try_me() {
    client::connect();
}

mod client;

mod network;
```

And try to build:

```bash
$ cargo build
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

Success! We now have two warnings. Let's try to get rid of the first one.
Modify `src/network/mod.rs`:

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

Hmmm, it says this is still dead, even though it's `pub`. This is because,
while the function is public, it's not totally public: the module it's in
is not public. Let's change `src/lib.rs`:

Filename: src/lib.rs

```rust,ignore
pub fn try_me() {
    client::connect();
}

mod client;

pub mod network;
```

Now, we're declaring the `network` module public. Lo and behold, that warning
is gone:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/network/server.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/server.rs:1 fn connect() {
                        ^
```

Only one last warning! So wait, why did we need to make `network` public, but
not `client`? The answer lies in the way that they were used: in the first
case, we're calling `client::connect()` from `try_me()`. But since it's inside a
private module, anyone using our library couldn't call `client::connect()`. So
why does it need to be public? Let's take a deeper look at privacy.

## Privacy rules

FIXME: these are the rules:

* If an item is public, then it can be used externally through any of its
  parent modules.
* If an item is private, it may be accessed by the current module and its
  submodules.


but describing them in a human way is very hard.

