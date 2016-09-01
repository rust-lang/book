# Error Handling

Rust's laser-focus on safety spills over into a related area: error handling.
Errors are a fact of life in software, so Rust has a number of features that you
can use to handle situations in which something bad happens. In many cases,
Rust requires you to acknowledge the possibility of an error occurring and take
some action in that situation. This makes your program safer and more robust by
eliminating the possibility of unexpected errors being discovered late in the
development process.

Rust groups errors into two major kinds: errors that are recoverable, and
errors that are not recoverable. What does it mean to "recover" from an
error? In the simplest sense, it relates to the answer of this question:

> If I call a function, and something bad happens, can I do something
> meaningful? Or should execution stop?

The technique that you use depends on the answer to this question. First,
we'll talk about `panic!`, Rust's way of signaling an unrecoverable error.
Then, we'll talk about `Result<T, E>`, the return type for functions that
may return an error you can recover from.
