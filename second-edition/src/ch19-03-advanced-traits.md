## Расширенные опции типажей

Мы уже познакомились с функционалом типажей в главе 10. Также как и переменные
времени жизни при первом знакомстве мы не раскрывали всех возможностей компонента
языка, остановившись лишь на основных. Теперь, когда вы стали уверенными пользователями
языка Rust, пора углубить ваши знания.

### Ассоциированные типы

Ассоциированные типы (*Associated types*) - это способ связи ассоциированного
типа-конетейнера с типажом таким образом, чтобы методы типажа могли бы использовать
типы ассоциированных типов в своём описании. Реализация типажа будет использовать
конкретные типы, которые будут использованы в соответствующей реализации.

Мы описали большинство вещей в этой главе как очень редкие.
Связанные типы находятся где-то посередине; они используются реже, чем элементы, которые
описали ранее (в предыдущих главах), но более распространенны, чем многие из вещей
этой главы.

Примером типажа, который ассоциируется с типом является `Iterator`, который входит
в стандартную библиотеку. Он имеет ассоциированный тип `Item`, который содержит
тип элементов, которые могут быть использованы. В главе 13 мы уже рассматривали
использование итератора 19-20:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

<span class="caption">код 19-20: определение типажа `Iterator`, который имеет
ассоциированный тип `Item`</span>

Типаж `Iterator` имеет ассоциированный тип `Item`. `Item` является контейнером типа.
Метод `next` возвращает значение `Option<Self::Item>`. Реализации этого типажа
должны определить конкретный тип для `Item` и будет возвращать значения этого типа.

#### Ассоциированные типы, как вид обобщенных типов

Когда в коде 13-6 мы реализовали типаж `Iterator` для структуры `Counter`, мы
установили тип `Item` равным `u32`:

```rust,ignore
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
```

Всё это весьма напоминает обобщенные типы. Так почему же типаж `Iterator` не определён,
как в коде 19-21?

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

<span class="caption">код 19-21: гипотетическое определение типажа `Iterator`
используя обобщенные типы</span>

Отличием реализации 19-21 является то, что для каждого типа мы должны будет написать
реализацию. Например, для `String` `Iterator<String> for Counter`. Т.е. если типаж
имеет обобщенный параметр мы можем реализовать типаж для типа множество раз, при
этом каждый раз меняя обобщенный параметр на конкретный. Когда мы используем метод
 `next` мы должны предоставить аннотации для указания какой `Iterator` должен быть
 использован.

При работе с ассоциированными типам и мы не должны реализовывать типаж множество
раз. Используя определение `Iterator` из кода 19-20 мы только лишь выбираем один
раз каким будет тип `Item`. Т.е. необходима только лишь `impl Iterator for Counter`.

Есть ещё одно преимущество использования ассоциированных типов. Рассмотрим два
типажа в примере 19-22. Оба типажа используют узлы и углы. `GGraph` испльзует обобщенные
параметры, `AGraph` ассоциированные типы

```rust
trait GGraph<Node, Edge> {
    // methods would go here
}

trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}
```

<span class="caption">код 19-22: для варианта определение типажа</span>

Предположим, что мы хотим реализовать функцию, которая рассчитывать дистанцию
между узлами любого типа. В типаже  `GGraph` такая реализация будет иметь вид 19-23:

```rust
# trait GGraph<Node, Edge> {}
#
fn distance<N, E, G: GGraph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    // ...snip...
#     0
}
```

<span class="caption">код 19-23: представление описания функции `distance`, которая
использует типаж `GGraph` и которая должна указать все обобщенные параметры</span>

Наша функция должна определить типы параметров `N`, `E` и `G`, где `G` ограничена
типажом `GGraph`, который имеет типы  `N` (`Node`), а `E` (`Edge`). Даже если
функция  `distance` не будет использовать данные типов углов, мы должны описать
`E`, т.к. мы используем типаж `GGraph` мы должны указать тип для `Edge`.

