
[TOC]

# Error Handling

Rust's focus on reliability extends to the area of error handling. Errors are a
fact of life in software, so Rust has a number of features that you can use to
handle situations in which something bad happens. In many cases, Rust requires
you to acknowledge the possibility of an error occurring and take some action
in that situation. This makes your program more robust by eliminating the
possibility of unexpected errors only being discovered after you've deployed
your code to production.

Rust groups errors into two major kinds: errors that are *recoverable*, and
errors that are *unrecoverable*. Recoverable errors are problems like a file not
being found, where it's usually reasonable to report that problem to the user
and retry the operation. Unrecoverable errors are problems like trying to
access a location beyond the end of an array, and these are always symptoms of
bugs.

Most languages do not distinguish between the two kinds of errors, so they
handle both kinds in the same way using mechanisms like exceptions. Rust
doesn't have exceptions. Instead, it has the value `Result<T, E>` to return in
the case of recoverable errors and the `panic!` macro that stops execution when
it encounters unrecoverable errors. This chapter will cover the more
straightforward case of calling `panic!` first. Then, we'll talk about
returning `Result<T, E>` values and calling functions that return `Result<T,
E>`. Finally, we'll discuss considerations to take into account when deciding
whether to try to recover from an error or to stop execution.

## Unrecoverable Errors with `panic!`

Sometimes, bad things happen, and there's nothing that you can do about it. For
these cases, Rust has a macro, `panic!`. When this macro executes, your program
will print a failure message, unwind and clean up the stack, and then quit. The
most common reason for this is when a bug of some kind has been detected, and
it's not clear how to handle the error.

<!-- PROD: START BOX -->

> #### Unwinding
> By default, when a `panic!` happens in Rust, the program starts
> *unwinding*, which means Rust walks back up the stack and cleans up the data
> from each function it encounters. Doing that walking and cleanup is a lot of
> work. The alternative is to immediately `abort`, which ends the program
> without cleaning up. Memory that the program was using will need to be cleaned
> up by the operating system. If you're in a situation where you need to make
> the resulting binary as small as possible, you can switch from unwinding on
> panic to aborting on panic by adding `panic = 'abort'` to the appropriate
> `[profile]` sections in your `Cargo.toml`.

<!-- PROD: END BOX -->

Let's try out calling `panic!()` with a simple program:

```rust,should_panic
fn main() {
    panic!("crash and burn");
}
```

If you run it, you'll see something like this:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished debug [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

There are three lines of error message here. The first line shows our panic
message and the place in our source code where the panic occurred:
`src/main.rs`, line two.

But that only shows us the exact line that called `panic!`. That's not always
useful. Let's look at another example to see what it's like when a `panic!`
call comes from code we call instead of from our code directly:

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

We're attempting to access the hundredth element of our vector, but it only has
three elements. In this situation, Rust will panic. Using `[]` is supposed to
return an element. If you pass `[]` an invalid index, though, there's no
element that Rust could return here that would be correct.

Other languages like C will attempt to give you exactly what you asked for in
this situation, even though it isn't what you want: you'll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn't belong to the vector. This is called a *buffer
overread*, and can lead to security vulnerabilities if an attacker can
manipulate the index in such a way as to read data they shouldn't be allowed to
that is stored after the array.

In order to protect your program from this sort of vulnerability, if you try to
read an element at an index that doesn't exist, Rust will stop execution and
refuse to continue with an invalid value. Let's try it and see:

```bash
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished debug [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', ../src/libcollections/vec.rs:1265
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

This points at a file we didn't write, `../src/libcollections/vec.rs`. That's
the implementation of `Vec<T>` in the standard library. While it's easy to see
in this short program where the error was, it would be nicer if we could have
Rust tell us what line in our program caused the error.

That's what the next line, the `note` is about. If we set the `RUST_BACKTRACE`
environment variable, we'll get a backtrace of exactly how the error happend.
Let's try that. Listing 9-1 shows the output:

```bash
$ RUST_BACKTRACE=1 cargo run
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
100', ../src/libcollections/vec.rs:1265
stack backtrace:
   1:     0x560956150ae9 -
