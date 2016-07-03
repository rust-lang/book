## Anatomy of a Rust Program

The foundation of virtually every program is the ability to store and modify
data, but to create this data, you first have to create a program. Here, we'll
write some code that demonstrates how to begin a Rust program, how to bind a
variable, and how to print text to the terminal.

### Keywords

<!--Au/TR: We ought to define keywords, and the difference between keywords and
statements, somewhere before we get to the Loops section - can you please add
an explanation? Would here be an appropriate place? /Liz--> <!-- I think the
beginning is as good a place as any to define what a keyword is and mention
that you can't use these words, wdyt? I'm leaving the formatting up to you....
/Carol -->

First, keep in mind that the Rust language has a set of *keywords* that have
been reserved for use by the language only. This means you cannot use these
words as names of variables or functions, for example. Most of these have
special meaning and we will be using them to do various things in our Rust
programs; a few have no current functionality associated but have been reserved
for functionality that might be in the Rust language in the future.

The keywords are:

* `abstract`
* `alignof`
* `as`
* `become`
* `box`
* `break`
* `const`
* `continue`
* `crate`
* `do`
* `else`
* `enum`
* `extern`
* `false`
* `final`
* `fn`
* `for`
* `if`
* `impl`
* `in`
* `let`
* `loop`
* `macro`
* `match`
* `mod`
* `move`
* `mut`
* `offsetof`
* `override`
* `priv`
* `proc`
* `pub`
* `pure`
* `ref`
* `return`
* `Self`
* `self`
* `sizeof`
* `static`
* `struct`
* `super`
* `trait`
* `true`
* `type`
* `typeof`
* `unsafe`
* `unsized`
* `use`
* `virtual`
* `where`
* `while`
* `yield`

### A Simple Program that Binds a Variable

Let’s start with a short example that binds a value to a variable, and then
uses that in a sentence that we'll print to the screen. First, we’ll generate a
new project with Cargo. Open a terminal, and navigate to the directory you want
to store your projects in. From there, generate a new project:

```bash
$ cargo new --bin bindings
$ cd bindings
```

This creates a new project called `bindings` and sets up our *Cargo.toml* and
*src/main.rs* files. As we saw in Chapter XX, Cargo will generate these files
and create a little "hello world" program like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Open *src/main.rs* and replace its code with the following:

```rust
fn main() {
    let x = 5;

    println!("The value of x is: {}", x);
}
```

This is the full program for our example. Enter the `run` command now to to see
it working:

```bash
$ cargo run
   Compiling bindings v0.1.0 (file:///projects/bindings)
     Running `target/debug/bindings`
The value of x is: 5
```

If you get an error instead of this output, double check that you've copied the
program exactly as written, and then try again. Now let’s break this program
down, line by line.

#### Starting a Program with the main() Function

Most Rust programs open with the same first line as this one from our example
program:
<!-- I'm not sure if I would agree with this statement, but my disagreement has
to do with binary vs library programs, and I would understand if this isn't the
right time or place to explain that distinction. It's tough to be correct but
not get bogged down in details early on.

My nitpick is most Rust *binaries* have a main function, and main doesn't have
to be first... Libraries, which might make up the majority of Rust programs out
there on the internet, don't have main functions at all... so this might give
someone the impression that they could open up any Rust program and the main
function would be first, but there might be one that isn't first, or there
might not be one... and they might infer from this that they always have to
have a main function and always have to put it first... /Carol -->

```rust,ignore
fn main() {
```

The `main()` function is the entry point of every Rust program. It doesn’t have
to be at the very beginning of our source code, <!-- Wait, so why did this
section just say most Rust programs have it at the beginning? Now I think the
first statement of this section should be reworded somehow, maybe just to
change "Most" to "Many"? /Carol --> but it will be the first bit of code that
runs when we execute our program. We’ll talk more about functions in the next
section, but for now, just know that ```main()``` is where our program begins.
The opening curly brace (`{`) indicates the start of the function’s body.

#### Binding a Variable with `let`

Inside the function body, we added the following:

```rust,ignore
    let x = 5;
```

This is a `let` statement, and it binds the value `5` to the variable `x`.
Basic `let` statements take the following form:

```text
let NAME = EXPRESSION;
```

A `let` statement first evaluates the `EXPRESSION`, and then binds the
resulting value to `NAME` to give us a variable to use later in the program.
Notice the semicolon at the end of the statement, too. As in many other
programming languages, statements in Rust must end with a semicolon.

In this simple example, the expression already is a value, but we could achieve
the same result like this:

