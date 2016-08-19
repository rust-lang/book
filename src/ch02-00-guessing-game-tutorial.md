# Guessing Game

Let's jump into Rust with a hands-on project! This chapter will introduce you to
a few common Rust concepts by showing how you would use them in a real program.
You'll learn about `let`, `match`, methods, associated functions, using
external crates, and more! Following chapters will explore these ideas in more
detail.

We’re going to implement a classic beginner programming problem: the guessing
game. Here’s how it works: Our program will generate a random integer between
one and a hundred. It will then prompt us to enter a guess. Upon entering our
guess, it will tell us if we’re too low or too high. Once we guess correctly,
it will congratulate us.

## Setting Up a New Project

Let’s set up a new project. Go to your projects directory from the previous
chapter, and create a new project using Cargo, like so:

```bash
$ cargo new guessing_game --bin
$ cd guessing_game
```

We pass the name of our project to `cargo new` and pass the `--bin` flag, since
we’re going to be making another binary like in Chapter 1.

Take a look at the generated `Cargo.toml`:

Filename: Cargo.toml

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

If the author information that Cargo got from your environment is not correct,
go ahead and fix that in the file and save it again.

And as we saw in the last chapter, `cargo new` generates a "Hello, world!"
program for us. Check out `src/main.rs`:

Filename: src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Let’s try compiling what Cargo gave us and running it in the same step, using
the `cargo run` command:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Hello, world!
```

Great! The `run` command comes in handy when you need to rapidly iterate on a
project. Our game is such a project: we want to quickly test each
iteration before moving on to the next one.

Now open up your `src/main.rs` again. We’ll be writing all of our code in this
file.

## Processing a Guess

Let’s get to it! We'll split the development of this game up into parts. This
first part will ask for input from a user and process the input, checking that
the input is in the form we expect. First we need to allow our player to input
a guess. Enter this in your `src/main.rs`:

Filename: src/main.rs

```rust,ignore
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

There’s a lot here! Let’s go over it, bit by bit.

```rust,ignore
use std::io;
```

We’ll need to take user input and then print the result as output, and for that
functonality we need to import the `io` (input/output) library from the
standard library (which is known as `std`).

By default, Rust only imports a few things into every program in [the
*prelude*][prelude]. If it’s not in the prelude, you’ll have to import it into
your program explicitly with a `use` statement. Using the `std::io` library
gets you a number of useful `io`-related things, including the functionality to
accept user input.

[prelude]: ../std/prelude/index.html

```rust,ignore
fn main() {
```

As you’ve seen in Chapter 1, the `main()` function is the entry point into the
program. The `fn` syntax declares a new function, the `()`s indicate that
there are no arguments, and `{` starts the body of the function.

```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```

As we learned in Chapter 1, `println!()` is a macro that prints a string to the
screen. This is just a prompt stating what the game is and requesting input from
the user.

### Storing Values with Variable Bindings

Next we need to store the user input.

```rust,ignore
let mut guess = String::new();
```

Now we’re getting interesting! There’s a lot going on in this little line.
The first thing to notice is that this is a `let` statement, which is
used to create *variable bindings*. Here's another example:

```rust,ignore
let foo = bar;
```

This will create a new binding named `foo`, and bind it to the value `bar`. In
many languages, this is called a *variable*, but Rust’s variable bindings have
a few differences.

For example, they’re immutable by default. To make our binding mutable, our
example uses `mut` before the binding name.

```rust
let foo = 5; // immutable.
let mut bar = 5; // mutable
```

> Note: The `//` syntax will start a comment that continues until the end of the
> line. Rust ignores everything in comments.

So now we know that `let mut guess` will introduce a mutable binding named
`guess`, but we have to look at the other side of the `=` for the value it’s
bound to: `String::new()`.

`String` is a string type, provided by the standard library. A
[`String`][string] is a growable, UTF-8 encoded bit of text.

[string]: ../std/string/struct.String.html

The `::` syntax in the `::new()` line indicates that `new()` is an *associated
function* of a particular type. An associated function is a function that is
associated with a type, in this case `String`, rather than a particular
instance of a `String`. Some languages call this a *static method*.

