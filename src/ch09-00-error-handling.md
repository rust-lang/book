# Error Handling

Rust's laser-focus on safety spills over into a related area: error handling.
Errors are a fact of life in software. Rust has a number of tools that you can
use to handle things when something bad happens.

Rust splits errors into two major kinds: errors that are recoverable, and
errors that are not recoverable. What does it mean to "recover" from an
error? In the simplest sense, it relates to the answer of this question:

> If I call a function, and something bad happens, can I do something
> meaningful? Or should execution stop?

The technique that you use depends on the answer to this question. First,
we'll talk about `panic!`, Rust's way of signaling an unrecoverable error.
Then, we'll talk about `Result<T, E>`, the return type for functions that
may return an error, but one you can recover from.
