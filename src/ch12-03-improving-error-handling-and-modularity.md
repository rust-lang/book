## Refactoring to Improve Modularity and Error Handling

To improve our program, we’ll fix four problems that have to do with the
program’s structure and how it’s handling potential errors.

First, our `main` function now performs two tasks: it parses arguments and
reads files. For such a small function, this isn’t a major problem. However, if
we continue to grow our program inside `main`, the number of separate tasks the
`main` function handles will increase. As a function gains responsibilities, it
becomes more difficult to reason about, harder to test, and harder to change
without breaking one of its parts. It’s best to separate functionality so each
function is responsible for one task.

This issue also ties into the second problem: although `query` and `filename`
are configuration variables to our program, variables like `contents` are used
to perform the program’s logic. The longer `main` becomes, the more variables
we’ll need to bring into scope; the more variables we have in scope, the harder
it will be to keep track of the purpose of each. It’s best to group the
configuration variables into one structure to make their purpose clear.

The third problem is that we’ve used `expect` to print an error message when
reading the file fails, but the error message just prints `Something went wrong
reading the file`. Reading a file can fail in a number of ways: for example,
the file could be missing, or we might not have permission to open it. Right
now, regardless of the situation, we’d print the `Something went wrong reading
the file` error message, which wouldn’t give the user any information!

Fourth, we use `expect` repeatedly to handle different errors, and if the user
runs our program without specifying enough arguments, they’ll get an `index out
of bounds` error from Rust that doesn’t clearly explain the problem. It would
be best if all the error-handling code were in one place so future maintainers
had only one place to consult in the code if the error-handling logic needed to
change. Having all the error-handling code in one place will also ensure that
we’re printing messages that will be meaningful to our end users.

Let’s address these four problems by refactoring our project.

### Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks to
the `main` function is common to many binary projects. As a result, the Rust
community has developed a process to use as a guideline for splitting the
separate concerns of a binary program when `main` starts getting large. The
process has the following steps:

* Split your program into a *main.rs* and a *lib.rs* and move your program’s
  logic to *lib.rs*.
* As long as your command line parsing logic is small, it can remain in
  *main.rs*.
* When the command line parsing logic starts getting complicated, extract it
  from *main.rs* and move it to *lib.rs*.

The responsibilities that remain in the `main` function after this process
should be limited to the following:

* Calling the command line parsing logic with the argument values
* Setting up any other configuration
* Calling a `run` function in *lib.rs*
* Handling the error if `run` returns an error

This pattern is about separating concerns: *main.rs* handles running the
program, and *lib.rs* handles all the logic of the task at hand. Because you
can’t test the `main` function directly, this structure lets you test all of
your program’s logic by moving it into functions in *lib.rs*. The only code
that remains in *main.rs* will be small enough to verify its correctness by
reading it. Let’s rework our program by following this process.

#### Extracting the Argument Parser

We’ll extract the functionality for parsing arguments into a function that
`main` will call to prepare for moving the command line parsing logic to
*src/lib.rs*. Listing 12-5 shows the new start of `main` that calls a new
function `parse_config`, which we’ll define in *src/main.rs* for the moment.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

<span class="caption">Listing 12-5: Extracting a `parse_config` function from
`main`</span>

We’re still collecting the command line arguments into a vector, but instead of
assigning the argument value at index 1 to the variable `query` and the
argument value at index 2 to the variable `filename` within the `main`
function, we pass the whole vector to the `parse_config` function. The
`parse_config` function then holds the logic that determines which argument
goes in which variable and passes the values back to `main`. We still create
the `query` and `filename` variables in `main`, but `main` no longer has the
responsibility of determining how the command line arguments and variables
correspond.

This rework may seem like overkill for our small program, but we’re refactoring
in small, incremental steps. After making this change, run the program again to
verify that the argument parsing still works. It’s good to check your progress
often, to help identify the cause of problems when they occur.

#### Grouping Configuration Values

We can take another small step to improve the `parse_config` function further.
At the moment, we’re returning a tuple, but then we immediately break that
tuple into individual parts again. This is a sign that perhaps we don’t have
the right abstraction yet.

