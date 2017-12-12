## Замыкания: анонимные функции, которые могут имеют доступ к своему окружению

<!-- Bill's suggested we flesh out some of these subtitles, which I think we
did more with earlier chapters but we (I, included!) have got a bit lax with. I
don't think this is quite right, is there a shorter heading we could use to
capture what a closure is/is for? -->
<!-- I've attempted a more descriptive subtitle, what do you think? /Carol -->

Замыкания в Rust - это анонимные функции, ссылки на которые вы можете сохранять в
переменные или сделать их аргументами других функций. Вы можете создавать в одном
месте, а вызывать и их работы в другом контексте. В отличии от функций, замыканиям
дозволено иметь доступ к переменным той области видимости, в которой они используются.
Далее будут продемонстрировано, как эти опции помогают сократить количество используемого
кода в программе и улучшить её поведение.

<!-- Can you say what sets closures apart from functions, explicitly, above? I
can't see it clearly enough to be confident, after one read through this
chapter. I think it would help to have the closure definition up front, to help
to let the reader know what they are looking out for in the examples. When
would you use a closure, for example, rather than using a function? And is it
the closure that's stored in the variable, or is it the result of applying the
closure to a value passed as an argument? -->
<!-- I've tried reworking the above paragraph and restructuring the examples to
be more motivating. I've also tried to make it clear throughout that storing a
closure is storing the *unevaluated* code, and then you call the closure in
order to get the result. /Carol -->

### Создание обобщенного поведения используя замыкания

Рассмотрим пример, демонстрирующий сохранение замыкания для дальнейшего использования.
Мы также рассмотрим синтаксис замыканий, типизированный интерфейс и типажи.

Представим, что мы работаем на в стартапе, где создаём приложение для генерации
планов тренировок. Серверная часть приложения создаётся на Rust. На сервере храниться
множество данных: возраст, индекс тела, предпочтения, последние результаты тренировок
и индекс интенсивности тренировок. При проектировании приложения конкретные алгоритмы
реализаций не важны. Важно, чтобы различные расчёты не занимали много времени.
Мы буде симулировать работу алгоритма расчета параметров с помощью функции
`simulated_expensive_calculation` (13-1), которая печатает `calculating slowly...`,
ждёт две секунды и выводит результат расчёта:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

<span class="caption">Код программы 13-1: Описания функции, которая моделирует
расчёт различных параметров</span>

Далее рассмотрим содержание функции `main`, которое содержит части нашего приложения.
В примере моделируется вызов кода, который генерирует план занятий. Т.к. взаимодействие
с клиентской частью программы не связано с использованием замыканий, мы также
смоделируем это взаимодействие. Программно будут вводиться данные и печататься результаты.

Описание входных данных:

- Индекс интенсивности (`intensity`) - определяет когда запрашивается тренировка.
  Этот индекс говорит о предпочтениях (низкая или высокая интенсивность)
- Случайный номер, который будет сгенерирован для выбора плана тренировки

В результате программа напечатает рекомендованный план занятий.

Код 13-2 показывает содержание функции `main`. Мы программно ввели вводимые пользователем
показатели для простоты демонстрации работы:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
# fn generate_workout(intensity: i32, random_number: i32) {}
```

<span class="caption">Код программы 13-2: Функция `main` содержащая симуляцию
пользовательского ввода данных и вызов функции `generate_workout`</span>

Это и есть контекст в котором мы будем работать. Функция `generate_workout` в
примере кода 13-3 содержит логику работу программы, которую мы будем изучать в
этом примере.

contains the business logic of the app that we’re most concerned
with in this example. Остальные изменения в коде будут сделаны в этой функции:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: i32) -> i32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: i32, random_number: i32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            )
        }
    }
}
```

<span class="caption">Код программы 13-3: Печать плана тренировки зависит от введенных
данных и вызова функции `simulated_expensive_calculation` </span>

Код 13-3 несколько раз вызывает функцию расчета.

<!-- Will add wingdings in libreoffice /Carol -->

