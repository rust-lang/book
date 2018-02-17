
[TOC]

# Introduction

Welcome to “The Rust Programming Language,” an introductory book about Rust.

Rust is a programming language that helps you write faster, more reliable
software. High-level ergonomics and low-level control are often at odds with
each other in programming language design; Rust stands to challenge that.
Through balancing powerful technical capacity and a great developer experience,
Rust gives you the option to control low-level details (such as memory usage)
without all the hassle traditionally associated with such control.

## Who Rust is For

Rust is great for many people for a variety of reasons. Let’s discuss a few of
the most important groups.

### Teams of Developers

Rust is proving to be a productive tool for collaborating among large teams of
developers with varying levels of systems programming knowledge. Low-level code
is prone to a variety of subtle bugs, which in most other languages can only be
caught through extensive testing and careful code review by experienced
developers. In Rust, the compiler plays a gatekeeper role by refusing to
compile code with these kinds of bugs--including concurrency bugs. By working
alongside the compiler, the team can spend more time focusing on the logic of
the program rather than chasing down bugs.

Rust also brings contemporary developer tools to the systems programming world:

* Cargo, the included dependency manager and build tool, makes adding,
  compiling, and managing dependencies painless and consistent across the Rust
  ecosystem.
* Rustfmt ensures a consistent coding style across developers.
* The Rust Language Server powers IDE integration for code completion and
  inline error messages.

By using these and other tools in the Rust ecosystem, developers can be
productive while writing systems-level code.

### Students

Rust is for students and people who are interested in learning about systems
concepts. Many people have learned about topics like operating systems
development through Rust. The community is happy to answer student questions.
Through efforts such as this book, the Rust teams want to make systems concepts
more accessible to more people, especially those getting started with
programming.

### Companies

Rust is used in production by hundreds of companies, large and small, for a
variety of tasks, such as command line tools, web services, DevOps tooling,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatics, search engines, internet of things applications, machine
learning, and even major parts of the Firefox web browser.

### Open Source Developers

Rust is for people who want to build the Rust programming language, community,
developer tools, and libraries. We’d love for you to contribute to the Rust
language.

### People Who Value Speed and Stability

By speed, we mean both the speed of the programs that Rust lets you create and
the speed at which Rust lets you write them. The Rust compiler’s checks ensure
stability through feature additions and refactoring, as opposed to brittle
legacy code in languages without these checks that developers are afraid to
modify. By striving for zero-cost abstractions, higher level features that
compile to lower level code as fast as code written manually, Rust endeavors to
make safe code be fast code as well.

This isn’t a complete list of everyone the Rust language hopes to support, but
these are some of the biggest stakeholders. Overall, Rust’s greatest ambition
is to take trade-offs that have been accepted by programmers for decades and
eliminate the dichotomy. Safety *and* productivity. Speed *and* ergonomics.
Give Rust a try, and see if its choices work for you.

## Who This Book is For

This book assumes that you’ve written code in some other programming language,
but doesn’t make any assumptions about which one. We’ve tried to make the
material broadly accessible to those from a wide variety of programming
backgrounds. We don’t spend a lot of time talking about what programming *is*
or how to think about it; someone new to programming entirely would be better
served by reading a book specifically providing an introduction to programming.

## How to Use This Book

This book generally assumes that you’re reading it front-to-back, that is,
later chapters build on top of concepts in earlier chapters, and earlier
chapters may not dig into details on a topic, revisiting the topic in a later
chapter.

There are two kinds of chapters in this book: concept chapters, and project
chapters. In concept chapters, you’ll learn about an aspect of Rust. In the
project chapters, we’ll build small programs together, applying what we’ve
learned so far. Chapters 2, 12, and 20 are project chapters; the rest are
concept chapters.

Additionally, Chapter 2 is a hands-on introduction to Rust as a language. We’ll
cover concepts at a high level, and later chapters will go into them in detail.
If you’re the kind of person who likes to get their hands dirty right away,
Chapter 2 is great for that. If you’re *really* that kind of person, you may
even wish to skip over Chapter 3, which covers features that are very similar
to other programming languages, and go straight to Chapter 4 to learn about
Rust’s ownership system. By contrast, if you’re a particularly meticulous
learner who prefers to learn every detail before moving onto the next, you may
want to skip Chapter 2 and go straight to Chapter 3.

