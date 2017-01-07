# Testing

> Program testing can be a very effective way to show the presence of bugs, but
> it is hopelessly inadequate for showing their absence.
>
> Edsger W. Dijkstra, "The Humble Programmer" (1972)

Rust is a programming language that cares a lot about correctness, but
correctness is a complex topic and isn't easy to prove. Rust places a lot of
weight on its type system to help ensure that our programs do what we intend,
but it cannot help with everything. As such, Rust also includes support for
writing software tests in the language itself.

For example, we can write a function called `add_two` with a signature that has
an integer as a parameter and returns an integer as a result. We can implement
and compile that function, and Rust can do all the type checking and borrow
checking that we've seen it's capable of doing. What Rust *can't* check for us
is that we've implemented this function to return the parameter plus two and
not the parameter plus 10 or the parameter minus 50! That's where tests come
in. We can write tests that, for example, pass `3` to the `add_two` function
and check that we get `5` back. We can run the tests whenever we make changes
to our code to make sure we didn't change any existing behavior from what the
tests specify it should be.

Testing is a skill, and we cannot hope to cover everything about how to write
good tests in one chapter of a book. What we can discuss, however, are the
mechanics of Rust's testing facilities. We'll talk about the annotations and
macros available to you when writing your tests, the default behavior and
options provided for running your tests, and how to organize tests into unit
tests and integration tests.
