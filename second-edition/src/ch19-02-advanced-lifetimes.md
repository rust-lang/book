## Расширенные модификаторы времени жизни (МВЖ) (Lifetimes)

В главе 10 мы изучили как аннотировать ссылочные переменные с помощью МВЖ для
помощи компилятору понять какие взаимосвязи существуют между данными. В этой секции
мы рассмотрим ещё не освященные опции МВЖ:  *подтипы*, *границы* и *объекты-типажи*.

### Подтипы

Представьте, что мы хотим реализовать текстовый анализатор (parser). Для этого
необходимо создать структуру, экземпляры которой будут хранить ссылки на строку, которую
мы анализируем. Назовём эту структуру `Context`. Мы создадим анализатор, который
будет анализировать эту строку и возвращать индикатор успеха или неудачи. Анализатору
необходимо заимствовать сороку для анализа. Реализация может быть похожа на код
19-12, который не скомпилируется, т.к. мы не указали МВЖ:

```rust,ignore
struct Context(&str);

struct Parser {
    context: &Context,
}

impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<span class="caption">код 19-12: определение структуры `Context`, которая содержит
строковый срез. Структура `Parser` содержит ссылку на экземпляр `Context`. Метод
`parse` всегда возвращает ошибку со ссылкой на строковый срез</span>

Для простоты функция `parse`  возвращается `Result<(), &str>`. Поэтому мы ничего
не делаем для успешной работы и при ошибке возвращаем часть строки, которая не смогла
быть обработана. Реальные функции такого рода должны содержать больше информации.

Как же записаь МВЖ для строкового среза в `Context` и ссылку в `Parser`? Очевидным
решением является использование МВЖ везде:

```rust
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
```

<span class="caption">код 19-13: аннотирование ссылок в `Context` и `Parser`</span>

Этот код скомпилируется. Следующий код (19-14) декларирует функцию, которая получает
входной параметр `Context` и используя `Parser` для анализа текста.

```rust,ignore
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<span class="caption">Listing 19-14: попытка добавить функцию `parse_context`,
которая получает `Context` и использует `Parser`</span>

При попытке компиляции мы получаем две хорошо описанные ошибки:

```text
error: borrowed value does not live long enough
  --> <anon>:16:5
   |
16 |     Parser { context: &context }.parse()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ does not live long enough
17 | }
   | - temporary value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the
body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^

error: `context` does not live long enough
  --> <anon>:16:24
   |
16 |     Parser { context: &context }.parse()
   |                        ^^^^^^^ does not live long enough
17 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the
body at 15:55...
  --> <anon>:15:56
   |
15 |   fn parse_context(context: Context) -> Result<(), &str> {
   |  ________________________________________________________^
16 | |     Parser { context: &context }.parse()
17 | | }
   | |_^
```

Эти ошибки говорят, что время жизниа `Parser`, который мы создаём и параметра метода
заканчивается после завершения работы функции. Но они должны продолжать жить далее.

Т.е. для `Parser` и `context` необходимо пережить ( *outlive*) работы функции и
быть действительными после.

Давайте рассмотрим определение функции `parse` в 19-13 снова. Обратим внимание на
сигнатуру метода:

```rust,ignore
    fn parse(&self) -> Result<(), &str> {
```

Напомним, что при использовании ПВЖ описание должно иметь вид:

```rust,ignore
    fn parse<'a>(&'a self) -> Result<(), &'a str> {
```

Проблема этой функции в том, что время жизни экземпляра `Parser` заканчивается после
завершения функции и время жизни параметра также.

Необходимо описать время жизни таким образом, чтобы срез в `Context` и ссылка в
`Parser` имели бы разные времена жизни и возвращаемое значение было связано с временем
жизни среза.

Мы можем попытаться дать `Parser` и `Context` различные параметры времени жизни.
К сожалению, это решение не решит полностью проблему, но это всё же правильный путь
на пути к решению:

```rust,ignore
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

<span class="caption">код 19-15: определение различных переменных времени жизни</span>


We’ve annotated the lifetimes of the references in all the same places that we
annotated them in Listing 19-13, but used different parameters depending on
whether the reference goes with the string slice or with `Context`. We’ve also
added an annotation to the string slice part of the return value of `parse` to
indicate that it goes with the lifetime of the string slice in `Context`.

Here’s the error we get now:

```text
error[E0491]: in type `&'c Context<'s>`, reference has a longer lifetime than the data it references
 --> src/main.rs:4:5
  |
4 |     context: &'c Context<'s>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: the pointer is valid for the lifetime 'c as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
note: but the referenced data is only valid for the lifetime 's as defined on the struct at 3:0
 --> src/main.rs:3:1
  |
3 | / struct Parser<'c, 's> {
4 | |     context: &'c Context<'s>,
5 | | }
  | |_^