```rust
let x = 2 + 3;
```

The expression `2 + 3` would evaluate to `5`, which would in turn be stored in
the `x` variable binding.

More generally, `let` statements take the form:

```text
let PATTERN = EXPRESSION;
```

*Patterns* are part of the ‘pattern matching’ feature of Rust. If you have
worked with regular expressions, you can think of patterns like a regular
expression that works on values in your program instead of characters in text.
A name like `x` is a particularly humble form of pattern; it will always match
and gets all the parts of the expression as its value. Patterns are a big part
of Rust, and we’ll see more complex and powerful patterns as we go along.
<!-- TR: Can you please help here? We need a clearer definition of a pattern in
rust, we're not really happy with this one as it is /Liz --> <!-- Ok, I tried
to rearrange the previous paragraph a bit and expand on what a pattern is.
Since the reader is expected to have used some other programming language in
the past, and most programming languages support regular expressions, this
definition tries to make an analogy to regular expressions as a touchstone. I'm
not sure if knowledge of regular expressions is too much of an assumption, and
I'm not sure this is an entirely accurate analogy! I think this early, the
point should just be to hint that it's possible to have more complex stuff on
the left side of the equals sign in a `let` statement... This could also be
made clearer with an example, say of a function that returns two values in a
tuple, and you could have:

```rust,ignore
// `some_function` is going to return the tuple `(1, 2)`.
let (a, b) = result_of_function();
// Here, the value of `a` will be `1`, and the value of `b` will be `2`.
// We could also write:
let y = result_of_function();
// And here the value of `y` will be the whole tuple `(1, 2)`.
```

... but then you have to explain what tuples are, so I'm not sure if this is a
great place to get into this.

(later) Ah, now I see the "creating multiple bindings" section, and I think
that's more appropriate than the example I've suggested here. So I would just
have the previous paragraph here, but I'm leaving the rest of my comment in
order to provide you with some options.

/Carol -->

#### Printing to the Screen with a Macro

The next line of our program is:

```rust,ignore
    println!("The value of x is: {}", x);
```

The `println!` command is a *macro* that prints the text passed to it to the
screen. Macros are indicated with the `!` character at the end of their name.
In Chapter <???>, you'll learn more about the details of macros and how to
write macros yourself, but for now we'll just be using macros provided by the
standard Rust library.

Macros can add new syntax to the language to enable convenient code reuse.
Using a macro may look similar to calling a function, but they do have
different capabilities. The `!` is a reminder that calling a macro may look
slightly unusual. For example, the "Hello, world!" program that `cargo new`
generated for us called the `println!` macro with one argument (the string
`"Hello, world!"`). Here, we are calling it with two arguments (the string
`"The value of x is: {}"` and `x`). Functions in Rust must always be called
with the same number of arguments that their definition specifies, but macros
have different rules that allow them to take different numbers of arguments.

<!--TR: Can you help to expand on this explanation of macros---In what way
would things look unusual? How does adding syntax to the langauge affect our
program? If you could suggest some text that would be great! /Liz --> <!--
Above is my attempt to address what is meant by macros looking unusual. As far
as "How does adding syntax to the langauge affect our program?" in some sense
adding a macro *doesn't* affect our program unless we choose to call that
macro... the point of macros though is to have another avenue for reusing code
that would otherwise be repetitive and have all the other drawbacks of
repeating yourself, but in a different way than functions allow for code reuse.
I don't think the explanation needs to be very in-depth at this point though...
/Carol -->

The `println!` macro only requires one argument: a format string. You can add
optional arguments inside this format string by using the special text `{}`.
Each instance of `{}` corresponds to an additional argument. Here’s an example:

```rust
let x = 2 + 3;
let y = x + 5;

println!("The value of x is {}, and the value of y is {}", x, y);
```

If you were to run a program containing these statements, it would print the
following:

```text
The value of x is 5, and the value of y is 10
```

Think of `{}` as little crab pincers, holding a value in place. The first `{}`
holds the first value after the format string, the second set holds the second
value, and so on. The `{}` placeholder has a number of more advanced formatting
options that we’ll discuss later.

After the `println!` macro, we match the opening curly brace that declared the
`main()` function with a closing curly brace to declare the end of the function:

```rust,ignore
}
```

And of course, when we run the program, our output is:

```text
The value of x is: 5
```

With this simple program, you've created your first variable and used your
first Rust macro. That makes you a Rust programmer. Welcome! Now that you've
seen the basics, let's explore variable bindings further.
