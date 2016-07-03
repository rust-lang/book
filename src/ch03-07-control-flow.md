## Control Flow

<!--Au: Can you please add a general intro for control flow and the following
if and loops sections here? /Liz -->

### `if` Statements

> Two roads diverged in a yellow wood,
> And sorry I could not travel both
> And be one traveler, long I stood
> And looked down one as far as I could
> To where it bent in the undergrowth;
>
> - Robert Frost, “The Road Not Taken”

In Rust, as in most programming languages, an `if` expression allows us to
branch our code depending on conditions. We provide a condition, and then say,
`if` this condition is met, then run this block of code. If the condition is
not met, do not run this block of code.

Let’s make a new project to explore `if`. Navigate to your projects directory,
and use Cargo to make a new project called `branches`:

```bash
$ cargo new --bin branches
$ cd branches
```

Write this sample program using `if` and save it in the *branches* directory in
`src/main.rs`:

```rust
fn main() {
    let condition = true;

    if condition {
        println!("condition was true");
    }
}
```

The `condition` variable is a boolean; here, it's set to true. All `if`
statements start with `if`, which is followed by a condition. The block of code
we want to execute if the condition is true goes immediately after the
condition, inside curly braces. These blocks are sometimes called ‘arms’.

Try running this code, and you should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

Let’s try changing the value of `condition` to `false` as follows to see what
happens:

```rust
    let condition = false;
```

