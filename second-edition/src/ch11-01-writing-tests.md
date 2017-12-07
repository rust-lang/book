## Как писать тесты

Тесты в Rust - это функции специального вида, которые проверяют работу отдельных
частей программы. Тело функции-теста состоит из нескольких частей: установки входных
данных, тестирования исследуемой функции на ожидаемое поведение. Далее, будут рассмотрен
состав возможностей, который предлагается в Rust для этого: атрибуты, макросы, и
специальный атрибут `should_panic`.

### Структура функции-теста

Простейшая функция-тест - это функция аннотируемая атрибутом `test`. Атрибуты -
это метаданные (мы их уже встречали в примерах кода (Глава 5)). Чтобы функция превратилась
в тест для этого необходимо добавить `#[test]` перед ключевым словом `fn`.
Далее, с помощью команды `cargo test` будут выполнены тесты и в строках вывода
будут информационные сообщения о ходе проведения тестирования.

При создании библиотеки кода в Главе 7 мы обращали внимание на то, что создаётся
специальный модуль и тестовая функция. Этот код создаётся для ускорения написания
тестов. Далее, мы можем добавить необходимое количество тестовых функций в наш проект.

Мы рассмотрим аспекты работы шаблонных тестов, а потом напишем тесты, которые
будут проверять корректность поведения написанного нами кода.

Создадим проект `adder`:

```text
$ cargo new adder
     Created library `adder` project
$ cd adder
```

Содержание файла `src/lib.rs`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

<span class="caption">Пример кода 11-1: Тестовый модуль и функция генерируемая при
создании проекта библиотеки кода с помощью команды `cargo new`</span>

Сейчас проигнорируем первый две строчки кода и сфокусируемся на функции для того,
чтобы увидеть её работу. Обратите внимание на синтаксис описания `#[test]` перед
ключевым словом `fn`. Это атрибут сообщает компилятору, что далее будет заголовок
функции-теста. Функционал запускающий тесты на выполнение теперь знает, что это
особая функция - функция-тест. Также в составе модуля-тестов у нас могут бы вспомогательные
функции, не являющиеся тестами. Поэтому специальная аннотация (описание) так важна
для явного объявления функций - тестами.

Пока, функция не имеет содержания, что означает, что нет кода, который мог бы повлиять
на работу теста. Такой тест считается корректным.

Команда `cargo test` выполнить все тесты в выбранном проекте и сообщит о результатах 11-2:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

<span class="caption">Результат работы программы 11-2: Вывод информации о работе
тестов</span>

Cargo скомпилировал и выполнил тест. После строк `Compiling`, `Finished` и
`Running` мы видим строку `running 1 test`. Следующая строка показывает имя функции-теста.
Её имя  `it_works`. Результат её работы - `ok`. Далее вы видите обобщенную информации
о работе всех тестов: `test result: ok.` Это означает, что все тесты пройдены успешно.

Мы не должны отмечать тесты быть игнорированы, поэтому написано `0 ignored`.
Мы поговорим об игнорировании тестов в следующей секции.`0 measured` - это информация
об измерения производительности.

Следующая часть информации `Doc-tests adder` - это информация о тестировании документации.
У нас пока нет тестов документации, но Rust может компилировать любые примеры кодов,
которые находятся в API документации. Такая возможность помогает поддерживать документацию
в актуальном состоянии. Мы поговорим о тестировании документации в Главе 14. Пока
просто не будем обращать на эту информацию нашего внимания.

Давайте поменяем название нашего теста и посмотрим что же измениться в строке вывода.
Назовём нашу функцию `it_works` другим именем - `exploration`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }
}
```

Снова выполним команду `cargo test`. В строке вывода мы увидим новое наименование
нашей функции-теста - `exploration` вместо `it_works`:

```text
running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Отлично! Добавим ещё один тест. Сделаем так, чтобы этот наш новый тест не срабатывал
специально. Используем для этого уже известный на макрос *panics*. Хочу обратить
ваше внимание на то, что каждый тест выполняет в новом потоке. Поэтому когда главный
поток выполнения тестов видит, что какой-либо тест не срабатывает - этот тест отмечается
как непройденный. Мы поговорим об особенностях использования макроса *panic* в Главе 9.
После написания нового тесты код будет выглядеть так (11-3):

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

