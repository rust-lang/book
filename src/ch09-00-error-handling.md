# Error Handling

Errors are a fact of life in software, so Rust has a number of features for
handling situations in which something goes wrong. In many cases, Rust requires
you to acknowledge the possibility of an error and take some action before your
code will compile. This requirement makes your program more robust by ensuring
that you’ll discover errors and handle them appropriately before you’ve
deployed your code to production!

Rust groups errors into two major categories: *recoverable* and *unrecoverable*
errors. For a recoverable error, such as a *file not found* error, we most
likely just want to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array, and so we want to immediately stop the
program.

Most languages don’t distinguish between these two kinds of errors and handle
both in the same way, using mechanisms such as exceptions. Rust doesn’t have
exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when the program encounters an
unrecoverable error. This chapter covers calling `panic!` first and then talks
about returning `Result<T, E>` values. Additionally, we’ll explore
considerations when deciding whether to try to recover from an error or to stop
execution.