```

Rust doesn’t know of any relationship between `'c` and `'s`. In order to be
valid, the referenced data in `Context` with lifetime `'s` needs to be
constrained to guarantee that it lives longer than the reference to `Context`
that has lifetime `'c`. If `'s` is not longer than `'c`, then the reference to
`Context` might not be valid.

Which gets us to the point of this section: Rust has a feature called *lifetime
subtyping*, which is a way to specify that one lifetime parameter lives at
least as long as another one. In the angle brackets where we declare lifetime
parameters, we can declare a lifetime `'a` as usual, and declare a lifetime
`'b` that lives at least as long as `'a` by declaring `'b` with the syntax `'b:
'a`.


Для того, чтобы сообщить компилятору, что время жизни `'s` будет не меньше времени
жизни `'c`, мы изменим описание структуры следующим образом:

```rust
# struct Context<'a>(&'a str);
#
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

Теперь всё в порядке. Такие особенности применяются не часть, но всё же бывают.

### Границы времени жизни переменных

В главе 10 мы обсуждали как использовать границы в типажах обобщенных типах.
Мы можем также добавить параметры времени жизни как ограничения в обобщенные типы.
Например, рассмотрим тип, который является оболочкой для ссылок. Вспомним тип
`RefCell<T>` из главы 15: он имеет методы `borrow` и `borrow_mut`, которые возвращают
`Ref` и `RefMut`. Это определение структуры `Ref` без переменной времени жизни:

```rust,ignore
struct Ref<'a, T>(&'a T);
```

<span class="caption">код 19-16: определение структуры-оболочки для ссылки на обобщенный
тип без переменной времени жизни</span>

Без связи обобщенного параметра и переменной времени жизни мы получим ошибку, т.к.
компилятор не знает как долго тип `T` будет существовать:

```text
error[E0309]: the parameter type `T` may not live long enough
 --> <anon>:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'a`...
note: ...so that the reference type `&'a T` does not outlive the data it points at
 --> <anon>:1:19
  |
1 | struct Ref<'a, T>(&'a T);
  |                   ^^^^^^
```

Т.к. `T` может быть любым типом, `T` сам может быть ссылкой или типом содержащим
ссылки. Поэтому компилятор не может определить время жизни `T`.

Для решения этой задачу в Rust есть подсказка:

```text
consider adding an explicit lifetime bound `T: 'a` so that the reference type
`&'a T` does not outlive the data it points at.
```

Код 19-17 демонстрирует реализацию данного совета:

```rust
struct Ref<'a, T: 'a>(&'a T);
```

<span class="caption">код 19-17: добавления ограничения времени жизни для `T`</span>

Мы можем решить эту задачу и другим способом. В коде 19-18 продемонстрирована
работа со статическими переменными. Это означает, что если `T` содержит какую-либо
ссылку, она должна иметь `'static` время жизни:

```rust
struct StaticRef<T: 'static>(&'static T);
```

<span class="caption">код 19-18: добавление `'static` время жизни для `T` для
введения ограничения `T`</span>


Types without any references count as `T: 'static`. Because `'static` means the
reference must live as long as the entire program, a type that contains no
references meets the criteria of all references living as long as the entire
program (since there are no references). Think of it this way: if the borrow
checker is concerned about references living long enough, then there’s no real
distinction between a type that has no references and a type that has
references that live forever; both of them are the same for the purpose of
determining whether or not a reference has a shorter lifetime than what it
refers to.

### Переменные времени жизни объектов-типажей

В главе 17 вы изучали объекты-типажи. Они применяются при динамической диспетчеризации.
Но мы ещё не обсуждали случай использования переменных времени жизни в таких
конструкциях. Рассмотрим такой пример. В коде 19-19 у нас есть типаж `Foo` и структура
`Bar`, которая содержит ссылку (и, следовательно, имеет переменную времени жизни):

```rust
trait Foo { }

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> { }

let num = 5;

let obj = Box::new(Bar { x: &num }) as Box<Foo>;
```

<span class="caption">код 19-19: использование типа, который имеет переменную времени
жизни</span>

Этот код компилируется без ошибок. Это происходит потому, что существуют правила
между типажами объектов и переменными времени жизни:

* по умолчанию ПВЖ для типажей-объектов `'static`.
* если мы имеем `&'a X` or `&'a mut X`, то по умолчанию `'a`.
* если мы имеем один `T: 'a`, то по умолчанию  `'a`.
* если мы имеем множество `T: 'a` типов, то время жизни на до указывать явным образом.

В случае явного указания времени жизни типажей-объектов, например `Box<Foo>`,
синтаксис будет следующий `Box<Foo + 'a>` или `Box<Foo + 'static>`.

Далее, мы рассмотрим расширенные возможности связанные с типажами.
