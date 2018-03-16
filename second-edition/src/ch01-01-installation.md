## Installation

The first step to using Rust is to install it. We’ll download Rust through
`rustup`, a command-line tool for managing Rust versions and associated tools.
For this you’ll need an internet connection.

The following steps will install the latest stable version of the Rust
compiler. The examples and output shown in this book all use stable Rust
1.21.0. Rust’s stability guarantees ensure that all of the examples in the book
that compile will continue to compile with newer versions of Rust. The output
may differ slightly between versions, as error messages and warnings are often
improved. In other words, any newer, stable version of Rust you will install
with these steps should work as expected with the content of this book.

<!-- PROD: Start Box -->

> #### Command Line Notation
>
> In this chapter and throughout the book we’ll be showing some commands used
> in the terminal. Lines that should be entered in a terminal all start with
> `$`. You don’t need to type in the `$` character, it is simply there to
> indicate the start of each command. Many tutorials use this convention: `$`
> for commands run as a regular user, and `#` for commands you should be
> running as an administrator. Lines that don’t start with `$` are typically
> showing the output of the previous command. Additionally, PowerShell specific
> examples will use `>` rather than `$`.

<!-- PROD: End box -->

### Installing Rustup on Linux or Mac

If you’re on Linux or a Mac, open a terminal and enter the following command:

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
next login. If you want to start using Rust right away instead of restarting
your terminal, run the following command in your shell to add Rust to your
system PATH manually:

<!-- what does this command do? Do you mean instead of logging out and logging
in, enter the following? -->
<!-- It runs a script that adds Rust to your system PATH manually. I've
clarified that yes, this is instead of logging out and back in to your
terminal. /Carol -->

```text
$ source $HOME/.cargo/env
```

Alternatively, you can add the following line to your `~/.bash_profile`:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Finally, you’ll need a linker of some kind. It’s likely you already have one
installed, but if you try to compile a Rust program and get errors telling you
that a linker could not be executed, you’ll need to install one. You can
install a C compiler, as that will usually come with the correct linker. Check
your platform’s documentation for how to install a C compiler. Some common Rust
packages depend on C code and will need a C compiler too, so it may be worth
installing one now regardless.

### Installing Rustup on Windows

On Windows, go to [https://www.rust-lang.org/en-US/install.html][install] and
follow the instructions for installing Rust. At some point in the installation
you’ll receive a message telling you you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2017][visualstudio], found in the Other
Tools and Frameworks section.

[install]: https://www.rust-lang.org/en-US/install.html
[visualstudio]: https://www.visualstudio.com/downloads/

The rest of this book will use commands that work in both `cmd.exe` and
PowerShell. If there are specific differences, we’ll explain which to use.

### Custom Installations Without Rustup

If you have reasons for preferring not to use `rustup`, please see [the Rust
installation page](https://www.rust-lang.org/install.html) for other options.

### Updating and Uninstalling

Once you have Rust installed via `rustup`, updating to the latest version is
easy. From your shell, run the update script:

```text
$ rustup update
```

To uninstall Rust and `rustup`, from your shell, run the uninstall script:

```text
$ rustup self uninstall
```

### Troubleshooting

To check whether you have Rust installed correctly, open up a shell and enter:

```text
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest
stable version at the time you install in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this, Rust has been installed successfully! Congrats!

If you don’t and you’re on Windows, check that Rust is in your `%PATH%` system
variable.

If that’s all correct and Rust still isn’t working, there are a number of
places you can get help. The easiest is [the #rust IRC channel on
irc.mozilla.org][irc]<!-- ignore -->, which you can access through
[Mibbit][mibbit]. Go to that address, and you’ll be chatting with other
Rustaceans (a silly nickname we call ourselves) who can help you out. Other
great resources include [the Users forum][users] and [Stack
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
