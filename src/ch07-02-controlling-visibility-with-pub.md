## Controlling Visibility with `pub`

We resolved the error messages shown in Listing 7-4 by moving the `network` and
`network::server` code into the *src/network/mod.rs* and
*src/network/server.rs* files, respectively. At that point, `cargo build` was
able to build our project, but we still get some warning messages about the
`client::connect`, `network::connect`, and `network::server::connect` functions
not being used:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

So why are we receiving these warnings? After all, we’re building a library
with functions that are intended to be used by our *users*, and not necessarily
by us within our own project, so it shouldn’t matter that these `connect`
functions go unused. The point of creating them is that they will be used by
another project and not our own.

To understand why this program invokes these warnings, let’s try using the
`connect` library as if we were another project, calling it externally. We can
do that by creating a binary crate in the same directory as our library crate,
by making a *src/main.rs* file containing this code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

We use the `extern crate` command to bring the `communicator` library crate
into scope, because our package actually now contains *two* crates. Cargo
treats *src/main.rs* as the root file of a binary crate, which is separate from
the existing library crate whose root file is *src/lib.rs*. This pattern is
quite common for executable projects: most functionality is in a library crate,
and the binary crate uses that library crate. This way, other programs can also
use the library crate, and it’s a nice separation of concerns.

Our binary crate right now just calls our library’s `connect` function from the
`client` module. However, invoking `cargo build` will now give us an error
after the warnings:

```text
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! This tells us that the `client` module is private, and this is the crux
of the warnings. It’s also the first time we’ve run into the concepts of
*public* and *private* in the context of Rust. The default state of all code in
Rust is private: no one else is allowed to use the code. If you don’t use a
private function within your own program, since your own program is the only
code allowed to use that function, Rust will warn you that the function has
gone unused.

Once we specify that a function like `client::connect` is public, not only will
our call to that function from our binary crate be allowed, the warning that
the function is unused will go away. Marking something public lets Rust know
that we intend for the function to be used by code outside of our program. Rust
considers the theoretical external usage that’s now possible as the function
“being used.” Thus, when something is marked as public, Rust will not require
that it’s used in our own program and will stop warning that the item is
unused.

### Making a Function Public

To tell Rust to make something public, we add the `pub` keyword to the start of
the declaration of the item we want to make public. We’ll focus on fixing the
warning that tells us that `client::connect` has gone unused for now, as well
as the “module `client` is private” error from our binary crate. Modify
*src/lib.rs* to make the `client` module public, like so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

mod network;
```

The `pub` goes right before `mod`. Let’s try building again:

```text
<warnings>
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Hooray! We have a different error! Yes, different error messages are a cause
for celebration. The new error says “function `connect` is private”, so let’s
edit `src/client.rs` to make `client::connect` public too:

<span class="filename">Filename: src/client.rs</span>

```rust
pub fn connect() {
}
```

And run `cargo build` again:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

It compiled, and the warning about `client::connect` not being used is gone!

Unused code warnings don’t always indicate that something needs to be made
public: if you *didn’t* want these functions to be part of your public API,
unused code warnings could be alerting you to code you no longer needed and can
safely delete. They could also be alerting you to a bug, if you had just
accidentally removed all places within your library where this function is
called.

In our case though, we *do* want the other two functions to be part of our
crate’s public API, so let’s mark them as `pub` as well to try to get rid of
the remaining warnings. Modify *src/network/mod.rs* to be:

<span class="filename">Filename: src/network/mod.rs</span>

```rust,ignore
pub fn connect() {
}

mod server;
```

And compile:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | pub fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Hmmm, we’re still getting an unused function warning even though
`network::connect` is set to `pub`. This is because the function is public
within the module, but the `network` module that the function resides in is not
public. We’re working from the interior of the library out this time, where
with `client::connect` we worked from the outside in. We need to change
`src/lib.rs` to make `network` public too:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;
```

Now if we compile, that warning is gone:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

Only one warning left! Try to fix this one on your own!

### Privacy Rules

Overall, these are the rules for item visibility:

1. If an item is public, it can be accessed through any of its
  parent modules.
2. If an item is private, it may be accessed only by the current module and its
  child modules.

### Privacy Examples

Let’s look at a few more examples to get some practice. Create a new library
project and enter the code in Listing 7-5 into your new project’s *src/lib.rs*:

<figure>
<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

<figcaption>

Listing 7-5: Examples of private and public functions, some of which are
incorrect

</figcaption>
</figure>

Before you try to compile this code, make a guess about which lines in `try_me`
function will have errors. Then try compiling to see if you were right, and read
on for discussion of the errors!

#### Looking at the Errors

The `try_me` function is in the root module of our project. The module named
`outermost` is private, but the second privacy rule says the `try_me` function
is allowed to access the `outermost` module since `outermost` is in the current
(root) module, as is `try_me`.

The call to `outermost::middle_function` will work. This is because
`middle_function` is public, and `try_me` is accessing `middle_function`
through its parent module, `outermost`. We determined in the previous paragraph
that this module is accessible.

The call to `outermost::middle_secret_function` will cause a compilation error.
`middle_secret_function` is private, so the second rule applies. The root
module is neither the current module of `middle_secret_function` (`outermost`
is), nor is it a child module of the current module of `middle_secret_function`.

The module named `inside` is private and has no child modules, so it can only
be accessed by its current module, `outermost`. That means the `try_me`
function is not allowed to call `outermost::inside::inner_function` or
`outermost::inside::secret_function` either.

#### Fixing the Errors

Here are some suggestions for changing the code in an attempt to fix the
errors. Before you try each one, make a guess as to whether it will fix the
errors, then compile to see if you’re right and use the privacy rules to
understand why.

* What if the `inside` module was public?
* What if `outermost` was public and `inside` was private?
* What if, in the body of `inner_function`, you called
  `::outermost::middle_secret_function()`? (The two colons at the beginning
  mean that we want to refer to the namespaces starting from the root
  namespace.)

Feel free to design more experiments and try them out!

Next, let’s talk about bringing items into a scope with the `use` keyword.
