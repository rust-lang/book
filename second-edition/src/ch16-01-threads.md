## 스레드를 이용하여 코드를 동시에 실행하기

대부분의 요즘 운영 체제에서, 실행되는 프로그램의 코드는 *프로세스* 내에서 실행되고,
운영 체제는 한번에 여러 개의 프로세스들을 관리합니다. 여러분의 프로그램 내에서도
동시에 실행되는 독립적인 부분들을 가질 수 있습니다. 이러한 독립적인 부분들을 실행하는
기능을 *스레드*라고 부릅니다.

여러분의 프로그램 내에 계산 부분을 여러 개의 스레드로 쪼개는 것은 프로그램이
동시에 여러 개의 일을 할 수 있기 때문에 성능을 향상시킬 수 있지만, 프로그램을
복잡하게 만들기도 합니다. 스레드가 동시에 실행될 수 있기 때문에, 다른 스레드
상에서 실행될 여러분의 코드 조각들의 실행 순서에 대한 내재적인 보장이 없습니다.
이는 다음과 같은 문제들을 야기할 수 있습니다:

* 여러 스레드들이 일관성 없는 순서로 데이터 혹은 리소스에 접근하게 되는, 경쟁 조건
  (race condition)
* 두 스레드가 서로 상대방 스레드가 가지고 있는 리소스의 사용을 끝내길 기다려서
  양쪽 스레드 모두 계속 실행되는 것을 막아버리는, 데드록 (deadlock)
* 특정한 상황에서만 발생되어 재현하기와 안정적으로 수정하기가 힘든
  버그들

러스트는 스레드 사용의 부정적인 효과를 줄이려는 시도를 하지만, 다중
스레드 콘텍스트 내에서의 프로그래밍은 여전히 신중하게 생각해야 하고
단일 스레드 내에서 실행되는 프로그램의 것과는 다른 코드 구조가
필요합니다.

프로그래밍 언어들은 몇가지 다른 방식으로 스레드를 구현합니다. 많은 운영 체제들이
새로운 스레드를 만들기 위한 API를 제공합니다. 언어가 운영 체제의 API를 호출하여
스레드를 만드는 이러한 구조는 때때로 *1:1*이라 불리는데, 이는 하나의 운영 체제
스레드가 하나의 언어 스레드에 대응된다는 의미입니다.

많은 프로그래밍 언어들은 그들만의 특별한 스레드 구현을 제공합니다. 프로그래밍
언어가 제공하는 스레드는 *그린 (green)* 스레드라고 알려져 있으며, 이러한
그린 스레드를 사용하는 언어들은 다른 숫자의 운영 체제 스레드로 구성된 콘텍스트
내에서 그린 스레드들을 실행할 것입니다. 이러한 이유로 인하여 그린 스레드 구조는
*M:N*이라고 불립니다: `M` 개의 그린 스레드가 `N` 개의 시스템 스레드에
대응되는데, 여기서 `M`과 `N`은 굳이 동일한 숫자가 아니어도
됩니다.

각각의 구조는 고유한 장점과 트레이드 오프를 가지고 있으며, 러스트에게 있어
가장 중요한 트레이드 오프는 런타임 지원입니다. *런타임*은 혼동하기 쉬운 용어이고
맥락에 따라 다른 의미를 가질 수 있습니다.

이 글의 맥락에서 *런타임*이라 하는 것은 언어에 의해 모든 바이너리 내에
포함되는 코드를 의미합니다. 이 코드는 언어에 따라 크거나 작을 수 있지만,
모든 어셈블리 아닌 언어들은 어느 정도 크기의 런타임 코드를 가지게 될 것입니다.
이러한 이유로 인하여, 흔히 사람들이 “런타임이 없다”라고 말할 때는, 종종
“런타임이 작다”는 것을 의미하는 것입니다. 런타임이 작을 수록 더 적은
기능을 갖지만 더 작아진 바이너리로 인해 얻어지는 장점을 갖는데, 이는
더 큰 콘텍스트 내에서 다른 언어들과 조합하기 쉬워진다는 점입니다. 비록
많은 언어들이 더 많은 기능을 위하여 런타임 크기를 늘리는 거래를 수락하더라도,
러스트는 거의 런타임이 없을 필요가 있고 성능을 관리하기 위해 C를 호출하는
것에 대해 타협할 수 없습니다.

그린 스레드 M:N 구조는 스레드들을 관리하기 위해 더 큰 언어 런타임이
필요합니다. 그런 이유로 러스트 표준 라이브러리는 오직 1:1 스레드 구현만
제공합니다. 러스트가 이러한 저수준 언어이기 때문에, 여러분이 예를 들어 어떤
스레드를 언제 실행시킬지에 대한 더 많은 제어권과 콘텍스트 교환(context
switching)의 더 저렴한 비용 같은 관점을 위해 오버헤드와 맞바꾸겠다면
M:N 스레드를 구현한 크레이트도 있습니다.

이제 러스트에서의 스레드를 정의했으니, 표준 라이브러리가 제공하는 스레드
관련 API를 어떻게 사용하는지를 탐구해봅시다.

