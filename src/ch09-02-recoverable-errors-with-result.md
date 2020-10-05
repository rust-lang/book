## Recoverable Errors with `Result`

Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily
interpret and respond to. For example, if you try to open a file and that
operation fails because the file doesn’t exist, you might want to create the
file instead of terminating the process.

Recall from [“Handling Potential Failure with the `Result`
Type”][handle_failure]<!-- ignore --> in Chapter 2 that the `Result` enum is
defined as having two variants, `Ok` and `Err`, as follows:

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters: we’ll discuss generics in more
detail in Chapter 10. What you need to know right now is that `T` represents
the type of the value that will be returned in a success case within the `Ok`
variant, and `E` represents the type of the error that will be returned in a
failure case within the `Err` variant. Because `Result` has these generic type
parameters, we can use the `Result` type and the functions that the standard
library has defined on it in many different situations where the successful
value and error value we want to return may differ.

Let’s call a function that returns a `Result` value because the function could
fail. In Listing 9-3 we try to open a file.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

<span class="caption">Listing 9-3: Opening a file</span>

How do we know `File::open` returns a `Result`? We could look at the [standard
library API documentation](../std/index.html)<!-- ignore -->, or we could ask
the compiler! If we give `f` a type annotation that we know is *not* the return
type of the function and then try to compile the code, the compiler will tell
us that the types don’t match. The error message will then tell us what the
type of `f` *is*. Let’s try it! We know that the return type of `File::open`
isn’t of type `u32`, so let’s change the `let f` statement to this:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/src/main.rs:here}}
```

Attempting to compile now gives us the following output:

```console
{{#include ../listings/ch09-error-handling/no-listing-02-ask-compiler-for-type/output.txt}}
```

This tells us the return type of the `File::open` function is a `Result<T, E>`.
The generic parameter `T` has been filled in here with the type of the success
value, `std::fs::File`, which is a file handle. The type of `E` used in the
error value is `std::io::Error`.

This return type means the call to `File::open` might succeed and return a file
handle that we can read from or write to. The function call also might fail:
for example, the file might not exist, or we might not have permission to
access the file. The `File::open` function needs to have a way to tell us
whether it succeeded or failed and at the same time give us either the file
handle or error information. This information is exactly what the `Result` enum
conveys.

In the case where `File::open` succeeds, the value in the variable `f` will be
an instance of `Ok` that contains a file handle. In the case where it fails,
the value in `f` will be an instance of `Err` that contains more information
about the kind of error that happened.

We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

<span class="caption">Listing 9-4: Using a `match` expression to handle the
`Result` variants that might be returned</span>

Note that, like the `Option` enum, the `Result` enum and its variants have been
brought into scope by the prelude, so we don’t need to specify `Result::`
before the `Ok` and `Err` variants in the `match` arms.

Here we tell Rust that when the result is `Ok`, return the inner `file` value
out of the `Ok` variant, and we then assign that file handle value to the
variable `f`. After the `match`, we can use the file handle for reading or
writing.

The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we’ve chosen to call the `panic!` macro. If
there’s no file named *hello.txt* in our current directory and we run this
code, we’ll see the following output from the `panic!` macro:

```text
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

As usual, this output tells us exactly what has gone wrong.

### Matching on Different Errors

The code in Listing 9-4 will `panic!` no matter why `File::open` failed. What
we want to do instead is take different actions for different failure reasons:
if `File::open` failed because the file doesn’t exist, we want to create the
file and return the handle to the new file. If `File::open` failed for any
other reason—for example, because we didn’t have permission to open the file—we
still want the code to `panic!` in the same way as it did in Listing 9-4. Look
at Listing 9-5, which adds an inner `match` expression.

<span class="filename">Filename: src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

<span class="caption">Listing 9-5: Handling different kinds of errors in
different ways</span>

The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value. The enum
`io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we’re trying to open doesn’t exist yet. So we match on `f`, but we
also have an inner match on `error.kind()`.

The condition we want to check in the inner match is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need a second arm in the inner `match` expression. When the
file can’t be created, a different error message is printed. The second arm of
the outer `match` stays the same, so the program panics on any error besides
the missing file error.

That’s a lot of `match`! The `match` expression is very useful but also very
much a primitive. In Chapter 13, you’ll learn about closures; the `Result<T,
E>` type has many methods that accept a closure and are implemented using
`match` expressions. Using those methods will make your code more concise. A
more seasoned Rustacean might write this code instead of Listing 9-5:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-03-closures/src/main.rs}}
```

Although this code has the same behavior as Listing 9-5, it doesn’t contain any
`match` expressions and is cleaner to read. Come back to this example after
you’ve read Chapter 13, and look up the `unwrap_or_else` method in the standard
library documentation. Many more of these methods can clean up huge nested
`match` expressions when you’re dealing with errors.

### Shortcuts for Panic on Error: `unwrap` and `expect`

Using `match` works well enough, but it can be a bit verbose and doesn’t always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various tasks. One of those methods, called `unwrap`, is a
shortcut method that is implemented just like the `match` expression we wrote in
Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

If we run this code without a *hello.txt* file, we’ll see an error message from
the `panic!` call that the `unwrap` method makes:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

Another method, `expect`, which is similar to `unwrap`, lets us also choose the
`panic!` error message. Using `expect` instead of `unwrap` and providing good
error messages can convey your intent and make tracking down the source of a
panic easier. The syntax of `expect` looks like this:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here’s what it looks like:

```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

Because this error message starts with the text we specified, `Failed to open
hello.txt`, it will be easier to find where in the code this error message is
coming from. If we use `unwrap` in multiple places, it can take more time to
figure out exactly which `unwrap` is causing the panic because all `unwrap`
calls that panic print the same message.

### Propagating Errors

When you’re writing a function whose implementation calls something that might
fail, instead of handling the error within this function, you can return the
error to the calling code so that it can decide what to do. This is known as
*propagating* the error and gives more control to the calling code, where there
might be more information or logic that dictates how the error should be
handled than what you have available in the context of your code.

For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called this function.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `match`</span>

This function can be written in a much shorter way, but we’re going to start by
doing a lot of it manually in order to explore error handling; at the end,
we’ll show the shorter way. Let’s look at the return type of the function first:
`Result<String, io::Error>`. This means the function is returning a value of
the type `Result<T, E>` where the generic parameter `T` has been filled in
with the concrete type `String` and the generic type `E` has been filled in
with the concrete type `io::Error`. If this function succeeds without any
problems, the code that calls this function will receive an `Ok` value that
holds a `String`—the username that this function read from the file. If this
function encounters any problems, the code that calls this function will
receive an `Err` value that holds an instance of `io::Error` that contains
more information about what the problems were. We chose `io::Error` as the
return type of this function because that happens to be the type of the error
value returned from both of the operations we’re calling in this function’s
body that might fail: the `File::open` function and the `read_to_string`
method.

The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value returned with a `match` similar to the `match` in
Listing 9-4, only instead of calling `panic!` in the `Err` case, we return
early from this function and pass the error value from `File::open` back to the
calling code as this function’s error value. If `File::open` succeeds, we store
the file handle in the variable `f` and continue.

Then we create a new `String` in variable `s` and call the `read_to_string`
method on the file handle in `f` to read the contents of the file into `s`. The
`read_to_string` method also returns a `Result` because it might fail, even
though `File::open` succeeded. So we need another `match` to handle that
`Result`: if `read_to_string` succeeds, then our function has succeeded, and we
return the username from the file that’s now in `s` wrapped in an `Ok`. If
`read_to_string` fails, we return the error value in the same way that we
returned the error value in the `match` that handled the return value of
`File::open`. However, we don’t need to explicitly say `return`, because this
is the last expression in the function.

The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. We
don’t know what the calling code will do with those values. If the calling code
gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upward for
it to handle appropriately.

This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.

#### A Shortcut for Propagating Errors: the `?` Operator

Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as it had in Listing 9-6, but this implementation uses the
`?` operator.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

<span class="caption">Listing 9-7: A function that returns errors to the
calling code using the `?` operator</span>

The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.

There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert errors from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent all
the ways a function might fail, even if parts might fail for many different
reasons. As long as each error type implements the `from` function to define how
to convert itself to the returned error type, the `?` operator takes care of the
conversion automatically.

In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `f`. If an error occurs, the
`?` operator will return early out of the whole function and give any `Err`
value to the calling code. The same thing applies to the `?` at the end of the
`read_to_string` call.

The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

<span class="caption">Listing 9-8: Chaining method calls after the `?`
operator</span>

We’ve moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn’t changed. Instead of creating a variable `f`, we’ve
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-6 and
Listing 9-7; this is just a different, more ergonomic way to write it.

Speaking of different ways to write this function, Listing 9-9 shows that
there’s a way to make this even shorter.

<span class="filename">Filename: src/main.rs</span>

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

<span class="caption">Listing 9-9: Using `fs::read_to_string` instead of
opening and then reading the file</span>

Reading a file into a string is a fairly common operation, so Rust provides the
convenient `fs::read_to_string` function that opens the file, creates a new
`String`, reads the contents of the file, puts the contents into that `String`,
and returns it. Of course, using `fs::read_to_string` doesn’t give us the
opportunity to explain all the error handling, so we did it the longer way
first.

#### The `?` Operator Can Be Used in Functions That Return `Result`

The `?` operator can be used in functions that have a return type of
`Result`, because it is defined to work in the same way as the `match`
expression we defined in Listing 9-6. The part of the `match` that requires a
return type of `Result` is `return Err(e)`, so the return type of the function
has to be a `Result` to be compatible with this `return`.

Let’s look at what happens if we use the `?` operator in the `main` function,
which you’ll recall has a return type of `()`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/src/main.rs}}
```

When we compile this code, we get the following error message:

```console
{{#include ../listings/ch09-error-handling/no-listing-06-question-mark-in-main/output.txt}}
```

This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result` or `Option` or another type that implements
`std::ops::Try`. When you’re writing code in a function
that doesn’t return one of these types, and you want to use `?` when you call other
functions that return `Result<T, E>`, you have two choices to fix this problem.
One technique is to change the return type of your function to be `Result<T,
E>` if you have no restrictions preventing that. The other technique is to use
a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in
whatever way is appropriate.

The `main` function is special, and there are restrictions on what its return
type must be. One valid return type for main is `()`, and conveniently, another
valid return type is `Result<T, E>`, as shown here:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-07-main-returning-result/src/main.rs}}
```

The `Box<dyn Error>` type is called a trait object, which we’ll talk about in
the [“Using Trait Objects that Allow for Values of Different
Types”][trait-objects]<!-- ignore --> section in Chapter 17. For now, you can
read `Box<dyn Error>` to mean “any kind of error.” Using `?` in a `main`
function with this return type is allowed.

Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.

[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
