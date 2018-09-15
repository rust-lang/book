## 공유 상태 동시성

메세지 패싱은 동시성을 다루는 좋은 방법이지만, 유일한 수단은
아닙니다. Go 언어 문서로부터 나온 슬로건의 일부를 다시한번 고려해보죠:
“메모리를 공유함으로써 소통하세요.”

메모리를 공유하는 통신은 어떤 형태로 보일까요? 더불어서 메세지 패싱의
열광적인 지지자들은 왜 이걸 안쓰고 대신 반대편의 것을 쓸까요?

어떤 면에서, 프로그래밍 언어의 채널들은 단일 소유권과 유사한데,
이는 여러분이 채널로 값을 송신하면, 그 값을 더이상 쓸 수 없게되기
때문입니다. 공유 메모리 동시성은 복수 소유권과 유사합니다: 복수개의
스레드들이 동시에 동일한 메모리 위치를 접근할 수 있지요. 스마트 포인터들이
복수 소유권을 가능하게 만드는 내용을 담은 15장에서 보셨듯이, 복수 소유권은
이 서로 다른 소유자들의 관리가 필요하기 때문에 복잡성을 더할 수 있습니다.
러스트의 타입 시스템과 소유권 규칙은 이러한 관리를 올바르도록 훌륭히 유도합니다.
예를 들면, 공유 메모리를 위한 더 일반적인 동시성의 기초 재료 중 하나인
뮤텍스 (mutex)를 살펴 봅시다.

### 뮤텍스를 사용하여 한번에 한 스레드에서의 데이터 접근을 허용하기

*뮤텍스*는 *상호 배제 (mutual exclusion)* 의 줄임말로서, 내부에서 뮤텍스는
주어진 시간에 오직 하나의 스레드만 데이터 접근을 허용합니다. 뮤텍스 내부의 데이터에
접근하기 위해서 스레드는 먼저 뮤텍스의 *락 (lock)* 을 얻기를 요청함으로써 접근을
윈한다는 신호를 보내야 합니다. 락은 누가 배타적으로 데이터에 접근하는지를 추적하는
뮤텍스의 부분인 데이터 구조입니다. 그러므로, 뮤텍스는 잠금 시스템을 통해 가지고 있는
데이터를 *보호하는* 것으로 묘사됩니다.

뮤텍스는 사용하기 어렵다는 평판을 가지고 있는데 이는 여러분이 다음 두 가지 규칙을
기억해야 하기 때문입니다:

* 여러분은 데이터를 사용하기 전에 반드시 락을 얻는 시도를 해야 합니다.
* 만일 뮤텍스가 보호하는 데이터의 사용이 끝났다면, 다른 스레드들이 락을
  얻을 수 있도록 반드시 언락해야 합니다.

뮤텍스에 대한 실세계 은유를 위해서, 마이크가 딱 하나만 있는 컨퍼런스 패널
토의를 상상해보세요. 패널 참가자들이 말하기 전, 그들은 마이크 사용을
원한다고 요청하거나 신호를 줘야 합니다. 마이크를 얻었을 때는
원하는 만큼 길게 말을 한 다음 말하기를 원하는 다음 매널 참가자에게
마이크를 건네줍니다. 만일 패널 참여자가 마이크 사용을 끝냈을 때
이를 건네주는 것을 잊어먹는다면, 그 외 아무도 말할 수 없게 됩니다.
공유된 마이크의 관리가 잘못되면, 패널은 계획된데로 되지
않을겁니다!

뮤텍스의 관리는 바로잡기 위해 믿을 수 없으리만치 교묘해질 수 있는데, 이것이 바로
많은 사람들이 체널의 열성 지지자가 되는 이유입니다. 하지만, 러스트의 타입 시스템과
소유권 규칙에 감사하게도, 여러분은 잘못 락을 얻거나 언락 할 수가 없습니다.

#### `Mutex<T>`의 API

어떻게 뮤텍스를 이용하는지에 대한 예제로서, Listing 16-12와 같이 단일
스레드 맥락 내에서 뮤텍스를 사용하는 것으로 시작해봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

<span class="caption">Listing 16-12: 단순함을 위해 단일 스레드 맥락 내에서
`Mutex<T>`의 API 탐색하기</span>

많은 타입들처럼 `Mutex<T>`는 연관함수 `new`를 사용하여 만들어집니다.
뮤텍스 내의 데이터에 접근하기 위해서는 `lock` 메소드를 사용하여 락을
엇습니다. 이 호출은 현재의 스레드를 막아설 것이므로, 락을 얻는 차례가
될 때까지 아무런 작업도 할 수 없습니다.

