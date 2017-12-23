## Рабочая среда Cargo

В Главе 12 вы создавали проект, который включал в себе как бинарный, так
и библиотечный контейнеры. Возможно, вы заметили, что при увеличении проекта
вполне естественно разделение кода на логически связанные составные части. Для
упрощения работы над логически связанными контейнерами Cargo предоставляет инфраструктуру
называемую *рабочим пространством* (*workspaces*).

*Рабочее пространство* является способом группировки пакетов, которые совместно
используют файл *Cargo.lock* и папку для хранения конечных программных продуктов
(будь то бинарные файлы или библиотеки). Далее бы будем создавать проект используя
возможности данного функционала. Это будет простой проект. В нём бы уделим внимание
организации рабочего пространства.
Итак, описание структуры. У нас есть бинарный проект, который использует библиотеки:
одна библиотека предоставляет функцию `add_one`, а другая предоставляет функцию
`add_two`. Эти три контейнера являются частью неявного рабочего пространства.
Реализуем эти связи явным образом! Первое, создадим новый бинарный контейнер `adder`:

```text
$ cargo new --bin adder
     Created binary (application) `adder` project
$ cd adder
```

Добавим в файл описания секцию `[workspace]` для того, чтобы сообщить Cargo, пакет
является рабочим пространством:

```toml
[workspace]
```

Как и многие другие опции Cargo, рабочее пространство поддерживает соглашения по
конфигурации. Поэтому для того, чтобы проект стал рабочим пространством на достаточно
просто добавить в файл конфигурации данную информацию.

<!-- Below -- any crates what depends on, specifically? The program? -->
<!-- They're all programs. We mean the top-level crate in the workspace here,
I've tried to clarify. /Carol -->

### Определение взаимосвязей в рабочем пространстве

Все контейнеры в поддиректориях от которых зависит контейнер верхнего уровня являются
частью рабочего пространства. Любые контейнеры, независимо от их места хранения,
могут добавлять в зависимости от других контейнеров находящихся в локальных директориях
с помощью атрибута `path` в файле *Cargo.toml*. Если описание контейнера содержит
ключ `[workspace]` и имеет описания зависимостей, где пути к зависящим контейнерам
являются поддиректориями, все эти контейнеры являются частью *рабочего пространства*.
Давайте определим в файле *Cargo.toml* контейнера `adder`, что он имеет зависимость
от контейнера `add-one`, которых расположен в поддиректории:

<!-- Above, what is the path dependency actually doing here, can you fill out
the paragraph above? -->
<!-- done /Carol -->

```toml
[dependencies]
add-one = { path = "add-one" }
```

Если же мы добавим описание зависимости в файл *Cargo.toml*, которое не будет
содержать `path`, такая зависимость будет считаться внешней зависимостью, данные о
которой будут запрашиваться из репозитория Crates.io.

### Добавление второго контейнера в рабочую среду

<!-- You can see I'm adding headings, here, trying to add some more navigable
structure -- can you improve these? I'm not sure mine are accurate -->
<!-- Yep! /Carol -->

Далее, после того как мы описали путь к контейнеру, мы создадим его с помощью
команды:

```text
$ cargo new add-one
     Created library `add-one` project
```

Файловая структура папки `adder` теперь будет иметь следующий вид:

```text
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── src
    └── main.rs
```

В код библиотеки *add-one/src/lib.rs* добавим описание функции `add_one`:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

<!-- below -- Where are we adding the extern crate line? -->
<!-- at the top, where all the extern crate lines go and as illustrated by the listing /Carol -->

Далее, откроем файл *src/main.rs* контейнера `adder` и добавим строку кода `extern crate`
в верхней строчке файла исходного кода для того, чтобы добавить возможность использовать
функционал контейнера `add-one` в исходном коде. Далее, изменим код функции `main`.
Будем использовать функцию импортированного контейнера (14-12):