При использовании ассоциированных типов описание метода `distance` будет выглядеть
следующим образом:

```rust
# trait AGraph {
#     type Node;
#     type Edge;
# }
#
fn distance<G: AGraph>(graph: &G, start: &G::Node, end: &G::Node) -> u32 {
    // ...snip...
#     0
}
```

<span class="caption">код 19-24: описание функции `distance`, которая использует
типаж `AGraph` и ассоциированный тип `Node`</span>

Такой вид намного нагляднее, т.к. нам нужно только один обобщенный параметр типа
`G`. Для использования типа `Node` ассоциированного с `AGraph` мы можем указать
`G::Node`.

#### Использование типажных объектов и ассоциированных типов

Вы, возможно, удивились почему мы не использовали типажи-объекты в функциях `distance`
в примерах 19-23 и Listing 19-24. Описание для функции `distance` при работе с
 `GGraph` было бы более компактным при использование типажных объектов:

```rust
# trait GGraph<Node, Edge> {}
#
fn distance<N, E>(graph: &GGraph<N, E>, start: &N, end: &N) -> u32 {
    // ...snip...
#     0
}
```

This might be a more fair comparison to Listing 19-24. Specifying the `Edge`
type is still required, though, which means Listing 19-24 is still preferable
since we don’t have to specify something we don’t use.

It’s not possible to change Listing 19-24 to use a trait object for the graph,
since then there would be no way to refer to the `AGraph` trait’s associated
type.

It is possible in general to use trait objects of traits that have associated
types, though; Listing 19-25 shows a function named `traverse` that doesn’t
need to use the trait’s associated types in other arguments. We do, however,
have to specify the concrete types for the associated types in this case. Here,
we’ve chosen to accept types that implement the `AGraph` trait with the
concrete type of `usize` as their `Node` type and a tuple of two `usize` values
for their `Edge` type:

```rust
# trait AGraph {
#     type Node;
#     type Edge;
# }
#
fn traverse(graph: &AGraph<Node=usize, Edge=(usize, usize)>) {
    // ...snip...
}
```

Хотя при использовании типажных объектов вам не нужно знать конкретный тип
параметра `graph` во время компиляции, необходимо ограничить использование типажа
`AGraph` с помощью конкретных ассоциированных типов. Без их указания компилятор
не сможет понять, какую реализацию использовать.

### Перезагрузка операторов и типы параметров по умолчанию

Синтаксис `<PlaceholderType=ConcreteType>` используется для указания типа по умолчанию
в обобщенном типе.

Rust не позволяет создавать собственные операторы или перезагружать произвольные
операторы. В тоже время возможно перезагружать операторы определенные в модуле
`std::ops`. Код 19-25 показывает, как перезагрузить оператор `+` с помощью
реализации типажа `Add` структурой `Point`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

<span class="caption">код 19-25: реализация типажа `Add` для перезагрузки оператора
`+` для структуры `Point`</span>

Мы реализовали метод `add`. Типаж `Add` имеет ассоциированный тип с именем `Output`,
который используется для определения типа данных в методе `add`.

Рассмотрим типаж `Add` более детально. Это его определение:

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

Эта конструкция похожа на типаж с одним методом и ассоциированным типом. Что-то
новенькое - это `RHS=Self` в угловых скобках. Этот синтаксис называется *параметрами
по умолчанию* (*default type parameters*). `RHS` является обобщенным типом параметра
(сокращение от “right hand side”). Если вы не определите конкретный тип для `RHS`
типом по умолчанию будет `Self`.

Рассмотрим другой пример реализации типажа `Add`. Представим, что у нас есть
структура содержащая значения в различных единицах изменения. Мы можем реализовать
`Add` для `Millimeters` различными способами 19-26:

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

<span class="caption">код 19-26: реализация типажа `Add` для `Millimeters`
для предоставления возможности добавлять `Millimeters` к `Millimeters` и
`Millimeters` к `Meters`</span>

