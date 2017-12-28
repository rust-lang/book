## Реализация ООП шаблона проектирования

Давайте рассмотрим пример шаблона "Состояние" и как его использовать в Rust.
Шаблон *Состояние* - это когда на поведение влияет внутренние значения. Внутреннее
состояние представлено набором объектов, наследующих общую функциональность
(мы будем использовать структуры и типажи поскольку Rust не имеет объектов и
наследования). Каждый объект состояния ответственный за свое поведение и правила
изменения своего состояния. Значение, которое содержит один из этих состояний, не знает
что-либо о различном поведении других состояний или о том, когда происходит переход
между состояниями. В будущем, когда меняются требования, нам не придется менять
код значения, содержащего состояние или код, который использует значение. Что ж
нужно только обновить код внутри одного из объектов состояния, чтобы изменить его
правила или, возможно, добавить больше объектов состояния.

Чтобы изучить эту идею, мы собираемся внедрить рабочий процесс в блоге с помощью
последовательности шагов. Мы хотим реализовать последовательность рабочих процессов:

1. При создания поста создаётся пустой черновик.
2. Далее мы запрашиваем проверку черновика.
3. После проверки происходит публикация.
4. После публикации становиться доступна опция публикации содержания.

Любые другие попытки изменения поста не должны иметь какого-либо эффекта на его
внутреннее содержание. Например, если мы попытаемся перевести пост из состояния
"черновик" в состояние "принят" без проведения проверки (состояние "проверка"),
то пост останется в состоянии "черновик".

Код 17-11 показывает последовательность рабочий цикл публикации поста. Этот пример
использования мы и собираемся реализовать в библиотечном контейнере `blog`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<span class="caption">код 17-11: демонстрация необходимого поведения</span>

Мы хотим создать новый черновик постав в блоге с помощью метода `Post::new`. Далее
мы хотим добавить текст. Если мы попытаемся напечатать внесённый текст, мы это
не сможем сделать, т.к. пост является черновиком. Мы добавили `assert_eq!` в целях
демонстрации.

Далее, мы хотим перевести пост в состояние "проверка". После получения статуса
"проверено" пост можно будет опубликовать. Это значит, что можно будет получить
текст с помощью метода `content`.

Обратите внимание, что мы взаимодействуем только с типом `Post`. Внутри данного
типа происходит изменения состояния (черновик, ожидание проверки, опубликован).
Состояния меняются посредством вызова методов экземпляра `Post`. Мы не можем изменять
состояние непосредственно. Это также значит, что мы не можем совершить ошибки в
изменении последовательности получения данным экземпляром определенного состояния.

### Определение `Post` и  создание нового экземпляра (при этом экземпляр будет находится
  в состоянии "черновик"

Приступим к реализации библиотеки! Мы знаем техническое задание. Надо напечатать
содержание структуры `Post`, которое содержит какой-то текст. Начнём с определение
структуры и создадим функцию `new` для создания экземпляра `Post` (код 17-12).
Мы также реализуем закрытый типаж `State`. `Post` будет содержать типажный объект
`Box<State>` внутри `Option` внутри закрытого поля `state`. Вы увидите почему
использование перечисления `Option` необходимо. Типаж `State` определяет все
поведения состояния: "черновик"(`Draft`), "ожидание проверки" (`PendingReview`)
и "опубликован" (`Published`) (все они реализуют типаж) `State`. Сейчас типаж не
имеет методов и мы начинаем с определения состояния "черновик":

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

<span class="caption">код 17-12: определение структуры `Post` и функции `new`,
которая создаёт новый экземпляр `Post`. Также создание типажа `State`и структуры
`Draft`, которая реализует `State`</span>

Когда мы создаём новый экземпляр `Post` мы устанавливаем значение в поле `state`.
Оно будет содержать `Some` содержащее умный указатель `Box`, который указывает на
экземпляр структуры `Draft`. Таким образом мы релизуем первоначальную концепцию.
Т.к. поле `state` является закрытым нет другой возможности создвать экземпляр
`Post` с каким-либо другим состоянием.

### Сохранение текста поста

В функции `Post::new` мы установили значение поля `content` пустую `String`.

В коде 17-11 мы показали, что мы можем вызвать метод `add_text` и передать ей
`&str`, для того чтобы добавить текс в содержание поста. Мы реализуем этот метод
для того, чтобы защитить переменную `content` от возможности внешнего изменения.
Мы хотим, чтобы был котроль над содержанием данной переменной.

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<span class="caption">код 17-13: реализация метода `add_text` для добавления текста
в `content`</span>

Функция `add_text` получает изменяемую ссылку на `self`, т.е. мы можем изменять
экземпляр `Post`. Внутри мы вызываем метод  `push_str` `String`. Этот функционал
не является составной частью шаблона влияющего на поведение.

### Содержание черновика пустое

