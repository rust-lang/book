
[TOC]

# Error Handling

Rust's commitment to reliability extends to error handling. Errors are a fact
of life in software, so Rust has a number of features for handling situations
in which something goes wrong. In many cases, Rust will require you to
acknowledge the possibility of an error occurring and take some action before
your code is deployed. This makes your program more robust by ensuring that you
won't only discover errors after you've deployed your code to production.

Rust groups errors into two major categories: *recoverable* and *unrecoverable*
errors. Recoverable errors are situations when it's usually reasonable to
report the problem to the user and retry the operation, like a file not being
found. Unrecoverable errors are always symptoms of bugs, like trying to access
a location beyond the end of an array.

Most languages don't distinguish between the two kinds of errors, and handle
both in the same way using mechanisms like exceptions. Rust doesn't have
exceptions. Instead, it has the value `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when it encounters unrecoverable
errors. This chapter will cover the more straightforward case of calling
`panic!` first. Then, we'll talk about returning `Result<T, E>` values.
Finally, we'll discuss considerations to take into account when deciding
whether to try to recover from an error or to stop execution.

## Unrecoverable Errors with `panic!`

Sometimes, bad things happen, and there's nothing that you can do about it. For
these cases, Rust has the `panic!` macro. When this macro executes, your
program will print a failure message, unwind and clean up the stack, and then
quit. The most common situation this occurs in is when a bug of some kind has
been detected and it's not clear to Rust how to handle the error.

<!-- PROD: START BOX -->

> #### Unwinding
> By default, when a `panic!` occurs, the program starts
> *unwinding*, which means Rust walks back up the stack and cleans up the data
> from each function it encounters, but this walking and cleanup is a lot of
> work. The alternative is to immediately `abort`, which ends the program
> without cleaning up. Memory that the program was using will then need to be
> cleaned up by the operating system. If in your program you need to make
> the resulting binary as small as possible, you can switch from unwinding to
> aborting on panic by adding `panic = 'abort'` to the appropriate `[profile]`
> sections in your `Cargo.toml`.
<!-- Which is the appropriate profile section, will that be obvious? Maybe we
could give a screenshot? -->

<!-- PROD: END BOX -->

Let's try calling `panic!()` with a simple program:

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

There are three lines of errors here. The first line shows our panic
message and the place in our source code where the panic occurred:
`src/main.rs:2` indicates that it's the second like of our *main.rs* file.

<!-- Can you clarify which three lines are the error, the last three? -->

But that only shows us the exact line that called `panic!`. That's not always
useful.
<!-- why isn't that always useful, you mean because it doesn't specify which
part of that line causes the panic? What's the alternative? I'm not entirely
sure what point we're making? -->

### Heading

<!-- I may be wrong, but this reads like a slightly different topic, can you
suggest a heading? Otherwise, perhaps just make it more clear how they connect?
Is this next section all just about tracing the error? It's not quite clear -->

Let's look at another example to see what it's like when a `panic!` call comes
from a bug in our code instead of from calling the macro directly:

```rust,should_panic
fn main() {
    let v = vec![1, 2, 3];

    v[100];
}
```

We're attempting to access the hundredth element of our vector, but it only has
three elements. In this situation, Rust will panic. Using `[]` is supposed to
return an element, but if you pass an invalid index, there's no element that
Rust could return here that would be correct.

Other languages like C will attempt to give you exactly what you asked for in
this situation, even though it isn't what you want: you'll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn't belong to the vector. This is called a *buffer
overread*, and can lead to security vulnerabilities if an attacker can
manipulate the index in such a way as to read data they shouldn't be allowed to
that is stored after the array.

In order to protect your program from this sort of vulnerability, if you try to
read an element at an index that doesn't exist, Rust will stop execution and
refuse to continue. Let's try it and see:

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
the implementation of `Vec<T>` in the standard library.

<!-- I'm not sure what Vec<T> is doing here, can you go over that a bit more?
Also, are there other types of panic errors that will automatically be caught
by Rust other than an index out of bounds type error? Can we give a general
rule on what causes panics? -->

While it's easy to see in this short program where the error was, it would be
helpful if Rust could tell us what line in our program caused the error.

<!-- Hm, earlier we seemed to say that pointing to the line in the program was
not very useful -->

The next `note` line tells us that we can set the `RUST_BACKTRACE` environment
variable to get a backtrace of exactly what happened to cause the error. Let's
try that. Listing 9-1 shows the output:

<figure>

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

<figcaption>

Listing 9-1: The backtrace generated by a call to `panic!` displayed when
the environment variable `RUST_BACKTRACE` is set

</figcaption>
</figure>

That's a lot of output! Line 11 of the backtrace points to the line in our
project causing the problem: `src/main.rs`, line four. The key to reading the
backtrace is to start from the top and read until you see files you wrote:
that's where the problem originated.

<!-- So is the rest of it code embedded in Rust? -->

If we don't want our program to panic, this line is where we would start
investigating in order to figure out how we got to this location with values
that caused the panic.

<!--Are we going to show them how to fix the panic, or is this just something
that's particular to each case?-->

We'll come back to `panic!` and when we should and should not use these methods
later in the chapter. Next, we'll now look at how to recover from an error with
`Result`.

## Recoverable Errors with `Result`

Most errors aren't so dire as to require the program to stop entirely and
reverse what they've done. Sometimes, when a function fails, it's for a reason
that we can easily interpret and respond to. For example, if we are making a
request to a website but it's down for maintenance: in this situation, we'd
want to wait and then try again, rather than terminate the process.

<!-- Can you recap briefly on what Result<T, E> is here, how it's used to
handle an error? -->

In these cases, Rust's standard library provides an `enum` to use as the return
type of the function that might succeed or fail:

<!-- Of what function, the request? I've edited assuming so --- also, can you
lay out what we mean by returning the error. -->

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!-- Would it make sense for this to be something like:

```
enum Result<T, E> {
    Ok(T) => successful_result,
    Err(E) => error,
}
```

instead? Then you could concretely explain the returned result.
-->

The `T` and `E` are generic type parameters; we'll go into generics in more
detail in Chapter XX. What you need to know right now is that the `T`
represents a data type, and 'Result' will treat every value of data type 'T'
the same, in this case by passing the value to 'Ok()'. Similarly, `E` also
represents another data type that will be passed instead to 'Err()'.

<!--- so we're saying that if input/recieved data matches data type T, we get
an ok, and if it matched E, we get an error, is that right? -->

The `Ok` variant indicates a successful result, and `Err` indicates an
unsuccessful result. Each variant contains one object: in `Ok`'s case, it's the
successful return value. With `Err`, it's some value that represents the error.

<!-- What is the successful return value, is this just a placeholder example?
Can you bring this round to the accessing website example above? Otherwise,
would it make sense to use this opening a file example from the start
instead--it may be confusing to mix up examples -->

As an example, Listing 9-2 shows a kind of request that could fail: opening a
file.

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<figcaption>

Listing 9-2: Opening a file

</figcaption>
</figure>

<!--Can you say explicitly why there being many ways things can fail means we
use the result type? Also, are we importing the File type from the standard
crate here? That seems worth mentioning. -->

In this example, we make `f` the type `Result`, because there are many ways in
which opening a file can fail. For example, unless we have already created
`hello.txt`, this file does not yet exist.

<!-- I'm not 100% on what you mean by "Before we can do anything with our
`File`, we need to extract it out of the result", can you explain? You mean we
need to check that it exists? -->

Before we can do anything with our `File`, we need to extract it out of the
result. Listing 9-3 shows one way to handle the `Result` with a basic tool: the
`match` expression that we learned about in Chapter 6.

<!-- I'll ghost everything except the match statement lines in the libreoffice
file /Carol -->

<figure>
<span class="filename">Filename: src/main.rs</span>

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

<!-- So we don't need the Result keyword in this code example? And what is the
{:?} syntax, can you include a line about that? -->

<figcaption>

Listing 9-3: Using a `match` expression to handle the `Result` variants we
might have

</figcaption>
</figure>

Here we tell Rust that when the result is `Ok` it should return the inner
`file` out of the `Ok` variant. We then have to decide what to do to resolve
the error in the case of an `Err` result. The simplest way to handle an error
is to call the `panic!` macro. We know the file doesn't exist yet, so we'll get
an `Err` result, and the program will panic and pass us the message we specify
in the program. If we run the program we'll see a message indicating that the
file doesn't exist when we print the error value:

```bash
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

