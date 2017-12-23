## Shared State Concurrency

Обмен сообщениями не является единственным способом взаимодействия в многопоточной
среде. Вспомните Go-слоган (о котором мы уже упоминали):

> Не взаимодействуйте с помощью разделения памяти - разделяйте память путём
> взаимодействия.

Что же значит "взаимодействие путём разделения памяти"? И более того, почему же
такой способ обмена информацией так не любят любители отправлять сообщения?

Суть в том, что взаимодействие с помощью каналов - это что-то вроде монопольного
владения. Многопоточное разделения памяти - это что-то вроде множественного владения.
Как вы знаете множественное владение возможно посредством умных указателей (Глава
15). Этот функционал несёт в себе дополнительные сложности.

Система владения Rust может помочь решить многие сложности. Рассмотрим один из
минимальных и неделимых компонент многопоточности - мьютексы.

### Мьютексы предоставляют доступ к данным из одного потока (за раз)

Мьютекс применяется для разделения памяти. Он позволяет только одному потоку получать
доступ к данным. Для организации такого доступа необходимо соблюсти последовательность
действий:

1. Перед тем как попытаться получить доступ к данным необходимо получить блокировку.
2. После того, как данные были использованы вам необходимо разблокировать их (отдать
 блокировку).

 Реальным примером мьютекса будет дискуссия на конференции, где есть только один
 микрофон. Прежде чем участник дискуссии может говорить, он должен
 спросить или сообщить, что он хотели бы использовать микрофон. Как только он получит
 микрофон, он может разговаривать столько, сколько захочет, а затем отдаст
 микрофон следующему участнику, который хотел бы поговорить. Было бы грубо
 начать кричать, не имея микрофона или украсть микрофон до того, как другой участник
 закончит. Никто больше не сможет говорить, если участник дискуссии забыл передать
 микрофон следующему, когда он закончил использовать его. Если управление общим
 микрофоном будет нарушено любым и вышеприведённых способов, общение не будет работать
 так, как планировалось!

 Управление мьютексами может быть невероятно сложным и именно поэтому
 многие люди с энтузиазмом относятся к каналам. Однако в Rust мы не можем использовать
 блокировку и разблокировка неправильным образом, благодаря системе типов и владению.

#### `Mutex<T>` API

Давайте рассмотрим пример использования мьютекса в коде 16-12, без использования
несколько потоков:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

<span class="caption">код 16-12: работа `Mutex<T>` API в однопоточном контексте</span>

Как и любые другие типы, тип `Mutex<T>` имеет ассоциированную функцию `new`.
Для доступа к данным внутри мьютекса мы используем метод `lock`. Блокировка будет
недоступна другим потокам пока текущий не отдаст её.

Как вы уже, наверное догадались `Mutex<T>` является умным указателем. Метод `lock`
возвращает умныйй указатель `MutexGuard`. Он реализовал `Deref` и `Drop`.

#### Разделение `Mutex<T>` между множеством потоков

Рассмотрим пример разделения значения между множеством потоков, используя `Mutex<T>`.


Let’s now try to share a value between multiple threads using `Mutex<T>`. Мы
создадим 10 потоков и каждый будет увеличивать счётчик на 1. Реализуя эту задачу
мы будем получать ошибки, на которых мы будем учиться и изучать работу с `Mutex<T>`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">код 16-13: создание 10 потоков, каждый из которых увеличивает
счётчик на 1 с помощью `Mutex<T>`</span>

Мы создаём переменную `counter` внутри `Mutex<T>`. Далее мы создаём 10 потоков с
помощью цикла `for`. Внутри цикла мы увеличиваем значение на 1

Для того, чтобы все дочерние потоки выполнили свою работу - мы используем `join`,
чтобы подождать выполнения каждого потока.

Сообщения об ошибках:

```text
error[E0373]: closure may outlive the current function, but it borrows
`counter`, which is owned by the current function
  -->
   |
9  |         let handle = thread::spawn(|| {
   |                                    ^^ may outlive borrowed value `counter`
10 |             let mut num = counter.lock().unwrap();
   |                           ------- `counter` is borrowed here
   |
help: to force the closure to take ownership of `counter` (and any other
referenced variables), use the `move` keyword, as shown:
   |         let handle = thread::spawn(move || {
```

Для решения этой проблемы будем использовать `move`:

```rust,ignore
thread::spawn(move || {
```

Запускаем - получили новые ошибки:

```text
error[E0382]: capture of moved value: `counter`
  -->
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  -->
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

К сожалению, это решение не работает.

Упростим наши пример, чтобы лучше разобраться. Вместо цикла просто создадим
два потока:

```rust,ignore
let handle = thread::spawn(move || {
    let mut num = counter.lock().unwrap();

    *num += 1;
});
handles.push(handle);

let handle2 = thread::spawn(move || {
    let mut num2 = counter.lock().unwrap();

    *num2 += 1;
});
handles.push(handle2);
```

Запускаем - получили ошибки:

```text
error[E0382]: capture of moved value: `counter`
  -->
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  -->
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

Компилятор сообщает, что нельзя передавать владение между потоками.

#### Множественное владение между множеством потоков

Мы уже знаем как можно разделять владение данных с помощью умного указателя `Rc<T>`.
Раньше мы использовали его только в однопоточном контексте. Рассмотрим его работу
в многопоточном. Обернём `Mutex<T>` в `Rc<T>` и будем клонировать `Rc<T>` перед
перемещение владения в поток:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">код 16-14: попытка использования `Rc<T>` для организации владения
потоками `Mutex<T>`</span>

Ошибка:

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied
  -->
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ the trait `std::marker::Send` is not
   implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   |
   = note: `std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads
   safely
   = note: required because it appears within the type
   `[closure@src/main.rs:11:36: 15:10
   counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
```

Мы поговорим о `Send` в следующей секции.

К сожалению, умный указатель `Rc<T>`не может защитить данные в многопоточной среде.
А что если у нас будет тип похожий на `Rc<T>`, но работающий в потокобезопасном
режиме?


#### Атомарный счётчик ссылок `Arc<T>`

Тип потобезопасный счётчик сылок - это `Arc<T>`. Он входит в стандартную библиотеку в
модуль `std::sync::atomic`.

Почему не все примитивные типы атомарны и почему не все типы стандартной библиотеки
реализовали `Arc<T>` по умолчанию. Ответ - из-за проблем с производительностью.
Многопоточность накладывает дополнительные расходы ресурсов.

По своему поведения `Arc<T>` и `Rc<T>` идентичны:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
    	let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">код 16-15: использование `Arc<T>` для предоставления владения
между множеством потоков</span>

This will print:

```text
Result: 10
```

Возможно, вы заметили, что, поскольку `counter` является неизменным, но мы можем
получить изменяемую ссылку на значение внутри него. Это означает, что `Mutex <T>`
обеспечивает внутреннюю изменчивость, как `Cell`. Точно так же, как мы использовали
`RefCell <T>` в главе 15, чтобы иметь возможность изменять содержимое внутри
`Rc <T>`, мы используем `Mutex <T>`, чтобы иметь возможность изменять содержимое
внутри `Arc <T>`.

Напомним, что `Rc <T>` не решило проблему доступа к данным в мнопоточной среде.
Также `Mutex <T>` не предотвращает взаимные блокировки.

В следующей секции мы поговорим о использовании типажей `Send` и `Sync` для работы
в многопоточной среде.