`lock`의 호출은 다른 스레드가 패닉 상태의 락을 가지고 있을 경우 실패할 수 있습니다.
그런 경우 아무도 락을 얻을 수 없게 되므로, `unwrap`을 택하여 그런 상황일
경우 이 스레드에 패닉을 일으킵니다.

락을 얻고난 다음에는 그 반환값 (위의 경우에는 `num`이라는 이름의 값) 을 내부의 데이터에
대한 가변 참조자처럼 다룰 수 있습니다. 타입 시스템은 `m` 내부의 값을 사용하기 전에 우리가
락을 얻는 것을 확실히 해줍니다: `Mutex<i32>`는 `i32`가 아니므로 우리는 *반드시*
`i32` 값을 사용하기 위해 락을 얻어야 합니다. 우리는 이를 잊어버릴 수 없습니다;
잊어버린다면 타입 시스템이 내부의 `i32`에 접근할 수 없게 할 것입니다.

여러분이 의심한 것처럼, `Mutex<T>`는 스마트 포인터입니다. 더 정확하게는, `lock`의
호출은 `MutexGuard`라고 불리우는 스마트 포인터를 *반환합니다.* 이 스마트 포인터는
우리의 내부 데이터를 가리키도록 `Deref`가 구현되어 있습니다; 이 스마트 포인터는 또한
`MutexGuard`가 스코프 밖으로 벗어났을 때 자동으로 락을 해제하는 `Drop` 구현체를
가지고 있는데, 이는 Listing 16-12의 내부 스코프의 끝에서 일어나는 일입니다.
결과적으로 락이 자동으로 해제되기 때문에, 우리는 락을 해제하는 것을 잊어버리고
다른 스레드에 의해 뮤텍스가 사용되는 것을 막는 위험을 짊어지지 않아도
됩니다.

락이 버려진 후, 뮤텍스 값을 출력하여 내부의 `i32`를 6으로 바꿀 수 있음을
확인할 수 있습니다.

#### 여러 스레드들 사이에서 `Mutex<T>` 공유하기

이제 `Mutex<T>`를 사용하여 여러 스레드들 사이에서 값을 공유하는 시도를 해봅시다.
우리는 10개의 스레드를 돌리고 이들이 카운터 값을 1만큼씩 증가 시켜서,
카운터가 0에서 10으로 가도록 할 것입니다. 다음 몇 개의 예제가 컴파일
에러가 날 것이고, 우리가 이 에러를 사용하여 `Mutex<T>`를 사용하는 방법과
러스트가 이를 고치는 것을 어떻게 돕는지에 대해 학습할 것임을 주의하세요.
Listing 16-13이 시작 예제입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-13: `Mutex<T>`에 의해 보소되는 카운터를
각자 증가시키는 10개의 스레드</span>

Listing 16-12에서 했던 것처럼 `Mutex<T>` 내부에 `i32`를 담는 `counter`
변수를 만듭니다. 그 다음, 숫자 범위로 반복하여 10개의 스레드를 만듭니다.
우리는 `thread::spawn`을 사용하여 동일한 클로저를 모든 스레드에게 주었는데,
이 클로저는 스레드로 카운터를 이동시키고, `lock` 메소드를 호출함으로써 `Mutex<T>`의
락을 얻은 다음, 뮤텍스 내의 값을 1만큼 증가시킵니다. 스레드가 자신의 클로저 실행을
끝냈을 때, `num`은 스코프 밖으로 벗어내고 락이 해제되어 다른 스레드가 이를 얻을 수
있습니다.

메인 스레드 내에서, 우리는 모든 조인 핸들을 수입합니다. 그리고나서
Listing 16-2에서 했던 것과 같이, 각 핸들 상에 `join`을 호출하여
모든 스레드가 종료되는 것을 확실히 합니다. 이 시점에서, 메인 스레드는 락을 얻고
이 프로그램의 결과를 출력합니다.

이 예제가 컴파일되지 않는다는 힌트를 줬었죠. 이제 왜 그런지 알아봅시다!

```text
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:10:27
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:21:29
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

이 에러 메세지는 `counter` 값이 클로저 내부로 이동되어서 우리가 `lock`을
호출할 떄 캡처되었다고 설명합니다. 이 설명은 우리가 원하는 것처럼 들리지만,
허용되지 않습니다!

프로그램을 단순화하여 이를 알아내봅시다. 10개의 스레드를 `for` 루프 내에서
만드는 대신, 루프 없이 두 개의 스레드만 만들어서 어떤 일이 일어나는지 봅시다.
Listing 16-13의 첫번째 `for` 루프를 아래 코드로 바꿔 넣으세요:

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

우리는 두 개의 스레드를 만들고 두번째 스레드에서 사용되는 변수 이름을
`handle2`와 `num2`로 바꿨습니다. 이제 이 코드를 실행하면, 컴파일러가
우리에게 다음 에러 메세지를 줍니다:

```text
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:16:24
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:26:29
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