Если мы прибавим `Millimeters` к другому `Millimeters` нам не нужно использовать
`RHS` с помощью определенного типа, т.к. по умолчанию используется тип `Self`.
Если мы хотим складывать `Millimeters` и `Meters`, в этом случае нам наобходимо
указать `impl Add<Meters>`.

Параметры по умолчанию используются в двух основных случаях:


1. Чтобы расширить тип без внесения изменений в существующий код.
2. Чтобы позволить сделать улучшения, которые не хотят большинство пользователей.

Пример второй цели: часто вы добавляете два типа вместе. Используя параметры по
умолчанию проще реализовать типаж без описания дополнительных параметров. Т.е.
мы переносим часто используемые определение в описание типажа.


### Использование полного имени для устранения неоднозначности

Компилятор не может предотвратить создание метода с тем же именем, что и в другом
типаже. Также он не может препятствовать реализовать эти два типажа в одном типе.
Мы также можем реализовать этот метод непосредственно в типе. Для того чтобы указать
какой же из этих методов мы хотим использовать необходимо проделать это правильно
указать. В примере 19-27, где типажи `Foo` и `Bar` оба имеют метод `f` и мы реализуем
оба типажа в структуре `Baz`, которая также имеет метод `f`:

<span class="filename">Filename: src/main.rs</span>

```rust
trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz’s impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz’s impl of Bar"); }
}

impl Baz {
    fn f(&self) { println!("Baz's impl"); }
}

fn main() {
    let b = Baz;
    b.f();
}
```

<span class="caption">код 19-27: реализация двух типажей, которые имеют метод с
одинаковым именем и которое совпадает с именем определенным в структуре</span>

Для реализации метода `f` для `Foo` в `Baz` мы печатаем `Baz's impl of Foo`.
Для реализации метода `f` для `Foo` в `Bar` мы печатаем `Baz's impl of Bar`.
При реализации метода `f` в самом `Foo` будет напечатано `Baz's impl`. При вызове
метода `b.f()` будет напечатано `Baz's impl`.

Для того, чтобы вызвать метод типажа `Foo` из экземпляра `Foo` необходимо использовать
полное имя метода (*fully qualified syntax*):

```rust,ignore
receiver.method(args);
```

Описание полного имени метода выглядит следующим образом:

```rust,ignore
<Type as Trait>::method(receiver, args);
```

Поэтому для устранения неоднозначности и получения возможности вызова всех методов
`f`, определенных в листинге 19-27 мы указываем, что мы хотим рассматривать тип
`Baz`, как каждый признак в угловых скобках, затем используйте два двоеточия, затем
вызовите метод `f` и использовать экземпляр `Baz` в качестве первого аргумента.
В листинге 19-28 показано, как вызвать `f` из` Foo`, а затем `f` из` Bar` на `b`:

<span class="filename">Filename: src/main.rs</span>

```rust
# trait Foo {
#     fn f(&self);
# }
# trait Bar {
#     fn f(&self);
# }
# struct Baz;
# impl Foo for Baz {
#     fn f(&self) { println!("Baz’s impl of Foo"); }
# }
# impl Bar for Baz {
#     fn f(&self) { println!("Baz’s impl of Bar"); }
# }
# impl Baz {
#     fn f(&self) { println!("Baz's impl"); }
# }
#
fn main() {
    let b = Baz;
    b.f();
    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);
}
```

<span class="caption">код 19-28: использование синтаксиса полного пути к методу
`f` в типажах `Foo` и `Bar`</span>

Будет напечатано:

```text
Baz's impl
Baz’s impl of Foo
Baz’s impl of Bar
```

Для выбора нужного типажа вам необходимо указать нужный типаж в `<>`. Если же
необходимо вызвать метод типажа `Foo` непосредственно из `Baz`  можно написать так:
`Foo::f(&b)`.

