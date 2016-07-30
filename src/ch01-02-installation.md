# Installation

The first step to using Rust is to install it. Generally speaking, you’ll need
an internet connection to run the commands in this chapter, as we’ll be
downloading Rust from the internet.

We’ll be showing off a number of commands using a terminal, and those lines all
start with `$`. You don't need to type in the `$`s, they are there to indicate
the start of each command. You’ll see many tutorials and examples around the web
that follow this convention: `$` for commands run as a regular user, and `#`
for commands you should be running as an administrator.

## Installing on Linux or Mac

If you're on Linux or a Mac, all you need to do is open a terminal and type this:

```bash
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

This will download a script, and start the installation. You may be prompted for your password.
If it all goes well, you’ll see this appear:

```text
    Rust is ready to roll.
```


## Installing on Windows

If you're on Windows, please download the appropriate [installer][install-page].

[install-page]: https://www.rust-lang.org/install.html

## Uninstalling

Uninstalling Rust is as easy as installing it. On Linux or Mac, just run
the uninstall script:

```bash
$ sudo /usr/local/lib/rustlib/uninstall.sh
```

If you used the Windows installer, you can re-run the `.msi` and it will give you
an uninstall option.

## Troubleshooting

If you've got Rust installed, you can open up a shell, and type this:

```bash
$ rustc --version
```

You should see the version number, commit hash, and commit date.

If you do, Rust has been installed successfully! Congrats!

If you don't and you're on Windows, check that Rust is in your `%PATH%` system
variable. If it isn't, run the installer again, select "Change" on the "Change,
repair, or remove installation" page and ensure "Add to PATH" is checked.

If not, there are a number of places where you can get help. The easiest is
[the #rust IRC channel on irc.mozilla.org][irc], which you can access through
[Mibbit][mibbit]. Click that link, and you'll be chatting with other Rustaceans
(a silly nickname we call ourselves) who can help you out. Other great resources
include [the user’s forum][users], and [Stack Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

## Local documentation

This installer also installs a copy of the documentation locally, so you can
read it offline. On UNIX systems, `/usr/local/share/doc/rust` is the location.
On Windows, it's in a `share/doc` directory, inside the directory to which Rust
was installed.
