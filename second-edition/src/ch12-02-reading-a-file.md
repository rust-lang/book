## Чтение файла

Далее, мы дополним функционал нашей программы, чтобы иметь возможность читать
содержимое файла. Для начала создадим файл в корне нашего проекта с текстовыми данными `poem.txt` и добавим
в него следующее содержание стихотворение “I’m nobody! Who are you?”:

<span class="filename">Filename: poem.txt</span>

```text
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<span class="caption">Тест 12-3: Стихотворение Эмили Дикинсон “I’m nobody! Who are you?”</span>

Далее, рассмотрим код решающий поставленную задачу  *src/main.rs*  12-4:

<span class="filename">Filename: src/main.rs</span>

```rust,should_panic
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let query = &args[1];
#     let filename = &args[2];
#
#     println!("Searching for {}", query);
    // ...snip...
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

<span class="caption">Код 12-4: Чтение содержания файла</span>

Изучим содержание исходного кода. Прежде всего, мы добавили импорт необходимых
модулей и структуры. Для чтения файла нам необходимо:
1. Модуль `std::env`.
2. Структура `std::fs::File`.
3. Всё содержимое модуля `std::io::prelude::*`. В этом модуле есть множество типажей для работы с файлами.

Коде функции `main` мы добавили вызов функции `File::open`, входным параметром, которой
является содержание переменной `filename`. Далее мы создали переменную `contents`,
установив атрибут изменяемости  и присвоив экземпляр структуры данных `String`.
Данная переменная будет содержащийся текст в открытом файле. Далее, используя метод
экземпляра структуры `File` `read_to_string` производим считывание текста в переменную
и выводим её содержание на консоль.

Пожалуйста, проверьте работу нашей программы. Введите какой-либо текст в качестве
первого аргумента и название файла вторым аргументом:

```text
$ cargo run the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Отлично! Наша программа может читать текстовые данные файла. Хотя наша программа
решает поставленную задачу, она не лишена недостатков. Прежде всего функция `main`
решает множество задач. Такую функцию неудобно тестировать. Далее, не отслеживаются
возможные ошибки ввода данных. Пока наша программа небольшая - данными недочётами
можно пренебречь. При увеличении размеров программы такую программу будет всё сложнее
и сложнее поддерживать. Хорошей практикой программирования является трансформация,
перестройка кода (refactoring) по мере её усложнения. Поэтому далее мы улучшим наш
код с помощью улучшения его структуры.
