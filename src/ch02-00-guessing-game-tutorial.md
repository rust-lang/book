# Guessing Game

Let’s jump into Rust by working on a hands-on project! This chapter introduces
you to a few common Rust concepts by showing you how to use them in a real
program. You’ll learn about let, match, methods, associated functions, using
external crates, and more! The following chapters will explore these ideas in
more detail: in this chapter, you’ll practice the fundamentals.

You’ll implement a classic beginner programming problem: a guessing game.
Here’s how it works: the program will generate a random integer between 1 and
100. It will then prompt you to enter a guess. After entering a guess, it will
indicate whether the guess is too low or too high. If you guess correctly, the
game will congratulate you.

## Setting Up a New Project

To set up a new project, go to your projects directory that you established in
Chapter 1, and create a new project using Cargo, like so:

```bash
$ cargo new guessing_game --bin
$ cd guessing_game
```

You pass the name of your project to cargo new and pass the --bin flag, because
you’ll be making another binary similar to the one in Chapter 1.

Look at the generated Cargo.toml file:

Filename: Cargo.toml

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

If the author information that Cargo obtained from your environment is not
correct, fix that in the file and save it again.

As you saw in Chapter 1, cargo new generates a “Hello, world!” program for you.

Check out the src/main.rs file:

Filename: src/main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