This `new()` function creates a new, empty `String`.
You’ll find a `new()` function on many types, as it’s a common name for making
a new value of some kind.

So to summarize, the `let mut guess = String::new();` line has created a
mutable binding that is currently bound to a new, empty instance of a `String`.
Whew!

Let’s move forward:

```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

Remember how we said `use std::io;` on the first line of the program? We’re now
calling an associated function on it. If we didn’t `use std::io`, we could
have written this line as `std::io::stdin()`.

This particular function returns an instance of [`std::io::Stdin`][iostdin],
which is a type that represents a handle to the standard input for your
terminal.

[iostdin]: ../std/io/struct.Stdin.html

The next part, `.read_line(&mut guess)`, calls the [`readline()`][read_line]
method on the standard input handle to get input from the user. We’re also
passing one argument to `read_line()`: `&mut guess`.

[read_line]: ../std/io/struct.Stdin.html#method.read_line

The job of `read_line()` is to take whatever the user types into standard input
and place that into a string, so it takes that string as an argument. The
string argument needs to be mutable so that the method can change the string's
content by adding the user input.

The `&` indicates that this argument is a *reference*, which gives you a way to
allow multiple parts of your code to access to one piece of data without
needing to copy that data into memory multiple times. References are a complex
feature, and one of Rust’s major advantages is how safe and easy it is to use
references. We don’t need to know a lot of those details to finish our program
right now, though; Chapter XX will cover references in more detail. For now,
all we need to know is that like `let` bindings, references are immutable by
default. Hence, we need to write `&mut guess`, rather than `&guess` to make it
mutable.

We’re not quite done with this line of code. While it’s a single line of text,
it’s only the first part of the single logical line of code. The second part is
this method:

```rust,ignore
.expect("Failed to read line");
```

When you call a method with the `.foo()` syntax, it's often wise to introduce a
newline and other whitespace. This helps you split up long lines. We _could_
have written this code as:

```rust,ignore
io::stdin().read_line(&mut guess).expect("failed to read line");
```

But that gets hard to read. So we’ve split it up, two lines for two method
calls. Now let's see what this line does.

### Handling Potential Failure with the `Result` Type

We mentioned that `read_line()` puts what the user types into the string we
pass it, but it also returns a value: in this case, an
[`io::Result`][ioresult]. Rust has a number of types named `Result` in its
standard library: a generic [`Result`][result], and then specific versions for
sub-libraries, like `io::Result`.

[ioresult]: ../std/io/type.Result.html
[result]: ../std/result/enum.Result.html

The `Result` types are [enums], which is short for *enumeration*. An
enumeration is a type that can have a fixed set of values, which are called the
`enum`'s *variants*. We will be covering enums in more detail in Chapter XX.

For `Result`, the variants are `Ok` or `Err`. `Ok` means the operation was
successful, and inside the `Ok` variant is the successfully generated value.
`Err` means the operation failed, and the `Err` contains information about how
or why the operation failed.

[enums]: ch06-00-enums.html

The purpose of these `Result` types is to encode error handling information.
Values of the `Result` type, like any type, have methods defined on them. In
this case, `io::Result` has an [`expect()` method][expect] that we can call. If
this instance of `io::Result` is an `Err` value, `expect()` will cause our
program to crash and display the message that we passed as an argument to
`expect()`. In this case, if the `read_line()` method returns an `Err`, it would
likely be the result of an error coming from the underlying operating system.
If this instance of `io::Result` is an `Ok` value, `expect()` will take the
return value that `Ok` is holding out of the `Ok` and return just that value to
us so that we can use it. In this case, that value will be what the user
entered into standard input.

[expect]: ../std/result/enum.Result.html#method.expect

If we don't call `expect()`, our program will compile, but we’ll get a warning:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
src/main.rs:10:5: 10:39 warning: unused result which must be used,
#[warn(unused_must_use)] on by default
src/main.rs:10     io::stdin().read_line(&mut guess);
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

Rust warns that we haven’t used the `Result` value, telling us that we
haven’t handled a possible error. The right way to suppress the warning is to
actually write error handling, but if we just want to crash the program when a
problem occurs, we can use `expect()`. We’ll save recovering from errors for a
future project.

### Printing Values with `println!()` Placeholders

There’s only one line of this first example left, aside from the closing curly
brace:

```rust,ignore
    println!("You guessed: {}", guess);