Another indicator that shows there’s room for improvement is the `config` part
of `parse_config`, which implies that the two values we return are related and
are both part of one configuration value. We’re not currently conveying this
meaning in the structure of the data other than by grouping the two values into
a tuple; we could put the two values into one struct and give each of the
struct fields a meaningful name. Doing so will make it easier for future
maintainers of this code to understand how the different values relate to each
other and what their purpose is.

Listing 12-6 shows the improvements to the `parse_config` function.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

<span class="caption">Listing 12-6: Refactoring `parse_config` to return an
instance of a `Config` struct</span>

We’ve added a struct named `Config` defined to have fields named `query` and
`filename`. The signature of `parse_config` now indicates that it returns a
`Config` value. In the body of `parse_config`, where we used to return string
slices that reference `String` values in `args`, we now define `Config` to
contain owned `String` values. The `args` variable in `main` is the owner of
the argument values and is only letting the `parse_config` function borrow
them, which means we’d violate Rust’s borrowing rules if `Config` tried to take
ownership of the values in `args`.

We could manage the `String` data in a number of different ways, but the
easiest, though somewhat inefficient, route is to call the `clone` method on
the values. This will make a full copy of the data for the `Config` instance to
own, which takes more time and memory than storing a reference to the string
data. However, cloning the data also makes our code very straightforward
because we don’t have to manage the lifetimes of the references; in this
circumstance, giving up a little performance to gain simplicity is a worthwhile
trade-off.

> ### The Trade-Offs of Using `clone`
>
> There’s a tendency among many Rustaceans to avoid using `clone` to fix
> ownership problems because of its runtime cost. In
> [Chapter 13][ch13]<!-- ignore -->, you’ll learn how to use more efficient
> methods in this type of situation. But for now, it’s okay to copy a few
> strings to continue making progress because you’ll make these copies only
> once and your filename and query string are very small. It’s better to have
> a working program that’s a bit inefficient than to try to hyperoptimize code
> on your first pass. As you become more experienced with Rust, it’ll be
> easier to start with the most efficient solution, but for now, it’s
> perfectly acceptable to call `clone`.

We’ve updated `main` so it places the instance of `Config` returned by
`parse_config` into a variable named `config`, and we updated the code that
previously used the separate `query` and `filename` variables so it now uses
the fields on the `Config` struct instead.

Now our code more clearly conveys that `query` and `filename` are related and
that their purpose is to configure how the program will work. Any code that
uses these values knows to find them in the `config` instance in the fields
named for their purpose.

#### Creating a Constructor for `Config`

So far, we’ve extracted the logic responsible for parsing the command line
arguments from `main` and placed it in the `parse_config` function. Doing so
helped us to see that the `query` and `filename` values were related and that
relationship should be conveyed in our code. We then added a `Config` struct to
name the related purpose of `query` and `filename` and to be able to return the
values’ names as struct field names from the `parse_config` function.

So now that the purpose of the `parse_config` function is to create a `Config`
instance, we can change `parse_config` from a plain function to a function
named `new` that is associated with the `Config` struct. Making this change
will make the code more idiomatic. We can create instances of types in the
standard library, such as `String`, by calling `String::new`. Similarly, by
changing `parse_config` into a `new` function associated with `Config`, we’ll
be able to create instances of `Config` by calling `Config::new`. Listing 12-7
shows the changes we need to make.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

<span class="caption">Listing 12-7: Changing `parse_config` into
`Config::new`</span>

We’ve updated `main` where we were calling `parse_config` to instead call
`Config::new`. We’ve changed the name of `parse_config` to `new` and moved it
within an `impl` block, which associates the `new` function with `Config`. Try
compiling this code again to make sure it works.

### Fixing the Error Handling

Now we’ll work on fixing our error handling. Recall that attempting to access
the values in the `args` vector at index 1 or index 2 will cause the program to
panic if the vector contains fewer than three items. Try running the program
without any arguments; it will look like this:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

The line `index out of bounds: the len is 1 but the index is 1` is an error
message intended for programmers. It won’t help our end users understand what
happened and what they should do instead. Let’s fix that now.

#### Improving the Error Message

In Listing 12-8, we add a check in the `new` function that will verify that the
slice is long enough before accessing index 1 and 2. If the slice isn’t long
enough, the program panics and displays a better error message than the `index
out of bounds` message.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

<span class="caption">Listing 12-8: Adding a check for the number of
arguments</span>