In the end, there’s no wrong way to read a book: if you want to skip ahead, go
for it! You may have to jump back if you find things confusing. Do whatever
works for you.

An important part of the process of learning Rust is learning how to read the
error messages that the compiler gives you. As such, we’ll be showing a lot of
code that doesn’t compile, and the error message the compiler will show you in
that situation. As such, if you pick a random example, it may not compile!
Please read the surrounding text to make sure that you didn’t happen to pick
one of the in-progress examples.

Finally, there are some appendices. These contain useful information about the
language in a more reference-like format.

## Contributing to the Book

This book is open source. If you find an error, please don’t hesitate to file
an issue or send a pull request on GitHub at
*https://github.com/rust-lang/book*. Please see CONTRIBUTING.md at
*https://github.com/rust-lang/book/blob/master/CONTRIBUTING.md* for more
details.

## Installation

The first step to using Rust is to install it. You’ll need an internet
connection to run the commands in this chapter, as we’ll be downloading Rust
from the internet. We’ll actually be installing Rust using `rustup`, a
command-line tool for managing Rust versions and associated tools.

The following steps will install the latest stable version of the Rust
compiler. The examples and output shown in this book used stable Rust 1.21.0.
Due to Rust’s stability guarantees, which we’ll discuss further in the “How
Rust is Made” section later in this chapter, all of the examples that compile
will continue to compile with newer versions of Rust. The output may differ
slightly as error messages and warnings are often improved. In other words, the
newer, stable version of Rust you will install with these steps should work as
expected with the content of this book.

> #### Command Line Notation
>
> We’ll be showing off a number of commands using a terminal, and those lines
> all start with `$`. You don’t need to type in the `$` character; they are
> there to indicate the start of each command. You’ll see many tutorials and
> examples around the web that follow this convention: `$` for commands run as
> a regular user, and `#` for commands you should be running as an
> administrator. Lines that don’t start with `$` are typically showing the
> output of the previous command. Additionally, PowerShell specific examples
> will use `>` rather than `$`.

### Installing Rustup on Linux or Mac

If you’re on Linux or a Mac, 99% of what you need to do is open a terminal and
type this:

```
$ curl https://sh.rustup.rs -sSf | sh
```

This will download a script and start the installation of the `rustup` tool,
which installs the latest stable version of Rust. You may be prompted for your
password. If it all goes well, you’ll see this appear:

```
Rust is installed now. Great!
```

Of course, if you distrust using `curl URL | sh` to install software, you can
download, inspect, and run the script however you like.

The installation script automatically adds Rust to your system PATH after your
next login. If you want to start using Rust right away, run the following
command in your shell:

```
$ source $HOME/.cargo/env
```

Alternatively, add the following line to your `~/.bash_profile`:

```
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Finally, you’ll need a linker of some kind. You likely have one installed. If
not, when you compile a Rust program, you’ll get errors that a linker could not
be executed. Check your platform’s documentation for how to install a C
compiler; they usually come with the correct linker as well, given that C needs
one. You may want to install a C compiler regardless of your need for only a
linker; some common Rust packages depend on C code and will need a C compiler
too.

### Installing Rustup on Windows

On Windows, go to https://www.rust-lang.org/en-US/install.html at
*https://www.rust-lang.org/en-US/install.html* and follow the instructions.
You’ll also need the C++ build tools for Visual Studio 2013 or later. The
easiest way to acquire the build tools is by installing Build Tools for Visual
Studio 2017 at *https://www.visualstudio.com/downloads/* which provides only
the Visual C++ build tools. Alternately, you can install at
*https://www.visualstudio.com/downloads/* Visual Studio 2017, Visual Studio
2015, or Visual Studio 2013 and during installation select the desktop
development with C++ workload.

The rest of this book will use commands that work in both `cmd.exe` and
PowerShell. If there are specific differences, we’ll explain which to use.

### Custom Installations Without Rustup

If you have reasons for preferring not to use `rustup`, please see the Rust
installation page at *https://www.rust-lang.org/install.html* for other options.

### Updating

Once you have Rust installed via `rustup`, updating to the latest version is
easy. From your shell, run the update script:

```
$ rustup update
```

### Uninstalling

Uninstalling Rust and Rustup is as easy as installing them. From your shell,
run the uninstall script:

```
$ rustup self uninstall
```

### Troubleshooting

To check that you have Rust installed, you can open up a shell and type this:

```
$ rustc --version
```

You should see the version number, commit hash, and commit date in a format
similar to this for the latest stable version at the time you install:

```
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this, Rust has been installed successfully! Congrats!