```

This prints out the string we saved our input in. The `{}`s are a placeholder:
think of `{}` as little crab pincers, holding a value in place. You can print
more than one value this way: the first `{}` holds the first value listed after
the format string, the second set holds the second value, and so on. Printing
out multiple values in one call to `println!()` would then look like this:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

Which would print out "x = 5 and y = 10".

### Testing the First Part

Back to our guessing game, let's test what we have so far. We can run it with
`cargo run`:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

All right! Our first part is done: we can get input from the keyboard and then
print it back out.

## Generating a Secret Number

Next, we need to generate a secret number that the user is trying to guess. The
secret number should be different every time so that the game is fun to play
more than once. So we'd like to have a random number between 1 and 100. Rust
does not yet include random number functionality in its standard library. The
Rust team does, however, provide a [`rand` crate][randcrate].

[randcrate]: https://crates.io/crates/rand

### Using a Crate to Get More Functionality

Remember that *crate* is what we call a package of Rust code. The project we’ve
been building is a *binary crate*, which is an executable. The `rand` crate is
a *library crate*, which contains code intended to be used in other programs.

Cargo's use of external crates is where it really shines. Before we can write
the code using `rand`, we need to modify our `Cargo.toml` to include the `rand`
crate as a dependency. Open it up, and add this line at the bottom beneath the
`[dependencies]` section header that Cargo created for you:

Filename: Cargo.toml

```toml
[dependencies]

rand = "0.3.14"
```

In the `Cargo.toml` file, everything that follows a header is part of a section
that goes until another section starts. Cargo uses the `[dependencies]` section
to know what external crates your project depends on and what versions of those
crates you require. In this case, we’ve specified the `rand` crate with the
semantic version specifier `0.3.14`. Cargo understands [Semantic
Versioning][semver] (sometimes called *semver*), which is a standard for
writing version numbers. A bare number like above is actually shorthand for
`^0.3.14`, which means "any version that has a public API compatible with
version 0.3.14".

[semver]: http://semver.org

Now, without changing any of our code, let’s build our project:

```bash
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
```

You may see different version numbers (but they will all be compatible with
your code, thanks to semver!) and the lines may be in a different order.

Lots of new output! Now that we have an external dependency, Cargo fetches the
latest versions of everything from the *registry*, which is a copy of data from
[Crates.io][cratesio]. Crates.io is where people in the Rust ecosystem
post their open source Rust projects for others to use.

[cratesio]: https://crates.io

After updating the registry, Cargo checks our `[dependencies]` and downloads
any we don’t have yet. In this case, while we only listed `rand` as a
dependency, we’ve also grabbed a copy of `libc`, because `rand` depends on
`libc` to work. After downloading them, Rust compiles them and then compiles
our project.

If we run `cargo build` again, we’ll get different output:

```bash
$ cargo build
```

That’s right, no output! Cargo knows that our project has been built, that
all of its dependencies are built, and that no changes have been made. There’s
no reason to do all that stuff again. With nothing to do, it simply
exits. If we open up `src/main.rs`, make a trivial change, then save it again,
we’ll only see one line:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
```
This just updates the build with your tiny change to the `main.rs` file.

#### The Cargo.lock File that Ensures Reproducible Builds

Cargo has a mechanism to make sure we can rebuild the exact same artifact every
time we or anyone else builds our code: Cargo will only use the versions of the
dependencies you specified until you say otherwise. For example, what happens
if next week version `v0.3.15` of the `rand` crate comes out, containing an
important bugfix, but that also contains a regression that will break our code?

The answer to this problem is the `Cargo.lock` file created the first time we
ran `cargo build` that is now in your project directory. When you build your
project for the first time, Cargo figures out all of the versions of your
dependencies that fit your criteria then writes them to the `Cargo.lock` file.
When you build your project in the future, Cargo will see that the `Cargo.lock`
file exists and use the versions specified there rather than doing all the work
of figuring out versions again. This lets you have a reproducible build
automatically. In other words, our project will stay at `0.3.14` until we
explicitly upgrade, thanks to the lock file.

