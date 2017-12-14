## Обработка группы элементов с помощью итераторов

<!-- From reading on, it seems like an iterator is useless without the methods
we use it with --- I think this is an important point to make early, I did find
it difficult to know what an iterator actually was throughout, and how it
depends on these other methods. Can you add something to this effect? -->
<!-- Reiterating the need for a clear definition of an iterator here--it seems
like an item that's used in iteration rather than something that performs the
process of iteration itself, is that right? Like a counter passed from element
to element? Can we define this at the begin of the iterator section? -->
<!-- I've added an explanation along these lines, does this help? /Carol -->

Шаблонное решение "итератор" позволяет вам работать с сгруппированными элементами
по отдельности. В задачу итератора входит последовательное получение доступа к элементам
и определения конца последовательности. При использовании итераторов вы не должны
нарушать (пересоздавать) логику работы этого шаблонного решения.

В Rust итераторы имею особенность - инициализация по запросу (т.е. *lazy*). Например,
в коде 13-13 создаётся итератор значений вектора `v1`:

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

<span class="caption">Код 13-13: Создание итератора</span>

После создания итератора, мы можем выбрать различные варианты его использования.
В примере 3-6 мы уже использовали итераторы для цикла `for`. Пример 13-14 подробно
явно показывает порядок работы с итераторов в цикле `for`. Итератор сохраняется в
переменной `v1_iter`, а далее цикл последовательно получает значения, которые печатаются
на консоль.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

<span class="caption">Код 13-14: использование итератора в цикле `for`</span>

В языках программирования, в которых не предоставляется подобное программное решение,
пишутся подобные решения. Производится перебор значений начиная с индекса 0, увеличивая
индекс на единицу и отслеживание текущего индекса и размера группы данных.
Всё это итераторы отслеживает без нашего участия. Итераторы позволяют использовать
логику своей работы при работе с любыми типами данных. Далее рассмотрим как это работает
на примерах.

### Типаж `Iterator` и метод `next`

Все итераторы реализуют типаж стандартной библиотеки `Iterator`. Так выглядит
его исходный код:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

Обратите внимание на элементы синтаксиса, которые мы ещё не рассматривали.
`type Item` и `Self::Item`, которые определяют *ассоциированный тип* (*associated type*)
с этим типажом. Мы подробнее поговорим о них в Главе 19. Сейчас вам нужно знать,
что этот код требует от реализаций этого типажа определить тип `Item`. Этот тип
используется в методе `next`. Другими словами, тип `Item` будет являться типом
элемента, который возвращает итератор.

Метод `next` необходимо реализовать. Возвращаемое значение находится внутри `Some`.
Когда перебор элементов завершен, возвращается  `None`. Мы можем вызвать метод
`next` непосредственно. Пример 13-15:

<span class="filename">Filename: src/lib.rs</span>

```rust,test_harness
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

<span class="caption">Код 13-15: Вызов метода итератора `next`</span>

Обратите внимание на необходимость сделать переменную-итератор изменяемой (mut).
Вызовом метода `next` изменяет состояние итератора. Каждый вызов метода выдаёт следующее
значения последовательности.

### Методы типажа `Iterator`

<!-- Can you explain what it is you mean by "consumes" an iterator here? It
doesn't look like we do in this section, I think that's important to lay that
out clearly -->
<!-- This next paragraph doesn't give much away to me I'm afraid, not being
clear what we mean by *consume* at this point. Is a consuming adaptor like a
catalyst? -->
<!-- I hope this section addresses these comments you had /Carol -->

Типаж `Iterator` имеет несколько различных методов с реализацией по умолчанию.

<!-- Is there somewhere they can learn about all the methods and what they do,
how to use them? This seems like a good sample example, and if we can broaden
it out that would be really helpful -->
<!-- I've moved this comment here since you made this comment on the last
version of this chapter right after a spot where we mentioned looking at the
standard library API documentation for the iterator trait, like we're now doing
in the above paragraph. That's where the reader should go to learn about
all the methods and what they do and how to use them. Can you elaborate on why
that wasn't clear in the previous version of the chapter? Is there a reason why
the standard library API documentation didn't sound like that place to go?
/Carol -->

Методы, которые вызывают метод `next` называют пользователя итератора (*consuming adaptors*).
Пример такого потребителя является метод `sum`. Этот метод получат владение итератором
и перебирает элементы с помощью метода `next`:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

<span class="caption">Код 13-16: Вызов метода `sum` для получения суммы всех
элементов вектора</span>

Переменную `v1_iter` после вызова метода `sum` уже использовать нельзя.

### Методы типажа `Iterator` для создания других итераторов

Другим типом методов в типаже `Iterator` являются методы создающие другие итераторы.
Эти методы называют адаптерами (*iterator adaptors*) и позволяют нам изменять
итераторы в различные типы итераторов. Мы можем использовать цепочки вызовов
таких адаптеров. Т.к. итераторы инициируются по запросу.
Пример использования метода `map`, который получает в качестве параметра замыкание.
Это замыкание вызывается для каждого элемента. Результатом работы функции будет
новый итератор и измененный вектор.

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

<span class="caption">Код 13-17: Вызов итератора-адаптера `map` для создания
нового итератора</span>

Предупреждение:

```text
warning: unused result which must be used: iterator adaptors are lazy and do
nothing unless consumed
 --> src/main.rs:4:1
  |
