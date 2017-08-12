## 구조체를 정의하고 초기화하기

구조체는 3장에서 학습한 튜플과 비슷합니다. 튜플과 유사하게, 구조체의 구성요소들은 각자 다른 타입을
지닐 수 있습니다. 그러나 튜플과는 다르게 각 구성요소들은 명명할 수 있어 값이 의미하는 바를 명확하게
인지할 수 있습니다. 구조체는 각 구성요소들에 명명을 할 수 있다는 점 덕분에 튜플보다 유연하게 다룰 수
있습니다. 구조체 내의 특정 요소 데이터 명세를 기술하거나, 접근할 때 순서에 의존할 필요가 없기 때문입니다.

구조체를 정의할 때는 `struct` 키워드를 먼저 입력하고 명명할 구조체명을 입력하면 됩니다. 구조체의 이름은
함께 묶이게 되는 구성요소들의 의미를 내포할 수 있도록 짓는 것이 좋습니다.
이후 중괄호 안에서는, 필드(*field*)라 불리는 각 구성요소들의 타입과 접근할 수 있는 이름을 정의합니다.

아래 예제 5-1에서는 사용자 계정에 대한 정보를 저장하는 구조체를 정의합니다.


```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Listing 5-1: 사용자 계정정보를 저장하는 `User` 구조체 정의</span>

정의한 구조체를 사용하려면, 각 필드의 값을 명세한 인스턴스(*instance*)를 생성해야 합니다.
인스턴스는 구조체의 이름을 명시함으로써 사용할 수 있고, 필드를 식별할 수 있는 이름인 키와
그 키에 저장하고자 하는 값의 쌍(`key:value`)을 이어지는 중괄호 안에 추가하여 생성할 수 있습니다.

구조체를 정의할때 필드들의 순서가 정의한 필드의 순서와 같을 필요는 없습니다. 달리 서술하자면, 구조체
정의는 무엇이 들어가야 하는 지 대략적으로 정의된 양식 정도라고 생각하시면 되고, 인스턴스는 그것에
특정한 값을 넣어 실체화한 것이라 생각하시면 됩니다. 아래 예제 5-2에서는 특정 사용자를 선언하는
과정을 보여줍니다.


```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

<span class="caption">Listing 5-2: 구조체 `User`의 인스턴스 생성하기</span>

구조체에서 특정한 값을 읽어오려면, 점(.) 표기법을 사용하시면 됩니다. 사용자의 이메일 값을 얻고자 하면,
`user1.email` 과 같은 방식으로 접근하실 수 있습니다. 변경이 가능한 구조체에 들어있는 값을 바꾸고자
할 때는, 아래와 같이 점(.) 표기법을 사용하여 새 값을 할당할 수 있습니다.
`user1.email = String::from("someone-else@example.com");`


### 변수명이 필드명과 같을 때 간단하게 필드 초기화하기

변수명과 구조체의 필드명이 같다면, 필드 초기화 축약법(*field init shorthand*) 을 이용할 수 있습니다. 
이를 활용하면 구조체를 생성하는 함수를 더 간단히 작성할 수 있게 됩니다.
아래 예제 5-3의 `build_user` 함수에는 `email`과 `username` 라는 매개변수가
있습니다. 함수는 `User`구조체가 구현된 인스턴스를 반환합니다.


```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">예제 5-3: 사용자의 이메일과 이름을 받아 `User`구조체의 인스턴스를
반환하는 `build_user` 함수</span>


매개변수인 `email`과 `username`이 `User`구조체의 필드명과 같기 떄문에, 함수 `build_user`
에서 `email`과 `username`를 명시하는 부분을 예제 5-4와 같이 다시 작성할 필요가 없습니다.

예제 5-4의 `build_user` 함수는 예제 5-3과 같은 방식으로 동작합니다. 필드 초기화를 이러한 방식으로
수행하는 문법은 간결한 코드를 작성하는데 도움이 되고, 많은 필드의 값이 정의되어야할 때 특히 유용합니다.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">예제 5-4: 매개변수 `email`과 `username`가 구조체의 필드와 이름이
같아, 함수 내에서 특별히 명시하지 않고 초기화한 예인 `build_user` 함수</span>

### 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기

존재하는 인스턴스에서 기존 값의 대부분은 재사용하고, 몇몇 값만 바꿔 새로운 인스턴스를 정의하는 방법은
유용합니다. 예제 5-5는 변수 `user2`에 `email`과 `username`은 새로 할당하고, 나머지
필드들은 예제 5-2에서 정의한 `user1`의 값들을 그대로 사용하는 방식으로 `User` 인스턴스를
생성하는 것을 보여줍니다.


```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

