# Control flow with `if`

> Two roads diverged in a yellow wood,  
> And sorry I could not travel both  
> And be one traveler, long I stood  
> And looked down one as far as I could  
> To where it bent in the undergrowth;  
> 
> - Robert Frost, “The Road Not Taken”

In Rust, there are a few ways to cause our code to branch. The most fundamental
way is by using `if`. An `if` expression gives us two paths forward, and asks
the question, “Which one should I take?”

Let’s make a new project to explore `if`. Navigate to your projects directory,
and use Cargo to make a new project called `branches`:

```bash
$ cargo new --bin branches
$ cd branches
```

Here’s a sample program using `if`:

```rust
fn main() {
    let condition = true;

    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}   
```

Let's try running it:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

We can change the value of `condition`:

```rust
    let condition = false;
```

And then run it again:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

This is the very basic structure of `if`: _if_ the condition is true, then
execute some code. If it’s not true, then execute some other code, after
`else`.

An `else` is not required:

```rust
fn main() {
    let condition = false;

    if condition {
        println!("condition was true");
    }
}
```

In this case, nothing is printed.

It’s also worth noting that `condition` here _must_ be a `bool`. Let’s try an
example with something else:

```rust,ignore
fn main() {
    let condition = 5;

    if condition {
        println!("condition was five");
    }
}
```

If we try to run this program, Rust will complain:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
src/main.rs:4:8: 4:17 error: mismatched types:
 expected `bool`,
    found `_`
(expected bool,
    found integral variable) [E0308]
src/main.rs:4     if condition {
                     ^~~~~~~~~
src/main.rs:4:8: 4:17 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
Could not compile `branches`.
```

We expected a `bool`, but got an integer. Rust will not automatically try to convert non-boolean types to a boolean here. We must be explicit.

## `else if`

We can make multiple decisions by combining `if` and `else` in another way:

```rust
fn main() {
    let number = 5;

    if number == 3 {
        println!("condition was 3");
    } else if number == 4 {
        println!("condition was 4");
    } else if number == 5 {
        println!("condition was 5");
    } else {
        println!("condition was something else");
    }
}
```

Let's try running it:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was 5
```

When this program executes, it will check each `if` in turn, and execute the
first body for which the condition holds true.

Using a single `else if` can be okay, but if you find yourself with more than one,
you may want to refactor your code. Rust has a more powerful branching construct
called `match` for these cases. We'll cover it later, when we talk about `enums`.

## `if` as an expression

There’s one last detail we need to learn about `if`: it’s an expression. That means
that we can use it on the right hand side of a `let` binding, for instance:

```rust
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
``` 

Let’s run this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
The value of number is: 5
```

Remember, blocks of code evaluate to the last expression in them. And numbers
by themselves are also expressions. So in this case, the value of the whole
`if` expression depends on which block of code executes.

There’s another small detail involved here: this means that if you use `if`
in this way, both arms of the `if` must be the same type. This doesn’t work:

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

If we try to run this, we’ll get an error:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
src/main.rs:4:18: 8:6 error: if and else have incompatible types:
 expected `_`,
    found `&‘static str`
(expected integral variable,
    found &-ptr) [E0308]
src/main.rs:4     let number = if condition {
src/main.rs:5         5
src/main.rs:6     } else {
src/main.rs:7         "six"
src/main.rs:8     };
src/main.rs:4:18: 8:6 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to previous error
Could not compile `branches`.
```

`if` and `else` have incompatible types. This can’t work.