This code is similar to [the `Guess::new` function we wrote in Listing
9-13][ch9-custom-types]<!-- ignore -->, where we called `panic!` when the
`value` argument was out of the range of valid values. Instead of checking for
a range of values here, we’re checking that the length of `args` is at least 3
and the rest of the function can operate under the assumption that this
condition has been met. If `args` has fewer than three items, this condition
will be true, and we call the `panic!` macro to end the program immediately.

With these extra few lines of code in `new`, let’s run the program without any
arguments again to see what the error looks like now:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

This output is better: we now have a reasonable error message. However, we also
have extraneous information we don’t want to give to our users. Perhaps using
the technique we used in Listing 9-13 isn’t the best to use here: a call to
`panic!` is more appropriate for a programming problem than a usage problem,
[as discussed in Chapter 9][ch9-error-guidelines]<!-- ignore -->. Instead, we
can use the other technique you learned about in Chapter 9—[returning a
`Result`][ch9-result]<!-- ignore --> that indicates either success or an error.

#### Returning a `Result` from `new` Instead of Calling `panic!`

We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. When
`Config::new` is communicating to `main`, we can use the `Result` type to
signal there was a problem. Then we can change `main` to convert an `Err`
variant into a more practical error for our users without the surrounding text
about `thread 'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.

Listing 12-9 shows the changes we need to make to the return value of
`Config::new` and the body of the function needed to return a `Result`. Note
that this won’t compile until we update `main` as well, which we’ll do in the
next listing.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

<span class="caption">Listing 12-9: Returning a `Result` from
`Config::new`</span>

Our `new` function now returns a `Result` with a `Config` instance in the
success case and a `&'static str` in the error case. Our error values will
always be string literals that have the `'static` lifetime.

We’ve made two changes in the body of the `new` function: instead of calling
`panic!` when the user doesn’t pass enough arguments, we now return an `Err`
value, and we’ve wrapped the `Config` return value in an `Ok`. These changes
make the function conform to its new type signature.

Returning an `Err` value from `Config::new` allows the `main` function to
handle the `Result` value returned from the `new` function and exit the process
more cleanly in the error case.

#### Calling `Config::new` and Handling Errors