4 | v1.iter().map(|x| x + 1);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
```

Код программы 13-17 ничего не делает, пока не будет вызвано другим элементом цепочки
вызовов. Об этом сообщается компилятором при вызове этого кода: т.к. адаптеры итераторов
работают только при внешнем использовании.

Для того чтобы исправить код и последовать рекомендациям компилятора, будем использовать
метод `collect` (который мы кратко представили в Главе 12). Этот метод использует
итератор для группировки результатов работы предыдущего метода в вектор. В примере
кода 13-18 мы группируем результаты работы метода `map` в вектор, который содержит
все элементы первоначального вектора при этом значение каждого числового элементам
 увеличено на 1:

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

<span class="caption">Код 13-18: вызов метода `map` для создания нового итератора,
далее вызов метода `collect` для создания и использования нового итератора, чтобы
создать новый вектор с данными</span>

Т.к. `map` получает замыкание мы можем применить любую операцию над содержимым.
Это прекрасный пример того, как использование замыканий позволяет улучшить поведение
итераторов (упростить обработку данных).


<!-- I'm not clear from this last sentence which part is iterating through each
element, iter or map? What is map actually doing?-->
<!--Ah, I'm afraid I completely failed to follow this. What is the second
iterator for? I'm still not clear on what map does, can you expand on this? It
seems crucial to using iterators. Map applies the iterator to each element,
which applies the closure?

Also, to generalize this discussion a bit, would you ever use iter without map?
-->
<!-- I hope this new breakdown/rearranging has cleared up these comments you
had on the last version of this chapter about the difference between
iter and map. I hope the added examples where we've used iter without map have
cleared up the last question. /Carol -->

### Использование замыканий для получения доступа к переменным среды при работе итераторов

Продолжим расширение наших знаний об совместном использовании замыканий и итераторов.
Рассмотрим пример использования замыканий для получения доступа к переменным
внешней среды и использования адаптера итераторов `filter`. Этот метод получает в
качестве параметра замыкание, применяет замыкание к каждому элементу и возвращается
булево значение. Если в результае работы кода замыкание возвращается значение `false`,
то данный элемент игнорируется при создание нового итератора. Код 13-19 демонстрирует
использование `filter` и замыкания, которое получает доступ к переменной `shoe_size`
при обрадотки коллекции данных структур `Shoe` для того, чтобы выбрать только те,
которые подходят под определенный размер:


<span class="filename">Filename: src/lib.rs</span>

```rust,test_harness
#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

<span class="caption">Код 13-19: использование метода `filter`, замыкания и переменной
функции `shoe_size`</span>

<!-- Will add wingdings in libreoffice /Carol -->

Функция `shoes_in_my_size` получает во владение вектор структур данных и числовое
значение. Данная функция возвращает вектор содержащий только структуры подходящие
под определенные критерии (в данном случае описания обуви определенного размера).
В теле функции мы вызываем метод `into_iter` для создания итератора, который получит
владение вектором. Далее вызываем метод `filter`, который применит к каждому элементу
вектора замыкание. Данное замыкание возвращает логическое значение результат сравнения
поля структуры с аргументом функции. В результате, метод `collect` объединит полученные
данные в вектор, который будет возвращен функцией в качестве выходных данных.

Тест наглядно демонстрирует результат работы функции.

### Реализация типажа `Iterator` для создания нового итератора

<!-- So it seems like we are creating a program with an iterator inside, is
that right? I assumed the whole thing we were making was an iterator at first,
which lead to a few confusions, can you lay it out up front? -->
<!-- I'm not sure what you mean here, can you elaborate on what the distinction
is to you between "a program with an iterator inside" and "whole thing we were
making was an iterator"? I don't understand what you mean by these terms so I'm
not sure how to clear this up. /Carol -->

