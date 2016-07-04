## How Functions Work in Rust

Functions are pervasive in Rust code. We’ve already seen one of the most
important functions in the language: the `main()` function that’s the entry
point of many programs. We've also seen the `fn` keyword, which allows us to
declare new functions.

Rust code uses *snake case* as the conventional style for function names. In
snake case, all letters are lower case, and there are underscores separating
words. (Rust also uses snake case for the names of variable bindings; we just
haven't used any variable bindings long enough to need underscores yet). Here's
a program containing an example function definition:

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
of parentheses. Since `another_function()` is defined in the program, it can be
called from inside the `main()` function. Note that we defined
`another_function()` _after_ the `main()` function in our source code; we could
have defined it before as well. Rust doesn’t care where you define your
functions, only that they are defined somewhere.

Let’s start a new project to explore functions further. Open a terminal, and
navigate to the directory you're keeping your projects in. From there, use
Cargo to generate a new project, as follows:

```bash
$ cargo new --bin functions
$ cd functions
```

Place the `another_function()` example in a file named *src/main.rs* and run
it. You should see the following output:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order they appear in the `main()` function. First, our
“Hello, world!” message prints, and then `another_function()` is called and its
message is printed.

### Function Arguments

Functions can also take arguments. The following rewritten version of
`another_function()` shows what arguments look like in Rust:

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

Since we passed `5` to `another_function()`, the `println!` macro put `5` where
the pair of curly braces were in the format string.

Let’s take a closer look at the signature of a function which takes a single
argument:

```text
fn NAME(PATTERN: TYPE) {
```

The parameter declaration in a single-argument function signature looks like
the `let` bindings we used earlier in the "Type Inference and Annotation"
section. Just look at both together, and compare them:

```rust,ignore
let x: i32;
fn another_function(x: i32) {
```

The one difference is that in function signatures, we _must_ declare the type.
This is a deliberate decision in the design of Rust; requiring type annotations
in function definitions means the compiler almost never needs you to use them
elsewhere in the code in order to figure out what you mean.

When you want a function to have multiple arguments, just separate them inside
the function signature with commas, like this:

```text
fn NAME(PATTERN: TYPE, PATTERN: TYPE, PATTERN: TYPE, PATTERN: TYPE...) {
```

And just like a `let` declaration with multiple patterns, a type must be
applied to each pattern separately. To demonstrate, here’s a full example of a
function with multiple arguments:

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

### Variable Bindings as Arguments

It's also possible to create bindings and pass them in as arguments in Rust.
For example:

```rust
fn main() {
    let a = 5;
    let b = 6;

    another_function(a, b);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

Instead of passing `5` and `6` directly, this first creates two bindings
containing the values and passes those bindings instead. When you run this,
you'll find that it has the same effect as just using integers:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Note that our bindings are called `a` and `b`, yet inside the function, we
refer to them by the names in the signature, `x` and `y`. Inside a function,
its parameters are in scope but the names of the bindings we passed as
parameters are not, so we need to use the parameter names within the function
block. Bindings passed as parameters don’t need to have the same names as the
arguments.

### Function Bodies

Function bodies are made up of a series of statements ending in an optional
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

`Let` bindings are statements. They instruct the program to create a binding
name and assign a value to it. `let y = 6;` in this example is a statement:

```rust
fn main() {
    let y = 6;
}
```

Function definitions are also statements-- so the entire previous example is a
statement as well.

Statements do not return values themselves. Therefore, you can’t assign a `let`
binding to another binding, as this code tries to do:

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

If we were to run this program, we’d get an error like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:2:14: 2:17 error: expected expression, found statement (`let`)
src/main.rs:2     let x = (let y = 6);
                           ^~~
src/main.rs:2:14: 2:17 note: variable declaration using `let` is a statement
error: aborting due to previous error
error: Could not compile `functions`.
```

The `let y = 6` statement does not return a value, so there isn't anything for
`x` to bind to. This is different than in other languages like C and Ruby where
the assignment returns the value of the assignment. In those languages, you
could write `x = y = 6` and have both `x` and `y` have the value `6`, but that
is not the case in Rust.

Expressions are most of the rest of the code that you will write in Rust.
Consider a simple math operation, like this:

```rust,ignore
5 + 6
```

This is an expression, and evaluating it results in the value `11`. Expressions
can be part of statements-- in the previous example that had the statement `let
y = 6;`, `6` is an expression that evaluates to the value `6`. Calling a
function is an expression. Calling a macro is an expression. The block that we
use to create new scopes, `{}`, is an expression, for example:

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

The expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, gets evaluated to `4`, which then gets bound to
`y` as part of the `let` statement.

Note that the line containing `x + 1` does not have a semicolon at the end like
most of the lines we've seen up until now have had. This is the most important
distinction between expressions and statements to remember: statements end in
semicolons while expressions do not. If you add a semicolon to the end of an
expression, that will turn it into a statement, which will then not return a
value. Keep this in mind as we explore function return values and expressions.

### Functions with Return Values

Functions can return values back to the code that calls them. In Rust, the
"return value of the function” is synonymous with “the value of the final
expression in the block of the body of a function.” A function that returns a
value looks like this:

```text
fn NAME(PATTERN: TYPE, PATTERN: TYPE, PATTERN: TYPE, PATTERN: TYPE...) -> TYPE {
    STATEMENT*
    EXPRESSION
}
```

The `*` by `STATEMENT` indicates "zero or more", meaning we can have any number
of statements inside the function body block, ending with an expression since
we are returning a value.

In Rust, we don’t name return values, but we do declare their type, after an
arrow (`->`). Here’s a sample program to illustrate this concept:

```rust
fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
```

There are no function calls, macros, or even `let` statements in the `five()`
function-- just the number `5` by itself. That's a perfectly valid function in
Rust. Note the function's return type, too. Try running this code, and the
output should look like this:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

The `5` in `five()` is actually the function's return value, which is why the
return type is `i32`. Let’s examine this in more detail. There are two
important bits. First, the line `let x = five();` in `main()` shows that we can
use the return value of a function to initialize a binding.

Because the function `five()` returns a `5`, that line is the same as saying:

```rust
let x = 5;
```

The second interesting bit is the `five()` function itself. It requires no
arguments and defines the type of the return value, but the body of the
function is a lonely `5` with no semicolon because it is an expression whose
value we want to return. Let's look at another example:

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
src/main.rs:7:1: 9:2 error: not all control paths return a value [E0269]
src/main.rs:7 fn plus_one(x: i32) -> i32 {
              ^
src/main.rs:7:1: 9:2 help: run `rustc --explain E0269` to see a detailed explanation
src/main.rs:8:10: 8:11 help: consider removing this semicolon:
src/main.rs:8     x + 1;
                       ^
error: aborting due to previous error
error: Could not compile `functions`.
```

The main error message, "not all control paths return a value", reveals the
core of the issue with this code. The definition of the function `plus_one`
says that it will return an `i32`, but statements don’t evaluate to a value.
Therefore, nothing is returned, which contradicts the function definition and
results in an error. In this output, Rust gives an option to rectify this: it
suggests removing the semicolon, which would fix the error.

#### Returning Multiple Values

By default, functions can only return single values. There’s a trick, however,
to get them to return multiple values: group them into a tuple!

```rust
fn main() {
    let (x, y) = two_numbers();

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn two_numbers() -> (i32, i32) {
    (5, 6)
}
```

Running this will give us the values:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Let's look at this more closely. First, we're assigning the return value of
calling `two_numbers()` to `x` and `y`. In the function signature, we would say
in plain English that the return type `(i32, i32)` translates to "a tuple with
two `i32`s in it". These two types are then applied to the tuple to be returned
by the function block. In this case, that tuple contains the values `5` and
`6`. This tuple is returned, and we destructure the tuple and assign the individual values to `x` and `y`.