If you don’t and you’re on Windows, check that Rust is in your `%PATH%` system
variable.

If it still isn’t working, there are a number of places where you can get help.
The easiest is the #rust IRC channel on irc.mozilla.org, which you can access
through Mibbit at
*http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust*. Go to that
address, and you’ll be chatting with other Rustaceans (a silly nickname we call
ourselves) who can help you out. Other great resources include the Users forum
at *https://users.rust-lang.org/* and Stack Overflow at
*http://stackoverflow.com/questions/tagged/rust*.

### Local Documentation

The installer also includes a copy of the documentation locally, so you can
read it offline. Run `rustup doc` to open the local documentation in your
browser.

Any time there’s a type or function provided by the standard library and you’re
not sure what it does or how to use it, use the API (Application Programming
Interface) documentation to find out!

## Hello, World!

Now that you have Rust installed, let’s write your first Rust program. It’s
traditional when learning a new language to write a little program to print the
text “Hello, world!” to the screen, and in this section, we’ll follow that
tradition.

> Note: This book assumes basic familiarity with the command line. Rust itself
> makes no specific demands about your editing, tooling, or where your code
> lives, so if you prefer an IDE (Integrated Development Environment) to the
> command line, feel free to use your favorite IDE. Many IDEs now have some
> degree of Rust support; check the IDE’s documentation for details. Enabling
> great IDE support has been a recent focus of the Rust team, and progress
> has been made rapidly on that front!

### Creating a Project Directory

First, make a directory to put your Rust code in. Rust doesn’t care where your
code lives, but for this book, we’d suggest making a *projects* directory in
your home directory and keeping all your projects there. Open a terminal and
enter the following commands to make a *projects* directory and a directory
inside that for the “Hello, world!” project:

Linux and Mac:

```
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD:

```
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

Windows PowerShell:

```
> mkdir $env:USERPROFILE\projects
> cd $env:USERPROFILE\projects
> mkdir hello_world
> cd hello_world
```

### Writing and Running a Rust Program

Next, make a new source file and call it *main.rs*. Rust files always end with
the *.rs* extension. If you’re using more than one word in your filename, use
an underscore to separate them. For example, you’d use *hello_world.rs* rather
than *helloworld.rs*.

Now open the *main.rs* file you just created, and enter the code shown in
Listing 1-1:

Filename: main.rs

```
fn main() {
    println!("Hello, world!");
}
```

Listing 1-1: A program that prints “Hello, world!”

Save the file, and go back to your terminal window. On Linux or macOS, enter
the following commands:

```
$ rustc main.rs
$ ./main
Hello, world!
```

On Windows, use `.\main.exe` instead of `./main`.

```
> rustc main.rs
> .\main.exe
Hello, world!
```

Regardless of your operating system, you should see the string `Hello, world!`
print to the terminal. If you did, then congratulations! You’ve officially
written a Rust program. That makes you a Rust programmer! Welcome!

### Anatomy of a Rust Program

Now, let’s go over what just happened in your “Hello, world!” program in
detail. Here’s the first piece of the puzzle:

```
fn main() {

}
```

These lines define a *function* in Rust. The `main` function is special: it’s
the first code that is run for every executable Rust program. The first line
declares a function named `main` that has no parameters and returns nothing. If
there were parameters, their names would go inside the parentheses, `(` and `)`.

Also note that the function body is wrapped in curly brackets, `{` and `}`.
Rust requires these around all function bodies. It’s considered good style to
put the opening curly bracket on the same line as the function declaration,
with one space in between.

> At the time of writing, an automatic formatter, `rustfmt`, is under
> development. If you’d like to stick to a standard style across Rust projects,
> `rustfmt` is a tool that will format your code in a particular style. The
> plan is to eventually include it with the standard Rust distribution, like
> `rustc`, so depending on when you read this book, you may have it already
> installed! Check the online documentation for more details.

