## Обрабатываемы ошибки и `Result`

Множество ошибок не являются настолько критичными, чтобы останавливать выполнение
программы. Весьма часто необходим просто правильная их обработка. К примеру, при
открытии файла может произойти ошибка из-за отсутствия файла. Решения могут быть
разные: от игнорирования до создания нового файла.

Надеюсь, что вы ещё помните содержание главы 2, где мы рассматривали перечисление
`Result`. Оно имеет два значения `Ok` и `Err`.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` и `E` параметры перечисления. `T`  - это тип, которые будет возвращён, при
успехе, а `E` при ошибке.

Let’s call a function that returns a `Result` value because the function could
fail: opening a file, shown in Listing 9-2.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<span class="caption">Listing 9-2: Opening a file</span>

Интересно, как узнать, какой тип возвращает метод `File::open`. Это просто. Надо
поставить тип  данных, который точно не подойдет и увидим тип данных в описании
ошибки:
```rust,ignore
let f: u32 = File::open("hello.txt");
```

Информационное сообещение:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
  = note:    found type `std::result::Result<std::fs::File, std::io::Error>`
```

Всё, я думаю, ясно из описания.

Для обработки исключительной ситуации необходимо добавить следующий код:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

<span class="caption">Listing 9-3: Использование выражения `match` для обработки
`Result`</span>

Обратите внимание, что перечисление `Result`, также как `Option` входит в состав
экспорта по умолчанию.

Здесь мы сообщаем значение `Ok` содержит значение типа `File` `file`.
Другое значение может хранить значение типа `Err`. В этом примере мы используем
вызов макроса `panic!`. Если нет файла с именем *hello.txt*, будет выполнен этот код.
Следовательно, будет выведено следующее сообщение:

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

### Обработка различных ошибок

Пример создание нового файла при отсутствии запрашиваемого файла:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
    print!("{:#?}",f);
}

```

<span class="caption">Listing 9-4: Обработка различных ошибок несколькими способами</span>


### Сокращенные макросы обработки ошибок `unwrap` и `expect`

Метод `unwrap` - это оболочка выражения `match`, которая возвращает `Ok` или `Err`.

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    print!("{:#?}", f);
}

```

Если мы выполним код без наличия файла *hello.txt*, будет выведена ошибка:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
/stable-dist-rustc/build/src/libcore/result.rs:868
```

Есть ещё один метод похожий на `unwrap`. Используя `expect` вместо `unwrap` и
предоставляющей хорошие информативные описания ошибок::

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    print!("{:?}", f);
}
```

Мы используем `expect` таким же образом, каким и `unwrap`: возвращаем ссылку на файл или
вызов макроса `panic!`.

### Генерировании ошибок

Когда вы пишите функцию, в результате работы которой может произойти непредвиденная
ошибка, вместо того, чтобы обрабатывать эту ошибку вы можете создать подробное
описание этой и передать ошибку по цепочке на верхний уровень обработки кода.


Например, код программы 9-5 читает имя пользователя из файла. Если файл не существует
или не может быть прочтён, то функция возвращает эти ошибку в код, которые вызвал
эту функцию:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

<span class="caption">Listing 9-5: Эта функция, которая возвращает ошибки вызова
выражения `match`</span>

Давайте, рассмотрим тип возвращаемого значения `Result<String, io::Error>`.Если
эта функция будет выполнена успешно, будет возвращено `Ok`, содержащее значение
типа `String`. Если же при чтении файла будут какие-либо проблемы - `io::Error`.

Тело этой функции начинает с вызову функции `File::open`. Далее мы получаем результат
анализа результата чтения файла функцией `match`. Если функция `File::open` сработала
успешно, мы сохраняет ссылку на файл в переменную `f` и программа продолжает свою
работу.

Далее, мы создаём строковую переменную `s` и вызываем метод файла `read_to_string`,
которая читает содержание файла, как строковые данные в переменную `s`. Результатом
работы этой фунции будет знанчение перечисления `Result`: `Ok` или `io::Error`.

Этого же результата можно достичь с помощью сокращенного написания (с помощью использования
символа `?`).

### A Shortcut for Propagating Errors: `?`

Listing 9-6 shows an implementation of `read_username_from_file` that has the
same functionality as it had in Listing 9-5, but this implementation uses the
question mark operator:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<span class="caption">Код программы 9-6: Пример функции, которая возвращает ошибку,
используя символ `?`</span>

Благодаря и использованию символа `?` сокращается запись кода (код, написанный в
предыдущем примере создаётся компилятором самостоятельно).

В коде примера 9-6 в первой строке функция `File::open` возвращает содержимое значения
перечисления `Ok` в переменную `f`. Если же в при работе этой функции происходит
ошибка, будет возвращен экземпляр структуры `Err`. Те же самые действия произойдут
при чтении текстовых данных из файла с помощью функции `read_to_string`.

Использование сокращенных конструкций позволят уменьшить количество строк кода и
 место потенциальных ошибок. Написанный в предыдущем примере сокращенный код можно
 сделать ещё меньше с помощью сокращения промежуточных переменных и конвейерного вызова
 методов:

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

Мы перенесли создание экземпляра структуры `String` в начало функции. Вместо того,
чтобы создавать переменную `f` мы последовательно вызываем методы экземпляров
выходные данных.

### Ограничения использования `?`

Сокращенную запись с помощью символа `?` можно использовать в функциях, которые
возвращают значение перечисления `Result`. Соответственно, если функция не возвращает
значение перечисления `Result`, а в коде написано обратное - компилятор сгенерирует
ошибку. Пример:

```rust,ignore
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

<!-- NOTE: as of 2016-12-21, the error message when calling `?` in a function
that doesn't return a result is STILL confusing. Since we want to only explain
`?` now, I've changed the example, but if you try running this code you WON'T
get the error message below.
I'm bugging people to try and get
https://github.com/rust-lang/rust/issues/35946 fixed soon, hopefully before this
chapter gets through copy editing-- at that point I'll make sure to update this
error message. /Carol -->

Описание ошибки:

```text
error[E0308]: mismatched types
 -->
  |
3 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum
`std::result::Result`
  |
  = note: expected type `()`
  = note:    found type `std::result::Result<_, _>`
```

В описании ошибки сообщается, что функция `main` должна возвращать кортеж, а вместо
этого - функция возвращает `Result`.

В следующем разделе будет рассказано об особенностях вызова макроса `panic!`, приведены
рекомендации при выборе конструкции для отслеживания ошибок.