<!-- Do we have to manually print the error message, or does it show when we
run the program? -->

### Matching on Different Errors

We've handled one type of error, but there are many reasons that opening a file
can fail, and we may not want to take the same recovery actions for all of
them. In this example, if the file we're trying to open doesn't exist, we'll
choose for the `Err` result to instigate the creation of the file. For any
other error, such as if the file exists but we don't have permission to read
it, we'll still want to `panic!` in the same way as our earlier example.

The `Err` type that `File::open` returns is `io::Error`, which is a struct
provided by the standard library. This struct has a method `kind` that we can
call to get an `io::ErrorKind` value, which we can use to handle different
causes of an `Err` returned from `File::open` in different ways, as in Listing
9-4:

<!-- Can you explain what io::ErrorKind is? I got lost with the paragraph
above, are we talking about the program below, or in a more general sense? -->

<figure>
<span class="filename">Filename: src/main.rs</span>

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

<figcaption>

Listing 9-4: Handling different kinds of errors in different ways

</figcaption>
</figure>

<!-- I will add ghosting and wingdings here in libreoffice /Carol -->

<!-- Can you explain what a match guard is with a direct definition here --- is
it just a second match nested within the first? -->

<!-- Hm, how come we use `ref` as the syntax here and not &? -->

The second arms' pattern uses a *match guard* to add a condition that further
refines the pattern. The `ref` in the pattern is needed so that the `error` is
not moved into the guard condition but is merely referenced by it.

