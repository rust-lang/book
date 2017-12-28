## Объекты-типажи  Trait Objects for Using Values of Different Types

В Главе 8 мы говорили о том что, ограничением векторов - хранение данных только
одного типа. В примере 8-1 мы демонстрировали, как перечисления могут хранить
разные типы данных внутри каждого значения. Вектор может хранить данные такого
перечисления. Такая конструкция удобна в том случае, когда необходима структура
состоящая из фиксированного набора данных.

<!-- The code example I want to reference did not have a listing number; it's
the one with SpreadsheetCell. I will go back and add Listing 8-1 next time I
get Chapter 8 for editing. /Carol -->

В некоторых случаях возникает необходимость иметь набор типов, которые можно было
бы расширять пользователям библиотеки. Например, множество графических библиотек
имеют интерфейсы, благодаря которым создаваемые на их основе новые компоненты
могут становиться частью системы, расширяя её возможности. Например, интерфейс
может иметь метод `draw` и каждый новый элемент, которые его реализует может
быть использован для рисования. Мы рассмотрим как эту концепцию можно реализовать
в Rust.

При создании графической библиотеки (назовём её `rust_gui`), мы не можем заранее
знать всех типов которые пользователи библиотеки хотят создать. Мы не можем создать
`enum`, которая содержала бы все возможные типы. Особенностью графической библиотеки
в том, что предоставив интерфейсы нельзя будет заранее знать, какой код будет выполнен
при вызове методов (например, при вызове метода `draw`).

В языках программирования, в которых реализовано наследование эту задачу можно
было бы решить созданием абстрактного класса `Component`, который бы имел
виртуальный метод `draw`. Все реализовавшие этот класс имели бы свою реализацию
метода. Использующий код данных методов относился бы ко всем реализациям класса
`Component`, как к этому классу (используя только функциональные возможности
предоставляемые этим классом).

### Определение типажа с виртуальным (общим) методом (поведением)

В Rust мы можем определить типаж, который мы назовем «Draw». Он будет иметь один
виртуальный метод (метод требующий реализации) `draw`. Далее, мы можем создать
вектор, который содержит список указателей (`&`-ссылок или умных указателей `Box<T>`).
 Мы поговорим о причинах, по которым объекты-типажи должны быть внутри указателями
 в главе 19.

Мы упоминали, что мы не можем называть структуры и перечисления "объектами", т.к.
они имеют особую природу (не похожую на объекты в других языках программирования).
В структурах и перечислениях объявление полей и поведений разделены. Объекты-типажи
больше напоминают объекты (ООП-языков), в том смысле, что они объединяют данные,
составленные из указателя на конкретный объект, с поведением методов, определенных в признаке.
В тоже время типажи не могут хранить данные. Их цель предоставлять поведение.

Типаж определяет поведение, которое необходимо в определённой ситуации. Далее,
вы можем использовать типаж-объект, где мы хотели бы использовать конкретный тип
или обобщенный тип. Система типов Rust предполагает, что реализации типажей имеют
соответствующий функционал. В примере 17-3 показано, как определить типаж `Draw`
содержащий метод `draw`:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub trait Draw {
    fn draw(&self);
}
```

<span class="caption">код 17-3: определение типажа `Draw` trait</span>

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

Думаю, что тут всё понятно. Далее, добавим кое-что новое. В коде 17-4 в структуре
`Screen`, которая содержит поле `components`, которое является вектором, содержащее
данные типа `Box<Draw>`. `Box<Draw>` является объектом-типажом, т.к. в обёртке
`Box<T>` может содержаться любое значение реализовавшее типаж `Draw`.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}
```

<span class="caption">код 17-4: определение структуры `Screen` с полем
`components`, который является вектором типажа-объектов, которые реализуют типаж
`Draw`</span>

В структуре `Screen`, мы определим метод `run`, который будет вызывать метод `draw`
каждого элемента вектора `components` (17-5):

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
# pub struct Screen {
#     pub components: Vec<Box<Draw>>,
# }
#
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

<span class="caption">код 17-5: реализация метода `run` в `Screen`, который вызывает
метод `draw` каждого элемента вектора</span>

Это решение отличается от использования обобщенного параметра и типажа. Обобщенный
параметр может быть использовать только для какого-то конкретного типа, в то время
как типаж объекта может быть использован для любого конкретного типа. Рассмотрим
пример совметного использования обобщенного типа и объекта-типажа (17-6):

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

<span class="caption">код 17-6: альтернативная реализация структуры `Screen` и
метода `run`, используя обобщенный объект-типаж </span>