std::sys::backtrace::tracing::imp::write::h482d45d91246faa2
   2:     0x56095615345c -
std::panicking::default_hook::_{{closure}}::h89158f66286b674e
   3:     0x56095615291e - std::panicking::default_hook::h9e30d428ee3b0c43
   4:     0x560956152f88 -
std::panicking::rust_panic_with_hook::h2224f33fb7bf2f4c
   5:     0x560956152e22 - std::panicking::begin_panic::hcb11a4dc6d779ae5
   6:     0x560956152d50 - std::panicking::begin_panic_fmt::h310416c62f3935b3
   7:     0x560956152cd1 - rust_begin_unwind
   8:     0x560956188a2f - core::panicking::panic_fmt::hc5789f4e80194729
   9:     0x5609561889d3 -
core::panicking::panic_bounds_check::hb2d969c3cc11ed08
  10:     0x56095614c075 - _<collections..vec..Vec<T> as
core..ops..Index<usize>>::index::hb9f10d3dadbe8101
                        at ../src/libcollections/vec.rs:1265
  11:     0x56095614c134 - panic::main::h2d7d3751fb8705e2
                        at /projects/panic/src/main.rs:4
  12:     0x56095615af46 - __rust_maybe_catch_panic
  13:     0x560956152082 - std::rt::lang_start::h352a66f5026f54bd
  14:     0x56095614c1b3 - main
  15:     0x7f75b88ed72f - __libc_start_main
  16:     0x56095614b3c8 - _start
  17:                0x0 - <unknown>
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```

<caption>
Listing 9-1: The backtrace generated by a call to `panic!` displayed when
the environment variable `RUST_BACKTRACE` is set
</caption>

That's a lot of output! Line 11 of the backtrace points to the line in our
project causing the problem: `src/main.rs` line four. The key to reading the
backtrace is to start from the top and read until we see files that we wrote:
that's where the problem originated. If we didn't want our program to panic
here, this line is where we would start investigating in order to figure out
how we got to this location with values that caused the panic.

Now that we've covered how to `panic!` to stop our code's execution and how to
debug a `panic!`, let's look at how to instead return and use recoverable
errors with `Result`.

## Recoverable Errors with `Result`

Most errors aren't so dire. Sometimes, when a function fails, it's for a reason
that we can easily interpret and respond to. As an example, maybe we are making
a request to a website, but it's down for maintenance. In this situation, we'd
like to wait and then try again. Terminating our process isn't the right thing
to do here.

In these cases, Rust's standard library provides an `enum` to use as the return
type of the function:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Ok` variant indicates a successful result, and `Err` indicates an
unsuccessful result. These two variants each contain one thing: in `Ok`'s case,
it's the successful return value. With `Err`, it's some value that represents
the error. The `T` and `E` are generic type parameters; we'll go into generics
in more detail in Chapter XX. What you need to know for right now is that the
`Result` type is defined such that it can have the same behavior for any type
`T` that is what we want to return in the success case, and any type `E` that
is what we want to return in the error case.

Listing 9-2 shows an example of something that might fail: opening a file.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<caption>
Listing 9-2: Opening a file
</caption>

The type of `f` in this example is a `Result`, because there are many ways in
which opening a file can fail. For example, unless we created `hello.txt`, this
file does not yet exist. Before we can do anything with our `File`, we need to
extract it out of the result. Listing 9-3 shows one way to handle the `Result`
with a basic tool: the `match` expression that we learned about in Chapter 6.