#### Updating a Crate to Get a New Version

When we _do_ want to update a crate, Cargo has another command,
`update`, which will:

- Ignore the `Cargo.lock` file and figure out all the latest versions that fit
  our specifications in `Cargo.toml`.
- If that works, write those versions out to the lock file.

But by default, Cargo will only look for versions larger than `0.3.0` and
smaller than `0.4.0`. If the `rand` crate has released two new versions,
`0.3.15` and `0.4.0`, this is what we would see if we ran `cargo update`:

```bash
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```

At this point, you would also notice a change in your `Cargo.lock` file noting
that the version of the `rand` crate you are now using is `0.3.15`.

If we wanted to use `rand` version `0.4.0` or any version in the `0.4.x`
series, we’d have to update what is in the `Cargo.toml` file to look like this
instead:

```toml
[dependencies]

rand = "0.4.0"
```

The next time we `cargo build`, assuming that the `rand` crate version `0.4.0`
has been released, Cargo will update the crates index and re-evaluate our
`rand` requirements according to the new version we have specified.

There’s a lot more to say about [Cargo][doccargo] and [its
ecosystem][doccratesio] that we will get into in Chapter XX, but for now,
that’s all we need to know. Cargo makes it really easy to re-use libraries, so
Rustaceans are able to write smaller projects which are assembled out of a
number of sub-packages.

[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html

### Generating a Random Number

Let’s get on to actually _using_ `rand`. Our next step is to update our
`main.rs` code as follows:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);
}
```

First we've added a line to the top, `extern crate rand`, that lets Rust know
we’ll be making use of that external dependency. This also does the equivalent
of calling `use rand`, so we can now call anything in the `rand` crate by
prefixing it with `rand::`.

Next, we added another `use` line: `use rand::Rng`. `Rng` is a trait that
defines methods that random number generators implement, and this trait must be
in scope for us to use those methods. We'll cover traits in detail in the
Traits section in Chapter XX.

We also added two more lines in the middle:

```rust,ignore
let secret_number = rand::thread_rng().gen_range(1, 101);

println!("The secret number is: {}", secret_number);
```

`rand::thread_rng()` is a function that will give us the particular random
number generator that we're going to use: one that is local to our current
thread of execution and seeded by the operating system. Next, we call the
`gen_range()` method on our random number generator. This method is one that is
defined by the Rng trait that we brought into scope with the `use rand::Rng`
statement above. `gen_range()` takes two numbers as arguments and generates a
random number between them. It’s inclusive on the lower bound but exclusive on
the upper bound, so we need `1` and `101` to ask for a number ranging from one
to a hundred.

Knowing what traits to import and what functions and methods to use from a
crate isn't something that you'll just *know*. Instructions for using a crate
are in each crate's documentation. Another neat feature of Cargo is that you
can run the `cargo doc --open` command to build documentation provided by all
of your dependencies locally and then open it in your browser. If you're
interested in other functionality in the `rand` crate, for example, run `cargo
doc --open` then click on "rand" in the sidebar on the left.

The second line that we added to our code prints out the secret number. This is
useful while we’re developing our program to let us easily test it out, but
we’ll be deleting it for the final version. It’s not much of a game if it
prints out the answer when you start it up!

Try running our new program a few times:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4
$ cargo run
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

You should get different random numbers, and they should all be numbers between
1 and 100. Great job!

## Comparing Our Guesses

Now that we’ve got user input, let’s compare our guess to the secret number.
Here’s that part of our next step:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

There are a few new bits here. The first is another `use`, bringing a type
called `std::cmp::Ordering` into scope from the standard crate. `Ordering` is
another enum, like `Result`, but the variants for `Ordering` are `Less`,
`Greater`, and `Equal`. These are the three outcomes that are possible when you
compare two things.

Then we add five new lines at the bottom that use the `Ordering` type:

```rust,ignore
match guess.cmp(&secret_number) {
    Ordering::Less    => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal   => println!("You win!"),
}
```

The `cmp()` method compares two values, and can be called on anything that can
be compared. It takes a reference to the thing you want to compare it to, so
here it's comparing our `guess` to our `secret_number`. `cmp()` returns a
variant of the `Ordering` type we imported with the `use` statement earlier. We
use a [`match`][match] statement to decide what to do next based on which
variant of `Ordering` we got back from our call to `cmp()` with the values in
`guess` and `secret_number`.

[match]: match.html

`match` statements are made up of *arms*. An arm consists of a *pattern* and
the code that we should run if the value given to the beginning of the `match`
statement fits that arm's pattern. Rust takes the value given to `match` and
looks through each arm's pattern in turn. The `match` construct and patterns
are powerful features in Rust that will be covered in detail in Chapter XX and
Chapter XX, respectively.

Let's walk through an example of what would happen with our `match`. Say that
the user has guessed 50, and the randomly-generated secret number this time
is 38. So when we compare 50 to 38, the `cmp()` method will return
`Ordering::Greater`, since 50 is greater than 38. `Ordering::Greater` is the
value that the `match` statement gets. It looks at the first arm's pattern,
`Ordering::Less`, and says nope, the value we have (`Ordering::Greater`) does
not match `Ordering::Less`. So it ignores the code in that arm and moves on to
the next arm. The next arm's pattern, `Ordering::Greater`, **does** match
`Ordering::Greater`! The associated code in that arm will get executed, which
prints "Too big!" to the screen. Then we're done with the `match` statement; we
don't look at the last arm at all in this particular scenario.

However, this code won’t quite compile yet. Let’s try it:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
src/main.rs:23:21: 23:35 error: mismatched types [E0308]
src/main.rs:23     match guess.cmp(&secret_number) {
                                   ^~~~~~~~~~~~~~
src/main.rs:23:21: 23:35 help: run `rustc --explain E0308` to see a detailed explanation
src/main.rs:23:21: 23:35 note: expected type `&std::string::String`
src/main.rs:23:21: 23:35 note:    found type `&_`
error: aborting due to previous error
Could not compile `guessing_game`.
```