### `spawn`으로 새로운 스레드 생성하기

새로운 스레드를 생성하기 위해서는 `thread::spawn` 함수를 호출하고 여기에
우리가 새로운 스레드 내에서 실행하기를 원하는 코드가 담겨 있는 클로저를 넘깁니다
(클로저에 대해서는 13장에서 다뤘습니다). Listing 16-1의 예제는 메인 스레드에서
어떤 텍스트를 출력하고 새로운 스레드에서는 다른 텍스트를 출력합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

<span class="caption">Listing 16-1: 메인 스레드에서 무언가를 출력하는 동안
다른 것을 출력하는 새로운 스레드 생성하기</span>

이 함수를 가지고, 새로운 스레드는 실행이 종료되었든 혹은 그렇지 않든
메인 스레드가 종료될 때 멈추게 될 것이라는 점을 주의하세요. 이
프로그램의 출력은 매번 약간씩 다를지도 모르겠으나, 아래와 비슷하게 보일
것입니다:

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep`의 호출은 강제로 스레드가 잠깐 동안 실행을 멈추게 하는데,
다른 스레드가 실행되는 것을 허용합니다. 스레드들은 아마도 교대로 실행될 것이지만,
보장되지는 않습니다: 여러분의 운영 체제가 어떻게 스레드를 스케줄링 하는지에 따라
달린 문제입니다. 위의 실행 예에서는 생성된 스레드로부터의 출력 구문이 코드의
첫번째에 나타나 있음에도 불구하고 메인 스레드가 먼저 출력하였습니다. 그리고
생성된 스레드에게 `i`가 9일때까지 출력하라고 했음에도 불구하고, 메인 스레드가
멈추기 전까지 고작 5에 도달했습니다.

만일 여러분이 이 코드를 실행하고 메인 스레드로부터의 출력만 보았다면, 혹은 어떠한
오버랩도 보지 못했다면, 숫자 범위를 늘려서 운영 체제로 하여금 스레드간의 전환에
더 많은 기회를 주는 시도를 해보세요.

### `join` 핸들을 사용하여 모든 스레드들이 끝날때까지 기다리기

Listing 16-1의 코드는 대개의 경우 메인 스레드가 종료되는 이유로 생성된
스레드가 조기에 멈출 뿐만 아니라, 생성된 스레드가 모든 코드를 실행할 것임을
보장해 줄수도 없습니다. 그 이유는 스레드들이 실행되는 순서에 대한 보장이
없기 때문입니다!

생성된 스레드가 실행되지 않거나, 전부 실행되지 않는 문제는
`thread::spawn`의 반환값을 변수에 저장함으로서 해결할 수 있습니다.
`thread::spawn`의 반환 타입은 `JoinHandle`입니다. `JoinHandle`은
이것이 가지고 있는 `join` 메소드를 호출했을 때 그 스레드가 끝날때까지 기다리는
소유된 값입니다. Listing 16-2는 어떤식으로 우리가 Listing 16-1에서 만들었던
스레드의 `JoinHandle`을 사용하고 `join`을 호출하여 `main`이 끝나기 전에
생성된 스레드가 종료되는 것을 확실하게 하는지를 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-2: 스레드가 완전시 실행되는 것을 보장하기
위해 `thread::spawn`으로부터 `JoinHandle`을 저장하기</span>

핸들에 대해 `join`을 호출하는 것은 핸들에 대한 스레드가 종료될 때까지 현재
실행중인 스레드를 블록합니다. 스레드를 *블록 (Block)* 한다는 것은 그 스레드의
작업을 수행하거나 종료되는 것이 방지된다는 의미입니다. 우리가 메인 스레드의 `for`
루프 이후에 `join`의 호출을 넣었으므로, Listing 16-2의 실행은 아래와 비슷한
출력을 만들어야 합니다:

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

두 스레드가 교차를 계속하지만, `handle.join()`의 호출로 인하여 메인
스레드는 기다리고 생성된 스레드가 종료되기 전까지 끝나지 않습니다.

그런데 만일 아래와 같이 `main`의 `for` 루프 이전으로 `handle.join()`을
이동시키면 어떤 일이 생기는지 봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

메인 스레드는 생성된 스레드가 종료될 때까지 기다릴 것이고 그 다음 자신의 `for`
루프를 실행시키게 되어, 아래처럼 출력값이 더 이상 교차되지 않을 것입니다:

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

`join`이 호출되는 위치와 같은 작은 디테일들이 여러분의 스레드가 동시에 실행되는지
혹은 아닌지에 대해 영향을 미칠 수 있습니다.

### 스레드에 `move` 클로저 사용하기

`move` 클로저는 `thread::spawn`와 함께 자주 사용되는데 그 이유는 이것이 여러분으로
하여금 어떤 스레드의 데이터를 다른 스레드 내에서 사용하도록 해주기 때문입니다.

13장에서는 클로저의 파라미터 목록 앞에 `move` 키워드를 이용하여
클로저가 그 환경에서 사용하는 값의 소유권을 강제로 갖게 한다고
언급했습니다. 이 기술은 값의 소유권을 한 스레드에서 다른 스레드로
이전하기 위해 새로운 스레드를 생성할 때 특히 유용합니다.

Listing 16-1에서 우리가 'thread::spawn'에 넘기는 클로저는 아무런 인자도 갖지
갖지 않는다는 점을 주목하세요: 생성된 스레드의 코드 내에서는 메인 스레드로부터 온 어떤 데이터도
이용하고 있지 않습니다. 메인 스레드로부터의 데이터를 생성된 스레드 내에서 사용하기 위해서는
생성된 스레드의 클로저가 필요로 하는 값을 캡처해야 합니다. Listing 16-3은 메인 스레드에서
백터 생성하여 이를 생성된 스레드 내에서 사용하는 시도를 보여주고 있습니다. 그러나 잠시 후에
보시게 될 것처럼 아직은 동작하지 않습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-3: 메인 스레드에서 생성된 벡터를 다른
스레드 내에서 사용하는 시도</span>

클로저는 `v`를 사용하므로, `v`는 캡처되어 클로저의 환경의 일부가 됩니다.
`thread::spawn`이 이 클로저를 새로운 스레드 내에서 실행하므로, `v`는
새로운 스레드 내에서 접근 가능해야 합니다. 하지만 이 예제를 컴파일하면
아래와 같은 에러를 얻게 됩니다:

```text
error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

