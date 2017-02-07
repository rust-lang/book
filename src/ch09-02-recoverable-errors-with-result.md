## Recoverable Errors with `Result`

Most errors aren't serious enough to require the program to stop entirely.
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
`match`. The `ref` in the pattern is needed so that the `error` is not moved
into the guard condition but is merely referenced by it. The reason `ref` is
used to take a reference in a pattern instead of `&` will be covered in detail
in Chapter XX. In short, in the context of a pattern, `&` matches a reference
and gives us its value, but `ref` matches a value and gives us a reference to it.

The condition we want to check in the match guard is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, since `File::create`
could also fail, we need to add an inner `match` statement as well! When the
file can't be opened, a different error message will be printed. The last arm
of the outer `match` stays the same so that the program panics on any error
besides the missing file error.

### Shortcuts for Panic on Error: `unwrap` and `expect`

Using `match` works well enough, but it can be a bit verbose and doesn't always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various things. One of those methods, called `unwrap`, is
a shortcut method that is implemented just like the `match` statement we wrote
in Listing 9-3. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us.

<!-- Can you explain a bit more what unwrap() does---you mean every time we
cause a panic it calls the unwrap method? -->
<!-- I'm not sure how the conclusion "every time we cause a panic it calls the
unwrap method" follows from the text that was here, but I've tried to reword.
Please let us know what part of the text specifically is implying that here so
that we can be sure that we've fixed it. /Carol -->

<!-- I'll ghost everything except `unwrap()` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

<!-- Can you talk ore about the syntax here, how it differs? It looks like
there aren't generics here for T and E. How is this still related to Result? -->
<!-- I'm not sure how to make this clearer. We're chaining the method call onto
the return value of the `File::open` function, which hasn't changed. The reader
should understand method calls by now. T and E are part of the *definition* of
the Result type, since Listing 9-2 we've been talking about *using* a Result
instance. Listings 9-2, 9-3, and 9-4 don't contain T and E either, so I'm not
sure why it's confusing that this code doesn't contain T and E. /Carol -->

If we run this code without a *hello.txt* file, we'll see an error message from
the `panic` call that the `unwrap` method makes:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
../src/libcore/result.rs:837
```

There's another method similar to `unwrap` that lets us also choose the
`panic!` error message: `expect`. Using `expect` instead of `unwrap` and
providing good error messages can convey your intent and make tracking down the
source of a panic easier. The syntax of`expect` looks like this:

<!-- I'll ghost everything except `expect(...)` in the libreoffice file /Carol -->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message that `expect` uses in its call to
`panic!` will be the parameter that we pass to `expect` instead of the default
`panic!` message that `unwrap` uses. Here's what it looks like:

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', ../src/libcore/result.rs:837
```

<!-- I added the above paragraph, can you review it and correct it as
necessary? So this is like what we did in Listing 9-3?-->
<!-- Yes, the implementations for both `unwrap` and `expect` are similar to 9-3,
which we want to show so that the reader knows they don't have to write out all
of 9-3 every time they have a `Result` value. Does this comment mean your
earlier comments in this section are moot? /Carol -->

<!-- Is panic used for both types of errors? The introduction makes it seem as
though it's only for unrecoverable errors -->
<!-- When you call panic, you are causing the program to crash and therefore
creating an unrecoverable error. You can choose to do that at any time, even
when there are *no* errors. There's nothing that prevents you from calling
`panic!` inappropriately, which is why the "to panic or not to panic" section
goes over the criteria the reader should use to decide if they're in a
situation that's recoverable or not. I've actually moved the text that was here
into that section to keep that whole discussion together. /Carol
-->

### Propagating Errors

When writing a function whose implementation calls something that might fail,
instead of handling the error within this function, you can choose to let your
caller know about the error so they can decide what to do. This is known as
*propagating* the error, and gives more control to the calling code where there
might be more information or logic that dictates how the error should be
handled than what you have available in the context of your code.

<!-- What's the benefit/result of returning the error to the code that called
the function, besides putting off handling it---can you lay that out? -->
<!-- We're giving control/decision making ability to the code that's calling
our code. I've tried to be more explicit here; please let me know what could be
improved if it's still not clear. /Carol -->

For example, Listing 9-5 shows a function that reads a username from a file. If
the file doesn't exist or can't be read, this function will return those errors
to the code that called this function:

<figure>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

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

