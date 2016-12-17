## To `panic!` or Not To `panic!`

So how do you decide when you should call `panic!` and when you should return
`Result`? Returning `Result` is often the good default choice for a function
that might fail, since that gives the caller of your function the most
flexibility.

But that answer is simplistic. There are cases where you might want to call
`panic!` in library code for safety reasons. Let's look at some more nuanced
guidelines.

### Guidelines for Error Handling

It's advisable to have your code`panic!` when it's possible that you could end
up in a *bad state*---in this context, *bad state* is when some assumption,
guarantee, contract, or invariant has been broken, such as when invalid values,
contradictory values, or missing values are passed to your code--plus one or
more of the following:

* The bad state is not something that's *expected* to happen occasionally
* Your code after this point needs to rely on not being in this bad state
* There's not a good way to encode this information in the types you use

If someone calls your code and passes in values that don't make sense, the best
thing might be to `panic!` and alert the person using your library to the bug
in their code so that they can fix it during development. Similarly, `panic!`
is often appropriate if you're calling external code that is out of your
control, and it returns an invalid state that you have no way of fixing.

However, in some cases, even when a bad state is reached, you may still want to
use a 'Result' instead of 'panic'. Some bad states are expected to happen, and
will happen no matter how well you write your code. Examples of this include a
parser being given malformed data, or an HTTP request returning a status that
indicates you have hit a rate limit. In these cases, you should indicate that
failure is an expected possibility by returning a `Result` and propagate these
bad states upwards so that the caller can decide how they would like to handle
the problem. To `panic!` wouldn't be the best way to handle these cases.

When your code performs operations on values, your code should verify the
values are valid first, then proceed confidently with the operations or
`panic!`. This is mostly for safety reasons: attempting to operate on invalid
data can expose your code to vulnerabilities. This is the main reason that the
standard library will `panic!` if you attempt an out-of-bounds array access:
trying to access memory that doesn't belong to the current data structure is a
common security problem. Functions often have *contracts*: their behavior is
only guaranteed if the inputs meet particular requirements. Panicking when the
contract is violated makes sense because a contract violation always indicates
a caller-side bug, and it is not a kind of error you want callers to have to
explicitly handle. In fact, there's no reasonable way for calling code to
recover: the calling *programmers* need to fix the code. Contracts for a
function, especially when a violation will cause a `panic`, should be explained
in the API documentation for the function.

Having lots of error checks in all of your functions would be verbose and
annoying, though. Luckily, you can use Rust's type system (and thus the type
checking the compiler does) to do a lot of the checks for you. If your function
takes a particular type as an argument, you can proceed with your code's logic
knowing that the compiler has already ensured you have a valid value. For
example, if you have a type rather than an `Option`, your program expects to
have *something* rather than *nothing*, and you don't have to have an explicit
check to make sure.

<!-- Can you go into more detail explaining this last sentence? Why is a type
better to use than an Option?-->

Another example is using an unsigned integer type like `u32`, which ensures the
argument value is never negative.

### Creating Custom Types for Validation

We'll take the idea of using Rust's type system to ensure we have a valid value
one step further, and look at creating a custom type for validation. Recall the
guessing game in Chapter 2, where our code asked the user to guess a number
between 1 and 100. We actually never validated that the user's guess was
between those numbers before checking it against our secret number, only that
it was positive. In this case, the consequences were not very dire: our output
of "Too high" or "Too low" would still be correct. It would be a useful
enhancement to guide the user towards valid guesses, though.

One way to do this would be to add a check after we parse the guess:

```rust,ignore
loop {
    // snip

    let guess: u32 = match guess.trim().parse() {
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
program took a value between 1 and 100, and it had many functions with this
requirement, it would be tedious (and potentially impact performance) to have a
check like this in every function.

Instead, we can make a new type and put the validations in the type's
constructor rather than repeating them. That way, it's safe for the functions
to assume the values meet the requirements and confidently use the type.
Listing 9-8 shows one way to define a `Guess` type that will only create an
instance of `Guess` if the `new` function receives a value between 1 and 100:

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

<!-- Can you slow this code explanation down a bit? It would be good to use wingdings here too. -->

If a value outside the range of 1 and 100 was passed in to a program using the
`Guess` type, it would violate the contract that `Guess::new` is relying on.

<!-- I'm not sure if you mean the function that creates the guess type (so
listing 9-8) or the function that uses the guess type, below. You mean the
wider function needs a way to signal that there's a bug leading to contract
violation? -->

This function needs to signal to the calling code that it has a bug somewhere
leading to the contract violation. The conditions in which `Guess::new` might
panic should be discussed in its public-facing API documentation, which we'll
cover in Chapter XX.

<!-- Is the API documentation part of the code or will that chapter cover
guidelines for good documentation? Can you clarify what you mean? -->

Something to note is that the `value` field of the `Guess` struct is private,
so code using this struct may not set that value directly. Callers *must* use
the `Guess::new` constructor function to create an instance of `Guess`, and
they may read the value using the public `value` function, but they may not
access the field directly. This means any instance of `Guess` that doesn't
cause a `panic!` when `new` is called is guaranteed to return numbers between 1
and 100 from its `value` function.

A function that takes as an argument or returns only numbers between 1 and 100
could then declare in its signature that it takes a `Guess` rather than a
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
