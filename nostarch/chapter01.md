# Introduction

Welcome to “The Rust Programming Language”, an introductory book about Rust.
Rust is a programming language that’s focused on safety, speed, and
concurrency. Its design lets you create programs that have the performance and
control of a low-level language, but with helpful abstractions that feel like a
high-level language. The Rust community welcomes all programmers who have their
experience in languages like C and are looking for a safer alternative, as well
as programmers from languages like Python who are looking for ways to write more
performant code without losing expressiveness.

Rust provides the majority of its safety checks at compile time and without a
garbage collector so that your program's runtime isn't impacted. This makes it
useful in a number of use cases that other languages aren’t good at: embedding
in other languages, programs with specific space and time requirements, and
writing low-level code, like device drivers and operating systems. It's also
great for web applications: it powers the Rust package registry site, crates.io!
We're excited to see what _you_ create with Rust.

This book is written for a reader who already knows how to program in at least
one programming language. After reading this book, you should be comfortable
writing Rust programs. We’ll be learning Rust through small, focused examples
that build on each other to demonstrate how to use various features of Rust as
well as how they work behind the scenes.

## Contributing to the book

This book is open source. If you find an error, please don’t hesitate to file an
issue or send a pull request on GitHub at *https://github.com/rust-lang/book*.

<!--[on GitHub]: https://github.com/rust-lang/book--> <!-- Where these links
occur, could you pull the actual link and place it in a sentence, like above?
That will make it easier for us to know what you want printed in the actual
book, and what's just going up on the online version -->

## Installation

The first step to using Rust is to install it. You’ll need an internet
connection to run the commands in this chapter, as we’ll be downloading Rust
from the internet.

We’ll be showing off a number of commands using a terminal, and those lines all
start with `$`. You don't need to type in the `$`s; they are there to indicate
the start of each command. You’ll see many tutorials and examples around the web
that follow this convention: `$` for commands run as a regular user, and `#`
for commands you should be running as an administrator. Lines that don't start
with `$` are typically showing the output of the previous command.

### Installing on Linux or Mac

If you're on Linux or a Mac, all you need to do is open a terminal and type
this:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

This will download a script and start the installation. You may be prompted for
your password. If it all goes well, you’ll see this appear:

```bash
Rust is installed now. Great!
```

### Installing on Windows

If you're on Windows, please download the appropriate [installer][install-page].

[install-page]: https://www.rust-lang.org/install.html

<!-- Are there are options you'd recommend clicking, too? There are two
download options: GNU and MSVC, what's the difference? What should the reader
choose? -->

### Uninstalling

Uninstalling Rust is as easy as installing it. On Linux or Mac, just run
the uninstall script:

```bash
$ rustup self uninstall
```

If you used the Windows installer, you can re-run the `.msi` and it will give
you an uninstall option.

### Troubleshooting

If you've got Rust installed, you can open up a shell, and type this:

```bash
$ rustc --version
```

You should see the version number, commit hash, and commit date in a format
similar to this for the latest stable version at the time you install:

