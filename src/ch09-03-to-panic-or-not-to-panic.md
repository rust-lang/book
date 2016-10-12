## To `panic!` or Not To `panic!`

So how do you decide when you should call `panic!` and when you should return
`Result`? The most concise answer is to `panic!` in `main`, at your outermost
layer, and return `Result` everywhere else in your code. Especially when you're
writing a library for others to use, it's best to not `panic!` if at all
possible. That way, people who use your crate get to decide how they want to
handle failures from your code, instead of you deciding for them.

But that answer is simplistic. There are cases where you might want to call
`panic!` in library code that have to do with Rust's quest for safety. Let's
look at some more nuanced guidelines.

### Guidelines for Error Handling

`panic!` when your code is in a situation where it's possible to be in a bad
state and:

* The cause of the bad state is not your code, it's caused by code that's
  calling your code or code that your code is calling that's out of your control
* The bad state is not something that's *expected* to happen occasionally
* Your code after this point needs to rely on not being in this bad state
* There's not a good way to encode this information in the types you use

Taking these in turn:

A bad state consists of things like invalid values, contradictory values, or
nothing when you expect to have something. If someone calls your code and
passes in values that don't make sense, the best thing might be to `panic!` and
alert the person using your library to the bug in their code so that they can
fix it during development. Similarly, `panic!` is often appropriate if you call
someone else's code that is out of your control, and it returns an invalid
state that you have no way of fixing. Getting null pointers back from calling
functions in C is an example of this situation. If the only place the bug could
possibly come from is your own code, however, you should fix your bug!

Some bad states are expected to happen sometimes, and will happen no matter how
well you write your code. Example of this include a parser being given
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
problem. A less important reason is that it makes your code more organized to
have the error checking first and the rest of the logic afterwards, rather than
interspersing error checks in with your logic.

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
have a valid value, let's look at an example of creating a custom type in this
situation. Recall the guessing game in Chapter 2, where our code asked the user
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
confidence that we have values that meet our requirements. Here's an example of
one way to define a `Guess` type that will only create an instance of `Guess`
if the `new` function gets a value between 1 and 100:

```rust
struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value: value,
        }
    }
}
```

A function that takes as an argument or returns only numbers between 1 and 100
could then declare in its signature to take a `Guess` rather than a `u32`, and
would not need to do any additional checks in its body.

One last guideline: since users of our library will expect `panic!` to be rare,
if we decide a library function fits these guidelines and should call `panic!`,
it's a good idea to document which functions `panic!` and in what conditions.
That way, users of our library can make sure their code uses our library
properly, or at least there is an explanation for any `panic!` they get from
our code.

## Summary

Now that we've reduced duplication in our validation code, let's look at a feature of Rust that helps reduce duplication in lots of code: generics!
