## Однопоточный веб-сервер

Для начала рассмотрим работу однопоточного веб-сервера. Мы будет работать с
байтовыми TCP и HTTP запросами и в качестве ответа будем возвращать HTML от клиента
к веб-серверу. Кратко рассмотрим протоколы, с которыми будет работать.

*Протокол передачи гипертекста* (*HTTP*), который использует Интернет, построен
над *Протоколом управления передачей* (*TCP*). Мы не будем вдаваться в подробности,
но вот краткий обзор: TCP - это протокол низкого уровня, а HTTP строит протокол
более высокого уровня поверх TCP. Оба протокола - это то, что называется
*протокол запроса-ответа*, то есть *клиент*, который инициирует
запросов и *сервера*, который прослушивает запросы и предоставляет ответ
клиент. Содержание этих запросов и ответов определяют сами протоколы.

TCP описывает низкоуровневые сведения о том, как информация поступает с одного
сервера на другой, но не указывает, что это за информация; это всего лишь куча
единиц и нулей. HTTP строит поверх TCP, определяя, чем содержимое запросов и ответов
 должны быть. Таким образом, технически возможно использовать HTTP с другими протоколами,
 но в подавляющем большинстве случаев HTTP отправляет данные поверх TCP.

Итак, первое, что нам нужно создать для нашего веб-сервера - это прослушивание
TCP-соединение. В стандартной библиотеке есть модуль `std::net`, который позволяет
делать это. Создадим новый проект:

```text
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

И добавим код 20-1 в файл `src/main.rs`. Функционал будет прослушивать адрес
`127.0.0.1:8080` входящих TCP-потоков. Когда мы получим входящий поток, будет
напечатано `Connection established!`:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

<span class="caption">код 20-1: чтение входящих потоков и печать сообщения, когда
мы получаем поток </span>


`TcpListener` позволяет прослушивать TCP-соединения. Мы решили прослушивать адрес
`127.0.0.1:8080`. Адрес делится  `:` на две части. Левая часть содержать IP-адрес,
идентифицирующий компьютер в сети, а правая часть содержит порт `8080`. Мы выбрали
именно этот порт, т.к. HTTP обычно принимает порт 80, но подключение к порту 80
требует привилегий администратора. Обычные пользователи могу прослушивать порты
с номером начиная с 1024. Порт 8080 легко запомнить, т.к. он повторяет порт HTTP 80.

Функция `bind` что-то типа функции `new`. Она возвращает экземпляр `TcpListener`.
В сетевой терминологии, мы часто говорим о "связывании с портом", поэтому функция,
которая создающая прослушивающее соединение называется `bind`.

Функция `bind` возвращает `Result<T, E>`. Связывание может быть неудачным, например,
если мы попытаемся соединиться с портом 80 без прав администратора. Другим примером
неудачи при связывании, это когда несколько программ пытаются получить доступ к
одному порту (например, два экземпляра одной программы). Т.к. мы собираемся делать
простой сервер и не собираемся беспокоится о подобных ошибках - мы просто будем
использовать `unwrap` для обработки возможных ошибок.

Метод `incoming` в` TcpListener` возвращает итератор, который предоставляет
последовательность потоков (более конкретно, потоки типа `TcpStream`).
*stream* представляет собой открытое соединение между клиентом и сервером.
*connection* - это имя для полного процесса запроса / ответа, когда клиент
подключается к серверу, сервер генерирует ответ, а сервер закрывает соединение.
Таким образом, «TcpStream» позволяет нам читать, чтобы увидеть, что клиент
отправил и мы можем написать наш ответ ему. Итак, этот цикл `for` будет обрабатывать
каждое соединение по очереди и производить серию потоков для обработки.

На данный момент обработка потока означает вызов `unwrap` для завершения нашей
программы, если поток имеет какие-либо ошибки, а затем печатает сообщение.
Ошибки могут произойти, потому что мы фактически не итерируем данные соединения,
мы итерируем через *попытки соединение*. Соединение может не работать по нескольким
причинам, многие из них специфические для операционной системы. Например, многие
операционные системы имеют ограниченное количество одновременных открытых соединений;
новые попытки подключения будут вызвать ошибку, пока некоторые из открытых соединений
не будут закрыты.

Давайте посмотрим, как работает этот код! Сначала вызовем `cargo run`, затем загрузим
`127.0.0.1:8080` в веб-браузере. В браузере появится сообщение об ошибке
скажет что-то похожее на «Сброс соединения», так как мы сейчас
отправка любых данных назад. Если мы посмотрим на наш терминал, мы увидим кучу
сообщения, которые были напечатаны при подключении браузера к серверу!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

Мы получили несколько сообщений, распечатанных для одного запроса браузера; эти
соединения делал браузер для получения различных данных: делающий запрос на страницу
запрос на значок `favicon.ico`, отображаемый на вкладке браузера, или браузер может
быть повторить соединение. Наш браузер общается запросами HTTP, но мы не
ответили ни на один запрос, просто закрываем соединение, перейдя к следующему
итерации цикла. Когда `stream` выходит за пределы области действия и удалён в конце
цикла, его соединение закрывается как часть реализации `drop` для
`TcpStream`. Браузеры иногда обрабатывают закрытые соединения, повторяя, поскольку
проблема может быть временной. Важно то, что мы успешно создали простой обработчик
TCP-соединении!

Не забудьте остановить программу с помощью комбинации клавиш <span class="keystroke">ctrl-C</span>.

### Чтение запросов

Давайте прочитаем запрос браузера! Для этого нам понадобится добавить функциональных
возможностей для этих целей. Создадим новую функцию для обработки соединения. В
этой функции (назовём её `handle_connection`) мы будем читать данные из потока
`stream` и выводить их на печать. Код 20-2:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

<span class="caption">код 20-2: чтение из потока `TcpStream` и печать данных</span>

Мы добавили `std::io::prelude` в начало, чтобы получить возможность использовать
функционал чтения и записи потока данных. Вместо того, чтобы печатать сообщение,
которое мы получили, мы используем функцию `handle_connection` и отправляем
`stream` в неё.

В функции `handle_connection` параметра `stream` является изменяемым `mut`. Во время
чтения из потока `TcpStream` мы можем прочитать больше запрашиваемых данных в буфер.
Также мы можем отслеживать полученную информацию. `mut` нам нужна т.к. поток
может изменяться.

Затем нам нужно прочитать из потока. Мы делаем это в два этапа: во-первых,
мы объявляем переменную `buffer` в стеке для хранения данных, которые мы читаем.
Мы сделали буфер размером 512 байт, который достаточно велик, чтобы хранить данные
запроса. Этого достаточно для наших целей в этой главе. Если бы мы хотели
обрабатывать запросы произвольного размера, управление буфером должно быть больше
сложным, но мы сохраняем его простым. Затем мы передаем буфер в `stream.read`,
который будет читать байты из` TcpStream` и помещать их в буфер.

Затем мы преобразуем байты в буфер в строку и распечатаем эту строку.
Функция `String::from_utf8_lossy` принимает `&[u8]` и создает `String`.
`lossy` часть имени происходит от поведения, когда эта функция видит недействительные
последовательности UTF-8: она заменяет недопустимые последовательности на
�, `U+FFFD REPLACEMENT CHARACTER`. Вы можете увидеть заменяющие символы для оставшихся
символов в буфере, которые не заполняются данными запроса.

Давайте попробуем! Запустите программу и сделайте запрос в веб-браузере еще раз.
Обратите внимание, что в браузере все равно будет отображаться страница с ошибкой,
но вывод нашей программы в терминале теперь будут выглядеть примерно так:


```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

Вероятно, вы получите немного другой результат (это зависимости от вашего браузера).
Вы также может повторить этот запрос. Теперь, когда мы печатаем запрашивать данные,
мы можем понять, почему мы получаем несколько соединений от одного
запрос браузера, посмотрев путь после `Request: GET`. Если повторение
все соединения запрашивают `/`, мы знаем, что браузер пытается извлечь `/`
неоднократно, так как он не получает от нас ответа.

Давайте разберем данные запроса, чтобы понять, что браузер запрашивает у нас.
HTTP - это текстовый протокол, и запрос принимает этот формат:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

Первая строка называется *строкой запроса*, и она содержит информацию о
что клиенте. Первая часть строки запроса - это *метод*, например `GET` или` POST`,
который описывает, как клиент делает этот запрос.

Затем идёт *URI* запроса, который обозначает *Uniform Resource Identifier*.
URI являются почти, но не совсем такими же, как URL (*Uniform Resource Locators*),
что мы обычно называем адресами, которые мы вводим в веб-браузер.
Спецификация HTTP использует термин URI, а разница между URI и URL-адресами не является
важной для наших целей этой главы, поэтому мы можем просто мысленно заменить
URL для URI здесь.

