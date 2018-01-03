## Проектирование интерфейса пула потоков

Давайте поговорим о том, как должен выглядеть пул. Авторы часто находят
что при попытке создать некоторый код, напив сначала клиентский интерфейс можно
лучше понять как лучше реализовать серверную часть. Напишите API кода, который будет
структурирован таким образом, чтобы его было удобно вызывать, а затем реализуйте
функциональность этой структуры, а не наоборот.

Подобно тому, как мы использовали Test Driven Development в проекте в главе 12,
здесь мы собираемся использовать Compiler Driven Development. Мы собираемся написать
код, который вызывает функции, которые мы хотели бы иметь. Ошибки компиляции будут
направлять нашу дальнейшую разработку

### Структура кода при использовании `thread::spawn`

Первое, мы рассмотрим код, который нам нужно реализовать для создания нового
потока. Это не будет окончательным решение, т.к. существует потенциальная проблема
(создание множества потоков), о которой мы говорили ранее. В коде 20-11 показаны
изменения в функции `main`, которые необходимы для создания нового потока в цикле
`for`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">код 20-11: Создание нового потока для каждого соединения с колентом</span>

Как мы узнали в главе 16, `thread::spawn` создаст новый поток, а затем запустит
код в замыкании. Если вы запустите этот код и загрузите `/sleep` и затем `/` в
двух вкладках браузера, вы действительно увидите, что запрос `/` не будет
дождаться окончания `/sleep`. Но, как мы уже говорили, это в конечном итоге
будет избыточно расходовать ресурсы системы, так как мы создаем новые потоки
без ограничений.

### Реализация подобного интерфейса с помощью `ThreadPool`

Мы хотим, чтобы пул потоков работал похожим образом. В коде 20-12 заменим
предыдущее решения использование структуры `ThreadPool`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
# struct ThreadPool;
# impl ThreadPool {
#    fn new(size: u32) -> ThreadPool { ThreadPool }
#    fn execute<F>(&self, f: F)
#        where F: FnOnce() + Send + 'static {}
# }
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">код 20-12: использование `ThreadPool` без реализации</span>

Мы используем `ThreadPool::new` для создания нового пула с изменяемым количеством
потоков (в данном случае 4). Далее в цикле `for` мы выполняем `pool.execute` также
как мы выполняли `thread::spawn`.

### Использование Compiler Driven Development для реализации рабочего кода

Давайте попробуем скомпилировать данный код. Мы получим следующие ошибки:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module
   `ThreadPool`

error: aborting due to previous error
```

Отлично! Нам нужен `ThreadPool`. Давайте вернёмся к контейнеру из бинарного файла.
Реализация `ThreadPool` будет независимой от работы веб-сервера. После того, как
библиотека реализующая работу пула потоков будет написана, мы сможем использовать
её в любых реализациях.

Итак, контейнер будет содержать файл *src/lib.rs*  с простыми определением структуры
`ThreadPool`, которую мы сейчас можем иметь:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;
```

Далее, мы создаём новую папку *src/bin* и перемещаем бинарный контейнер  *src/main.rs*
в *src/bin/main.rs*. Это сделает библиотечный контейнер основным в папке *hello*.
Это перемещение не повлияет на порядок запуска `cargo run` бинарного файла. После
перемещения файла *main.rs* внесите в самом верху текста программы изменения,
описав подключение библиотеки `hello` и её содержания в область программы
*src/bin/main.rs*:

<span class="filename">Filename: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;
```

Далее, попытайтесь теперь проверить корректность нашего кода, получил следующие
указания компилятора для нас:

```text
$ cargo check --bins
   Compiling hello v0.1.0 (file:///projects/hello)
error: no associated item named `new` found for type `hello::ThreadPool` in the
current scope
  --> src\main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^
   |
```

Отлично! Следующим нашим действием будет реализация функции `new` для структуры
`ThreadPool`. Также мы знаем, что функции `new` потребуется один параметр, который
может принять знание `4` в качестве аргумента. Также эта функция должна возвращать
экземпляр структуры `ThreadPool`. Давайте реализуем такую функцию, которая будет
иметь все эти характеристики:

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        ThreadPool
    }
}
```

Му установили `u32` в качестве типа входящего параметра переменной `size`, т.к.
отрицательные значения не имеют смысла. Запустим проверку узнаем наше следующее
рекомендуемое действие:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

error: no method named `execute` found for type `hello::ThreadPool` in the
current scope
  --> src/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

Отлично. Предостережение и ошибка. Пока проигнорируем предостережение. Исправим
ошибку. Реализуем метод `execute`. Если вы помните главу 13, мы можем использовать
замыкание в качестве параметра, как в трёх различных типажах: `Fn`, `FnMut` и `FnOnce`.
Какой же типаж нам лучше использовать? Т.к. мы должны реализовать что-то вроде
`thread::spawn` мы можем посмотреть документацию:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` is the parameter we care about here; `T` is related to the return value and
we’re not concerned with that. Given that `spawn` uses `FnOnce` as the trait
bound on `F`, it’s probably what we want as well, since we’ll eventually be
passing the argument we get in `execute` to `spawn`. We can be further
confident that `FnOnce` is the trait that we want to use since the thread for
running a request is only going to execute that request’s closure one time.

`F` also has the trait bound `Send` and the lifetime bound `'static`, which
also make sense for our situation: we need `Send` to transfer the closure from
one thread to another, and `'static` because we don’t know how long the thread
will execute. Let’s create an `execute` method on `ThreadPool` that will take a
generic parameter `F` with these bounds:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    // ...snip...

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

The `FnOnce` trait still needs the `()` after it since this `FnOnce` is
representing a closure that takes no parameters and doesn’t return a value.
Just like function definitions, the return type can be omitted from the
signature, but even if we have no parameters, we still need the parentheses.

Again, since we’re working on getting the interface compiling, we’re adding the
simplest implementation of the `execute` method, which does nothing. Let’s
check again:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`, #[warn(unused_variables)] on by default
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: u32) -> ThreadPool {
  |                ^^^^

warning: unused variable: `f`, #[warn(unused_variables)] on by default
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
```

Обратите внимание, что код компилируется. Но если вы попытаетесь запустить программу
`cargo run` вы получите ошибки, как в начала нашей главы. Пока наша библиотека не
готова к использованию.

> О языках со строгими компиляторами говорят (как о Haskell  и Rust), что если
> код компилируется - он работает. Очень важно понять, что это всего лишь этап, а
> не конечное решение. Наш код компилируется, но он пока ещё ничего не делает. Сейчас
> наступает этап написать тесты, которые бы проверили бы корректность поведения кода.
