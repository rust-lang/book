## Ссылочные переменные и заимствование

Необходимость использования кортежа в коде (в последнем примере предыдущей секции главы 4),
обусловлена дальнейшим использованием переменной типа `String`. Для этого мы должны
вернуть владение из метода обратно. Т.е. метод `calculate_length` помимо результата
должен вернуть входной параметр назад из функции.

А теперь приведём пример использования передачи в метод ссылки для решения этой задачи.
При таком решении, возвращать кортеж нет необходимости. Владение не будет передано
внутрь метода:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Обратите внимание:
1. Кортеж удалён из декларирования переменной и возвращаемых данных метода.
2. Переменная метода имеет ссылочный тип `&String`.

`&` обозначает что тип данных ссылочный и поэтому передавать владение не нужно.
Иллюстрация работы 4-8.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figure 4-8: `&String s` ссылка на `String s1`</span>

Давайте подробнее рассмотрим механизм вызова функции:

```rust
# fn calculate_length(s: &String) -> usize {
#     s.len()
# }
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

Синтаксическая конструкция `&s1` создаёт ссылку на переменную `s1`. Передачи ей
владения не происходит. Т.к. нет передачи владения, переменная не удаляется из
области видимости, её статус не изменяется.

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

Важное замечание - переменные ссылочного типа никогда не имеют владения, поэтому
не влияют на него.

В Rust передача ссылки в функцию в качестве параметра называется заимствованием.
Всё как в жизни, после того как что-то взято на время, потом надо это отдать.

А что произойдёт, если попытаться изменить то, что было заимствовано? Проверим на
примере 4-9 (предупреждение - это код с ошибкой):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<span class="caption">Listing 4-9: Попытка модификации заимствованной переменной</span>

Here’s the error:

```text
error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

Как и переменные, ссылочные переменные неизменяемые. Т.е. изменять данные ссылки
нельзя.

### Изменяемые ссылочные данные

Для того, чтобы исправить ошибку в предыдущем примере 4-9 необходимо сделать небольшие
изменения в коде:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Для того, чтобы данные ссылочной переменной можно было изменить, также как и обычную
переменную её надо сделать изменяемой при инициализации. Это делается с помощью
префикса `mut`. В данному случае, в качестве параметра функции нужно написать `&mut s`.

Изменяемое ссылочная переменная имеет значительное ограничение: у одной переменной
может быть только одна изменяемая ссылочная переменная в данной области видимости.
Такой код не будет скомпилирован:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
  let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}",r1);
println!("{}",r2);
}

```

Описание ошибки:

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

Это ограничение позволяет изменять данные, но в тоже время позволяет всё держать
под контролем. Это немного удивляет, но всё это сделано для минимизации ошибок.

Это ограничение не даёт появлению эффекта гонки. Ошибки такого рода трудноуловимы
и сложны.

Создание вложенных областей видимости, может быть полезным (при необходимости).

```rust
fn main() {
  let mut s = String::from("hello");

{
let r1 = &mut s;
println!("{}",r1);
}
{
let r2 = &mut s;
println!("{}",r2);
}
}
```
Подобное правило действительно и для комбинации изменяемых и неизменяемых
ссылочных переменных. Пример кода с ошибкой:

```rust
fn main() {
  let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &mut s;
println!("{}",r1);
println!("{}",r2);
println!("{}",r3);
}


```

Here’s the error:

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Обратите внимание, что вы не можете иметь изменяемой ссылочной переменной пока
существует изменяемая переменная.  Наличие множества неизменяемых переменных допускается,
т.к. они не могут изменить данные.

Статической анализатор кода помогает предотвратить таким образом возможные скрытые
ошибка в коде. Такие ошибки легко устранить.

### Недействительный ссылки

Работая с ссылками весьма легко создать недействительную ссылку или ссылку на
участок памяти, который уже или ещё используется другими переменными приложения.
Rust компилятор гарантирует защиту от создания подобных ссылок.

Попытаемся смоделировать подобную ошибку и посмотрим как с ней справится компилятор:
Let’s try to create a dangling reference:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

```

Текст в строке терминала:

```text
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^^^^^^^
  |
  = help: this function's return type contains a borrowed value, but there is no
    value for it to be borrowed from
  = help: consider giving it a 'static lifetime

error: aborting due to previous error
```

Эта ошибка сообщает об ещё не освещённой нами опции языка Rust - *времени жизни переменной*.
Мы расскажем подробнее о этой опции в главе 10. Также компилятор сообщил кое-что
ещё:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Давайте рассмотрим что же происходит во время работы кода, который создаёт недействительные
ссылки:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); //создаётся новая переменная s типа String

    &s // вы возвращаем ссылку на созданную строку. При выходе из области видимости

}
  // переменная становится недействительной и удаляется. Ссылка становится недействительной!
```

Т.к. переменная создаётся внутри метода, когда область действия метода заканчивается,
`s` удаляется. Но код метода пытается возвратить ссылку на эту недействительную
переменную. Это ошибка, которую компилятор Rust предотвращает.

Исправлением ошибки в данном случае будет возвращение из функции самой созданной
переменной, а не ссылки на неё:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

Это решние прекрасно работает, т.к. соблюдаются правила владения.

### Правила работы с ссылками

Список правил:

1. В одной области видимости единовременно может существовать только один тип ссылочных переменных на одни данные:
  - одна изменяемая ссылка на данные,
  - любое количество неизменяемых ссылок на данные.
2. Все ссылки должны быть действительными.

Сейчас вы рассмотрели ссылку на данные (на переменную).
В следующей главе мы рассмотрим другой тип ссылочных переменных - динамические массивы.
