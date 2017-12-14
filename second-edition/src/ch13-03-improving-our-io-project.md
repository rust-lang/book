## Улучшение нашего проекта работы с системой ввода вывода (I/O Project)

Теперь, когда мы изучили возможности замыканий и итераторов мы можем улучшить
код проекта, который мы реализовывали в Главе 12. Мы сделаем код более кратким и
ясным. Мы улучшим код функций `Config::new` и `search`.

### Замена функции `clone` с помощью итератора

В коде (12-6) мы, получив срез строк и создав экземпляр структуры `Config`, мы
клонировали значения, чтобы передать их в поля структуры. Продемонстрируем этот код::

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
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

<span class="caption">Код 13-24: вид реализации функции `Config::new` из Главы 12</span>

<!--Is this why we didn't want to use clone calls, they were inefficient, or
was it that stacking clone calls can become confusing/is bad practice? -->
<!-- Yep, it's for performance reasons /Carol -->

К сожалению использование метода `clone` не является эффективным решением. Далее
мы покажем альтернативное решение.

Причина использования метода `clone` является необходимость получить возможность
полям экземпляра структуры владеть данными (в данном случае строковыми значениями).

Используя полученные знания об итераторах мы можем изменить содержание функции
`new`.

<!-- use the iterator functionality to what? How will iterating allow us to do
the same thing, can you briefly lay that out? -->
<!-- It's mostly for clarity and using a good abstraction, I've tried fixing
/Carol -->

Т.к.`Config::new` получает во владение итератор и не использует доступ по индексу.
Мы можем переместить знанчения `String` из итератора в `Config`.

<!-- below: which file are we in, can you specify here? -->
<!-- done /Carol -->

#### Использование итератора возвращаемого функцией `env::args`

В файле *src/main.rs* проекта Главы 12 изменим содержание функции `main`:

```rust,ignore
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
}
```

На код примера  13-25:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // ...snip...
}
```

<span class="caption">Код 13-25: удаление переменной `args` и направление результата
вызова функции `env::args` непосредственно в функцию `Config::new`</span>

<!-- I think, if we're going to be building this up bit by bit, it might be
worth adding listing numbers and file names to each, can you add those? Don't
worry about being accurate with the numbers, we can update them more easily
later -->
<!-- That's nice of you to offer, but since we're maintaining an online version
that we're keeping in sync with each round of edits, we need to keep the
listing numbers making sense as well. We'll just take care of them. /Carol -->

Обратите внимание, что функция `env::args` возвращает итератор! Вместо того, чтобы
преобразовывать значения итератора в вектор и затем направлять его в функцию
`Config::new`, мы передаём владение итератором из функции `env::args` непосредственно
в `Config::new`.

Далее, нам необходимо внести изменения в функцию `Config::new` в файле *src/lib.rs*:

<!-- can you give the filename here too? -->
<!-- done /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        // ...snip...
```

<span class="caption">Код 13-26: изменение описания функции `Config::new`</span>

Т.к. функция `env::args` возвращает итератор `std::env::Args`, мы используем его
для в описании входных данных.

#### Использование методов типажа `Iterator` вместо индексов

Далее мы вносим изменения в содержание функции `Config::new`. Т.к. `std::env::Args`
является итератором, т.е. реализует типаж `Iterator`, то он может использовать
все методы данного типажа:

<span class="filename">Filename: src/lib.rs</span>

```rust
# use std::env;
#
# struct Config {
#     query: String,
#     filename: String,
#     case_sensitive: bool,
# }
#
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    	args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query, filename, case_sensitive
        })
    }
}
```

<span class="caption">Код 13-27: Новое содержание функции `Config::new`</span>

<!-- is this the *full* new lib.rs code? Worth noting for ghosting purposes -->
<!-- No, this is just the `Config::new` function, which I thought would be
clear by saying "Next, we'll fix the body of `Config::new`.", can you elaborate
on why that's not clear enough? I would expect programmers to be able to
understand where a function starts and ends. /Carol -->

Обратите внимание, что первым элементом аргументов является имя программы, поэтому,
в данном случае, оно должно быть проигнорировано с помощью функции `next`. Следующий
вызов функции `next` вернет значение `query`, а последующий `filename`.

<!-- Hm, if ? would not work anyway, I'm not clear on why we mention, why it's
a shame we cant use it on Option? -->
<!-- We've taken this out, it's something that a portion of the readers might
be wondering and something that Rust might let you do someday, but yeah, it's
probably just distracting to most people /Carol -->

### Упрощаем код с помощью итераторов-адаптеров (Iterator Adaptors)

Следующая функция, которую мы можем улучшиться в нашем проекте - это `search`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

<span class="caption">Код 13-28: реализация функции `search` в Главе 12</span>

Мы можем сократить код этой функции благодаря использованию итераторов-адаптеров.
Также дополнительным плюсом этого решения станет удаление промежуточной переменной
`results`. Функциональный стиль программирования рекомендует минимизацию количества
изменяемых состояний. Это делает код устойчивым от ошибок. Удаление возможности
изменять вектор даст нам в будущем возможность реализовать параллельный поиск.
Код с изменениями 13-29 демонстрирует изменения:

<!-- Remind us why we want to avoid the mutable results vector? -->
<!-- done /Carol -->

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

<span class="caption">Код 13-29: использование методов итератора-адаптера</span>

Напомним, что целью функции `search` является возвращение всех строк из текста
`contents`, в которой содержится `query`. Функция `filter` решает задачу поиска,
а `collect` формирование вектора. Код стал проще, не правда ли?! Пожалуйста,
самостоятельно реализуйте подобное улучшение в функции `search_case_insensitive`.

<!-- what is that, here, only lines that contain a matching string? A bit more
context would help out, we probably can't rely on readers remembering all the
details I'm afraid -->
<!-- done /Carol -->

При наличии выбора стиля программирования, какой же лучше выбрать (13-28 или 13-29)?
Большинство программистов Rust выбирают второй вариант. Хотя, конечно, новичку
может этот стиль показаться сложнее для понимания, но чем больше у Вас будет опыта
работы с итераторами-адапторами, тем легче будет их использовать. Вместо циклов и
промежуточных переменных лучше использовать итераторы-адаптеры.

Но действительно ли эти конструкции равнозначны. Это вызывает сомнение. Рассуждения
по поводу производительности мы продолжим в следующей секции этой главы.
