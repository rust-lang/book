## Динамические массивы

*Динамический массив*  - это ссылочный тип не использующий владение.
Это непрерывная коллекция упорядоченных элементов.

Рассмотрим учебную задачу. Необходимо написать функцию, входным параметром которой
является строка. Выходным значением функции является первое слово, которое будет
найдено в этой строке. Если функция не находит разделителя слов (пробела), она
возвращает эту строку.

Прежде всего рассмотрим описание этой функции:

```rust,ignore
fn first_word(s: &String) -> ?
```

Функция `first_word` имеет входной параметр типа `&String`. Нам не нужно владение
переменной для её работы, так что это то что нам нужно. Для решения задачи мы можем
найти индекс конца строки в тексте. Вот как это можно сделать с помощью функции 4-10:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main(){
let index = first_word(&String::from("hello, Nik!"));
println!("{}",index);
}
```

<span class="caption">Listing 4-10: Пример функции `first_word`, которая возвращает
index пробела в строке типа `String`</span>

Теперь давайте изучим код этой функции. Для нахождения пробела в строке необходимо
необходимо превратить в массив байтов используя метод `as_bytes`:

```rust,ignore
let bytes = s.as_bytes();
```

Далее, используя метода массива `iter()` мы создаём объект для последовательного
перебора содержания массива - итератор. Далее, используя цикл `for`, мы перебираем
байты и анализируем каждый из них. Обратите внимание, что при каждой итерации мы
получаем индекс элемента и ссылку на него:

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

Мы будет изучать итераторы более детально в главе 13. Сейчас, достаточно понять,
что метод `iter`, который возвращает каждый элемент коллекции. Метод `enumerate`
передаёт результаты работы метода `iter` в кортеж. Первый элемент этого кортежа
возвращает индекс, второй элемент - ссылку на элемент. Такой способ перебора элементов
массива наиболее удобный.

Так как метод `enumerate` возвращает кортеж, мы можем использовать шаблон создающий
переменные, которые в дальнейшем можно использовать внутри тела цикла.

Нам надо найти байт, который представляет собой значение пробела. Для этого мы
приводим символьную константу ' ' к типу байт *b' '*. В выражении `if` мы сравниваем
полученное таким образом константное значение с текущим байтом из массива.

Если мы находим пробел, вы возвращаем позицию пробела. Иначе мы возвращаем длину
массива `s.len()`:

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

Таким образом мы получаем искомое значение. Но оно может устареть в будущем  4-11:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &String) -> usize {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return i;
#         }
#     }
#
#     s.len()
# }
#
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5.

    s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

<span class="caption">Listing 4-11: Сохранение результата вызова функции `first_word`,
потом изменяем содержимое переменной `s`</span>

Эта программа скомпилирует без всяких проблем.

Создадим ещё одну функцию, которая возвращает индексы начала и конца первого слова.
Вот как будет выглядеть её описание:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Обратите внимание, что весьма сложно удерживать в синхронном состоянии вcе эти переменные
(входящие и исходящие). Для этих целей существуют динамические массивы.

### Строковые динамические массивы

Строковый динамический массив - это ссылка на часть строки `String` и её инициализация
выглядит следующим образом:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

Ота инициализация похожа на создание ссылки на переменную `String`, но с дополнительными
условиями - указанием отрезка `[0..5]`. Вместо целой переменной мы получаем ссылку
на её часть. Начало и конец отрезка включено в динамический массив, а вот окончание
нет.

Мы можем создавать динамические массивы используя определение отрезка `[starting_index..ending_index]`.
Внутренне, переменная типа динамический массив устроена следующим образом:
начальная позиция, длина отрезка.

Рисунок 4-12.

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-12: Динамический массив ссылается на часть
`String`</span>

Синтаксис Rust позволяет упростить описание динамического массива, если он начинается
с 0-го индекса:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

Таким же образом можно поступить с последним элементом, если это последний байт в
`String`:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

Таким образом динамический массив целого массива можно описать так:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

Применим полученные знания и перепишем метод `first_word`. Для представления
динамического массива строк существует короткая запись `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Теперь, вызвав метод `first_word`, мы получим один объект, которые включает в себя
всю необходимую информацию.

Аналогичным образом можно переписать и второй метод `second_word`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

Благодаря использованию динамических массивом нельзя изменить данные строки, если
на неё ссылается динамический массив (т.к. это может привести к ошибке):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Ошибка компиляции:

```text
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

Благодаря соблюдению правил, Rust просто исключает класс подобных ошибок.

#### Строковые константы и динамические массивы

Вооружившись знаниями о динамических массивах по-новому можно посмотреть на
инициализацию переменной строкового типа:

```rust
let s = "Hello, world!";
```

Тип `s` является `&str` - это динамический массив бинарных данных специального вида.
Поэтому строковой литерал неизменяемый, а тип `&str` это неизменяемая ссылка.

#### Строковые динамические массивы как параметры

Используя строковые динамические массивы, как параметры вы можете улучшить
код наших методов:

```rust,ignore
fn first_word(s: &String) -> &str {
```

Также можно записать этот код следующим образом:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Если мы используем динамический массив, мы может его передавать в методы.
Использование динамических массивов вместо переменных делает код боле удобным:

<span class="filename">Filename: src/main.rs</span>

```rust
# fn first_word(s: &str) -> &str {
#     let bytes = s.as_bytes();
#
#     for (i, &item) in bytes.iter().enumerate() {
#         if item == b' ' {
#             return &s[0..i];
#         }
#     }
#
#     &s[..]
# }
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Другие динамические массивы

Существую также динамические массивы общего типа. Рассмотрим массив:
```rust
let a = [1, 2, 3, 4, 5];
```
Создадим динамический массив:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

Этот динамический массив имеет тип данных `&[i32]`. Мы поговорим о таком типе
коллекций в главе 8.

## Итоги

Такие концепции как владение, заимствование и динамические массивы - это способы
защиты использования памяти.  Rust даёт вам возможность контролировать использование
памяти.

Владение влияет на множество других концепций языка Rust.
В следующей главе мы рассмотрим способ группировки данных в  `struct`.
