## Controlling Visibility with `pub`

We resolved the error messages shown in Listing 7-5 by moving the `network` and
`network::server` code into the *src/network/mod.rs* and
*src/network/server.rs* files, respectively. At that point, `cargo build` was
able to build our project, but we still get warning messages about the
`client::connect`, `network::connect`, and `network::server::connect` functions
not being used.

So why are we receiving these warnings? After all, we’re building a library
with functions that are intended to be used by our *users*, not necessarily by
us within our own project, so it shouldn’t matter that these `connect`
functions go unused. The point of creating them is that they will be used by
another project, not our own.

To understand why this program invokes these warnings, let’s try using the
`communicator` library from another project, calling it externally. To do that,
we’ll create a binary crate in the same directory as our library crate by
making a *src/main.rs* file containing this code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

We use the `extern crate` command to bring the `communicator` library crate
into scope. Our package now contains *two* crates. Cargo treats *src/main.rs*
as the root file of a binary crate, which is separate from the existing library
crate whose root file is *src/lib.rs*. This pattern is quite common for
executable projects: most functionality is in a library crate, and the binary
crate uses that library crate. As a result, other programs can also use the
library crate, and it’s a nice separation of concerns.

From the point of view of a crate outside the `communicator` library looking
in, all the modules we’ve been creating are within a module that has the same
name as the crate, `communicator`. We call the top-level module of a crate the
*root module*.

Also note that even if we’re using an external crate within a submodule of our
project, the `extern crate` should go in our root module (so in *src/main.rs*
or *src/lib.rs*). Then, in our submodules, we can refer to items from external
crates as if the items are top-level modules.

Right now, our binary crate just calls our library’s `connect` function from
the `client` module. However, invoking `cargo build` will now give us an error
after the warnings:

```text
error[E0603]: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! This error tells us that the `client` module is private, which is the
crux of the warnings. It’s also the first time we’ve run into the concepts of
*public* and *private* in the context of Rust. The default state of all code in
Rust is private: no one else is allowed to use the code. If you don’t use a
private function within your program, because your program is the only code
allowed to use that function, Rust will warn you that the function has gone
unused.

After you specify that a function such as `client::connect` is public, not only
will your call to that function from your binary crate be allowed, but also the
warning that the function is unused will go away. Marking a function as public
lets Rust know that the function will be used by code outside of your program.
Rust considers the theoretical external usage that’s now possible as the
function “being used.” Thus, when a function is marked public, Rust will not
require that it be used in your program and will stop warning that the function
is unused.

### Making a Function Public

To tell Rust to make a function public, we add the `pub` keyword to the start
of the declaration. We’ll focus on fixing the warning that indicates
`client::connect` has gone unused for now, as well as the `` module `client` is
private `` error from our binary crate. Modify *src/lib.rs* to make the
`client` module public, like so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

mod network;
```

The `pub` keyword is placed right before `mod`. Let’s try building again:

```text
error[E0603]: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Hooray! We have a different error! Yes, different error messages are a cause
for celebration. The new error shows `` function `connect` is private ``, so
let’s edit *src/client.rs* to make `client::connect` public too:

<span class="filename">Filename: src/client.rs</span>

```rust
pub fn connect() {
}
```

Now run `cargo build` again:

```text
warning: function is never used: `connect`
 --> src/network/mod.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
```

The code compiled, and the warning that `client::connect` is not being used is
gone!

Unused code warnings don’t always indicate that an item in your code needs to
be made public: if you *didn’t* want these functions to be part of your public
API, unused code warnings could be alerting you to code you no longer need that
you can safely delete. They could also be alerting you to a bug if you had just
accidentally removed all places within your library where this function is
called.

But in this case, we *do* want the other two functions to be part of our
crate’s public API, so let’s mark them as `pub` as well to get rid of the
remaining warnings. Modify *src/network/mod.rs* to look like the following:

<span class="filename">Filename: src/network/mod.rs</span>

```rust,ignore
pub fn connect() {
}

mod server;
```

Then compile the code:

```text
warning: function is never used: `connect`
 --> src/network/mod.rs:1:1
  |
1 | / pub fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
```

Hmmm, we’re still getting an unused function warning, even though
`network::connect` is set to `pub`. The reason is that the function is public
within the module, but the `network` module that the function resides in is not
public. We’re working from the interior of the library out this time, whereas
with `client::connect` we worked from the outside in. We need to change
*src/lib.rs* to make `network` public too, like so:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;
```

Now when we compile, that warning is gone:

```text
warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default
```

Only one warning is left—try to fix this one on your own!

### Privacy Rules

Overall, these are the rules for item visibility:

- If an item is public, it can be accessed through any of its parent modules.
- If an item is private, it can be accessed only by its immediate parent
  module and any of the parent’s child modules.

### Privacy Examples

Let’s look at a few more privacy examples to get some practice. Create a new
library project and enter the code in Listing 7-6 into your new project’s
*src/lib.rs*:

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

<span class="caption">Listing 7-6: Examples of private and public functions,
some of which are incorrect</span>

Before you try to compile this code, make a guess about which lines in the
`try_me` function will have errors. Then, try compiling the code to see whether
you were right—and read on for the discussion of the errors!

#### Looking at the Errors

The `try_me` function is in the root module of our project. The module named
`outermost` is private, but the second privacy rule states that the `try_me`
function is allowed to access the `outermost` module because `outermost` is in
the current (root) module, as is `try_me`.

The call to `outermost::middle_function` will work because `middle_function` is
public and `try_me` is accessing `middle_function` through its parent module
`outermost`. We determined in the previous paragraph that this module is
accessible.

The call to `outermost::middle_secret_function` will cause a compilation error.
Because `middle_secret_function` is private, the second rule applies. The root
module is neither the current module of `middle_secret_function` (`outermost`
is), nor is it a child module of the current module of `middle_secret_function`.

The module named `inside` is private and has no child modules, so it can be
accessed only by its current module `outermost`. That means the `try_me`
function is not allowed to call `outermost::inside::inner_function` or
`outermost::inside::secret_function`.

#### Fixing the Errors

Here are some suggestions for changing the code in an attempt to fix the
errors. Make a guess as to whether it will fix the errors before you try each
one. Then compile the code to see whether or not you’re right, using the
privacy rules to understand why. Feel free to design more experiments and try
them out!

* What if the `inside` module were public?
* What if `outermost` were public and `inside` were private?
* What if, in the body of `inner_function`, you called
  `::outermost::middle_secret_function()`? (The two colons at the beginning mean
  that we want to refer to the modules starting from the root module.)

Next, let’s talk about bringing items into scope with the `use` keyword.