Whew! This is a big error. The core of the error says that we have *mismatched
types*. Rust has a strong, static type system. However, it also has type
inference. When we wrote `let guess = String::new()`, Rust was able to infer
that `guess` should be a `String` and didn’t make us write the type out. Our
`secret_number` on the other hand is a number type. There are a few number
types which can have a value between one and a hundred: `i32`, a thirty-two-bit
number; or `u32`, an unsigned thirty-two-bit number; `i64`, a sixty-four-bit
number; or others. Rust defaults to an `i32`, so that's the type of
`secret_number`. The error is because Rust will not compare two different types.

Ultimately, we want to convert the `String` we read as input
into a real number type so that we can compare it to the guess numerically. We
can do that with two more lines; add this to your program:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

The two new lines are:

```rust,ignore
let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

We create a variable binding `guess`. But wait a minute, don't we already have
a variable binding named `guess`? We do, but Rust allows us to *shadow* the
previous value of `guess` with a new one. This is often used in this exact
situation, where we want to convert a value from one type into another type.
Shadowing lets us re-use the `guess` variable name rather than forcing us to
come up with two unique bindings, like `guess_str` and `guess` or something.

We bind `guess` to the expression `guess.trim().parse()`.

The `guess` in the expression refers to the original `guess` that was a
`String` with our input in it. The `trim()` method on `String`s will eliminate
any white space at the beginning and end. Our u32 can only contain numerical
characters, but we have to press the "return" key to satisfy `read_line()`.
When we press the return key, it introduces a newline character. For example,
if we type `5` and hit return, `guess` looks like this: `5\n`. The `\n`
represents "newline", the return key. The `trim()` method gets rid of this,
leaving our string with only the `5`.

The [`parse()` method on strings][parse] parses a string into some kind of
number. Since this method can parse a variety of number types, we need to tell
Rust the exact type of number we want with `let guess: u32`. The colon (`:`)
after `guess` tells Rust we’re going to annotate its type. Rust has a few
built-in number types, but we’ve chosen `u32`, an unsigned, thirty-two bit
integer. It’s a good default choice for a small positive number. You'll see the
other number types in Chapter XX.

[parse]: ../std/primitive.str.html#method.parse

Our call to `parse()` could quite easily cause an error, if, for example, our
string contained `A👍%`; there’d be no way to convert that to a number. Because
it might fail, the `parse()` method returns a `Result` type, much like the
`read_line()` method does that we discussed earlier. We're going to treat this
`Result` the same way by using the `expect()` method again. If `parse()`
returns an `Err` `Result` variant because it could not create a number from the
string, the `expect()` call will crash the game and print the message we give
it. If `parse()` can successfully turn the string into a number, it will return
the `Ok` variant of `Result`, and `expect()` will return the number that we
want that it will take out of the `Ok` value for us.

Let’s try our program out!

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Nice! You can see we even added spaces before our guess, and it still figured
out that we guessed 76. Run the program a few times to verify the different
behavior with different kinds of input: guess the number correctly, guess a
number that is too high, and guess a number that is too low.

Now we’ve got most of the game working, but we can only make one guess. Let’s
change that by adding a loop!

## Allowing Multiple Guesses with Looping

The `loop` keyword gives us an infinite loop. We'll add that in to give us more
chances at guessing the number:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}
```

