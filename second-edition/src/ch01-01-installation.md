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

```text
$ curl https://sh.rustup.rs -sSf | sh
```

This will download a script and start the installation of the `rustup` tool,
which installs the latest stable version of Rust. You may be prompted for your
password. If it all goes well, you’ll see this appear:

```text
Rust is installed now. Great!
```

Of course, if you distrust using `curl URL | sh` to install software, you can
download, inspect, and run the script however you like.

The installation script automatically adds Rust to your system PATH after your
next login. If you want to start using Rust right away, run the following
command in your shell:

```text
$ source $HOME/.cargo/env
```

Alternatively, add the following line to your `~/.bash_profile`:

```text
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

On Windows, go to [https://www.rust-lang.org/en-US/install.html][install] and
follow the instructions. You’ll also need the C++ build tools for Visual Studio
2013 or later. The easiest way to acquire the build tools is by installing
[Build Tools for Visual Studio 2017][visualstudio] which provides only the
Visual C++ build tools. Alternately, you can [install][visualstudio] Visual
Studio 2017, Visual Studio 2015, or Visual Studio 2013 and during installation
select the desktop development with C++ workload.

[install]: https://www.rust-lang.org/en-US/install.html
[visualstudio]: https://www.visualstudio.com/downloads/

The rest of this book will use commands that work in both `cmd.exe` and
PowerShell. If there are specific differences, we’ll explain which to use.

### Custom Installations Without Rustup

If you have reasons for preferring not to use `rustup`, please see [the Rust
installation page](https://www.rust-lang.org/install.html) for other options.

### Updating

Once you have Rust installed via `rustup`, updating to the latest version is
easy. From your shell, run the update script:

```text
$ rustup update
```

### Uninstalling

Uninstalling Rust and Rustup is as easy as installing them. From your shell,
run the uninstall script:

```text
$ rustup self uninstall
```

### Troubleshooting

To check that you have Rust installed, you can open up a shell and type this:

```text
$ rustc --version
```

You should see the version number, commit hash, and commit date in a format
similar to this for the latest stable version at the time you install:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this, Rust has been installed successfully! Congrats!

If you don’t and you’re on Windows, check that Rust is in your `%PATH%` system
variable.

If it still isn’t working, there are a number of places where you can get help.
The easiest is [the #rust IRC channel on irc.mozilla.org][irc]<!-- ignore -->,
which you can access through [Mibbit][mibbit]. Go to that address, and you’ll
be chatting with other Rustaceans (a silly nickname we call ourselves) who can
help you out. Other great resources include [the Users forum][users] and [Stack
Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Local Documentation

The installer also includes a copy of the documentation locally, so you can
read it offline. Run `rustup doc` to open the local documentation in your
browser.

Any time there’s a type or function provided by the standard library and you’re
not sure what it does or how to use it, use the API (Application Programming
Interface) documentation to find out!
