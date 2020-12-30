## 구조체 정의 및 인스턴트화

구조체는 3장에서 배운 튜플과 비슷합니다.
튜플처럼 구조체의 구성 요소들은 각각 다른 타입이 될 수 있습니다.
그리고 여기에 더해서, 구조체는 각각의 구성 요소에 이름을 붙일 수 있습니다.
따라서 각 요소가 더 명확한 의미를 갖게 되고, 특정 요소에 접근할 때
순서에 의존할 필요도 사라집니다. 결론적으로, 튜플보다 유연하게 사용할 수 있습니다.

구조체를 정의할 땐 `struct` 키워드와 해당 구조체에 지어줄 이름을 입력하면 됩니다.
이때 구조체 이름은 함께 묶을 데이터의 의미에 맞도록 지어주세요.
이후 중괄호 안에서는 필드(*field*)라 하는
각 구성 요소의 이름 및 타입을 정의합니다.
다음 Listing 5-1은 사용자 계정 정보를 저장하는 구조체입니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Listing 5-1: 사용자 계정 정보를 저장하는 `User` 구조체 정의</span>

정의한 구조체를 사용하려면 해당 구조체 내
각 필드의 값을 정해 인스턴스(*instance*)를 생성해야 합니다.
인스턴스를 생성하려면 먼저 구조체의 이름을 적고, 중괄호를 열고,
그 안에 필드의 이름(key)와 해당 필드에 저장할 값을 `key:value` 형태로 추가 해야합니다.
이때 필드의 순서는 구조체를 정의했을 때와 동일하지 않아도 됩니다.
요약하자면, 구조체 정의는 대충 해당 구조체에 무엇이 들어갈지를 정해둔 양식이며,
인스턴스는 거기에 실제 값을 넣은 것이라고 생각하시면 됩니다.
예시로 확인해 보죠. Listing 5-2 에서 앞서 정의한 `User` 구조체로
사용자를 선언해보겠습니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Listing 5-2: `User` 구조체의
인스턴스 생성</span>

구조체 내 특정 값은 점(.) 표기법으로 얻어올 수 있습니다.
사용자의 이메일 주소를 얻어야 한다치면 `user1.email` 처럼 사용할 수 있죠.
변경 가능한 인스턴스라면, 같은 방식으로 특정 필드의 값을 변경할 수도 있습니다.
다음 Listing 5-3 이 변경 가능한 인스턴스의
`email` 필드 값을 변경하는 예시입니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Listing 5-3: `User` 인스턴스의
`email` 필드 값 변경</span>

가변성은 해당 인스턴스 전체가 지니게 됩니다.
일부 필드만 변경 가능하도록 만들 수는 없으니, 기억해두시기 바랍니다.
또한, 구조체도 다른 표현식과 마찬가지로 함수 마지막 표현식에서
암묵적으로 새 인스턴스를 생성하고 반환할 수 있습니다.

Listing 5-4 에선 `build_user` 함수가 사용자 이메일과 이름을 전달 받고,
`acitve`, `sign_in_count` 를 각각 `true`, `1` 로 설정한
`User` 인스턴스를 반환하는 모습을 보여줍니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Listing 5-4: 사용자의 이메일과 이름을 전달 받고
`User` 인스턴스를 반환하는 `build_user` 함수</span>

특별히 나쁜 부분은 없지만, 매개변수명과 구조체 필드명이
`email`, `username` 으로 동일한데 굳이 반복해서 작성하는 건 귀찮은 감이 있군요.
구조체의 필드 개수가 많아지면 많아질수록 이런 귀찮음은 커질 겁니다.
한번 축약법을 사용해볼까요?

## 변수명과 필드명이 같을 때 간단하게 필드 초기화하기

Listing 5-4 처럼 변수명과 구조체 필드명이 같을 땐,
필드 초기화 축약법(*field init shorthand*)을 사용해서 더 적은 타이핑으로
같은 기능을 구현할 수 있습니다. 다음 Listing 5-5 는 `email`, `username` 을
반복 작성하는 대신 필드 초기화 축약법을 사용한 예제입니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Listing 5-5: 변수명과 필드명이 같던
`email`, `username` 에 필드 초기화 축약법을 적용한
`build_user` 함수</span>

이번에는 `build_user` 함수에서
`User` 구조체의 인스턴스를 생성할 때
`email: email` 처럼 작성하는 대신,
변수명과 필드명이 같다는 점을 이용해 `email` 로만 작성한 모습입니다.
물론, 함수는 이전과 같이 잘 작동합니다.

### 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 갱신법 사용하기

기존에 있던 인스턴스에서 대부분의 값을 유지한 채로 몇몇 값만 바꿔 새로운 인스턴스를 생성하게 되는 경우가 간혹 있습니다.
그럴 때 유용한 게 바로 구조체 갱신법(*struct update syntax*)입니다.