<!-- I'll ghost everything except the match statement lines in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}",
error),
    };
}
```

<caption>
Listing 9-3: Using a `match` expression to handle the `Result` variants we might have
</caption>

If we see an `Ok`, we can return the inner `file` out of the `Ok` variant. If
we see `Err`, we have to decide what to do with it. The simplest thing is to
turn our error into a `panic!` instead, by calling the macro. And since we
haven't created that file yet, we'll see a message indicating as such when we
print the error value:

```bash
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

### Matching on Different Errors

There are many reasons why opening a file might fail, and we may not want to
take the same actions to try to recover for all of them. For example, if the
file we're trying to open does not exist, we could choose to create it. If the
file exists but we don't have permission to read it, or any other error, we
still want to `panic!` in the same way as above and not create the file.

The `Err` type `File::open` returns is `io::Error`,
which is a struct provided by the standard library. This struct has a method
`kind` that we can call to get an `io::ErrorKind`
value that we can use to handle different causes of an `Err` returned from
`File::open` differently as in Listing 9-4:

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            }
        },
        Err(error) => panic!("There was a problem opening the file: {:?}",
error),
    };
}
```

<caption>
Listing 9-4: Handling different kinds of errors in different ways
</caption>

<!-- I will add ghosting and wingdings here in libreoffice /Carol -->

This example uses a *match guard* with the second arm's pattern to add a
condition that further refines the pattern. The `ref` in the pattern is needed
so that the `error` is not moved into the guard condition. The condition we
want to check is that the value `error.kind()` returns is the `NotFound`
variant of the `ErrorKind` enum. Note that `File::create` could also fail, so
we need to add an inner `match` statement as well! The last arm of the outer
`match` stays the same to panic on any error besides the file not being found.

### Shortcuts for Panic on Error: `unwrap` and `expect`

Using `match` works okay but can be a bit verbose, and it doesn't always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various things. "Panic on an error result" is one of those
methods, and it's called `unwrap()`:

<!-- I'll ghost everything except `unwrap()` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

This has similar behavior as the example using `match` in Listing 9-3: If the
call to `open()` returns `Ok`, return the value inside. If it's an `Err`, panic.

There's also another method that is similar to `unwrap()`, but lets us choose
the error message: `expect()`. Using `expect()` instead of `unwrap()` and
providing good error messages can convey your intent and make tracking down the
source of a panic easier. `expect()` looks like this:

<!-- I'll ghost everything except `expect()` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt.");
}
```

This isn't the only way to deal with errors, however. This entire section is
supposed to be about recovering from errors, but we've gone back to panic. This
observation gets at an underlying truth: you can easily turn a recoverable
error into an unrecoverable one with `unwrap()` or `expect()`, but you can't
turn an unrecoverable `panic!` into a recoverable one. This is why good Rust
code chooses to make errors recoverable: you give your caller choices.

The Rust community has a love/hate relationship with `unwrap()` and `expect()`.
They're very handy when prototyping, before you're ready to decide how to
handle errors, and in that case they leave clear markers to look for when you
are ready to make your program more robust. They're useful in tests since they
will cause the test to fail if there's an error any place you call them. In
examples, you might not want to muddy the code with proper error handling. But
if you use them in a library, mis-using your library can cause other people's
programs to halt unexpectedly, and that's not very user-friendly.

Another time it's appropriate to call `unwrap` is when we have some other logic
that ensures the `Result` will have an `Ok` value, but the logic isn't
something the compiler understands. If you can ensure by manually inspecting
the code that you'll never have an `Err` variant, it is perfectly acceptable to
call `unwrap`. Here's an example:

```rust
use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();
```

We're creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `"127.0.0.1"` is a valid IP address, so it's acceptable to use `unwrap`
here. If we got the IP address string from a user of our program instead of
hardcoding this value, we'd definitely want to handle the `Result` in a more
robust way instead.

### Propagating errors with `try!` or `?`

When writing a function, if you don't want to handle the error where you are,
you can return the error to the calling function. For example, Listing 9-5
shows a function that reads a username from a file. If the file doesn't exist
or can't be read, this function will return those errors to the code that
called this function:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

