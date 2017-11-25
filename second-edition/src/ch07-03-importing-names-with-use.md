## Импорт имён

Мы изучили как вызывать функции, определённые в модуле используя имена модулей, как
часть вызова. Пример 7-6:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

<span class="caption">Пример 7-6: Вызов функции, указав полный к ней путь</span>

Как вы видите, указание полного пути к функции весьма утомительно. Конечно же, в Rust
имеется функционал упрощающий вызов функций.

### Краткий импорт. Использование `use`

Использование ключевого слова `use` сокращает указание полного пути к функции, которую
вы хотите использовать в определённой области видимости. Пример применения `use`:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

Строка `use a::series::of;`, что в данной области видимости могут использовать элементы,
которые находятся в модуле `of`. Их можно вызывать просто указывая префикс имени этого
модуля `of::`.

В область видимости попадают только элементы модуля. Подчиненные модуле не включаются.
Если в этом будет необходимость - надо явным образом это указать.
Поэтому укажем `of::nested_modules`, вместо `nested_modules`.

Чтобы не указывать имя модуля можно выполнить т.н. статический импорт функции в
область видимости:

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}
use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

Такой способ импорт даёт нам возможность сокращать список импорта.

Очень интересная возможность импорта значений перечислений!
Т.к. перечисления можно назвать разновидность пространств имеет, то можно указать
только необходимые элементы перечисления при импорте:

```rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("{:?}",red);
    println!("{:?}",yellow);
    println!("{:?}",green);
}
```
Так как мы не включили `TrafficLight` в список импортированных значений перечисления,
то для его использования нам необходимо указать полный путь до этого элемента.

### Импорт всех элементов с помощью `*`

Есть ли возможность импортирования всех элементов выбранного пространсва имён?!
Да. Есть. Используйте `*`:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```
Символ `*` называют *glob* и его функция - импорт всех элементов, видимых извне
пространства имён. Обратите также внимание, что наряду с удобствами, существуют
также недоставки использования полного импорта пространства имён, т.к. это может привести
к конфликтными или неожиданным ситуациями, когда в разных пространствах имён существуют
одинаковые (по имени) функции, которые будут импортироваться.
Пример:

```rust
pub mod a {
    pub mod series {
        pub mod of1 {
            pub fn nested_modules() {
                println!("nested_modules 1");
            }
        }
        pub mod of2 {
            pub fn nested_modules() {
                println!("nested_modules 2");
            }
        }
    }
}
use a::series::of1::*;
use a::series::of2::*;

fn main() {
    nested_modules();
}
```

Описание ошибки:
```
error: `nested_modules` is ambiguous
  --> src/main.rs:19:5
   |
19 |     nested_modules();
   |     ^^^^^^^^^^^^^^
   |
note: `nested_modules` could refer to the name imported here
  --> src/main.rs:15:5
   |
15 | use a::series::of1::*;
   |     ^^^^^^^^^^^^^^^^^^
note: `nested_modules` could also refer to the name imported here
  --> src/main.rs:16:5
   |
16 | use a::series::of2::*;
   |     ^^^^^^^^^^^^^^^^^^
   = note: consider adding an explicit import of `nested_modules` to disambiguate
```

### Доступ к функционалу родительского модуля с помощью `super`

Как вы помните, при создании библиотеки Cargo предлагает использовать модуль `tests`.
Сейчас разберёмся подробнее. Добавим код теста в исходный код файла *src/lib.rs*:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

В главе 11  подробно рассказывается о тестировании. Сейчас мы только немного расскажем.
Обратите внимание на специальную аннотацию и то что это отдельный модуль в нашем коде.
Модульная система нашего проекта теперь имеет вид:

```text
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

Тесты помогат отлаживать код библиотеки. Напишем наш первый тест. Он будет вызывать
функцию `client::connect`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Выполнение тестов осуществляется командой`cargo test`:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `client`
```

Почему-то компиляция прошла неуспешно. Почему же? Нам не надо добавлять префикс
библиотеки `communicator::`, т.к. мы находимся внутри неё.

Как же вызвать функцию `client::connect` из модуля `tests`? В модуле `tests` мы
можем указать что мы хотим начать поиски модулей с корневого модуля:

```rust,ignore
::client::connect();
```

Или мы можем использовать `super` для того чтобы переместиться по модульной иерархии
на один уровень выше текущаего модуля:

```rust,ignore
super::client::connect();
```

Эти две опции выглядят одинаковыми в этом примере. Если находитесь глубоко
внутри модульной иерархии, то начиная с корневого модуля ваш код будет длинным.
Есть случаи, когда использование `super` более удобно.

Это бывает утомительно печать `super::` в каждом тесте. Есть решение `use`.
Функциональность `super::` изменяет путь, который вы используете в `use`.

Для тех случаев, когда вы пишите тесты к библиотекам использование `use super::something`
наилучшее решение.

Пример:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

Когда вы теперь выполните команду `cargo test`  вы увидите следующий вывод:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Итоги

Теперь вы знаете ещё один способ, как можно организовать ваш код. Её можно использовать
для группировки различных элементов вместе, при рефакторинг большого количества
кода.

Далее, мы рассмотрим структуры данных стандартной библиотеки.
