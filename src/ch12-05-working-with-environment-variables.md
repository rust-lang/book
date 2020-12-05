## Working with Environment Variables

We’ll improve `minigrep` by adding an extra feature: an option for
case-insensitive searching that the user can turn on via an environment
variable. We could make this feature a command line option and require that
users enter it each time they want it to apply, but instead we’ll use an
environment variable. Doing so allows our users to set the environment variable
once and have all their searches be case insensitive in that terminal session.

### Writing a Failing Test for the Case-Insensitive `search` Function

We want to add a new `search_case_insensitive` function that we’ll call when
the environment variable is on. We’ll continue to follow the TDD process, so
the first step is again to write a failing test. We’ll add a new test for the
new `search_case_insensitive` function and rename our old test from
`one_result` to `case_sensitive` to clarify the differences between the two
tests, as shown in Listing 12-20.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

<span class="caption">Listing 12-20: Adding a new failing test for the
case-insensitive function we’re about to add</span>

Note that we’ve edited the old test’s `contents` too. We’ve added a new line
with the text `"Duct tape."` using a capital D that shouldn’t match the query
`"duct"` when we’re searching in a case-sensitive manner. Changing the old test
in this way helps ensure that we don’t accidentally break the case-sensitive
search functionality that we’ve already implemented. This test should pass now
and should continue to pass as we work on the case-insensitive search.

The new test for the case-*insensitive* search uses `"rUsT"` as its query. In
the `search_case_insensitive` function we’re about to add, the query `"rUsT"`
should match the line containing `"Rust:"` with a capital R and match the line
`"Trust me."` even though both have different casing from the query. This is
our failing test, and it will fail to compile because we haven’t yet defined
the `search_case_insensitive` function. Feel free to add a skeleton
implementation that always returns an empty vector, similar to the way we did
for the `search` function in Listing 12-16 to see the test compile and fail.

### Implementing the `search_case_insensitive` Function

The `search_case_insensitive` function, shown in Listing 12-21, will be almost
the same as the `search` function. The only difference is that we’ll lowercase
the `query` and each `line` so whatever the case of the input arguments,
they’ll be the same case when we check whether the line contains the query.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

<span class="caption">Listing 12-21: Defining the `search_case_insensitive`
function to lowercase the query and the line before comparing them</span>

First, we lowercase the `query` string and store it in a shadowed variable with
the same name. Calling `to_lowercase` on the query is necessary so no matter
whether the user’s query is `"rust"`, `"RUST"`, `"Rust"`, or `"rUsT"`, we’ll
treat the query as if it were `"rust"` and be insensitive to the case. While
`to_lowercase` will handle basic Unicode, it won't be 100% accurate. If we were
writing a real application, we'd want to do a bit more work here, but this section
is about environment variables, not Unicode, so we'll leave it at that here.

Note that `query` is now a `String` rather than a string slice, because calling
`to_lowercase` creates new data rather than referencing existing data. Say the
query is `"rUsT"`, as an example: that string slice doesn’t contain a lowercase
`u` or `t` for us to use, so we have to allocate a new `String` containing
`"rust"`. When we pass `query` as an argument to the `contains` method now, we
need to add an ampersand because the signature of `contains` is defined to take
a string slice.

Next, we add a call to `to_lowercase` on each `line` before we check whether it
contains `query` to lowercase all characters. Now that we’ve converted `line`
and `query` to lowercase, we’ll find matches no matter what the case of the
query is.

Let’s see if this implementation passes the tests:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Great! They passed. Now, let’s call the new `search_case_insensitive` function
from the `run` function. First, we’ll add a configuration option to the
`Config` struct to switch between case-sensitive and case-insensitive search.
Adding this field will cause compiler errors because we aren’t initializing
this field anywhere yet:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

Note that we added the `case_sensitive` field that holds a Boolean. Next, we
need the `run` function to check the `case_sensitive` field’s value and use
that to decide whether to call the `search` function or the
`search_case_insensitive` function, as shown in Listing 12-22. Note this still
won’t compile yet.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

<span class="caption">Listing 12-22: Calling either `search` or
`search_case_insensitive` based on the value in `config.case_sensitive`</span>

Finally, we need to check for the environment variable. The functions for
working with environment variables are in the `env` module in the standard
library, so we want to bring that module into scope with a `use std::env;` line
at the top of *src/lib.rs*. Then we’ll use the `var` function from the `env`
module to check for an environment variable named `CASE_INSENSITIVE`, as shown
in Listing 12-23.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

<span class="caption">Listing 12-23: Checking for an environment variable named
`CASE_INSENSITIVE`</span>

Here, we create a new variable `case_sensitive`. To set its value, we call the
`env::var` function and pass it the name of the `CASE_INSENSITIVE` environment
variable. The `env::var` function returns a `Result` that will be the successful
`Ok` variant that contains the value of the environment variable if the
environment variable is set. It will return the `Err` variant if the
environment variable is not set.

We’re using the `is_err` method on the `Result` to check whether it’s an error
and therefore unset, which means it *should* do a case-sensitive search. If the
`CASE_INSENSITIVE` environment variable is set to anything, `is_err` will
return false and the program will perform a case-insensitive search. We don’t
care about the *value* of the environment variable, just whether it’s set or
unset, so we’re checking `is_err` rather than using `unwrap`, `expect`, or any
of the other methods we’ve seen on `Result`.

We pass the value in the `case_sensitive` variable to the `Config` instance so
the `run` function can read that value and decide whether to call `search` or
`search_case_insensitive`, as we implemented in Listing 12-22.

Let’s give it a try! First, we’ll run our program without the environment
variable set and with the query `to`, which should match any line that contains
the word “to” in all lowercase:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Looks like that still works! Now, let’s run the program with `CASE_INSENSITIVE`
set to `1` but with the same query `to`.

If you're using PowerShell, you will need to set the environment
variable and run the program as separate commands:

```console
PS> $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
```

This will make `CASE_INSENSITIVE` persist for the remainder of your shell
session. It can be unset with the `Remove-Item` cmdlet:

```console
PS> Remove-Item Env:CASE_INSENSITIVE
```

We should get lines that contain “to” that might have uppercase letters:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
CASE_INSENSITIVE=1 cargo run to poem.txt
can't extract because of the environment variable
-->

```console
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent, we also got lines containing “To”! Our `minigrep` program can now do
case-insensitive searching controlled by an environment variable. Now you know
how to manage options set using either command line arguments or environment
variables.

Some programs allow arguments *and* environment variables for the same
configuration. In those cases, the programs decide that one or the other takes
precedence. For another exercise on your own, try controlling case
insensitivity through either a command line argument or an environment
variable. Decide whether the command line argument or the environment variable
should take precedence if the program is run with one set to case sensitive and
one set to case insensitive.

The `std::env` module contains many more useful features for dealing with
environment variables: check out its documentation to see what is available.