<span class="caption">예제 5-5: `user1`을 일부 값들을 재사용하여, 구조체 `User`의 인스턴스
`user2`를 새로 생성</span>

  구조체 갱신법(*struct update syntax*)은 예제 5-5에서 작성한 짧은 코드와 같은 효과를 낼 수
있습니다. 구조체 갱신법은, 입력으로 주어진 인스턴스와 변화하지 않는 필드들을 명시적으로 할당하지
않기 위해 `..` 구문을 사용합니다. 예제 5-6의 코드는 `user1` 인스턴스와 `active`,
`sign_in_count` 필드의 값은 같고, `email`과 `username` 필드들은 값은 다른 `user2`
인스턴스를 생성할 때 구조체 갱신법을 사용하는 것을 보여줍니다.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

<span class="caption">예제 5-6: 인스턴스 갱신 문법의 사용 예시 - 새 `User` 구조체 생성 시
`email`과 `username` 필드에는 새 값을 할당하고, 나머지 필드는 `user1`에서 재사용</span>

### 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체

구조체명을 통해 의미를 부여할 수 있으나 필드의 타입만 정의할 수 있고 명명은 할 수 없는,
튜플 구조체(*tuple structs*)라 불리는 튜플과 유사한 형태의 구조체도 정의할 수 있습니다.

튜플 구조체는 일반적인 구조체 정의방법과 똑같이 `struct` 키워드를 통해 정의할 수 있고, 튜플의
타입 정의가 키워드 뒤에서 이루어지면 됩니다. 아래는 튜플 구조체인 `Color`, `Point`의 정의와
사용 예시 입니다.


```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
다른 튜플 구조체이기 때문에, `black`과 `origin`이 다른 타입이란 것을 유념해 두셔야 합니다.
구조체 내의 타입이 모두 동일하더라도 각각의 구조체는 고유의 타입이기 때문입니다. 한편 튜플 구조체
인스턴스는, 3장에서 살펴 본 튜플과 비슷하게 동작합니다.


### 필드가 없는 유사 유닛 구조체

또한 어떤 필드도 없는 구조체 역시 정의할 수 있습니다! 이는 유닛 타입인 `()`와 비슷하게 동작하고,
그 때문에 유사 유닛 구조체(*unit-like structs*)라 불립니다.
유사 유닛 구조체는 특정한 타입의 트레잇(trait)을 구현해야하지만 타입 자체에 데이터를 저장하지 않는
경우에 유용합니다. 트레잇(trait)에 대해서는 10장에서 더 살펴보도록 하겠습니다.



> ### 구조체 데이터의 소유권(Ownership)
>
> 예제 5-1에서의 `User` 구조체 정의에서는, `&str` 문자 슬라이스 타입 대신 `String`타입을
> 사용했습니다. 이는 의도적인 선택으로, 구조체 전체가 유효한 동안 구조체가 그 데이터를 소유하게 하고자
> 함입니다.
>
> 구조체가 소유권이 없는 데이터의 참조를 저장할수는 있지만, 10장에서 언급 될 라이프타임(*lifetimes*)
> 의 사용을 전제로 합니다.
> 라이프타임은 구조체가 존재하는동안 참조하는 데이터를 계속 존재할 수 있도록 합니다. 라이프타임을
> 사용하지 않고 참조를 저장하고자 하면 아래와 같은 일이 발생합니다.
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
> 컴파일러는 라이프타임이 명시되어야 한다고 에러를 발생시킵니다.
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
> 참조가 저장이 불가능한 위 에러 개선에 대해서는 10장에서 살펴보도록 하겠습니다. 지금은 `&str` 대신
> `String` 을 사용하는 방식으로 에러를 고치도록 하겠습니다.
