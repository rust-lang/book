## Установка 

Первым делом необходимо установить Rust. Мы загрузим Rust при помощи `rustup`,
утилиты командной строки для управления версиями Rust и связанным с ним инструментарием. Для скачивания вам понадобится
подключение к интернету.

> Заметка: Если вы по некоторым причинам не желаете использовать `rustup`, пожалуйста посетите [the Rust
> installation page](https://www.rust-lang.org/install.html) для рассмотрения иных способов установки.

Следующие шаги ведут к установке последней стабильной версии компилятора Rust. Все
примеры и выводы программ в этой книге приведены для стабильной версии Rust 1.21.0. Гарантии стабильности Rust обеспечивают то, что все примеры в книге, которые компилируются, будут продолжать компилироваться и с более новыми версиями Rust. Выходные данные могут незначительно отличаться от версии к 
версии, потому что Rust часто улучшает сообщения об ошибках и предупреждениях. Иначе говоря
, любая более новая, стабильная версия Rust, которую вы установите при помощи следующих шагов будет работать
 в соответствии с поведением, описанным в этой книге.

> ### Примечание к командной строке
>
> В этой главе и во всей книге, мы будем показывать некоторые команды, используемые в
> терминале. Строки, которые вы должны ввести в терминал, начинаются с `$`. Вам
> не нужно вводить символ `$`; он указывает на начало каждой 
> команды. Строки, которые не начинаются с `$` обычно показывают вывод
> предыдущей команды. Кроме того, в примерах с PowerShell вместо `$` будет
> использоваться `>`.

### Установка `rustup` на Linux или macOS

Если вы используете Linux или macOS, откройте терминал и введите следующую команду:

```text
$ curl https://sh.rustup.rs -sSf | sh
```

Команда загружает скрипт и запускает установку инструмента `rustup`
, который устанавливает последнюю стабильную версию Rust. Вам может быть понадобится ввести свой пароль
. Если установка прошла успешно, появится следующая строка:

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

On Windows, go to [https://www.rust-lang.org/install.html][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
receive a message explaining that you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2017][visualstudio]. The tools are in
the Other Tools and Frameworks section.

[install]: https://www.rust-lang.org/install.html
[visualstudio]: https://www.visualstudio.com/downloads/

The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.

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
a number of places you can get help. The easiest is [the #rust IRC channel on
irc.mozilla.org][irc]<!-- ignore -->, which you can access through
[Mibbit][mibbit]. At that address you can chat with other Rustaceans (a silly
nickname we call ourselves) who can help you out. Other great resources include
[the Users forum][users] and [Stack Overflow][stackoverflow].

[irc]: irc://irc.mozilla.org/#rust
[mibbit]: http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust
[users]: https://users.rust-lang.org/
[stackoverflow]: http://stackoverflow.com/questions/tagged/rust

### Local Documentation

The installer also includes a copy of the documentation locally, so you can
read it offline. Run `rustup doc` to open the local documentation in your
browser.

Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
