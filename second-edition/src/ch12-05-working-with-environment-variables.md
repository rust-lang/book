## Работа с системными переменными

Мы хотим улучшить наше приложение - добавить дополнительную функцию - установить
опцию для поиска текста без учёта регистра символов. Это опцию пользователь программы
может установить в своё системе с помощью переменных среды. Мы можем установить
опцию такую при запуске программы, что потребует от пользователя при каждом запуске
программы использовать данную опцию. Мы хотим упростить решение такой задачи.
Переменные среды позволяют установить какую-либо опцию один раз.


### Написание теста с ошибкой для решения задачи поиск без учёта регистра

Добавим новую функцию `search_case_insensitive`, которую мы будем вызывать для того,
чтобы установить переменную среды.

Напишем тест с ошибкой. Добавим новый тест `case_insensitive` (12-20):

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

<span class="caption">Код 12-20: добавление нового тест с ошибок для проверки поиск
без учёта регистра</span>

Обратите внимание, что мы также поменяли входные параметры теста `case_sensitive`.
Мы добавили новую строку “Duct tape”, чтобы была возможность поиска строки “duct”
при установленной опции игнорирования регистра символов. Благодаря этому изменению
мы не нарушим ненароком работу предыдущего теста (он сработает при любом установленном
режиме поиска).

Новый тест будет использовать следующий шаблон для поиска - “rUsT”. В функции
`search_case_insensitive` функция поиска должна найти текст и в строке содержащей
“Rust:” и в “Trust me.”. Пока наш тест ещё не будет срабатывать, т.к. мы ещё не
реализовали функцию `search_case_insensitive`. Добавим описание функции так, как
мы это уже делали в функции `search` (без рабочей реализации, но с соблюдением
всех синтаксических формальностей.).

### Реализация функции `search_case_insensitive`

Функция `search_case_insensitive` показанная в примере кода 12-21 почти нечем не
отличается от функции `search`. Отличие в том, что производится принудительное
приведение значения переменной `line` и  в нижний регистр и аргумента `query`.

<span class="filename">Filename: src/lib.rs</span>

```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">Код программы 12-21: Определение функции `search_case_insensitive`
и реализация тела этой функции</span>

Описание решения. Сначала получили значение аргумента `query` в нижнем регистре.
Для этого скроим аргумент с помощью одноимённой локальной переменный и используем
метод `to_lowercase`. Каждый раз при её использовании создаётся новый экземпляр
`String`. Метод `contains` требует предоставить срез в качестве входных данных.

Т.к. мы приводим сравниваемые значение в один регистр неважно в каком регистре они
были изначально.

Проверим работу тестов:

```text
$ cargo test --lib

running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured
```

Отлично! Тесты срабатывают. Теперь мы можем использовать нашу новую функцию в `run`.
Сделаем рефакторинг кода. Добавим поле `case_sensitive: bool` в структуру `Config`:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

Дополнительное поле нужно для проверки в функции `run` для выбора какой функцией
воспользоваться для поиска: `case_sensitive` или `search_case_insensitive`. Пример
кода 12-22:


<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::error::Error;
# use std::fs::File;
# use std::io::prelude::*;
#
# fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
#      vec![]
# }
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
pub fn run(config: Config) -> Result<(), Box<Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

<span class="caption">Код программы 12-22: вызов функции в зависимости от значения
поля `config.case_sensitive`</span>

Следующим этапом нашего рефакторинга будет реализации проверки переменной среды в
функции `Config::new`.

Функции для работы с системными переменными находятся в модуле `env` стандартной
библиотеки. Добавим импорт данного модуля `use std::env;` в файле *src/lib.rs*.
Далее, мы можем использовать функцию `var` данного модуля для проверки значения
системной переменной `CASE_INSENSITIVE`. Пример кода 12-23:

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::env;
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }

// ...snip...

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

<span class="caption">Код программы 12-23: проверка системной переменной `CASE_INSENSITIVE`</span>

Мы создаём новую переменную `case_sensitive`. Для установки значения этой переменной
мы получаем значение системной переменной `CASE_INSENSITIVE`. Функция `var` возвращает
перечисление `Result`.

Метод `is_err` используется для проверки наличия ошибочного значения. Если ошибки
нет, код продолжает свою работу. Нас пока интересует только наличие данной переменой
с списке системных переменных.

Далее значение переменной `case_sensitive` используется для инициализации экземпляра
структуры  `Config`.

Сначала проверим работу программы без системной переменной и шаблоном поиска "to",
который должны подойти только точном совпадении символов:

```text
$ cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

Код работает. Далее установим системную переменную и попытаемся запустить программу
снова.
Looks like that still works! Now, let’s run the program with `CASE_INSENSITIVE`
set to 1 but with the same query “to”, and we should get lines that contain
“to” that might have uppercase letters:

```text
$ set CASE_INSENSITIVE=1 && cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Отлично! Теперь мы наши и больше "to". Теперь вы знаете, как устанавливать системные
переменные с помощью командной строки и как и получать к ним доступ.

Некоторые программы допускают использование обоих решений для установки конфигурации
(установки параметров командной строки и отслеживание значений системных переменных).
В качестве самостоятельной работы, пожалуйста, проверьте работу программы с помощью
аргументов командной строки и системных переменных. Для удаления системной переменной
используйте команду:

```text
$ set CASE_INSENSITIVE=

```

Модуль `std::env` содержит множество полезных функций для работы с системными
переменными. Подробнее о них вы можете узнать из документации.