Inside the `main` function, we have this code:

```
    println!("Hello, world!");
```

This line does all of the work in this little program: it prints text to the
screen. There are a number of details to notice here. The first is that Rust
style is to indent with four spaces, not a tab.

The second important part is `println!`. This is calling a Rust *macro*, which
is how metaprogramming is done in Rust. If it were calling a function instead,
it would look like this: `println` (without the `!`). We’ll discuss Rust macros
in more detail in Appendix D, but for now you just need to know that when you
see a `!` that means that you’re calling a macro instead of a normal function.

> ### Why `println!` is a Macro
>
> There are multiple reasons why `println!` is a macro rather than a function,
> and we haven’t really explained Rust yet, so it’s not exactly obvious. Here
> are the reasons:
>
> * The string passed to `println!` can have formatting specifiers in it,
>   and those are checked at compile-time.
> * Rust functions can only have a fixed number of arguments, but `println!`
>   (and macros generally) can take a variable number.
> * The formatting specifiers can have named arguments, which Rust functions
>   cannot.
> * It implicitly takes its arguments by reference even when they’re passed
>   by value.
>
> If none of this makes sense, don’t worry about it. We’ll cover these concepts
> in more detail later.

Next is `"Hello, world!"` which is a *string*. We pass this string as an
argument to `println!`, which prints the string to the screen. Easy enough!

The line ends with a semicolon (`;`). The `;` indicates that this expression is
over, and the next one is ready to begin. Most lines of Rust code end with a
`;`.

### Compiling and Running Are Separate Steps

In the “Writing and Running a Rust Program” section on page XX, we showed you
how to run a newly created program. We’ll break that process down and examine
each step now.

Before running a Rust program, you have to compile it. You can use the Rust
compiler by entering the `rustc` command and passing it the name of your source
file, like this:

```
$ rustc main.rs
```

If you come from a C or C++ background, you’ll notice that this is similar to
`gcc` or `clang`. After compiling successfully, Rust outputs a binary
executable.

On Linux, Mac, and PowerShell on Windows, you can see the executable by
entering the `ls` command in your shell as follows:

```
$ ls
main  main.rs
```

With CMD on Windows, you’d enter:

```
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

This shows we have two files: the source code, with the *.rs* extension, and
the executable (*main.exe* on Windows, *main* everywhere else). All that’s left
to do from here is run the *main* or *main.exe* file, like this:

```
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
grows, you’ll want to be able to manage all of the options your project has and
make it easy to share your code with other people and projects. Next, we’ll
introduce you to a tool called Cargo, which will help you write real-world Rust
programs.

## Hello, Cargo!

Cargo is Rust’s build system and package manager, and Rustaceans use Cargo to
manage their Rust projects because it makes a lot of tasks easier. For example,
Cargo takes care of building your code, downloading the libraries your code
depends on, and building those libraries. We call libraries your code needs
*dependencies*.

The simplest Rust programs, like the one we’ve written so far, don’t have any
dependencies, so right now, you’d only be using the part of Cargo that can take
care of building your code. As you write more complex Rust programs, you’ll
want to add dependencies, and if you start off using Cargo, that will be a lot
easier to do.

As the vast, vast majority of Rust projects use Cargo, we will assume that
you’re using it for the rest of the book. Cargo comes installed with Rust
itself, if you used the official installers as covered in the “Installation”
section. If you installed Rust through some other means, you can check if you
have Cargo installed by typing the following into your terminal:

```
$ cargo --version
```

If you see a version number, great! If you see an error like `command not
found`, then you should look at the documentation for your method of
installation to determine how to install Cargo separately.

### Creating a Project with Cargo

Let’s create a new project using Cargo and look at how it differs from our
project in `hello_world`. Go back to your projects directory (or wherever you
decided to put your code):

Linux, Mac, and PowerShell:

```
$ cd ~/projects
```

CMD for Windows:

```
> cd \d "%USERPROFILE%\projects"
```

And then on any operating system run:

```
$ cargo new hello_cargo --bin
$ cd hello_cargo
```

We passed the `--bin` argument to `cargo new` because our goal is to make an
executable application, as opposed to a library. Executables are binary
executable files often called just *binaries*. We’ve given `hello_cargo` as the
name for our project, and Cargo creates its files in a directory of the same
name that we can then go into.

