## Контролирование хода выполнение тестов

Также как команда `cargo run` компилирует и затем выполняет созданный бинарный файл,
команда `cargo test` компилирует его в тестовом режиме а затем выполнят его.
Для изменения поведения команды по умолчанию существуют параметры, на которые можно
влиять при запуске тестирования. Например, по умолчанию, система управлением запуска
тестов (СУЗТ) выполняет тесты параллельно (т.е. создаётся поток для каждого теста, чтобы
обеспечить изолированное выполнение каждого теста). Кроме того СУЗТ собирает консольные
сообщения каждого теста и форматирует их и выводит по своим правилам. Есть возможность
менять это поведение с помощь опций командной строки при запуске команды `cargo test`.

Опции команды `cargo test`  могут быть добавлены после,
опции для тестов должны устанавливаться дополнительно (следовать далее). Для разделения
этих двух типов аргументов используется разделитель `--`. Чтобы узнать подробнее об
доступных опциях команды `cargo test` - используйте опцию `--help`. Для того, чтобы узнать
об опциях доступных для непосредственно для тестов используйте команду `cargo test -- --help`.
Обратите внимание, что данную команду необходимо запускать внутри cargo-проекта
(пакета).

### Выполнение тестов параллельно или последовательно

Когда выполняется несколько тестов они выполняются параллельно (по умолчанию).
Это значит, что тесты завершат свою работу быстрее, т.е. мы быстрее узнаем успешно
работают тесты или нет. Важно соблюдать независимость работы тестов.

Например, когда тесты создают в одном и том же месте  на диске файл с одним и тем же
названием, читают из него данные, записывают их - вероятность ошибки в работе таких
тестов (из-за конкурирования доступа к ресурсу, некорректных данных в файле) весьма
высока. Решением будет использование уникальных имён создаваемых и используемых каждым
тестом в отдельности, либо выполнение таких тестов последовательно.

Для выполнения тестов последовательно, пожалуйста запустите следующую команду в
папке проекта с тестами:

```text
$ cargo test -- --test-threads=1
```

В данной команде мы сообщили количество потоков, которое будет использовано системой
тестирования для запуска всех тестов (т.к. количество 1, то все тесты будут работать
последовательно).

### Демонстрация результатов работы функции

По умолчанию, если тест пройден, СУЗТ блокирует вывод на печать, т.е. если вы вызовете
макрос `println!` внутри кода теста и тест будет пройден, вы не увидите вывода
на консоль результатов вызова `println!`. Если же тест не был пройден, все информационные
сообщение, а также описание ошибки будет выведено на консоль.

Например, в коде (11-10) функция выводить значение параметра с поясняющим текстовым
сообщением, а также возвращает целочисленное константное значение `10`. Далее,
следуют тест, который имеет правильный входной параметр и тест, который имеет ошибочный
входной параметр:

<span class="filename">Filename: src/lib.rs</span>

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

<span class="caption">Listing 11-10: Тест функции, которая использует макрос `println!`
`println!`</span>

Результат вывода на консоль команды `cargo test`:

```text
running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
	I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left ==
right)` (left: `5`, right: `10`)', src/lib.rs:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Обратите внимание, что мы не увидели вывода на консоль работы корректно сработавшего
теста `I got the value 4`. Этот вывод был проигнорирован. А вот результат работы
программы, при неработающем тесте был показан (для лучшего понимания ошибки).

Для того, чтобы всегда видеть вывод на консоль и корректно работающих программ,
используйте флаг `--nocapture`:

```text
$ cargo test -- --nocapture
```

Выполним тесты ещё раз с этим флагом:

```text
running 2 tests
I got the value 4
I got the value 8
test tests::this_test_will_pass ... ok
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left ==
right)` (left: `5`, right: `10`)', src/lib.rs:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
test tests::this_test_will_fail ... FAILED

failures:

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured
```

Обратите внимание, что есть какая-то неразбериха в косольном выводе. Всё это из-за
того, что тесты выполняются параллельно. Этого можно избежать с помощью опции
`--test-threads=1`. Пожалуйста, проверьте работу команды с флагом `--nocapture` ещё
раз с последовательном выводом данных на экран.

### Запуск теста по имени Running a Subset of Tests by Name

Бывают случаи, когда в запуске всех тестов нет необходимости и нужно запустить
только несколько тестов. Если вы работаете на над функцией и хотите запустить
тесты, которые исследуют её работу - это было бы удобно. Вы можете это сделать,
используя команду `cargo test`.

Для демонстрации как запустить группу тестов мы создадим группу тестов для функции
`add_two` (код программы 11-11) и постараемся выбрать интересующие нас тесты при
их запуске:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

<span class="caption">Код программы 11-11: Три теста с различными именами</span>

Если вы выполните команду `cargo test` без уточняющих аргументов, все тесты выполнятся
параллельно:

```text
running 3 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured
```

#### Запуск одного теста

Мы можем запустить один тест с помощью указания его имени в команде `cargo test`:

```text
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

К сожалению, таким простым образом (списком тестов) мы не можем запустить несколько тестов.
Только первый тест из списка будет запущен. Пожалуйста, проверьте как это работает
(точнее, убедитесь, что это не работает).

#### Использование фильтров для запуска нескольких тестов

Существует возможность по имени (с использованием фильтров) запустить несколько тестов.
Например, мы знаем, что несколько тестов содержат `add`. Для того, чтобы запустить
вполне достаточно этих знаний:

```text
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Обратите внимание, что у нас получилось выполнили тесты с именем `add`. Также
обратите внимание, что имя модуля включено в имя теста. Таким образом мы можем
запустить тесты используя имя модуля, в котором он находятся.

### Игнорирование тестов

Бывают случаи, когда выполнение тестов может занимать продолжительное время и
нет необходимости в их постоянном запуске. Для этих случаев существует
атрибут `ignore`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn it_works() {
    assert!(true);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

Мы добавили атрибут `#[ignore]` для того, чтобы описать исключаемую функцию из списка тестов.
Теперь, когда мы запустим команду `cargo test`, данный тест будет проигнорировал,
о чём будет сообщено в описании результатов тестирования:

```text
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Выполнение теста `expensive_test` было проигнорировано. Если же вы хотите выполнить
только проигнорированные тесты, вы можете сообщить это с помощью команды
`cargo test -- --ignored`:

```text
$ cargo test -- --ignored
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Подведём итоги. Вы можете фильтровать тесты по имени при запуске. Вы также можете указать
какие тесты должны быть проигнорированы, а также отдельно запускать проигнорированные
тесты.