아하! 첫번째 에러 메세지는 `counter`가 `handle`과 연괸된 스레드에 대한 클로저
내부로 이동되었음을 나타냅니다. 이 이동이 우리가 두번째 스레드에서 `lock`의
호출을 시도하고 `num2`에 결과를 저장할 때 `counter`를 캡처하는 것을 방지합니다!
따라서 러스트는 우리가 `counter`의 소유권을 여러 스레드로 이동시킬 수 없음을
말하는 중입니다. 이는 더 읽찍 발견하기 어려운데 그 이유는 우리의 스레드들이
루프 내에 있었고, 러스트는 루프의 다른 반복 회체 내의 다른 스레드를 지적할 수
없기 때문입니다. 우리가 15장에서 다루었던 복수 소유자 메소드를 이용하여 이 컴파일에러를
고쳐봅시다.

#### 여러 스레드들과 함께하는 복수 소유권

15장에서 우리는 참조 카운팅 값을 만들기 위해 스마트 포인터 `Rc<T>`을
사용함으로써 값에게 복수의 소유권자를 주었습니다. 동일한 일을 여기서도 해서
어떻게 되는지 봅시다. Listing 16-14에에서 `Mutex<T>`를 `Rc<T>`로 감싸서
스레드로 소유권을 이동시키기 전에 이 `Rc<T>`를 복제하겠습니다. 이제는 우리가
에러를 봤으므로, `for` 로프를 이용하도록 다시 전환하고 클로저와 함꼐 쓴 `move`
키워드를 유지하겠습니다.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-14: 여러 스레드가 `Mutex<T>`를 소유할 수
있도록 `Rc<T>`를 사용하는 시도</span>

다시 한번 컴파일을 하고 그 결과가... 다른 에러들이네요! 컴파일러는 우리에게
많은 것을 가르치고 있습니다.

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied in `[closure@src/main.rs:11:36:
15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
  --> src/main.rs:11:22
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>`
cannot be sent between threads safely
   |
   = help: within `[closure@src/main.rs:11:36: 15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send` is
not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   = note: required because it appears within the type
`[closure@src/main.rs:11:36: 15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
```

와우, 이 에러는 정말 장황하네요! 여기 초점을 맞출 몇몇 중요한 부분이 있습니다:
첫번째 인라인 에러는 `` `std::rc::Rc<std::sync::Mutex<i32>>`는 스레드
사이에 안전하게 보내질 수 없다 `` 라고 말합니다. 이에 대한 이유는 초점을 맞출
그 다음 중요한 부분인 에러 메세지 내에 있습니다. 정제된 에러 메세지는 `` 트레잇 바운드
`Send`가 만족되지 않았다 `` 라고 말합니다. `Send`는 다음 절에서 얘기할 것입니다:
이것은 우리가 스레드와 함께 사용하는 타입들이 동시적 상황들 내에서 쓰이기 위한 것임을
확실히 하는 트레잇 중 하나입니다.

안타깝게도, `Rc<T>`는 스레드를 교차하면서 공유하기에는 안전하지 않습니다. `Rc<T>`가
참조 카운트를 관리할 때, 각각의 `clone` 호출마다 카운트에 더하고 각 클론이
버려질 때마다 카운트에서 제합니다. 하지만 그것은 다른 스레드에 의해 카운트를
변경하는 것을 방해할 수 없도록 확실히 하는 어떠한 동시성 기초 재료도 이용하지
않습니다. 이는 잘못된 카운트를 야기할 수 있습니다-결과적으로 메모리 누수를
발생시키거나 아직 다 쓰기 전에 값이 버려질 수 있는 교묘한 버그를 낳겠죠.
우리가 원하는 것은 정확히 `Rc<T>`와 비슷하지만 스레드-안전한 방식으로
참조 카운트를 바꾸는 녀석입니다.

#### Atomic Reference Counting with `Arc<T>`
#### `Arc<T>`을 이용하는 아토믹 (atomic) 참조 카운팅