Таким образом можно вызвать и метод структуры `Baz::f(&b)`.

### Супертипажи. Реализация наследования

Бывает, что необходимо использовать функционал одного типажа в другом. Родительский
типаж называют супертипажом (*supertrait*).

Например, мы хотим реализовать типаж `OutlinePrint` с методом `outline_print`,
который печатает значения внутри звёздочек. Т.е. если структура `Point` реализует
`Display` и результатом будет текст `(x, y)`, то вызов `outline_print` текст вывода
будет выглядеть:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

В реализации `outline_print` мы хотим иметь возможность использовать `Display`.
Для этого необходимо описать типаж `OutlinePrint`, чтобы дать компилятору понять,
что он реализовал типаж `Display`. Мы можем сделать это в описании типажа:

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

<span class="caption">код 19-29: реализация типажа `OutlinePrint`, которая наследует
функциона `Display`</span>

Т.к. мы определили зависимость типажа `OutlinePrint` от `Display`, мы можим
использовать метод `to_string` в `outline_print`.

Если мы попытаемся реализовать `OutlinePrint`, в типе который не реализовал `Display`,
мы получим ошибку:

```rust
# trait OutlinePrint {}
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

Описание ошибки:

```text
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ the trait `std::fmt::Display` is not implemented for
   `Point`
   |
   = note: `Point` cannot be formatted with the default formatter; try using
   `:?` instead if you are using a format string
   = note: required by `OutlinePrint`
```

Реализация типажа `Display` в `Point` выглядит следующим образом:

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

реализация типажа `OutlinePrint` в `Point` скомпилируется без ошибок. Мы можем
вызвать `outline_print` из экземпляра `Point` и увидеть результат.

### Шаблон Newtype для реализация внешних типажей во внешних типах

В главе 10 мы упоминали о правиле, по которому следует, что дозволено реализация
типажей в типе только если они находятся в одном контейнере. Способом обойти это
ограничение является *newtype pattern*, который предназначен для создания нового
типа используя структур кортежа с одним полем.

In Chapter 10, we mentioned the orphan rule, which says we’re allowed to
implement a trait on a type as long as either the trait or the type are local
to our crate. One way to get around this restriction is to use the *newtype
pattern*, which involves creating a new type using a tuple struct with one
field as a thin wrapper around the type we want to implement a trait for. Then
the wrapper type is local to our crate, and we can implement the trait on the
wrapper. “Newtype” is a term originating from the Haskell programming language.
There’s no runtime performance penalty for using this pattern. The wrapper type
is elided at compile time.

Например, если мы хотим реализовать `Display` в `Vec`, мы можем создать структуру
`Wrapper`, которая содержит экземпляр `Vec`. Далее, мы реализуем `Display` для
`Wraper` и используем значение `Vec`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

<span class="caption">код 19-30: создание типа `Wrapper` вокруг `Vec<String>`
для реализации `Display`</span>

Реализация `Display` использует `self.0` для доступа к внутреннему `Vec` и далее
мы можем использовать функционал `Display` в `Wrapper`.

Недостатком является то, что, поскольку «Wrapper» является новым типом, он не имеет
методов данных, которую он держит; мы должны были бы реализовать все методы `Vec`,
как `push`,` pop` и все остальное непосредственно на `Wrapper`, чтобы делегировать
`self.0` для того, чтобы иметь возможность рассматривать «Wrapper» точно так же,
как «Vec». Если бы мы хотим, чтобы новый тип имел все методы, который имеет
внутренний тип, реализуя `Deref`. Если мы не хотим, чтобы тип обертки имел все
методы внутреннего типа, чтобы ограничить поведение типа обертки, нам нужно будет
реализовать только те методы, которые мы хотим сами.

Вот как используется шаблон newtype по отношению к типажам; это также
полезный шаблон без вовлечения типажей. В следующей секции мы переключим внимание
на разговор о некоторых продвинутых способах взаимодействия с системой типов в Rust.
