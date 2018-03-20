# Writing Automated Tests

In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that
“Program testing can be a very effective way to show the presence of bugs, but
it is hopelessly inadequate for showing their absence.” That doesn’t mean we
shouldn’t try to test as much as we can!

Correctness in our programs is the extent to which our code does what we intend
it to do. Rust is designed with a high degree of concern about the correctness
of programs, but correctness is complex and not easy to prove. Rust’s type
system shoulders a huge part of this burden, but the type system cannot catch
every kind of incorrectness. As such, Rust includes support for writing
automated software tests within the language.

As an example, say we write a function called `add_two` that adds 2 to whatever
number is passed to it. This function’s signature accepts an integer as a
parameter and returns an integer as a result. When we implement and compile
that function, Rust does all the type checking and borrow checking that you’ve
learned so far to ensure that, for instance, we aren’t passing a `String` value
or an invalid reference to this function. But Rust *can’t* check that this
function will do precisely what we intend, which is return the parameter plus 2
rather than, say, the parameter plus 10 or the parameter minus 50! That’s where
tests come in.

We can write tests that assert, for example, that when we pass `3` to the
`add_two` function, the returned value is `5`. We can run these tests whenever
we make changes to our code to make sure any existing correct behavior has not
changed.

Testing is a complex skill: although we can’t cover every detail about how to
write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing
facilities. We’ll talk about the annotations and macros available to you when
writing your tests, the default behavior and options provided for running your
tests, and how to organize tests into unit tests and integration tests.
