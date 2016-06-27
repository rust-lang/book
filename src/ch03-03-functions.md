# Functions

Functions are pervasive in Rust code. We’ve already seen the most important
function, `main()`, in previous sections of the book:

```rust
fn main() {
    println!("Hello, world!");
}
```

We can declare new functions with the `fn` keyword:

```rust
fn another_function() {
    println!("Another function.");
}
```

Rust code uses `snake_case` as a style for function names: all lower case, with
underscores separating words. (It also uses them for variable names, too.) We
can can call any function we’ve defined by using its name and some parentheses:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Let’s start a new project to explore functions. Open a terminal, and navigate
to the directory where you’d like to keep your projects. From there, use Cargo
to generate a new project:

```bash
$ cargo new --bin functions
$ cd functions
```

Place the new example in `src/main.rs`, and run it:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

As we can see, the lines execute in order: first, we print out our “Hello,
world!” message, and then `another_function()` is called. It then prints its
message as well.

## Function Arguments

Functions can also take arguments:

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Let’s try running it:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

Let’s take a closer look at `another_function()`’s signature:

```rust,ignore
fn another_function(x: i32) {
```

Declaring a function which takes a single argument looks like this:

```text
fn NAME(PATTERN: TYPE) {
```

That’s right, patterns appear again. Consider how the parameter declaration
here looks like the `let` bindings we used earlier:

```rust,ignore
let x: i32;
fn another_function(x: i32) {
```

There’s only one difference here: in function signatures, we _must_ declare the
type. This is a deliberate decision; we find that requiring type annotations in
functions means that you almost never need them anywhere else.

You can separate multiple arguments with a comma:

```text
fn NAME(PATTERN, PATTERN, PATTERN, PATTERN...) {
```

Here’s a full example:

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

Let’s try it:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

We could also create bindings, and pass them in as arguments:

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

This has the same effect:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

Note that our bindings are called `a` and `b`, yet inside of the function, we
refer to them by the names in the signature, `x` and `y`. Inside a function,
only its parameters are in scope, so we need to use those names. Bindings
passed as parameters don’t need to have the same name as the arguments.

## Return values

Functions can also return values back to the function that called them:

```TEXT
fn NAME(PATTERN, PATTERN, PATTERN, PATTERN...) -> TYPE {
```

We don’t name return values, but we do declare their type, after an arrow:
`->`. Here’s a sample program:

```rust
fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
```

Let’s try running it:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

Let’s examine this in more detail. There are two important bits. First, we can
use the return value of a function to initialize a binding:

```rust,ignore
let x = five();
```

Because `five()` returns a `5`, this is the same as:

```rust
let x = 5;
```

The second interesting bit is `five()` itself:

```rust
fn five() -> i32 {
    5
}
```

We have no arguments, and our return type, `i32`. However, the body of this
function is a lonely `5`. There’s a detail here that you may or may not have
noticed: we’ve ended almost every line in our programs with a semicolon.
There’s no semicolon here, though. Why not?

The answer to this question is:

> The return value of a function is the value of its final expression.

We haven’t talked about expressions yet, so this definition doesn’t help a lot.
Let’s go over that now.

## Statements and Expressions

Expressions are bits of code that evaluate to a value. Consider some math
operations, like this:

```rust,ignore
5 + 6
```

We can evaluate this expression, and come up with a value: `11`. In Rust, most
bits of code are expressions. For example, calling a function is an expression:

```rust,ignore
foo(5)
```

The value is equal to whatever the return value of `foo()` is.

So why does this matter? Well, not everything is an expression. Some things are
‘statements’. Expressions _compute_ something, but statements _bind_ or _do_
something. It’s a subtle difference. We’ve already seen two kinds of
statements: `let` statements, and `fn` declarations.

Because `let` is a statement, not an expression, you can’t assign it to another
binding. Here’s an example that doesn’t work:

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

If we try to run this program, we’ll get an error:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:2:14: 2:17 error: expected identifier, found keyword `let`
src/main.rs:2     let x = (let y = 6);
                           ^~~
src/main.rs:2:18: 2:19 error: expected one of `!`, `)`, `,`, `.`, `::`, `{`, or an operator, found `y`
src/main.rs:2     let x = (let y = 6);
                               ^
