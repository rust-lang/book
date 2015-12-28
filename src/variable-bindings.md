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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
     Running `target/debug/bindings`
The value of x is: 12
```

This lets us modify `x`, but not deal with mutation. This is nice because we
know that the compiler will let us know if we try to modify it later. Let’s
assume that after we calculate `12`, we don’t want to modify `x` again. If we
had written this program in a mutable style, like this:

```
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
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
   Compiling bindings v0.1.0 (file:///home/steve/tmp/bindings)
     Running `target/debug/bindings`
Before shadowing, x is: 5
Now that x is shadowed, x is: 6
After shadowing, x is: 5
```