```bash
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this, Rust has been installed successfully!
Congrats!

If you don't and you're on Windows, check that Rust is in your `%PATH%` system
variable. If it isn't, run the installer again, select "Change" on the "Change,
repair, or remove installation" page and ensure "Add to PATH" is checked.

If it still isn't working, there are a number of places where you can get help. The easiest is
[the #rust IRC channel on irc.mozilla.org][irc], which you can access through
[Mibbit][mibbit]. Click that link, and you'll be chatting with other Rustaceans
(a silly nickname we call ourselves) who can help you out. Other great resources
include [the user’s forum][users] and [Stack Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Local documentation

The installer also includes a copy of the documentation locally, so you can
read it offline. On Linux or Mac, run `rustup doc` to open the local
documentation in your browser. On Windows, the documentation is in a
`share/doc` directory inside the directory where Rust was installed.

## Hello, World!

Now that you have Rust installed, let’s write your first Rust program. It's
traditional when learning a new language to write a little program to print the
text “Hello, world!” to the screen, and in this section, we'll follow that
tradition.

> Note: This book assumes basic familiarity with the command line. Rust itself
> makes no specific demands about your editing, tooling, or where your code
> lives, so if you prefer an IDE to the command line, feel free to use your favored IDE.

### Creating a Project File

First, make a file to put your Rust code in. Rust doesn't care where your code
lives, but for this book, we'd suggest making a *projects* directory in your
home directory and keeping all your projects there. Open a terminal and enter
the following commands to make a directory for this particular project:

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

> Note: If you’re on Windows and not using PowerShell, the `~` that represents
> your home directory may not work.
> Consult the documentation for your shell for more details.

<!-- Could you be more specific on Windows instructions? For me, I just had to
remove the tilde and it worked perfectly -->

### Writing and Running a Rust Program

Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, you'd use *hello_world.rs* rather
than *helloworld.rs*.

Now open the *main.rs* file you just created, and type the following code:

```rust
fn main() {
    println!("Hello, world!");
}
```

Save the file, and go back to your terminal window. On Linux or OSX, enter the
following commands:

```bash
$ rustc main.rs
$ ./main
Hello, world!
```
<!-- On Windows I had to run `rustc main.rs` first to create a main.exe file,
and just typed "main" without ./ or `exe` to run it. Can you be more specific
here, maybe just lay out the windows version, since it's so short? -->

On Windows, just replace `main` with `main.exe`. Regardless of your operating
system, you should see the string `Hello, world!` print to the terminal. If you
did, then congratulations! You've officially written a Rust program. That makes
you a Rust programmer! Welcome.

### Anatomy of a Rust Program

Now, let’s go over what just happened in your "Hello, world!" program in
detail. Here's the first piece of the puzzle:

```rust
fn main() {

}
```

These lines define a *function* in Rust. The `main` function is special: it's
the first thing that is run for every executable Rust program. The first line
says, “I’m declaring a function named `main` that takes no arguments and
returns nothing.” If there were arguments, they would go inside the parentheses,
`(` and `)`. <!--- We aren’t returning anything from this function, so we have
omitted the return type entirely. If there was a return type, there would be a
`->` and the return type after the parentheses. -->

Also note that the function body is wrapped in curly braces, `{` and `}`. Rust
requires these around all function bodies. It's considered good style to put
the opening curly brace on the same line as the function declaration, with one
space in between.

Inside the `main()` function:

```rust
    println!("Hello, world!");