Run the program again, and look at the output:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
```

Nothing was output, because the condition was false and the `if` block was not
run.

We can optionally also include an `else` statement, which gives the program a
block of code to execute should `condition` evaluate to false.

```rust
fn main() {
    let condition = false;

    if condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

If we run this program, the output will look like:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

This time, because `condition` was false and we have an `else` block, the
`else` block was executed.

<!-- TR: Do these examples make sense this way round, or would it be clearer to
give the example using the if statement alone, and then the example giving the
if and else statements? /Liz -->
<!-- I think having just `if` first, then `if`+`else` would be better... this
section also reads pretty repetitively to me, which I think would be great if
the target audience was people who hadn't programmed before, but I think people
who have used at least one other programming language should be pretty familiar
with the concept of how an `if` statement works and just need to know the
particulars of the syntax in Rust. I've given rearranging this section a try,
see what you think! /Carol -->

It’s also worth noting that `condition` here _must_ be a `bool`. To see what
happens if the condition isn't a `bool`, try running this code:

```rust,ignore
fn main() {
    let condition = 5;

    if condition {
        println!("condition was five");
    }
}
```

The `condition` variable is assigned a value of `5` this time, and Rust will
complain about it:

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

The error tells us that Rust expected a `bool`, but got an integer. Rust will
not automatically try to convert non-boolean types to a boolean here, unlike
languages like Ruby or JavaScript. We must be explicit and always give `if` a
`boolean` as its condition.

<!--TR: What must we be explicit about? /Liz -->
<!-- Expanded a bit /Carol -->

#### Multiple Conditions with `else if`

We can have multiple coniditions by combining `if` and `else` in an `else if`
expression. For example:

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

This program has four possible paths it can take. If you try running it, you
should see output like this:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was 5
```

When this program executes, it will check each `if` expression in turn and
execute the first body for which the condition holds true.

Using too many `else if` expressions can clutter your code, so if you find
yourself with more than one, you may want to look at refactoring your code. In
Chapter XX, we'll talk about a powerful Rust branching construct called `match`
for these cases.

#### Using `if` in a Binding

The last detail you need to learn about `if` is that it’s an expression. That
means that we can use it on the right hand side of a `let` binding, for
instance:

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
expression depends on which block of code executes. This means that the value
in both arms of the `if` must be the same type; in the previous example, they
were both `i32` integers. But what happens if the types are mismatched, as in
the following example?

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

The expression in one block of the `if` statement, is an integer and the
expresion in the other block is a string. If we try to run this, we’ll get an
error:

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

The `if` and `else` arms have value types that are incompatible, and Rust tells
us exactly where to find the problem in our program. This can’t work, because
variable bindings must have a single type.

### Repetition with Loops

It’s often useful to be able to execute a block of code more than one time. For
this, Rust has several constructs called *loops*. A loop runs through the code
inside it to the end and then starts immediately back at the beginning. To try
out loops, let’s make a new project. Navigate to your *projects* folder and use
Cargo to make a new project:

```bash
$ cargo new --bin loops
$ cd loops
```

There are three kinds of loops in Rust: `loop`, `while`, and `for`. Let’s dig
in.

#### Repeating Code with `loop`

<!--TR: By "keyword" do we just mean a statement or function embedded in the
Rust language, that can't be changed? /Liz -->
<!-- Yup, see the keyword section I added at the beginning. /Carol -->

The `loop` keyword tells Rust to execute a block of code over and over again
forever, or until we explicitly tell it to stop.

For an example, change the *src/main.rs* file in your *loops* directory to look
like this:

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
<!-- TR: Is this correct, it will print "again" one last time after hitting
ctrl c? /Liz -->
<!-- Sometimes it will-- it depends on where in the execution the program is at
the exact instant you hit control-c. /Carol -->

That `^C` there is where I hit `control-c`. Fortunately, Rust provides another,
more reliable way to break out of a loop. We can place the `break` keyword
within the loop to tell the program when to stop executing the loop. Try this
version of the program out:

```rust
fn main() {
    loop {
        println!("once!");
        break;
    }
}
```

If you run this program, you’ll see that it only executes one time:

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
once!
```

When a Rust program hits a `break` statement, it will exit the current loop.
This on its own is not very useful; if we wanted to print somtheing just once,
we wouldn't put it in a loop. This is where conditions come in again.

#### Conditional Loops With `while`

To make `break` useful, we need to give our program a condition. While the
condition is true, the loop runs. When the condition ceases to be true, the
`break` code runs, stopping the loop.

Try this example:

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

If we run this, we’ll get:

```bash
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
3!
2!
1!
LIFTOFF!!!
```

This program loops three times, counting down each time. Finally, after the
loop, it prints another message, then exits.

The core of this example is in the combination of `loop`, `if`, `else`, and
`break`.

<!--```rust,ignore
    loop {
        if number != 0 {
            // do stuff
        } else {
            break;
        }
```-->
<!--TR/au: I'm not sure we need to repeat this so close to its first appearance
/Liz -->
<!-- Agreed /Carol -->

We want to `loop`, but only while some sort of condition is true. As soon as it
isn't, we want to `break` out of the loop. This pattern is so common that Rust
has a more efficient language construct for it, called a `while` loop. Here's
the same example, but using `while` instead:

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

This gets rid of a lot of nesting, and it's more clear. While a condition
holds, run this code; otherwise, do nothing.

#### Looping Though a Collection with `for`

We can use this `while` construct to loop over the elements of a collection,
like an array. For example:

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

<!-- I changed the numbers in the array to be 10, etc just to avoid any
possible confusion with 0-based and 1-based array indexing that some people
might have. /Carol -->

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

As a more efficient alternative, we can use a `for` loop. A `for` loop looks
something like this:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

** NOTE: see [https://github.com/rust-lang/rust/issues/25725#issuecomment-166365658](https://github.com/rust-lang/rust/issues/25725#issuecomment-166365658), we may want to change this **


If we run this, we'll see the same output as the previous example.

<!--- Perhaps explain the difference, and how this handles the same operations
differently to while -->

<!-- Steve - so this needs wrapping up, and the chapter needs a general summary
of some sort. Does this need to wait for long? -->

** I'm going to leave it at this for now until we decide how we want to do it**

<!-- Au/TR Has this issue been resolved? We'll need a chapter wrap up, do we
need to come back to this at a later date? /Liz -->

<!-- The issue has not been resolved... but I think it's ok to gloss over the
details a bit, or point to the chapters where method syntax or vecs will be
discussed? There are other issues that people will run into pretty quickly if
they try to actually use arrays at this point, such as the fact that they have
to be a fixed size known at compile time, so it's not like someone would be
able to put down the book at this point and be productive in Rust. /Carol -->
