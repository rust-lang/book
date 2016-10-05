## How Functions Work

Functions are pervasive in Rust code. We’ve already seen one of the most
important functions in the language: the `main` function that’s the entry
point of many programs. We've also seen the `fn` keyword, which allows us to
declare new functions.

Rust code uses *snake case* as the conventional style for function and variable
names. In snake case, all letters are lower case, and there are underscores
separating words. Here's a program containing an example function definition:

Filename: src/main.rs

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Function definitions in Rust start with `fn` and have a set of parentheses
after the function name. The curly braces tell the compiler where the function
body begins and ends.

We can call any function we’ve defined by entering its name followed by a pair
of parentheses. Since `another_function` is defined in the program, it can be
called from inside the `main` function. Note that we defined
`another_function` _after_ the `main` function in our source code; we could
have defined it before as well. Rust doesn’t care where you define your
functions, only that they are defined somewhere.

Let’s start a new binary project named `functions` so that we can explore
further. Place the `another_function` example in `src/main.rs` and run it.
You should see the following output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order they appear in the `main` function. First, our
“Hello, world!” message prints, and then `another_function` is called and its
message is printed.

### Function Arguments

Functions can also take arguments. The following rewritten version of
`another_function` shows what arguments look like in Rust:

Filename: src/main.rs

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Try running this program, and you should get this output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

In the declaration of `another_function`, we place one argument named `x`. We
specify the type of `x` as `i32`. When we pass `5` to `another_function`,
the `println!` macro puts `5` where the pair of curly braces were in the format
string.

In function signatures, we _must_ declare the type. This is a deliberate
decision in the design of Rust; requiring type annotations in function
definitions means the compiler almost never needs you to use them elsewhere in
the code in order to figure out what you mean.

When you want a function to have multiple arguments, just separate them inside
the function signature with commas, like this:

Filename: src/main.rs

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

In this example, we make a function with two arguments, both of which are
`i32`s. If your function has multiple arguments, they don’t need to be the same
type, but they just happen to be in this example. Our function then prints out
the values of both of its arguments.

Let’s try out this code. Replace the program currently in your `function`
project's `main.rs` file with the example above, and run it as follows:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Since `5` is passed as the `x` argument and `6` is passed as the `y` argument,
the two strings are printed with these values.

### Function Bodies

Function bodies are made up of a series of statements optionally ending in an
expression. So far, we've only seen functions without an ending expression, but
we have seen expressions as parts of statements. Since Rust is an
expression-based language, this is an important distinction to understand.
Other languages don't have the same distinctions, so let's look at what
statements and expressions are and how their differences affect the bodies of
functions.

#### Statements and Expressions

We've already been using both statements and expressions. *Statements* are
instructions that perform some action and do not return a value. *Expressions*
evaluate to a resulting value. Let's look at some examples.

Creating a variable and assigning a value to it with the `let` keyword
is a statement. In this example, `let y = 6;` is a statement:

Filename: src/main.rs

```rust
fn main() {
    let y = 6;
}
```

Function definitions are also statements; the entire previous example is a
statement in itself.

Statements do not return values themselves. Therefore, you can’t assign a `let`
statement to another variable, as this code tries to do:

Filename: src/main.rs

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

If we were to run this program, we’d get an error like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
2 |>     let x = (let y = 6);
  |>              ^^^
note: variable declaration using `let` is a statement

error: aborting due to previous error
error: Could not compile `functions`.
```

The `let y = 6` statement does not return a value, so there isn't anything for
`x` to bind to. This is different than in other languages like C and Ruby where
the assignment returns the value of the assignment. In those languages, we can
write `x = y = 6` and have both `x` and `y` have the value `6`; that is not the
case in Rust.

Expressions are code that evaluate to something, and make up most of the rest
of the code that you will write in Rust. Consider a simple math operation, like
this:

```rust,ignore
5 + 6
```

This is an expression, and evaluating it results in the value `11`. Expressions
can be part of statements-- in the previous example that had the statement `let
y = 6;`, `6` is an expression that evaluates to the value `6`. Calling a
function is an expression. Calling a macro is an expression. The block that we
use to create new scopes, `{}`, is an expression, for example:

Filename: src/main.rs

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
<!-- If we use wingding numbers to call out code, we might delete the
repetition here and just use those numbers--that can help the flow of the text.
I'm flagging this as a reminder for when we transfer to libreoffice -->

The expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to `4`, and then gets assigned to
`y` as part of the `let` statement.

Note that the line containing `x + 1` does not have a semicolon at the end,
unlike most of the lines we've seen up until now. This is the most important
distinction between expressions and statements to remember: statements end in
semicolons while expressions do not. If you add a semicolon to the end of an
expression, that will turn it into a statement, which will then not return a
value. Keep this in mind as we explore function return values and expressions.

### Functions with Return Values

Functions can return values back to the code that calls them. We don’t name
return values, but we do declare their type, after an arrow (`->`). In Rust,
the "return value of the function” is synonymous with the "value of the final
expression in the block of the body of a function.” Here's an example of a
function that returns a value:

Filename: src/main.rs

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, or even `let` statements in the `five`
function: just the number `5` by itself. That's a perfectly valid function in
Rust. Note the function's return type is specified, too, as `-> i32`. Try
running this code, and the output should look like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

The `5` in `five` is the function's return value, which is why the return type
is `i32`. Let’s examine this in more detail. There are two important bits.
First, the line `let x = five();` shows us using the return value of a function
to initialize a variable.

Because the function `five` returns a `5`, that line is the same as saying:

```rust
let x = 5;
```

The second interesting bit is the `five` function itself. It requires no
arguments and defines the type of the return value, but the body of the
function is a lonely `5` with no semicolon because it is an expression whose
value we want to return. Let's look at another example:

Filename: src/main.rs

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this code will print `The value of x is: 6`. What happens if we put a
semicolon at the end of the line containing `x + 1`, changing it from an
expression to a statement?

Filename: src/main.rs

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Running this code gives an error, as follows:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: not all control paths return a value [--explain E0269]
 --> src/main.rs:7:1
7 |> fn plus_one(x: i32) -> i32 {
  |> ^
help: consider removing this semicolon:
 --> src/main.rs:8:10
8 |>     x + 1;
  |>          ^

error: aborting due to previous error
error: Could not compile `functions`.
```

The main error message, "not all control paths return a value", reveals the
core of the issue with this code. The definition of the function `plus_one`
says that it will return an `i32`, but statements don’t evaluate to a value.
Therefore, nothing is returned, which contradicts the function definition and
results in an error. In this output, Rust gives an option to rectify this: it
suggests removing the semicolon, which would fix the error.
