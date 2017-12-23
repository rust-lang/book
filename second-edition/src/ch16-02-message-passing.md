## Отправление сообщений (данных) потоками

Одна из опций многопоточности, которая делает её столько популярной - это
*передача сообщений*. Это когда потоки или субъекты общаются путем отправки друг другу
сообщений, содержащие данные. Вот идея в форме лозунга:

> Не обмениваться данными путем разделения памяти; вместо этого, делитесь памятью
> путём общения.
>
> - [Эффективный переход] (http://golang.org/doc/effective_go.html)

Основным инструментом для достижения этой цели является *канал*. Канал имеет две
половинки: передатчик и приемник. Одна часть нашего кода может вызывать методы
передатчика данных, которые мы хотим отправить, а другая часть проверяет сообщения.

Мы приступим к примеру, в котором у нас есть один поток, который будет
генерировать значения и отправлять их по каналу. Основной поток получит
значения и распечатает их.

Приступим! Давайте начнем с создания канала (16-6):


<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
#     tx.send(()).unwrap();
}
```

<span class="caption">код 16-6: создание канала и присвоение значений переменным
`tx` и `rx`</span>

Функция `mpsc::channel` создаёт новый канал. Модуль `mpsc` содержит в себе функционал
для *множества отправителей и одного получателя*. Начнём с одного отправителя.
В дальнейшем добавим ещё.

Функция `mpsc::channel` возвращает кортеж. Отправитель и получатель. Это сокращения
от *transmitter* *receiver*. Здесь также мы используем выражение `let` для превращения
элементов кортежа в переменные (мы поговорим подробнее о возможностях `let` в
Главе 18).

Пример отправки текстового сообщения 16-7:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

<span class="caption">код 16-7: отправление из потока `tx` сообщения “hi”</span>

Мы используем функцию `thread::spawn` для создания нового потока. Мы используем
`move` для передачи переменой `tx`.

Сообщение отправляется с помощью метода `send`. Метод возвращает `Result<T, E>`.
Если получателя нет (или он удалён) будет ошибка. Чтобы её изменить используется
метод `unwrap` (для игнорирования ошибки).

В коде 16-8 мы продемонстрируем получаете значения:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

<span class="caption">код 16-8: отправление сообщения по каналу между потоками</span>

Отправитель имеет два полезных метода `recv` and `try_recv`.


Результат:

```text
Got: hi
```

### Как каналы используют владение

Рассмотрим взаимодействие каналов и владения на примере (16-9):

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

<span class="caption">код 16-9: попытка доступа к `val`после её отправки через канал</span>

Описание ошибки:

```text
error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
   not implement the `Copy` trait
```

### Отправка множества значений

Пример отправки множества сообщений.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

<span class="caption">код 16-10: отправка множества сообщений</span>

Результат:

```text
Got: hi
Got: from
Got: the
Got: thread
```

### Создание множества отправителей путем клонировния

Пример:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
# use std::time::Duration;
#
# fn main() {
// ...snip...
let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
// ...snip...
#
#     for received in rx {
#         println!("Got: {}", received);
#     }
# }
```

<span class="caption">код 16-11: отправка множества сообщений и остановка между каждым</span>

Вывод:

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```