Желаемое поведение функции `generate_workout` следующее: проверка хочет ли
пользователь низкой интенсивности тренировки (индекс меньше 25) или высокой (25 и более).
Невысокая интенсивность будет рекомендовать количество повторений и подходов на
основании сложного алгоритма, который мы моделируем функцией `simulated_expensive_calculation`.

Если же пользователь хочет высокую интенсивность тренировок - добавляется опция -
случайный образом выбираемое число. Если оно равно 3, то предлагается сделать перерыв.

Общая логика представлена. Теперь можно заняться рефакторингом кода. Для начала
устраним дублирование кода. Пример первого приближения 13-4:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: i32) -> i32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: i32, random_number: i32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            )
        }
    }
}
```

<span class="caption">Код программы 13-4: Перенос вызова функции
`simulated_expensive_calculation` в одно место перед блоком `if` и сохранение
результата в переменную `expensive_result`</span>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Теперь  в любом случае данная функция вызывается. Это не хорошо, т.к. не весь код
метода использует результаты её работы. Для решения этой задачи замыкания подходят
лучше всего.

### Замыкания сохраняю код, который может быть запущен позднее

Вместо того, чтобы всегда запускать функцию `simulated_expensive_calculation`
перед блоком `if`, мы может определить замыкание и сохранить его в переменную.
Пример 13-5:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
# expensive_closure(5);
```

<span class="caption">Код программы 13-5: Инициализация замыкания</span>

<!-- Can you elaborate on *how* to define the closure first? I've had a go here
based on what I can see but not sure it's correct. Are we saying that a closure
is function that assigned its result to a variable you can then use? -->
<!-- I've attempted to elaborate on defining a closure in the next few
paragraphs starting here, is this better? /Carol -->

Определения замыкания мы начинаем с пары палочек (vertical pipes (`|`)). Внутри
этой конструкции мы определяем параметры замыкания. Такой синтаксис был выбран
под влиянием языков Ruby и Smalltalk. Замыкание имеет параметр `num`. Несколько
параметров разделяются запятыми `|param1, param2|`.

Далее идёт тело функции-замыкания. Фигурные скобки могут не использоваться, если
код функции состоит только из одной строчки кода. После закрытия фигурных скобок
необходим символ `;`. Обратите внимание, что после `num` нет `;`. Это означает, что
переменная будет возращена функцией.

Также обратите внимание, что `let`-переменная `expensive_closure`содержит
определение функции-замыкания, а не результат её работы.

Теперь, после определения замыкания мы можем изменить код в блоках `if`, вызывая
код замыкания по необходимости. Вызов функции-замыкания очень напоминает вызов
функции.

