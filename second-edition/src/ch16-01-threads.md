## Использование потоков для запуска кода параллельно

В большинстве современных операционных систем, выполнения кода программы называют
*процессом*. Операционная система одновременно выполняет и управляет множеством
процессов.

Для упрощения работы с процессам вводится логическое понятие *потока*.

Разделение вычислений, которые ваша программа должна выполнять в нескольких потоках,
может повысить производительность, поскольку программа будет выполнять несколько
действий в в то же время. Однако программирование с помощью потоков может добавить
сложностей. Поскольку потоки выполняются одновременно, нет никакой гарантии
относительно порядка, в котором будут выполняться части вашего кода в разных потоках.
Это может привести к гонке условия, при которых потоки получают доступ к данным
или ресурсам в непоследовательном порядке; взаимоблокировкам, когда два потока
предотвращают продолжение друг друга или ошибки, которые происходят только в определенных
ситуациях, которые трудно воспроизвести надежно. Rust уменьшает влияние этих и
других недостатков использования потоков, но программирование в многопоточном
контексте все еще требует вдумчивости и код таких программ структурирован по-другому,
чем для однопотоковых программ.

Существует несколько разных способов, которыми языки программирования реализуют потоки.
Многие операционные системы предоставляют API для создания новых потоков. К тому же,
многие языки программирования предоставляют собственную специальную реализацию потоков.
Язык программирования, предоставляемые потоки, иногда называет *легким* или
*зеленые* потоки. Эти языки принимают несколько таких зеленых потоков и выполняют
их в контексте различного количества потоков операционной системы. По этой причине,
модель, в которой язык вызывает API-интерфейсы операционной системы для создания
потоки иногда называются *1:1*, один поток ОС для одного языкового потока.
Модель зеленых потоков называется *M:N* модель, «M» зеленых потоков на `N` ОС
потоков, где `M` и` N` не обязательно совпадают.

Каждая модель имеет свои преимущества и недостатки. Компромисс, который больше всего
важен значение для Rust - это поддержка времени выполнения. *Runtime* - запутанный
термин; он может иметь разное значение в разных контекстах. Здесь мы подразумеваем
некоторый код, включенный в каждый двоичный формат. Для некоторых языков этот код
большой, для других этот код невелик. Между собой, в обиходе программистов, когда
говорят «без времени исполнения» часто бывает имеют ввиду, «небольшое время выполнения».
Любой язык, не являющийся ассемблером, имеет некоторое количество времени выполнения.
Языки с малым бинарным кодом имеют меньше возможностей, но в результате это приводит
к меньшим двоичным файлам. Меньшие двоичные файлы упрощают объединение языка с другими
языками в других контекстах. Многие языки увеличивают двоичный файл в обмен на
дополнительные функции. Rust стремиться не использовать двоичные файлы.

Лёгкие потоки предполагают использование большие двоичные файлы, которые включают
в себе механизмы управления потоками. Поэтому Rust предоставляет модель 1:1.
Т.к. Rust - это низкоуровневый язык программирования, существуют решения позволяющие
реализовать N:M потоковые модели.

Теперь, когда мы определили, какие типы потоков использует Rust, давайте рассмотрим,
как использовать связанных с потоками API в стандартной библиотеке.

### Создание нового поток с помощью  `spawn`

Чтобы создать новый поток, мы вызываем функцию `thread::spawn` и передаем ему
замыкание (мы говорили о замыканиях в главе 13), содержащий код, который мы хотим
запускать в новом потоке. Пример в листинге 16-1 печатает некоторый текст из
нового потока и другой текст из основного потока:

To create a new thread, we call the `thread::spawn` function and pass it a
closure (we talked about closures in Chapter 13), containing the code we want
to run in the new thread. The example in Listing 16-1 prints some text from a
new thread and other text from the main thread:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

<span class="caption">код 16-1: создание нового потока для печати в отдельном
потоке чего-либо во время печати в главном потоке</span>

Обратите внимание, что когда главный поток прекратит работу, в новом потоке также
прекратится печать. Вывод может быть различным при каждом запуске программы:

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

Потоки, вероятно, по очереди будут работать, но это не гарантировано. В этом примере,
основной поток, напечатает первым, хотя в коде печать в коде будет объявлена раньше.
И хотя мы сказали, что создали поток для печати `i` до 9, он только напечатает до 5,
т.е. до закрытия основного потока. Если вы всегда видите работу только один потока
или если вы не видите перекрытия, попробуйте увеличивая число в диапазонах, чтобы
создать больше возможностей для потока сделать перерыв и дать другому потоку поворот.

#### Ожидание окончания работы всех потоков используя `join`

Не только код в листинге 16-1 не позволяет завершить запущенный поток. Нет никаких
гарантий, что потом может быть запущен. Для решения этой задачи используется
функция `thread :: spawn` и возвращаемому значению `JoinHandle`:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join();
}
```

<span class="caption">код 16-2: сохранение значения `JoinHandle` из `thread::spawn`
для гарантированного ожидания конца работы потока</span>

`JoinHandle` владеет значением, которое запустить ожидание выполнение потока с
помощью метода `join()`. Вывод будет примерно таким:

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```
Потоки всё ещё работают хаотично, но главный поток ожидает выполнения дочернего.

Если же мы перенесём код `handle.join()` перед циклом основного потока, то последовательность
строк вывода будет иной:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    handle.join();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
}
```

Главный поток будет ожидать окончания работы подчинённого:

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```


### Использование `move`-замыканий в потоках

Есть ещё одна опция замыканий, о которой мы ещё не говорили. Это `move`-замыкания

> Создание замыканий, которые получают доступ к значения из среды выполнения часто
> используются при запуске нового потока.

Продемонстрируем это на примерах!

Обратите внимание на замыкание, которое мы послали в качестве входных данные в
функцию `thread::spawn` в коде 16-1. Мы не использовали данных из в основного потока.
Рассмотрим первое приближение:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}
```

<span class="caption">код 16-3: попытка использования вектора, созданного в основном
потоке из дочернего потока</span>

Замыкание использует `v`, т.е. `v` будет частью среды замыкания. Т.к. `thread::spawn`
запускает выполнение замыкания в новом потоке.

Описание ошибки:

```text
error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
 -->
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

К сожалению, существуют определенные проблемы: мы заранее не можем знать время жизни
дочернего потока и как долго переменная будет действительна.

Демонстрация проблемы явным образом (16-4):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join();
}
```

<span class="caption">код 16-4: поток с замыканием, который пытается получить ссылку
на `v` из главного потока, который удаляет `v`</span>


Для решения проблемы последуем совету компилятора:

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword, as shown:
  |     let handle = thread::spawn(move || {
```

Добавив `move` перед замыканием, мы представляем замыканию владение значением.
Пример рабочего приложения:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join();
}
```

<span class="caption">код 16-5: использование `move` для передачи владения переменной
в дочерний поток</span>

А что если добавить `drop`? Код не будет скомпилирован:

```text
error[E0382]: use of moved value: `v`
  -->
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
```

Функционал "владение" помог компилятору обнаружить и избежать ошибки. Ура!

Теперь, когда вы познакомились с потоками и некоторыми элементами API, пора рассмотреть
более интересные примеры.