```rust,ignore
extern crate add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

<span class="caption">код 14-12: использование функционала библиотечного контейнера
`add-one` в исходном коде контейнера `adder`</span>

Создадим объектный код контейнера `adder` с помощью команды `cargo build` внутри
папки *adder*.

```text
$ cargo build
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68 secs
```

С помощью данной команды мы создали объектный код контейнеров `adder` и `add-one`.
Теперь структура каталога `adder` выглядит следующим образом:

```text
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── src
│   └── main.rs
└── target
```

Рабочее пространство имеет одну директорию для хранения объектных файлов (*target*
directory). Обратите внимание, что контейнер *add-one* не имеет своей собственной
директории для хранения объектных файлов (*target* directory). Даже если вы перейдёте
в папку *adder/add-one/target* и запустим команду `cargo build`, то данный пакет
будем всё равно скомпилирован в директорию *adder/target*. Контейнеры в рабочем
пространстве могут быть связаны между собой. Если, по какой-то причине, подчиненному
контейнеру необходимо иметь свою директорию для хранения объектных файлов, каждый
то контейнеры, которые зависят от него также будут перекомпилированы. Это источник
ошибок и множества проблем. Поэтому для упрощения в рабочем пространстве есть только
одна одна директория для хранения объектных файлов.

<!-- Above -- I have no idea what this means for our project here, can you put
it in more practical terms, or otherwise maybe just explain what this means for
the user? -->
<!-- I added more explanation for the target directory in this section and
added more explanation for the Cargo.lock in the next section, since the
Cargo.lock advantages aren't as visible until you start adding dependencies on
external crates. What do you think? /Carol -->

#### Зависимость от внешних контейнеров в рабочей среде

Обратите внимание, что рабочая в рабочей среде есть только один файл *Cargo.lock*
на все имеющиеся контейнеры. В данном примере мы предполагаем, что все контейнеры
используют одни и те же зависимости, которые описаны в файле *Cargo.toml*. Также
такой способ обеспечивает совместимость всем зависимых контейнеров.

Добавим зависимость `rand` в секцию `[dependencies]` файла *add-one/Cargo.toml*::

<span class="filename">Filename: add-one/Cargo.toml</span>

```toml
[dependencies]

rand = "0.3.14"
```

Далее, мы добавим код `extern crate rand;` в файл *add-one/src/lib.rs* и соберем
объектные файлы с помощью команды `cargo build` в директории *adder*. Обратите
внимание, что в этой директории будет скомпилирован контейнер `rand`:

```text
$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading rand v0.3.14
   ...snip...
   Compiling rand v0.3.14
   Compiling add-one v0.1.0 (file:///projects/adder/add-one)
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18 secs
```
Файл *Cargo.lock* теперь содержит информацию о зависимости контейнера `add-one`
от `rand`. Хотя `rand` используется рабочем пространстве, вы не можете его
использовать без явного указания на него в зависимостях контейнера. Например,
если вы добавите строку кода `extern crate rand;` в файл исходного кода контейнера
`adder`, то код перестанет компилироваться:

```text
$ cargo build
   Compiling adder v0.1.0 (file:///projects/adder)
error[E0463]: can't find crate for `rand`
 --> src/main.rs:1:1
  |
1 | extern crate rand;
  | ^^^^^^^^^^^^^^^^^^^ can't find crate
```

Для исправления этой ошибки добавте ссылку на него в файл *Cargo.toml*. После
запустите команду `cargo build` ещё раз. Описание зависимостей изменится в файле
*Cargo.lock*, но дополнительных копий контейнера `rand` не будет скачано. Использование
одной и той же копии зависимости внутри рабочего пространства экономит и упрощает
хранение контейнеров-зависимостей.

#### Добавление теста в рабочее пространство

В целях дальнейших улучшений нашего проекта, добавим тест функции `add_one::add_one`
в контейнере `add_one`:

<span class="filename">Filename: add-one/src/lib.rs</span>

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Выполните команду `cargo test` в рабочем пространстве *adder*:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running target/debug/adder-f0253159197f7841

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Как же так? Нет тестов? Но у нас же есть один? Обратите внимание, что тесты были
выполнены только дня проекта модуля рабочего пространства. Для того, чтобы выполнить
все тесты используйте опцию (флаг) `--all`:

```text
$ cargo test --all
    Finished dev [unoptimized + debuginfo] target(s) in 0.37 secs
     Running target/debug/deps/add_one-abcabcabc

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/adder-abcabcabc

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Для того, чтобы запустить тесты определённого контейнера необходимо использовать
флаг `-p` и имя контейнера:

```text
$ cargo test -p add-one
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/add_one-b3235fea9a156f74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Этот вывод информации сообщил только о тестах в контейнере `add-one`.

Если вы захотите публиковать контейнеры из данной рабочей среды на сайте crates.io,
каждый контейнер будет публиковаться отдельно. Команда `cargo publish` не имеет
уточняющих флагов, поэтому должна использоваться только в том контейнере, который
нужно опубликовать.

<!-- What does that mean, we have to publish them all one at a time?-->
<!-- Yep, we've tried to clarify /Carol -->

Сейчас создаёте ещё один контейнер в рабочей среде - `add-two`.

Использование рабочей среды упрощает разработку многокомпонентных контейнеров.
