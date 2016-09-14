# Recoverable errors with `Result<T, E>`

Most errors aren't so dire. Sometimes, when a function fails, it's for a reason
that we can easily interpret and respond to. As an example, maybe we are
making a request to a website, but it's down for maintenance. In this
situation, we'd like to wait and then try again. Terminating our process isn't
the right thing to do here.

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

As an example, let's try opening a file:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

The `open` function returns a `Result`: there are many ways in which opening
a file can fail. For example, unless we created `hello.txt`, this file does
not yet exist. Before we can do anything with our `File`, we need to extract
it out of the result. Let's start with a basic tool: `match`. We've used it
to deal with enums previously.

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

If we see an `Ok`, we can return the inner `file` out of the `Ok` variant. If
we see `Err`, we have to decide what to do with it. The simplest thing is to
turn our error into a `panic!` instead, by calling the macro. And since we
haven't created that file yet, we'll see a message indicating as such when we
print the error value:

```bash
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

This works okay. However, `match` can be a bit verbose, and it doesn't always
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

This has the same behavior as our previous example: If the call to `open()`
returns `Ok`, return the value inside. If it's an `Err`, panic.

There's also another method, similar to `unwrap()`, but that lets us choose the
error message: `expect()`. Using `expect()` instead of `unwrap()` and providing
good error messages can convey your intent and make tracking down the source of
a panic easier. `expect()` looks like this:

<!-- I'll ghost everything except `expect()` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("failed to open hello.txt");
}
```

This isn't the only way to deal with errors, however. This entire section is
supposed to be about recovering from errors, but we've gone back to panic. This
observation gets at an underlying truth: you can easily turn a recoverable
error into an unrecoverable one with `unwrap()` or `expect()`, but you can't
turn an unrecoverable `panic!` into a recoverable one. This is why good Rust
code chooses to make errors recoverable: you give your caller choices.

The Rust community has a love/hate relationship with `unwrap()` and `expect()`.
They're useful in tests since they will cause the test to fail if there's an
error anyplace you call them. In examples, you might not want to muddy the code
with proper error handling. But if you use them in a library, mis-using your
library can cause other people's programs to halt unexpectedly, and that's not
very user-friendly.

## Propagating errors with `?`

When writing a function, if you don't want to handle the error where you are,
you can return the error to the calling function. Within your function, that
would look like:

<!-- I'll ghost everything except `return Err(e)` in the libreoffice file /Carol -->

```rust,ignore
# use std::fs::File;
# fn foo() -> std::io::Result<()> {
let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
};

# Ok(())
# }
```

This is a very common way of handling errors: propagate them upward until
you're ready to deal with them. This pattern is so common in Rust that there is
dedicated syntax for it: the question mark operator. We could have also written
the example like this:

<!-- I'll ghost everything except `?` in the libreoffice file /Carol -->

```rust,ignore
#![feature(question_mark)]

use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

The `?` operator at the end of the `open` call does the same thing as our
previous example: It will return the value inside an `Ok` to the binding `f`,
but will return early out of the whole function and give any `Err` value we get
to our caller.

There's one problem though; let's try compiling the example:

```rust,ignore
   Compiling result v0.1.0 (file:///projects/result)
error[E0308]: mismatched types
 --> src/main.rs:6:13
  |
6 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum
`std::result::Result`
  |
  = note: expected type `()`
  = note:    found type `std::result::Result<_, _>`

error: aborting due to previous error
```

What gives? The issue is that the `main()` function has a return type of `()`,
but the question mark operator is trying to return a `Result`. This doesn't
work. Instead of `main()`, let's create a function that returns a `Result` so
that we are allowed to use the question mark operator:

```rust
#![feature(question_mark)]

use std::fs::File;
use std::io;

pub fn process_file() -> Result<(), io::Error> {
    let f = File::open("hello.txt")?;

    // do some stuff with f

    Ok(())
}
```

Since the `Result` type has two type parameters, we need to include them both
in our function signature. In this case, `File::open` returns `std::io::Error`,
so we will use it as our error type. But what about success? This function is
executed purely for its side effects; no value is returned when everything
works. Functions with no return type, as we just saw with `main()`, are the
same as returning the unit type, `()`. So we can use the unit type as the
return type here, too.

This leads us to the last line of the function, the slightly silly-looking
`Ok(())`. This is an `Ok()` with a `()` value inside.

Now we have the function `process_file` that uses the question mark operator and compiles successfully. We can call this function and handle the return value in any other function that returns a `Result` by also using the question mark operator there, and that would look like `process_file()?`. However, the `main` function still returns `()`, so we would still have to use a `match` statement, an `unwrap()`, or an `expect()` if we wanted to call `process_file` from `main`. Using `expect()` would look like this:

```rust,ignore
fn main() {
    process_file().expect("There was a problem processing the file");
}
```

## Chaining Question Mark Operators

TODO
