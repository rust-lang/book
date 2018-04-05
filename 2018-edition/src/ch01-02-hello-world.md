## Hello, World!

Now that you have Rust installed, let’s write your first Rust program. It’s
traditional when learning a new language to write a little program to print the
text “Hello, world!” to the screen, so we’ll do the same here!

> Note: This book assumes basic familiarity with the command line. Rust itself
> makes no specific demands about your editing, tooling, or where your code
> lives, so if you prefer an IDE (Integrated Development Environment) to the
> command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Enabling
> great IDE support has been a recent focus of the Rust team, and progress
> has been made rapidly on that front!

### Creating a Project Directory

First, make a directory to put your Rust code in. Rust doesn’t care where your
code lives, but for the exercises and projects in this book, we’d suggest
making a *projects* directory in your home directory and keeping all your
projects there.

Open a terminal and enter the following commands to make a *projects* directory
and, inside that, a directory for this “Hello, world!” project:

Linux and Mac:

```text
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

Windows PowerShell:

```powershell
> mkdir $env:USERPROFILE\projects
> cd $env:USERPROFILE\projects
> mkdir hello_world
> cd hello_world
```

### Writing and Running a Rust Program

Next, make a new source file and call it *main.rs*---Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, you’d use *hello_world.rs* rather
than *helloworld.rs*.

Now open the *main.rs* file you just created, and enter the code shown in
Listing 1-1:

<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<span class="caption">Listing 1-1: A program that prints “Hello, world!”</span>

Save the file, and go back to your terminal window. On Linux or macOS, enter
the following commands to compile and run the file:

```text
$ rustc main.rs
$ ./main
Hello, world!
```

On Windows, use `.\main.exe` instead of `./main`.

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

Regardless of your operating system, you should see the string `Hello, world!`
print to the terminal. If you don’t see this output, see the “Troubleshooting”
section earlier for ways to get help.

If you did see `Hello, world!` printed, then congratulations! You’ve officially
written a Rust program. That makes you a Rust programmer! Welcome!

<!-- Any quick words of advice for if they didn't? (Disclosure: I tried
following this using Bash on windows and couldn't get it working) -->
<!-- Added a pointer to the previous troubleshooting section which also applies
here /Carol -->

### Anatomy of a Rust Program

Now, let’s go over what just happened in your “Hello, world!” program in
detail. Here’s the first piece of the puzzle:

```rust
fn main() {

}
```

These lines define a *function* in Rust. The `main` function is special: it is
always the first code that is run for every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, their names would go inside the parentheses,
`(` and `)`.

Also note that the function body is wrapped in curly brackets, `{` and `}`.
Rust requires these around all function bodies. It’s considered good style to
place the opening curly bracket on the same line as the function declaration,
with one space in between.

> At the time of writing, an automatic formatter, `rustfmt`, is under
> development. If you’d like to stick to a standard style across Rust projects,
> `rustfmt` is a tool that will format your code in a particular style. The
> plan is to eventually include it with the standard Rust distribution, like
> `rustc`, so depending on when you read this book, you may have it already
> installed! Check the online documentation for more details.

Inside the `main` function, we have this code:

```rust
    println!("Hello, world!");
```

This line does all of the work in this little program: it prints text to the
screen. There are a number of details to notice here. The first is that Rust
style is to indent with four spaces, not a tab.

The second important detail is the `println!` call. This code is calling a Rust
*macro*. If it were calling a function instead, it would be entered as
`println` (without the `!`). We’ll discuss Rust macros in more detail in
Appendix D, but for now you just need to know that when you see a `!` that
means that you’re calling a macro instead of a normal function.

<!-- I might suggest just cutting this next macro section -- for the sake of
the intro, we don't really need this info, and I feel like this first exercise
should be short and sweet and simple -->
<!-- I'm ok with cutting this; it's a fairly common question that some folks
have at this point, but I'm ok with those people having to do some research
online if they're curious /Carol -->

Next comes`"Hello, world!"` which is a *string*. We pass this string as an
argument to `println!` and the total effect is that the string is printed to
the screen. Easy enough!

We end the line with a semicolon `;`, which indicates that this expression is
over, and the next one is ready to begin. Most lines of Rust code end with a
`;`.

### Compiling and Running Are Separate Steps

You’ve just seen how to run a newly created program, so now let’s break that
process down and examine each step.

Before running a Rust program, you have to compile it using the Rust compiler
by entering the `rustc` command and passing it the name of your source file,
like this:

```text
$ rustc main.rs
```

If you come from a C or C++ background, you’ll notice that this is similar to
`gcc` or `clang`. After compiling successfully, Rust outputs a binary
executable.

On Linux, Mac, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell as follows:

```text
$ ls
main  main.rs
```

With CMD on Windows, you’d enter:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

This shows we have two files: the source code, with the *.rs* extension, and
the executable (*main.exe* on Windows, *main* everywhere else). All that’s left
to do from here is run the *main* or *main.exe* file, like this:

```text
$ ./main  # or .\main.exe on Windows
```

If *main.rs* were your “Hello, world!” program, this would print `Hello,
world!` to your terminal.

If you come from a dynamic language like Ruby, Python, or JavaScript, you may
not be used to compiling and running a program being separate steps. Rust is an
*ahead-of-time compiled* language, which means that you can compile a program,
give the executable to someone else, and they can run it even without having
Rust installed. If you give someone a `.rb`, `.py`, or `.js` file, on the other
hand, they need to have a Ruby, Python, or JavaScript implementation installed
(respectively), but you only need one command to both compile and run your
program. Everything is a tradeoff in language design.

Just compiling with `rustc` is fine for simple programs, but as your project
grows, you’ll want to be able to manage all of the options and make it easy to
share your code. Next, we’ll introduce you to a tool called Cargo, which will
help you write real-world Rust programs.
