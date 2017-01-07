## To `panic!` or Not To `panic!`

So how do you decide when you should `panic!` and when you should return
`Result`? When code panics, there's no way to recover. You could choose to call
`panic!` for any error situation, whether there's a possible way to recover or
not, but then you're making the decision for your callers that a situation is
unrecoverable. When you choose to return a `Result` value, you give your caller
options, rather than making the decision for them. They could choose to attempt
to recover in a way that's appropriate for their situation, or they could
decide that actually, an `Err` value in this case is unrecoverable, so they can
call `panic!` and turn your recoverable error into an unrecoverable one.
Therefore, returning `Result` is a good default choice when you're defining a
function that might fail.

There are a few situations in which it's more appropriate to write code that
panics instead of returning a `Result`, but they are less common. Let's discuss
why it's appropriate to panic in examples, prototype code, and tests, then
situations where you as a human can know a method won't fail that the compiler
can't reason about, and conclude with some general guidelines on how to decide
whether to panic in library code.

### Examples, Prototype Code, and Tests: Perfectly Fine to Panic

When you're writing an example to illustrate some concept, having robust error
handling code in the example as well can make the example less clear. In
examples, it's understood that a call to a method like `unwrap` that could
`panic!` is meant as a placeholder for the way that you'd actually like your
application to handle errors, which can differ based on what the rest of your
code is doing.

Similarly, the `unwrap` and `expect` methods are very handy when prototyping,
before you're ready to decide how to handle errors. They leave clear markers in
your code for when you're ready to make your program more robust.

If a method call fails in a test, we'd want the whole test to fail, even if that
method isn't the functionality under test. Because `panic!` is how a test gets
marked as a failure, calling `unwrap` or `expect` is exactly what makes sense to
do.

### Cases When You Have More Information Than The Compiler

It would also be appropriate to call `unwrap` when you have some other logic
that ensures the `Result` will have an `Ok` value, but the logic isn't
something the compiler understands. You'll still have a `Result` value that you
need to handle: whatever operation you're calling still has the possibility of
failing in general, even though it's logically impossible in your particular
situation. If you can ensure by manually inspecting the code that you'll never
have an `Err` variant, it is perfectly acceptable to call `unwrap`. Here's an
example:

<!-- If we know that there won't be an error, why do we still need to use
unwrap()? Can you clarify that in the text? -->
<!-- Because you still have to extract the value from the `Ok`; knowing there
won't be an error doesn't change the types. I've tried to clarify in the
paragraph above and again below. /Carol-->

```rust
use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();
```

We're creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `"127.0.0.1"` is a valid IP address, so it's acceptable to use `unwrap`
here. However, having a hardcoded, valid string doesn't change the return type
of the `parse` method: we still get a `Result` value, and the compiler will
still make us handle the `Result` as if the `Err` variant is still a possibility
since the compiler isn't smart enough to see that this string is always a
valid IP address. If the IP address string came from a user instead of being
hardcoded into the program, and therefore *did* have a possibility of failure,
we'd definitely want to handle the `Result` in a more robust way instead.

### Guidelines for Error Handling

It's advisable to have your code `panic!` when it's possible that you could end
up in a *bad state*—in this context, *bad state* is when some assumption,
guarantee, contract, or invariant has been broken, such as when invalid values,
contradictory values, or missing values are passed to your code—plus one or
more of the following:

* The bad state is not something that's *expected* to happen occasionally
* Your code after this point needs to rely on not being in this bad state
* There's not a good way to encode this information in the types you use

If someone calls your code and passes in values that don't make sense, the best
thing might be to `panic!` and alert the person using your library to the bug
in their code so that they can fix it during development. Similarly, `panic!`
is often appropriate if you're calling external code that is out of your
control, and it returns an invalid state that you have no way of fixing.

When a bad state is reached, but it's expected to happen no matter how well you
write your code, it's still more appropriate to return a `Result` rather than
calling `panic!`. Examples of this include a parser being given malformed data,
or an HTTP request returning a status that indicates you have hit a rate limit.
In these cases, you should indicate that failure is an expected possibility by
returning a `Result` in order to propagate these bad states upwards so that the
caller can decide how they would like to handle the problem. To `panic!`
wouldn't be the best way to handle these cases.

When your code performs operations on values, your code should verify the
values are valid first, and `panic!` if the values aren't valid. This is mostly
for safety reasons: attempting to operate on invalid data can expose your code
to vulnerabilities. This is the main reason that the standard library will
`panic!` if you attempt an out-of-bounds array access: trying to access memory
that doesn't belong to the current data structure is a common security problem.
Functions often have *contracts*: their behavior is only guaranteed if the
inputs meet particular requirements. Panicking when the contract is violated
makes sense because a contract violation always indicates a caller-side bug,
and it is not a kind of error you want callers to have to explicitly handle. In
fact, there's no reasonable way for calling code to recover: the calling
*programmers* need to fix the code. Contracts for a function, especially when a
violation will cause a `panic`, should be explained in the API documentation
for the function.