Это решение вводит ограничение для экземпляров структуры `Screen` (вектор может
хранить только однотипные экземпляры, реализовавшие  типаж `Draw` - например, только
`Button` или только `TextField`). Если у вас будут только однородные коллекции,
использование дженериков и объектов-типажей предпочтительнее, поскольку экземпляры
будут мономорфны.

В отличие от предыдущего примера, один экземпляр `Screen` может хранить вектор,
содержащий или `Box<Button>` или `Box<TextField>`. Далее мы рассмотрим как это работает
подробнее, а также поговорим о вопросах производительности во время выполнения программы.

### Реализации типажа

Теперь добавим реализации типажа `Draw` в наш код. Наши реализации не будут содержать
кода внутри методов (для простоты) 17-7:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub trait Draw {
#     fn draw(&self);
# }
#
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Code to actually draw a button
    }
}
```

<span class="caption">код 17-7: структура `Button` реализует типаж `Draw`</span>

Поля `width`, `height` и `label` структуры  `Button` будут отличаться от, например,
полей другой структуры `TextField`, которая может иметь поля `width`, `height`,
`label` и `placeholder`. Каждая реализация также имеет свою версию метода `draw`.
Дополнительно структура `Button` может иметь ещё одни блок `impl`, содержащие
дополнительные методы. Этим методы может не иметь любая другая реализация `Draw`.
Во внешней библиотеке также можно реализовать типаж `Draw` (17-8):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rust_gui;
use rust_gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
    }
}
```

<span class="caption">код 17-8: использование внешнего контейнера `rust_gui` и
реализация типажа `Draw` структурой `SelectBox`</span>

Пользователь нашей библиотеки может реализовать функцию `main` и в ней создать
экземпляр `Screen` и добавить экземпляры структур `SelectBox` и `Button` в вектор
с помощью умного указателя `Box<T>`. В коде можно вызывать метод `run` структуры
`Screen`, который в свою очередь вызовет метод `draw` в каждом компоненте вектора.
Код 17-9 демонстрирует реализацию:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use rust_gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

<span class="caption">код 17-9: использование объектов-типаже для хранения значений
различных типов, которые реализовали типаж</span>

Несмотря на то, что мы не знали, что кто-то однажды добавит тип `SelectBox`,
наша реализация `Screen` смогла работать с `SelectBox` и нарисовать её,
т.к. `SelectBox` реализует тип` Draw`, что означает, что он реализует
метод `draw`.

Это демонстрация неявной типизации (т.н. *duck typing*). В реализации метода `run`
структуры `Screen` (17-5) не нужно знать тип конкретного элемента. Необходимо только
знать, что этот элемент имеет метод `draw`.

Плюсом реализации подобного решения является отсутствие необходимости проверять
начие реализации метода во время работы программы. Код не будет скомпилирован, если
какой-либо элемент вектора не реализует типаж.

Например, код (17-10) демонстрирует, что случится если мын попытаемся добавить
`String` в качестве компонента вектора:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate rust_gui;
use rust_gui::Draw;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}
```

<span class="caption">код 17-10: попытка использования типа, который не реализовал
типаж объекта-типажа</span>

Мы получили ошибку, т.к. `String` не реализовал типаж `Draw`:

```text
error[E0277]: the trait bound `std::string::String: Draw` is not satisfied
  -->
   |
 4 |             Box::new(String::from("Hi")),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not
   implemented for `std::string::String`
   |
   = note: required for the cast to the object type `Draw`