Could not compile `functions`.
```

We also cannot somehow assign a `fn` declaration to a binding, either.

So what’s this have to do with return values? Well, `{}`, a ‘block’ that we
used earlier to create new scopes, _is_ an expression. Let’s take a closer look
at `{}`. It looks like this:

```text
{
    STATEMENT*
    EXPRESSION
}
```

The `*` there means ‘zero or more’, so we can have any number of statements
followed by an expression. Since blocks are expressions themselves, we can nest
blocks inside of blocks. And since they return a value, we can use them in
`let` statements:

```rust
fn main() {
    let x = 5;

    let y = {
        let z = 1;

        x + z + 5
    };

    println!("The value of y is: {}", y);
}
```

Let’s try running this program:

```bash
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of y is: 11
```

We’re now using a block to give us a value for `y`:

```rust,ignore
let y = {

};
```

Since the block can contain statements, we create a new variable binding, `z`,
and give it a value. We then do some math for the final expression of the
block:

```rust,ignore
{
    let z = 1;

    x + z + 5
}
```

`5 + 1 + 5` is `11`, and so the value of the entire block is `11`. This gets
substituted into our `let` statement for `y`:

```rust,ignore
let y = 11;
```

Hence our output saying `y` is `11`.

Where else do we use blocks? As the body of functions! They’re very similar:

```rust
fn main() {
    let x = 5;

    let y = {
        x + 1
    };

    println!("The value of y is: {}", y);

    let y = plus_one(x);

    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this gives:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of y is: 6
The value of y is: 6
```

In both cases, we use a block to produce a value. In the first case, it’s
assigning with `let`:

```rust,ignore
let y = {
```

In the second, it’s the return value of the function:

```rust,ignore
fn plus_one(x: i32) -> i32 {
```

### Expression statements

There’s one more detail about expressions and statements: a semicolon takes any
expression, and turns it into a statement. Let’s accidentally cause an error
with `plus_one()`:

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Instead of an expression, `x + 1`, we’ve now turned it into a statement,
`x + 1;`.

Running this gives an error:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:7:1: 9:2 error: not all control paths return a value [E0269]
src/main.rs:7 fn plus_one(x: i32) -> i32 {
src/main.rs:8     x + 1;
src/main.rs:9 }
src/main.rs:7:1: 9:2 help: run `rustc --explain E0269` to see a detailed explanation
src/main.rs:8:10: 8:11 help: consider removing this semicolon:
src/main.rs:8     x + 1;
                       ^
error: aborting due to previous error
Could not compile `functions`.
```

Rust has our back here: it even suggests removing the semicolon, which fixes
the error. But the main error message is the core of the issue: statements
don’t evaluate to a value, yet we want to return an `i32`.

In practice, Rust programmers don’t often think about these rules at this
level. Usually, you have a semicolon at the end of most lines, and maybe not at
the end of blocks.

## Multiple return values

Functions cannot directly return multiple values. There’s a trick, however.
Remember the `()`s we used when showing off complex bindings?

```rust
fn main() {
    let (x, y) = (5, 6);
}
```

They form something called a ‘tuple’, one of Rust’s basic types. A tuple is an
anonymous collection of elements. But since a tuple is a singular thing, we can
use it as a way to return multiple values from functions:

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

Running this will show us the values:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

There are two interesting changes here: assigning the return value of
`two_numbers()` to `x` and `y`, and the declaration of `two_numbers()` itself.

Let’s look at the declaration first:

```rust
fn two_numbers() -> (i32, i32) {
    (5, 6)
}
```

The `(i32, i32)` should look familiar. We saw it in `let` bindings earlier:

```rust
let (x, y): (i32, i32) = (5, 6);
```

The `(i32, i32)` syntax says “a tuple with two `i32`s in it.” The `(5, 6)`
syntax creates a new one, with `5` and `6`.

This tuple is then returned, and assigned to `x` and `y`:

```rust,ignore
let (x, y) = two_numbers();
```

See how all these bits fit together?

We call this behavior of `let` ‘destructuring’, because it takes the structure
of the expression that comes after the `=` and takes it apart.
