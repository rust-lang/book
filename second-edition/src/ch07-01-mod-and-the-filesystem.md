## `mod` и файловая система

Мы начнём создавать наш пример использования модуля. Создадим проект библиотеки
кода.

Cоздадим основные блоки нашей библиотеки, которая будет предоставлять функциональные
возможности использования сетевых технологий. Назовём нашу библиотеку `communicator`.
По умолчанию Cargo создаёт библиотеки кода. Если при создании нового проекта мы
не установим флаг `--bin`, то будет создана библиотека:

```text
$ cargo new communicator
$ cd communicator
```

Обратите внимание, что Cargo создаёт *src/lib.rs* вместо *src/main.rs*, в котором
мы видим вот такое содержание:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```
Cargo  создаёт пустой тест, чтобы показать как можно тестировать функционал библиотеки.
Мы изучим использование синтаксических конструкций `#[]` и `mod tests` в последующей
секции "Использование `super` для доступа к родительскому модулю" этой главы.

Сейчас же мы не будем использовать данный функционал, поэтому просто удалите этот код.

Т.к. у нас нет файла *src/main.rs*, нечего запускать на выполнение с помощью команды
`cargo run`. В тоже время мы можем воспользоваться командой `cargo build` для компиляции
нашей библиотеки.

Мы рассмотрим различные опции организации кода нашей библиотеки.

### Определение модуля

Первым делом напишем определение модуля `network`, который будет содержать
определение функции `connect`. Определение начинается с ключевого слова `mod`.


<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}
```

После определения модуля, внутри фигурных скобок пишем определения функции и
все что входит в состав модуля. В нашем случае это описание функции.
Если мы хотим вызывать функцию извне модуля, мы должны явно указать это `network::connect()`.

Мы можем иметь множество описаний модулей в одном файле  *src/lib.rs*.
К примеру, модуль `client`, может содержать функцию `connect` 7-1:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

<span class="caption">Пример 7-1: Определение модулей `network` и `client`в файле
 *src/lib.rs*</span>

Теперь у нас есть описание двух функций, которые могут быть вызваны с помощью
синтаксических конструкций `network::connect` и `client::connect`.
Каждая из функций может иметь различные функциональные возможности и не имеют
между собой никакого конфликта имён.

В этом случае, как мы создаём библиотеку, файл который хранит точку доступа к
библиотеки это  *src/lib.rs*.  Также мы можем создать модуль в файле  *src/main.rs*
для какой-либо бинарной программы. Также очень важная особенностью модулей - они
могут быть вложенными. Это весьма удобно для логической организации кода.
Пример 7-2:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

<span class="caption"> Пример 7-2: Перемещение модуля `client` внутрь модуля `network`</span>

Теперь у нас есть две разные функции `network::connect` и `network::client::connect`.
Каждая из которых находится в своём пространстве имён.

Теперь организация нашего кода имеет вот такую структуру:

```text
communicator
 ├── network
 └── client
```

Пример вложенных модулей 7-2:

```text
communicator
 └── network
     └── client
```

Логическая организация кода зависит от ваших задач.

### Размещение модулей по нескольким файлам

Модульная структура похожа на файловую систему. Мы можем использовать модульную
систему для хранения кода в разных файлах. Рассмотрим пример 7-3:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<span class="caption">Пример 7-3: Модули `client`, `network` и`network::server`,
все они находятся в *src/lib.rs*</span>

Архитектура модулей *src/lib.rs*:

```text
communicator
 ├── client
 └── network
     └── server
```

Если модули имеет множество функций и эти функции длинные, было бы удобно разделить
такой код на несколько файлов.

Сначала заменим код модуля `client` на декларацию модуля:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

Тут мы видим декларацию модуля. Этим мы сообщаем, что в другом месте есть определение
модуля `client`:

```rust,ignore
mod client {
    // contents of client.rs
}
```

Теперь создадим файл *client.rs* в папке исходных кодов.:

<span class="filename">Filename: src/client.rs</span>

```rust
fn connect() {
}
```
Обратите внимание, что вам не надо декларировать модуль, т.к. вы уже декларировали
его в файле *src/lib.rs*. Этот файл содержит компоненты модуля `client`. Если вы
здесь напишите декларацию модуля `mod client`, то это будет значит, что внутри модуля
`client` есть модуль `client`.

По умолчанию, компилятор сначала исследует содержание файла *src/lib.rs*. Если
есть необходимость добавить несколько файлов в проект, необходимо сообщить об этом
в файле *src/lib.rs*. Именно поэтому, модуль `client` надо определить в файле *src/lib.rs*
и не надо делать этого в файле *src/client.rs*.

Компиляция проекта пройдет успешно:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:4:5
  |
4 |     fn connect() {
  |     ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:8:9
  |
8 |         fn connect() {
  |         ^
```

Эти сообщения сигнализируют нам, что наши функции нигде не используются. Проигнорируем
их до секции "Управление доступом с помощью ключевого слова `pub`".

Теперь перенесём модуль `network` в свой файл:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network;
```

Далее, создадим файл *src/network.rs* и введём в него следующий код:

<span class="filename">Filename: src/network.rs</span>

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```
Обратите внимание, что у нас есть описание модулей в файле, т.к. у нас всё еще есть
вложенность модулей.

Выполним команды `cargo clean` а потом `cargo build`. Всё в порядке! Отлично!
Теперь осталось создать файл только для ещё одного модуля. Для этого создадим
описание подчиненного модуля в файле *src/network.rs* `mod server;`:

<span class="filename">Filename: src/network.rs</span>

```rust,ignore
fn connect() {
}

mod server;
```

Далее создадим файл *src/server.rs* и добавим в него содержание:

<span class="filename">Filename: src/server.rs</span>

```rust
fn connect() {
}
```

Выполним команды `cargo clean` а потом `cargo build`. Получим сообщение об ошибке 7-4:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

<span class="caption">Код 7-4: Ошибка при переносе кода вложенного модуля `server`
в файл *src/server.rs*</span>

Компилятор предлагает решение:

```text
note: maybe move this module `network` to its own directory via
`network/mod.rs`
```
Вместо того, чтобы создавать файл, сделаем следующее:

1. Создадим папку *directory* с именем *network* (это имя нашего родительского модуля).
2. Перенесём файл *src/network.rs* в эту новую папку и переменуем файл в *src/network/mod.rs*.
3. Далее перенесём файл *src/server.rs* в папку *network*.

 Для подчиненных модулей проделаем тоже самое.

### Правила модульной файловой системы

Список правил:

* Если модуль `foo` не имеет подчиненных модулей, вы можете сохранить код модуля в
файл *foo.rs*.
* Если модуль `foo` имеет подмодуль, вы должны перенести код модуля в файл *foo/mod.rs*

Это правило применяется рекурсивно если модуль с именем `foo` имеет подмодуль с
`bar` и `bar` не имеет подмодулей, то у Вас получить вот такая файловая система
в папке *src*:

```text
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

Модули должны быть определены в своих файлах используя ключевое слово `mod`.

Далее, мы поговорим о модификаторе доступа `pub` и устраним сообщения о проблемах
в коде.
