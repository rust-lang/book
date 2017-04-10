## To `panic!` or Not To `panic!`

So how do you decide when you should `panic!` and when you should return
`Result`? As a general rule, we recommend you only use `panic!` for situations
where the error indicates a bug in the program. For instance, library
functions often have *contracts*: their inputs must meet inputs particular
requirements, and if they don't, the calling code has a bug. Panicking when
the contract is violated makes sense because a contract violation always
indicates a bug in the program, and trying to recover may only make matters
worse. This is the main reason that the standard library will `panic!` if you
attempt an out-of-bounds array access: trying to access memory that doesn’t
belong to the current data structure is a common security problem. Contracts
for a function, especially when a violation will cause a panic, should be
explained in the API documentation for the function.

If you design your API to take good advantage of Rust's type system, you will
often find that contract checking is unnecessary, because violations cannot
happen. If your function has a particular type as a parameter, you can proceed
with your code’s logic knowing that the compiler has already ensured you have
a valid value. For example, if you have a type rather than an `Option`, your
program expects to have *something* rather than *nothing*. Your code then
doesn’t have to handle two cases for the `Some` and `None` variants, it will
only have one case for definitely having a value. Code trying to pass nothing
to your function won’t even compile, so your function doesn’t have to check
for that case at runtime.  Another example is using an unsigned integer type
like `u32`, which ensures the parameter is never negative.

On the other hand, when errors happen due to a problem _outside_ the program,
that is almost always something you should handle with a `Result`, even if
there's nothing more that the program can do but print an error message and
exit. We've already seen that converting strings into numbers with `parse`
returns a `Result`, because strings containing invalid numbers are usually
_not_ bugs; they are usually because the _input_ to the program was
incorrect. When you try to access files and get errors from the operating
system, that is also not a bug in your program; at worst it indicates that
there's something wrong with the _computer_ that a human needs to fix (Unix
systems don't work very well if `/etc/passwd` doesn't exist). Network servers
might be inaccessible or malfunctioning for reasons completely out of your
control, or they might be refusing service because the user doesn't have the
proper credentials.

Another thing to keep in mind is that when you use `panic!` you are declaring
that there is nothing a caller could possibly do to recover from the error.
When you choose to return a `Result` value, you give your caller options,
rather than making the decision for them. They could choose to attempt to
recover in a way that’s appropriate for their situation, or they could decide
that actually, an `Err` value in this case is unrecoverable, so they can call
`panic!` and turn your recoverable error into an unrecoverable one.
Therefore, returning `Result` is a good default choice when you’re defining a
function that might fail.

### Reporting Errors to the User

At the highest levels of your program, you will need to intercept error
`Results` and report them to a human. Exactly how to do this will depend on
the environment in which you are running—a command-line tool is different from
a network server or a graphical application. Rust's standard library has all
the facilities needed to report errors in command-line tools, so we will use
that situation for an example.

Recall from the previous chapter, the function that read text from a file.

``` rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

If we call this function from `main` in a command-line tool, what should we do
when it returns an error?  It's not right to use `expect` or `panic!`, but we
can't use the `?` operator either, because `main` doesn't return anything.
Here's one way to handle it:

``` rust,ignore
use std::process;