Вызов замыкания очень похож на вызов функции. Мы определяем имя переменной, которая
содержит определение замыкания и в скобках указываем аргументы, которые мы хотим
использовать для вызова. Пример 13-6:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
fn generate_workout(intensity: i32, random_number: i32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            )
        }
    }
}
```

<span class="caption">Пример 13-6: Вызов замыкания `expensive_closure`</span>

Мы решили задачу многократного использования (вызова) одного кода (т.е. кода
объявленного в одном месте). Но мы всё-таки не решили вопрос минимизации количества
вызываемого код (кэширования результата). Код по-прежнему может вызываться дважды.
Этот вопрос может решить локальная переменная, объявленная в блоке `if`.  Есть ещё
более лучшее решение, к которому мы вернемся чуть позже. А сейчас обсудим почему
у замыканий не может быть аннотаций типов и ещё кое-что о связях типажей и замыканий.

### Интерфейс типа замыкания и аннотация (Closure Type Inference and Annotation)

Замыкания отличаются от функций определяемых с помощью ключевого слова `fn` в некоторых
аспектах. Замыкания не требуют аннотирования типов параметров или возвращаемого
значения как это могут делать функции `fn`.

<!-- I've suggested moving this next paragraph up from below, I found this
section difficult to follow with this next paragraph -->

Типизированные аннотации необходимы функциям т.к. они являются частью проявленного
интерфейса с пользователем. Определение этого интерфейса важно для уверенности в
том, что все использующие данную функцию принимают её входные и выходные данные
корректно. Замыкания же не используют явного интерфейса, хотя они сохраняются в
переменных и используются без имени и давая возможность запускать их на выполнение.

Кроме того, описание замыкания может быть более коротким и может быть использовано
только в узком контексте, а не в любом месте программы. Из-за ограничения контекстов,
Благодаря этого ограничению компилятор может правильно подбирать типы данных
параметров и возвращаемого типа.

<!--Can you expand above on what you mean by "stored in bindings and called
directly"? Do you mean stored in a variable? I'm struggling to visualize how
closures are used, and what the important difference is between them and
functions. I think a clearer definition of what they are, what they do, and
what they're used for at the start of the closures section would help clear
this up -->
<!-- Yes, sorry, in Rust terminology "binding" is mostly synonymous to
"variable", but when we started working on the book we decided to be consistent
and more like what people are used to by referring to the concept as "variable"
throughout, but we missed this spot. /Carol -->

Также как и при определении переменных, мы можем добавить описание типа данных
переменных замыкания и типа возвращаемого значения (для большей информативности).
Пример 13-7:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num: i32| -> i32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

<span class="caption">Код 13-7: добавления описания типов данных замыкания</span>

<!-- Why might you want to, if you don't need to? In a particular situation? -->
<!-- I've added an explanation /Carol -->

<!-- Below -- Am I right in assuming the closures below are doing the same
thing as the functions? -->
<!-- Yes /Carol -->

Синтаксическое сравнения описания замыкания и функции:

<!-- Prod: can you align this as shown in the text? -->
<!-- I'm confused, does this note mean that production *won't* be aligning all
of our other code examples as shown in the text? That's concerning to me, we're
trying to illustrate idiomatic Rust style in all the code examples, so our
alignment is always intentional... /Carol -->

```rust,ignore
fn  add_one_v1   (x: i32) -> i32 { x + 1 }
let add_one_v2 = |x: i32| -> i32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

<!-- Can you point out where we're looking at here, where the important
differences lie? -->
<!-- The importance isn't the difference but the similarity... I've tried to
clarify /Carol -->

<!-- Will add wingdings and ghosting in libreoffice /Carol -->

Проиллюстрированные описания замыканий развноценны.

<!--Below--I'm not sure I'm following, is the i8 type being inferred? It seems
like we're annotating it. -->
<!-- The types in the function definitions are being inferred, but since Rust's
variable types for numbers defaults to `i32`, in order to illustrate our point
here we're forcing the type of the *variable* to be `i8` by annotating it in
the *variable* declaration. I've changed the example to hopefully be less
confusing and convey our point better. /Carol -->

Определения замыканий будут иметь один конкретный тип данных для каждого из параметров
и выходных данных. Например (код 13-8) показывает определение замыкания и его
использование:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

<span class="caption">Код 13-8: Попытка использовать замыкание с различными типами
данных</span>

The compiler gives us this error:

```text
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integral variable
  |
  = note: expected type `std::string::String`
             found type `{integer}`
```

<!-- Will add wingdings in libreoffice /Carol -->

После того, как вы в первый раз вызвали замыкание и использовали переменные типа
данных `String`, компилятор неявным образом подставит этот тип в замыкание. Этот
тип данных будет неизменным у замыкания на протяжении всего времени жизни.

### Использование замыканий совместно с обобщёнными типами (дженериками) и типажом `Fn`

Возвратимся к нашему приложению для генерации тренировочных программ. В коде 13-6
мы ещё используем неоднократно замыкание. Больше чем это на надо. Решение с кэшированием
данных вычислений в переменной увеличит и усложнит наш код.

