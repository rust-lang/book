## `mod`와 파일 시스템

먼저 카고를 이용해서 새로운 프로젝트를 만드는 것으로 모듈 예제를 시작하려고 하는데,
바이너리 크레이트(crate)을 만드는 대신에 라이브러리 크레이트을 만들 것입니다. 여기서
라이브러리 크레이트이란 다른 사람들이 자신들의 프로젝트에 디펜던시(dependency)로 추가할
수 있는 프로젝트를 말합니다. 예를 들어, 2장의 `rand` 크레이트은 우리가 추측 게임
프로젝트에서 디펜던시로 사용했던 라이브러리 크레이트입니다.

우리는 몇가지 일반적인 네트워크 기능을 제공하는 라이브러리의 뼈대를 만들 것입니다;
여기서는 모듈들과 함수들의 조직화에 집중할 것이고, 함수의 본체에 어떤 코드가 들어가야
하는지는 신경쓰지 않겠습니다. 이 라이브러리를 `communicator`라고 부르겠습니다.
기본적으로, 카고는 다른 타입의 프로젝트로 특정하지 않는 이상 라이브러리를 만들
것입니다: 이전의 모든 장들에서 사용해왔던 `--bin` 옵션을 제거하면, 프로젝트는
라이브러리가 될 것입니다:

```text
$ cargo new communicator
$ cd communicator
```

카고가 *src/main.rs* 대신 *src/lib.rs*을 생성했음을 주목하세요. *src/lib.rs*
내부를 보면 다음과 같은 코드를 찾을 수 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

카고는 우리가 만든 라이브러리의 작성 시작을 돕기 위해 빈 테스트를 만드는데,
이는 `--bin` 옵션을 사용했을때 “Hello, world!” 바이너리를 만들어준 것과 사뭇
다릅니다. `#[]`와 `mod tests` 문법은 이 장의 “`super`를 이용하여 부모 모듈에
접근하기”절에서 더 자세히 다룰 것이지만, 당장은 *src/lib.rs*의 아래쪽에 이 코드를
남겨두겠습니다.

*src/main.rs* 파일이 없기 떄문에, `cargo run` 커맨드로 카고가 실행할 것이 없습니다.
따라서, 여기서는 라이브러리 크레이트의 코드를 컴파일하기 위해 `cargo build`를 사용할
것입니다.

이제 여러분이 작성하는 코드의 의도에 따라 만들어지는 다양한 상황에 알맞도록 라이브러리
코드를 조직화하는 다양한 옵션들을 살펴보겠습니다.

### 모듈 정의

우리의 `communicator` 네트워크 라이브러리를 위해서, 먼저 `connect`라는 이름의 함수가
정의되어 있는 `network`라는 이름의 모듈을 정의하겠습니다. 러스트 내 모듈 정의는 모두
`mod`로 시작됩니다. 이 코드를 *src/lib.rs*의 시작 부분, 즉 테스트 코드의 윗 쪽에
추가해봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}
```

`mod` 키워드 뒤에, 모듈의 이름 `network`가 쓰여지고 중괄호 안에 코드 블록이 옵니다.
이 블록 안의 모든 것은 이름공간 `network` 안에 있습니다. 위의 경우 `connect`라는
이름의 함수 하나가 있습니다. 이 함수를 `network` 모듈 바깥의 스크립트에서 호출하고자
한다면, 우리는 모듈을 특정할 필요가 있으므로 이름공간 문법 `::`를 이용해야 합니다:
`connect()` 이렇게만 하지 않고 `network::connect()` 이런 식으로요.

또한 같은 *src/lib.rs* 파일 내에 여러 개의 모듈을 나란히 정의할 수도 있습니다.
예를 들어, `connect`라는 이름의 함수를 갖고 있는 `client` 모듈을 정의하려면,
Listing 7-1에 보시는 바와 같이 이를 추가할 수 있습니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```

<span class="caption">Listing 7-1: *src/lib.rs* 내에 나란히 정의된 `network`
모듈과 `client` 모듈</span>

이제 우리는 `network::connect` 함수와 `client::connect` 함수를 갖게 되었습니다.
이들은 완전히 다른 기능을 갖고 있을 수 있고, 서로 다른 모듈에 정의되어 있기 때문에
함수 이름이 서로 부딪힐 일은 없습니다.

