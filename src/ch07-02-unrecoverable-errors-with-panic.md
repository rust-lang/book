# Unrecoverable errors with panic!

You've already seen the way to signal an unrecoverable error: the `panic!`
macro. Here's an example of using `panic!`:

```rust
fn check_guess(number: u32) -> bool {
    if number > 100 {
        panic!("Guess was too big: {}", number);
    }

    number == 34
}
```

This function accepts a guess between zero and a hundred, and checks if it's
equivalent to the correct number, which is `34` in this case. It's kind of a
silly function, but we need an example, so it works.

There's no number type for "between zero and a hundred" in Rust, so we are
accepting a `u32`, and then checking in the function's body to make sure the
guess is in-bounds. If the number is too big, there's been an error: someone
made a mistake. We then invoke `panic!` to say "something went wrong, we cannot
continue to run this program."

Checking some sort of condition and then `panic!`ing if something is wrong is a
common use of `panic!`. In fact, there's a second macro in Rust, `assert!` for
this case. `assert!` will check some kind of condition, and `panic!` if the
condition is false. We could also write our function like this:

```rust
fn check_guess(number: u32) -> bool {
    assert!(number < 100);

    number == 34
}
```

If we try and use `check_guess` in a program, and make an error:

```rust,should_panic
fn check_guess(number: u32) -> bool {
    assert!(number < 100);

    number == 34
}

fn main() {
    let answer = check_guess(5);
    println!("answer was: {}", answer);

    let answer = check_guess(34);
    println!("answer was: {}", answer);

    let answer = check_guess(500);
    println!("answer was: {}", answer);
}
```

We'll see output like this:

```text
answer was: false
answer was: true

thread '<main>' panicked at 'assertion failed: number < 100', <anon>:2
```

First, `5` was okay, but false. Then, `34` was okay, but true. Finally, `500`
caused a panic.

Panics cause your program to stop executing. To check this, we could move the
failing case above the good cases:

```rust,should_panic
# fn check_guess(number: u32) -> bool {
#     assert!(number < 100);
# 
#     number == 34
# }
# 
fn main() {
    let answer = check_guess(500);
    println!("answer was: {}", answer);

    let answer = check_guess(5);
    println!("answer was: {}", answer);

    let answer = check_guess(34);
    println!("answer was: {}", answer);
}
```

If we run it, we'll see that we never check `5` or `34`:

```text
thread '<main>' panicked at 'assertion failed: number < 100', <anon>:2
```

This is why we call `panic!` an unrecoverable error: the other code never gets a
chance to run. Our program just ends.

But what does it mean to "end a program"? As it turns out, there are multiple
strategies for processing an unrecoverable error. The two main ones are
'unwinding' and 'aborting'.

## Unwinding

By default, when a `panic!` happens in Rust, it starts doing something called
"unwinding". To explain unwinding, let's consider a slightly more complex
program:

```rust,should_panic
fn step1() {
    let s = String::from("Step 1");
    step2();
}

fn step2() {
    let s = String::from("Step 2");
    step3();
}

fn step3() {
    let s = String::from("Step 3");
    check_guess(500);
}

fn check_guess(number: u32) -> bool {
    assert!(number < 100);

    number == 34
}

fn main() {
    step1();
}
```

Here, we have four functions total: `step1` calls `step2` which calls `step3`
which then calls `check_guess`. Something like this diagram, with each of the
variable bindings written in:

```text
> main
  |
  > step1 String: "Step 1"
    |
    > step2 String: "Step 2"
      |
      > step3 String: "Step 3"
        |
        > check_guess: u32: 500
```

When `check_guess` causes a `panic!` via `assert!`, it will walk back through
each of these functions, and clean them up. We haven't yet talked about
destructors in Rust, that'll come in Chapter XX. For now, think about it this
way: simple values like that `u32` can be destroyed by freeing their memory.
More complicated values, like `String`s, have more complicated needs. In these
cases, there is a function that does this cleanup. We call this a "drop
function" in Rust.

So, the first thing that will happen is that `check_guess`, our current
function, gets cleaned up. There's only one value, the `500`, and so its memory
will be freed. Now our program looks like this:

```text
> main
  |
  > step1 String: "Step 1"
    |
    > step2 String: "Step 2"
      |
      > step3 String: "Step 3"
```

Now we're on `step3`. In the same way, Rust will call the `String`'s drop
function, deallocating the `String`. Once that's done, we can move on:

```text
> main
  |
  > step1 String: "Step 1"
    |
    > step2 String: "Step 2"
```

The pattern continues: `step2` has a single value, and `"Step 2"` has its
drop function called. Next!

```text
> main
  |
  > step1 String: "Step 1"
```

Almost done! `step1` also has a `String`, so Rust will invoke its drop function.

```text
> main
```

Finally, all we have left is `main()`. It has no variable bindings or arguments
in this case, so there's nothing to clean up. Our whole program has been delt
with, and so terminates execution, after printing a message.

## Aborting

Doing all that is a lot of work! And the end result is that our program
terminates. Handing panics with unwinding is useful in many scenarios, but some
applications would rather skip straight to the end, and 'abort'. With some
configuration, Cargo will allow us to use this alternate implementation of
`panic!`. What happens when that's enabled? Let's consider what our call stack
looked like:

```text
> main
  |
  > step1 String: "Step 1"
    |
    > step2 String: "Step 2"
      |
      > step3 String: "Step 3"
        |
        > check_guess: u32: 500
```

With an abort implementation of `panic!`, instead of walking back up through the
previous functions and cleaning everything up, we skip straight to the end: our
program terminates. But what about our resources? In the case of memory, like is
the case with `String`s, the operating system will reclaim the memory. So for
this program, the two cases are identical, but aborting is much more efficient.
Other, more complex resources work differently, however, and so aborting may not
always be the right choice either. It depends!
