## Расширенные типы

Система типа Rust имеет некоторые функции, которые мы упоминали или использовали
без обсуждение. Мы начали говорить о шаблоне newtype в отношении признаков;
мы начнем с более общей дискуссии о том, почему newtypes полезны как
типы. Затем мы перейдем к типу псевдонимов, который похож на newtypes
но имеет немного другую семантику. Мы также обсудим тип `!` И
динамически размерные типы.


### Использование шаблон для безопасности типов и реализации абстракций

Шаблон newtype, который мы начали обсуждать в конце предыдущей секции, где мы
создаем новый тип как структуру кортежа с одним полем который обертывает тип,
также может быть полезен для статического применения этих значений.

### Синонимы типов

Существует ключевое слово, которое позволяет создавать синониы типов. Например,
чтобы создать синоним `Kilometers` для `i32`:

```rust
type Kilometers = i32;
```

Это значит, что `Kilometers` является синонимом для `i32`. `Millimeters` и `Meters`
являются различными типами данных. Значения, к которым будет применяться тип
`Kilometers` будет считаться типом `i32`:

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

Т.к. `Kilometers` является синонимом `i32` они являются одинаковыми типами, т.е.
они взаимозаменяемы.

Синонимы в основном используются для уменьшения повторяемости. Например, мы у нас
есть тип:

```rust,ignore
Box<Fn() + Send + 'static>
```
Написание сигнатур функций и аннотаций типов может быть весьма утомительным и
нести ошибки. Например, у нас есть код:

```rust
let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<Fn() + Send + 'static>) {
    // ...snip...
}

fn returns_long_type() -> Box<Fn() + Send + 'static> {
    // ...snip...
#     Box::new(|| ())
}
```

<span class="caption">код 19-31: использование длинных описаний типов</span>

Синонимы типов могут сделать этот код более удобный для изучения и работы. В следующем
примере мы покажем как можно применить синонимы. Создадим синоним `Thunk`:

```rust
type Thunk = Box<Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // ...snip...
}

fn returns_long_type() -> Thunk {
    // ...snip...
#     Box::new(|| ())
}
```

<span class="caption">код 19-32: использование синонима `Thunk` для уменьшения
повторений</span>

Ещё один пример использование синонима (в стандартной библиотеке):

```rust
use std::io::Error;
use std::fmt;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

Сделаем замену  `Result<..., Error>`:

```rust,ignore
type Result<T> = Result<T, std::io::Error>;
```

Because this is in the `std::io` module, the fully qualified alias that we can
use is `std::io::Result<T>`; that is, a `Result<T, E>` with the `E` filled in
as `std::io::Error`. The `Write` trait function signatures end up looking like
this:

```rust,ignore
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}
```

### The Never Type, `!`, that Never Returns

Rust имеет специальный тип `!`. Это пустой тип, т.к. он не имеет знанчений. Мы
предпочитаем называть его, как *тип никогда* (*never type*). При использовании этого
типа в описании функции - он обозначает, что данная функция не возвращает какого-либо
значения:

```rust,ignore
fn bar() -> ! {
    // ...snip...
}
```

This is read as “the function `bar` returns never,” and functions that return
never are called *diverging functions*. We can’t create values of the type `!`,
so `bar` can never possibly return. What use is a type you can never create
values for? If you think all the way back to Chapter 2, we had some code that
looked like this, reproduced here in Listing 19-33:

```rust
# let guess = "3";
# loop {
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
# break;
# }
```

<span class="caption">код 19-33: пример использования `continue` в `match`</span>

Рассмотрим другой пример (из главы 6). Данный код в отличии от предыдущего
не скомпилируется.

```rust,ignore
let guess = match guess.trim().parse()  {
    Ok(_) => 5,
    Err(_) => "hello",
}
```

Каким будет тип переменной `guess`? В Rust нельзя чтобы тип одновременно имеет
возможность содержать данные разных типов. Так почему же в предыдущем пример
всё работало. Это потому что `continue` возвращает тип `!`.

Еще один пример использования `!`-типа - макрос `panic!`. Пимер использования
`panic!`:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Т.к. `panic!` не возвращает типа - код компилируется.

Ещё одно выражение имеющее тип `!` является `loop`:

```rust,ignore
print!("forever ");

loop {
    print!("and ever ");
}
```

### Типы динамического размера & `Sized`

Т.к. Rust нужно знать размер памяти заранее, концепция динамической памяти может
возникнуть ощущение противоречивость. Далее раскроем эту концепцию.

Для этого рассмотрим тип, который мы использовали на протяжении всей книги - `str`.
`str` является примером использования динамической памяти. Обратите внимание, что
вы не можете создать переменную с данным типом данных:

```rust,ignore
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

Тут мы видим две переменные имеют разную длину. Поэтому нельзя создать переменную
динамической размерности.

Что же нам делать? Ответ мы уже знаем. Нужно создавать переменные ссылочного типа
`&str`.

Обратите внимание, почему такой тип данные является действительным - ссылочный тип
всегда имеет фиксированный тип данных. Он состоит из двух частей - адреса и размера.


<!-- Note for Carol: `Rc<str>` is only in an accepted RFC right now, check on
its progress and pull this out if it's not going to be stable by Oct -->

While we’ve talked a lot about `&str`, we can combine `str` with all kinds of
pointers: `Box<str>`, for example, or `Rc<str>`. In fact, you’ve already seen
this before, but with a different dynamically sized type: traits. Every trait
is a dynamically sized type we can refer to by using the name of the trait. In
Chapter 17, we mentioned that in order to use traits as trait objects, we have
to put them behind a pointer like `&Trait` or `Box<Trait>` (`Rc<Trait>` would
work too). Traits being dynamically sized is the reason we have to do that!

#### The `Sized` Trait

<!-- If we end up keeping the section on object safety in ch 17, we should add
a back reference here. /Carol -->

Существует типаж, функционал которого сообщает компилятору имеет ли тип определённый
размер или нет. Это типа `Sized`. Этот типаж добавляется неявным образом каждый
раз при декларировании обобщенных типов:

```rust,ignore
fn generic<T>(t: T) {
    // ...snip...
}
```

вот так выглядит код, если указывать супертипы явным образом:

```rust,ignore
fn generic<T: Sized>(t: T) {
    // ...snip...
}
```

Также существую синтаксис который может сделать требования к типам более либеральные::

```rust,ignore
fn generic<T: ?Sized>(t: &T) {
    // ...snip...
}
```

Т.е. может размер может быть известным, но может и не быть. Обратите внимание, что
такой синтаксис работает только совместно с типажом `Sized`.

Также обратите внимание, что мы переключили тип параметра `t` с` T` на `& T`: поскольку
тип не может быть `Sized`, мы должны использовать его за каким-то указателем. В
в этом случае мы выбрали ссылку.

В следующей секции поговорим о функциях и закрытиях!