fn main() {
    match read_username_from_file() {
        Ok(s) => {
            println!("Hello, {}.", s);
        },
        Err(e) => {
            eprintln!("Failed to read from 'hello.txt': {}", e);
            process::exit(1);
        }
    }
}
```

This uses two library features we haven't seen before.  `eprintln!` is like
`println!`, except for one thing: it prints text to the standard _error_
stream, instead of standard output. This prevents error messages from getting
mixed up with the "normal" output of the program—if you've ever tried to print
a document on fancy paper, but what got printed was error messages, you'll
understand why this is important. (The messages printed by `panic!` are
also sent to standard error.)

`process::exit(1)` ends the program with an _unsuccessful_ "exit code"
reported to the command-line environment. (By convention, an exit code of zero
means success, and any nonzero value is some sort of failure.) Rust doesn't
let you do this by returning a value from `main`, the way C does, because the
whole idea of an "exit code" is peculiar to multiprocessing operating systems
that work essentially the same way Unix does. In other environments it isn't
possible to return an "exit code", or it might even be a bug to return from
`main` at all. But if you're in an environment where exit codes exist, then
`process::exit` will be available, and the number passed to it will mean the
same thing it does in C. And falling off the end of `main`, which is what
happens in the `Ok` case, will be the same as calling `process::exit(0)`.

### Examples, Prototype Code, and Tests: Perfectly Fine to Panic

When you’re writing an example to illustrate some concept, having robust error
handling code in the example as well can make the example less clear. In
examples, it’s understood that a call to a method like `unwrap` that could
`panic!` is meant as a placeholder for the way that you’d actually like your
application to handle errors, which can differ based on what the rest of your
code is doing.

Similarly, the `unwrap` and `expect` methods are very handy when prototyping,
before you’re ready to decide how to handle errors. They leave clear markers in
your code for when you’re ready to make your program more robust.

If a method call fails in a test, we’d want the whole test to fail, even if
that method isn’t the functionality under test. Because `panic!` is how a test
gets marked as a failure, calling `unwrap` or `expect` is exactly what makes
sense to do.

### Cases When You Have More Information Than The Compiler

It would also be appropriate to call `unwrap` when you have some other logic
that ensures the `Result` will have an `Ok` value, but the logic isn’t
something the compiler understands. You’ll still have a `Result` value that you
need to handle: whatever operation you’re calling still has the possibility of
failing in general, even though it’s logically impossible in your particular
situation. If you can ensure by manually inspecting the code that you’ll never
have an `Err` variant, it is perfectly acceptable to call `unwrap`. Here’s an
example:

```rust
use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();
```

We’re creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `127.0.0.1` is a valid IP address, so it’s acceptable to use `unwrap`
here. However, having a hardcoded, valid string doesn’t change the return type
of the `parse` method: we still get a `Result` value, and the compiler will
still make us handle the `Result` as if the `Err` variant is still a
possibility since the compiler isn’t smart enough to see that this string is
always a valid IP address. If the IP address string came from a user instead of
being hardcoded into the program, and therefore *did* have a possibility of
failure, we’d definitely want to handle the `Result` in a more robust way
instead.

### Creating Custom Types for Validation

Let’s take the idea of using Rust’s type system to ensure we have a valid value
one step further, and look at creating a custom type for validation. Recall the
guessing game in Chapter 2, where our code asked the user to guess a number
between 1 and 100. We actually never validated that the user’s guess was
between those numbers before checking it against our secret number, only that
it was positive. In this case, the consequences were not very dire: our output
of “Too high” or “Too low” would still be correct. It would be a useful
enhancement to guide the user towards valid guesses, though, and have different
behavior when a user guesses a number that’s out of range versus when a user
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

The `if` expression checks to see if our value is out of range, tells the user
about the problem, and calls `continue` to start the next iteration of the loop
and ask for another guess. After the `if` expression, we can proceed with the
comparisons between `guess` and the secret number knowing that `guess` is
between 1 and 100.

However, this is not an ideal solution: if it was absolutely critical that the
program only operated on values between 1 and 100, and it had many functions
with this requirement, it would be tedious (and potentially impact performance)
to have a check like this in every function.

Instead, we can make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere. That
way, it’s safe for functions to use the new type in their signatures and
confidently use the values they receive. Listing 9-8 shows one way to define a
`Guess` type that will only create an instance of `Guess` if the `new` function
receives a value between 1 and 100:

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

<span class="caption">Listing 9-8: A `Guess` type that will only continue with
values between 1 and 100</span>

First, we define a struct named `Guess` that has a field named `value` that
holds a `u32`. This is where the number will be stored.

Then we implement an associated function named `new` on `Guess` that creates
instances of `Guess` values. The `new` function is defined to have one
parameter named `value` of type `u32` and to return a `Guess`. The code in the
body of the `new` function tests `value` to make sure it is between 1 and 100.
If `value` doesn’t pass this test, we call `panic!`, which will alert the
programmer who is calling this code that they have a bug they need to fix,
since creating a `Guess` with a `value` outside this range would violate the
contract that `Guess::new` is relying on. The conditions in which `Guess::new`
might panic should be discussed in its public-facing API documentation; we’ll
cover documentation conventions around indicating the possibility of a `panic!`
in the API documentation that you create in Chapter 14. If `value` does pass
the test, we create a new `Guess` with its `value` field set to the `value`
parameter and return the `Guess`.

Next, we implement a method named `value` that borrows `self`, doesn’t have any
other parameters, and returns a `u32`. This is a kind of method sometimes
called a *getter*, since its purpose is to get some data from its fields and
return it. This public method is necessary because the `value` field of the
`Guess` struct is private. It’s important that the `value` field is private so
that code using the `Guess` struct is not allowed to set `value` directly:
callers *must* use the `Guess::new` function to create an instance of `Guess`,
which ensures there’s no way for a `Guess` to have a `value` that hasn’t been
checked by the conditions in the `Guess::new` function.

A function that has a parameter or returns only numbers between 1 and 100 could
then declare in its signature that it takes or returns a `Guess` rather than a
`u32`, and wouldn’t need to do any additional checks in its body.

## Summary

Rust’s error handling features are designed to help you write more robust code.
The `panic!` macro signals that your program is in a state it can’t handle, and
lets you tell the process to stop instead of trying to proceed with invalid or
incorrect values. The `Result` enum uses Rust’s type system to indicate that
operations might fail in a way that your code could recover from. You can use
`Result` to tell code that calls your code that it needs to handle potential
success or failure as well. Using `panic!` and `Result` in the appropriate
situations will make your code more reliable in the face of inevitable problems.

Now that we’ve seen useful ways that the standard library uses generics with
the `Option` and `Result` enums, let’s talk about how generics work and how you
can make use of them in your code.
