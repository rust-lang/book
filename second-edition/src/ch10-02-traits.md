## Типажи: определение общего поведения

Типажи позволяют использовать особые виды абстракций. Он позволяют типам данных
получать поведение. *Типаж* сообщает компилятору о функциональности определенного
типа, который может быть передан другому типу. Дополнительно к тому, что мы можем
использовать обобщенные типы данных, мы можем использовать *связывание с типажами*.

> Обратите внимание: *Типажи* похожи на интерфейсы (например, в языке Java), но
> имеют отличия.

### Определение типажа

Поведение типа определяется теми методами, которые мы можем использовать.
Различные типы разделяют поведение, если мы можем вызвать одни и теже методы во всех
типах. Определение типажей - это способ группировки определений методов вместе для
того чтобы иметь множество поведений необходимых для достижения каких-либо целей.

Например, у нас есть несколько структур, которые имеют различные типы и количество
текста. Структура `NewsArticle` содержит новости, которые печатаются в различных
местах в мире. Структура `Tweet` имеет 140 символов для содержания ссылки и короткого
сообщения.

Мы хотим создать библиотеку для хранения и отображения коротких описаний данных,
которые могли бы быть сохранены в экземплярах структур `NewsArticle` или `Tweet`.
Необходимо, чтобы каждая структура имела возможность делать короткие заметки
на основе имеющихся данных. Это должно происходить при вызове метода экземпляра
`summary`. Пример (10-12) иллюстрирует определение типажа `Summarizable`, в котором
есть необходимый метод (действие, поведение):

<span class="filename">Filename: lib.rs</span>

```rust
pub trait Summarizable {
    fn summary(&self) -> String;
}
```

<span class="caption">Код 10-12: Определение типажа `Summarizable`, который содержит
поведение (метод `summary`)</span>

Обратите внимание на синтаксис определения поведения. Синтаксис напоминает определение
структуры. Отличие вы, наверное, тоже заметили. Оно в описании метода. Только описание.
Реализации нет. Каждый тип данных, которые реализует это поведение должен иметь
свою реализацию. Компилятор будет проверять, что каждый тип реализующий данное поведение
делал это в точном соответствии с описанием.

Типаж может иметь несколько описаний методов. Каждое описание должно находиться на
одной строке и все они должны закачиваться символом ";".

### Реализация типажа в типах

После описания типажа `Summarizable` можно описать, реализовать типы имеющие соответствующее
поведение. Код (10-13) показывает реализацию типажа `Summarizable` в структуре
`NewsArticle`. Эта структура имеет поля для формирования описания.
 Структура `Tweet` также имеет поля, для формирования содержания описания.

<span class="filename">Filename: lib.rs</span>

```rust
 pub trait Summarizable {
     fn summary(&self) -> String;
 }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

<span class="caption">Код программы 10-13: Реализация типажа `Summarizable`
в структурах `NewsArticle` и `Tweet`</span>

Реализация типажа это тоже самое, что реализация методов, которые не связаны с
типажом. Различия в том что после ключевого слова `impl` мы сообщаем имя типажа,
который будем реализовывать. Далее идет ключевое слово `for` и затем типа. Внутри
блока мы пишем определение функции и её реализацию.

После того, как мы реализовали типаж, мы можем вызвать методы экземпляров
`NewsArticle` и `Tweet` тем же способом, что и вызов методов, которые не являются
частью типажа:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

Чтобы проверить работу созданных программных решений в одной проекте, создадим
новый проект с названием aggregator:

```
cargo new aggregator && cd aggregator
```

Далее в файле *lib.rs* внесем код (10-13). Далее создадим файл *main.rs*.
Внесем в него следующий код:

```rust,ignore
extern crate aggregator;

use aggregator::Summarizable;
use aggregator::Tweet;


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}

