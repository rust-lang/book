## Как работают функции

Функция - это ключевые части Rust кода. Вы, конечно, знакомы с самой важной функцией.
Это функция `main`, которая является точкой входа в программу. Также вы уже познакомились
с ключевым словом `fn` - обозначающее начало объявления функции.

В Rust используется т.н. "змеиный" стиль написания функций и переменных: это когда
все слова пишутся в нижнем регистре и слова в многословных обозначениях разделяются
нижним подчёркиванием. Пример объявления функции:

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```
Обозначение функции состоит из группы скобок после имени функции. В фигурных скобках
заключено тело функции.

Мы можем вызвать функцию по её имени. Обратите внимание, что мы можем объявить функцию
как после, так и до её вызова.

Давайте рассмотрим работы с функциями на практическом примере. Пожалуйста, создайте,
проект *functions*.  Пожалуйста выполните код программы. Сейчас мы рассмотрим
содержания строки вывода.

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
Hello, world!
Another function.
```

Как вы видите, код программы выполняется последовательно. Сначала выполняется печать
текста, а потом вызывается функция, которая также выводит на печать текст.

### Параметры функции

При объявлении функции могут быть определены входные параметры. Конкретные значения,
которые посылаются в функцию, называются аргументами.

Пример определения функции с параметром:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

Работа программы в терминальной строке:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

Обратите внимание на описание параметра `x` при объявлении функции `another_function`.
Тип параметра `i32`. Когда аргумент функции `5` передаётся на вход, это значение используется
в её теле.

При объявлении нескольких входных параметров, они разделяются запятыми:

<span class="filename">Файл: src/main.rs</span>

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```
Думаю, что тут всё ясно и понятно и подробные разъяснения излишни.

Работа кода `cargo run`:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
The value of y is: 6
```

### Внутреннее содержание функций

Важной особенностью Rust является тот факт, что в язык разделяет понятия операторы
и выражения. Последующий материал поможет во всём этом разобраться.

### Выражения и высказывания

Операторы (*Statements*) - это инструкци, которые выполняют действия, но не возвращают
значение. Выражения возвращают значение.

Рассмотрим использование этого теоритического материала на примере.

Создадим переменную и присвоим ей значение. `let y = 6;` - это оператор (a statement).

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let y = 6;
}
```

<span class="caption">Listing 3-3: Объявление функции `main`, содержащей один оператор.</span>

Исходя из вышесказанного, оператор не может быть присвоен переменной:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = (let y = 6);
}
```

Это код с ошибкой:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

Это важное отличие от других языков программирования.

В нашем примере `let y = 6`, `6` - это выражение, которое возвращает конкретное
значение. Вызов функции, макроса, блок кода - это всё выражения.

Пример:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

Блок кода (варажение):

```rust,ignore
{
    let x = 3;
    x + 1
}
```

возвращающий `4`. Возвращаемое значение присваивается переменной `y`. Обратите внимание,
что выражения не заканчиваются символом `;`. Если вы добавите к выражению символ
`;` - это оно превратиться в оператора.

### Функции возвращающие значения

Функции могут возвращать значения. Определение возвращаемых значений следует после
символа `->`. Возвращаемым значением функции является последнее выражение. Пример:

<span class="filename">Filename: src/main.rs</span>

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

Обратите внимание на тело функции `five`! Функция имеет определение типа возвращаемого
значения. Внутри только выражение `5`. Этого достаточно для корректной работы функции:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
     Running `target/debug/functions`
The value of x is: 5
```

Обратите внимание, что возвращаемое значение функции присваевается переменной при
её инициализации:

```rust
let x = 5;
```

Рассмотрим ещё один пример::

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Будет выведена строка `The value of x is: 6`. Какое значение будет присвоено
переменной, если в конце выражения будет символ `;`? Давайте проверим:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

Будуте ошибка при компиляции с весьма доходчивым описанием, как же эту ошибку можно
исправить:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
help: consider removing this semicolon:
 --> src/main.rs:8:10
  |
8 |     x + 1;
  |          ^
```

Для того, чтобы исправить ошибку просто удалите символ `;`.
