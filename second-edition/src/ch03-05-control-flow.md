## Управление выполнением кода

Решение выполнять ту или иную часть кода зависит от логических условий. Конструкция
`if` и циклы является в Rust такими управляющими выражениями.

### `if`-выражения

if`-выражение позволяет создавать ветви кода работающие при соблюдение определенных
логических условий.

Создадим новый проект *branches* и добавить следующий код:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

Все `if`-выражения начинаются с ключевого слова `if`. Далее следует логическое выражение.
При необходимости, после тела блока `if` может следовать `else`-выражение.

Результат работы программы:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was true
```

Если изменить значение переменной на большее, например на `7` программа выполнит
другую ветвь кода:

```rust,ignore
let number = 7;
```

Результат работы программы:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
condition was false
```

Тип `if`-выражение должен быть логическим:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

Иначе будет ошибка при компиляции:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected bool, found integral variable
  |
  = note: expected type `bool`
             found type `{integer}`
```

В Rust нельзя автоматически конвертировать в `bool` тип данных. Исправим ошибку
в коде:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

Будет выведено следующая строка `number was something other than zero`.

#### Использование выражений `else if`

`if`-выражения могут быть достаточно сложными. Например:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Результат работы программы:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
number is divisible by 3
```

Для кода с большим количеством `else if`-выражений большое подойдёт конструкция `match`.
О ней мы расскажем в главе 6.

#### Использование `if` в `let`-операторах

По определению выражений, мы можем их использовать в операторах.
Пример, Listing 3-4:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

<span class="caption">Listing 3-4: Присвоение результата `if`-выражения переменной
при её инициализации</span>

Результат работы программы:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
     Running `target/debug/branches`
The value of number is: 5
```

Обратите внимание, что у данной инициирующей конструкции есть ограничения. Все возвращаемые
значения `if`-выражения должны иметь один тип данных.
Пример:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

Результат работы программы:

```text
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integral variable, found reference
  |
  = note: expected type `{integer}`
             found type `&'static str`
```

Такое поведение кода делает вашу работу предсказуемой и хранить от потенциальных
ошибок.

### Повторение выполнения кода с помощью циклов

Для многократного выполнения кода существуют *циклы*. В Rust существует несколько
видов циклов. Создадим проект *loops*.

Виды циклов: `loop`, `while` и `for`. Рассмотрим каждый в отдельности.

#### `loop`

Ключевое слово тело `loop` - это бесконечный цикл.

Пример:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    loop {
        println!("again!");
    }
}
```

Остановить выполнения кода можно лишь аварийно завершив работу программы сочетанием
клавиш <span class="keystroke">ctrl-C</span>.

Пример:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

Существует ключевое слово `break`, благодаря которому можно остановить работу бесконечного
цикла.

#### `while`

Цикл с условием `while` выполняет тело пока выполняется логическое условие параметра.

Пример:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

#### `for`

Использование `while`-цикла для перебора элементов массива:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
```

<span class="caption">Listing 3-5: Перебор элементов массива с помощью цикла `while` loop</span>

Этот код выполняет тело цикла пока переменная `index` меньше 5:

```text
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
the value is: 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

Такой код выполняется весьма медленно и никак не защищён от ошибки во время работы
программы. Для этих целей больше подходит другая конструкция - `for`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

<span class="caption">Listing 3-6: Перебор элементов коллекции с помощью цикла
`for`</span>

Данный код более защищён от ошибок.

Безопасный цикл `for` наиболее часто используется в Rust.

Реализация обратного осчёта в Rust:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## Summary

Вы познакомились с конструкциями управления ходом выполнения программы, а также с
рекомендациями разработчиков Rust.
Для закрепления изученного материала мы предлагаем написать следующие программы:

* Конвертер температур из Фаренгейта в Цельсий.
* Генератор чисел Фибоначчи.
* Генератор строк сказки "12 дней Рождества".

В следующей главе мы расскажем о владении - концепции языка программирования Rust.