이 경우, 우리가 라이브러리를 만드는 중이기 때문에, 라이브러리의 시작 지점으로서
제공되는 파일은 *src/lib.rs* 입니다. 하지만 모듈을 만드는 것에 관하여
*src/lib.rs*는 특별할 것이 없습니다. 우리는 라이브러리 크레이트의 *src/lib.rs* 내에
모듈을 만드는 것과 똑같은 방식으로 바이너리 크레이트의 *src/main.rs* 내에도 모듈을
만들 수 있습니다. 사실 모듈 안에 다른 모듈을 집어넣는 것도 가능한데, 이는 여러분의
모듈이 커짐에 따라 관련된 기능이 잘 조직화 되도록 하는 한편 각각의 기능을 잘 나누도록
하는데 유용할 수 있습니다. 여러분의 코드를 어떻게 조직화 할 것인가에 대한 선택은
여러분이 코드의 각 부분 간의 관계에 대해 어떻게 생각하고 있는지에 따라 달라집니다.
예를 들어, Listing 7-2와 같이 `client` 모듈과 `connect` 함수가 `network` 이름공간
내에 있다면 우리의 라이브러리 사용자가 더 쉽게 이해할지도 모릅니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

<span class="caption">Listing 7-2: `client` 모듈을 `network` 모듈 안으로 이동</span>

*src/lib.rs* 파일에서 Listing 7-2와 같이 `client` 모듈이 `network` 모듈의
내부 모듈이 되도록 `mod network`와 `mod client`의 위치를 바꿔 봅시다. 이제
우리는 `network::connect`와 `network::client::connect` 함수를 갖게 되었습니다:
다시 말하지만, `connect`라는 이름의 두 함수는 서로 다른 이름공간에 있으므로
부딪힐 일이 없습니다.

이런 식으로 모듈들은 계층을 구성하게 됩니다. *src/lib.rs*의 내용은 가장 위의 층을
이루고, 서브 모듈들은 그보다 낮은 층에 있습니다. Listing 7-1 예제에서의 조직화가
계층 구조를 생각했을 때 어떻게 보일지 살펴봅시다:

```text
communicator
 ├── network
 └── client
```

그리고 Listing 7-2 예제에 대응되는 계층 구조는 이렇습니다:

```text
communicator
 └── network
     └── client
```

Listing 7-2에서 계층 구조는 `client`가 `network`의 형제이기 보다는 자식임을
보여줍니다. 더 복잡한 프로젝트는 많은 수의 모듈을 갖고 있을 수 있고, 이들은 지속적인
트래킹을 위해 논리적으로 잘 조직화될 필요가 있을 것입니다. 여러분의 프로젝트 내에서
“논리적으로”가 의미하는 것은 여러분에게 달려 있는 것이며, 여러분과 여러분의 라이브러리
사용자들이 프로젝트 도메인에 대해 어떻게 생각하는지에 따라 달라집니다. 여러분이 선호하는
어떤 형태의 구조이건 간에 여기서 보여준 나란한 모듈 및 중첩된(nested) 모듈을 만드는
테크닉을 이용해 보세요.

### 모듈을 다른 파일로 옮기기

모듈은 계층적인 구조를 형성하는데, 여러분이 익숙하게 사용하고 있는 다른 구조와 매우 닮았습니다: 바로
파일 시스템이죠! 러스트에서는 프로젝트를 잘게 나누기 위해 여러 개의 파일 상에서 모듈 시스템을 사용할
수 있어, 모든 것들이 *src/lib.rs*나 *src/main.rs* 안에 존재하지 않게할 수 있습니다. 이러한
예를 위해서, Listing 7-3에 있는 코드를 시작해봅시다:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

<span class="caption">Listing 7-3: 세 개의 모듈 `client`, `network`, `network::server`가
모두 *src/lib.rs*에 정의되어 있음</span>

파일 *src/lib.rs*는 아래와 같은 모듈 계층을 갖고 있습니다:

```text
communicator
 ├── client
 └── network
     └── server
```

만일 이 모듈들이 여러 개의 함수들을 갖고 있고, 이 함수들이 길어지고 있다면, 우리가 작업하고자 하는
코드를 찾으려고 이 파일을 스크롤 하기가 까다로워질 것입니다. 함수들은 하나 혹은 그 이상의 `mod` 블록
안에 포함되어 있기 떄문에, 함수 내의 코드 라인들 또한 길어지기 시작할 것입니다. 이는 `client`,
`network`, 그리고 `server` 모듈을 *src/lib.rs*로부터 떼어내어 각자를 위한 파일들에 위치시키기
좋은 이유가 되겠습니다.

먼저 `client` 모듈의 코드를 `client` 모듈의 선언 부분만 남겨두는 것으로 바꾸세요. 그러니까 여러분의
*src/lib.rs*는 아래와 같이 될 것입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