```

This line does all of the work in this little program: it prints text to the
screen. There are a number of details that are important here. The first is
that it’s indented with four spaces, not a tab.

The second important part is `println!()`. This is calling a Rust *macro*,
which is how metaprogramming is done in Rust. If it were calling a function
instead, it would look like this: `println()` (without the `!`). We'll discuss
Rust macros in more detail in Chapter XX, but for now you just need to know
that when you see a `!` that means that you’re calling a macro instead of a
normal function.

Next is `"Hello, world!"` which is a *string*. We pass this string as an
argument to `println!()`, which prints the string to the screen. Easy enough!

The line ends with a semicolon (`;`). The `;` indicates that this expression is
over, and the next one is ready to begin. Most lines of Rust code end with a
`;`.

### Compiling and Running Are Separate Steps

In "Writing and Running a Rust Program", we showed you how to run a newly
created program. We'll break that process down and examine each step now.

Before running a Rust program, you have to compile it. You can use the Rust
compiler by entering the `rustc` command and passing it the name of your source
file, like this:

```bash
$ rustc main.rs
```

If you come from a C or C++ background, you'll notice that this is similar to
`gcc` or `clang`. After compiling successfully, Rust should output a binary
executable, which you can see on Linux or OSX by entering the `ls` command in
your shell as follows:

```bash
$ ls
main  main.rs
```

On Windows, you'd enter:

```bash
$ dir
main.exe  main.rs
```
<!-- This is different for me too -- on windows `ls` worked but `dir` gave me
different information. However! My colleague also on Windows 10 couldn't get ls
to work, but dir gave the wrong info. I think we may need to look at getting a
tech reviewer to test Windows instructions -->

This shows we have two files: the source code, with the `.rs` extension, and the
executable (`main.exe` on Windows, `main` everywhere else). All that's left to
do from here is run the `main` or `main.exe` file, like this:

```bash
$ ./main  # or main.exe on Windows
```
<!-- `$ main` worked for me, without `./` or `exe` -->

If *main.rs* were your "Hello, world!" program, this would print `Hello,
world!` to your terminal.

If you come from a dynamic language like Ruby, Python, or JavaScript, you may
not be used to compiling and running a program being separate steps. Rust is an
*ahead-of-time compiled* language, which means that you can compile a program,
give it to someone else, and they can run it even without having Rust
installed. If you give someone a `.rb`, `.py`, or `.js` file, on the other
hand, they need to have a Ruby, Python, or JavaScript implementation installed
(respectively), but you only need one command to both compile and run your
program. Everything is a tradeoff in language design.

Just compiling with `rustc` is fine for simple programs, but as your project
grows, you'll want to be able to manage all of the options your project has
and make it easy to share your code with other people and projects. Next, we'll
introduce you to a tool called Cargo, which will help you write real-world Rust
programs.

## Hello, Cargo!

Cargo is Rust’s build system and package manager, and Rustaceans use Cargo to
manage their Rust projects because it makes a lot of tasks easier. For example,
Cargo takes care of building your code, downloading the libraries your code
depends on, and building those libraries. We call libraries your code needs
*dependencies*.

The simplest Rust programs, like the one we've written so far, don’t have any
dependencies, so right now, you'd only be using the part of Cargo that can take
care of building your code. As you write more complex Rust programs, you’ll
want to add dependencies, and if you start off using Cargo, that will be a lot
easier to do.

As the vast, vast majority of Rust projects use Cargo, we will assume that
you’re using it for the rest of the book. Cargo comes installed with Rust
itself, if you used the official installers as covered in the Installation
chapter. If you installed Rust through some other means, you can check if you
have Cargo installed by typing the following into your terminal:

```bash
$ cargo --version
```

If you see a version number, great! If you see an error like `command not
found`, then you should look at the documentation for your method of installation
to determine how to install Cargo separately.

### Creating a Project with Cargo

Let's create a new project using Cargo and look at how it differs from our
project in `hello_world`. Go back to your projects directory (or wherever you
decided to put your code):

```bash
$ cd ~/projects
```

And then run:

```bash
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

We passed the `--bin` argument to `cargo new` because our goal is to make an
executable application, as opposed to a library. Executables are often called
*binaries* (as in `/usr/bin`, if you’re on a Unix system). We've given `hello_cargo` as the
name for our project, and Cargo creates its files in a directory
of the same name that we can then go into.

If we list the files in the `hello_cargo` directory, we can see that Cargo has
generated two files and one directory for us: a `Cargo.toml` and a *src*
directory with a *main.rs* file inside. It has also initialized a new `git`
repository in the `hello_cargo` directory for us; you can change this to use a
different version control system, or no version control system, by using the
`--vcs` flag.

Open up `Cargo.toml` in your text editor of choice. It should look something
like this:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