Having lots of error checks in all of your functions would be verbose and
annoying, though. Luckily, you can use Rust's type system (and thus the type
checking the compiler does) to do a lot of the checks for you. If your function
has a particular type as a parameter, you can proceed with your code's logic
knowing that the compiler has already ensured you have a valid value. For
example, if you have a type rather than an `Option`, your program expects to
have *something* rather than *nothing*. Your code then doesn't have to handle
two cases for the `Some` and `None` variants, it will only have one case for
definitely having a value. Code trying to pass nothing to your function won't
even compile, so your function doesn't have to check for that case at runtime.
Another example is using an unsigned integer type like `u32`, which ensures the
parameter is never negative.

<!-- Can you go into more detail explaining this last sentence? Why is a type
better to use than an Option?-->
<!-- I tried to reword, but I'm not sure if I made it any clearer. You don't
have to have extra checks, so your code is simpler; I'm not sure why it's not
clear that simpler is better. /Carol -->

### Creating Custom Types for Validation

Let's take the idea of using Rust's type system to ensure we have a valid value
one step further, and look at creating a custom type for validation. Recall the
guessing game in Chapter 2, where our code asked the user to guess a number
between 1 and 100. We actually never validated that the user's guess was
between those numbers before checking it against our secret number, only that
it was positive. In this case, the consequences were not very dire: our output
of "Too high" or "Too low" would still be correct. It would be a useful
enhancement to guide the user towards valid guesses, though, and have different
behavior when a user guesses a number that's out of range versus when a user
types, for example, letters instead.

One way to do this would be to parse the guess as an `i32` instead of only a
`u32`, to allow potentially negative numbers, then add a check for the number
being in range:

```rust,ignore
loop {
    // snip

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // snip
}
```

<!-- I'll add wingding numbers in the libreoffice file /Carol -->

The `if` expression checks to see if our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that guess is between
1 and 100.

However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, it would be tedious (and potentially impact performance)
to have a check like this in every function.

Instead, we can make a new type and put the validations in the type's
constructor rather than repeating them. That way, it's safe for functions to
use the new type in their signatures and confidently use the values they
receive. Listing 9-8 shows one way to define a `Guess` type that will only
create an instance of `Guess` if the `new` function receives a value between 1
and 100:

<figure>

```rust
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value: value,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```

<figcaption>

Listing 9-8: A `Guess` type that will only continue with values between 1 and
100

</figcaption>
</figure>

<!-- I'll add wingding numbers in the libreoffice file /Carol -->

First, we define a struct named `Guess` that has a field named `value` that
holds a `u32`. This is where the number will be stored.

Then we implement an associated function named `new` on `Guess` that is a
constructor of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `u32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it is between 1 and 100.
If `value` doesn't pass this test, we call `panic!`, which will alert the
programmer who is calling this code that they have a bug they need to fix,
since creating a `Guess` with a `value` outside this range would violate the
contract that `Guess::new` is relying on. The conditions in which `Guess::new`
might panic should be discussed in its public-facing API documentation; we'll
cover documentation conventions around indicating the possibility of a `panic!`
in the API documentation that you create in Chapter 14. If `value` does pass
the test, we create a new `Guess` with its `value` field set to the `value`
parameter, and return the `Guess`.

<!-- I'm not sure if you mean the function that creates the guess type (so
listing 9-8) or the function that uses the guess type, below. You mean the
wider function needs a way to signal that there's a bug leading to contract
violation? -->
<!-- I'm not sure what part is confusing, and I'm not sure what you mean by
"wider function". I hope the slower explanation of the code has cleared
this up; please provide more detail on what's confusing if not. /Carol -->

Next, we implement a method named `value` that borrows `self`, doesn't have any
other parameters, and returns a `u32`. This is a kind of method sometimes called
a *getter*, since its purpose is to get some data from its fields and return
it. This public method is necessary because the `value` field of the `Guess`
struct is private. It's important that the `value` field is private so that
code using the `Guess` struct is not allowed to set `value` directly: callers
*must* use the `Guess::new` constructor function to create an instance of
`Guess`, which ensures there's no way for a `Guess` to have a `value` that
hasn't been checked by the conditions in the constructor.

A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than a
`u32`, and wouldn't need to do any additional checks in its body.

## Summary

Rust's error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can't handle, and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust's type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.

Now that we've seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, let's talk about how generics work and how you
can make use of them in your code.