Затем идёт версия HTTP, которую использовал клиент, а затем строка запроса
заканчивается последовательностью CRLF. Последовательность CRLF также может быть
записана как `\ r \ n`:` \ r` - это *возврат каретки*, а `\ n` - *перевод строка*.
Эти термины остались со времён пишущие машинки! Последовательность CRLF отделяет
строку запроса от остальной части данные запроса.

Взглянув на данные строки запроса, мы увидели наш код:

```text
GET / HTTP/1.1
```

`GET` является методом, `/` URI запроса и `HTTP/1.1` версией протокола.

Остальные строки, начинающиеся с `Host:` - являются заголовками; Запросы `GET`
не имеют тела.

Посмотрите, как будут меняться данные при использовании разных браузеров или при попытке
получить разные данные (`127.0.0.1:8080/test`).

Теперь, когда мы знаем, что запрашивает браузер, давайте вернем некоторые данные!

### Написание ответа браузеру

Отправим данные обратно в наш браузер в ответ на его запрос. Формат ответа:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

Первая строка называется *строкой статуса* и содержит версию HTTP, используемую в
ответе, числовой код состояния, который описывает результат запроса текстовое описание
кода состояния. После последовательности CRLF могут идти заголовки в любом порядке,
другую последовательность CRLF и тело ответа.

Вот пример ответа, который использует версию 1.1 HTTP, имеет код состояния
`200`, фразу `OK` и никаких заголовков и тела:

```text
HTTP/1.1 200 OK\r\n\r\n
```
Этот текст является простым успешным ответом HTTP. Давайте напишем его в поток!
Удалите `println!`, который печатает данные запроса, и добавьте код в код 20-3:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="caption">код 20-3: написание простого успешного HTTP-ответа в поток</span>

Первая новая строка определяет переменную `response`, которая содержит данные
успешного ответа, который мы отправляем обратно. Затем мы вызываем `as_bytes`
`response`, потому что метод` write` в `stream` принимает` & [u8] `и отправляет
эти байты непосредственно вниз по соединению.

Операция `write` может завершиться неудачей, поэтому` write` возвращает `Result <T, E>`;
мы продолжает использовать `unwrap` для обработки ошибок. Наконец, `flush` будет
ожидать, пока все байты записываются в соединение; `TcpStream` содержит внутренний
буфер для минимизирования вызовов в базовую операционную систему.

С этими изменениями давайте запустим наш код и сделаем запрос! Мы больше не будем
печатать каких-либо данных на терминал, поэтому мы не увидим каких-либо результатов,
кроме выход от Cargo. Когда мы загружаем `127.0.0.1:8080` в веб-браузере, вместо
ошибки мы получаем пустую страницу. Ура! Вы только что закодированы вручную
HTTP-запрос и ответ!

### Возвращение HTML браузеру

Давайте вернем больше, чем просто пустую страницу. Создайте новый файл, * hello.html *,
в корневой папке вашего каталога проекта, то есть не в каталоге `src`. Вы можете
поместите любой HTML-код в него. Например, код 20-4:


<span class="filename">Filename: hello.html</span>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

<span class="caption">код 20-4: содержание HTML-файла, который будет возвращаться
при ответе</span>

Это простой HTML-файл с заголовком и абзацем. Для его отправке браузеру изменим
код нашей программы (функцию `handle_connection`), как показано в коде 20-5:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
use std::fs::File;

// ...snip...

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="caption">код 20-5: отправка содержания HTML-файла *hello.html* браузеру</span>

Мы добавили строку вверху, чтобы добавить ссылку на `File` стандартной библиотеки.
Код открытия и чтения файла должен быть вам уже знаком, поскольку мы уже имели
аналогичный код в главе 12, когда мы читали содержимое файла для нашего ввода-вывода
в коде 12-4.

