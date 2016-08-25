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

We have `Ok` for successful results, and `Err` for ones that have an error.
These two variants each contain one thing: in `Ok`'s case, it's the successful
return value. With `Err`, it's some type that represents the error.

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

If we see an `Ok`, we can return the inner `file` out of it. If we see `Err`,
we have to decide what to do with it. The simplest thing is to turn our error
into a `panic!` instead, by calling the macro. And since we haven't created
that file yet, we'll see it in the error message:

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

This works okay. However, `match` can be a bit verbose, and it doesn't always
communicate intent well. The `Result<T, E>` type has many helper methods
defined it to do various things. "Panic on an error result" is one of those
methods, and it's called `unwrap`:

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

This has the same behavior as our previous example: If the call to `open`
returns `Ok`, return the value inside. If it's an `Err`, panic.

This isn't the only way to deal with errors, however. This entire section is
supposed to be about recovering from errors, but we've gone back to panic.
This is true, and gets at an underlying truth: you can easily turn a
recoverable error into an unrecoverable one with `unwrap`, but you can't turn
an unrecoverable error into a recoverable one. This is why good Rust code
chooses to make errors recoverable: you give your caller options.

The Rust community has a love/hate relationship with `unwrap`. It's useful
in tests, and in examples where you don't want to muddy the example with proper
error handling. But if used in library code, mis-using that library can cause
your program to blow up, and that's not good.

## Propagating errors with `?`

Sometimes, when writing a function, you don't want to handle the error where
you are, but instead would prefer to return the error to the calling function.
Something like this:

```rust
use std::fs::File;

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

```rust
#![feature(question_mark)]

use std::fs::File;

# fn foo() -> std::io::Result<()> {
let f = File::open("hello.txt")?;
# Ok(())
# }
```

The `?` operator at the end of the `open` call does the same thing as our
previous example: It will return the value of an `Ok`, but return the value of
an `Err` to our caller.

There's one problem though: let's try compiling the example:

```rust,ignore
   Compiling result v0.1.0 (file:///home/steve/tmp/result)
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
but the question mark operator is trying to return a result. This doesn't work.
Instead of `main()`, let's create a function that returns a result:

```rust
#![feature(question_mark)]

use std::fs::File;
use std::io;

fn main() {
}

pub fn process_file() -> Result<(), io::Error> {
    let f = File::open("hello.txt")?;

    // do some stuff with f

    Ok(())
}
```

Since the result type has two type parameters, we need to include them. In this
case, `File::open` returns an `std::io::Error`, so we will use it as our error
type. But what about success? This function is executed purely for its side
effects; nothing is returned upon success. Well, functions with no return type,
as we just saw with `main()`, are the same as returning unit. So we can use
it as the return type here, too. This leads to the last line of the function,
the slightly silly-looking `Ok(())`. This is an `Ok()` with a `()` inside.

In chapter XX, we'll learn how to make our own types like these, but for now,
an understanding of the core `Result<T, E>` is enough.
