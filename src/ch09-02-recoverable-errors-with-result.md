## Recoverable Errors with `Result`

Most errors aren't so dire as to require the program to stop entirely.
Sometimes, when a function fails, it's for a reason that we can easily
interpret and respond to. For example, if we try to open a file and that
operation fails because the file doesn't exist, we might want to create the
file instead of terminating the process.

Recall from Chapter 2 the section on "Handling Potential Failure with the
`Result` Type" that the `Result` enum is defined as having two variants, `Ok`
and `Err`, as follows:

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
<!-- This notation looks similar to a `match`, but it's not a `match`, so we
think this would be confusing. We've tried to clarify better in the text.
/Carol -->

The `T` and `E` are generic type parameters; we'll go into generics in more
detail in Chapter 10. What you need to know right now is that `T` represents
the type of the value that will be returned in a success case within the `Ok`
variant, and `E` represents the type of the error that will be returned in a
failure case within the `Err` variant. Because `Result` has these generic type
parameters, we can use the `Result` type and the functions that the standard
library has defined on it in many different situations where the successful
value and error value we want to return may differ.

Let's call a function that returns a `Result` value because the function could
fail: opening a file, shown in Listing 9-2.

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

How do we know `File::open` returns a `Result`? We could look at the standard
library API documentation. We could ask the compiler! If we give `f` a type
annotation of some type that we know the return type of the function is *not*,
then we try to compile the code, the compiler will tell us that the types don't
match. The error message will then tell us what the type of `f` *is*! Let's try
it: we know that the return type of `File::open` isn't of type `u32`, so let's
change the `let f` statement to:

```rust,ignore
let f: u32 = File::open("hello.txt");
```

Attempting to compile now gives us:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum `std::result::Result`
  |
  = note: expected type `u32`
  = note:    found type `std::result::Result<std::fs::File, std::io::Error>`
```

This tells us the return type of the `File::open` function is a `Result<T, E>`.
The generic parameter `T` has been filled in here with the type of the success
value, `std::fs::File`, which is a file handle. The type of `E` used in the
error value is `std::io::Error`.

This return type means the call to `File::open` might succeed and return to us
a file handle that we can read from or write to. The function call also might
fail: for example, the file might not exist, or we might not have permission to
access the file. The `File::open` function needs to have a way to tell us
whether it succeeded or failed, and at the same time give us either the file
handle or error information. This information is exactly what the `Result` enum
conveys.

In the case where `File::open` succeeds, the value we will have in the variable
`f` will be an instance of `Ok` that contains a file handle. In the case where
it fails, the value in `f` will be an instance of `Err` that contains more
information about the kind of error that happened.

<!--Can you say explicitly why there being many ways things can fail means we
use the result type? Also, are we importing the File type from the standard
crate here? That seems worth mentioning. -->
<!-- We think it would be repetitive to point out every example that imports a
type from the standard library. We're past the Modules Chapter 7 "Importing
Names With Use" section that explains the concept in depth, as well as multiple
examples in the Hash maps section of Chapter 8 that show how and why to import
types from the standard library. /Carol -->

We need to add to the code from Listing 9-2 to take different actions depending
on the value `File::open` returned. Listing 9-3 shows one way to handle the
`Result` with a basic tool: the `match` expression that we learned about in
Chapter 6.

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

<figcaption>

Listing 9-3: Using a `match` expression to handle the `Result` variants we
might have

</figcaption>
</figure>

<!-- So we don't need the Result keyword in this code example? And what is the
{:?} syntax, can you include a line about that? -->
<!-- We've added an explanation that Result is like Option in that it's
imported into the prelude, which the reader should be familiar with. We
explained the {:?} syntax in Structs, chapter 5, in the section "Adding Useful
Functionality with Derived Traits". It's the debug format. Having to re-explain
multiple concepts that are not the primary focus of this example really
obscures the point of the section. /Carol -->

Note that, like the `Option` enum, the `Result` enum and its variants have been
imported in the prelude, so we don't need to specify `Result::` before the `Ok`
and `Err` variants in the `match` arms.

Here we tell Rust that when the result is `Ok`, return the inner `file` value
out of the `Ok` variant, and we then assign that file handle value to the
variable `f`. After the `match`, we can then use the file handle for reading or
writing.

The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we've chosen to call the `panic!` macro. If
there's no file named `hello.txt` in our current directory and we run this
code, we'll see the following output from the `panic!` macro:

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

<!-- Do we have to manually print the error message, or does it show when we
run the program? -->
<!-- No, the `panic!` macro prints what we give to it, which we covered in the
section previous to this one. /Carol -->

### Matching on Different Errors

The code in Listing 9-3 will `panic!` no matter the reason that `File::open`
failed. What we'd really like to do instead is take different actions for
different failure reasons: if `File::open` failed because the file doesn't
exist, we want to create the file and return the handle to the new file. If
`File::open` failed for any other reason, for example because we didn't have
permission to open the file, we still want to `panic!` in the same way as we
did in Listing 9-3. Let's look at Listing 9-4, which adds another arm to the
`match`:

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

The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value.
`io::ErrorKind` is an enum provided by the standard library that has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we're interested in is `ErrorKind::NotFound`, which
indicates the file we're trying to open doesn't exist yet.

The condition `if error.kind() == ErrorKind::NotFound` is called a *match
guard*: it's an extra condition on a `match` arm that further refines the arm's
pattern. This condition must be true in order for that arm's code to get run;
otherwise, the pattern matching will move on to consider the next arm in the
`match`.  The `ref` in the pattern is needed so that the `error` is
not moved into the guard condition but is merely referenced by it.

<!-- Hm, how come we use `ref` as the syntax here and not &? -->

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
defined on it to do various things. One of those methods, called `unwrap`, is
what causes the program to "panic on an error result":

<!-- I'll ghost everything except `unwrap()` in the libreoffice file /Carol -->

<span class="filename">Filename: src/main.rs</span>

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

<span class="filename">Filename: src/main.rs</span>

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

<figure>

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

<figcaption>

Listing 9-5: A function that returns errors to the calling code using `match`

</figcaption>
</figure>

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

<figure>

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

<figcaption>

Listing 9-6: A function that returns errors to the calling code using `try!`

</figcaption>
</figure>

Or as in Listing 9-7, which uses the question mark operator:

<figure>

```rust
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

<figcaption>

Listing 9-7: A function that returns errors to the calling code using `?`

</figcaption>
</figure>

The `?` operator at the end of the `open` call does the same thing as the
example that uses `match` and the example that uses the `try!` macro: It will
return the value inside an `Ok` to the binding `f`, but will return early out
of the whole function and give any `Err` value we get to our caller. The same
thing applies to the `?` at the end of the `read_to_string` call.

The advantage of using the question mark operator over the `try!` macro is the
question mark operator permits chaining. We could further shorten this code
by instead doing:

```rust
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

<span class="filename">Filename: src/main.rs</span>

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

```text
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
