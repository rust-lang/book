## 이름 가져오기 (Importing Names)

우리는 Listing 7-6에서 보시는 것과 같이 `nested_modules` 함수를 호출하는 것처럼,
모듈 이름을 호출 구문의 일부분으로 사용하여 해당 모듈 내에 정의된 함수를 호출하는
방법을 다룬바 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

<span class="caption">Listing 7-6: 함수에 인접한 모듈 경로를 완전히 특정한 함수
호출하기</span>

보시다시피 완전하게 경로를 지정한 이름을 참조하는 것은 너무 길어질 수 있습니다.
다행히도 러스트는 이러한 호출을 더 간결하게 만들어주는 키워드를 가지고 있습니다.

### `use`를 이용한 간결한 가져오기

러스트의 `use` 키워드는 여러분이 스코프 내에서 호출하고 싶어하는 함수의 모듈을
가져옴으로써 긴 함수 호출을 줄여줍니다. `a::series::of` 모듈을 바이너리 크레이트의
루트 스코프로 가져온 예제입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

`use a::series::of;` 줄은 `of` 모듈을 참조하고 싶은 곳마다 `a::series::of` 전부를
사용하기 보다는 `of`를 사용할 수 있다는 뜻입니다.


`use` 키워드는 우리가 명시한 것만 스코프 내로 가져옵니다: 즉 모듈의 자식들을
스코프 내로 가져오지는 않습니다. 이는 `nested_modules` 함수를 호출하고자 할 떄
여전히 `of::nested_modules`를 사용해야 하는 이유입니다.

다음과 같이 `use` 구문 안에서 모듈 대신 함수를 명시하여 스코프 내에서 함수를 가져올
수도 있습니다:

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

이렇게 하면 모든 모듈을 안 써주고 함수를 직접 참조하도록 해줍니다.

열거형 또한 모듈과 비슷한 일종의 이름공간을 형성하고 있기 때문에, 열거형의 variant
또한 `use`를 이용하여 가져올 수 있습니다. 어떠한 `use` 구문이건 하나의 이름공간으로부터
여러 개의 아이템을 가져오려 한다면, 여러분은 아래와 같이 중괄호와 쉼표를 구문의 마지막
위치에 사용하여 이 아이템들을 나열할 수 있습니다:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```

`Green` variant에 대해서는 여전히 `TrafficLight` 이름공간을 명시하고 있는데,
이는 `use` 구문 내에 `Green`를 포함하지 않았기 때문입니다.

### `*`를 이용한 모두(glob) 가져오기

이름공간 내의 모든 아이템을 가져오기 위해서는 `*` 문법을 이용할 수 있습니다. 예를 들면:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

`*`는 *글롭(glob)* 이라고 부르며, 이는 이름공간 내에 공개된 모든 아이템을 가져올
것입니다. 여러분은 글롭을 아껴가며 써야 합니다: 글롭은 편리하지만, 여러분이 예상한
것보다 더 많은 아이템을 끌어와서 이름 간의 충돌(naming conflict)의 원인이 될수도
있습니다.

### `super`를 사용하여 부모 모듈에 접근하기

이 장의 시작 부분에서 보셨듯이, 여러분이 라이브러리 크레이트를 만들때, 카고는
여러분들을 위해 `tests` 모듈을 만들어줍니다. 지금부터 이에 대한 구체적인 부분들을
봅시다. 여러분의 `communicator` 프로젝트 내에 있는 *src/lib.rs*을 여세요:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

11장에서 테스트에 관한 더 많은걸 설명하고 있습니다만, 이 예제는 지금도 이해가
되시리라 봅니다: `tests`라는 이름의 모듈이 우리의 다른 모듈들 옆에 있고,
`it_works`라는 이름의 함수 하나를 담고 있지요. 좀 특별한 주석(annotation)이 있지만,
`tests` 모듈은 그냥 또다른 모듈일 뿐입니다! 따라서 우리의 모듈 계층 구조는 아래와
같이 생겼습니다:

```text
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

