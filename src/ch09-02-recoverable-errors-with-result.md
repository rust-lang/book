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
