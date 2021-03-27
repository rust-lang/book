## 별개의 파일로 모듈 분리하기

이번 장에서 여태 나온 모든 예제들은 하나의 파일에 여러 모듈을 정의했습니다.
큰 모듈이라면, 정의를 여러 파일로 나누어 코드를 쉽게 찾아갈 수 있도록
만들어야 하겠죠.

Listing 7-17 코드로 시작해봅시다.
크레이트 루트 파일의 내용을 Listing 7-21 코드로 바꾸고,
`front_of_house` 모듈을 *src/front_of_house.rs* 라는 파일로 옮겨보죠.
이 경우 크레이트 루트 파일은 *src/lib.rs* 이지만,
크레이트 루트 파일이 *src/main.rs* 인 바이너리 크레이트에서도 과정은 동일합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listing 7-21: 본문이 *src/front_of_house.rs* 에 위치할
`front_of_house` 모듈 선언</span>

*src/front_of_house.rs* 파일에는 Listing 7-22 처럼
`front_of_house` 모듈 정의의 본문을 작성합니다.

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Listing 7-22: *src/front_of_house.rs* 파일에
`front_of_house` 모듈 본문 정의</span>

`mod front_of_house` 뒤에 블록이 아닌 세미콜론을 작성하면,
러스트는 해당 모듈과 동일한 이름의 파일에서 모듈 내용을 로드합니다.
이어서 `hosting` 모듈을 별개의 파일로 분리해보죠.
*src/front_of_house.rs* 파일에 `hosting` 모듈 선언만 남도록
수정합니다.

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

*src/front_of_house* 디렉토리와
*src/front_of_house/hosting.rs* 파일을 생성하고,
`hosting` 모듈 내용을 작성합니다.

<span class="filename">Filename: src/front_of_house/hosting.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

각종 정의를 다른 파일로 이동했지만, 모듈 트리는 이전과 동일합니다.
`eat_at_restaurant` 함수 내의 여러 함수 호출도 그대로 작동합니다.
거대한 모듈을 파일 하나에 전부 작성하지 않고, 필요에 따라 새로운 파일을 만들어
분리할 수 있도록 하는 것이 모듈 분리 기법입니다.

*src/lib.rs* 파일의 `pub use crate::front_of_house::hosting` 구문을 변경하지 않았으며,
`use` 문이 크레이트의 일부로 컴파일 되는 파일에 영향을 주지 않는다는 점도 주목해 주세요.
`mod` 키워드는 모듈을 선언하고,
러스트는 해당 모듈까지의 코드를 찾아서
모듈명과 동일한 이름의 파일을 찾습니다.

## 정리

러스트에서는 패키지를 여러 크레이트로 나눌 수 있고, 크레이트는 여러 모듈로 나눌 수 있습니다.
절대 경로나 상대 경로를 작성하여 어떤 모듈 내 항목을 다른 모듈에서 참조할 수 있습니다.
경로는 `use` 구문을 사용해 스코프 내로 가져와,
항목을 해당 스코프에서 여러 번 사용해야 할 때 더 짧은 경로를 사용할 수 있습니다.
모듈 코드는 기본적으로 비공개이지만,
`pub` 키워드를 추가해 정의를 공개할 수 있습니다.

다음 장에서는 여러분의 깔끔하게 구성된 코드에서 사용할 수 있는
표준 라이브러리의 컬렉션 자료구조를 몇 가지 배워보겠습니다.