테스트는 우리 라이브러리 내에 있는 코드를 연습하기 위한 것이므로, 현재로서는 어떠한
기능도 확인할 게 없긴 하지만, `it_works` 함수 안에서 우리의 `client::connect` 함수를
호출해 봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

`cargo test` 명령을 써서 테스트를 실행하면:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `client`
```

컴파일이 실패했습니다, 하지만 대체 왜일까요? 우리는 *src/main.rs*에서 했었던 것과
마찬가지로 함수 앞에 `communicator::`를 붙일 필요가 없는데, 왜냐하면 이 코드가
분명히 `communicator` 라이브러리 크레이트 안에 있기 때문입니다. 원인은 경로가
항상 현재 모듈을 기준으로 상대적인데, 여기는 `test`이기 때문입니다. 딱 하나의
예외는 `use` 구문인데, 이는 기본적으로 크레이트 루트에 대한 상대적인 경로로
인식됩니다. 우리의 `tests` 모듈은 이 스코프 내에서 `client` 모듈이 필요합니다!

그러면 어떻게 모듈 계층 구조 내에서 한 모듈 위로 거슬러 올라가 `tests` 모듈 안에서
`client::connect` 함수를 호출할 수 있을까요? 아래와 같이 앞에 콜론 두개를 사용하여
러스트에게 우리가 루트부터 시작하여 전체 경로를 나열하겠다고 알려주는 방법이 있습니다:

```rust,ignore
::client::connect();
```

혹은, 아래와 같이 `super`를 사용하여 계층 구조 상에서 현재 모듈로부터 한 모듈 거슬러
올라갈 수도 있습니다:

```rust,ignore
super::client::connect();
```

이 두 가지 옵션은 이번 예제에서는 차이가 없는 것처럼 보이지만, 여러분의 모듈 계층
구조가 깊어진다면, 매번 루트에서부터 경로를 시작하는 것은 여러분의 코드를 길게
만들 것입니다. 그런 경우에는 `super`를 이용하여 현재 모듈에서 형제 모듈을 가져오는
것이 좋은 지름길이 됩니다. 여기에 더해서, 만약 여러분이 여러 군데에 루트로부터
시작되는 경로를 명시한 뒤에 서브트리를 다른 곳으로 옮기는 식으로 여러분의 모듈을
재정리한다면, 여러분은 여러 군데의 경로를 갱신하도록 요구받는 처지가 될 것이고,
이는 지루한 작업이 될 것입니다.

각각의 테스트에 `super::`를 타이핑해야 하는 것이 짜증날수 있겠지만, 여러분은
이미 여기에 대한 해답이 될 도구를 보셨습니다: `use` 말이죠! `super::`의 기능은
`use`에 제공한 경로를 변경시켜서 이제 루트 모듈 대신 부모 모듈에 상대적인 경로가
되게 해줍니다.

이러한 이유로, 특히 `tests` 모듈 내에서는 보통 `use super::something`이 가장 좋은
해결책이 됩니다. 따라서 이제 우리의 테스트는 이렇게 됩니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

`cargo test`를 다시 실행시키면, 테스트가 통과되고 테스트 결과 출력의 첫번째 부분이
아래와 같이 나타날 것입니다:

```text
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## 정리

이제 여러분은 코드를 조직화하기 위한 몇가지 새로운 기술을 알게 되었습니다! 관련된
기능들을 함께 묶여주는 이 기술들을 사용하고, 파일들이 너무 길어지지 않게 하고,
여러분의 라이브러리 사용자들에게 깔끔한 공개 API를 제공해 보세요.

다음으로 여러분의 멋지고 깔끔한 코드에 사용할 수 있는 표준 라이브러리 내의 몇가지
컬렉션 데이터 구조를 보겠습니다!
