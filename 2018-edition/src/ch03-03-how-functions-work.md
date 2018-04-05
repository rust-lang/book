## Functions

Functions are pervasive in Rust code. You’ve already seen one of the most
important functions in the language: the `main` function, which is the entry
point of many programs. You’ve also seen the `fn` keyword, which allows you to
declare new functions.

Rust code uses *snake case* as the conventional style for function and variable
names. In snake case, all letters are lowercase and underscores separate words.
Here’s a program that contains an example function definition:

<span class="filename">Filename: src/main.rs</span>

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
after the function name. The curly brackets tell the compiler where the
function body begins and ends.

We can call any function we’ve defined by entering its name followed by a set
of parentheses. Because `another_function` is defined in the program, it can be
called from inside the `main` function. Note that we defined `another_function`
*after* the `main` function in the source code; we could have defined it before
as well. Rust doesn’t care where you define your functions, only that they’re
defined somewhere.

Let’s start a new binary project named *functions* to explore functions
further. Place the `another_function` example in *src/main.rs* and run it. You
should see the following output:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

The lines execute in the order in which they appear in the `main` function.
First, the “Hello, world!” message prints, and then `another_function` is
called and its message is printed.

### Function Parameters

Functions can also be defined to have *parameters*, which are special variables
that are part of a function’s signature. When a function has parameters, you
can provide it with concrete values for those parameters. Technically, the
concrete values are called *arguments*, but in casual conversation, people tend
to use the words *parameter* and *argument* interchangeably for either the
variables in a function’s definition or the concrete values passed in when you
call a function.

The following rewritten version of `another_function` shows what parameters
look like in Rust:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Try running this program; you should get the following output:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
     Running `target/debug/functions`
The value of x is: 5
```

The declaration of `another_function` has one parameter named `x`. The type of
`x` is specified as `i32`. When `5` is passed to `another_function`, the
`println!` macro puts `5` where the pair of curly brackets were in the format
string.

In function signatures, you *must* declare the type of each parameter. This is
a deliberate decision in Rust’s design: requiring type annotations in function
definitions means the compiler almost never needs you to use them elsewhere in
the code to figure out what you mean.

When you want a function to have multiple parameters, separate the parameter
declarations with commas, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

This example creates a function with two parameters, both of which are `i32`
types. The function then prints the values in both of its parameters. Note that
function parameters don’t all need to be the same type, they just happen to be
in this example.

Let’s try running this code. Replace the program currently in your *functions*
project’s *src/main.rs* file with the preceding example and run it using `cargo
run`:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Because we called the function with `5` as the value for  `x` and `6` is passed
as the value for `y`, the two strings are printed with these values.

### Function Bodies

Function bodies are made up of a series of statements optionally ending in an
expression. So far, we’ve only covered functions without an ending expression,
but you have seen an expression as part of statements. Because Rust is an
expression-based language, this is an important distinction to understand.
Other languages don’t have the same distinctions, so let’s look at what
statements and expressions are and how their differences affect the bodies of
functions.

### Statements and Expressions

We’ve actually already used statements and expressions. *Statements* are
instructions that perform some action and do not return a value. *Expressions*
evaluate to a resulting value. Let’s look at some examples.

Creating a variable and assigning a value to it with the `let` keyword is a
statement. In Listing 3-1, `let y = 6;` is a statement:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let y = 6;
}
```

<span class="caption">Listing 3-1: A `main` function declaration containing one statement</span>

Function definitions are also statements; the entire preceding example is a
statement in itself.

Statements do not return values. Therefore, you can’t assign a `let` statement
to another variable, as the following code tries to do; you’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

When you run this program, the error you’ll get looks like this:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

The `let y = 6` statement does not return a value, so there isn’t anything for
`x` to bind to. This is different from what happens in other languages, such as
C and Ruby, where the assignment returns the value of the assignment. In those
languages, you can write `x = y = 6` and have both `x` and `y` have the value
`6`; that is not the case in Rust.

Expressions evaluate to something and make up most of the rest of the code that
you’ll write in Rust. Consider a simple math operation, such as `5 + 6`, which
is an expression that evaluates to the value `11`. Expressions can be part of
statements: in Listing 3-1, the `6` in the statement `let y = 6;` is an
expression that evaluates to the value `6`. Calling a function is an
expression. Calling a macro is an expression. The block that we use to create
new scopes, `{}`, is an expression, for example:

<span class="filename">Filename: src/main.rs</span>

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

This expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to `4`. That value gets bound to `y`
as part of the `let` statement. Note the `x + 1` line without a semicolon at
the end, which is unlike most of the lines you’ve seen so far. Expressions do
not include ending semicolons. If you add a semicolon to the end of an
expression, you turn it into a statement, which will then not return a value.
Keep this in mind as you explore function return values and expressions next.

### Functions with Return Values

Functions can return values to the code that calls them. We don’t name return
values, but we do declare their type after an arrow (`->`). In Rust, the return
value of the function is synonymous with the value of the final expression in
the block of the body of a function. You can return early from a function by
using the `return` keyword and specifying a value, but most functions return
the last expression implicitly. Here’s an example of a function that returns a
value:

<span class="filename">Filename: src/main.rs</span>

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
function—just the number `5` by itself. That’s a perfectly valid function in
Rust. Note that the function’s return type is specified, too, as `-> i32`. Try
running this code; the output should look like this:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
The value of x is: 5
```

The `5` in `five` is the function’s return value, which is why the return type
is `i32`. Let’s examine this in more detail. There are two important bits:
first, the line `let x = five();` shows that we’re using the return value of a
function to initialize a variable. Because the function `five` returns a `5`,
that line is the same as the following:

```rust
let x = 5;
```

Second, the `five` function has no parameters and defines the type of the
return value, but the body of the function is a lonely `5` with no semicolon
because it’s an expression whose value we want to return.

Let’s look at another example:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this code will print `The value of x is: 6`. But if we place a
semicolon at the end of the line containing `x + 1`, changing it from an
expression to a statement, we’ll get an error.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Running this code produces an error, as follows:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
  | |          - help: consider removing this semicolon
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
```

The main error message, “mismatched types,” reveals the core issue with this
code. The definition of the function `plus_one` says that it will return an
`i32`, but statements don’t evaluate to a value, which is expressed by `()`,
the empty tuple. Therefore, nothing is returned, which contradicts the function
definition and results in an error. In this output, Rust provides a message to
possibly help rectify this issue: it suggests removing the semicolon, which
would fix the error.
