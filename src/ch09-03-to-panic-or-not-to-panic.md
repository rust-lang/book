## To `panic!` or Not to `panic!`

So how do you decide when you should call `panic!` and when you should return
`Result`? When code panics, there’s no way to recover. You could call `panic!`
for any error situation, whether there’s a possible way to recover or not, but
then you’re making the decision that a situation is unrecoverable on behalf of
the calling code. When you choose to return a `Result` value, you give the
calling code options. The calling code could choose to attempt to recover in a
way that’s appropriate for its situation, or it could decide that an `Err`
value in this case is unrecoverable, so it can call `panic!` and turn your
recoverable error into an unrecoverable one. Therefore, returning `Result` is a
good default choice when you’re defining a function that might fail.

In situations such as examples, prototype code, and tests, it’s more
appropriate to write code that panics instead of returning a `Result`. Let’s
explore why, then discuss situations in which the compiler can’t tell that
failure is impossible, but you as a human can. The chapter will conclude with
some general guidelines on how to decide whether to panic in library code.

### Examples, Prototype Code, and Tests

When you’re writing an example to illustrate some concept, also including robust
error-handling code can make the example less clear. In
examples, it’s understood that a call to a method like `unwrap` that could
panic is meant as a placeholder for the way you’d want your application to
handle errors, which can differ based on what the rest of your code is doing.

Similarly, the `unwrap` and `expect` methods are very handy when prototyping,
before you’re ready to decide how to handle errors. They leave clear markers in
your code for when you’re ready to make your program more robust.

If a method call fails in a test, you’d want the whole test to fail, even if
that method isn’t the functionality under test. Because `panic!` is how a test
is marked as a failure, calling `unwrap` or `expect` is exactly what should
happen.

### Cases in Which You Have More Information Than the Compiler

It would also be appropriate to call `unwrap` or `expect` when you have some
other logic that ensures the `Result` will have an `Ok` value, but the logic
isn’t something the compiler understands. You’ll still have a `Result` value
that you need to handle: whatever operation you’re calling still has the
possibility of failing in general, even though it’s logically impossible in
your particular situation. If you can ensure by manually inspecting the code
that you’ll never have an `Err` variant, it’s perfectly acceptable to call
`unwrap`, and even better to document the reason you think you’ll never have an
`Err` variant in the `expect` text. Here’s an example:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

We’re creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `127.0.0.1` is a valid IP address, so it’s acceptable to use `expect`
here. However, having a hardcoded, valid string doesn’t change the return type
of the `parse` method: we still get a `Result` value, and the compiler will
still make us handle the `Result` as if the `Err` variant is a possibility
because the compiler isn’t smart enough to see that this string is always a
valid IP address. If the IP address string came from a user rather than being
hardcoded into the program and therefore *did* have a possibility of failure,
we’d definitely want to handle the `Result` in a more robust way instead.
Mentioning the assumption that this IP address is hardcoded will prompt us to
change `expect` to better error handling code if in the future, we need to get
the IP address from some other source instead.

### Guidelines for Error Handling

It’s advisable to have your code panic when it’s possible that your code
could end up in a bad state. In this context, a *bad state* is when some
assumption, guarantee, contract, or invariant has been broken, such as when
invalid values, contradictory values, or missing values are passed to your
code—plus one or more of the following:

* The bad state is something that is unexpected, as opposed to something that
  will likely happen occasionally, like a user entering data in the wrong
  format.
* Your code after this point needs to rely on not being in this bad state,
  rather than checking for the problem at every step.
* There’s not a good way to encode this information in the types you use. We’ll
  work through an example of what we mean in the [“Encoding States and Behavior
  as Types”][encoding]<!-- ignore --> section of Chapter 17.

If someone calls your code and passes in values that don’t make sense, it’s
best to return an error if you can so the user of the library can decide what
they want to do in that case. However, in cases where continuing could be
insecure or harmful, the best choice might be to call `panic!` and alert the
person using your library to the bug in their code so they can fix it during
development. Similarly, `panic!` is often appropriate if you’re calling
external code that is out of your control and it returns an invalid state that
you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a `Result`
than to make a `panic!` call. Examples include a parser being given malformed
data or an HTTP request returning a status that indicates you have hit a rate
limit. In these cases, returning a `Result` indicates that failure is an
expected possibility that the calling code must decide how to handle.