<span class="caption">Описание 11-3: Добавление второго теста. Второй тест вызывает
макрос `panic!`</span>

Запустим команду `cargo test`. Вывод результатов 11-4, которое сообщает, что тест
`exploration` пройден, а `another` нет:

```text
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
	thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured

error: test failed
```

<span class="caption">Описание 11-4: Описание результаты выполнения тестов</span>

Вместо `ok`, строка `test tests::another` сообщает `FAILED`. У нас есть две новых
секции между результатами и итогами. Первая секция показывает детальную причину
ошибки теста. В данном случае тест `another` не сработал, т.к.  `panicked at 'Make this test fail'`,
в строке 9 файла *src/lib.rs*. В следующей секции находятся имена всех непройденных тестов.
Это удобно, когда тестов очень много. Мы можем использовать имя непройденного
теста для отладки. Это обсудим в следующей секции.

Далее следуют итоговые данные. У нас один тест пройден, а 1 непройден.

Теперь мы знаем как выглядят описания при различных ситуациях работы системы тестирования
в Rust. Далее мы расширим наши знания о тестировании и познакомимся с макросами для
тестирования.

### Проверка результатов с помощью макроса `assert!`

Макрос `assert!` доступен в стандартной библиотеке. Он удобен, когда вы хотите проверить
какое-либо условие. Внутри входных данных данного макроса вычисляет логическое
значение. Если результат `true`, `assert!` ничего не делает и тест считается пройденным.
Если же значение входного параметра макроса `assert!` `false` вызывается макрос
 `panic!` и данный тест считается непройденным.

Вспомним пример кода из Главы 5 (5-9), где у нас была структура `Rectangle`
и метод `can_hold`. Повторим здесь код примера. Добавим код примера в файл *src/lib.rs*
и напишем тесты используя макрос `assert!`.

<span class="filename">Filename: src/lib.rs</span>

```rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```

<span class="caption">Код 11-5: Структура `Rectangle` и его метод `can_hold`</span>

Метод `can_hold` возвращает булево (логическое) значение. Такой метод удобен для
тестирование. Сейчас напишим тест (11-6), который будет проверять результат работы
метода `can_hold` экземпляра структуры `Rectangle`. С помощью теста проверим может
ли прямоугольник шириной 8 и длинной 7 содержать прямоугольник другого размера
(длинной 5 и шириной 1):

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

<span class="caption">Пример кода 11-6: Код теста для метода структуры Rectangle
`can_hold`, который проверяет корректность его работы</span>

Если вы пишите тесты, которые мы описываем в один и тот же проект и файл, вы, наверное,
обратили внимание на работу с модулями. Описание модуля - это иерархическая синтаксическая
конструкция, которые может быть только в одном экземпляре в тексте программы.
Не может быть несколько объявлений модуля *tests* в файле тестируемой библиотеки.

Также обратите внимание на сроку кода `use super::*;`. Модуль `tests` подчиняется
тем же правилам видимости, что и все остальные модули (всё то, что мы обсуждали
в Главе 7). Т.к. этот модуль внутренний, ему нужно дать доступ на верхний уровень,
чтобы можно было бы создать экземпляр структуры и вызвать его метод (или получить
доступ к чему-либо ещё на этом уровне).

Обратите на реализацию нашего нового теста. Его название `larger_can_hold_smaller`.
В теле теста мы создаём два экземпляра структуры `Rectangle`. Далее, мы вызываем
метод из одно из экземпляров (в данном случае из переменной `larger`) и передаём в
метод ссылку  на вторую переменную `larger.can_hold(&smaller)`. Это выражение мы
помещаем, как аргумент в макрос `assert!`. Т.к. метод возвращает логическое значение,
а макрос `assert!` принимает в качестве аргумента логическое значение, синтаксически
всё верно. Далее проверяем работу теста.

