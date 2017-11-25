## `if let`

Конструкция `if let` позволяет комбинировать `if` и `let`, что позволяет присваивать
значение переменной, удовлетворяющее определённому шаблону. Пример 6-6:

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

<span class="caption">Пример 6-6. Будет напечатана строка, если some_u8_value будет
равна `Some(3)`</span>

Существует возможность написать более компактный код этого примера. Пример 6-6:

```rust
# let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

Выбор между использованием `match` и `if let` зависит от задачи. Выбор за вами.

`if let` является синтаксическим упрощением `match` в определённых обстоятельствах.

Мы также можем написать `if let ` c `else`. Это будет полностью похоже на `match`.
Пример:

```rust
# #[derive(Debug)]
# enum UsState {
#    Alabama,
#    Alaska,
# }
#
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter(UsState),
# }
# let coin = Coin::Penny;
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Этот же код можно переписать с использованием `if let` и `else`:

```rust

#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}

fn doit(coin:Coin){

  let mut count = 0;

  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }

  println!("{:?}", count);
}

fn main(){
  doit(Coin::Penny);
  doit(Coin::Nickel);
  doit(Coin::Dime);
  doit(Coin::Quarter(UsState::Alabama));
  doit(Coin::Quarter(UsState::Alaska));
}

```

Вам выбирать какая конструкция подходит для вашего кода лучше сего.

## Итоги

В этой главе мы рассмотрели, как использовать перечисление (создание, примеры использования).
Также на пример типа из стандартной библиотеки `Option<T>` выяснили, как
предотвратить ошибки в коде.  Изучили использование конструкций `match` и `if let`
для анализа и выборки данных из значений перечислений, а также некоторые возможные
улучшения и упрощения кода.

Теперь вы можете создавать программы и использовать возможности группировочных структур
Rust.

Для логической организации большого количества файлов кода весьма удобно использовать модули,
о которых мы поговорим далее.