Есть ещё одно решение. Мы можем создать структуру, которая будет хранить замыкание
и результат её работы. Структура выполнить код замыкания если только в этом будет
необходимость. Данная структура будет кэшировать результат работы замыкания,
благодаря чему в коде программы не будет необходимости в усложнении кода. Такое
шаблонное решение называется *запоминанием* (*memoization*) или *ленивой инициализацией*
(*lazy evaluation*).

Для того чтобы структура могла в качестве поля иметь замыкание, мы должны явным
образом указать его тип. Каждое замыкание имеет свой уникальный тип (даже если два
замыкания имеют одни и туже сигнатуру, их типы будут считаться различными). Для
установки типа данных замыкания в структуре, перечислении или функции мы должны
использовать обобщающие типы и определенные типажи.

<!-- So Fn is a trait built into the language, is that right? I wasn't sure if
it was just a placeholder here -->
<!-- Fn is provided by the standard library; I've clarified here. /Carol -->

Типаж `Fn` входит в состав стандартной библиотеки. Все замыкания реализуют один из
типажей: `Fn`, `FnMut` или `FnOnce`. Мы поговорим о различиях между ним в следующей
секции. В данном примере мы можем использовать типаж `Fn`.

Мы добавим типы в типаж `Fn` для описания типов параметров и возвращаемого значения,
которое замыкания должны иметь для того, чтобы соответствовать данному типажу. В данном
случае, наше замыкание имеет тип параметр `i32` и возвращает `i32`. Сигнатура типажа
имеет вид: `Fn(i32) -> i32`.

Код 13-9 показывает определение структуры `Cacher` содержащей замыкание:

<span class="filename">Filename: src/main.rs</span>

```rust
struct Cacher<T>
    where T: Fn(i32) -> i32
{
    calculation: T,
    value: Option<i32>,
}
```

<span class="caption">Код 13-9: определение структуры `Cacher` содержащей замыкание
 `calculation` и результат в `value`</span>

Структура `Cacher` имеет поле `calculation` типа `T`. Тип данных замыкания `T`
описывается сигнатурой типажа `Fn`. Любые замыкания, которые может содержать поле
`calculation` в экземпляре `Cacher` должно иметь один параметр типа `i32` и возвращать
 `i32` (определено после `->`).

Поле `value` имеет тип `Option<i32>`. Перед выполнением замыкания `value` будет `None`.
Если код использует структуру `Cacher` хочет получить результат замыкания, мы
выполним замыкания и сохраним результат в значении перечисления `Some`. Если же код
программы запросит значение замыкания ещё раз будет возвращено значение из `Some`.

Описанная логика реализована в примере кода 13-10:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct Cacher<T>
#     where T: Fn(i32) -> i32
# {
#     calculation: T,
#     value: Option<i32>,
# }
#
impl<T> Cacher<T>
    where T: Fn(i32) -> i32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

<!-- Liz: the `new` function is using the "struct init shorthand" by just
saying `calculation` instead of `calculation: calculation`, since the parameter
matches the struct field name. This was recently added to stable Rust and we've
added an introduction of it in Chapter 5. Just adding an explanation here for
you in case you read this chapter before the changes we've made to Chapter 5!
/Carol -->

<span class="caption">Код 13-10: Реализация структуры `Cacher`, метода `new` и
метода `value`, который управляет логикой кэширования</span>

Поля структуры `Cacher` закрытые, т.к. мы хотим, чтобы экземпляр структуры управлял
содержание полей и не было возможности извне каким-либо образом на это влиять. Функция
`Cacher::new` получает обобщенный параметр `T`. Данная функция возвращает экземпляр
структуры `Cacher` содержащая замыкание в поле `calculation` и `None` в поле `value`.

Когда вызывающий код хочет получить результат работы замыкания, вместо того чтобы
вызывать замыкание непосредственно, он вызывает метод `value`. Этот метод проверяет
есть ли уже результат работы замыкания в поле `self.value` внутри значения перечисления
`Option::Some`. Если там есть значение, это значение возвращается вызывающему коду.
При этом замыкание больше не используется для получения результата.

