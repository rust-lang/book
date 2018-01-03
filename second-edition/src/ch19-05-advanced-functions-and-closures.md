## Дополнительные сведения о функциях и замыканиях

И наконец рассмотрим дополнительные сведения о функциях и замыканиях: указателях
на функции, наследующие функции и возвращение замыканий.

Finally, let’s discuss some advanced features having to do with functions and
closures: function pointers, diverging functions, and returning closures.

### Указатели на функции

Вы уже знаете, как отправлять замыкания в функцию. Теперь мы рассмотри отправление
функции в функцию. Тип функции `fn`. Типаж функции `Fn`. `fn` является указателем
на функцию.

<span class="filename">Filename: src/main.rs</span>

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

<span class="caption">код 19-34: использование типа `fn`</span>

будет напечатано `The answer is: 12`. Мы установили параметра `f` функции `do_twice`,
как  `fn`, который получает один параметр типа `i32` и возвращает `i32`. Далее
мы можем вызвать `f` в теле функции `do_twice`. В `main` мы можем отправить имя
функции `add_one`, как первый аргумент в `do_twice`.

В отличии от замыканий, `fn` является типом, а не типажом. Поэтому мы определяем
`fn`, как тип данных параметра.

Указатели на функции реализуют все три типажа замыканий (`Fn`, `FnMut` и `FnOnce`).
Поэтому мы всегда можем отправить указатель на функцию в качестве аргумента.
Предпочтительнее создавать функции используя обобщенные типы и типажи замыканий.
Единственное где вам обязательно могут понадобиться указатели на функции - это при работе
с внешними функциями (например, C-функциями).

Например, если мы хотим использовать функцию `map`, чтобы преобразовать числовой
вектор в строковый вектор, мы можем использовать замыкания:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```

Или мы можем использовать имя функции в качестве аргумента в `map` вместо замыкания:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

Использовать замыкания или имена функций - выбор вкуса. Результат будет одинаковый.

### Возвращение замыканий

Т.к. замыкания представляют типажами, возвращение замыканий немного сложно, т.к.
мы не можем этот сделать явным образом.
Because closures are represented by traits, returning closures is a little
tricky; we can’t do it directly. In most cases where we may want to return a
trait, we can instead use the concrete type that implements the trait of what
we’re returning as the return value of the function. We can’t do that with
closures, though. They don’t have a concrete type that’s returnable; we’re not
allowed to use the function pointer `fn` as a return type, for example.

Этот код не скомпилируется:

```rust,ignore
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```

Описание ошибки:

```text
error[E0277]: the trait bound `std::ops::Fn(i32) -> i32 + 'static:
std::marker::Sized` is not satisfied
 --> <anon>:2:25
  |
2 | fn returns_closure() -> Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^ the trait `std::marker::Sized` is
  not implemented for `std::ops::Fn(i32) -> i32 + 'static`
  |
  = note: `std::ops::Fn(i32) -> i32 + 'static` does not have a constant size
  known at compile-time
  = note: the return type of a function must have a statically known size
```

Опять типаж `Sized`! Rust не знает размер для хранения замыкания заранее. Для решения
задачи используем объект типажа:

```rust
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## Итоги

Вуф! К этой части книги мы рассмотрели возможности Rust, которые не часто используются,
но доступны если ни вам понадобятся. Мы обзорно рассмотрели множество сложных тем.
Мы надеемся, что когда вы столкнетесь с описанными трудностями, вы сможете найти
решение, или, по крайней мере, будет знать что и где искать.

Теперь давайте воспользуемся всем тем, что мы изучили на протяжении всей книги,
реализуем ещё один проект!