После того, как мы вызвали метод `add_text` и добавили содержание в наше сообщение,
мы всёже хотим чтобы метод `content` возвращал пустую строку. Реализуем этот метод:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn content(&self) -> &str {
        ""
    }
}
```

<span class="caption">код 17-14: добавление реализации метода  `content`, который
всегда возвращает пустое значение</span>

Пока всё до стоки 8 кода 17-11 реализовано.

### запрос проверки сделанных изменений

Далее запросим проверку содержания сообщения, для того, чтобы изменить состояние
с `Draft` на `PendingReview`. Для этого мы хотим, чтобы `Post` имел открытый метод
`request_review`, который получает изменяемую ссылку на `self`. Далее мы вызываем
закрытый метод `request_review` у состояния и этот метод вернёт новое состояние.
Для того, чтобы удалить старое состояние, метод `request_review` должен получить
владение значением состояния. Для этого нам понадобиться перечисление `Option`.
Мы получим значение из `Some` и вернём `None`. Далее мы установим значение в поле
`state`:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<span class="caption">код 17-15: реализация методов  `request_review` в типажах
`Post` и `State`</span>

Мы добавили метод `request_review` в типаж `State`. Теперь всем реализации, типажа
должны реализовать метод `request_review`.
Обратите внимение, что вместо `self`, `&self` или  `&mut self` первым параметром
метода мы имеем `self: Box<Self>`. Это означает, что метод считается действительным
(правильным) если вызывается из `Box` содержащего данный тип. Также мы видим,
что происходит получение владения `Box<Self>`, т.к. нам необходимо изменить состояние.

Также мы видим реализацию метода `request_review` в структуре `Draft`. Метод
возвращает экземпляр структуры `PendingReview`, которая является типом данных
реализовавшим `State`.

Мы уже видим удобство использование данного шаблона. Неважно, какое состояние
сейчас у сообщения. У каждого экземпляра состояния можно вызвать метод `request_review`.

Мы реализовали весь функционал до 11 строки кода 17-11!

### Улучшим код изменение сообщения при изменение состояния

Метод `approve` в  `Post` похож на `request_review` - он устанавливает состояние
на подтвержденное. Нам надо добавить метод `approve` в типаж `State`:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         Box::new(PendingReview {})
#     }
#
    // ...snip...
    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
#     fn request_review(self: Box<Self>) -> Box<State> {
#         self
#     }
#
    // ...snip...
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}
```

<span class="caption">код 17-16: реализация метода `approve` типажей `Post` и `State`
</span>


Внесём изменение в метод `content`: мы хотим получать содержание, если состояние
 `Published` иначе возвращаете пустую строку:

<span class="filename">Filename: src/lib.rs</span>

```rust
# trait State {
#     fn content<'a>(&self, post: &'a Post) -> &'a str;
# }
# pub struct Post {
#     state: Option<Box<State>>,
#     content: String,
# }
#
impl Post {
    // ...snip...
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    // ...snip...
}
```

<span class="caption">код 17-17: обновление метода `content`</span>

Мы взывали метод `as_ref`, т.к. хотим получить ссылку на значение внутри. Далее,
мы вызываем метод `unwrap`.

Добавим реализацию по умолчанию в типаж `State`. В структуре `Published` мы
перезапишем метод `content`:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String
# }
trait State {
    // ...snip...
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// ...snip...
struct Published {}

impl State for Published {
    // ...snip...
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

<span class="caption">код 17-18: добавление метода `content` к типажу `State`
</span>

### Недостатки шаблона "Состояние"

Мы показали, что Rust может реализовать ООП шаблон "Состояние" для того чтобы хранить
различные поведения, которые публикация имеет в зависимости от внутреннего состояния.
Методы структуры `Post` ничего не знают о различных вариантах поведения. Вся логика
шаблон сосредоточена в типаже  `State` и его реализациях.

Могут быть альтернативные решения. Например, выражение `match` в методе структуры
`Post` выбирает нужное поведение. Или даже в коде, который использует экземпляр
`Post` (`main`) проверяет состояние статьи и изменяет поведение. Чем больше будет
состояний, тем сложнее будет код.

#### Упрощаем решение

Мы собираемся показать, как немного переосмыслить шаблон состояния, чтобы получить
другое, более ясное решение.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
       &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

<span class="caption">код 17-19: `Post` с методом `content` `DraftPost` без метода
`content`</span>

#### Реализация превращений в другие типы

Продолжим доработку нашего нового решения:

<span class="filename">Filename: src/lib.rs</span>

```rust
# pub struct Post {
#     content: String,
# }
#
# pub struct DraftPost {
#     content: String,
# }
#
impl DraftPost {
    // ...snip...

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

<span class="caption">код 17-20:  `PendingReviewPost` создаётся с помощью метода
`request_review` в `DraftPost`. Метод `approve` создаёт `Post`</span>

Продолжаем улучшать наш код.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

<span class="caption">код 17-21: изменения в `main` для использования новой
версии кода библиотеки</span>

Rust предлагает более простые решения проблемы, которые возникают при проектировании
шаблонных решений.

## Итоги

Не важно, что вы думаете о объектно-ориентированных возможностях Rust. После прочтения
данной главы, вы теперь выдите, что с помощью объектов-типажей мы можем реализовать
опции, которые предоставляет ООП. Динамическое связывание может дать гибкость
в коде. ООП шаблон не всегда является лучшим решением, а является только лишь
иллюстрацией его возможностей.

Далее мы рассмотрим другую опцию Rust - шаблоны. Рассмотрим этот функционала подробнее.