먼저 구조체 갱신법을 사용하지 않았을 때를 살펴봅시다.
Listing 5-6 은 Listing 5-2 에서 만든 `user1` 을 구조체 갱신법 없이
`email` 과 `username` 만 새로운 값으로 변경해 `user2` 를 생성하는 코드입니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Listing 5-6: `user1` 의 일부 값을 이용해
새로운 `User` 인스턴스를 생성</span>

이번엔 구조체 갱신법을 사용해볼까요?
다음 Listing 5-7 처럼, 구조체 갱신법은 더 적은 코드로 같은 효과를 낼 수 있습니다.
`..` 은 따로 명시된 필드를 제외한 나머지 필드를, 주어진 인스턴스의 필드 값으로 설정하는 구문입니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Listing 5-7: 새로운 `email`, `username` 값으로
`User` 구조체의 인스턴스를 생성하되, 나머지 필드는
구조체 갱신법을 이용해 `user1` 의 필드 값을 사용하기</span>

Listing 5-7 은 `user2` 인스턴스를 생성할때
`active`, `sign_in_count` 는 `user1` 의 필드와 같은 값을 갖고
`email`, `username` 은 다른 값을 갖도록 하는 코드입니다.

### 필드명이 없는, 타입 구분용 튜플 구조체

구조체를 사용해 튜플과 유사한 형태의 튜플 구조체(*tuple structs*)를
정의할 수도 있습니다. 튜플 구조체는 필드의 이름을 붙이지 않고
필드 타입 만을 정의하며, 구조체 명으로 의미를 갖는 구조체입니다.
이는 튜플 전체에 이름을 지어주거나 특정 튜플을 다른 튜플과 구분 짓고 싶은데,
그렇다고 각 필드명을 일일이 정해 일반적인 구조체를 만드는 것은
배보다 배꼽이 더 큰 격이 될 수 있을 때 유용합니다.

튜플 구조체의 정의는 일반적인 구조체처럼 `struct` 키워드와 구조체 명으로 시작되나,
그 뒤에는 타입들로 이루어진 튜플이 따라옵니다. 예시로 살펴볼까요?
다음은 각각 `Color`, `Point` 라는 두 개의 튜플 구조체 정의 및 사용 예시입니다.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs:here}}
```

`black`, `origin` 이 서로 다른 튜플 구조체의 인스턴스이므로,
타입이 서로 달라진다는 점이 중요합니다.
구조체 내 필드 구성이 같더라도 각각의 구조체는 별도의 타입이기 때문이죠.
즉, `Color` 타입과 `Point` 타입은 둘 다 `i32` 값 3 개로 이루어진 타입이지만,
`Color` 타입을 매개변수로 받는 함수에
`Point` 타입을 인자로 넘겨주는 건 불가능합니다.
앞서 말한 점을 제외하면 튜플처럼 사용할 수 있습니다.
여러 부분으로 해체할 수도 있고, `.` 과 색인으로 요소에 접근할 수도 있죠.

### 필드가 없는 유사 유닛 구조체

필드가 아예 없는 구조체를 정의할 수도 있습니다.
이는 유닛 타입인 `()` 과 비슷하게 동작하므로
유사 유닛 구조체(*unit-like structs*) 라 지칭하며,
어떤 타입을 내부 데이터 저장 없이 10장에서 배울 트레잇을
구현하기만 하는 용도로 사용할 때 주로 활용됩니다.

> ### 구조체 데이터의 소유권
>
> Listing 5-1 의 `User` 구조체 정의에서는 의도적으로
> `&str` 문자열 슬라이스 대신 구조체가 소유권을 갖는 `String` 타입을 사용했습니다.
> 구조체 인스턴스가 유효한 동안 인스턴스 내의
> 모든 데이터가 유효하도록 만들기 위해서죠.
>
> 참조자를 이용해 구조체가 소유권을 갖지 않는 데이터도 저장할 수는 있지만,
> 이는 10장에서 배울 라이프타임(*lifetime*)을 활용해야 합니다.
> 라이프타임을 사용하면 구조체가 존재하는 동안에
> 구조체 내 참조자가 가리키는 데이터의 유효함을 보장받을 수 있기 때문이죠.
> 만약 라이프타임을 명시하지 않고 참조자를 저장하고자 하면 다음처럼 문제가 발생합니다.
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
> 
> ```rust,ignore,does_not_compile
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
> 라이프타임이 명시돼야 한다며 컴파일러가 에러를 일으킬 겁니다.
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:2:15
>   |
> 2 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 | struct User<'a> {
> 2 |     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:12
>   |
> 3 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 | struct User<'a> {
> 2 |     username: &str,
> 3 |     email: &'a str,
>   |
>
> error: aborting due to 2 previous errors
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs`
>
> To learn more, run the command again with --verbose.
> ```
>
> 위 에러를 해결하여 구조체에 참조자를 저장하는 방법은 10장에서 알아볼 겁니다.
> 지금 당장은 `&str` 대신 `String` 을 사용함으로써
> 넘어가도록 하죠.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->
