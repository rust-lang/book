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
try using them as if we were another project using our library. Create a
`src/main.rs` file with this code:

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
build output above. Invoking `cargo build` will now give us an error:

```bash
$ cargo build
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
being used and it will stop warning us.

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
Since we also want the other two functions to be part of our crate's public
API, let's mark them as `pub` as well to get rid of the remaining warnings. If
we *didn't* want these functions to be part of our public API and we got these
warnings, they could be alerting us to code we no longer needed, or a bug if we
just removed the place within our library where we called this function
accidentally.

So to fix the next warning, let's make `network::connect()` public. Modify
`src/network/mod.rs` to be:

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
is not public. We're working from the interior of the library out, this time,
as opposed to with `client` where we worked from the outside in. Let's change
`src/lib.rs` to add the same fix though:

Filename: src/lib.rs

```rust,ignore
pub mod client;

pub mod network;
```

Now, we're declaring that the `network` module should be public as well. Lo and
behold, that warning is gone:

```bash
$ cargo build
   Compiling modules v0.1.0 (file:///projects/modules)
src/network/server.rs:1:1: 2:2 warning: function is never used: `connect`,
#[warn(dead_code)] on by default
src/network/server.rs:1 fn connect() {
                        ^
```

Only one last warning! Try to fix this one on your own!

## Visibility in tests

Remember when we created our crate that Cargo made a `tests` module for us? Let's talk about that now. It was in `src/lib.rs`:

Filename: src/lib.rs

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

We'll explain more about testing in Chapter XX, but we've talked about modules
now, so parts of this should make sense: we have a module named `tests` that
lives next to our other modules and contains one function named `it_works()`.

Since tests are for exercising the code within our library, let's try to call
our `client::connect()` function from this `it_works()` function, even though
we're not going to be testing any functionality:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Run the tests by invoking the `cargo test` command:

```bash
$ cargo test
   Compiling modules v0.1.0 (file:///projects/modules)
src/lib.rs:9:9: 9:24 error: failed to resolve. Use of undeclared type or module
`client` [E0433]
src/lib.rs:9         client::connect();
                     ^~~~~~~~~~~~~~~
src/lib.rs:9:9: 9:24 help: run `rustc --explain E0433` to see a detailed
explanation
```

Why doesn't this work? It's not because we don't have `modules::` in front of
the function like we had in `src/main.rs`: we are definitely within the
`modules` library crate here. It's because we have to be explicit about the
names we want to `use` in scope, even with sibling modules in the same library.
We need to bring `client` in scope, and because `client` is a sibling module to
`tests`, we can move up a level using the `super` module to refer to the parent
module of `tests`:

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

If we run `cargo test` again, the test will pass and the first part of the test
result output will be:

```bash
$ cargo test
   Compiling modules v0.1.0 (file:///projects/modules)
     Running target/debug/modules-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Privacy rules

FIXME: these are the rules:

* If an item is public, then it can be used externally through any of its
  parent modules.
* If an item is private, it may be accessed by the current module and its
  submodules.


but describing them in a human way is very hard.