When your code performs an operation that could put a user at risk if it’s
called using invalid values, your code should verify the values are valid first
and panic if the values aren’t valid. This is mostly for safety reasons:
attempting to operate on invalid data can expose your code to vulnerabilities.
This is the main reason the standard library will call `panic!` if you attempt
an out-of-bounds memory access: trying to access memory that doesn’t belong to
the current data structure is a common security problem. Functions often have
*contracts*: their behavior is only guaranteed if the inputs meet particular
requirements. Panicking when the contract is violated makes sense because a
contract violation always indicates a caller-side bug and it’s not a kind of
error you want the calling code to have to explicitly handle. In fact, there’s
no reasonable way for calling code to recover; the calling *programmers* need
to fix the code. Contracts for a function, especially when a violation will
cause a panic, should be explained in the API documentation for the function.

However, having lots of error checks in all of your functions would be verbose
and annoying. Fortunately, you can use Rust’s type system (and thus the type
checking done by the compiler) to do many of the checks for you. If your
function has a particular type as a parameter, you can proceed with your code’s
logic knowing that the compiler has already ensured you have a valid value. For
example, if you have a type rather than an `Option`, your program expects to
have *something* rather than *nothing*. Your code then doesn’t have to handle
two cases for the `Some` and `None` variants: it will only have one case for
definitely having a value. Code trying to pass nothing to your function won’t
even compile, so your function doesn’t have to check for that case at runtime.
Another example is using an unsigned integer type such as `u32`, which ensures
the parameter is never negative.

### Creating Custom Types for Validation

Let’s take the idea of using Rust’s type system to ensure we have a valid value
one step further and look at creating a custom type for validation. Recall the
guessing game in Chapter 2 in which our code asked the user to guess a number
between 1 and 100. We never validated that the user’s guess was between those
numbers before checking it against our secret number; we only validated that
the guess was positive. In this case, the consequences were not very dire: our
output of “Too high” or “Too low” would still be correct. But it would be a
useful enhancement to guide the user toward valid guesses and have different
behavior when a user guesses a number that’s out of range versus when a user
types, for example, letters instead.

One way to do this would be to parse the guess as an `i32` instead of only a
`u32` to allow potentially negative numbers, and then add a check for the
number being in range, like so:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

The `if` expression checks whether our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that `guess` is
between 1 and 100.

However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, having a check like this in every function would be
tedious (and might impact performance).

Instead, we can make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere. That
way, it’s safe for functions to use the new type in their signatures and
confidently use the values they receive. Listing 9-13 shows one way to define a
`Guess` type that will only create an instance of `Guess` if the `new` function
receives a value between 1 and 100.

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file requires the `rand` crate. We do want to include it for reader
experimentation purposes, but don't want to include it for rustdoc testing
purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-13/src/main.rs:here}}
```

<span class="caption">Listing 9-13: A `Guess` type that will only continue with
values between 1 and 100</span>

First, we define a struct named `Guess` that has a field named `value` that
holds an `i32`. This is where the number will be stored.

Then we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `i32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it’s between 1 and 100.
If `value` doesn’t pass this test, we make a `panic!` call, which will alert
the programmer who is writing the calling code that they have a bug they need
to fix, because creating a `Guess` with a `value` outside this range would
violate the contract that `Guess::new` is relying on. The conditions in which
`Guess::new` might panic should be discussed in its public-facing API
documentation; we’ll cover documentation conventions indicating the possibility
of a `panic!` in the API documentation that you create in Chapter 14. If
`value` does pass the test, we create a new `Guess` with its `value` field set
to the `value` parameter and return the `Guess`.

Next, we implement a method named `value` that borrows `self`, doesn’t have any
other parameters, and returns an `i32`. This kind of method is sometimes called
a *getter*, because its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It’s important that the `value` field be private so code
using the `Guess` struct is not allowed to set `value` directly: code outside
the module *must* use the `Guess::new` function to create an instance of
`Guess`, thereby ensuring there’s no way for a `Guess` to have a `value` that
hasn’t been checked by the conditions in the `Guess::new` function.

A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than an
`i32` and wouldn’t need to do any additional checks in its body.

## Summary

Rust’s error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can’t handle and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust’s type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.

Now that you’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, we’ll talk about how generics work and how you
can use them in your code.

[encoding]: ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