Now let’s compile this “Hello, world!” program and run it in the same step
using the cargo run command:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Hello, world!
```

The run command comes in handy when you need to rapidly iterate on a project,
and this game represents such a project: you want to quickly test each
iteration before moving on to the next one.

Reopen the src/main.rs file. You’ll be writing all your code in this file.

## Processing a Guess

Next, you’ll split the game development into parts. The first part will ask for
user input, process that input, and check that the input is in the expected
form. To start, you’ll need to allow the player to input a guess. Enter the
code in Listing 2-1 into src/main.rs.

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

Listing 2-1:

This code contains a lot of information, so let’s go over it bit by bit. To
obtain user input and then print the result as output, you need to import the
io (input/output) library from the standard library (which is known as std):

```rust,ignore
use std::io;
```

By default, Rust imports only a few things into every program in the prelude.
If it’s not in the prelude, you must import it into your program explicitly
with a use statement. Using the std::io library provides you with a number of
useful io-related things, including the functionality to accept user input.

As you saw in Chapter 1, the main() function is the entry point into the
program:

```rust,ignore
fn main() {
```

The fn syntax declares a new function, the ()indicate there are no arguments,
and { starts the body of the function.

As you also learned in Chapter 1, println!() is a macro that prints a string to the screen:

```rust,ignore
println!("Guess the number!");

println!("Please input your guess.");
```

This code is just a prompt stating what the game is and requesting input from
the user.

### Storing Values with Variable Bindings

Next, you need to store the user input, like this:

```rust,ignore
let mut guess = String::new();
```

Now the program is getting interesting! There’s a lot going on in this little
line. Notice that this is a let statement, which is used to create variable
bindings. Here’s another example:

```rust,ignore
let foo = bar;
```

This line will create a new binding named foo and bind it to the value bar. In
many languages, this is called a variable, but Rust’s variable bindings have a
few differences. For example, they’re immutable by default. To make your
binding mutable, the following example shows you how to use mut before the
binding name:

let foo = 5; // immutable.
let mut bar = 5; // mutable

> Note: The // syntax starts a comment that continues until the end of the line.
> Rust ignores everything in comments.

Now you know that let mut guess will introduce a mutable binding named guess,
but you have to look at the other side of the equal sign (=) for the value it’s
bound to, which is String::new(). String is a string type provided by the
standard library. A String is a growable, UTF-8 encoded bit of text.

The :: syntax in the ::new() line indicates that new() is an associated
function of a particular type. An associated function is associated with a
type, in this case String, rather than a particular instance of a String. Some
languages call this a static method.

This new() function creates a new, empty String. You’ll find a new() function
on many types, because it’s a common name for making a new value of some kind.

To summarize, the let mut guess = String::new(); line has created a mutable
binding that is currently bound to a new, empty instance of a String. Whew!

Recall that you included the input/output functionality from the standard
library with use std::io; on the first line of the program. Now you call an
associated function, stdin(), on io:

```rust,ignore
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

If you didn’t have the use std::io line at the beginning of the program, you
could have written this function call as std::io::stdin(). The stdin() function
returns an instance of std::io::Stdin, which is a type that represents a handle
to the standard input for your terminal.

The next part of the code, .read_line(&mut guess), calls the read_line() method
on the standard input handle to get input from the user. You’re also passing
one argument to read_line(): &mut guess.

The job of read_line() is to take whatever the user types into standard input
and place that into a string, so it takes that string as an argument. The
string argument needs to be mutable so the method can change the string’s
content by adding the user input.

The & indicates that this argument is a reference, which gives you a way to let
multiple parts of your code access one piece of data without needing to copy
that data into memory multiple times. References are a complex feature, and one
of Rust’s major advantages is how safe and easy it is to use references. But
right now you don’t need to know a lot of those details to finish your program:
Chapter XX will cover references in more detail. For now, all you need to know
is that like let bindings, references are immutable by default. Hence, you need
to write &mut guess rather than &guess to make it mutable.

You’re not quite done with this line of code. Although it’s a single line of
text, it’s only the first part of the single logical line of code. The second
part is this method:

```rust,ignore
.expect("Failed to read line");
```

When you call a method with the .foo() syntax, it’s often wise to introduce a
newline and other whitespace to help break up long lines. You could have
written this code as:

```rust,ignore
io::stdin().read_line(&mut guess).expect("failed to read line");
```

However, one long line is difficult to read, so it’s best to divide it, two
lines for two method calls. Now let’s discuss what this line does.

### Handling Potential Failure with the Result Type

As mentioned earlier, read_line() puts what the user types into the string you
pass it, but it also returns a value—in this case, an io::Result. Rust has a
number of types named Result in its standard library: a generic Result as well
as specific versions for sublibraries, such as io::Result.

The Result types are enums, or enumerations. An enumeration is a type that can
have a fixed set of values, which is called the enum’s variants. Chapter XX
will cover enums in more detail.

For Result, the variants are Ok or Err. Ok indicates the operation was
successful, and inside the Ok variant is the successfully generated value. Err
means the operation failed, and Err contains information about how or why the
operation failed.

The purpose of these Result types is to encode error handling information.
Values of the Result type, like any type, have methods defined on them. In this
case, io::Result has an expect() method that you can call. If this instance of
io::Result is an Err value, expect() will cause the program to crash and
display the message that you passed as an argument to expect(). In this case,
if the read_line() method returns an Err, it would likely be the result of an
error coming from the underlying operating system. If this instance of
io::Result is an Ok value, expect() will take the return value that Ok is
holding and return just that value to you so you can use it. In this case, that
value will be what the user entered into standard input.

If you don’t call expect(), the program will compile, but you’ll get a warning:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
src/main.rs:10:5: 10:39 warning: unused result which must be used,
#[warn(unused_must_use)] on by default
src/main.rs:10     io::stdin().read_line(&mut guess);
                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

Rust warns that you haven’t used the Result value returned from read_line(),
indicating that you haven’t handled a possible error. The right way to suppress
the warning is to actually write error handling, but if you just want to crash
the program when a problem occurs, you can use expect(). You’ll learn about
recovering from errors in a future project.

### Printing Values with println!() Placeholders

Aside from the closing curly brace in the guessing game example, there’s only
one more line to discuss, which is the following:

```rust,ignore
    println!("You guessed: {}", guess);
```

This line prints out the string you saved the user’s input in. The set of {} is
a placeholder that holds a value in place. You can print more than one value
using {}: the first set of {} holds the first value listed after the format
string, the second set holds the second value, and so on. Printing out multiple
values in one call to println!() would look like this:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

This code would print out “x = 5 and y = 10”.

### Testing the First Part

Let’s test the first part of the guessing game. You can run it using cargo run:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

At this point, the first part of the game is done: you can get input from the
keyboard and then print it.

## Generating a Secret Number

Next, you need to generate a secret number that the user will try to guess. The
secret number should be different every time so the game is fun to play more
than once. Let’s use a random number between 1 and 100 so the game isn’t too
difficult. Rust doesn’t yet include random number functionality in its standard
library. However, the Rust team does provide a rand crate at
https://crates.io/crates/rand.

### Using a Crate to Get More Functionality

Remember that crate is a package of Rust code. The project you’ve been building
is a binary crate, which is an executable. The rand crate is a library crate,
which contains code intended to be used in other programs.

Cargo’s use of external crates is where it really shines. Before you can write
code that uses rand, you need to modify the Cargo.toml file to include the rand
crate as a dependency. Open that file now and add the following line to the
bottom beneath the [dependencies] section header that Cargo created for you:

Filename: Cargo.toml

```toml
[dependencies]

rand = "0.3.14"
```

In the Cargo.toml file, everything that follows a header is part of a section
that continues until another section starts. Cargo uses the [dependencies]
section to know which external crates your project depends on and which
versions of those crates you require. In this case, you’ve specified the rand
crate with the semantic version specifier 0.3.14. Cargo understands Semantic
Versioning (sometimes called SemVer), which is a standard for writing version
numbers. The number 0.3.14 is actually shorthand for ^0.3.14, which means “any
version that has a public API compatible with version 0.3.14.”

Now, without changing any of the code, let’s build the project, as shown in
Listing 2-2:

```bash
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
 Downloading libc v0.2.14
   Compiling libc v0.2.14
   Compiling rand v0.3.14
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
```

Listing 2-2:

You may see different version numbers (but they will all be compatible with
your code, thanks to SemVer!), and the lines may be in a different order.

Now that you have an external dependency, Cargo fetches the latest versions of
everything from the registry, which is a copy of data from https://crates.io.
Crates.io is where people in the Rust ecosystem post their open source Rust
projects for others to use.

After updating the registry, Cargo checks your [dependencies] and downloads any
you don’t have yet. In this case, although you only listed rand as a
dependency, Cargo also grabbed a copy of libc, because rand depends on libc to
work. After downloading them, Rust compiles them and then compiles your project.

If you run cargo build again, you’ll get different output:

```bash
$ cargo build
```

That’s right, no output! Cargo knows that your project has been built, that all
of its dependencies are built, and that no changes have been made. There’s no
reason to do all that stuff again. With nothing to do, Cargo simply exits. If
you open src/main.rs, make a trivial change, and then save it again, you’ll see
only one line:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
```

This line just updates the build with your tiny change to the src/main.rs file.

#### The Cargo.lock File Ensures Reproducible Builds

Cargo has a mechanism that ensures you can rebuild the same artifact every time
you or anyone else builds your code: Cargo will use only the versions of the
dependencies you specified until you specify otherwise. For example, what
happens if next week version v0.3.15 of the rand crate comes out and contains
an important bug fix but also contains a regression that will break your code?

The answer to this problem is the Cargo.lock file, which was created the first
time you ran cargo build and is now in your project directory. When you build
your project for the first time, Cargo figures out all the versions of your
dependencies that fit your criteria and then writes them to the Cargo.lock
file. When you build your project in the future, Cargo will see that the
Cargo.lock file exists and use the versions specified there rather than doing
all the work of figuring out versions again. This lets you have a reproducible
build automatically. In other words, your project will remain at 0.3.14 until
you explicitly upgrade, thanks to the Cargo.lock file.

#### Updating a Crate to Get a New Version

When you do want to update a crate, Cargo provides another command, update,
which will:

* Ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml.
* If that works, Cargo will write those versions to the Cargo.lock file.

But by default, Cargo will only look for versions later than 0.3.0 and earlier
than 0.4.0. If the rand crate has released two new versions, 0.3.15 and 0.4.0,
you would see the following if you ran cargo update:

```bash
$ cargo update
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Updating rand v0.3.14 -> v0.3.15
```

At this point, you would also notice a change in your Cargo.lock file noting
that the version of the rand crate you are now using is 0.3.15.

If you wanted to use rand version 0.4.0 or any version in the 0.4.x series,
you’d have to update the Cargo.toml file to look like this instead:

```toml
[dependencies]

rand = "0.4.0"
```

The next time you run cargo build, assuming that the rand crate version 0.4.0
has been released, Cargo will update the crate’s index and reevaluate your rand
requirements according to the new version you specified.

There’s a lot more to say about Cargo and its ecosystem that Chapter XX will
discuss, but for now, that’s all you need to know. Cargo makes it very easy to
reuse libraries, so Rustaceans are able to write smaller projects that are
assembled from a number of subpackages.

### Generating a Random Number

Let’s start using rand. The next step is to update your main.rs code, as shown
in Listing 2-3:

Filename: src/main.rs

```rust,ignore
 extern crate rand;

use std::io;
 use rand::Rng;

fn main() {
    println!("Guess the number!");

     let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

Listing 2-3:

A line is added to the top  that lets Rust know you’ll be using that external
dependency. This also does the equivalent of calling use rand, so you can now
call anything in the rand crate by prefixing it with rand::.

Next, another use line is added: use rand::Rng . Rng is a trait that defines
methods that random number generators implement, and this trait must be in
scope for you to use those methods. Chapter XX will cover traits in detail.

Also, two more lines are added in the middle . The rand::thread_rng() function
will give you the particular random number generator that you’re going to use:
one that is local to your current thread of execution and seeded by the
operating system. Next, you call the gen_range() method on your random number
generator. This method is defined by the Rng trait that you brought into scope
with the use rand::Rng statement. The gen_range() method takes two numbers as
arguments and generates a random number between them. It’s inclusive on the
lower bound but exclusive on the upper bound, so you need to specify 1 and 101
to request a number between 1 and 100.

Knowing which traits to import and which functions and methods to use from a
crate isn’t something that you’ll just know. Instructions for using a crate are
in each crate’s documentation. Another neat feature of Cargo is that you can
run the cargo doc --open command to build documentation provided by all of your
dependencies locally, and then open it in your browser. If you’re interested in
other functionality in the rand crate—for example, run cargo doc --open—click
rand in the sidebar on the left.

The second line that was added to the code prints the secret number. This is
useful while you’re developing your program to let you easily test it, but
you’ll delete it in the final version. It’s not much of a game if it prints the
answer as soon as you start it!

Try running your new program a few times:

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

## Comparing the Guesses

Now that you have user input, you can compare the guess to the secret number.
That step is shown in Listing 2-4:

Filename: src/main.rs

```rust,ignore
extern crate rand;

use std::io;
 use std::cmp::Ordering;
use rand::Rng;

fn main() {

---snip---

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
```

Listing 2-4:

The first new bit here is another use , bringing a type called
std::cmp::Ordering into scope from the standard crate. Ordering is another
enum, like Result, but the variants for Ordering are Less, Greater, and Equal.
These are the three outcomes that are possible when you compare two values.

Then five new lines are added at the bottom that use the Ordering type. The
cmp() method  compares two values and can be called on anything that can be
compared. It takes a reference to whatever you want to compare with: here it’s
comparing the guess to the secret_number. cmp() returns a variant of the
Ordering enum you imported with the use statement earlier. You use a match
statement  to decide what to do next based on which variant of Ordering you
received from your call to cmp() with the values in guess and secret_number.

match statements are made up of arms. An arm consists of a pattern and the code
you should run if the value given to the beginning of the match statement fits
that arm’s pattern. Rust takes the value given to match and looks through each
arm’s pattern in turn. The match construct and patterns are powerful features
in Rust that will be covered in detail in Chapter XX and Chapter XX,
respectively.

Let’s walk through an example of what would happen with the match statement
used here. Say that the user has guessed 50, and the randomly generated secret
number this time is 38. So when you compare 50 to 38, the cmp() method will
return Ordering::Greater, because 50 is greater than 38. Ordering::Greater is
the value that the match statement gets. It looks at the first arm’s pattern,
Ordering::Less, but the value (Ordering::Greater) does not match
Ordering::Less. So it ignores the code in that arm and moves to the next arm.
The next arm’s pattern, Ordering::Greater, does match Ordering::Greater! The
associated code in that arm will execute and print “Too big!” to the screen.
The match statement ends because it has no need to look at the last arm in this
particular scenario.

However, the code in Listing 2-4 won’t compile yet. Let’s try it:

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

This result shows a significant error. The core of the error states that there
are mismatched types. Rust has a strong, static type system. However, it also
has type inference. When you wrote let guess = String::new(), Rust was able to
infer that guess should be a String and didn’t make you write the type. The
secret_number, on the other hand, is a number type. A few number types can have
a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit
number; i64, a 64-bit number; as well as others. Rust defaults to an i32, which
is the type of secret_number unless you add type information elsewhere that
would cause Rust to infer a different numerical type. The reason for the error
is that Rust will not compare a string and a number type.

Ultimately, you want to convert the String the program reads as input into a
real number type so you can compare it to the guess numerically. You can do
that by adding the following two lines to your fn main() body:

Filename: src/main.rs

```rust,ignore
---snip---

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

You create a variable binding named guess. But wait, doesn’t the program
already have a variable binding named guess? It does, but Rust allows you to
shadow the previous value of guess with a new one. This feature is often used
in similar situations in which you want to convert a value from one type to
another type. Shadowing lets you reuse the guess variable name rather than
forcing you to create two unique bindings, like guess_str and guess or some
such. (Chapter 3 covers shadowing in more detail.)

You bind guess to the expression guess.trim().parse(). The guess in the
expression refers to the original guess that was a String with the input in it.
The trim() method on Strings will eliminate any whitespace at the beginning and
end. u32 can only contain numerical characters, but you must press the Return
key to satisfy read_line(). When you press Return, a newline character is
introduced. For example, if you type 5 and press return, guess looks like this:
5\n. The \n represents “newline,” the return key. The trim() method eliminates
\n, resulting in just 5.

The parse() method on strings parses a string into some kind of number. Because
this method can parse a variety of number types, you need to tell Rust the
exact number type you want by using let guess: u32. The colon (:) after guess
tells Rust you’ll annotate its type. Rust has a few built-in number types, but
you’ve chosen u32, an unsigned, 32-bit integer. It’s a good default choice for
a small positive number. You’ll learn about other number types in Chapter XX.
Additionally, the u32 annotation in this example program and the comparison
with secret_number means that Rust will infer that secret_number should be a
u32 as well. So now the comparison will be between two values of the same type!

The call to parse() could easily cause an error. If, for example, the string
contained A👍%, there would be no way to convert that to a number. Because it
might fail, the parse() method returns a Result type, much like the read_line()
method does that was discussed earlier. You’ll treat this Result the same way
by using the expect() method again. If parse() returns an Err Result variant
because it couldn’t create a number from the string, the expect() call will
crash the game and print the message you give it. If parse() can successfully
convert the string to a number, it will return the Ok variant of Result, and
expect() will return the number that you want, which it will take from the Ok
value.

Let’s run the program now!

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

Nice! Even though spaces were added before the guess, the program still figured
out that the user guessed 76. Run the program a few times to verify the
different behavior with different kinds of input: guess the number correctly,
guess a number that is too high, and guess a number that is too low.

You have most of the game working now, but the user can make only one guess.
Let’s change that by adding a loop!

## Allowing Multiple Guesses with Looping

The loop keyword gives you an infinite loop. You’ll add that now to give users
more chances at guessing the number:

Filename: src/main.rs

```rust,ignore
---snip---

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        ---snip---

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => println!("You win!"),
        }
    }
}
```

As you can see, everything was moved from the guess input onward into a loop.
Be sure to indent those lines another four spaces each, and run the program
again. Notice that there is a new problem because the program is doing exactly
what it was told to do: ask for another guess forever! It doesn’t seem like you
can quit!

You could always halt the program by using the keyboard shortcut Ctrl-C. But
there’s another way to escape this insatiable monster that was mentioned in the
parse()discussion in “Comparing the Guesses” on page XX: if you enter a
non-number answer, the program will crash. You can use that to quit, as shown
here:

Production: See the cross-reference above.

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

Typing quit actually quits the game, but so will any other non-number input.
However, this is suboptimal to say the least. You want the game to
automatically stop when the correct number is guessed.

#### Quitting When You Win

Let’s program the game to quit when you win by adding a break:

Filename: src/main.rs

```rust,ignore
---snip---

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