Let's look at the return type of the function first: `Result<String,
io::Error>`. This means that the function is returning a value of the type
`Result<T, E>` where the generic parameter `T` has been filled in with the
concrete type `String`, and the generic type `E` has been filled in with the
concrete type `io::Error`. If this function succeeds without any problems, the
caller of this function will receive an `Ok` value that holds a `String`—the
username that this function read from the file. If this function encounters any
problems, the caller of this function will receive an `Err` value that holds an
instance of `io::Error` that contains more information about what the problems
were. We chose `io::Error` as the return type of this function because that
happens to be the type of the error value returned from both of the operations
we're calling in this function's body that might fail: the `File::open`
function and the `read_to_string` method.

The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value returned with a `match` similar to the `match` in
Listing 9-3, only instead of calling `panic!` in the `Err` case, we return
early from this function and pass the error value from `File::open` back to the
caller as this function's error value. If `File::open` succeeds, we store the
file handle in the variable `f` and continue.

Then we create a new `String` in variable `s` and call the `read_to_string`
method on the file handle in `f` in order to read the contents of the file into
`s`. The `read_to_string` method also returns a `Result` because it might fail,
even though `File::open` succeeded. So we need another `match` to handle that
`Result`: if `read_to_string` succeeds, then our function has succeeded, and we
return the username from the file that's now in `s` wrapped in an `Ok`. If
`read_to_string` fails, we return the error value in the same way that we
returned the error value in the `match` that handled the return value of
`File::open`. We don't need to explicitly say `return`, however, since this is
the last expression in the function.

The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. We
don't know what the caller will do with those values. If they get an `Err`
value, they could choose to call `panic!` and crash their program, use a
default username, or look up the username from somewhere other than a file, for
example. We don't have enough information on what the caller is actually trying
to do, so we propagate all the success or error information upwards for them to
handle as they see fit.

This pattern of propagating errors is so common in Rust that there is dedicated
syntax to make this easier: `?`.

### A Shortcut for Propagating Errors: `?`

<!-- The `?` ended up stabilizing in 1.13 and is quickly becoming preferred over
`try!`, so we decided to only cover `?`. /Carol -->

Listing 9-6 shows an implementation of `read_username_from_file` that has the
same functionality as it had in Listing 9-5, but this implementation uses the
question mark:

<!-- I'll ghost everything except the question mark in libreoffice. /Carol -->

<figure>

```rust
use std::io;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<figcaption>

Listing 9-6: A function that returns errors to the calling code using `?`

</figcaption>
</figure>

<!-- Below, are we talking about what just the ? operator does, or what the
program with the ? operator does? -->
<!-- I'm not sure what the difference is. We're talking about what the ? does
in the context of this program... /Carol -->

The `?` placed after a `Result` value is defined to work the exact same way as
the`match` expressions we defined to handle the `Result` values in Listing 9-5.
If the value of the `Result` is an `Ok`, the value inside the `Ok` will get
returned from this expression and the program will continue. If the value is an
`Err`, the value inside the `Err` will be returned from the whole function as
if we had used the `return` keyword so that the error value gets propagated to
the caller.

In the context of Listing 9-6, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the binding `f`. If an error occurs, `?`
will return early out of the whole function and give any `Err` value to our
caller. The same thing applies to the `?` at the end of the `read_to_string`
call.

The `?` eliminates a lot of boilerplate and makes this function's
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

<!-- Can you explain what is happening in this code and how it differs? -->
<!-- I've tried to make it even clearer that the functionality does NOT differ
/Carol -->

We've moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn't changed. Instead of creating a variable `f`, we've
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-5 and
Listing 9-6, this is just a different, more ergonomic way to write it.

#### `?` Can Only Be Used in Functions That Return `Result`

<!-- I think we need a new heading here, could you suggest something? I'm sure
there's a better way to phrase this!-->
<!-- I've tried, but I'm not really sure how to say it any more succinctly than
this, I'm not sure if it's better than what you suggested /Carol -->

The `?` can only be used in functions that have a return type of `Result`,
since it is defined to work in exactly the same way as the `match` expression
we defined in Listing 9-5. The part of the `match` that requires a return type
of `Result` is `return Err(e)`, so the return type of the function must be a
`Result` to be compatible with this `return`.

<!-- Which functions return a Result and how would the reader know? I'm also not
sure what you mean by "expand", that they have the same functionality (but
condensed!)? -->
<!-- You can tell what any function returns by looking at the return type
defined in the function signature, I'm not sure what part of Chapter 3 wasn't
clear enough to convey that. The reader should be comfortable with function
signatures by this point, and could also use the API docs to tell what a
function returns.

I've reworded to remove the word expand, but yes, we meant "functionally
equivalent to replacing it with the longer code"

/Carol
-->

Let's look at what happens if use `try!` in the `main` function, which you'll
recall has a return type of `()`:

```rust,ignore
fn main() {
    let f = File::open("hello.txt")?;
}
```

<!-- NOTE: as of 2016-12-21, the error message when calling `?` in a function
that doesn't return a result is STILL confusing. Since we want to only explain
`?` now, I've changed the example, but if you try running this code you WON'T
get the error message below.

I'm bugging people to try and get
https://github.com/rust-lang/rust/issues/35946 fixed soon, hopefully before this
chapter gets through copy editing-- at that point I'll make sure to update this
error message. /Carol -->

When we compile this, we get the following error message:

```text
error[E0308]: mismatched types
 -->
  |
3 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
  |
  = note: expected type `()`
  = note:    found type `std::result::Result<_, _>`
```

This error is pointing out that we have mismatched types: the `main()` function
has a return type of `()`, but the `?` might return a `Result`. In functions
that don't return `Result`, when you call other functions that return `Result`,
you'll need to use a `match` or one of the `Result` methods to handle it,
instead of using `?` to potentially propagate the error to the caller.

Now that we've discussed the details of calling `panic!` or returning `Result`,
let's return to the topic of how to decide which is appropriate to use in which
cases.
