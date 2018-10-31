## `pub`으로 가시성(visibility) 제어하기

우리는 `network`와 `network::server` 코드를 각각 *src/network/mod.rs*와
*src/network/server.rs* 파일 안으로 이동시켜서 Listing 7-4에 나온 에러 메세지를
해결했습니다. 이 지점에서 `cargo build`로 프로젝트를 빌드할 수 있긴 했지만,
사용하지 않고 있는 `client::connect`, `network::connect`, 그리고
`network::server::connect` 함수에 대한 경고 메세지를 보게 됩니다:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
src/client.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

그럼 이런 경고들은 왜 나오는 걸까요? 결국, 우리는 우리 자신의 프로젝트 내에서
사용할 필요가 있는 것이 아닌, *사용자*가 사용할 수 있도록 만들어진 함수들의
라이브러리를 만드는 중이므로, 이런 `connect` 함수 등이 사용되지 않는 것은
큰 문제가 아닙니다. 이 함수들을 만든 의도는 함수들이 우리의 지금 이 프로젝트가
아닌 또다른 프로젝트에 사용될 것이란 점입니다.

이 프로그램이 이러한 경고들을 들먹이는 이유를 이해하기 위해, `connect` 라이브러리
를 다른 프로젝트에서 사용하기를 시도해 봅시다. 이를 위해서, 아래의 코드를 담은
*src/main.rs* 파일을 만듦으로서 같은 디렉토리에 라이브러리 크레이트와 마찬가지로
바이너리 크레이트를 만들겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

`communicator` 라이브러리 크레이트를 가져오기 위해 `extern crate` 명령어를
사용합니다. 우리의 패키지는 이제 *두 개의* 크레이트를 담고 있습니다. 카고는
*src/main.rs*를 바이너리 크레이트의 루트 파일로 취급하는데, 이 바이너리 크레이트는
*src/lib.rs*가 루트 파일인 이미 있던 라이브러리 크레이트는 별개입니다. 이러한 패턴은
실행 가능한 프로젝트에서 꽤 흔합니다: 대부분의 기능은 라이브러리 크레이트 안에 있고,
바이너리 크레이트는 이 라이브러리 크레이트를 이용합니다. 결과적으로, 다른 프로그램 또한
그 라이브러리 크레이트를 이용할 수 있고, 이는 멋지게 근심을 덜어줍니다.

From the point of view of a crate outside the `communicator` library looking
in, all the modules we’ve been creating are within a module that has the same
name as the crate, `communicator`. We call the top-level module of a crate the
*root module*.
`communicator` 라이브러리 밖의 크레이트가 안을 들여다 보는 시점에서, 우리가 만들어왔던
모든 모듈들은 `communicator`라는 이름을 갖는 모듈 내에 있습니다. 크레이트의 최상위
모듈을 *루트 모듈 (root module)* 이라 부릅니다.

또한. 비록 우리의 프로젝트의 서브모듈 내에서 외부 크레이트를 이용하고 있을지라도,
`extern crate`이 루트 모듈에 와 있어야 한다는 점(즉 *src/main.rs* 혹은
*src/lib.rs*)을 기억하세요. 그러면 서브모듈 안에서 마치 최상위 모듈의 아이템을 참조하듯
외부 크레이트로부터 아이템들을 참조할 수 있습니다.

현시점에서 우리의 바이너리 크레이트는 고작 라이브러리의 `client` 모듈로부터 `connect`
함수를 호출할 뿐입니다. 하지만 `cargo build`을 실행하면 경고들 이후에 에러를 표시할
것입니다:

```text
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

아하! 이 에러는 `client` 모듈이 비공개(private) 임을 알려주고 있는데, 이는 그 경고들의
요점입니다. 또한 러스트의 내용 중에서 *공개(public)* 그리고 *비공개(private)* 에 대한
개념에 대해 알아보게 될 첫번째 시간입니다. 러스트의 모든 코드의 기본 상태는 비공개입니다:
즉, 다른 사람은 이 코드를 사용할 수 없습니다. 만일 여러분의 프로그램 내에서 비공개 함수를
이용하지 않는다면, 여러분의 프로그램이 그 함수를 이용할 수 있는 유일한 곳이기 때문에,
러스트는 그 함수가 사용된 적이 없다며 경고해줄 것입니다.

`client::connect`와 같은 함수를 공개로 지정한 뒤에는 우리의 바이너리 크레이트 상에서
이 함수를 호출하는 것이 가능해질 뿐만 아니라, 그 함수가 사용된 적이 없다는 경고 또한
사라질 것입니다. 함수를 공개로 표시하는 것은 러스트로 하여금 그 함수가 우리 프로그램
외부의 코드에 의해 사용될 것이라는 점을 알게끔 해줍니다. 러스트는 이제부터 가능하게
된 이론적인 외부 사용에 대해 이 함수가 “사용되었다”라고 간주합니다. 따라서, 어떤 것이
공개로 표시될 때, 러스트는 그것이 우리 프로그램 내에서 이용되는 것을 요구하지
않으며 해당 아이템이 미사용에 대한 경고를 멈출 것입니다.

### 함수를 공개로 만들기

러스트에게 어떤 것을 공개하도록 말하기 위해서는, 공개하길 원하는 아이템의 선언 시작
부분에 `pub` 키워드를 추가합니다. 지금은 `client::connect`가 사용된 적 없음을 알리는
경고와 바이너리 크레이트에서 나온 `` module `client` is private `` 에러를 제거하는데
집중하겠습니다. 아래와 같이 *src/lib.rs*을 수정하여 `client` 모듈을 공개로 만드세요:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

mod network;
```

`pub` 키워드는 `mod` 바로 전에 위치합니다. 다시 빌드를 시도해봅시다:

```text
error: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

만세! 다른 에러가 나왔습니다! 네, 다른 에러 메세지라는건 축하할만한 이유죠. 새로운
에러는 `` function `connect` is private ``라고 하고 있으므로, *src/client.rs*를
수정해서 `client::connect`도 공개로 만듭시다:

<span class="filename">Filename: src/client.rs</span>

```rust
pub fn connect() {
}
```

이제 `cargo build`를 다시 실행하면:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

코드가 컴파일되었고, `client::connect`가 사용된 적 없다는 것에 대한 경고도 사라집니다!

미사용 코드 경고가 항상 여러분의 코드에 있는 아이템이 공개로 만들
필요가 있음을 나타내는 것은 아닙니다: 이 함수들이 여러분의 공개 API의 일부분으로서
들어가길 원하지 *않는다면*, 미사용 코드 경고는 여러분에게 해당 코드가 더이상 필요
없고 안전하게 지울 수 있음을 알려줄 수 있습니다. 또한 이 경고는 여러분의 라이브러리
내에서 해당 함수가 호출된 모든 곳을 실수로 지웠을 경우 발생할 수 있는 버그를 알려줄
수도 있습니다.

하지만 지금의 경우, 우리는 다른 두 함수들이 우리 크레이트의 공개 API의 일부분이
되길 원하고 있으므로, 이들에게 `pub`를 표시해줘서 남은 경고들을 제거합시다.
*src/network/mod.rs*를 아래와 같이 수정하세요:

<span class="filename">Filename: src/network/mod.rs</span>

```rust,ignore
pub fn connect() {
}

mod server;
```

그리고 컴파일하면:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/mod.rs:1:1
  |
1 | pub fn connect() {
  | ^

warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

흠, `network::connect`가 `pub`으로 설정되어 있음에도, 여전히 미사용 함수 경고가
나옵니다. 그 이유는 함수가 모듈 내에서 공개지만, 함수가 상주해 있는 `network` 모듈은
공개가 아니기 때문입니다. 이번에는 모듈의 안쪽에서 작업하고 있지만,
`client::connect`에서는 바깥쪽에서 작업을 했었죠. *src/lib.rs*을 수정하여 `network`가
공개가 되도록 할 필요가 있습니다. 이렇게요:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
pub mod client;

pub mod network;
```