Затем мы используем макрос `format!', чтобы добавить содержимое файла в качестве
тела ответ ответа, который мы пишем в поток.

С помощью команды `cargo run` и запроса браузера `127.0.0.1:8080` мы можем увидеть
результат работы программы в окне браузера.

Обратите внимание, что мы в настоящее время игнорируем данные запроса в переменной
`buffer` и отправляем содержимое файла HTML. Попробуйте запросить
`127.0.0.1:8080/something-else` в вашем браузере, и мы увидим тот же HTML-код.
Отправка назад того же ответа для всех запросов довольно ограничена. Давайте теперь
анализировать запрос и будем обрабатывать только правильно оформленные запросы
на адрес `/`.

### Проверка запроса и выборочное возвращение ответа

Прямо сейчас наш веб-сервер возвращает HTML независимо от того, что клиент запросил.
Давайте проверим, что браузер запрашивает `/` или будем возвращать ошибку, если
браузер запрашивает что-либо еще. Давайте изменим `handle_connection`, как показано
в коде 20-6, который содержать необходимые изменения. Этот код проверяет содержимое
полученного нами запроса:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// ...snip...

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // some other request
    };
}
```

<span class="caption">код 20-6: Согласование запроса с содержанием. Мы ожидаем запрос
`/`. Для этого мы настраиваем анализ строки</span>

Here, we hardcoded the data corresponding to the request that we’re looking for
in the variable `get`. Because we’re reading raw bytes into the buffer, we use
a byte string, created with `b""`, to make `get` a byte string too. Then, we
check to see if `buffer` starts with the bytes in `get`. If it does, we’ve
gotten a well-formed request to `/`, which is the success case that we want to
handle in the `if` block. The `if` block contains the code we added in Listing
20-5 that returns the contents of our HTML file.

If `buffer` does not start with the bytes in `get`, we’ve gotten some other
request. We’ll respond to all other requests using the code we’re about to add
in the `else` block.

If you run this code and request `127.0.0.1:8080`, you’ll get the HTML that’s
in *hello.html*. If you make any other request, such as
`127.0.0.1:8080/something-else`, you’ll get a connection error like we saw when
running the code in Listing 20-1 and Listing 20-2.

Let’s add code to the `else` block as shown in Listing 20-7 to return a
response with the status code `404`, which signals that the content for the
request was not found. We’ll also return HTML for a page to render in the
browser indicating as such to the end user:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
# fn handle_connection(mut stream: TcpStream) {
# if true {
// ...snip...

} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
# }
```

<span class="caption">Listing 20-7: Responding with status code `404` and an
error page if anything other than `/` was requested</span>

Here, our response has a status line with status code `404` and the reason phrase
`NOT FOUND`. We still aren’t returning any headers, and the body of the
response will be the HTML in the file *404.html*. Also create a *404.html* file
next to *hello.html* for the error page; again feel free to use any HTML you’d
like or use the example HTML in Listing 20-8:

<span class="filename">Filename: 404.html</span>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

<span class="caption">Listing 20-8: Sample content for the page to send back
with any `404` response</span>

With these changes, try running your server again. Requesting `127.0.0.1:8080`
should return the contents of *hello.html*, and any other request, like
`127.0.0.1:8080/foo`, should return the error HTML from *404.html*!

There’s a lot of repetition between the code in the `if` and the `else` blocks:
they’re both reading files and writing the contents of the files to the stream.
The only differences between the two cases are the status line and the
filename. Let’s pull those differences out into an `if` and `else` of one line
each that will assign the values of the status line and the filename to
variables; we can then use those variables unconditionally in the code to read
the file and write the response. The resulting code after this refactoring is
shown in Listing 20-9:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// ...snip...

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
#
#     let get = b"GET / HTTP/1.1\r\n";
    // ...snip...

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="caption">код 20-9: сокращение строчек кода</span>

Here, the only thing the `if` and `else` blocks do is return the appropriate
values for the status line and filename in a tuple; we then use destructuring
to assign these two values to `status_line` and `filename` using a pattern in
the `let` statement like we discussed in Chapter 18.

The duplicated code to read the file and write the response is now outside the
`if` and `else` blocks, and uses the `status_line` and `filename` variables.
This makes it easier to see exactly what’s different between the two cases, and
makes it so that we only have one place to update the code if we want to change
how the file reading and response writing works. The behavior of the code in
Listing 20-9 will be exactly the same as that in Listing 20-8.

Отлично! Мы реализовали простейший веб-сервер и уложилисьв 40 строчек кода. Мы
реализовали логичные ответ - если запрашивается страница - возвращаем страницу,
если что-либо ещё - возвращаем страницу с информацией об ошибке `404`.

Т.к. сервер работает в однопоточном режиме, одновременно он может обрабатывать
только один запрос. Далее мы смоделируем работу сервера под нагрузкой.
