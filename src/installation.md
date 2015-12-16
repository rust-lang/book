# Installation

The first step to using Rust is to install it. Generally speaking, you’ll need
an Internet connection to run the commands in this chapter, as we’ll be
downloading Rust from the internet.

We’ll be showing off a number of commands using a terminal, and those lines all
start with `$`. We don't need to type in the `$`s, they are there to indicate
the start of each command. We’ll see many tutorials and examples around the web
that follow this convention: `$` for commands run as our regular user, and `#`
for commands we should be running as an administrator.

## Mac dependencies (for package or source install)

XCode command line developer tools have dependencies needed to run rust. You can 
install XCode from the AppStore, and from there install the command line tools 
with `xcode-select --install`. Alternatively, you can download XCode or just the 
cli tools from developer.apple.com/resources with a developer account.

## Installing on Linux or Mac

If we're on Linux or a Mac, all we need to do is open a terminal and type this:

```bash
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

This will download a script, and stat the installation. If it all goes well,
you’ll see this appear:

```text
Welcome to Rust.

This script will download the Rust compiler and its package manager, Cargo, and
install them to /usr/local. You may install elsewhere by running this script
with the --prefix=<path> option.

The installer will run under ‘sudo’ and may ask you for your password. If you do
not want the script to run ‘sudo’ then pass it the --disable-sudo flag.

You may uninstall later by running /usr/local/lib/rustlib/uninstall.sh,
or by running this script again with the --uninstall flag.

Continue? (y/N) 
```

From here, press `y` for ‘yes’, and then follow the rest of the prompts.

## Installing on Windows

If you're on Windows, please download the appropriate [installer][install-page].

[install-page]: https://www.rust-lang.org/install.html

## Uninstalling

Uninstalling Rust is as easy as installing it. On Linux or Mac, just run
the uninstall script:

```bash
$ sudo /usr/local/lib/rustlib/uninstall.sh
```

If we used the Windows installer, we can re-run the `.msi` and it will give us
an uninstall option.

## Troubleshooting

If we've got Rust installed, we can open up a shell, and type this:

```bash
$ rustc --version
```

You should see the version number, commit hash, and commit date.

If you do, Rust has been installed successfully! Congrats!

If you don't and you're on Windows, check that Rust is in your %PATH% system
variable. If it isn't, run the installer again, select "Change" on the "Change,
repair, or remove installation" page and ensure "Add to PATH" is checked.

If not, there are a number of places where we can get help. The easiest is
[the #rust IRC channel on irc.mozilla.org][irc], which we can access through
[Mibbit][mibbit]. Click that link, and we'll be chatting with other Rustaceans
(a silly nickname we call ourselves) who can help us out. Other great resources
include [the user’s forum][users], and [Stack Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

This installer also installs a copy of the documentation locally, so we can
read it offline. On UNIX systems, `/usr/local/share/doc/rust` is the location.
On Windows, it's in a `share/doc` directory, inside the directory to which Rust
was installed.
