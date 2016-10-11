# Error Handling

Rust's focus on reliability extends to the area of error handling. Errors are a
fact of life in software, so Rust has a number of features that you can use to
handle situations in which something bad happens. In many cases, Rust requires
you to acknowledge the possibility of an error occurring and take some action
in that situation. This makes your program more robust by eliminating the
possibility of unexpected errors only being discovered after you've deployed
your code to production.

Rust groups errors into two major kinds: errors that are *recoverable*, and
errors that are *unrecoverable*. Recoverable errors are problems like a file not
being found, where it's usually reasonable to report that problem to the user
and retry the operation. Unrecoverable errors are problems like trying to
access a location beyond the end of an array, and these are always symptoms of
bugs.

Most languages do not distinguish between the two kinds of errors, so they
handle both kinds in the same way using mechanisms like exceptions. Rust
doesn't have exceptions. Instead, it has the value `Result<T, E>` to return in
the case of recoverable errors and the `panic!` macro that stops execution when
it encounters unrecoverable errors. This chapter will cover the more
straightforward case of calling `panic!` first. Then, we'll talk about
returning `Result<T, E>` values and calling functions that return `Result<T,
E>`. Finally, we'll discuss considerations to take into account when deciding
whether to try to recover from an error or to stop execution.