By adding the break line after You win!, you’ll exit the loop when you guess
the secret number correctly. Exiting the loop also means exiting the program,
because the loop is the last part of main().

#### Handling Invalid Input

To further refine the game’s behavior, rather than crashing the program when
you input a non-number, let’s make the game ignore a non-number so you can
continue guessing. You can do that by altering the line where guess is
converted from a String to a u32:

```rust,ignore
---snip---

io::stdin().read_line(&mut guess)
    .expect("failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

---snip---
```

Switching from an expect() statement to a match statement is how you generally
move from “crash on error” to “actually handling the error.” Remember that
parse() returns a Result type, and Result is an enum that has the variants Ok
or Err. You use a match statement here, like you did with the Ordering result
of the cmp() method.

If parse() is able to successfully turn the string into a number, it will
return an Ok value that contains the resulting number. That Ok value will match
the first arm’s pattern, and the match statement will just return the num value
that parse() produced and put it inside the Ok value. That number will end up
right where you want it in the new guess binding you’re creating.

If parse() is not able to turn the string into a number, it will return an Err
value that contains more information about the error. The Err value does not
match the Ok(num) pattern in the first match arm, but it does match the Err(_)
pattern in the second arm. The _ is a catchall value; in this example, you’re
saying you want to match all Err values, no matter what information they have
inside them. So you execute the second arm’s code, continue, which means to go
to the next iteration of the loop and ask for another guess. So effectively,
the program ignores all errors that parse() might encounter!

Now everything in the program should work as expected! Let’s try it by running
cargo run:

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

Awesome! With one tiny final tweak, you can finish the guessing game: recall
that the program is still printing out the secret number. That worked well for
testing, but it ruins the game. Let’s delete the println! that outputs the
secret number. Listing 2-5 shows the final code:

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

Listing 2-5:

## Complete!

At this point, you’ve successfully built the guessing game! Congratulations!

This project was a hands-on way to introduce you to many new Rust concepts:
let, match, methods, associated functions, using external crates, and more. In
the next few chapters, you’ll learn about these concepts in more detail.
Chapter 3 covers concepts that most programming languages have, such as
variables, data types, and functions, and shows how to use them in Rust.
Chapter 4 explores ownership, which is a Rust feature that is most different
from other languages. Chapter 5 discusses structs and method syntax, and
Chapter 6 endeavors to explain enums.
