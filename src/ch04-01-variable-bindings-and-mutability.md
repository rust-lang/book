## Variable Bindings and Mutability

We mentioned in Chapter XX that by default, variable bindings are *immutable*.
This is one of many nudges that Rust's design has to encourage us to write our
code to get the most of the safety and easy concurrency that Rust has to offer.
We still have the option to make our bindings mutable, though. Let's explore
how and why Rust encourages us to favor immutability, and why we might want to
opt out of that.

Variable bindings being immutable means that once a value is bound, you can't
change that value. To illustrate this, let's generate a new project with Cargo.
Open a terminal, and navigate to the directory you want to store your projects
in. From there, run these commands:

```bash
$ cargo new --bin bindings
$ cd bindings
```

Then open *src/main.rs* and replace its code with the following:

```rust,ignore
fn main() {
    let x = 5;
    x = 6;
    println!("The value of x is: {}", x);
}
```

Save and run the program using `cargo run`, and you should receive an error
message, as in this output:

```bash
$ cargo run
   Compiling bindings v0.0.1 (file:///projects/bindings)
error: re-assignment of immutable variable `x` [--explain E0384]
 --> src/main.rs:3:5
3 |>     x = 6;
  |>     ^^^^^
note: prior assignment occurs here
 --> src/main.rs:2:9
2 |>     let x = 5;
  |>         ^
```

This is our first example of the compiler helping us find an error in our
program! Compiler errors can be frustrating. Keep in mind that they only mean
your program isn't safely doing what you want it to do yet; they do _not_ mean
that you're not a good programmer! Experienced Rustaceans still get compiler
errors. Try to keep in mind that the Rust compiler is trying to help your
program be the very best.

PROD: START BOX
######Extended Error Explanations

Now that you've seen an example of a Rust error, let's look at one particularly
useful aspect of errors. Rust encourages you to seek further information on the
kind of error you've received with output like this:

```bash
error: re-assignment of immutable variable `x` [--explain E0384]
```

This tells us that if we pass the `--explain` flag to `rustc` with the provided
error code, we can see an extended explanation which will try to explain common
causes of and solutions to that kind of error. Not every error has a longer
explanation, but many do. Here’s the explanation for the `E0384` error we
received:

````bash
$ rustc --explain E0384
This error occurs when an attempt is made to reassign an immutable variable.
For example:

```
fn main(){
    let x = 3;
    x = 5; // error, reassignment of immutable variable
}
```

By default, variables in Rust are immutable. To fix this error, add the keyword
`mut` after the keyword `let` when declaring the variable. For example:

```
fn main(){
    let mut x = 3;
    x = 5;
}
```
````

These explanations can really help if you’re stuck on an error, so don't
hesitate to look up the error code. The compiler is your friend, and it's there
to help.

PROD: END BOX

The error includes the message `re-assigment of immutable variable` because the
program tried to assign a second value to the `x` variable.


******* insert *why* immutability is desirable here *********


But bindings are
immutable only by default; you can make them mutable by adding `mut` in front
of the variable name. For example, change the program you just wrote to the
following:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

Running this, we get:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
The value of x is: 6
```

Using `mut`, we change the value that `x` binds to from `5` to `6`.


******* insert *why* you might want to use mutability here *********




One final thing about bindings: they can *shadow* previous bindings. Shadowing
is what happens when you declare two bindings with the same name. We say that
the first binding is ‘shadowed’ by the second, which means that the second
binding's value is what you will see when you use the variable after the second
binding. This can be useful if you’d like to perform a few transformations on a
value, but still leave the binding immutable. For example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

This program first binds `x` to a value of `5`. Then, it shadows `x` by saying
`let x =` again, taking the original value and adding `1` so that the value of
`x` is then `6`. The third `let` statement also shadows `x`, taking the
previous value and multiplying it by `2` to give `x` a final value of `12`. If
you run this, it will output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

Shadowing is useful because it lets us modify `x` without having to make the
variable mutable. This means the compiler will still keep us from accidentally
trying to mutate `x` directly later.

Now let's look at some of the types of values that we can bind variables to.
