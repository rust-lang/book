## How Functions Work in Rust

Functions are pervasive in Rust code. We’ve already seen one of the most
important functions in the language: the `main()` function that’s the entry
point of every program. We've also seen the `fn` keyword, which allows us to
declare new functions.

Rust code uses *snake case* as the conventional style for function names. In
snake case, all letters are lower case, and there are underscores separating
words. (Rust also uses snake case for the names of variable bindings; we just
haven't used any variable bindings long enough to need underscores yet.) Here's
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

Function definitions in Rust always start with `fn` <!-- Nitpick: this isn't
entirely true, what about `pub fn whatever`? Or is the `pub` not considered to
be "part" of the function definition...? I guess this isn't super important.
/Carol --> and have a set of parentheses after the function name. The curly
braces tell the compiler where the function body begins and ends.

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
the `let` bindings we used earlier. Just look at both together, and compare
them:

<!--
Whoa, hold on... the earlier section on variable bindings didn't explain
optional type annotations at all. There is one occurrence of a `let` type
annotation, but it's in the error explanation output and isn't explained.

I'm guessing the variable binding section used to have that explanation but
doesn't anymore? I'm not sure if you want to add some explanation in that
section or reword this section to not draw comparisons between `let` bindings
and function signatures...
/Carol
-->

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

In this example, we make a function with two arguments. In this case, both are
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
containing the values, and passes those bindings instead. When you run this,
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

<!--Au/TR - if we don't want to move this section earlier in the chapter, when
we first talk about statements and expressions, could you add some text just
explaining that we've already seen statements and expressions, but that until
now we haven't had the motivation to use expressions on their own, etc /Liz -->
<!-- I gave it a shot but Steve should take a look. I'm not clear from the
comment if this section has been moved around or not /Carol -->

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
src/main.rs:2:14: 2:17 error: expected identifier, found keyword `let`
src/main.rs:2     let x = (let y = 6);
                           ^~~
src/main.rs:2:18: 2:19 error: expected one of `!`, `)`, `,`, `.`, `::`, `{`, or an operator, found `y`
src/main.rs:2     let x = (let y = 6);
                               ^
Could not compile `functions`.
```
<!--TR: Why didn't this work? Could you please suggest an explanation /Liz -->

<!--
The error message I get when I run this on the beta channel is different and
clearer:
https://play.rust-lang.org/?code=fn%20main%28%29%20{%0A%20%20%20%20let%20x%20%3D%20%28let%20y%20%3D%206%29%3B%0A}%0A&version=beta&backtrace=0

```bash
<anon>:2:14: 2:17 error: expected expression, found statement (`let`)
<anon>:2     let x = (let y = 6);
                      ^~~
note: variable declaration using `let` is a statement
error: aborting due to previous error
```

So perhaps by the time the book is finished, beta will be stable and this should make a bit more sense?

Here's a suggested explanation: /Carol
-->

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

<!-- Commenting these few paragraphs out because of the rearranging I'm doing,
but wanted to leave my comments about them /Carol Statements are instructions.
While expressions _compute_ something, statements perform some action. <!--
Isn't "computing" an "action" though? I'm not sure the previous sentence is a
meaningful distinction. Maybe "While expressions perform actions that compute a
value, statements perform actions that either don't result in values or don't
care about the resulting values." This is hard. /Carol ---- For example, `let`
statements bind variables, and `fn` declarations are statements that begin
functions.

One practical difference between an expression and a statement is that you can
bind an expression, but you can't bind a statement, since a statement doesn't
return a value.

<!--- TR: can you help with an explanation of why you can't bind a statement?
Is it because a statement doesn't evaluate to a value you can bind to? /Liz
---- <!-- Yes, that's pretty much it-- I've added a bit to the previous
sentence. /Carol ---- -->

### Functions with Return Values

<!--- TR: Do we need to put this in context, let the reader know when this
would be useful, or is this something they'll already know? /Liz -->
<!-- I'd expect anyone knowing another programming language to be familiar with
the concept and usefulness of returning values from a function. I checked with
some programming language enthusiasts on twitter and the reader of this book
would pretty much have to ONLY ever have had experience with, like, BASIC or
Forth, which I think would be rare. /Carol -->

Functions can return values back to the code that calls them. In Rust, the
"return value of the function” is synonymous with “the value of the final
expression in the block of the body of a function.” A function that returns a
value looks like this:
<!--Au/TR: I liked your summary of a return value in a comment you'd left, I've
added it here to clarify what we mean by return value, can you please check
this? /Liz -->
<!-- I added the "In Rust"-- I know this is a Rust book so that might be
obvious, but this is different than many other languages that aren't
expression-oriented, so I think it's worth making the distinction for that
statement, whereas "functions can return values" is a statement that applies to
most programming languages. I also reworded the first sentence a bit because it
felt like it was saying function too much. /Carol -->

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
arguments and defines the type of the return, but the body of the function is a
lonely `5` with no semicolon because it is an expression whose value we want to
return. Let's look at another example:

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
src/main.rs:8     x + 1;
src/main.rs:9 }
src/main.rs:7:1: 9:2 help: run `rustc --explain E0269` to see a detailed explanation
src/main.rs:8:10: 8:11 help: consider removing this semicolon:
src/main.rs:8     x + 1;
                       ^
error: aborting due to previous error
Could not compile `functions`.
```

The main error message, "not all control paths return a value," reveals the
core of the issue with this code. The definition of the function `plus_one`
says that it will return an `i32`, but statements don’t evaluate to a value.
Therefore, nothing is returned, which contradicts the function definition and
results in an error. In this output, Rust gives an option to rectify this: it
suggests removing the semicolon, which would fix the error.

#### Returning Multiple Values

By default, functions can only return single values. There’s a trick, however
to get them to return multiple values. Remember how we used `()`s to create
complex bindings in the "Creating Multiple Bindings" section on page XX?

```rust
fn main() {
    let (x, y) = (5, 6);
}
```

Parentheses used in this way form a *tuple*-- a collection of elements that
isn't assigned a name. Tuples are also a basic data type in Rust, and we'll
cover them in detail in the "Tuples" section later in this chapter. For our
purposes now, we can use tuples to return multiple values from functions, as so:

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
in plain English that the return type `(i32, i32)` translates to “a tuple with
two `i32`s in it". These two types are then applied to the tuple to be returned
by the function block. In this case, that tuple contains the values `5` and
`6`. This tuple is returned, and assigned to `x` and `y`.

See how all these bits fit together? We call this behavior of `let`
‘destructuring’, because it takes the structure of the expression that follows
the `=` and takes it apart.

<!--TR: Do you think it's necessary to break this explanation up and repeat
lines, or is the code short enough that we can just explain it all in one
paragraph, with the examples from the code in line? /Liz -->
<!-- I think the code is short enough, so I've removed the broken out code
blocks. /Carol -->

<!-- Commenting out paragraphs that were incorporated in the rearranging I did,
but kept because I have comments in here that might be relevant /Carol

#### Expressions as Return Values

So what does the way statements and expressions work have to do with return
values? Well, the block that we use to create new scopes, `{}`, is an
expression. Let’s take a closer look at `{}` with the following signature:

```text
{
    STATEMENT*
    EXPRESSION
}
```

The `*` by `STATEMENT` indicates "zero or more," meaning we can have any number
of statements inside a block, followed by an expression. Since blocks are
expressions themselves, we can nest blocks inside of blocks.

<!---TR: Does each new expression require a new block, then? /Liz ----
<!--
No-- since blocks ARE expressions, that would mean each new block would require
a new block-- that doesn't make sense. Expressions are everywhere, not just in
blocks-- expressions are part of statements.

In the next example, the `5` in line 3 is an expression that is not in a new
block, it's in a statement. Likewise, the `1` in line 6 is also an expression
in a statement. `x + z + 5` on line 8 is an expression, and the block starting
with the `{` on line 5 and ending with the `}` on line 9 is an expression that
contains a statement (which contains an expression) and an expression, and
whose value is bound to y in that statement. The `println!` line is also a
statement.

I'm... not sure if that clears things up or not!
/Carol

And since blocks return a value, we can use them in `let` statements. For
example:

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

Here, we're using a block to give us a value for the `y` variable. Inside that
block, we create a new variable binding, `z`, with a `let` statement and give
`z` a value. For the final expression of the block, we do some math, ultimately
binding the result to `y`. Since `x` is `5` and `z` is `1`, the calculation is
`5 + 1 + 5`, and so the value of the entire block is `11`. This gets
substituted into our `let` statement for `y`, making that statement equivalent
to:

```rust
let y = 11;
```

Try running the program, and you should see the following output:

```bash
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of y is: 11
```

<!--- TR: Could you help summarize the takeaway from this section, what is it
that we're learning here? That we're using the return value from the block in a
function? Also, is it worth pointing out that the final expression in the
nested blocks lacks a semicolon, and why that works. /Liz ----
<!--
I think the semicolon is explained thoroughly in the "Expression Statements"
section. But I'm starting to feel a bit like the set-up at the end of the
previous section, about why the `5` in the `fn five()` doesn't have a
semicolon, is too far from the payoff in the "Expression Statements" section.

I kind of feel like this is a lot of detail in *why* there's no semicolon, what
an expression is, what a statement is, what things are and are not expressions,
that aren't really necessary this early on in someone's experience with Rust. I
feel like the most important thing for people to know at this point is "if
you're returning a value, you must not put a semicolon, or else your value
won't get returned" (see the "not all control paths return a value" error
explored below) and "If you're not returning a value, you must put a
semicolon". I'm not entirely convinced that spending time naming the concepts
of expression and statement and exploring all the variations of them will aid
understanding at this point, and that it would be more appropriate

As expected, the output string says that `y` is `11`.

/Carol
-->
