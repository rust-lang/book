## Creating a Library Crate to Separate Responsibilities

Our `main` function now performs two tasks: it parses arguments and opens up
files. For such a small function, this isn’t a huge problem. However, we now
want to implement the part of `minigrep` that will search for a query within
the contents of the file. This would be a third responsibility for `main` to
handle. As a function gains responsibilities, it gets harder to reason about,
harder to test, and harder to change without breaking one of its parts. It’s
better to separate out functionality so that each function is responsible for
one task.

There are crates that help make command line argument parsing more robust and
easy to maintain; at this point we would recommend using a crate and extracting
the command line argument parsing logic from `main`. This is an optional step
for you to explore on your own!

Another problem with adding more functionality to `main` is that `main` is not
easily testable. If we added the searching logic to `main` and wanted to test
how this logic worked on different contents, we would need to have a different
file for each different set of contents we wanted to test. If instead we define
a library function that takes a string slice of contents, `main` can keep its
responsibility of reading the file contents and pass them to the library
function. Our tests can pass string slices to the library function from static
string values that we specify in the test directly. These tests will be easier
to set up and easier to understand since the contents will be inline. The
tests will be calling this function in the same way that `main` calls the
function, so the tests are representative of the actual usage.

Extracting a library crate from a binary crate once the binary crate starts to
have too many responsibilities is a common pattern in Rust. This will create
two separate crates with the same name within one project. The library crate we
are going to create will be responsible for the search logic. For now, let's
define a public `search` function that doesn't take any arguments, return any
values, or contain any code in order to concentrate on setting up the library
crate. To do this, create a *src/lib.rs* file with the contents shown in
Listing 12-5:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn search() {}
```

<span class="caption">Listing 12-5: A library crate containing one public
function without any functionality yet</span>

In order to be able to call this function from `main`, we need to bring the
library crate into the binary crate's scope by adding `extern crate minigrep;`
to the top, and then we can call the function in the body of `main` by using
`minigrep::search()` as shown in Listing 12-6:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate minigrep;
// ...snip...

fn main() {
    // ...snip...
    minigrep::search();
}
```

<span class="caption">Listing 12-6: Calling the library crate's function from
the binary crate</span>

This should compile but have no effect on the way the program works yet. Let's
change that!