여기서는 여전히 `client` 모듈을 *선언*하고 있지만, 코드 블록을 세미콜론으로 대체함으로써, 우리는
러스트에게 `client` 모듈의 스코프 내에 정의된 코드를 다른 위치에서 찾으라고 말하는 것입니다. 달리
말하면, `mod client;`라는 라인의 뜻은 이렇습니다:

```rust,ignore
mod client {
    // contents of client.rs
}
```

이제 모듈의 이름과 같은 이름을 가진 외부 파일을 만들 필요가 있습니다. *client.rs* 파일을 여러분의
*src/* 디렉토리에 생성하고 여세요. 그런 뒤 아래와 같이 앞 단계에서 제거했던 `client` 모듈내의
`connect` 함수를 입력해세요:

<span class="filename">Filename: src/client.rs</span>

```rust
fn connect() {
}
```

이미 *src/lib.rs* 안에다 `client` 모듈을 `mod`를 이용하여 선언을 했기 때문에,
이 파일 안에는 `mod` 선언이 필요없다는 점을 기억하세요. 이 파일은 단지 `client` 모듈의 *내용물*만
제공할 뿐입니다. 만일 `mod client`를 여기에 또 집어넣는다면, 이는 `client` 모듈 내에 서브모듈
`client`를 만들게 됩니다!

러스트는 기본적으로 *src/lib.rs*만 찾아볼줄 압니다. 만약에 더 많은 파일을 프로젝트에 추가하고
싶다면, *src/lib.rs* 내에서 다른 파일을 찾아보라고 러스트에게 말해줄 필요가 있습니다; 이는
`mod client`라는 코드가 왜 *src/lib.rs* 내에 정의될 필요가 있는지, 그리고 *src/client.rs*
내에는 정의될 수 없는지에 대한 이유입니다.

이제 몇 개의 컴파일 경고가 생기지만, 프로젝트는 성공적으로 컴파일 되어야 합니다. 우리가 바이너리 크레이트
대신 라이브러리 크레이트를 만드는 중이므로 `cargo run` 대신 `cargo build`를 이용해야 한다는 점을
기억해두세요:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:4:5
  |
4 |     fn connect() {
  |     ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/lib.rs:8:9
  |
8 |         fn connect() {
  |         ^
```

이 경고들은 사용된 적이 없는 함수가 있음을 우리에게 알려줍니다. 지금은 이 경고들을 너무 걱정하지 마세요:
이 장의 뒤에 나오는 “`pub`을 이용하여 가시성 제어하기”절에서 이 문제에 대해 알아볼 것입니다. 좋은 소식은
이들이 그냥 경고일 뿐이란 것입니다; 우리 프로젝트는 성공적으로 빌드됐습니다!

다음으로 같은 방식을 이용하여 `network` 모듈을 개별 파일로 추출해봅시다. *src/lib.rs* 안에서,
아래와 같이 `network` 모듈의 몸체를 지우고 선언부의 끝부분에 세미콜론을 붙이세요:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod client;

mod network;
```

그리고나서 새로운 *src/network.rs* 파일을 만들어서 아래를 입력하세요:

<span class="filename">Filename: src/network.rs</span>

```rust
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

이 모듈 파일 내에는 `mod` 선언이 여전히 있음을 주목하세요; 이는 `server`가 `network`의 서브모듈로서
여전히 필요하기 때문입니다.

`cargo build`를 다시 실행시키세요. 성공! 여기 또 추출할만한 모듈이 하나 더 있습니다: `server` 말이죠.
이것이 서브모듈(즉, 모듈 내의 모듈)이기 때문에, 모듈을 파일로 추출해서 파일 이름을 모듈 이름으로 사용하는
전략은 사용하기 힘듭니다. 어쨌든 시도해서 에러를 확인해보겠습니다. 먼저, *src/network.rs* 내에서
`server` 모듈의 내용물 대신에 `mod server`을 쓰세요:

<span class="filename">Filename: src/network.rs</span>

```rust,ignore
fn connect() {
}

mod server;
```

그후 *src/server.rs* 파일을 만들고 추출해둔 `server` 모듈의 내용물을 입력하세요:

<span class="filename">Filename: src/server.rs</span>

```rust
fn connect() {
}
```

`cargo build`를 실행해보면, Listing 7-4와 같은 에러를 얻게 됩니다:

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

<span class="caption">Listing 7-4: `server` 서브모듈을 *src/server.rs*로 추출을 시도했을 때
발생하는 에러</span>

에러는 `이 위치에 새로운 모듈을 선언할수 없다`고 말해주며 *src/network.rs*의 `mod server;`
라인을 지적하고 있습니다. *src/network.rs*는 *src/lib.rs*와는 다소 다릅니다: 왜 그런지
이해하려면 계속 읽어주세요.

Listing 7-4의 중간의 노트는 실질적으로 매우 도움이 되는데, 그 이유는 우리가 아직 설명하지 않은
무언가를 지적하고 있기 때문입니다:

```text
note: maybe move this module `network` to its own directory via
`network/mod.rs`
```

전에 사용했던 똑같은 파일 이름 쓰기 패턴을 계속해서 따르는 대신, 아래 노트에서 제안하는 것을 해볼
수 있습니다:

1. 부모 모듈의 이름에 해당하는, *network*라는 이름의 새로운 *디렉토리*를 만드세요.
2. *src/network.rs* 파일을 이 새로운 *network* 디렉토리 안으로 옮기고, 파일 이름을
   *src/network/mod.rs*로 고치세요.
3. 서브모듈 파일 *src/server.rs*를 *network* 디렉토리 안으로 옮기세요.

위의 단계들을 실행하기 위한 명령들입니다:
```text
$ mkdir src/network
$ mv src/network.rs src/network/mod.rs
$ mv src/server.rs src/network
```

이제 `cargo build`를 다시 실행하면, 컴파일은 작동할 것입니다 (여전히 경고는 좀 있지만요). 우리의
모듈 레이아웃은 여전히 아래와 같이 되는데, 이는 Listing 7-3의 *src/lib.rs* 내의 코드에서 만든 것과
정확하게 동일합니다:

```text
communicator
 ├── client
 └── network
     └── server
```

이에 대응하는 파일 레이아웃는 아래와 같이 생겼습니다:

```text
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```

그러니까 우리가 `network::server` 모듈을 추출하고자 할 때, 왜 `network::server` 모듈을
*src/server.rs*로 추출하는 대신, *src/network.rs* 파일에 *src/network/mod.rs*로 옮기고
`network::server` 코드를 *network* 디렉토리 안에 있는 *src/network/server.rs*에 넣었을까요?
그 이유는 *src* 디렉토리 안에 *server.rs* 파일이 있으면, 러스트는 `server`가 `network`의
서브모듈이라고 인식할 수 없기 때문입니다. 러스트가 동작하는 방식을 명확하게 알기 위해서, 아래와 같은 모듈
계층 구조를 가진, *src/lib.rs* 내에 모든 정의가 다 들어있는 다른 예제를 봅시다:

```text
communicator
 ├── client
 └── network
     └── client
```

이 예제에는 또다시 `client`, `network`, 그리고 `network::client`라는 세 개의 모듈이 있습니다.
모듈을 파일로 추출하기 위해 앞서 했던 단계를 따르면, `client` 모듈을 위한 *src/client.rs*을
만들게 될 것입니다. `network` 모듈을 위해서는 *src/network.rs* 파일을 만들게 될 것입니다.
하지만 `network::client` 모듈을 *src/client.rs*로 추출하는 것은 불가능한데, 그 이유는
최상위 층에 `client` 모듈이 이미 있기 때문이죠! 만일 `client`와 `network::client` 모듈
*둘다* *src/client.rs* 파일에 집어넣는다면, 러스트는 이 코드가 `client`를 위한 것인지,
아니면 `network::client`를 위한 것인지 알아낼 방법이 없을 것입니다.

따라서, `network` 모듈의 `network::client` 서브모듈을 위한 파일을 추출하기 위해서는
*src/network.rs* 파일 대신 `network` 모듈을 위한 디렉토리를 만들 필요가 있습니다. `network`
모듈 내의 코드는 그후 *src/network/mod.rs* 파일로 가고, 서브모듈 `network::client`은
*src/network/client.rs* 파일을 갖게할 수 있습니다. 이제 최상위 층의 *src/client.rs*는
모호하지 않게 `client` 모듈이 소유한 코드가 됩니다.



### 모듈 파일 시스템의 규칙

파일에 관한 모듈의 규칙을 정리해봅시다:

* 만일 `foo`라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, *foo.rs*라는 이름의 파일 내에
  `foo`에 대한 선언을 집어넣어야 합니다.
* 만일 `foo`가 서브모듈을 가지고 있다면, *foo/mod.rs*라는 이름의 파일에 `foo`에 대한 선언을
  집어넣어야 합니다.

이 규칙들은 재귀적으로 적용되므로, `foo`라는 이름의 모듈이 `bar`라는 이름의 서브모듈을 갖고 있고
`bar는 서브모듈이 없다면, 여러분의 *src* 디렉토리 안에는 아래와 같은 파일들이 있어야 합니다:

```text
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

이 모듈들은 부모 모듈의 파일에 `mod` 키워드를 사용하여 선언되어 있어야 합니다.

다음으로, `pub` 키워드에 대해 알아보고 앞의 그 경고들을 없애봅시다!
