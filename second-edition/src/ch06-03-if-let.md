## `if let`을 사용한 간결한 흐름 제어

`if let` 문법은 `if`와 `let`을 조합하여 하나의 패턴만 매칭 시키고 나머지 경우는 무시하는 값을
다루는 덜 수다스러운 방법을 제공합니다. 어떤 `Option<u8>` 값을 매칭 하지만 그 값이 3일 경우에만
코드를 실행시키고 싶어 하는 Listing 6-6에서의 프로그램을 고려해 보세요:

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

<span class="caption">Listing 6-6: 어떤 값이 `Some(3)` 일 때에만 코드를 실행하도록
하는 `match`</span>

우리는 `Some(3)`에 매칭 되는 경우에만 뭔가를 하지만 다른 `Some<u8>` 값 혹은 `None` 값인 경우에는
아무것도 하지 않고 싶습니다. 이러한 `match` 표현식을 만족시키기 위해, `_ => ()`을 단 하나의
variant를 처리한 다음에 추가해야 하는데, 이는 추가하기에 너무 많은 보일러 플레이트 코드입니다.

그 대신, `if let`을 이용하여 이 코드를 더 짧게 쓸 수 있습니다. 아래의 코드는 Listing 6-6에서의
`match`와 동일하게 동작합니다:

```rust
# let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

`if let`은 `=`로 구분된 패턴과 표현식을 입력받습니다. 이는 `match`와 동일한 방식으로 작동하는데,
여기서 표현식은 `match`에 주어지는 것이고 패턴은 이 `match`의 첫 번째 갈래와 같습니다.

`if let`을 이용하는 것은 여러분이 덜 타이핑하고, 덜 들여 쓰기 하고, 보일러 플레이트 코드를 덜
쓰게 된다는 뜻입니다. 하지만, `match`가 강제했던 하나도 빠짐없는 검사를 잃게 되었습니다.
`match`와 `if let` 사이에서 선택하는 것은 여러분의 특정 상황에서 여러분이 하고 있는 것에 따라,
그리고 간결함을 얻는 것이 전수 조사를 잃는 것에 대한 적절한 거래인지에 따라 달린 문제입니다.

바꿔 말하면, 여러분은 `if let`를 어떤 값이 하나 패턴에 매칭 되었을 때 코드를 실행하고 다른 값들에
대해서는 무시하는 `match` 문을 위한 문법적 설탕(syntax sugar)으로 생각할 수 있습니다.

`if let`과 함께 `else`를 포함시킬 수 있습니다. `else` 뒤에 나오는 코드 블록은 `match` 표현식에서
`_` 케이스 뒤에 나오는 코드 블록과 동일합니다. Listing 6-4에서 `Quarter` variant가 `UsState`
값도 들고 있었던 `Coin` 열거형 정의부를 상기해 보세요. 만일 우리가 쿼터가 아닌 모든 동전을 세고
싶은 동시에 쿼터 동전일 경우 또한 알려주고 싶었다면, 아래와 같이 `match`문을 쓸 수 있었을 겁니다:


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

혹은 아래와 같이 `if let`과 `else` 표현식을 이용할 수도 있겠지요:

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
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

만일 여러분의 프로그램이 `match`로 표현하기에는 너무 수다스러운 로직을 가지고 있는 경우에 놓여 있다면,
여러분의 러스트 도구 상자에는 또한 `if let`이 있음을 기억하세요.

## 정리

지금까지 우리는 열거한 값들의 집합 중에서 하나가 될 수 있는 커스텀 타입을 만들기 위해서 열거형을
사용하는 방법을 다뤄보았습니다. 우리는 표준 라이브러리의 `Option<T>` 타입이 에러를 방지하기 위해
어떤 식으로 타입 시스템을 이용하도록 도움을 주는지 알아보았습니다. 열거형 값들이 내부에 데이터를
가지고 있을 때는, `match`나 `if let`을 이용하여 그 값들을 추출하고 사용할 수 있는데,
둘 중 어느 것을 이용할지는 여러분이 다루고 싶어 하는 경우가 얼마나 많은지에 따라 달라집니다.

여러분의 러스트 프로그램은 이제 구조체와 열거형을 이용해 여러분의 영역 내의 개념을 표현할 수 있습니다.
여러분의 API 내에서 사용할 커스텀 타입을 생성하는 것은 타입 안전성을 보장합니다: 컴파일러는
여러분의 특정 함수들이 예상하는 특정 타입의 값만 갖도록 만들어줄 것입니다.

사용하기 직관적이고 여러분의 사용자가 필요로 할 것만 정확히 노출된 잘 조직화된 API를 여러분의 사용들에게
제공하기 위해서, 이제 러스트의 모듈로 넘어갑시다.