This file is in the *[TOML]* (Tom's Obvious, Minimal Language) format. TOML is
similar to INI but has some extra goodies and is used as Cargo’s
configuration format.

[TOML]: https://github.com/toml-lang/toml

The first line, `[package]`, is a section heading that indicates that the
following statements are configuring a package. As we add more information to
this file, we’ll add other sections.

The next three lines set the three bits of configuration that Cargo needs to
see in order to know that it should compile your program: its name, what
version it is, and who wrote it. Cargo gets your name and email information
from your environment. If it’s not correct, go ahead and fix that and save the
file.

The last line, `[dependencies]`, is the start of a section for you to list any
crates that your project will depend on so that Cargo knows to download and
compile those too. We won't need any other crates for this project, but we will
in the guessing game tutorial in the next chapter.

Now let's look at `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a "Hello World!" for you, just like the one we wrote
earlier! So that part is the same. The differences between our previous project
and the project generated by Cargo that we've seen so far are:

1. Our code goes in the `src` directory
2. The top level contains a `Cargo.toml` configuration file

Cargo expects your source files to live inside the *src* directory so that the
top-level project directory is just for READMEs, license information,
configuration files, and anything else not related to your code. In this way,
using Cargo helps you keep your projects nice and tidy. There's a place for
everything, and everything is in its place.

If you started a project that doesn't use Cargo, as we did with our project in
the `hello_world` directory, you can convert it to a project that does use
Cargo by moving your code into the `src` directory and creating an appropriate
`Cargo.toml`.

### Building and Running a Cargo Project

Now let's look at what's different about building and running your Hello World
program through Cargo! To do so, enter the following commands:

```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///home/yourname/projects/hello_cargo)
```

This should have created an executable file in `target/debug/hello_cargo` (or `target/debug/hello_cargo.exe` on Windows), which you can run with this command:

```bash
$ ./target/debug/hello_cargo # or ./target/debug/hello_cargo.exe on Windows
Hello, world!
```
<!-- Windows needs to use \ without the . -->

Bam! If all goes well, `Hello, world!` should print to the terminal once more.

Running `cargo build` for the first time also causes Cargo to create a new file
at the top level called *Cargo.lock*, which looks like this:

```toml
[root]
name = "hello_cargo"
version = "0.1.0"
```

Cargo uses the *Cargo.lock* file to keep track of dependencies in your
application. This project doesn't have dependencies, so the file is a bit
sparse. Realistically, you won't ever need to touch this file yourself; just
let Cargo handle it.

We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can actually do both in one step with
`cargo run` as follows:

```bash
$ cargo run
     Running `target/debug/hello_cargo`
Hello, world!
```

Notice that this time, we didn't see the output telling us that Cargo was compiling
`hello_cargo`. Cargo figured out that the files haven’t changed, so it just ran
the binary. If you had modified your source code, Cargo would have rebuilt the
project before running it, and you would have seen something like this:

```bash
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///home/yourname/projects/hello_cargo)
     Running `target/debug/hello_cargo`
Hello, world!
```

So a few more differences we've now seen:

3. Instead of using `rustc`, build a project using `cargo build` (or build and run it in one step with `cargo run`)
4. Instead of the result of the build being put in the same directory as our code, Cargo will put it in the `target/debug` directory.

### Building for Release

When your project is finally ready for release, you can use `cargo build
--release` to compile your project with optimizations. This will create an
executable in `target/release` instead of `target/debug`. These optimizations
make your Rust code run faster, but turning them on makes your program take
longer to compile. This is why there are two different profiles: one for
development when you want to be able to rebuild quickly and often, and one for
building the final program you’ll give to a user that won't be rebuilt and
that we want to run as fast as possible. If you're benchmarking the running
time of your code, be sure to run `cargo build --release` and benchmark with
the executable in `target/release`.

### Cargo as Convention

With simple projects, Cargo doesn't provide a whole lot of value over just
using `rustc`, but it will prove its worth as you continue. With complex
projects composed of multiple crates, it’s much easier to let Cargo coordinate
the build. With Cargo, you can just run `cargo build`, and it should work the
right way. Even though this project is simple, it now uses much of the real
tooling you’ll use for the rest of your Rust career. In fact, you can get
started with virtually all Rust projects you want to work
on with the following commands:

```bash
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

> Note: If you want to look at Cargo in more detail, check out the official
[Cargo guide], which covers all of its features.

[Cargo guide]: http://doc.crates.io/guide.html