러스트는 `v`를 어떻게 캡처하는지 *추론하고*, `println!`이 `v`의 참조자만
필요로 하기 때문에, 클로저는 `v`를 빌리는 시도를 합니다. 하지만 문제가 있습니다:
러스트는 생성된 스레드가 얼마나 오랫동안 실행될지 말해줄 수 없으므로, `v`에 대한
참조자가 항상 유효할 것인지를 알지 못합니다.

Listing 16-4는 유효하지 않게 된 `v`의 참조자를 갖게 될 가능성이 더 높은
시나리오를 제공합니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-4: `v`를 드롭하는 메인 스레드로부터 `v`에
대한 참조자를 캡처하는 시도를 하는 클로저를 갖는 스레드</span>

만약 우리가 이 코드를 실행할 수 있다면, 생성된 스레드가 전혀 실행되지 않고
즉시 백그라운드에 들어갈 가능성이 있습니다. 생성된 스레드는 내부에 `v`의
참조자를 가지고 있지만, 메인 스레드는 우리가 15장에서 다루었던 `drop`
함수를 사용하여 `v`를 즉시 드롭시킵니다. 그러면 생성된 스레드가 실행되기
시작할 때 `v`가 더 이상 유효하지 않게 되어, 참조자 또한 유요하지 않게
됩니다. 이런!

Listing 16-3의 컴파일 에러를 고치기 위해서는 에러 메세지의 조언을 이용할
수 있습니다:

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

`move` 키워드를 클로저 앞에 추가함으로서 우리는 러스트가 값을 빌려와야
된다고 추론하도록 하는 것이 아니라 사용하는 값의 소유권을 강제로 가지도록
합니다. Listing 16-3을 Listing 16-5에서 보이는 것처럼 수정하면
컴파일되어 우리가 원하는 대로 실행됩니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-5: `move` 키워드를 사용하여 사용하는 값의
소유권을 클로저가 갖도록 강제하기</span>

메인 스레드에서 `drop`을 호출하는 Listing 16-4의 코드에서 `move` 클로저를
이용한다면 어떤 일이 벌어질까요? `move`가 이 경우도 고칠 수 있을까요? 불행하게도,
아닙니다; Listing 16-4이 시도하고자 하는 것이 다른 이유로 허용되지 않기 때문에
우리는 다은 에러를 얻게 됩니다. 만일 클로저에 `move`를 추가하면, `v`를 클로저의
환경으로 이동시킬 것이고, 더이상 메인 스레드에서 이것에 대한 `drop` 호출을 할
수 없게 됩니다. 대신 우리는 아래와 같은 컴파일 에러를 얻게 됩니다:

```text
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
```

러스트의 소유권 규칙이 다시 한번 우리를 구해주었습니다! Listing 16-3의
코드로부터 에러를 받은 이유는 러스트가 보수적이고 스레드를 위해 `v`를 단지
빌리려고만 했기 때문이었는데, 이는 메인스레드가 이론적으로 생성된 스레드의
참조자를 무효화할 수 있음을 의미합니다. 러스트에게 `v`의 소유권을 생성된 스레드로
이동시키라 말해줌으로서, 우리는 러스트에게 메인 스레드가 `v`를 더 이상 이용하지
않음을 보장하고 있습니다. 만일 우리가 Listing 16-4를 같은 방식으로 바꾸면,
우리가 `v`를 메인스레드 상에서 사용하고자 할 때 소유권 규칙을 위반하게 됩니다.
`move` 키워드는 러스트의 빌림에 대한 보수적인 기본 기준을 무효화합니다;
즉 우리가 소유권 규칙을 위반하지 않도록 해줍니다.

스레드와 스레드 API에 대한 기본적인 이해를 하고서, 우리가 스레드를 가지고 어떤 것을
*할 수* 있는지 살펴봅시다.
