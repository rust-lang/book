## Control Flow

Decisions on whether or not to run some code depending on if a condition is
true, or deciding to run some code repeatedly while a condition is true, are
basic building blocks in most programming languages. The most common constructs
that let us control the flow of execution of our Rust code are `if` expressions
and loops.

### `if` Expressions

An `if` expression allows us to branch our code depending on conditions. We
provide a condition and then say, "If this condition is met, run this
block of code. If the condition is not met, do not run this block of code."

Let’s make a new project to explore `if`, called `branches`. In `src/main.rs`,
put:

Filename: src/main.rs

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

All `if` expressions start with the keyword `if`, which is followed by a
condition. In this case, our condition is checking if our variable
`number` has a value that is less than 5. The block of code we want to execute
if the condition is true goes immediately after the condition, inside curly
braces. These blocks are sometimes called *arms*. We can optionally also
include an `else` expression, which we have chosen to do here. This gives the
program an alternative block of code to execute should the condition evaluate
to false. If you don't give an `else` expression and the condition is false,
the program will just skip the `if` block and move on to the next bit of code.

Try running this code, and you should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

Let’s try changing the value of `number` to a value that makes the condition
`false` to see what happens:

```rust,ignore
let number = 7;
```

Run the program again, and look at the output:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

It’s also worth noting that the condition here _must_ be a `bool`. To see what
happens if the condition isn't a `bool`, try running this code:

Filename: src/main.rs

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

The `if` condition evaluates to a value of `3` this time, and Rust will
complain about it:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
error: mismatched types [--explain E0308]
 --> src/main.rs:4:8
