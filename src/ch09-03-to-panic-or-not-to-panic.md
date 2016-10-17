## To `panic!` or Not To `panic!`

So how do you decide when you should call `panic!` and when you should return
`Result`? A good default for a function that might fail is to return `Result`
since that gives the caller of your function the most flexibility.

But that answer is simplistic. There are cases where you might want to call
`panic!` in library code that have to do with Rust's quest for safety. Let's
look at some more nuanced guidelines.

### Guidelines for Error Handling

`panic!` when your code is in a situation where it's possible to be in a bad
state and:

* The bad state is not something that's *expected* to happen occasionally
* Your code after this point needs to rely on not being in this bad state
* There's not a good way to encode this information in the types you use

By *bad state*, we mean some assumption, guarantee, contract, or invariant has
been broken. Some examples are invalid values, contradictory values, or
nothing when you expect to have something. If someone calls your code and
passes in values that don't make sense, the best thing might be to `panic!` and
alert the person using your library to the bug in their code so that they can
fix it during development. Similarly, `panic!` is often appropriate if you call
someone else's code that is out of your control, and it returns an invalid
state that you have no way of fixing.

Taking each point in turn:

Some bad states are expected to happen sometimes, and will happen no matter how
well you write your code. Examples of this include a parser being given
malformed data to parse, or an HTTP request returning a status that indicates
you have hit a rate limit. In these cases, you should indicate that failure is
an expected possibility by returning a `Result` and propagate these bad states
upwards so that the caller can decide how they would like to handle the
problem. `panic!` would not be the best way to handle these cases.

When your code performs operations on values, your code should verify the
values are valid first, then proceed confidently with the operations. This is
mostly for safety reasons: attempting to operate on invalid data can expose
your code to vulnerabilities. This is the main reason that the standard library
will `panic!` if you attempt an out-of-bounds array access: trying to access
memory that doesn't belong to the current data structure is a common security
problem. Functions often have *contracts*: their behavior is only guaranteed if
the inputs meet particular requirements. Panicking when the contract is
violated makes sense because a contract violation always indicates a
caller-side bug, and it is not a kind of error you want callers to have to
explicitly handle. In fact, there's no reasonable way for calling code to
recover: the calling *programmers* need to fix the code. Contracts for a
function, especially when a violation will cause a `panic`, should be explained
in the API documentation for the function.

Having lots of error checks in all of your functions would be verbose and
annoying, though. Luckily, our last guideline has a tip for this situation: use
Rust's type system (and thus the type checking the compiler does) to do a lot
of the checks for you. If your function takes a particular type as an argument,
you can proceed with your code's logic knowing that the compiler has already
ensured you have a valid value. For example, if you have a type rather than an
`Option`, you know that you will have something rather than nothing and you
don't have to have an explicit check to make sure. Another example is using an
unsigned integer type like `u32`, which ensures the argument value is never
negative.

### Creating Custom Types for Validation

Going a step further with the idea of using Rust's type system to ensure we
have a valid value, let's look at an example of creating a custom type for
validation. Recall the guessing game in Chapter 2, where our code asked the user
to guess a number between 1 and 100. We actually never validated that the
user's guess was between those numbers before checking it against our secret
number, only that it was positive. In this case, the consequences were not very
dire: our output of "Too high" or "Too low" would still be correct. It would be
a nice enhancement to guide the user towards valid guesses, though. We could
add a check after we parse the guess:

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

If we had a situation where it was absolutely critical we had a value between 1
and 100, and we had many functions that had this requirement, it would be
tedious (and potentially impact performance) to have a check like this in every
function. Instead, we can make a new type and put the validations in one place,
in the type's constructor. Then our functions can use the type with the
confidence that we have values that meet our requirements. Listing 9-8 shows
one way to define a `Guess` type that will only create an instance of `Guess`
if the `new` function gets a value between 1 and 100:

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

<caption>
Listing 9-8: A `Guess` type that will only hold values between 1 and 100
</caption>

Important to note is the `value` field of the `Guess` struct is private, so
code using this struct may not set that value directly. Callers *must* use the
`Guess::new` constructor function to create an instance of `Guess`, and they
may read the value using the public `value` function, but they may not access
the field directly. This means any created instance of `Guess` that does not
cause a `panic!` when `new` is called is guaranteed to return numbers between 1
and 100 from its `value` function.

A function that takes as an argument or returns only numbers between 1 and 100
could then declare in its signature to take a `Guess` rather than a `u32`, and
would not need to do any additional checks in its body.

## Summary

Rust's error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can't handle, and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust's type system as a sign that
operations you call might fail in a way that your code could recover from. You
can use `Result` to tell code that calls yours that it needs to handle
potential success or failure as well. Using `panic!` and `Result` in the
appropriate situations will help your code be more reliable in the face of
inevitable problems.

Now that we've seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, let's talk about how generics work and how you
can make use of them in your code.