<!-- Flagging to remind us to put a wingding number by the conditions.  -->

The condition we want to check in the match guard is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with 'File::create'. However, since `File::create`
could also fail, we need to add an inner `match` statement as well! When the
file can't be opened, a different error message will be printed. The last arm
of the outer `match` stays the same so that the program panics on any error
besides the missing file error.

### Shortcuts for Panic on Error: `unwrap` and `expect`

Using `match` works well enough, but it can be a bit verbose and doesn't always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various things. One of those methods, called `unwrap(), is
what causes the program to "panic on an error result":

<!-- Can you explain a bit more what unwrap() does---you mean every time we
cause a panic it calls the unwrap method? -->

<!-- I'll ghost everything except `unwrap()` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

<!-- Can you talk ore about the syntax here, how it differs? It looks like
there aren't generics here for T and E. How is this still related to Result? -->

This has similar behavior to the example using `match` in Listing 9-3: If the
call to `open()` returns `Ok`, it returns the value inside. If it returns an
`Err`, it panics.

There's another method similar to `unwrap()` that lets us also choose an error
message to show: `expect()`. Using `expect()` instead of `unwrap()` and
providing good error messages can convey your intent and make tracking down the
source of a panic easier. The syntax of`expect()` looks like this:

<!-- I'll ghost everything except `expect()` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt.");
}
```

We use 'expect()' in the same way as 'unwrap()': to return the file to open or
an error. The error it returns will be the string parameter passed to it
instead of the default panic error message.

<!-- I added the above paragraph, can you review it and correct it as
necessary? So this is like what we did in Listing 9-3?-->

Panicing isn't the only way to deal with errors, however. Using the panic macro
means you can easily turn a recoverable error into an unrecoverable one with
`unwrap()` or `expect()`, but you can't turn an unrecoverable `panic!` into a
recoverable one. This is why good Rust code chooses to make errors recoverable:
you give your caller choices.

<!-- Is panic used for both types of errors? The introduction makes it seem as
though it's only for unrecoverable errors -->

### When to use Recoverable and Unrecoverable Error Methods

`unwrap()` and `expect()` are very handy when prototyping, before you're ready
to decide how to handle errors, and in that case they leave clear markers in
your code for when you're ready to make your program more robust. In examples,
you might not want to muddy the code with proper error handling, so these panic
methods are useful for tests. But if you use them in a library, other people's
mis-use of your library can cause their programs to halt unexpectedly, and
that's not very user-friendly.

It would also be appropriate to call `unwrap` when you have some other logic
that ensures the `Result` will have an `Ok` value, but the logic isn't
something the compiler understands. If you can ensure by manually inspecting
the code that you'll never have an `Err` variant, it is perfectly acceptable to
call `unwrap`. Here's an example:

<!-- If we know that there won't be an error, why do we still need to use
unwrap()? Can you clarify that in the text? -->

```rust
use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();
```

We're creating an `IpAddr` instance by parsing a hardcoded string. We can see
that `"127.0.0.1"` is a valid IP address, so it's acceptable to use `unwrap`
here. If the IP address string came from a user instead of being hardcoded into
the program, we'd definitely want to handle the `Result` in a more robust way
instead.

### Propagating Errors

When writing a function, if you don't want to handle the error immediately in
the code, you can return the error to the calling function---this is known as
*propogating* the error. For example, Listing 9-5 shows a function that reads a
username from a file. If the file doesn't exist or can't be read, this function
will return those errors to the code that called this function:

<!-- What's the benefit/result of returning the error to the code that called
the function, besides putting off handling it---can you lay that out? -->

<figure>

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

<figcaption>

Listing 9-5: A function that returns errors to the calling code using `match`

</figcaption>
</figure>

The `Result` type has two type parameters, so we need to include them both in
our function signature. In this case, `File::open` and `read_to_string` return
`std::io::Error` as the value inside the `Err` variant, so we'll also use it as
our error type. If this function succeeds, we want to return the username as a
`String` inside the `Ok` variant, so that is our success type.

<!-- I think we might need to slow down the explanation of the code by walking
through it line by line a bit more, what does it mean to return std::io::Error
as the value? How does this return the error to the code and what does that
mean for the user/programmer? -->

This is a very common way of handling errors: propagate them upward until
you're ready to deal with them. This pattern is so common in Rust that there is
a macro for it: `try!`.

### Delaying Errors with Try

Listing 9-6 uses the `try!` macro to do the exact same as the code in Listing
9-5, and has the same functionality as the `match` expressions:

<!-- I'll ghost everything except the calls to `try!` in the libreoffice file
/Carol -->

<figure>

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = try!(File::open("hello.txt"));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));

    Ok(s)
}
```