Если же поле `self.value` имеет значение `None`, то вызывается замыкание из поля
`self.calculation` и результат работы записывается в поле `self.value` для будущего
использования и, далее, полученное значение также возвращается вызывающему коду.

Пример кода 13-11 демонстрирует использование структуры `Cacher` в функции
`generate_workout` из примера 13-6:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# struct Cacher<T>
#     where T: Fn(i32) -> i32
# {
#     calculation: T,
#     value: Option<i32>,
# }
#
# impl<T> Cacher<T>
#     where T: Fn(i32) -> i32
# {
#     fn new(calculation: T) -> Cacher<T> {
#         Cacher {
#             calculation,
#             value: None,
#         }
#     }
#
#     fn value(&mut self, arg: i32) -> i32 {
#         match self.value {
#             Some(v) => v,
#             None => {
#                 let v = (self.calculation)(arg);
#                 self.value = Some(v);
#                 v
#             },
#         }
#     }
# }
#
fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}
```

<span class="caption">Код 13-11: Использование экземпляра структуры `Cacher` в
функции `generate_workout` для реализации кэширования</span>

<!-- Will add ghosting and wingdings in libreoffice /Carol -->

Вместо сохранения замыкания в переменную, мы создаём новый экземпляр структуры
`Cacher` для хранения замыкания. Далее, в каждом месте, где необходим результат
работы замыкания мы вызываем метод `value`. Мы может вызывать этот метод сколько
угодно или вообще не вызывать. При любом количестве вызовов функции `value` (один
раз или более) замыкание будет использовано только один раз. Пожалуйста, проверьте
работу кода с использованием функции `main`.

Хотя экземпляр структуры `Cacher` прекрасно справляется со своими обязанностями
и в функции `generate_workout` можно без каких-либо дополнительных затрат описать
логику работы, у текущей реализации `Cacher` есть всё же ограничения с контекстом
использования замыкания.

Первое ограничение - предполагается, что параметр `arg` всегда будет одинаковым.
Изменение этого условия приводит к ошибке:

```rust,ignore
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

Этот тест создаёт новый экземпляр `Cacher` с замыканием и возвращает значение.
Мы вызываем метод `value` с параметром `arg` со значением `1`, а потом с `2`.
Предполагаем, что когда мы введём значение `2`, то и должны получить это значение.

Тест не будет пройден:

```text
thread 'call_with_different_arg_values' panicked at 'assertion failed:
`(left == right)` (left: `1`, right: `2`)', src/main.rs
```

Проблема в том, что при первом вызове `c.value` с аргументом 1 экземпляр `Cacher`
сохранит значение `Some(1)` в `self.value`. После этого, неважно какие будут входные
параметры. Функция всегда будет возвращать 1.

Решением будет использования хэш-таблицы вместо одного значения. Ключи будут значениями
входных данных `arg`, а значениями будут результаты работы замыкания. Вместо того,
чтобы изучения значений перечислений функция `value` должна произвести поиск в
хэш-таблице и возвратить значение, если оно там находится. При необходимости будет
вызван код замыкания и будут произведены соответствующие вычисления.

```rust,ignore
fn value(&mut self, arg: i32) -> i32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg,v);
                v
            },
        }
}
```

Ещё одним ограничением является тип данных. На данным момент ими могут быть
только целочисленные значения типа `i32`. Мы бы хотели иметь возможность использовать
различные типы данных (срезы строки, `usize` и другие). Попытаемся решить этот
вопросы с использованием обобщенных параметров.

### Замыкания могут получать доступ переменным области видимости

В рассматриваемом нами примере генератора учебных планов мы использовали замыкания
только как встроенные анонимные функции. Возможности же замыкание шире.

<!-- To clarify, by enclosing scope, do you mean the scope that the closure is
inside? Can you expand on that?-->
<!-- Yes, I've tried here to clarify that it's the scope in which the closure
is defined /Carol -->

