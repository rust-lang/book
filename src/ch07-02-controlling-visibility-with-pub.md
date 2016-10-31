## Controlling Visibility with `pub`

Listing 7-4 showed the error message we received when we built our
`communicator` as it was then, warning us that the `connect` function is never
used.

<!--
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 -->
```bash
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

So why are we receiving these errors? After all, we're building a library with
functions that are intended to be used by our *users*, and not necessarily by
us within our own project, so it shouldn't matter that `connect` goes unused.
The point of creating them is that they will be used by another project and not
our own.

To understand why this program invokes these warnings, let's try using the
`connect` library as if we were another project, calling it externally. Create
a `src/main.rs` file and fille it with with this code:

<!--- do they need to create a new binary/cargo project? Or is this within the
communcator library? -->

Filename: src/main.rs

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

<!-- I'm not sure what the phrase "as the crate root of a binary crate" means
or refers to here, I didn't can you expand on that? What is the binary crate
here, I can't see where the binary came from? And what is the exisiting library
create, you mean the `communicator` crate?-->

We use the `extern crate` command to bring the `communicator` library crate
into scope, because our package actually now contains *two* crates: Cargo
treats src/main.rs as the crate root of a binary crate, and the existing
library crate. This pattern is quite common for executable crates: most
functionality is in a library crate, and the executable crate uses that
library. This way, other programs can also use the library crate, and it’s a
nice separation of concerns.

Our binary crate right now just calls our library's `connect` function from the
`client` module. However, invoking `cargo build` will now give us an error
after the warnings:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! This tells us that the `client` module is private, and this is the crux
of the warnings. It's also the first time you've run into the concepts of
'public' and 'private' in the context of Rust. The default state of all
programs in Rust is private, where no one else can possibly use the code. That
menas that so if you don't use a function within your own program, Rust will
warn you that it's gone unused. Once we specify that the function is public,
Rust knows that you intend the functions for external use and considers the
theoretical external usage that's now possible as "being used". Thus, when
something is marked as public, Rust will not require that it's ussed in your
own program and will stop warning that the item is unused.

### Making a Function Public

To tell Rust to make something public, you add the `pub` keyword to the start
of the declaration of the item you want to make public. We'll focus on fixing
that tells us that `client::connect` has gone unused for now. To fix the error,
modify `src/lib.rs` to make the `client` module public, like so:

Filename: src/lib.rs

```rust,ignore
pub mod client;

mod network;
```

The `pub` goes right before `mod`. Let's try building again:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
<warnings>
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Hooray! We have a different error! Yes, different error messages are a cause
for celebration. The new error says "function `connect` is private", so let's
edit `src/client.rs` to make `client::connect` public too:

Filename: src/client.rs

```rust
pub fn connect() {
}
```

And run `cargo build` again:

```bash
 cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
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

> Note: Unused code warnings don't always indicate that something needs to be
> made public: if you *didn't* want these functions to be part of your public
> API, unused code warnings could be alerting you to code you no longer needed
> and can safely delete. They could also be alerting you to a bug, if you had
> just accidentally removed all places within your library where this function
> is called.

In our case though, we *do* want the other two functions to be part of our
crate's public API, so let's mark them as `pub` as well to try to get rid of
the remaining warnings. Modify `src/network/mod.rs` to be:

Filename: src/network/mod.rs

```rust,ignore
pub fn connect() {
}

mod server;
```

And compile:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
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

Hmmm, we're still getting an unused warning even though `connect` is set to
`pub`. This is, because while the function is public within the module, the
`network` module the function resides in is not public. We're working from the
interior of the library out this time, where with `client` where we worked from
the outside in. We need to change `src/lib.rs` to make `network` public too:

Filename: src/lib.rs

```rust,ignore
pub mod client;

pub mod network;
```

Now if we compile, that warning is gone:

```bash
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
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

### A Privacy Example

Let's look at a few more examples to get some practice. Create a new libary
project and enter the code in Listing 7-5 into your new project's `src/lib.rs`:

Filename: src/lib.rs

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

Before you try to compile this code, make a guess about which lines in
`try_me` function will have errors.

When you've made some educated guesses, read on and we'll talk through them!

#### A Privacy Example: Looking at the Errors

The `try_me` function is in the root module of your project. The module named
`outermost` is private, but the second rule says you, the compiler, are allowed
to access it since `outermost` is in your current root module.

The function call `outermost::middle_function()` will work. This is because
`middle_function` is public, and you are accessing it through its parent
module, `outermost`, which we just determined you can access in the previous
paragraph.

`outermost::middle_secret_function()` will cause a compilation error.
`middle_secret_function` is private, so the second rule applies. Your current
root module is neither the current module of `middle_secret_function`
(`outermost` is), nor is it a child module of the current module of
`middle_secret_function`.

The module named `inside` is private and has no child modules, so it can only
be accessed by its current module, `outermost`. That means the `try_me`
function is not allowed to call `outermost::inside::inner_function()` or
`outermost::inside::secret_function()`.

#### A Privacy Example: Fixing the Errors

Here we provide you with some suggestions for fixing the code. Before you try
each one, make a guess as to whether it will fix the errors, then compile to
see if you're right and use the privacy rules to understand why.

* What if the `inside` module was public?
* What if `outside` was public and `inside` was private?
* What if, in the body of `inner_function`, you called
  `::outermost::middle_secret_function()`? (The two colons at the beginning
  mean that we want to refer to the namespaces starting from the root
  namespace.)

Feel free to design more experiments and try them out!

Next, let's talk about bringing items into a scope with the `use` keyword.
