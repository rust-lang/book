## Обрабатываемы ошибки и `Result`

Множестов ошибок не являются настолько критичными, чтобы останавливать выполнение
программы. Весьма часто необходим просто правильная их обработка. К примеру, при
открытии файла может произойти ошибка из-за отсутствия файла. Решения могут быть
разные: от игнорирования до создания нового файла.

Надеюсь, что вы ещё помните содержание главы 2, где мы рассмотривали перечисление
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

Здесь мы сообщаем значение `Ok` содерживт знечение типа `File` `file`.
Другое значение может хранить значение типа `Err`. В этом примере мы исполуем
вызов макроса `panic!`. Если нет файла с именем *hello.txt*, будет выполнен этот код.
Следовательно, будет выведено следующее сообщение:

```text
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:8
```

### Обработка различных ошибок

Пример создание нового файла при отсутсвии запрашиваемого файла:

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
    print!("{:?}",f);
}

```

<span class="caption">Listing 9-4: Обработка различных ошибок несколькими способами</span>


### Сокращенные макросы обработки ошибок `unwrap` и `expect`

Метод `unwrap` - это оболочнка выражения `match`, которая возвращает `Ok` или `Err`.

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    print!("{:?}", f);
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

Мы используем `expect` таким же образом, каки и `unwrap`: возвращаем ссылку на файл или
вызов макроса `panic!`.

### Генерировани ошибок

Когда вы пишите функцию, в результате работы которой может произойти непредвиденная
ошибка, вместо того, чтобы обрабатывать эту ошибоку вы можите создать подробноое
описание этой и передать ошибку по цепочке на верхний уровень обработки кода.


For example, Listing 9-5 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called this function:

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



The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value returned with a `match` similar to the `match` in
Listing 9-3, only instead of calling `panic!` in the `Err` case, we return
early from this function and pass the error value from `File::open` back to the
caller as this function’s error value. If `File::open` succeeds, we store the
file handle in the variable `f` and continue.

Then we create a new `String` in variable `s` and call the `read_to_string`
method on the file handle in `f` in order to read the contents of the file into
`s`. The `read_to_string` method also returns a `Result` because it might fail,
even though `File::open` succeeded. So we need another `match` to handle that
`Result`: if `read_to_string` succeeds, then our function has succeeded, and we
return the username from the file that’s now in `s` wrapped in an `Ok`. If
`read_to_string` fails, we return the error value in the same way that we
returned the error value in the `match` that handled the return value of
`File::open`. We don’t need to explicitly say `return`, however, since this is
the last expression in the function.

The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. We
don’t know what the caller will do with those values. If they get an `Err`
value, they could choose to call `panic!` and crash their program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the caller is actually trying
to do, so we propagate all the success or error information upwards for them to
handle as they see fit.

This pattern of propagating errors is so common in Rust that there is dedicated
syntax to make this easier: `?`.

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

<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `?`</span>

The `?` placed after a `Result` value is defined to work the exact same way as
the `match` expressions we defined to handle the `Result` values in Listing
9-5. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression and the program will continue. If the value
is an `Err`, the value inside the `Err` will be returned from the whole
function as if we had used the `return` keyword so that the error value gets
propagated to the caller.

In the context of Listing 9-6, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `f`. If an error occurs, `?`
will return early out of the whole function and give any `Err` value to our
caller. The same thing applies to the `?` at the end of the `read_to_string`
call.

The `?` eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`:

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

We’ve moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn’t changed. Instead of creating a variable `f`, we’ve
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-5 and
Listing 9-6, this is just a different, more ergonomic way to write it.

### `?` Can Only Be Used in Functions That Return `Result`

The `?` can only be used in functions that have a return type of `Result`,
since it is defined to work in exactly the same way as the `match` expression
we defined in Listing 9-5. The part of the `match` that requires a return type
of `Result` is `return Err(e)`, so the return type of the function must be a
`Result` to be compatible with this `return`.

Let’s look at what happens if we use `?` in the `main` function, which you’ll
recall has a return type of `()`:

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

When we compile this, we get the following error message:

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

This error is pointing out that we have mismatched types: the `main` function
has a return type of `()`, but the `?` might return a `Result`. In functions
that don’t return `Result`, when you call other functions that return `Result`,
you’ll need to use a `match` or one of the `Result` methods to handle it,
instead of using `?` to potentially propagate the error to the caller.

Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.