Код 13-12 демонстрирует пример переменной замыкания `equal_to_x`, содержание которой
использует переменные в облсти видимости (переменная `x`):

<!-- To clarify how we talk about a closure, does the closure include the
variable name, or are we referring to the closure as the functionality that is
on the right side of the = and so not including to variable name? I thought it
was the former, but it seems like the latter above. If it's the former, would
"an example of a closure with the variable `equal_to_x`" make more sense? -->
<!-- No, the closure does not include the variable name in which it's stored;
storing a closure in a variable is optional. It should always be the latter,
and I hope the reworking of the example used throughout the closure section
and making the wording consistent has cleared this confusion up by this point.
/Carol -->

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
    println!("{}",equal_to_x(y));
}
```

<span class="caption">Код 13-12: пример замыкания, которое использует внешнюю
переменную</span>

В этом примере показано, что замыканию позволена использовать переменную `x`,
которая определена в той же области видимости, что и переменная `equal_to_x`.

<!-- So *why* is this allowed with closures and not functions, what about
closures makes this safe? -->
<!-- It's not really about safety; capturing the environment is the defining
functionality a closure has. The reason functions *don't* capture their
environment is mostly around storage overhead; I've added an explanation of
that aspect. Can you elaborate on what led to the conclusion that allowing
captures with functions wouldn't be safe and what you mean by "safe" here?
/Carol -->

Такой функциональной возможности функции не имеют:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```

Описание ошибки:

```text
error[E0434]: can't capture dynamic environment in a fn item; use the || { ... }
closure form instead
 -->
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^
```

Рассмотрим внутреннюю организацию работы с внешними переменными у замыканий. Для
хранения данных о внешних переменных у замыканий предусмотрена хранилище (кэш).
Использование данной возможности накладывает дополнительную нагрузку на ресурсы
системы (память). Для большей безопасности и устойчивой работы системы было принято
решение отключить у функций такою возможность.

<!-- Why didn't this work, is there a reason ingrained in the language? Or is
that not really relevant? -->
<!-- I've added an explanation /Carol -->

Замыкания могут получить доступ к переменным среды выполнения несколькими (тремя)
способами (которые соответствуют возможностям функций при работе с своими аргументами):
владение, заимствование, изменяемое (нефиксированное) заимствование. Все эти возможности
описаны в типажах, которые замыкания могут следовать:

* `FnOnce` получает значения из области видимости (*environment*). Для получения
  доступа к переменным замыкание должно получить во владения используемые переменные.
  Замыкание не может получить во владение одну и туже переменную несколько раз.
* `Fn` заимствует значения из среды (не изменяя при этом их значений).
* `FnMut` может изменять значения переменных.

Когда мы создаём замыкание, компилятор делает выводы о целях использования переменных
среды на основании используемых значений. В примере 13-12 `equal_to_x` получает
доступ к  `x` (readonly), т.е. замыкания реализует `Fn`.

Для получения владения переменными используется ключевое слово `move` перед списком
параметров. Это удобно, когда замакание перемещается в другой поток. Мы рассмотрим
примеры использования `move` в Главе 16, когда будем рассматривать возможности Rust
для разработки многопоточных приложений. В примере 13-12 ключевое слово `move` добавлено
в определении замыкания и используется вектор вместо целочисленного значения.
Примитивные типы (как мы знаем) могут быть скопированы (а нам надо перемещать):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

Описание ошибки:

```text
error[E0382]: use of moved value: `x`
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
    implement the `Copy` trait
```

Здесь переменная `x` перемещена в замыкание её определении. Поэтому в функции
`main` переменная `x` большое не может быть использована. Для устранения ошибки
компиляции, устраните эту ошибку (например, удалите строку 6).

В большинстве случаев типаж `Fn` будет использован. Компилятор сам вам сообщит,
когда лучшем решение было бы использовать `FnMut` или `FnOnce` (на основании использования
внешних переменных замыканием).

Иллюстрации использования замыканий в качестве параметров функции мы рассмотрим в
следующей секции, "Итераторы".