To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::new`, as shown in
Listing 12-10. We’ll also take the responsibility of exiting the command line
tool with a nonzero error code from `panic!` and implement it by hand. A
nonzero exit status is a convention to signal to the process that called our
program that the program exited with an error state.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

<span class="caption">Listing 12-10: Exiting with an error code if creating a
new `Config` fails</span>

In this listing, we’ve used a method we haven’t covered in detail yet:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method’s behavior is similar
to `unwrap`: it returns the inner value `Ok` is wrapping. However, if the value
is an `Err` value, this method calls the code in the *closure*, which is an
anonymous function we define and pass as an argument to `unwrap_or_else`. We’ll
cover closures in more detail in [Chapter 13][ch13]<!-- ignore -->. For now,
you just need to know that `unwrap_or_else` will pass the inner value of the
`Err`, which in this case is the static string `"not enough arguments"` that we
added in Listing 12-9, to our closure in the argument `err` that appears
between the vertical pipes. The code in the closure can then use the `err`
value when it runs.

We’ve added a new `use` line to bring `process` from the standard library into
scope. The code in the closure that will be run in the error case is only two
lines: we print the `err` value and then call `process::exit`. The
`process::exit` function will stop the program immediately and return the
number that was passed as the exit status code. This is similar to the
`panic!`-based handling we used in Listing 12-8, but we no longer get all the
extra output. Let’s try it:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Great! This output is much friendlier for our users.

### Extracting Logic from `main`

Now that we’ve finished refactoring the configuration parsing, let’s turn to
the program’s logic. As we stated in [“Separation of Concerns for Binary
Projects”](#separation-of-concerns-for-binary-projects)<!-- ignore -->, we’ll
extract a function named `run` that will hold all the logic currently in the
`main` function that isn’t involved with setting up configuration or handling
errors. When we’re done, `main` will be concise and easy to verify by
inspection, and we’ll be able to write tests for all the other logic.

Listing 12-11 shows the extracted `run` function. For now, we’re just making
the small, incremental improvement of extracting the function. We’re still
defining the function in *src/main.rs*.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

<span class="caption">Listing 12-11: Extracting a `run` function containing the
rest of the program logic</span>

The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.

#### Returning Errors from the `run` Function

With the remaining program logic separated into the `run` function, we can
improve the error handling, as we did with `Config::new` in Listing 12-9.
Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate into `main` the logic around handling errors in a
user-friendly way. Listing 12-12 shows the changes we need to make to the
signature and body of `run`.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

<span class="caption">Listing 12-12: Changing the `run` function to return
`Result`</span>

We’ve made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<dyn Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.

For the error type, we used the *trait object* `Box<dyn Error>` (and we’ve
brought `std::error::Error` into scope with a `use` statement at the top).
We’ll cover trait objects in [Chapter 17][ch17]<!-- ignore -->. For now, just
know that `Box<dyn Error>` means the function will return a type that
implements the `Error` trait, but we don’t have to specify what particular type
the return value will be. This gives us flexibility to return error values that
may be of different types in different error cases. The `dyn` keyword is short
for “dynamic.”

Second, we’ve removed the call to `expect` in favor of the `?` operator, as we
talked about in [Chapter 9][ch9-question-mark]<!-- ignore -->. Rather than
`panic!` on an error, `?` will return the error value from the current function
for the caller to handle.

Third, the `run` function now returns an `Ok` value in the success case. We’ve
declared the `run` function’s success type as `()` in the signature, which
means we need to wrap the unit type value in the `Ok` value. This `Ok(())`
syntax might look a bit strange at first, but using `()` like this is the
idiomatic way to indicate that we’re calling `run` for its side effects only;
it doesn’t return a value we need.

When you run this code, it will compile but will display a warning:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust tells us that our code ignored the `Result` value and the `Result` value
might indicate that an error occurred. But we’re not checking to see whether or
not there was an error, and the compiler reminds us that we probably meant to
have some error-handling code here! Let’s rectify that problem now.

#### Handling Errors Returned from `run` in `main`

We’ll check for errors and handle them using a technique similar to one we used
with `Config::new` in Listing 12-10, but with a slight difference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

We use `if let` rather than `unwrap_or_else` to check whether `run` returns an
`Err` value and call `process::exit(1)` if it does. The `run` function doesn’t
return a value that we want to `unwrap` in the same way that `Config::new`
returns the `Config` instance. Because `run` returns `()` in the success case,
we only care about detecting an error, so we don’t need `unwrap_or_else` to
return the unwrapped value because it would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases: we print the error and exit.

### Splitting Code into a Library Crate

Our `minigrep` project is looking good so far! Now we’ll split the
*src/main.rs* file and put some code into the *src/lib.rs* file so we can test
it and have a *src/main.rs* file with fewer responsibilities.

Let’s move all the code that isn’t the `main` function from *src/main.rs* to
*src/lib.rs*:

* The `run` function definition
* The relevant `use` statements
* The definition of `Config`
* The `Config::new` function definition

The contents of *src/lib.rs* should have the signatures shown in Listing 12-13
(we’ve omitted the bodies of the functions for brevity). Note that this won’t
compile until we modify *src/main.rs* in Listing 12-14.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

<span class="caption">Listing 12-13: Moving `Config` and `run` into
*src/lib.rs*</span>

We’ve made liberal use of the `pub` keyword: on `Config`, on its fields and its
`new` method, and on the `run` function. We now have a library crate that has a
public API that we can test!

Now we need to bring the code we moved to *src/lib.rs* into the scope of the
binary crate in *src/main.rs*, as shown in Listing 12-14.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

<span class="caption">Listing 12-14: Using the `minigrep` library crate in
*src/main.rs*</span>

We add a `use minigrep::Config` line to bring the `Config` type from the
library crate into the binary crate’s scope, and we prefix the `run` function
with our crate name. Now all the functionality should be connected and should
work. Run the program with `cargo run` and make sure everything works
correctly.

Whew! That was a lot of work, but we’ve set ourselves up for success in the
future. Now it’s much easier to handle errors, and we’ve made the code more
modular. Almost all of our work will be done in *src/lib.rs* from here on out.

Let’s take advantage of this newfound modularity by doing something that would
have been difficult with the old code but is easy with the new code: we’ll
write some tests!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch17]: ch17-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