4 |>     if number {
  |>        ^^^^^^ expected bool, found integral variable
note: expected type `bool`
note:    found type `_`

error: aborting due to previous error
Could not compile `branches`.
```

The error tells us that Rust expected a `bool`, but got an integer. Rust will
not automatically try to convert non-boolean types to a boolean here, unlike
languages like Ruby or JavaScript. We must be explicit and always give `if` a
`boolean` as its condition. If you wanted your `if` code block to run only when
a number is not equal to `0`, for example, we would change the `if` expression
to read:

Filename: src/main.rs

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Running this will print `number was something other than zero`.

#### Multiple Conditions with `else if`

We can have multiple conditions by combining `if` and `else` in an `else if`
expression. For example:

Filename: src/main.rs

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

This program has four possible paths it can take. If you try running it, you
should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
number is divisible by 3
```

When this program executes, it will check each `if` expression in turn and
execute the first body for which the condition holds true. Note that even
though 6 is divisible by 2, we did not see the output `number is divisible by
2`, nor did we see the `number is not divisible by 4, 3, or 2` text from the
`else` block. That's because Rust will only execute the block for the first
true condition, and once it finds one, it won't even check the rest.

Using too many `else if` expressions can clutter your code, so if you find
yourself with more than one, you may want to look at refactoring your code. In
Chapter 6, we'll talk about a powerful Rust branching construct called `match`
for these cases.

#### Using `if` in a `let` statement

The last detail you need to know about `if` is that it’s an expression. That
means that we can use it on the right hand side of a `let` statement, for
instance:

Filename: src/main.rs

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

The `number` variable will be bound to a value based on the outcome of the `if`
expression. Let’s run this to see what happens:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
The value of number is: 5
```

Remember, blocks of code evaluate to the last expression in them, and numbers
by themselves are also expressions. In this case, the value of the whole `if`
expression depends on which block of code executes. This means the value that
results from both arms of the `if` must be the same type; in the previous
example, they were both `i32` integers. But what happens if the types are
mismatched, as in the following example?

Filename: src/main.rs

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

If we try to run this, we’ll get an error. The `if` and `else` arms have value
types that are incompatible, and Rust tells us exactly where to find the
problem in our program:

```bash
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |     let number = if condition {
  |                  ^ expected integral variable, found reference
  |
  = note: expected type `{integer}`
  = note:    found type `&'static str`

error: aborting due to previous error

error: Could not compile `branches`.
```

The expression in the `if` block evaluates to an integer and the expression in
the `else` block evaluates to a string. This can’t work, because variables
must have a single type. Rust needs to know at compile time what type
the `number` variable is, definitively, so that it can verify at compile time
that its type is valid everywhere we use `number`. Rust wouldn't be able to do
that if the type of `number` was only determined at runtime; the compiler would
be more complex and be able to make fewer guarantees about our code if it had
to keep track of multiple hypothetical types for any variable.

### Repetition with Loops

It’s often useful to be able to execute a block of code more than one time. For
this, Rust has several *loops*. A loop runs through the code inside the loop
body to the end and then starts immediately back at the beginning. To try out
loops, let’s make a new project called `loops`.

There are three kinds of loops in Rust: `loop`, `while`, and `for`. Let’s dig
in.

#### Repeating Code with `loop`

The `loop` keyword tells Rust to execute a block of code over and over again
forever or until we explicitly tell it to stop.

For an example, change the `src/main.rs` file in your *loops* directory to look
like this:

Filename: src/main.rs

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

If we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support a keyboard shortcut,
`control-c`, to halt a program stuck in a continual loop. Give it a try:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

That `^C` there is where we hit `control-c`. You may or may not see "again!"
printed after the `^C`, depending on where the code was in the loop when it
received the signal to halt.

Fortunately, Rust provides another, more reliable way to break out of a loop.
We can place the `break` keyword within the loop to tell the program when to
stop executing the loop. Recall that we did this in the guessing game in
Chapter 2 to exit the program when the user won the game by guessing the number
correctly.

#### Conditional Loops With `while`

It's often useful for a program to have a condition that can be evaluated
within a loop. While the condition is true, the loop runs. When the condition
ceases to be true, we call `break`, stopping the loop. This could be
implemented with a combination of `loop`, `if`, `else`, and `break`; you could
try that now in a program, if you'd like.

But this pattern is so common that Rust has a more efficient language construct
for it, called a `while` loop. Here's an example using `while`: this program
loops three times, counting down each time. Finally, after the loop, it prints
another message, then exits:

Filename: src/main.rs

```rust
fn main() {
    let mut number = 3;

    while number != 0  {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

This gets rid of a lot of nesting that would be necessary if we used `loop`,
`if`, `else`, and `break`, and it's more clear. While a condition holds, run
this code; otherwise, exit the loop.

#### Looping Through a Collection with `for`

We could use this `while` construct to loop over the elements of a collection,
like an array. For example:

Filename: src/main.rs

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

Here, we're counting up through the elements in the array. We start at index 0,
then loop until we hit the final index of our array (that is, when `index < 5`
is no longer true). Running this will print out every element of the array:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `6` at some point, the loop stops executing before trying
to fetch a sixth value from the array.

This approach is error-prone, though; we could cause our program to panic by
getting the index length incorrect. It's also slow, as the compiler needs to
perform the conditional check on every element on every iteration through the
loop.

As a more efficient alternative, we can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like this:

Filename: src/main.rs

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

If we run this, we'll see the same output as the previous example. Importantly,
though, we've now increased the safety of our code and eliminated the chance of
bugs that might result from going beyond the end of the array or not going far
enough and missing some items.

For example, in the previous code that uses the `while` loop, if we removed an
item from the `a` array but forgot to update the condition to `while index <
4`, our code would panic. Using the `for` loop means we would not need to
remember to change any other code if we changed the number of values in the
array.

If you're wondering about the `iter` code in this example, keep reading! We
will cover method syntax generally in Chapter XX and iterators specifically in
Chapter XX.

The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations where you want to run some code a certain
number of times, like our countdown example that used a `while` loop, most
Rustaceans would use a `for` loop. The way to do that would be to use a
`Range`, which is a type provided by the standard library that generates all
numbers in sequence starting from one number and ending before another number.

Here's what the countdown would look like with a for loop, and using another
method we haven't yet talked about, `rev`, to reverse the range:

Filename: src/main.rs

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

That's a bit nicer, isn't it?

## Summary

You made it! That was a big chapter: we covered variables, scalar and
compound data types, functions, comments, `if` expressions, and loops! If you'd
like to get some practice with the concepts in this chapter, try building
programs to do things like:

* Converting temperatures between Fahrenheit and Celsius.
* Generate the nth Fibonacci number.
* Print the lyrics to the Christmas carol *The Twelve Days of Christmas*,
  taking advantage of the repetition in the song.

When you're ready to move on, we'll talk about a concept in Rust that _doesn't_
commonly exist in other programming languages: ownership.
