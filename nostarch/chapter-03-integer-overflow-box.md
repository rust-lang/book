Please place this text in a box after the "Integer Types" section ends and before the "Floating-Point Types" section begins on page 38.

##### Integer Overflow

Let’s say that you have a variable of type `u8`, which can hold values
between 0 and 255. What happens if you try to change the variable's value to
256? This is called *integer overflow*, and Rust has some interesting rules
around this behavior. When compiling in debug mode, Rust includes checks for
integer overflow that will cause your program to *panic* at runtime if integer
overflow occurs. Panicking is the term Rust uses when a program exits with an
error; we’ll discuss panics more in the "Unrecoverable Errors with `panic!`
section" of Chapter 9 on page XX.

When compiling in release mode with the `--release` flag, Rust does not
include checks for integer overflow that cause panics. Instead, if overflow
occurs, Rust will perform something called *two’s complement wrapping*. In
short, values greater than the maximum value the type can hold "wrap around"
to the minimum of the values the type can hold. In the case of a `u8`, 256
becomes 0, 257 becomes 1, etc. Relying on the wrapping behavior of integer
overflow is considered an error. If you want to wrap explicitly, the standard
library has a type named `Wrapping` that provides this behavior.
