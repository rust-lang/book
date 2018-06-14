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

Установочный скрипт автоматически добавляет Rust в системную переменную PATH после следующего входа в систему
. Если вы хотите начать использовать Rust сразу, вместо того чтобы перезапускать сеанс
, выполните следующую команду в консоли, чтобы вручную добавить Rust в вашу системную переменную
 PATH:

```text
$ source $HOME/.cargo/env
```

Или же вы можете добавить следующую строку в *~/.bash_profile*:

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

### Установка `rustup` на Windows

На Windows, перейдите на [https://www.rust-lang.org/install.html][install] и следуйте
инструкциям по установке Rust. На одном из этапов установки, вы
получите сообщение о том, что вам также необходимы инструменты сборки C++ для
Visual Studio 2013 и новее. Самый простой способ получить инструменты сборки — это установить
[Build Tools for Visual Studio 2017][visualstudio]. Инструменты находятся в разделе
Other Tools and Frameworks.

[install]: https://www.rust-lang.org/install.html
[visualstudio]: https://www.visualstudio.com/downloads/

В этой книге будут использоваться команды, работающие и а *cmd.exe*, и в PowerShell.
Если будут какие-либо специфические различия, мы укажем что использовать.

### Обновление и Удаление

 После того, как вы установите Rust при помощи `rustup`, вы можете легко его обновлять
. Для этого запустите следующий сценарий обновления из вашей консоли:

```text
$ rustup update
```

Чтобы удалить Rust и `rustup`, запустите следующий сценарий удаления из вашей консоли:

```text
$ rustup self uninstall
```

### Решение проблем

Чтобы проверить, правильно ли вы установили Rust, откройте консоль и введите эту
строку:

```text
$ rustc --version
```

 Вы должны увидеть версию, хэш коммита, и дату коммита последней стабильной версии
в следующем формате:

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
