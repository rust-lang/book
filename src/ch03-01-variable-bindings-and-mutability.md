## Variable Bindings and Mutability

We mentioned in Chapter 2 that by default, variable bindings are *immutable*.
This is one of many nudges in Rust that encourages us to write our code in a
way that gets the most of the safety and easy concurrency that Rust has to
offer. We still have the option to make our bindings mutable, though. Let's
explore how and why Rust encourages us to favor immutability, and why we might
want to opt out of that.

Variable bindings being immutable means that once a value is bound, you can't
change that value. To illustrate this, let's generate a new project in your
projects directory called `bindings` by using `cargo new --bin bindings`.

Then, in your new `bindings` directory, open `src/main.rs` and replace its code
with the following:

Filename: src/main.rs

```rust,ignore
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
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
 --> src/main.rs:4:5
4 |>     x = 6;
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
errors. The Rust compiler is trying to help your program be the very best.

<!-- PROD: START BOX -->
###### Extended Error Explanations

Now that you've seen a Rust error, let's take a moment to look at one
particularly useful aspect of errors. Rust encourages you to seek further
information on the kind of error you've received with output like this:

```bash
error: re-assignment of immutable variable `x` [--explain E0384]
```

This tells us that if we pass the `--explain` flag to `rustc` with the provided
error code, we can see an extended explanation which will try to explain common
causes of and solutions to that kind of error. Not every error has a longer
explanation, but many do. Here’s a portion of the explanation for the `E0384`
error we received:

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
````

These explanations can really help if you’re stuck on an error, so don't
hesitate to look up the error code. The compiler is your friend, and it's there
to help.

<!-- PROD: END BOX -->

The error tells us that the cause of the error is `re-assigment of immutable
variable`, because we tried to assign a second value to the immutable `x`
variable.

It's important that we get compile-time errors when we attempt to change a
value that we previously said was immutable because this very situation can
lead to bugs. If one part of our code operates on an assumption that a value
will never change, and another part of our code changes that value, it's
possible that the first part of the code won't do what it was designed to do.
This cause of bugs can be difficult to track down after the fact, especially
when the second piece of code only changes the value _sometimes_.

In Rust, we can trust that a value we say won't change really won't change,
because the compiler is enforcing that guarantee for us. When reading and
writing code, we don't have to keep track in our head how and where a value
might change. This can make code easier to reason about.

Mutability can be really useful, though! Bindings are immutable only by
default; you can make them mutable by adding `mut` in front of the variable
name. In addition to allowing this value to be changed, it conveys intent to
future readers of the code by indicating that other parts of the code will be
changing this value.

For example, change the program you just wrote to the following:

Filename: src/main.rs

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

Using `mut`, we are allowed to change the value that `x` binds to from `5` to
`6`. In some cases you'll want to make a binding mutable because it makes the
code easier to understand than an implementation that only uses immutable
bindings. In cases where you're using large data structures, mutating an
instance in place may be faster than copying and returning newly allocated
instances. It all depends on the tradeoffs you want to make in your situation.

### Shadowing

As we saw in the guessing game tutorial, we can declare new bindings with the
same name as a previous binding, and the new binding *shadows* the previous
binding. We say that the first binding is *shadowed* by the second, which means
that the second binding's value is what you will see when you use the variable.
We can shadow a binding by using the same binding's name and repeating the use
of the `let` keyword as follows:

Filename: src/main.rs

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

This program first binds `x` to a value of `5`. Then, it shadows `x` by
repeating `let x =`, taking the original value and adding `1` so that the value
of `x` is then `6`. The third `let` statement also shadows `x`, taking the
previous value and multiplying it by `2` to give `x` a final value of `12`. If
you run this, it will output:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

This is different from marking a binding as `mut` because unless we use the
`let` keyword again, we'll get a compile-time error if we accidentally try to
reassign to this binding. We can perform a few transformations on a value, but
have the binding be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that, since we're
effectively creating a new binding when we use the `let` keyword again, we can
change the type of the value we're binding to but reuse the same name. For
example, say we ask a user to show us how many spaces they want between some
text by sending us space characters, but we really want to store that as a
number:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

This is allowed: the first `spaces` binding is a string type, and the second
`spaces` binding, which is a brand new binding that happens to have the same
name as the first one, is a number type. Shadowing thus saves us from having to
come up with different names like `spaces_str` and `spaces_num`; we can reuse
the simpler `spaces` name. If we try to use `mut` for this, however, like this:

```rust,ignore
let mut spaces = "   ";
spaces = spaces.len();
```

We will get a compile-time error because we are not allowed to mutate a
binding's type:

```bash
error: mismatched types [--explain E0308]
 -->
  |>
4 |> spaces = spaces.len();
  |>          ^^^^^^^^^^^^ expected &-ptr, found usize
note: expected type `&str`
note:    found type `usize`

error: aborting due to previous error
```

Now that we've explored how variable bindings work, let's look at some more
data types of values that we can bind variables to.
