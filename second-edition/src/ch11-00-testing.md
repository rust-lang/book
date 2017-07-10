# Writing Automated Tests

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
> Edsger W. Dijkstra, “The Humble Programmer” (1972)

Correctness in our programs means that our code does what we intend for it to
do. Rust is a programming language that cares a lot about correctness, but
correctness is a complex topic and isn’t easy to prove. Rust’s type system
shoulders a huge part of this burden, but the type system cannot catch every
kind of incorrectness. As such, Rust includes support for writing software
tests within the language itself.

As an example, say we write a function called `add_two` that adds two to
whatever number is passed to it. This function’s signature accepts an integer
as a parameter and returns an integer as a result. When we implement and
compile that function, Rust will do all the type checking and borrow checking
that we’ve seen so far to make sure that, for instance, we aren’t passing a
`String` value or an invalid reference to this function. What Rust *can’t*
check is that this function will do precisely what we intend: return the
parameter plus two, rather than, say, the parameter plus 10 or the parameter
minus 50! That’s where tests come in.

We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, we get `5` back. We can run these tests whenever we make
changes to our code to make sure any existing correct behavior has not changed.

Testing is a complex skill, and we cannot hope to cover everything about how to
write good tests in one chapter of a book, so here we’ll just discuss the
mechanics of Rust’s testing facilities. We’ll talk about the annotations and
macros available to you when writing your tests, the default behavior and
options provided for running your tests, and how to organize tests into unit
tests and integration tests.
