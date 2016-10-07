# Documentation Tests

Nothing is better than documentation with examples. Nothing is worse than
examples that don't actually work, because the code has changed since the
documentation has been written. To this end, Rust supports automatically
running examples in your documentation for library crates. Here's a fleshed-out
`src/lib.rs` with examples:

```rust
//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}
```

Note the module-level documentation with `//!` and the function-level
documentation with `///`. Rust's documentation supports Markdown in comments,
and so triple graves mark code blocks. It is conventional to include the
`# Examples` section, exactly like that, with examples following.

Let's run the tests again:

```bash
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
     Running target/adder-91b3e234d4ed382a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/lib-c18e7d3494509e74

running 1 test
test it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 2 tests
test add_two_0 ... ok
test _0 ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Now we have all three kinds of tests running! Note the names of the
documentation tests: the `_0` is generated for the module test, and `add_two_0`
for the function test. These will auto increment with names like `add_two_1` as
you add more examples.

## Automatic `main` insertion

Let's discuss our sample example documentation:

```rust
/// ```
/// println!("Hello, world");
/// ```
# fn foo() {}
```

You'll notice that you don't need a `fn main()` or anything here. `rustdoc`
will automatically add a `main()` wrapper around your code, using heuristics to
attempt to put it in the right place. For example:

```rust
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
# fn foo() {}
```

This will end up testing:

```rust
fn main() {
    use std::rc::Rc;
    let five = Rc::new(5);
}
```

Here's the full algorithm rustdoc uses to preprocess examples:

1. Any leading `#![foo]` attributes are left intact as crate attributes.
2. Some common `allow` attributes are inserted, including
   `unused_variables`, `unused_assignments`, `unused_mut`,
   `unused_attributes`, and `dead_code`. Small examples often trigger
   these lints.
3. If the example does not contain `extern crate`, then `extern crate
   <mycrate>;` is inserted (note the lack of `#[macro_use]`).
4. Finally, if the example does not contain `fn main`, the remainder of the
   text is wrapped in `fn main() { your_code }`.

This generated `fn main` can be a problem! If you have `extern crate` or a
`mod` statements in the example code that are referred to by `use` statements,
they will fail to resolve unless you include at least `fn main() {}` to inhibit
step 4. `#[macro_use] extern crate` also does not work except at the crate
root, so when testing macros an explicit `main` is always required. It doesn't
have to clutter up your docs, though — keep reading!

## Hiding extraneous code with `#`

Sometimes this algorithm isn't enough, though. For example, all of these code
samples with `///` we've been talking about? The raw text:

```text
/// Some documentation.
# fn foo() {}
```

looks different than the output:

```rust
/// Some documentation.
# fn foo() {}
```

Yes, that's right: we can add lines that start with `# `, and they will be
hidden from the output, but will be used when compiling our code. We can use
this to our advantage. In this case, documentation comments need to apply to
some kind of function, so if We want to show off a documentation comment, I
need to add a little function definition below it. At the same time, it's only
there to satisfy the compiler, so hiding it makes the example more clear. We
can use this technique to explain longer examples in detail, while still
preserving the testability of our documentation.

For example, imagine that we wanted to document this code:

```rust
let x = 5;
let y = 6;
println!("{}", x + y);
```

We might want the documentation to end up looking like this:

> First, we set `x` to five:
>
> ```rust
> let x = 5;
> # let y = 6;
> # println!("{}", x + y);
> ```
>
> Next, we set `y` to six:
>
> ```rust
> # let x = 5;
> let y = 6;
> # println!("{}", x + y);
> ```
>
> Finally, we print the sum of `x` and `y`:
>
> ```rust
> # let x = 5;
> # let y = 6;
> println!("{}", x + y);
> ```

To keep each code block testable, we want the whole program in each block, but
we don't want the reader to see every line every time.  Here's what we put in
our source code:

```text
    First, we set `x` to five:

    ```rust
    let x = 5;
    # let y = 6;
    # println!("{}", x + y);
    ```

    Next, we set `y` to six:

    ```rust
    # let x = 5;
    let y = 6;
    # println!("{}", x + y);
    ```

    Finally, we print the sum of `x` and `y`:

    ```rust
    # let x = 5;
    # let y = 6;
    println!("{}", x + y);
    ```
```

By repeating all parts of the example, we can ensure that our example still
compiles, while only showing the parts that are relevant to that part of our
explanation.

Another case where the use of `#` is handy is when you want to ignore
error handling. Lets say you want the following,

```rust,ignore
/// use std::io;
/// let mut input = String::new();
/// try!(io::stdin().read_line(&mut input));
```

The problem is that `try!` returns a `Result<T, E>` and test functions
don't return anything so this will give a mismatched types error.

```rust,ignore
/// A doc test using try!
///
/// ```
/// use std::io;
/// # fn foo() -> io::Result<()> {
/// let mut input = String::new();
/// try!(io::stdin().read_line(&mut input));
/// # Ok(())
/// # }
/// ```
# fn foo() {}
```

You can get around this by wrapping the code in a function. This catches
and swallows the `Result<T, E>` when running tests on the docs. This
pattern appears regularly in the standard library.

## Adding attributes to control documentation testing.

In the first part of the chapter, we talked about attributes that help with
testing:

```rust
#[test]
#[ignore]
fn it_works() {
}

#[should_panic]
fn it_works() {
    assert!(false);
}
```

We can use these annotations in documentation tests as well:

```rust
/// ```rust,ignore
/// fn foo() {
/// ```
fn foo() {}

/// ```rust,should_panic
/// assert!(false);
/// ```
fn bar() {}
```

## The `no_run` attribute

There's one attribute that's specific to documentation tests:

```rust
/// ```rust,no_run
/// loop {
///     println!("Hello, world");
/// }
/// ```
# fn foo() {}
```

The `no_run` attribute will compile your code, but not run it. This is
important for examples such as "Here's how to start up a network service,"
which you would want to make sure compile, but might run in an infinite loop!