<figcaption>

Listing 9-6: A function that returns errors to the calling code using `try!`

</figcaption>
</figure>

<!-- Can you walk through the code for the new parts? -->

### Chaining Errors with ?

This is so common that as of Rust 1.14 <!-- 1.14 has not been released as a
stable version yet, but that's the version the question mark operator will be
released in /Carol -->, there is dedicated syntax for `try!`: the question mark
operator. Listing 9-7 shows the same functionality again with the question mark
operator:

<!-- I'll ghost everything except the question mark operator in the libreoffice
file. Also note the `#![feature(question_mark)]` line won't be needed once this
feature has made it into a stable version of Rust, which will happen well
before the book's publication.

In order to run the code examples that have the `#![feature(question_mark)]`
line, you'll need to install a nightly version of the Rust compiler. Again,
readers of the book won't need to do this since we expect the question mark
feature to stabilize before publication.

/Carol -->

<figure>

```rust
#![feature(question_mark)]

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<figcaption>

Listing 9-7: A function that returns errors to the calling code using `?`

</figcaption>
</figure>

<!-- Below, are we talking about what just the ? operator does, or what the
program with the ? operator does? -->

The `?` operator at the end of the `open` call does the same thing as the
`match` guard and the `try!` macro: in the case of a success, it will return
the value inside an `Ok` to the binding `f`. If an error occurs, it will return
early out of the whole function and give any `Err` value to our caller. The
same thing applies to the `?` at the end of the `read_to_string` call.

The advantage of using the question mark operator over the `try!` macro is the
question mark operator permits chaining. We could further shorten this code
by instead entering:

```rust
#![feature(question_mark)]

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

<!-- Can you explain what is happening in this code and how it differs? -->

Much nicer, right? The `try!` macro and the `?` operator make propagating
errors upwards much more ergonomic.

#### Functions That Don't Return a Result

<!-- I think we need a new heading here, could you suggest something? I'm sure there's a better way to phrase this!-->

There's one catch though: both the `try!` and `?` macros can only be used in
functions that return a `Result`, since they expand to the same `match`
expression we saw above that had a potential early return of an `Err` value.
Let's look at what happens if use `try!` in the `main` function, which you'll
recall has a return type of `()`:

<!-- Which functions return a Result and how would the reader know? I'm also not sure what you mean by "expand", that they have the same functionality (but condensed!)? -->

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

This error is pointing out that we have mismatched types; the `main()` function
has a return type of `()`, but the `try!` macro might return a `Result`. In
functions that don't return `Result`, when you call other functions that do
return `Result`, you'll need to use a `match` or one of the `Result` methods to
handle it, instead of using `try!` or `?`.

Now that we've discussed the details of calling `panic!` or returning `Result`,
let's return to the topic of how to decide which is appropriate to use in which
cases.

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