<caption>
Listing 9-5: A function that returns errors to the calling code using `match`
</caption>

Since the `Result` type has two type parameters, we need to include them both
in our function signature. In this case, `File::open` and `read_to_string`
return `std::io::Error` as the value inside the `Err` variant, so we will also
use it as our error type. If this function succeeds, we want to return the
username as a `String` inside the `Ok` variant, so that is our success type.

This is a very common way of handling errors: propagate them upward until
you're ready to deal with them. This pattern is so common in Rust that there is
a macro for it, `try!`, and as of Rust 1.14 <!-- 1.14 has not been released as
a stable version yet, but that's the version the question mark operator will be
released in /Carol -->, dedicated syntax for it: the question mark
operator. We could have written the code in Listing 9-5 using the `try!` macro,
as in Listing 9-6, and it would have the same functionality as the `match`
expressions:

<!-- I'll ghost everything except the calls to `try!` in the libreoffice file
/Carol -->

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = try!(File::open("hello.txt"));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));

    Ok(s)
}
```

<caption>
Listing 9-6: A function that returns errors to the calling code using `try!`
</caption>

Or as in Listing 9-7, which uses the question mark operator:

<!-- I'll ghost everything except the question mark operator in the libreoffice
file. Also note the `#![feature(question_mark)]` line won't be needed once this
feature has made it into a stable version of Rust, which will happen well
before the book's publication.

In order to run the code examples that have the `#![feature(question_mark)]`
line, you'll need to install a nightly version of the Rust compiler. Again,
readers of the book won't need to do this since we expect the question mark
feature to stabilize before publication.

/Carol -->

```rust
#![feature(question_mark)]

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<caption>
Listing 9-7: A function that returns errors to the calling code using `?`
</caption>

The `?` operator at the end of the `open` call does the same thing as the
example that uses `match` and the example that uses the `try!` macro: It will
return the value inside an `Ok` to the binding `f`, but will return early out
of the whole function and give any `Err` value we get to our caller. The same
thing applies to the `?` at the end of the `read_to_string` call.

The advantage of using the question mark operator over the `try!` macro is the
question mark operator permits chaining. We could further shorten this code
by instead doing:

```rust
#![feature(question_mark)]

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

Much nicer, right? The `try!` macro and the `?` operator make propagating
errors upwards much more ergonomic. There's one catch though: they can only be
used in functions that return a `Result`, since they expand to the same `match`
expression we saw above that had a potential early return of an `Err` value.
Let's look at what happens if we try to use `try!` in the `main` function,
which you'll recall has a return type of `()`:

```rust,ignore
fn main() {
    let f = try!(File::open("hello.txt"));
}
```

<!-- NOTE: as of 2016-10-12, the error message when calling `?` in a function
that doesn't return a result is confusing. `try!` isn't as bad, so I'm using
that. When https://github.com/rust-lang/rust/issues/35946 is fixed, we can
switch this example to use `?`. /Carol -->

When we compile this, we get the following error message:

```bash
error[E0308]: mismatched types
 -->
  |
3 |     let f = try!(File::open("hello.txt"));
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
  = note:    found type `std::result::Result<_, _>`
```

The mismatched types that this error is pointing out says the `main()` function
has a return type of `()`, but the `try!` macro might return a `Result`. So in
functions that don't return `Result`, when you call other functions that return
`Result`, you'll need to use a `match` or one of the methods on `Result` to
handle it instead of using `try!` or `?`.

Now that we've discussed the details of calling `panic!` or returning `Result`,
let's return to the topic of how to decide which is appropriate in which cases.

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

If code calling `Guess::new` passed in a value that was not between 1 and 100,
that would be a violation of the contract that `Guess::new` is relying on. This
function needs to signal to the calling code that it has a bug somewhere
leading to the contract violation. The conditions in which `Guess::new` might
panic should be discussed in its public-facing API documentation, which we will
cover in Chapter XX.

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