Вы уже видели в примерах, как можно создать итератор вектора (с помощью вызовов
функций `iter`, `into_iter` или `iter_mut`). Мы также можем создать итераторы из
других типов коллекций стандартной библиотеки (например, `map`). Помимо этого мы
можем реализовать `Iterator` для обработки любых данных. Для этого необходимо
реализовать метод `next`. После этого мы можем использовать все методы типажа
`Iterator` (используя реализации самого типажа).


<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

Реализуемый нами итератор будет считать от одного до пяти. Для начала создадим
структуры для хранения значений. Далее напишем реализацию типажа `Iterator`

В коде 13-20 определение структуры `Counter` и реализации функции `new` для
создания экземпляра структуры `Counter`:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

<span class="caption">Код 13-20: определения структуры `Counter` и функции `new`,
которая создаёт экземпляр структуры `Counter` с инициализированным значением `0` поля
`count`</span>

<!-- Could you add a filename here? I think that will help the reader keep
track of what they're working on. Can you also just sum up in a line what this
code has accomplished so far? I moved this down from above the code, if this
will do? -->
<!-- Done /Carol -->

<!-- Why define the new method, if it isn't necessary? Or is that what this
next line is telling us? -->
<!-- So does this code just initialize it with 0? Is that jat { count: 0 }
does?-->
<!-- I've rearranged to make this clearer /Carol -->

Далее мы реализуем метод `next` (код 13-21):

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<span class="caption">Код 13-21: реализация типажа `Iterator` в структуре `Counter`</span>

<!-- I will add wingdings in libreoffice /Carol -->

Рассмотрим содержание кода реализации типажа подробнее. Мы установили тип `Item`
(тип выходных данных метода `next`) `u32`. Более подробно о ассоциированных типах
мы поговорим в Главе 19. Обратим теперь внимание на содержание реализации метода.
Мы хотим чтобы наш итератор добавлял единицу к текущему значению. Поэтому мы инициировали
поле `count` 0. Если значение этого поля меньше 6, функция `next` возвращает текущее
значение внутри `Some`. Если же это поле равно 6 или больше итератор вернёт `None`.

#### Пример использования итератора `Counter`

После того как метод `next` реализован, т.е. соблюдены все ограничения типажа
`Iterator` - мы получили реализацию итератора. Код 13-22 демонстрирует проверку
работы нашей реализации:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Iterator for Counter {
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         self.count += 1;
#
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

<span class="caption">Код 13-22: тестирования реализации метода `next`</span>

В этом тесте создаётся экземпляр структуры `Counter` - переменная `counter`.
Далее вызывается метод `next` и проверяется его выходные данные. Как и предполагалось,
метод возвращает числа от 1 до 5, а при последующих вызовах возвращает `None`.

<!-- So if I have this right, the first line creates a new Counter called
counter, and the rest of them merely call counter with next, store it in x, and
then print x? And we have to do that 5 times to get the 1-5 count? Phew, could
you wrap that up if indeed it is correct!) and sum up here? -->
<!-- I decided to change this into a test rather than printing out values, and
I added some summary text about what the test is doing. Is this clearer? /Carol
-->

#### Использование других методов типажа `Iterator`

Т.к. мы реализовали типаж `Iterator`, мы можем использовать все его доступные
методы.

<!-- So we can't just use these methods anyway? It seems like we did earlier,
but here we have to use next first, before we cam access these methods? -->
<!-- No, we don't have to *use* `next` before we can use the other methods, we
have to *define* `next` before we can use the other methods. I hope the various
rewordings and reworkings have made this clearer by this point. /Carol -->

<!-- below: once you've done what, defined a default implementation? Only then
can you use other adapters, is that what we're saying? And I'm still not clear
on what an adapter does/means, as opposed to a method, or consumer, at this
point. -->
<!-- I hope this comment has been cleared up too /Carol -->

Пример использования методов типажа, доступных её реализации (13-23):

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Counter {
#     fn new() -> Counter {
#         Counter { count: 0 }
#     }
# }
#
# impl Iterator for Counter {
#     // Our iterator will produce u32s
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         // increment our count. This is why we started at zero.
#         self.count += 1;
#
#         // check to see if we've finished counting or not.
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```

<span class="caption">Listing 13-23: Использование множества методов типажа `Iterator`
</span>

Вызов всех методов доступен, т.к. были соблюдены все формальные условия реализации
типажа `Iterator`.
