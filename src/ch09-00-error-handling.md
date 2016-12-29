# Error Handling

Rust's commitment to reliability extends to error handling. Errors are a fact
of life in software, so Rust has a number of features for handling situations
in which something goes wrong. In many cases, Rust will require you to
acknowledge the possibility of an error occurring and take some action before
your code will compile. This makes your program more robust by ensuring that you
won't only discover errors after you've deployed your code to production.

Rust groups errors into two major categories: *recoverable* and *unrecoverable*
errors. Recoverable errors are situations when it's usually reasonable to
report the problem to the user and retry the operation, like a file not being
found. Unrecoverable errors are always symptoms of bugs, like trying to access
a location beyond the end of an array.

Most languages don't distinguish between the two kinds of errors, and handle
both in the same way using mechanisms like exceptions. Rust doesn't have
exceptions. Instead, it has the value `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when it encounters unrecoverable
errors. This chapter will cover calling `panic!` first, then talk about
returning `Result<T, E>` values. Finally, we'll discuss considerations to take
into account when deciding whether to try to recover from an error or to stop
execution.