```

Далее, запускайте проект на выполнение:

```
cargo run
```

В результате работы кода этой программы будет напечатано:
`1 new tweet: horse_ebooks: of course, as you probably already know, people`.

Обратите внимание, что т.к. мы объявили типаж `Summarizable` и типы `NewsArticle`
и `Tweet` в файле `lib.rs`, все они в одной и той же области видимости.
Если кто-либо ещё захочет использовать функционал нашего контейнера и также реализовать
поведение `Summarizable`, то в этом случае необходимо импортировать типаж `Summarizable`
в область видимости (10-14):

<span class="filename">Filename: lib.rs</span>

```rust,ignore
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}
```

<span class="caption">Listing 10-14: Добавление типажа `Summarizable` из контейнера
`aggregator` в область видимости другого крата</span>

Для проверки работы этого кода программы создайте новый проект ch10_weather_forecast.
В файл lib.rs добавьте код (10-14). Далее, создайте файл main.rs в него внесите
следующий код:

```rust,ignore
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}
```

Обратите внимание, что данный код не будет скомпилирован, т.к. контейнер `aggregator`
не будет найден. В этом проявляется ограничение работы с типажами.

Пожалуйста, обратите внимание на реализации типажей. Нельзя реализовывать внешние типажи
во внешних типах данных. Например, нельзя реализовать типаж `Display` в структуре `Vec`,
т.к. их код находится в стандартной библиотеке. Разрешается реализовывать внешние
типажи во внутренних типах. Например, типаж `Display` можно реализовать в структуре
`Tweet` внутри крата `aggregator`. Также можно изменить структуру стандартной библиотеки
`Vec` реализовав в ней типаж `Summarizable` также  внутри контейнера `aggregator`.
Такое ограничение называется *orphan rule* и оно существует для предотвращения
дублирования и надёжности библиотек кода.

### Реализации по умолчанию

Весьма удобно, когда существует поведение по умолчанию. Это может уменьшить
трудозатраты программиста, если это поведения ему подходи в его структурах.

Код (10-15) демонстрирует реализацию поведения в типаже. Это поведение можно будет
переписать в реализации:

<span class="filename">Filename: lib.rs</span>

```rust
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

<span class="caption">Listing 10-15: Реализация в типаже `Summarizable` поведения
`summary`</span>

Если по каким-то причинам вы хотите реализовать поведение типажа без его перезаписывания,
то просто напишите следующий код (пустой блок):

```rust,ignore

impl Summarizable for DefaultArticle {}
```

Использование структуры и унаследованного поведения от типажа `NewsArticle`:

```rust,ignore
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summary());
```

This code prints `New article available! (Read more...)`.

Изменения сделанные в типаже `Summarizable` (реализация поведения) не отражается
каким-либо образом на структурах, которые ещё реализовали ранее.

Можно усложнить реализацию метода и в нём вызывать методы, которые не имеют
реализации:

```rust
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}
```

Теперь для того чтобы использовать типаж `Summarizable` в каждой реализации необходимо
реализовать метод `author_summary`:

```rust,ignore
impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
```

Т.е. после того как вы реализуете `author_summary` в структуре `Tweet` реализация
по умолчанию вызовет метод `author_summary`.
Если вы знакомы с объектно-ориентированным программированием в Java, уверен, тут
у Вас не будет возникать вопросов. Всё это напоминает интерфейсы и абстрактные классы
Java.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```

This will print `1 new tweet: (Read more from @horse_ebooks...)`.

Обратите также внимание, что нельзя вызвать метод по умолчанию из реализации.

### Связывание с типажом

Теперь, когда мы создали типажи и реализовали их в типах, мы можем усложнить код и
 добавить в них элементы обобщенного программирования. Мы может сократить количество
 возможных обобщенных типов данных с помощью типажей. Этот приём называется
 *связывание с типажом* в обобщенном типе данных.

 Например, в примере кода (10-13) мы реализовали типаж `Summarizable` в структурах
 `NewsArticle` и `Tweet`. Теперь мы можем объявить функцию `notify`, которая будет
 вызывать метод `summary` из переменной. Мы объявим в коде, что тип переменной `item`
 должен реализовывать типаж `Summarizable`:

```rust,ignore
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
```
Добавим этот метов внутрь блока `Summarizable`:

```rust,ignore
pub trait Summarizable {
//  fn summary(&self) -> String;
//  fn summary(&self) -> String {
//      String::from("(Read more...)")
//  }

  fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }

    fn notify<T: Summarizable>(item: T) {
        println!("Breaking news! {}", item.summary());
    }
}
```

Связывание с типажом - это ограничение накладываемое на обобщенный тип. Входящая
переменной может быть экземпляром любой структуры реализовавшей типаж `Summarizable`.
Внешний код программы, может вызвать метод `notify`, как, например `WeatherForecast`.
Все остальные типы данных, такие как `String` или `i32` не могут быть входными данными
этой функции, т.к. они не реализовали типаж `Summarizable`.

Для того, чтобы указать несколько типажей в список ограничений необходимо использовать
знак `+`, чтобы объединить их названия. Например, можно добавить `T: Summarizable +
Display`, чтобы наложить ограничение на особенности форматирования и реализацию
методов `Summarizable`.

Для функций, которые имеют множество обобщенных типов параметров, каждый обобщенный
параметр может иметь своё собственное связывание.

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

Для упрощения этой записи существует альтернативный синтаксис (использование `where`):

```rust,ignore
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

Такое описание более понятно.

### Исправление кода функции `largest` с помощью связывание с типажом

Каждый раз, когда вы хотите использовать поведение определенное в типаже в
обобщенном методе, вам необходимо определить обобщенный тип в области типа.

Для того, чтобы использовать оператор сравнения вам нужно указать в ограничении
типаж стандартной библиотеки  `std::cmp::PartialOrd`:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

Но это ещё не всё (попытавшись скомпилировать этот код, вы получите ошибку):

```text
error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut largest = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref largest` or `ref mut largest`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

Т.к. вы сделали функцию `largest`, то есть потенциальная возможность использовать
типы данных, которые не реализовали типаж `Copy` (только реализации типажа `Copy`)
имеют известный размер, а следовательно могут быть измерены, сравнены. Добавим
это ограничение в список:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

<span class="caption">Код 10-16: Реализация `largest` с помощью наложения ограничений
на типы входных параметров</span>

Если вы не хотите накладывать ограничение типажа `Copy` вы можете указать вместо
него типаж `Clone`. При этом будет использована куча для хранения данных.
Если же вы не хотите наложения ограничений - просто используйте ссылку `&T`, как
результат работы функции.

### Использование связывание типажа при выполнении определенных условий

Пойдём дальше и сделаем наш код ещё интереснее. Мы можем связывать с типажом при
наступлении определенных условий.

Например, тип `Pair<T>` из примера кода (10-17), всегда реализует метод `new`, но
в тоже время, реализует метод `cmp_display` только лишь если внутренний тип `T`
реализует типаж `PartialOrd` и `Display`:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

<span class="caption">Код программы 10-17: Реализация метода только при соблюдении
типами параметров определенных условий</span>

Мы также можем опционально реализовать типа для типа для любого типа который реализует
типаж. Эта особенность весьма часто используется в стандартной библиотеке.
Например, в стандартной библиотеке типаж `ToString` реализован, только если реализован
типаж `Display`. Синтаксис такого условия выглядит следующим образом:

```rust,ignore
impl<T: Display> ToString for T {
    // ...snip...
}
```

При таком условии мы можем вызвать метод `to_string` определенный в типаже `ToString`,
который реализовал типаж `Display`. Например, такой код будет корректным, т.к.
целочисленный тип реализовал типаж `Display`:

```rust
let s = 3.to_string();
```

В секции документации “Implementors” описываются подобные случаи использования типажей.

Все эти возможности, предоставляют инструмент для минимизации дублирования кода.
Rust предоставляет такие широкие возможности для того чтобы на этапе компиляции
можно было отследить возможные ошибки связывания типов.

Существует ещё одни тип обобщенного типов данных, которые можно использовать
даже без реализации. Это т.н. *lifetimes*. Вместо того, чтобы проверять наличие
реализаций в типах эти языковые конструкции помогают удостовериться в том, что
ссылки действительны. В следующей части этой главы вы узнаете об этом подробнее.