As you can see, we've moved everything from the guess input onwards into a
loop. Make sure to indent those lines another four spaces each, and try it out.
You'll notice we have a new problem because the program is doing exactly what we
told it to do: ask for another guess forever! It doesn't seem like we can quit!

We could always halt the program by using the keyboard shortcut `control-c`.
There's another way to escape the monster we've created that will infinitely
demand more guesses, though, that can be found in our discussion about
`parse()`: if we give a non-number answer, the program will crash. We can use
that to quit! Observe:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/libcore/result.rs:785
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/guess` (exit code: 101)
```

This method means that typing `quit` actually quits the game, but so does any
other non-number input. This is suboptimal, to say the least. We want the game
to automatically stop when the correct number is guessed.

#### Quitting When you Win

Let’s program the game to quit when you win by adding a `break` in that case:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

By adding the `break` line after `You win!`, we’ll exit the loop when we guses
the secret number correctly. Exiting the loop also means exiting the program,
since the loop is the last thing in `main()`.

#### Handling Invalid Input

For our final refinement of the game's behavior, rather than crashing the
program when someone inputs a non-number, we want the game to ignore it so we
can continue guessing. We can do that by altering the line where we convert
`guess` from a `String` to a `u32`:

```rust,ignore
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

This is how you generally move from "crash on error" to "actually handle the
error": by switching from an `expect()` statement to a `match` statement.
Remember that `parse()` returns a `Result` type, and `Result` is an enum that
has the variants `Ok` or `Err`. We're going to use a `match` statement here,
like we did with the `Ordering` result of the `cmp()` method.

If `parse()` is able to successfully turn the string into a number, it will
return an `Ok` value that contains the resulting number. That `Ok` value will
match the first arm's pattern, and the match statement will just return the
`num` value that `parse()` produced and put inside the `Ok` value. That number
will end up right where we want it, in the new `guess` binding we're creating.

If `parse()` is *not* able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first match arm, but it does match
the `Err(_)` pattern in the second arm. The `_` is a catch-all value; we're
saying we want to match all `Err` values, no matter what information they have
inside them. So we execute the second arm's code, `continue`: this means to go
to the next iteration of the `loop` and ask for another guess. So we have
effectively ignored all errors that `parse()` might hit!

Now everything in our program should work as we expect it to! Let’s try it:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Awesome! With one tiny last tweak, we can finish the guessing game: we're still
printing out the secret number. That was good for testing, but it kind of ruins
the game. Let's delete the `println!` that outputs the secret number. Here’s our
full, final code:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Complete!

At this point, you have successfully built the Guessing Game! Congratulations!

This project was a hands-on way to introduce you to a lot of new Rust concepts:
`let`, `match`, methods, associated functions, using external crates, and more.
In the next few chapters, we will go through these concepts in more detail.
Chapter 3 covers concepts that most programming languages have, like variables,
data types, and functions, and shows how to use them in Rust. Chapter 4 gets
into ownership, which is the feature of Rust that is most different from other
languages. Chapter 5 discusses structs and method syntax, and Chapter 6
endeavors to explain enums.
