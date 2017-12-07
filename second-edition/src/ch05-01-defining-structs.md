## Определение и инициализация структур

Внешне структуры похожи на кортежи. Также как кортежи, структуры могут содержать
разные типы данных. Но в отличии от кортежей, все данные должны быть именованными.
Поэтому структуры более удобные для создания новых типов данных, т.к. нет необходимости
запоминать порядковый номер какого-либо значения внутри экземпляра структуры.

Для определения структуры, необходимо указать ключевое слово и `struct` имя.
Имя должно описывать содержание. Далее, в фигурных скобках, через запятую, вписывается
именованный состав данного типа данных. Каждый элемент, *поле*, имеет тип данных.
Пример 5-1, описывает структуру для хранения информации о учётной записи пользователя:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Пример 5-1: определение структуры `User`</span>

После определения структуры можно создавать её *экземпляры*. Для этого каждому полю
определяется конкретное значение, соответствующее типу данных. Мы создаём экземпляр
указывая его имя и далее, в фигурных скобках, вписываем вместо типа данных конкретные
данные. Нет необходимости чётко следовать порядку следования полей (но всё-таки
желательно, для удобства чтения). Структура - это шаблон, а экземпляр - это шаблон
с данными. Пример 5-2:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
let user1 = User {
   email: String::from("someone@example.com"),
   username: String::from("someusername123"),
   active: true,
   sign_in_count: 1,
};

}
```

<span class="caption">Пример 5-2: создание экземпляра структуры `User`</span>

Чтобы получить какое-нибудь значения поля структуры, мы можем использовать
точечную нотацию (как в кортеже). Например: `user1.email`. Для изменения значения
данных поля структуры (если оно изменяемое), мы просто присваиваем ему новое значение.
Пример 5-3:
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!(
        "[{};{};{};{}]",
        user1.username,
        user1.email,
        user1.active,
        user1.sign_in_count
    );
}

```

<span class="caption">Пример 5-3: изменение значения поля `email` структуры `User`</span>

Как в любом выражении, мы можем вернуть экземпляр структуры из функции.
Пример 5-4:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
  let mut user1 = build_user(String::from("someone@example.com"),String::from("someusername123"));
  user1.email = String::from("anotheremail@example.com");
  println!("[{};{};{};{}]", user1.username,user1.email,user1.active,user1.sign_in_count);

}
```

<span class="caption">Пример 5-4: Функция `build_user` имеющая две входные переменные</span>

Если имя переменной функции и поля структуры повторяется, то можно не писать повторяющиеся
наименования:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
  let mut user1 = build_user(String::from("someone@example.com"),String::from("someusername123"));
  user1.email = String::from("anotheremail@example.com");
  println!("[{};{};{};{}]", user1.username,user1.email,user1.active,user1.sign_in_count);

}

```

### Создание экземпляра структуры из экземпляра другой структуры

Часто бывает удобно создавать новый экземпляр на основе старого. Пример 5-6 показывает
пример создания нового экземпляра на основе старого:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
  let mut user1 = build_user(String::from("someone@example.com"),String::from("someusername123"));
  user1.email = String::from("anotheremail@example.com");
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

  println!("[{};{};{};{}]", user1.username,user1.email,user1.active,user1.sign_in_count);
  println!("[{};{};{};{}]", user2.username,user2.email,user2.active,user2.sign_in_count);

}
```

<span class="caption">Пример 5-6: Создание экземпляра `User` `user2` и
присвоение полям значений `user1`</span>

Очень интересный вариант установки значений из другого экземпляра. Обновление использует
синтаксис `..` для передачи данных из полей, которые не были установлены явно:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
  let mut user1 = build_user(String::from("someone@example.com"),String::from("someusername123"));
  user1.email = String::from("anotheremail@example.com");
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};

  println!("[{};{};{};{}]", user1.username,user1.email,user1.active,user1.sign_in_count);
  println!("[{};{};{};{}]", user2.username,user2.email,user2.active,user2.sign_in_count);

}
```

<span class="caption">Пример 5-7: Использование сокращенного синтаксиса</span>

### Сокращенное определение структур (как кортежи)

Мы также можем определять структуры, с помощью сокращенной записи, очень напоминающую
кортежи (такое определение называют *кортежными структурами*). При определении такого
вида имена полей не определяются.
Пример:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Обратите внимание, что переменные `black` и `origin` имеют разные типы данных.
Обращение к полям структур осуществляется с помощью `.` точечной нотации.

### Структуры без полей

Создателю новых типов доступно создание структур без полей. Такой объект бывает
полезен при более сложной работе, которая будет обсуждать в главе 10.

> ### Владение данными структуры
> При определении структуры `User` в примере 5-1 мы предпочли использовать тип `String`
> вместо `&str`. Это было осознанное решение, т.к. мы хотели чтобы экземпляры структур
> владели действительными данными во время своего существования в памяти.
>
> Конечно, возможно чтобы структуры сохраняли ссылки на данные это накладывает
> определённые ограничения, о которых мы поговорим в главе 10. Без учёта времени
> жизни - такой код не будет действительным:
>
> <span class="filename">Filename: src/main.rs</span>
>
> ```rust,ignore
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> Ошибка. Необходимы определители времени жизни:
>
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> Мы расскажим, как исправить эту ошибку в главе 10. Для исправления этой ошибки
> с помощью имеющегося у Вас багажа знаний по Rust - используйте тип `String` вместо
> `&str`.

```rust

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!(
        "[{};{};{};{}]",
        user1.username,
        user1.email,
        user1.active,
        user1.sign_in_count
    );
    println!(
        "[{};{};{};{}]",
        user2.username,
        user2.email,
        user2.active,
        user2.sign_in_count
    );
}


```