If we list the files in the *hello_cargo* directory, we can see that Cargo has
generated two files and one directory for us: a *Cargo.toml* and a *src*
directory with a *main.rs* file inside. It has also initialized a new git
repository in the *hello_cargo* directory for us, along with a *.gitignore*
file. Git is a common version control system. You can change `cargo new` to use
a different version control system, or no version control system, by using the
`--vcs` flag. Run `cargo new --help` to see the available options.

Open up *Cargo.toml* in your text editor of choice. It should look similar to
the code in Listing 1-2:

Filename: Cargo.toml

```
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
```

Listing 1-2: Contents of *Cargo.toml* generated by `cargo new`

This file is in the *TOML* (Tom’s Obvious, Minimal Language) format. TOML is
used as Cargo’s configuration format.

The first line, `[package]`, is a section heading that indicates that the
following statements are configuring a package. As we add more information to
this file, we’ll add other sections.

The next three lines set the three bits of configuration that Cargo needs to
see in order to know that it should compile your program: its name, what
version it is, and who wrote it. Cargo gets your name and email information
from your environment. If it’s not correct, go ahead and fix that and save the
file.

The last line, `[dependencies]`, is the start of a section for you to list any
*crates* (which is what we call packages of Rust code) that your project will
depend on so that Cargo knows to download and compile those too. We won’t need
any other crates for this project, but we will in the guessing game tutorial in
Chapter 2.

Now let’s look at *src/main.rs*:

Filename: src/main.rs

```
fn main() {
    println!("Hello, world!");
}
```

Cargo has generated a “Hello World!” for you, just like the one we wrote in
Listing 1-1! So that part is the same. The differences between our previous
project and the project generated by Cargo that we’ve seen so far are:

- Our code goes in the *src* directory
- The top level contains a *Cargo.toml* configuration file

Cargo expects your source files to live inside the *src* directory so that the
top-level project directory is just for READMEs, license information,
configuration files, and anything else not related to your code. In this way,
using Cargo helps you keep your projects nice and tidy. There’s a place for
everything, and everything is in its place.

If you started a project that doesn’t use Cargo, as we did with our project in
the *hello_world* directory, you can convert it to a project that does use
Cargo by moving your code into the *src* directory and creating an appropriate
*Cargo.toml*.

### Building and Running a Cargo Project

Now let’s look at what’s different about building and running your Hello World
program through Cargo! To do so, enter the following commands:

```
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This creates an executable file in *target/debug/hello_cargo* (or
*target\\debug\\hello_cargo.exe* on Windows), which you can run with this
command:

```
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

Bam! If all goes well, `Hello, world!` should print to the terminal once more.

Running `cargo build` for the first time also causes Cargo to create a new file
at the top level called *Cargo.lock*. Cargo uses *Cargo.lock* to keep track of
the exact versions of dependencies used to build your project. This project
doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to
touch this file yourself; Cargo will manage its contents for you.

We just built a project with `cargo build` and ran it with
`./target/debug/hello_cargo`, but we can also use `cargo run` to compile and
then run:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Notice that this time, we didn’t see the output telling us that Cargo was
compiling `hello_cargo`. Cargo figured out that the files haven’t changed, so
it just ran the binary. If you had modified your source code, Cargo would have
rebuilt the project before running it, and you would have seen output like
this:

```
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Finally, there’s `cargo check`. This will quickly check your code to make sure
that it compiles, but not bother producing an executable:

```
$ cargo check
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? `cargo check` is often much faster than
`cargo build`, because Cargo can skip the entire step of producing the
executable. If we’re checking our work throughout the process of writing the
code, this will speed things up! As such, many Rustaceans run `cargo check` as
they write their program to make sure that it compiles, and then run `cargo
build` once they’re ready to give it a spin themselves.

So a few more differences we’ve now seen:

- Instead of using `rustc`, build a project using `cargo build` or
  `cargo check` (or build and run it in one step with `cargo run`).
- Instead of the result of the build being put in the same directory as our
  code, Cargo will put it in the *target/debug* directory.

