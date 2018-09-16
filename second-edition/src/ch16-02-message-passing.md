## 메세지 패싱을 사용하여 스레드 간에 데이터 전송하기

안전한 동시성을 보장하는 인기 상승중인 접근법 하나는 *메세지 패싱 (message passing)*
인데, 이는 스레드들 혹은 액터들이 데이터를 담고 있는 메세지를 서로 주고받는
것입니다. [Go 언어 문서](http://golang.org/doc/effective_go.html)
의 슬로건에 있는 아이디어는 다음과 같습니다: "메모리를 공유하는 것으로 통신하지
마세요; 대신, 통신해서 메모리를 공유하세요"

러스트가 메세지 보내기 방식의 동시성을 달성하기 위해 갖춘 한가지 주요
도구는 *채널 (channel)* 인데, 이는 러스트의 표준 라이브러리가
구현체를 제공하는 프로그래밍 개념입니다. 프로그래밍에서의 채널은 개울이나
강 같은 물의 통로와 비슷하다고 상상할 수 있습니다. 만일 여러분이 고무
오리나 배 같은 것을 개울에 띄우면, 물길의 끝까지 하류로 여행하게 될
것입니다.

프로그래밍에서의 채널은 둘로 나뉘어져 있습니다: 바로 송신자(transmitter)와
수신자(receiver)입니다. 송신자 측은 여러분이 강에 고무 오리를 띄우는
상류 위치이고, 수신자 측은 하류에 고무 오리가 도달하는 곳입니다. 여러분
코드 중 한 곳에서 여러분이 보내고자 하는 데이터와 함꼐 송신자의 메소드를
호출하면, 다른 곳에서는 도달한 메세지에 대한 수신 종료를 검사합니다.
송신자 혹은 송신자가 드롭되면 채널이 *닫혔다 (closed)* 라고
말합니다.

여기서 우리는 값을 생성하여 채널로 내려보내는 한 스레드와, 값을 받아서
이를 출력하는 또다른 스레드를 가지고 있는 프로그램을 만들어볼 것입니다.
우리는 기능을 설명하기 위해서 채널을 사용해 스레드 간에 단순한 값들을
보내게 될 것입니다. 여러분이 이 기술에 익숙해지고 나면, 여러분은 채팅
시스템이나 다수의 스레드가 계산의 일부분을 수행하여 결과를 종합하는
하나의 스레드에 이를 보내는 시스템을 구현하기 위해 채널을 이용할 수
있습니다.

먼저 Listing 16-6에서는 채널을 만들지만 이걸 가지고 아무것도 하지 않을 것입니다.
우리가 채널을 통해 어떤 타입의 값을 보내는지에 대해 러스트에게 말하지 않았기 때문에
아직 컴파일되지 않는다는 점을 주의하세요.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
#     tx.send(()).unwrap();
}
```

<span class="caption">Listing 16-6: 채널을 생성하여 두 결과값을
`tx`와 `rx`에 할당하기</span>

우리는 `mpsc::channel` 함수를 사용하여 새로운 채널을 생성합니다; `mpsc`는
*복수 생성자, 단수 소비자 (multiple producer, single consumer)* 를
나타냅니다. 짧게 줄이면, 러스트의 표준 라이브러리가 채널을 구현한 방법은 한 채널이
값을 생성하는 복수개의 *송신* 단말을 가질 수 있지만 값을 소비하는 단 하나의
*수신* 단말을 가질 수 있음을 의미합니다. 하나의 큰 강으로 함께 흐르는 여러 개울들을
상상해 보세요: 개울 중 어떤 쪽에라도 흘려보낸 모든 것은 끝에 하나의 강에서 끝날
것입니다. 지금은 단일 생성자를 가지고 시작하겠지만, 이 예제가 동작하기 시작하면
여러 생성자를 추가할 것입니다.

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

`mpsc::channel` 함수는 튜플을 반환하는데, 첫번째 요소는 송신 단말이고
두번째 요소는 수신 단말입니다. `tx`와 `rx`라는 약어는 많은 분야에서 각각
*송신자 (transmitter)* 와 *수신자 (receiver)* 를 위해 사용하므로,
각각의 단말을 가리키기 위해 그렇게 변수명을 지었습니다. 우리는 튜플을 해체하는
패턴과 함께 `let` 구문을 사용하는 중입니다; `let` 구문 내에서의 패턴의
사용과 해체에 대해서는 18장에서 다룰 것입니다. 이런 방식으로 `let` 구문을
사용하는 것은 `mpsc::channel`이 반환하는 튜플의 조각들을 추출하는데
편리한 접근법입니다.

Listing 16-7에서 보는 바와 같이 송신 단말을 생성된 스레드로 이동시키고
하나의 스트링을 전송하게 하여 생성된 스레드가 메인 스레드와 통신하도록
해봅시다. 이는 강 상류에 고무 오리를 띄우는 것 혹은 한 스레드에서 다른
스레드로 채팅 메세지를 보내는 것과 비슷합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

<span class="caption">Listing 16-7: `tx`를 생성된 스레드로 이동시키고
“hi”를 보내기</span>

다시 한번 `thread::spawn`을 이용하여 새로운 스레드를 생성한 뒤 `move`를
사용하여 `tx`를 클로저로 이동시켜 생성된 스레드가 `tx`를 소유하도록 합니다.
생성된 스레드는 채널을 통해 메세지를 보낼 수 있도록 하기 위해 채널의 송신 단말을
소유할 필요가 있습니다.

송신 단말은 우리가 보내고 싶어하는 값을 취하는 `send` 메소드를 가집니다.
`send` 메소드는 `Result<T, E>` 타입을 반환하므로, 만일 수신 단말이
이미 드롭되어 있고 값을 보내는 곳이 없다면, 송신 연산은 에러를 반환할 것입니다.
이 예제에서는 에러가 나는 경우 패닉을 일으키기 위해 `unwrap`을 호출하는
중입니다. 그러나 실제 애플리케이션에서는 이를 적절히 다뤄야 할 것입니다:
적절한 에러 처리를 위한 전략을 다시 보려면 9장으로 돌아가세요.

Listing 16-8에서 우리는 메인 스레드에 있는 채널의 수신 단말로부터 값을
받을 것입니다. 이는 강의 끝물에서 고무 오리를 건져올리는 것 혹은 채팅
메세지를 받는 것과 비슷합니다.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

<span class="caption">Listing 16-8: 메인 스레드에서 “hi” 값을
받아 출력하기</span>

채널의 수신 단말은 두 개의 유용한 메소드를 가지고 있습니다: `recv`와 `try_recv`
입니다. 우리는 *수신 (receive)* 의 줄임말인 `recv`를 사용하는 중인데, 이는 메인
스레드의 실행을 블록시키고 채널로부터 값이 보내질 때까지 기다릴 것입니다. 값이 일단
전달되면, `recv`는 `Result<T, E>` 형태로 이를 반환할 것입니다. 채널의 송신
단말이 닫히면, `recv`는 더 이상 어떤 값도 오지 않을 것이란 신호를 하는 에러를
반환할 것입니다.

`try_recv` 메소드는 블록하지 않는 대신 즉시 `Result<T, E>`를
반환합니다: 전달 받은 메세지가 있다면 이를 담고 있는 `Ok` 값을, 이
시점에서 메세지가 없다면 `Err` 값을 반환합니다. `try_recv`를 사용하는
것은 메세지를 기다리는 동안 해야 하는 다른 작업이 있을 때 유용합니다:
`try_recv`을 매번마다 호출하여, 가능한 메세지가 있으면 이를 처리하고,
그렇지 않으면 다음번 검사때까지 잠시동안 다른 일을 하는 루프를 만들 수
있습니다.

이 예제에서는 단순함을 위해 `recv`를 이용했습니다; 이 메인 스레드에서는
메세지를 기다리는 동안 해야 할 다른 일이 없으므로, 메인 스레드를 블록시키는
것이 적절합니다.

Listing 16-8의 코드를 실행하면, 메인 스레드로부터 출력된 값을 보게
될 것입니다:

```text
Got: hi
```

완벽하군요!

### 채널과 소유권 전달

소유권 규칙은 여러분들이 안전하고 동시적인 코드를 작성하는 것을 돕기 때문에
메세지 보내기 방식 내에서 강건한 역할을 합니다. 동시성 프로그래밍 내에서 에러를
방지하는 것은 여러분의 러스트 프로그램 전체에 걸친 소유권에 대한 생각해볼 수 있는
장점이 있습니다. 어떤 식으로 채널과 소유권이 문제를 방지하기 위해 함께 동작하는지를
보기 위한 실험을 해봅시다: 우리가 채널로 `val` 값을 내려보낸 *이후에* 생성된
스레드에서 이 값을 사용하는 시도를 해볼 것입니다. Listing 16-9의 코드를 컴파일하여
이 코드가 왜 허용되지 않는지를 보세요:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

<span class="caption">Listing 16-9: `val`을 채널로 내려보낸 뒤
이에 대한 사용 시도</span>

여기서는 `tx.send`를 통하여 채널에 `val`을 내려보낸 뒤 이를 출력하는 시도를
하였습니다. 이 코드를 허용하는 것은 나쁜 생각입니다: 일단 값이 다른 스렏로 보내지고
나면, 우리가 값을 다시 사용해보기 전에 그 스레드에서 수정되거나 버려질 수 있습니다.
잠재적으로, 다른 스레드에서의 수정은 불일치하거나 존재하지 않는 데이터로 인한 에러를
일으킬 수 있습니다. 그러나, 우리가 Listing 16-9의 코드를 컴파일 시도하면 러스트는
에러를 내놓습니다:

```text
error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
not implement the `Copy` trait
```

우리의 동시성에 관한 실수가 컴파일 타임 에러를 야기했습니다. `send` 함수가
그 파라미터의 소유권을 가져가고, 이 값이 이동될 때, 수신자가 이에 대한 소유권을
얻습니다. 이는 우리가 값을 보낸 이후에 우발적으로 이 값을 다시 사용하는 것을
방지합니다; 소유권 시스템은 모든게 정상인지 확인합니다.

### 복수의 값들을 보내고 수신자가 기다리는지 보기

Listing 16-8의 코드는 컴파일되고 실행도 되지만, 두개의 분리된 스레드가
채널을 통해 서로 대화를 했는지를 우리에게 명확히 보여주진 못했습니다.
Listing 16-10에서는 Listing 16-8의 코드가 동시에 실행된다는 것을
입증해 중 수정본을 만들었습니다: 이제 생성된 스레드가 여러 메세지를 보내면서
각 메세지 사이에 1초씩 잠깐 멈출 것입니다.

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

<span class="caption">Listing 16-10: 여러 메세지를 보내고 각
사이마다 멈추기</span>

이번에 생성된 스레드는 우리가 메인 스레드로 보내고 싶어하는 스트링의 벡터를 
가지고 있습니다. 스트링마다 반복하여 각각의 값을 개별적으로 보내고,
`Duration` 값에 1을 넣어서 `thread::sleep` 함수를 호출하는 것으로 각각의
사이에 멈춥니다.

메인 스레드에서는 더 이상 `recv` 함수를 명시적으로 호출하지 않고 있습니다:
대신 `rx`를 반복자처럼 다루고 있습니다. 각각의 수신된 값에 대해서 이를
출력합니다. 채널이 닫힐 때는 반복이 종료될 것입니다.

Listing 16-10의 코드를 실행시키면 다음과 같은 출력이 각 줄마다 1초씩
멈추면서 보일 것입니다:

```text
Got: hi
Got: from
Got: the
Got: thread
```

메인 스레드의 `for` 루프 내에는 어떠한 멈춤 혹은 지연 코드를 넣지 않았으므로,
우리는 메인 스레드가 생성된 스레드로부터 값을 전달받는 것을 기다리는 중이라고
말할 수 있습니다.

### 송신자를 복제하여 여러 생성자 만들기

이전에 `mpsc`가 *복수 생성자 단일 소비자 (multiple producer, single consumer)*
의 약어라는 것을 언급했었지요. `mpsc`를 Listing 16-10의 코드에 넣어 모두 동일한
수신자로 값들을 보내는 여러 스레드들을 만들도록 코드를 확장해봅시다. Listing 16-11에서
보시는 것처럼 채널의 송신자를 복제하는 것으로 그렇게 할 수 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
# use std::time::Duration;
#
# fn main() {
// --snip--

let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
# }
```

