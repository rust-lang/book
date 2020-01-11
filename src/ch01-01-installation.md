## Installation

The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.

> Note: If you prefer not to use `rustup` for some reason, please see [the Rust
> installation page](https://www.rust-lang.org/tools/install) for other options.

The following steps install the latest stable version of the Rust compiler.
Rust’s stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions, because Rust often improves error messages
and warnings. In other words, any newer, stable version of Rust you install
using these steps should work as expected with the content of this book.

> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type in the `$` character; it indicates the start of each
> command. Lines that don’t start with `$` typically show the output of the
> previous command. Additionally, PowerShell-specific examples will use `>`
> rather than `$`.

### Installing `rustup` on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:

```text
Rust is installed now. Great!
```

If you prefer, feel free to download the script and inspect it before running
it.

The installation script automatically adds Rust to your system PATH after your
next login. If you want to start using Rust right away instead of restarting
your terminal, run the following command in your shell to add Rust to your
system PATH manually:

```text
$ source $HOME/.cargo/env
```

Alternatively, you can add the following line to your *~/.bash_profile*:

```text
$ export PATH="$HOME/.cargo/bin:$PATH"
```

Additionally, you’ll need a linker of some kind. It’s likely one is already
installed, but when you try to compile a Rust program and get errors indicating
that a linker could not execute, that means a linker isn’t installed on your
system and you’ll need to install one manually. C compilers usually come with
the correct linker. Check your platform’s documentation for how to install a C
compiler. Also, some common Rust packages depend on C code and will need a C
compiler. Therefore, it might be worth installing one now.

### Installing `rustup` on Windows

On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
receive a message explaining that you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2019][visualstudio]. The tools are in
the Other Tools and Frameworks section.

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2019

The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.

#### VS Build Tools Caveats

When selecting the Build Tools only instalation, the Rust tooling will work as
expected, but you might face a couple of issues when compiling `*-sys` 
dependencies.

`*-sys` dependencies often link to C/C++ libraries as part of their build 
system, and for that, a set of environment variables and paths should be setup.

Selecting only the Build tools instead of a complete installation of Visual Studio
installation might not configure everything properly. Here are some indicators
of problems on the development environment setup and some suggestion to fix them.

<details>
 <summary><code>error MSB4019: The imported project "C:\Microsoft.Cpp.Default.props" was not found. Confirm that the path in the &lt;Import&gt; declaration is correct, and that the file exists on disk.</code></summary>
  This error indicates that the environent is not properly setup, and you have a couple of of options to try to fix it:

- [Easiest] Open the `Developer Command Prompt for VS 2019` application to compile the project. This terminal will setup all the needed environment variables to let it compile.
- Setup the environment variables (eg: `PATH`, `LIB`) as [documented on Microsoft's website](https://docs.microsoft.com/en-us/cpp/build/setting-the-path-and-environment-variables-for-command-line-builds?view=vs-2019)
- Install the complete setup of Visual Studio 2015 or Visual Studio 2017 - not only the C++ Build tools

If the errors persists, maybe the `*-sys` dependency in question requires [a similar patch](https://github.com/compass-rs/sass-rs/commit/2e8289539fcb2b11812b666b5104d94744fa93b6) to select the proper `msbuild.exe` executable from the environment.
</details>

<details>
 <summary><code>fatal error LNK1112: module machine type 'x86' conflicts with target machine type 'x64'</code></summary>
 This happens when the variables used setup tools for the wrong architecture, where you have a linker for x86 instead of the target x64.


 Possible fixes:
 - Open the `x64 Native Tools Command Prompt` instead of `Developer Command Prompt`
 - Change the env variables to point to the correct version of the tools
 - Use `vcvars64.bat` instead of `vcvars32.bat` to configure the environment
</details>

<details>
 <summary><code>error MSB8036: The Windows SDK version 8.1 was not found. Install the required version of Windows SDK or change the SDK version in the project property pages or by right-clicking the solution and selecting "Retarget solution".</code></summary>
 If you get this error while developing on Windows 8.1, please, instell the required SDK using Visual Code Installer.

 If you get this error while developing on Windows 10, it means that the `*-sys` has outdated references to the SDK, and the build script is not being able to upgrade the project to a newer SDK.
 A possible solution is to uninstall Build Tools 2017 and install the Build Tools 2019. Alternatively, install the complete Visual Studio to have `devenv.exe /upgrade` available on your environment.
 </details>

### Updating and Uninstalling

After you’ve installed Rust via `rustup`, updating to the latest version is
easy. From your shell, run the following update script:

```text
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your
shell:

```text
$ rustup self uninstall
```

### Troubleshooting

To check whether you have Rust installed correctly, open a shell and enter this
line:

```text
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest
stable version that has been released in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t
see this information and you’re on Windows, check that Rust is in your `%PATH%`
system variable. If that’s all correct and Rust still isn’t working, there are
a number of places you can get help. The easiest is the #beginners channel on
[the official Rust Discord][discord]. There, you can chat with other Rustaceans
(a silly nickname we call ourselves) who can help you out. Other great
resources include [the Users forum][users] and [Stack Overflow][stackoverflow].

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Local Documentation

The installation of Rust also includes a copy of the documentation locally, so
you can read it offline. Run `rustup doc` to open the local documentation in
your browser.

Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