The other advantage of using Cargo is that the commands are the same no matter
what operating system you’re on, so at this point we will no longer be
providing specific instructions for Linux and Mac versus Windows.

### Building for Release

When your project is finally ready for release, you can use `cargo build
--release` to compile your project with optimizations. This will create an
executable in *target/release* instead of *target/debug*. These optimizations
make your Rust code run faster, but turning them on makes your program take
longer to compile. This is why there are two different profiles: one for
development when you want to be able to rebuild quickly and often, and one for
building the final program you’ll give to a user that won’t be rebuilt and that
we want to run as fast as possible. If you’re benchmarking the running time of
your code, be sure to run `cargo build --release` and benchmark with the
executable in *target/release*.

### Cargo as Convention

With simple projects, Cargo doesn’t provide a whole lot of value over just
using `rustc`, but it will prove its worth as you continue. With complex
projects composed of multiple crates, it’s much easier to let Cargo coordinate
the build. With Cargo, you can just run `cargo build`, and it should work the
right way.

Even though the `hello_cargo` project is simple, it now uses much of the real
tooling you’ll use for the rest of your Rust career. In fact, you can get
started with virtually all Rust projects you want to work on with the following
commands to check out the code using Git, change into the project directory,
and build:

```
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```

If you want to look at Cargo in more detail, check out its documentation at
*https://doc.rust-lang.org/cargo/*, which covers all of its features.

## How Rust is Made and “Nightly Rust”

Before we dive into the language itself, we’d like to finish up the
introductory chapter by talking about how Rust is made, and how that affects
you as a Rust developer. We mentioned in the “Installation” section that the
output in this book was generated by stable Rust 1.21.0, but any examples that
compile should continue to compile in any stable version of Rust greater than
that. This section is to explain how we ensure this is true!

### Stability Without Stagnation

As a language, Rust cares a *lot* about the stability of your code. We want
Rust to be a rock-solid foundation you can build on, and if things were
constantly changing, that would be impossible. At the same time, if we can’t
experiment with new features, we may not find out important flaws until after
their release, when we can no longer change things.

Our solution to this problem is what we call “stability without stagnation” and
is the way we can change and improve Rust while making sure that using Rust
stays nice, stable, and boring.

Our guiding principle for Rust releases is this: you should never have to fear
upgrading to a new version of stable Rust. Each upgrade should be painless. At
the same time, the upgrade should bring you new features, fewer bugs, and
faster compile times.

### Choo, Choo! Release Channels and Riding the Trains

Rust development operates on a *train schedule*. That is, all development is
done on the `master` branch of the Rust repository. Releases follow a software
release train model, which has been used by Cisco IOS and other software
projects. There are three *release channels* for Rust:

* Nightly
* Beta
* Stable

Most Rust developers primarily use the stable channel, but those who want to
try out experimental new features may use nightly or beta.

Here’s an example of how the development and release process works: let’s
assume that the Rust team is working on the release of Rust 1.5. That release
happened in December of 2015, but it will provide us with realistic version
numbers. A new feature is added to Rust: a new commit lands on the `master`
branch. Each night, a new nightly version of Rust is produced. Every day is a
release day, and these releases are created by our release infrastructure
automatically. So as time passes, our releases look like this, once a night:

```
nightly: * - - * - - *
```

Every six weeks, it’s time to prepare a new release! The `beta` branch of the
Rust repository branches off from the `master` branch used by nightly. Now,
there are two releases:

```
nightly: * - - * - - *
                     |
beta:                *
```

Most Rust users do not use beta releases actively, but test against beta in
their CI system to help Rust discover possible regressions. In the meantime,
there’s still a nightly release every night:

```
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Let’s say a regression is found. Good thing we had some time to test the beta
release before the regression snuck into a stable release! The fix is applied
to `master`, so that nightly is fixed, and then the fix is backported to the
`beta` branch, and a new release of beta is produced:

```
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Six weeks after the first beta was created, it’s time for a stable release! The
`stable` branch is produced from the `beta` branch:

```
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Hooray! Rust 1.5 is done! However, we’ve forgotten one thing: because the six
weeks have gone by, we also need a new beta of the *next* version of Rust, 1.6.
So after `stable` branches off of `beta`, the next version of `beta` branches
off of `nightly` again:

```
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