```

### Типажи-объектов выполняют динамическую диспетчеризацию (связывание)

Напомним что в главе 10, когда мы обсуждали процесс мономорфизации, что
компилятор выполняет, когда мы используем типажи для ограничения в дженероках:
компилятор создаёт реализации для конкретных типов, которые использует в месте
использования такого обобщенного параметра. Т.е. компилятор выполняет
*статическую связывание*. Такой код работает очень быстро.

Когда же мы используем типажи-объекты, компилятор не может выполнить мономорфизацию,
т.к. мы не знаемвсех типов, которые могут быть использованы в коде, Rust отслеживает
код, который может использоваться при вызове метода и выполнят это работу во время
выполнения программы. Это приводик к замедлению работы программы.

Динамическая диспечеризация также предостерегает компилятор выбирать встроенный
код метода, который предотвращает некоторые оптимизации. Благодаря чему мы получили
дополнительную гибкость в коде.

### Безопасность типажей-объектов

<!-- Liz: we're conflicted on including this section. Not being able to use a
trait as a trait object because of object safety is something that
beginner/intermediate Rust developers run into sometimes, but explaining it
fully is long and complicated. Should we just cut this whole section? Leave it
(and finish the explanation of how to fix the error at the end)? Shorten it to
a quick caveat, that just says something like "Some traits can't be trait
objects. Clone is an example of one. You'll get errors that will let you know
if a trait can't be a trait object, look up object safety if you're interested
in the details"? Thanks! /Carol -->

Не все типажи могут быть типажами-объектами. Только безопасные типажи-объекты могут
ими быть. Чтобы быть безопасным типаж должен соответствовать следующим условиям:


* The trait does not require `Self` to be `Sized`
* все методы типажа являются безопасными.

`Self` - это ключевое слово, которое является псевдонимом типа, который реализовал
типаж или методы. `Sized` является типажём-маркером, таким как `Send` и `Sync`.
 `Sized` автоматически реализуется в типах, которые имеют известный тип во время
 компиляции (такие как `i32` и ссылки). Типы, которые не имеют известный размер,
 включая срезы (`[T]`) и объекты-типажи.
`Sized` является неявным типов всех обобщенных параметров по умолчанию. Большинство
полезных операций в Rust требуют от использованного типа реализовать `Sized`.
Если нам необходимо использовать в срезах типажи, то мы должны явным образом указать
это с помощью `T: ?Sized`.

Типажи имеют опцию по умолчанию `Self: ?Sized`. Это значит, что они могут быть
реализованы в типах, которым могут быть, а могут не быть `Sized`.  Например, если
мы создадим типаж `Foo`, который реализовывает `Self: ?Sized`, то он выглядит так:

```rust
trait Foo: Sized {
    fn some_method(&self);
}
```

Теперь типаж `Sized` является родительским типажом (*supertrait*) `Foo`. Это значит,
что `Foo` требует от своих реализаций реализацию методов описанных в декларации
`Foo` (т.е. `Self`) были также `Sized`. Мы поговорим о *supertrait* в Главе 19.

`Foo` требует чтобы `Self` реализовал `Sized`. В тоже время `Self` нельзя использовать
в типажах объектов.

`Foo` requires `Self` to be `Sized`, and therefore is not allowed to be used in
a trait object like `Box<Foo>`. This is because it would be impossible to implement
the trait `Foo` for a trait object like `Box<Foo>`: trait objects aren’t sized,
but `Foo` requires `Self` to be `Sized`. A type can’t be both sized and unsized
at the same time!

For the second object safety requirement that says all of a trait’s methods
must be object safe, a method is object safe if either:

* It requires `Self` to be `Sized` or
* It meets all three of the following:
    * It must not have any generic type parameters
    * Its first argument must be of type `Self` or a type that dereferences to
      the Self type (that is, it must be a method rather than an associated
      function and have `self`, `&self`, or `&mut self` as the first argument)
    * It must not use `Self` anywhere else in the signature except for the
      first argument

Those rules are a bit formal, but think of it this way: if your method requires
the concrete `Self` type somewhere in its signature, but an object forgets the
exact type that it is, there’s no way that the method can use the original
concrete type that it’s forgotten. Same with generic type parameters that are
filled in with concrete type parameters when the trait is used: the concrete
types become part of the type that implements the trait. When the type is
erased by the use of a trait object, there’s no way to know what types to fill
in the generic type parameters with.

Пример типажа, у которого методы не безопасны - это типаж стандартной библиотеки
`Clone`. Реализация метода `clone` в `Clone` выглядит следующим образом:

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```
Типаж `String` реализует `Clone` и вызывает метод  `clone` вы получаете ссылку на
экземпляр `String`.
`String` implements the `Clone` trait, and when we call the `clone` method on
an instance of `String` we get back an instance of `String`. Similarly, if we
call `clone` on an instance of `Vec`, we get back an instance of `Vec`. The
signature of `clone` needs to know what type will stand in for `Self`, since
that’s the return type.

If we try to implement `Clone` on a trait like the `Draw` trait from Listing
17-3, we wouldn’t know whether `Self` would end up being a `Button`, a
`SelectBox`, or some other type that will implement the `Draw` trait in the
future.

Компилятор сообщит вам, если вы попытаетесь нарушить правила. Например, такой код не
скомпилируется:

```rust,ignore
pub struct Screen {
    pub components: Vec<Box<Clone>>,
}
```

Мы получим ошибку:

```text
error[E0038]: the trait `std::clone::Clone` cannot be made into an object
 -->
  |
2 |     pub components: Vec<Box<Clone>>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone` cannot be
  made into an object
  |
  = note: the trait cannot require that `Self : Sized`
```

<!-- If we are including this section, we would explain how to fix this
problem. It involves adding another trait and implementing Clone manually for
that trait. Because this section is getting long, I stopped because it feels
like we're off in the weeds with an esoteric detail that not everyone will need
to know about. /Carol -->