이제 컴파일하면, 그 경고는 사라집니다:

```text
warning: function is never used: `connect`, #[warn(dead_code)] on by default
 --> src/network/server.rs:1:1
  |
1 | fn connect() {
  | ^
```

경고 딱 하나 남았네요! 여러분이 직접 고쳐보세요!

### 비공개 규칙(Privacy Rules)

종합해보면, 아이템 가시성에 관한 규칙은 다음과 같습니다:

1. 만일 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근 가능합니다.
2. 만일 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의
   자식 모듈에서만 접근 가능합니다.

### 비공개 예제(Privacy Examples)

연습을 위해 몇 가지 비공개에 관한 예제를 봅시다. 새로운 라이브러리 프로젝트를 만들고
이 새로운 프로젝트의 *src/lib.rs*에 Listing 7-5와 같이 코드를 넣으세요:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

<span class="caption">Listing 7-5: 비공개 및 공개 함수 예제. 몇 가지는 잘못되었음.</span>

이 코드를 컴파일하기 전에, `try_me` 함수의 어떤 라인이 에러를 발생시킬지 추측해보세요.
그리고나서 컴파일을 하여 여러분이 맞았는지 확인하고, 에러에 대한 논의를 위해 계속
읽어주세요!

#### 에러 보기

`try_me` 함수는 우리 프로젝트의 루트 모듈 내에 있습니다. `outermost` 모듈은
비공개지만, 두 번째 비공개 규칙은 `try_me`함수가 `outermost` 모듈에 접근하는 것이
허용됨을 알려주는데, 이는 `outermost`가 `try_me` 함수와 마찬가지로 현재의 (루트)
모듈 내에 있기 때문입니다.

`middle_function`이 공개이므로 `outermost::middle_function` 호출은 작동할 것이며,
`try_me`는 `middle_function`의 부모 모듈인 `outermost`를 통해 `middle_function`에
접근하고 있습니다. 이 모듈에 접근 가능하하는 것은 이전 문단에서 알아냈죠.

`outermost::middle_secret_function` 호출은 컴파일 에러를 일으킬 것입니다.
`middle_secret_function`는 비공개이므로, 두번째 규칙이 적용됩니다. 루트 모듈은
`middle_secret_function`의 현재 모듈도 아니고 (`outermost`가 현재 모듈입니다),
`middle_secret_function`의 현재 모듈의 자식 모듈도 아닙니다.

`inside` 모듈은 비공개고 자식 모듈이 없으므로, 이것의 현재 모듈인 `outermost`에 의해서만
접근될 수 있습니다. 이는 즉 `try_me` 함수는 `outermost::inside::inner_function`나 
`outermost::inside::secret_function`를 호출할 수 없음을 의미합니다.

#### 에러 고치기

여기 이 에러들을 고치기 위해 코드를 수정하는것에 관한 몇 가지 제안이 있습니다.
각각을 시도해보기 전에, 이 시도가 에러를 고칠지 그렇지 않을지 추측해 보고,
컴파일을 해서 여러분이 맞췄는지 그렇지 않은지 확인하고, 왜 그랬는지 이해하기 위해
비공개 규칙을 이용해보세요.

* `inside` 모듈이 공개라면 어떨까요?
* `outermost`가 공개고 `inside`가 비공개면 어떨까요?
* `inner_function`의 내부에서 `::outermost::middle_secret_function()`을
  호출한다면 어떨까요? (시작 부분의 콜론 두개는 루트 모듈로부터 시작하여 모듈을 참조하고
  싶음을 나타냅니다)

자유롭게 더 많은 실험을 설계하고 시도해 보세요!

다음으로, `use` 키워드를 사용하여 아이템을 스코프 내로 가져오는 것에 대해 이야기해
봅시다.