This is called the “train model” because every six weeks, a release “leaves the
station”, but still has to take a journey through the beta channel before it
arrives as a stable release.

Rust releases every six weeks, like clockwork. If you know the date of one Rust
release, you can know the date of the next one: it’s six weeks later. A nice
aspect of having releases scheduled every six weeks is that the next train is
coming soon. If a feature happens to miss a particular release, there’s no need
to worry: another one is happening in a short time! This helps reduce pressure
to sneak possibly unpolished features in close to the release deadline.

Thanks to this process, you can always check out the next build of Rust and
verify for yourself that it’s easy to upgrade to: if a beta release doesn’t
work as expected, you can report it to the team and get it fixed before the
next stable release happens! Breakage in a beta release is relatively rare, but
`rustc` is still a piece of software, and bugs do exist.

### Unstable Features

There’s one more catch with this release model: unstable features. Rust uses a
technique called “feature flags” to determine what features are enabled in a
given release. If a new feature is under active development, it lands on
`master`, and therefore, in nightly, but behind a *feature flag*. If you, as a
user, wish to try out the work-in-progress feature, you can, but you must be
using a nightly release of Rust and annotate your source code with the
appropriate flag to opt in.

If you’re using a beta or stable release of Rust, you can’t use any feature
flags. This is the key that allows us to get practical use with new features
before we declare them stable forever. Those who wish to opt into the bleeding
edge can do so, and those who want a rock-solid experience can stick with
stable and know that their code won’t break. Stability without stagnation.

This book only contains information about stable features, as in-progress
features are still changing, and surely they’ll be different between when this
book was written and when they get enabled in stable builds. You can find
documentation for nightly-only features online.

### Rustup and the Role of Rust Nightly

Rustup makes it easy to change between different release channels of Rust, on a
global or per-project basis. By default, you’ll have stable Rust installed. To
install nightly, for example:

```
$ rustup install nightly
```

You can see all of the *toolchains* (releases of Rust and associated
components) you have installed with `rustup` as well. Here’s an example on one
of your authors’ computers:

```
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

As you can see, the stable toolchain is the default. Most Rust users use stable
most of the time. You might want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override` in that project’s directory to set the
nightly toolchain as the one `rustup` should use when you’re in that directory:

```
$ cd ~/projects/needs-nightly
$ rustup override add nightly
```

Now, every time you call `rustc` or `cargo` inside of
*~/projects/needs-nightly*, `rustup` will make sure that you are using nightly
Rust, rather than your default of stable Rust. This comes in handy when you
have a lot of Rust projects!

### The RFC Process and Teams

So how do you learn about these new features? Rust’s development model follows
a *Request For Comments (RFC) process*. If you’d like an improvement in Rust,
you can write up a proposal, called an RFC.

Anyone can write RFCs to improve Rust, and the proposals are reviewed and
discussed by the Rust team, which is comprised of many topic subteams. There’s
a full list of the teams on Rust’s
website at *https://www.rust-lang.org/en-US/team.html*, which includes teams for
each area of the project: language design, compiler implementation,
infrastructure, documentation, and more. The appropriate team reads the
proposal and the comments, writes some comments of their own, and eventually,
there’s consensus to accept or reject the feature.

If the feature is accepted, an issue is opened on the Rust repository, and
someone can implement it. The person who implements it very well may not be the
person who proposed the feature in the first place! When the implementation is
ready, it lands on the `master` branch behind a feature gate, as we discussed
in the “Unstable Features” section.

After some time, once Rust developers who use nightly releases have been able
to try out the new feature, team members will discuss the feature, how it’s
worked out on nightly, and decide if it should make it into stable Rust or not.
If the decision is to move forward, the feature gate is removed, and the
feature is now considered stable! It rides the trains into a new stable release
of Rust.

## Summary

You’re already off to a great start on your Rust journey! In this chapter,
you’ve:

* Learned what makes Rust unique
* Installed the latest stable version of Rust
* Written a “Hello, world!” program using both `rustc` directly and using
  the conventions of `cargo`
* Found out about how Rust is developed

This is a great time to build a more substantial program, to get used to
reading and writing Rust code. In the next chapter, we’ll build a guessing game
program. If you’d rather start by learning about how common programming
concepts work in Rust, see Chapter 3.
