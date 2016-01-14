# Up and Running

We’ll start our journey with Rust by talking about the absolute basics —
concepts that appear in almost every programming language. Many programming
languages have lots in common at their core. None of the concepts presented are
unique to Rust, but we’ll cover Rust’s particular syntax and conventions around
these common concepts.

If you want to skip this section, you can, but you may end up coming back later
to find out small details. These foundations will be in every single useful
Rust program, and learning them gives us a strong core to start from.
# Variable Bindings

The foundation of virtually every program is the ability to store and modify
data. Rust programs are no different. Let’s start with a short example.

## The basics of bindings

First, we’ll generate a new project with Cargo. Open a terminal, and navigate
to the directory where you’d like to keep your projects. From there, let’s
generate a new project:

```bash
$ cargo new --bin bindings
$ cd bindings
```

This creates a new project, ‘bindings’, and sets up our `Cargo.toml` and
`src/main.rs` files. As we saw in “Hello, World!”, Cargo will generate these
files and create a little ‘hello world’ program for us:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let’s replace that program with this one:

```rust
fn main() {
    let x = 5;

    println!("The value of x is: {}", x);
}
```

And finally, run it:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
```

If you see an error instead, double check that you have copied the program
exactly as written. Let’s break this down, line by line.

```rust,ignore
fn main() {
```

The `main()` function is the entry point of every Rust program. We’ll talk more
about functions in the next section, but for now, all we need to know is that
this is where our program begins. The opening curly brace, `{`, indicates the
start of the function’s body.

```rust,ignore
    let x = 5;
```

This is our first ‘variable binding’, which we create with a ‘`let` statement’.

This `let` statement has this form:

```text
let NAME = EXPRESSION;
```

A `let` statement first evaluates the `EXPRESSION`, and then binds the
resulting value to `NAME` so that it can be referred to later in the program.
In our simple example, the expression was already a value, 5, but we could
achieve the same effect with:

```rust
let x = 2 + 3;
```

In general, `let` statements work with patterns; a name is a particularly
humble form of pattern. Patterns are a big part of Rust, we’ll see more complex
and powerful patterns as we go along.

Before we do that, though, let’s finish investigating this example. Here’s the
next line:

```rust,ignore
    println!("The value of x is: {}", x);
```

The `println!` macro prints text to the screen. We can tell that it’s a macro
due to the `!`. We won’t learn how to write macros until much later in the
book, but we’ll use macros provided by the standard library throughout. Every
time you see a `!`, remember that it signifies a macro. Macros can add new
syntax to the language, and the `!` is a reminder that things may look slightly
unusual.

`println!`, specifically, has one required argument, a ‘format string’, and
zero or more optional arguments. The format string can contain the special text
`{}`. Each instance of `{}` corresponds to an additional argument. Here’s an
example:

```rust
let x = 2 + 3;
let y = x + 5;
println!("The value of x is {}, and the value of y is {}", x, y);
```

You can think of `{}` as little crab pincers, holding the value in place. This
placeholder has a number of more advanced formatting options that we’ll discuss
later.

```rust,ignore
}
```

Finally, a closing curly brace matches up with the opening curly brace that
declared the `main()` function, and declares its end.

This explains our output:

```text
The value of x is: 5
```

We assign `5` to a binding, `x`, and then print it to the screen with
`println!`.

## Multiple binding

Let’s try a more complex pattern. Change our example program to this:

```rust
fn main() {
    let (x, y) = (5, 6);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

And run it with `cargo run`:

```text
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
The value of y is: 6
```

We’ve created two bindings with one `let`! Here’s our pattern:

```text
(x, y)
```

And here’s the value:

```text
(5, 6)
```

As you can see, the two line up visually, and so `let` binds `5` to `x` and `6`
to `y`. We could have used two `let` statements as well:

```rust
fn main() {
    let x = 5;
    let y = 6;
}
```

In simple cases like this, two `let`s may be clearer, but in others, creating
multiple bindings at once is nice. As we become more proficient in Rust, we’ll
figure out which style is better, but it’s mostly a judgement call.

## Type annotations

You may have noticed that we didn’t declare the type of `x` or `y` in our
previous examples. Rust is a *statically typed* language, which means that at
compile time, we must know the types of all bindings. But annotating every
single binding with a type can feel like busywork, and make code noisy. To
solve this issue, Rust uses ‘type inference’, meaning that it attempts to infer
the types of your bindings.

The primary way that the type is inferred is by looking at how it is used.
Let’s look at the example again:

```rust
fn main() {
    let x = 5;
}
```

When we bind `x` to `5`, the compiler knows that `x` should be a numeric type.
Without any other information, it defaults to `i32`, a thirty-two bit integer
type. We’ll talk more about Rust’s basic types in section 3.3.

Here’s what a `let` statement with a ‘type annotation’ looks like:

```rust
fn main() {
    let x: i32 = 5;
}
```

We can add a colon, followed by the type name. Here’s the structure of a `let`
statement with a type annotation:

```text
let PATTERN: TYPE = VALUE;
```

Note that the colon and the `TYPE` go _after_ the `PATTERN`, not in the pattern
itself. As an example, here’s our more complex pattern with two bindings:

```rust
fn main() {
    let (x, y): (i32, i32) = (5, 6);
}
```

Just like we match up the `VALUE` with the `PATTERN`, we match up the `TYPE`
with the `PATTERN`.

## Delayed Initialization

We do not have to provide bindings with an initial value, and can assign it
later. Try this program:

```rust
fn main() {
    let x;

    x = 5;

    println!("The value of x is: {}", x);
}
```

And run it with `cargo run`:

```text
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
```

It’s all good. This raises a question, though: what if we try to print out a
binding before we declare a value? Here’s a program that demonstrates this
question:

```rust,ignore
fn main() {
    let x;

    println!("The value of x is: {}", x);

    x = 5;
}
```

We can find out the answer with `cargo run`:

```text
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:4:39: 4:40 error: use of possibly uninitialized variable: `x` [E0381]
src/main.rs:4     println!(“The value of x is: {}”, x);
                                                    ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:4:5: 4:42 note: in this expansion of println! (defined in <std macros>)
src/main.rs:4:39: 4:40 help: run `rustc --explain E0381` to see a detailed explanation
error: aborting due to previous error
Could not compile `bindings`.

To learn more, run the command again with --verbose.
```

An error! The compiler won’t let us write a program like this. This is our
first example of the compiler helping us find an error in our program.
Different programming languages have different ways of approaching this
problem. Some languages always initialize values with some sort of default.
Other languages leave the value uninitialized, and make no promises about what
happens if you try to use something before initialization. Rust chooses
something else: error and force the programmer to explain what they want. We
must do some sort of initialization before we can use `x`.

### Extended error explanations

There’s one more interesting part of this error message:

```text
src/main.rs:4:39: 4:40 help: run `rustc --explain E0381` to see a detailed explanation
```

We can see an extended explanation by passing the `--explain` flag to `rustc`.
Not every error has a longer explanation, but many of them do. These extended
explanations try to show off common ways that the error occurs, and common
solutions to the issue. Here’s `E0381`:

```bash
$ rustc --explain E0381
It is not allowed to use or capture an uninitialized variable. For example:

fn main() {
    let x: i32;
    let y = x; // error, use of possibly uninitialized variable

To fix this, ensure that any declared variables are initialized before being
used.
```

These explanations can really help if you’re stuck on an error. The compiler is
your friend, and is here to help.

## Mutable bindings

What about changing the value of a binding? Here’s another sample program that
asks this question:

```rust,ignore
fn main() {
    let x = 5;

    x = 6;

    println!("The value of x is: {}", x);
}
```

`cargo run` has the answer for us:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:4:5: 4:10 error: re-assignment of immutable variable `x` [E0384]
src/main.rs:4     x = 6;
                  ^~~~~
src/main.rs:4:5: 4:10 help: run `rustc --explain E0384` to see a detailed explanation
src/main.rs:2:9: 2:10 note: prior assignment occurs here
src/main.rs:2     let x = 5;
                      ^
```

The error mentions `re-assigment of immutable variable`. That’s right: bindings
are immutable. But they’re only immutable by default. In a pattern, when we’re
creating a new name, we can add `mut` in front to make the binding a mutable
one. Here’s an example:

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

We can now change the value that `x` binds to. Note that the syntax is not `let
mut` exactly; it’s using `mut` in a pattern. This becomes more obvious with our
`()` pattern:


```rust,ignore
fn main() {
    let (mut x, y) = (5, 6);

    x = 7;
    y = 8;
}
```

The compiler will complain about this program:

```bash
$ cargo build
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:5:5: 5:10 error: re-assignment of immutable variable `y` [E0384]
src/main.rs:5     y = 8;
                  ^~~~~
src/main.rs:5:5: 5:10 help: run `rustc --explain E0384` to see a detailed explanation
src/main.rs:2:17: 2:18 note: prior assignment occurs here
src/main.rs:2     let (mut x, y) = (5, 6);
                              ^
```

It’s fine with re-assigning `x`, but not `y`. The `mut` only applies to the
name that follows it, not the whole pattern.

### Reassignment, not mutation

There is one subtlety we haven’t covered yet: `mut` allows you to mutate _the
binding_, but not _what the binding binds to_. In other words:

```rust
fn main() {
    let mut x = 5;

    x = 6;
}
```

This is not changing the value that `x` is bound to, but creating a new value,
`6`, and changing the binding to bind to it instead. It’s a subtle but
important difference. Well, for now, it does not make a lot of difference, but
when our programs get more complex, it will. Specifically, passing arguments to
functions will illustrate the difference. We’ll talk about that in the next
section, when we discuss functions.

## Scope

Variable bindings have a ‘scope’ in which they’re valid. That scope begins from
the point at which the binding is declared, and ends at the end of the next
block of code. We can only access bindings which are ‘in scope’. We cannot
access them ‘before they come into scope’ or ‘after they go out of scope’.
Here’s an example:

```rust
fn main() {
    println!("x is not yet in scope");

    let x = 5;
    println!("x is now in scope");

    println!("In real code, we’d now do a bunch of work."); 
    
    println!("x will go out of scope now! The next curly brace is ending the main function.");
}
```

We can create arbitrary scopes through the use of `{` and `}`:

```rust
fn main() {
    println!("x is not yet in scope");

    let x = 5;
    println!("x is now in scope");

    println!("Let’s start a new scope!");

    {
        let y = 5;
        println!("y is now in scope");
        println!("x is also still in scope");

        println!("y will go out of scope now!");
        println!("The next curly brace is ending the scope we started.");
    }

    println!("x is still in scope, but y is now out of scope and is not usable");
    
    println!("x will go out of scope now! The next curly brace is ending the main function.");
}
```

What bindings are in and out of scope will become much more important later,
once we learn about ‘references’ and ‘traits’.

## Shadowing

A final thing about bindings: they can ‘shadow’ previous bindings with the same
name. Here’s a sample program:

```rust
fn main() {
    let x = 5;
    let x = 6;

    println!("The value of x is: {}", x);
}  
```

Running it, we can see the shadowing in action:

```text
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
src/main.rs:2     let x = 5;
                      ^
     Running `target/debug/bindings`
The value of x is: 6
```

There are two interesting things in this output. First, Rust will compile and
run this program, no problem. And as we can see, the value of `x` is `6`. But
we didn’t declare `x` as mutable. Instead, we declared a _new_ binding that is
_also_ named `x`, and gave it a new value. The older value that we bound `x` to
is inaccessible as soon as the new `x` is declared. This can be useful if you’d
like to perform a few transformations on a value, and leave it immutable. For
example:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

This will print:

```bash
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

This lets us modify `x`, but not deal with mutation. This is nice because we
know that the compiler will let us know if we try to modify it later. Let’s
assume that after we calculate `12`, we don’t want to modify `x` again. If we
had written this program in a mutable style, like this:

```rust
fn main() {
    let mut x = 5;
    x = x + 1;
    x = x * 2;

    println!("The value of x is: {}", x);

    x = 15;

    println!("The value of x is: {}", x);
}
```

Rust is happy to let us mutate it again, to `15`. A similar program in our
immutable style will let us know about that accidental mutation, however:

```rust,ignore
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    x = 15;

    println!("The value of x is: {}", x);
}
```

If we try to compile, we get an error:

```bash
$ cargo build
   Compiling bindings v0.1.0 (file:///projects/bindings)
src/main.rs:8:5: 8:11 error: re-assignment of immutable variable `x` [E0384]
src/main.rs:8     x = 15;
                  ^~~~~~
src/main.rs:8:5: 8:11 help: run `rustc --explain E0384` to see a detailed explanation
src/main.rs:4:9: 4:10 note: prior assignment occurs here
src/main.rs:4     let x = x * 2;
                      ^
error: aborting due to previous error
Could not compile `bindings`.
```

Exactly what we wanted.

Shadowing can take some time to get used to, but it’s very powerful, and works
well with immutability.

There was one more thing we should talk about in the output from compiling our
initial program. It’s this part:

```text
src/main.rs:2:9: 2:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
```

Here’s the two lines of relevant code:

```rust
let x = 5;
let x = 6;
```

Rust knows that we shadowed `x`, but we never ended up using the initial value.
This isn’t _wrong_, exactly, it just may not have been what we wanted. In this
case, the compiler issues a ‘warning’, but still compiles our program. The
`#[warn(unused_variables)]` syntax is called an ‘attribute’, which we’ll
discuss in a later section. More specifically, a warning like this is called a
‘lint’, which is an old term for the bits of sheep’s wool that you wouldn’t
want to put in cloth. Similarly, this lint is telling us that we may have an
extra bit of code we don’t need. Our program would work just fine without it.
It’s worth listening to these warnings, and fixing the problems they point out.
They can be signs of a larger problem. In this case, we may not have realized
that we were shadowing `x`.

### Shadowing and scopes

Like any binding, a binding that shadows another binding will go away at the
end of a scope. Here’s an example program:

```rust
fn main() {
    let x = 5;

    println!("Before shadowing, x is: {}", x);

    {
        let x = 6;

        println!("Now that x is shadowed, x is: {}", x);
    }

    println!("After shadowing, x is: {}", x);
}
```

If we run this example, we can see the shadow appear and disappear:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
Before shadowing, x is: 5
Now that x is shadowed, x is: 6
After shadowing, x is: 5
```
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
# Scalar Types

We’ve seen that every value in Rust has a type of some kind. There are a number
of types which are built into the language itself. First, we’ll take a look at
‘scalar’ types, that is, types which represent a single value.

Remember, you can rely on type inference to figure out the type of a binding,
or you can annotate it explicitly:

```rust
fn main() {
    let x: i32 = 5;
}
```

## Integers

You’ve already seen one primitive type: `i32`. There are a number of built-in
number types in Rust.

Here’s a chart of Rust’s integer types:

|        | signed | unsigned |
|--------|--------|----------|
|  8-bit |  i8    |  u8      |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

We have both signed and unsigned variants of numbers, and each variant has an
explicit size. Unsigned numbers are never negative, and signed numbers can be
positive or negative. (Think ‘plus sign’ or ‘minus sign’: that’s a signed
number.) Signed numbers are stored using ‘two’s complement’ representation.

Finally, `isize` and `usize` are different sizes based on the kind of computer
your program is running on. If you are on a 64-bit architecture, they are 64
bits, and if you’re on a 32-bit one, they’re 32 bits.

So how do you choose from all these options? Well, if you really don’t know,
the defaults are a good choice: integer types default to `i32`. The primary use
case for `isize`/`usize` is when indexing some sort of collection. We’ll talk
more about our first collection, arrays, in just a moment.

## Floating-point numbers

Rust also has two primitive floating-point numbers: `f32` and `f64`. They are
32 bits and 64 bits in size, respectively. The default is `f64`.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the IEEE-754 standard.
`f32` is a single-precision float, `f64` is double-precision.

## Numeric operations

Rust supports the usual operations you’d expect on all of these number types:

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // modulus
    let remainder = 43 % 5;
}
```

## Booleans

Somewhat fundamental to all computing, Rust has a boolean type, `bool`, with
two possible values:

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explict type annotation
}
```

The main way to consume boolean values is through conditionals like `if`, which
we’ll see later in the chapter.

## Characters

We’ve only worked with numbers so far, but what about letters? Rust’s most
primitive alphabetic type is the `char`:

```rust
fn main() {
   let c = 'z';
   let z = 'ℤ';
}
```

Rust’s `char` represents a [Unicode Scalar Value], which means that it can
represent a lot more than just ASCII. ‘Character’ isn’t really a concept in
Unicode, however: your human intutition for what a ‘character’ is may not match
up with a `char`. It also means that `char`s are four bytes each.

[Unicode Scalar Value]: http://www.unicode.org/glossary/#unicode_scalar_value
# Compound Types

Now that we’ve discussed scalar types, let’s talk about compound types.
These types can group multiple values of scalar types into another type.

## Tuples

We’ve seen tuples before, in the guise of binding or returning multiple values
at once. It turns out that there’s no magic here: tuples are a general way of
making a compound value that groups some number of other values with distinct
types. The number of values grouped is the ‘arity’ of the tuple.

We create a tuple by writing a comma-separated list of values inside
parentheses; each position in the tuple has a distinct type:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Note that, unlike the examples of multiple bindings, here we bound the
single name `tup` to the entire tuple. We can then use pattern
matching to destructure this tuple value:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

Tuples are used sparingly in Rust code. This is because the elements of a tuple
are anonymous, which can make code hard to read.

### Tuple indexing

In addition to destructuring through pattern matching, we can also access a
tuple element directly using `.`, followed by the index we want to access:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

As you can see, the first index is `0`.

### Single-element tuples

There’s one last trick with tuples: `(5)` is actually ambiguous: is it a tuple,
or is it a `5` in parethesis? If you need to disambiguate, use a comma:

```rust
fn main() {
    let x = (5); // x is an i32, no tuple. Think of it like (5 + 1) without the + 1, they’re for grouping.

    let x = (5,); // x is a (i32), a tuple with one element.
}
```

## Arrays

So far, we’ve only represented single values in a binding. Sometimes, though,
it’s useful to have more than one value. These kinds of data structures are
called ‘collections’, and arrays are the ones we’ll learn about first. Arrays
look like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

An array’s type consists of the type of the elements it contains, as well as
the length:

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

An array is a single chunk of memory, allocated on the stack.

We can access elements of an array using indexing:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, `first` will hold the value `1`, and `second` will be bound to
`2`. Note that these values are copied out of the array; if the array changes,
these bindings will not. Here’s an example, which also shows us how we can
modify elements of the array:

```rust
fn main() {
    let mut a = [1, 2, 3, 4, 5];

    let first = a[0];
    
    a[0] = 7;

    println!("The value of first is: {}", first);
}
```

Running this example will show that `first` is still `1`. If we didn’t want a
copy, but instead wanted to refer to the first element, whatever its value was,
we need a new concept. We’ll talk about ‘references’ in Section 4.

One last thing: now that we are modifying the array, `a` needs to be declared
`mut`.

Arrays are our first real data structure, and so there’s a few other concepts
that we haven’t covered in full yet. There are two: the `panic!` macro, and a
new way of printing things: `Debug`.

### Panic

We showed what happens when you access elements of an array, but what if we
give an invalid index?

```rust,should_panic
fn main() {
    let a = [1, 2, 3, 4, 5];

    let invalid = a[10];

    println!("The value of invalid is: {}", invalid);
}
```

If we run this example, we will get an error. Let’s re-use our `functions`
project from before. Change your `src/main.rs` to look like the example, and
run it:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
thread ‘<main>’ panicked at ‘index out of bounds: the len is 5 but the index is 10’, src/main.rs:4
Process didn’t exit successfully: `target/debug/functions` (exit code: 101)
```

It says that our thread panicked, and that our program didn’t exit
successfully. There’s also a reason: we had a length of five, but an index of
10.

For now, all you need to know is that a panic will crash your program. Rust’s
error handling story is described in full in a later chapter.

So why did this code panic? Well, arrays know how many elements they hold. When
we access an element via indexing, Rust will check that the index is less than
the length. If it’s greater, it will panic, as something is very wrong. This is
our first example of Rust’s safety principles in action. In many low-level
languages, this kind of check is not done. If you have an incorrect index,
invalid memory can be accessed. Rust protects us against this kind of error.

### Debug

So far, we’ve been printing values using `{}`. If we try that with an array,
though...

```rust,ignore
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is: {}", a);
}
```

... we will get an error:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
src/main.rs:4:25: 4:26 error: the trait `core::fmt::Display` is not implemented for the type `[_; 5]` [E0277]
src/main.rs:4     println!(“a is {}”, a);
                                      ^
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
src/main.rs:4:5: 4:28 note: in this expansion of println! (defined in <std macros>)
src/main.rs:4:25: 4:26 help: run `rustc --explain E0277` to see a detailed explanation
src/main.rs:4:25: 4:26 note: `[_; 5]` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
src/main.rs:4:25: 4:26 note: required by `core::fmt::Display::fmt`
error: aborting due to previous error
```

Whew! The core of the error is this part: the trait `core::fmt::Display` is not
implemented. We haven’t discussed traits yet, so this is bound to be confusing!
Here’s all we need to know for now: `println!` can do many kinds of formatting.
By default, `{}` implements a kind of formatting known as `Display`: output
intended for direct end-user consumption. The primitive types we’ve seen so far
implement `Display`, as there’s only one way you’d show a `1` to a user. But
with arrays, the output is less clear. Do you want commas or not? What about
the `[]`s?

Due to these questions, more complex types in the standard library do not
implement `Display` formatting. There is another kind of formatting, `Debug`,
which is a bit different: intended for programmer consumption. We can ask
`println!` to use `Debug` formatting with `:?`:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("a is {:?}", a);
}
```

This will work:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
a is [1, 2, 3, 4, 5]
```

You’ll see this repeated later, with other types. And we’ll cover traits fully
later in the book, Section 9.
# Comments

We strive to make our programs easy to understand, but sometimes, some extra
explanation is warranted. We can leave notes in our source code that the
compiler will ignore. These notes are called ‘comments’.

Here’s a comment:

```rust
// Hello, world.
```

Comments start with two slashes, and last until the end of the line. Larger
comments will need more lines:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments can also go at the end of lines:

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today.
}
```

But you’ll more often see them above:

```rust
fn main() {
    // I’m feeling lucky today.
    let lucky_number = 7;
}
```

That’s all there is to it. Comments are not particularly complicated.

## Documentation comments

However, Rust has another kind of comment: a documentation comment. These
comments don’t affect the way that the code works, but they do work with Rust’s
tools. More specifically, the `rustdoc` tool that comes with Rust reads
documentation comments and produces HTML documentation from them.

Documentation comments use an extra slash:

```rust
/// The foo function doesn’t really do much.
fn foo() {
}

/// We also can use
/// multiple comments here too,
/// like we did before
fn bar() {
}
```

This comment would then be interpreted by `rustdoc` as documenting the thing
that follows it: `foo()` and `bar()`.

Because documentation comments have semantic meaning to `rustdoc`, the compiler
will pay attention to the placement of your documentation comments. For
example, a program with only this:

```rust,ignore
/// What am I documenting?
```

Will give a compiler error:

```text
src/main.rs:1:1: 1:27 error: expected item after doc comment
src/main.rs:1 /// What am I documenting?
              ^~~~~~~~~~~~~~~~~~~~~~~~~~
```
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
# Loops

It’s often quite useful to be able to execute a block of code more than one
time. For this, we have several constructs, called ‘loops’.

To try out loops, let’s make a new project. Navigate to your projects folder
and use Cargo to make a new one:

```bash
$ cargo new --bin loops
$ cd loops
```

There are three kinds of loops in Rust: `loop`, `while`, and `for`. Let’s dig
in.

## `loop`

The `loop` keyword is very straightforward: it executes a block of code over
and over and over and over and over and over forever. Change your `src/main.rs`
file to look like this:

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

If we run this program, we’ll see ‘`again!`’ printed over and over again. So
how does our program end? It doesn’t, until we kill it. Most terminals support
a keyboard shortcut, ‘control-c’, to stop a runaway program. Give it a try:

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

That `^C` there is where I hit control-c.

That’s a lot of trouble though! Luckily, there’s a way to break an infinite `loop`.

### Breaking out of a loop

The `break` keyword will allow us to quit looping. Try this version out:

```rust
fn main() {
    loop {
        println!("once!");
        break;
    }
}
```

If you run this program with `cargo run`, you’ll see that it only executes one
time:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
once!
```

When a Rust program hits a `break` statement, it will exit the current loop.

## `while`

What if we took `loop`, `break`, and `if`, and put them together? Something
like this:

```rust
fn main() {
    let mut number = 3;

    loop {
        if number != 0 {
            println!("{}!", number);

            number = number - 1;
        } else {
            break;
        }

    }

    println!("LIFTOFF!!!");
}
```

If we run this, we’ll get some output:

```bash
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
3!
2!
1!
LIFTOFF!!!
```

The core of this example is in the combination of these three constructs:

```rust,ignore
    loop {
        if number != 0 {
            // do stuff
        } else {
            break;
        }
```

We want to `loop`, but only while some sort of condition is true. As soon as it
isn't, we want to `break` out of the loop.

This pattern is so common that we have a language construct for it: `while`.
Here's the same example, but using `while` instead:

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

This lets us get rid of a lot of nesting, and is more clear: while a condition holds,
run this code.

## `for`

We can use this `while` construct to loop over the elements of a collection, like an
array:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value is is: {}", a[index]);

        index = index + 1;
    }
}
```

Running this will print out every element of the array:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
the value is: 1
the value is: 2
the value is: 3
the value is: 4
the value is: 5
```

Here, we're counting up instead of down: we start at zero, then loop until we
hit the final index of our array.

This approach is error-prone, though. If we get the index length incorrect, we
will end up causing a `panic!`. This is also slow, as the compiler needs to do
that check on every element on every iteration through the loop.

Instead, we can use our last kind of loop: the `for` loop. It looks like this:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

** NOTE: see [https://github.com/rust-lang/rust/issues/25725#issuecomment-166365658](https://github.com/rust-lang/rust/issues/25725#issuecomment-166365658), we may want to change this **

If we run this, we'll see the same output as the previous example. 

** I'm going to leave it at this for now until we decide how we want to do it**