```text
running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Всё в порядке. Тест пройдёт. Теперь проверим, сообщит ли макрос об ошибке, если
мы попытаемся поместить большой прямоугольник в маленький:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
```
Т.к. правильный входной параметр данного макроса должен возвращать отрицательное значение,
с помощью логического "не" (*!*) мы корректно организовали проверку:

```text
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Отлично! Тесты работают. Теперь проверим, как отреагируют тесты, если мы добавим
ошибку в код метода `can_hold` - изменим знак сравнения в одной из логических выражений
на противоположное:

```rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}
```

Строки вывода:

```text
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
	thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Наши тесты нашли ошибки! В тесте  `larger.length` равно 8 и `smaller.length` равно 5.
Выражения сравнения в методе `can_hold` дают результат `false`, т.к. 8 больше 5.

### Проверка на равенство с помощью макросов `assert_eq!` и `assert_ne!`

Весьма часто для проверки работы методов и функций используется сравнение выходного
 результата и предполагаемого значения. Для этого мы можем использовать макрос `assert!`
 и оператор `==`. Важно также знать, что кроме этого макроса стандартная библиотека
 предлагает использовать макросы `assert_eq!` и `assert_ne!`. Использование этих
 макросов повышает читабельность кода. Кроме собственно проверки на равенство,
 эти макросы также печатают значения входных параметров, если тест завершился ошибкой.
 Эти макросы также более информативны, чем предыдущий, т.к. мы увидим ошибочные
 входные данные.

В примере кода 11-7, мы создадим функции `add_two`, которая прибавляет к входному
параметру 2 и возвращает значение. Then let’s test this function using the
`assert_eq!` macro:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

<span class="caption">Код 11-7: Тестирование функции `add_two` используя макрос
`assert_eq!` macro</span>

Проверим! Запустим тесты снова.

```text
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Всё в порядке. Функция работает как и предполагалось.

Теперь проверим, как будет выявлена ошибка. Изменим реализацию функции `add_two`:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}
```

Попробуем выполнить данный тест ещё раз:

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
	thread 'tests::it_adds_two' panicked at 'assertion failed: `(left ==
    right)` (left: `4`, right: `5`)', src/lib.rs:11
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

Наш тест нашел ошибку. Мест `it_adds_two` не сработал и сообщил важную информацию
для начала поиска ошибки в коде программы.

Обратите внимание, что в некоторых языках (таких как Java) в библиотеках кода
для тестирования принято именовать входные параметры проверочных функций "предполагаемое"
(`expected`) и "фактическое" (`actual`). В Rust приняты следующие обозначения
`left` и `right`, соответственно. Кроме того макросам тестирования совершенно не важно,
где находится предполагаемое (слева или справа), а где фактическое. Информационные
сообщения будут достаточно информативны:
`` assertion failed: `(left == right)` (left: `5`, right: `4`) ``.

Макрос `assert_ne!` сработает успешно, если входные параметры не равны друг другу.
Этот макрос будет полезен в тех случаях, когда вы не знаете, какое точно может быть
значение, то знаете точно, каким оно быть не может. К примеру, если у вас есть
функция, которая изменяет входные данные определённым образом. Лучший способ проверить
правильность работы такой функции - сравнить входное и выходное значения. Они не
должны быть равными.

С своей работе макросы `assert_eq!` и `assert_ne!` неявным образом используют
операторы `==` и `!=`. Когда тест не сработает, макросы напечатают значения аргументов
с помощь отладочного форматирования (что в свою очередь значит, что значения аргументов
должны реализовать типажи `PartialEq` и `Debug`). Все примитивные типы стандартной
библиотеки Rust реализовали эти типажи. Для структур и перечислений, которые вы
сами реализуете вы должны реализовать типаж `PartialEq` для сравнения значений.
Для печати отладочной информации в виде сообщений в строку вывода консоли необходимо
реализовать типаж `Debug`. Эти типажи можно реализовать добавив аннотацию
`#[derive(PartialEq, Debug)]`на определение структуры или перечисления.

