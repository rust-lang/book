## Hello, World!

Now that you’ve installed Rust, let’s write your first Rust program. It’s
traditional when learning a new language to write a little program that prints
the text `Hello, world!` to the screen, so we’ll do the same here!

> Note: This book assumes basic familiarity with the command line. Rust makes
> no specific demands about your editing or tooling or where your code lives, so
> if you prefer to use an integrated development environment (IDE) instead of
> the command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Recently,
> the Rust team has been focusing on enabling great IDE support, and progress
> has been made rapidly on that front!

### Creating a Project Directory

You’ll start by making a directory to store your Rust code. It doesn’t matter
to Rust where your code lives, but for the exercises and projects in this book,
we suggest making a *projects* directory in your home directory and keeping all
your projects there.

Open a terminal and enter the following commands to make a *projects* directory
and a directory for the “Hello, world!” project within the *projects* directory.

For Linux, macOS, and PowerShell on Windows, enter this:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

For Windows CMD, enter this:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Writing and Running a Rust Program

Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, use *hello_world.rs* rather than
*helloworld.rs*.

Now open the *main.rs* file you just created and enter the code in Listing 1-1.

<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<span class="caption">Listing 1-1: A program that prints `Hello, world!`</span>

Save the file and go back to your terminal window. On Linux or macOS, enter
the following commands to compile and run the file:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

On Windows, enter the command `.\main.exe` instead of `./main`:

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

Regardless of your operating system, the string `Hello, world!` should print to
the terminal. If you don’t see this output, refer back to the
[“Troubleshooting”][troubleshooting]<!-- ignore --> part of the Installation
section for ways to get help.

If `Hello, world!` did print, congratulations! You’ve officially written a Rust
program. That makes you a Rust programmer—welcome!

### Anatomy of a Rust Program

Let’s review in detail what just happened in your “Hello, world!” program.
Here’s the first piece of the puzzle:

```rust
fn main() {

}
```

These lines define a function in Rust. The `main` function is special: it is
always the first code that runs in every executable Rust program. The first
line declares a function named `main` that has no parameters and returns
nothing. If there were parameters, they would go inside the parentheses, `()`.

Also, note that the function body is wrapped in curly brackets, `{}`. Rust
requires these around all function bodies. It’s good style to place the opening
curly bracket on the same line as the function declaration, adding one space in
between.

If you want to stick to a standard style across Rust projects, you can use an 
automatic formatter tool called `rustfmt` to format your code in a particular
style. The Rust team has included this tool with the standard Rust distribution,
like `rustc`, so it should already be installed on your computer! Check the online
documentation for more details.

Inside the `main` function is the following code:

```rust
    println!("Hello, world!");
```

This line does all the work in this little program: it prints text to the
screen. There are four important details to notice here.

First, Rust style is to indent with four spaces, not a tab.

Second, `println!` calls a Rust macro. If it called a function instead, it
would be entered as `println` (without the `!`). We’ll discuss Rust macros in
more detail in Chapter 19. For now, you just need to know that using a `!`
means that you’re calling a macro instead of a normal function.

Third, you see the `"Hello, world!"` string. We pass this string as an argument
to `println!`, and the string is printed to the screen.

Fourth, we end the line with a semicolon (`;`), which indicates that this
expression is over and the next one is ready to begin. Most lines of Rust code
end with a semicolon.

### Compiling and Running Are Separate Steps

You’ve just run a newly created program, so let’s examine each step in the
process.

Before running a Rust program, you must compile it using the Rust compiler by
entering the `rustc` command and passing it the name of your source file, like
this:

```console
$ rustc main.rs
```

If you have a C or C++ background, you’ll notice that this is similar to `gcc`
or `clang`. After compiling successfully, Rust outputs a binary executable.

On Linux, macOS, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell. On Linux and macOS, you’ll see two
files. With PowerShell on Windows, you’ll see the same three files that you
would see using CMD.

```text
$ ls
main  main.rs
```

With CMD on Windows, you would enter the following:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

This shows the source code file with the *.rs* extension, the executable file
(*main.exe* on Windows, but *main* on all other platforms), and, when using
Windows, a file containing debugging information with the *.pdb* extension.
From here, you run the *main* or *main.exe* file, like this:

```console
$ ./main # or .\main.exe on Windows
```

If *main.rs* was your “Hello, world!” program, this line would print `Hello,
world!` to your terminal.

If you’re more familiar with a dynamic language, such as Ruby, Python, or
JavaScript, you might not be used to compiling and running a program as
separate steps. Rust is an *ahead-of-time compiled* language, meaning you can
compile a program and give the executable to someone else, and they can run it
even without having Rust installed. If you give someone a *.rb*, *.py*, or
*.js* file, they need to have a Ruby, Python, or JavaScript implementation
installed (respectively). But in those languages, you only need one command to
compile and run your program. Everything is a trade-off in language design.

Just compiling with `rustc` is fine for simple programs, but as your project
grows, you’ll want to manage all the options and make it easy to share your
code. Next, we’ll introduce you to the Cargo tool, which will help you write
real-world Rust programs.

[troubleshooting]: ch01-01-installation.html#troubleshooting