다행히도, `Arc<T>`가 *바로* 동시적 상황에서 안전하게 사용할 수 있는 `Rc<T>`
타입입니다. *a*는 *아토믹 (atomic)* 을 의미하는데, 즉 이것이 *원자적으로
참조자를 세는 (atomically reference counted)* 타입임을 의미합니다.
아토믹은 우리가 여기서 자세히 다루지 않을 추가적인 동시성 기초 제료 종류입니다:
더 자세히 알고 싶으면 `std::sync::atomic`에 대한 표준 라이브러리 문서를 보세요.
이 시점에서 여러분은 아토믹이 기초 타입처럼 동작하지만 스레드를 교차하며 공유해도
안전하다는 것만 알면 됩니다.

그렇다면 여러분은 왜 모든 기초 타입이 아토믹하지 않은지, 그리고 표준 라이브러리 타입은 왜
기본적으로 `Arc<T>`을 구현에 이용하지 않는지를 궁금해 할런지도 모르겠습니다. 그 이유는
스레드 안전성이란 것이 여러분이 정말로 원할 때만 지불하고 싶을 성능 저하를 일으키기 
때문입니다. 만일 여러분이 단일 스레드 내의 값에 대한 연산만 수행하는 중이라면,
아토믹이 제공하는 보장을 강지하지 않아도 된다면 여러분의 코드는 더 빠르게 실행될
수 있습니다.

우리의 예제로 다시 돌아갑시다: `Arc<T>`와 `Rc<T>`는 같은 API를 가지고 있으므로,
우리는 `use`을 사용하는 라인과 `new` 호출, 그리고 `clone` 호출 부분을 바꾸는 것으로
프로그램을 수정합니다. Listing 16-15의 코드는 마침내 컴파일 및 실행이 될 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-15: `Arc<T>`를 사용하여 `Mutex<T>`를 감싸서
여러 스레드 사이에서 소유권을 공유할 수 있도록 하기</span>

이 코드는 다음을 출력할 것입니다:

```text
Result: 10
```

해냈군요! 우리는 0부터 10까지 세었고, 이는 그렇게 크게 인상적인 것 같이 않을런지도 모르겠지만,
우리에게 `Mutex<T>`와 스레드 안전성에 대하여 많은 것을 가르쳐 주었습니다. 여러분은 또한
이 프로그램의 구조를 사용하여 단지 카운터를 증가시키는 것 보다 더 복잡한 연산을 할 수도
있습니다. 이 전략을 사용하여, 여러분은 계산할 것을 독립적인 부분들로 나누고, 해당 부분들을
스레드로 쪼갠 다음, `Mutex<T>`를 사용하여 각 스레드가 해당 부분의 최종 결과를 갱신하도록
할 수 있습니다.

###`RefCell<T>`/`Rc<T>`와 `Mutex<T>`/`Arc<T>` 간의 유사성

여러분은 `counter`이 불변적이지만 이것 내부의 값에 대한 가변 참조자를 가지고 올 수
있었음을 알아챘을런지 모르겠습니다; 이는 `Mutex<T>`가 `Cell` 가족이 그러하듯
내부 가변성을 제공한다는 의미입니다. 우리가 15장에서 `Rc<T>`의 내용물을 변경할 수
있도록 하기 위해 `RefCell<T>`을 사용한 것과 같은 방식으로, `Arc<T>` 내부의
값을 변경하기 위해 `Mutex<T>`를 이용합니다.

주목할만한 또다른 세부 사항은 여러분이 `Mutex<T>`를 사용할 때 러스트가 여러분을
모든 종류의 논리적 에러로부터 보호해줄 수없다는 것입니다. 15장에서 `Rc<T>`를 사용하는
것은 두 `Rc<T>` 값들이 서로를 참조하여 메모리 누수를 야기하는 순환 참조자를 만들
위험성이 따라오는 것이었음을 상기하세요. 이와 유사하게, `Mutex<T>`는
*데드락 (deadlock)* 을 생성할 위험성이 따라옵니다. 이것은 어떤 연산이 두 개의
리소스에 대한 락을 얻을 필요가 있고 두 개의 스레드가 하나씩의 락을 얻는다면,
서로가 서로를 영원히 기다리는 식으로 발생됩니다. 여러분이 데드락에 흥미가 있다면,
데드락이 있는 러스트 프로그램 만들기를 시도해보세요; 그리고나서 어떤 언어에 있는
뮤텍스를 위한 데드락 완화 전략를 연구해보고 이를 러스트에서 구현해보세요.
`Mutex<T>`와 `MutexGuard`에 대한 표준 라이브러리 API 문서가
유용한 정보를 제공합니다.

이제 `Send`와 `Sync` 트레잇에 대해 얘기하고 커스텀 타입과 함께 어떻게 이용할 수 있는지에
대해 얘기하는 것으로 이 장을 마무리 하겠습니다.