<span class="caption">Listing 16-11: 여러 개의 생성자로부터 여러 메세지
보내기</span>

이번에는 우리가 첫번째 스레드를 생성하기 전에, 채널의 송신 단말에
대해 `clone`을 호출했습니다. 이는 우리에게 첫번째 생성된 스레드로
값을 보낼 수 있는 새로운 송신 핸들을 제공해줄 것입니다. 두번째 생성된
스레드에게는 원래의 채널 송신 단발을 넘깁니다. 이렇게 함으로써 각각이
다른 메세지를 채널의 수신 단말로 보내주는 두 스레드를 만듭니다.

여러분이 이 코드를 실행시키면, 다음과 같은 출력과 비슷하게 보여야 합니다:

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

값들의 순서가 다르게 보일 수도 있습니다; 이는 여러분의 시스템에 따라 다릅니다.
이것이 바로 동시성을 흥미로울 뿐만 아니라 어렵게 만드는 것입니다. 만일 여러분이
`thread::sleep`을 가지고 실험하면서 서로 다른 스레드마다 다양한 값을 썼다면,
각각의 실행이 더욱 비결정적이고 매번 다른 출력을 생성할 것입니다.

이제 채널이 어떤 식으로 동작하는지 보았으니, 동시성을 위한 다른 방법을
알아봅시다.
