## Recoverable errors with `Result<T, E>`

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

The `Err` type `File::open` returns is [`io::Error`][ioerror]<!-- ignore -->,
which is a struct provided by the standard library. This struct has a method
`kind` that we can call to get an [`io::ErrorKind`][iokind]<!-- ignore -->
value that we can use to handle different causes of an `Err` returned from
`File::open` differently as in Listing 9-4:

[ioerror]: ../std/io/struct.Error.html
[iokind]: ../std/io/enum.ErrorKind.html

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
will cause the test to fail if there's an error anyplace you call them. In
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
# use std::fs::File;
# use std::io;
# use std::io::Read;
#
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
a macro for it, `try!`, and as of Rust 1.XX <!-- We will fill this in once the
question mark is released in a stable version; we don't know for sure which
version it will be yet /Carol -->, dedicated syntax for it: the question mark
operator. We could have written the code in Listing 9-5 using the `try!` macro,
as in Listing 9-6, and it would have the same functionality as the `match`
expressions:

<!-- I'll ghost everything except the calls to `try!` in the libreoffice file
/Carol -->

```rust
# use std::fs::File;
# use std::io;
# use std::io::Read;
#
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
# fn main() {}
# use std::fs::File;
# use std::io;
# use std::io::Read;
#
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
# fn main() {}
# use std::fs::File;
# use std::io;
# use std::io::Read;
#
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
# use std::fs::File;
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