### Создание сообщений об ошибках

Продолжим изучать работу с макросами для тестирования. Конечно, было бы удобно,
если была бы возможность добавить дополнительную информацию при выводе ошибки.
И такая возможность есть. Это опциональный текстовый аргумент, которые обрабатывается
макросом `format!`. Такие сообщения удобны для более детального раскрытия информации
о состоянии теста, ожидаемых результатах, возможных причинах ошибки и способах её
устранения.

Например, создадим функцию, которая приветствует человека по имени. Протестируем
эту функцию. Мы хотим чтобы вводимое имя выводилось на консоль:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

Теперь внесём ошибку в функцию:

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

Running this test produces:

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'assertion failed:
    result.contains("Carol")', src/lib.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::greeting_contains_name
```

Сообщение содержит лишь информацию о том что сравнение не было успешным. Сообщение
было бы более информативным, если бы выводило также выходные данные. Изменим
тестовую функцию для того, чтобы выводились форматированное сообщение:

```rust,ignore
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

После того, как выполним тест ещё раз мы получим подробное ожидаемое сообщение:

```text
---- tests::greeting_contains_name stdout ----
	thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain
    name, value was `Hello`', src/lib.rs:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Это сообщение поможет нам в отладке (получим то, что есть, вместо того что должно быть).

### Проверка с помощью макроса `should_panic`

Кроме проверок выходных данных также важно, проверить условия при которых могу быть
ошибки. Например, рассмотрим инициализацию структуры `Guess`, которую мы создали в
Главе 9 (9-8). При создании экземпляра структуры проверяется входной параметр (он
должен быть между 1 и 100). Мы можем написать тест, который проверит реакцию кода
инициализации на неправильные входные данные.

Реализуем это с помощью атрибута функции теста `#[should_panic]`. Этот атрибут
сообщает системе тестирования, что этот метод должен генерировать ошибку. Если ошибка
не генерируется - тест считается непройденым.

Код программы 11-8 показывает, как надо написать такой тест:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

<span class="caption">Код программы 11-8: Тестирование генерации `panic!`</span>

Атрибут `#[should_panic]` следует после `#[test]` и до объявления текстовой функции.
Строка вывода может выглядеть следующим образом:

```text
running 1 test
test tests::greater_than_100 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Отлично! Теперь внесём ошибку:

```rust
# struct Guess {
#     value: u32,
# }
#
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}
```

Вид обновлённой строки вывода:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

Обратите внимание, что сообщение не очень информативное.
Мы можем улучшить взаимодействие с атрибутом `should_panic` добавив параметр - ожидаемую
подстрку выводимого при генерации ошибки сообщения:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

<span class="caption">Код программы 11-9: Тестирования случая вызова макроса
`panic!` содержащего предполагаемую ошибку</span>

Этот тест сработает, т.к. соблюдены все условия.

Изменим код (внесём ошибку и посмотрим на результат):

```rust,ignore
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {}.", value);
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {}.", value);
}
```

Ошибка:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
	thread 'tests::greater_than_100' panicked at 'Guess value must be greater
    than or equal to 1, got 200.', src/lib.rs:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: Panic did not include expected string 'Guess value must be less than or
equal to 100'

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

Эта ошибка говорит нам о том, что параметр атрибута не содержиться в генерируемом
макросом `panic!` сообщении. Также приводится тестовый параметр атрибута `should_panic`,
что, возможно, поможет найти ошибку в коде.

Теперь, когда вы научились писать тесты, в следующей секции мы приступим к детальной
настройке запуска тестов с помощью команды `cargo test`.
