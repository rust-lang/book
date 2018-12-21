## `Drop` 트레잇은 메모리 정리 코드를 실행시킵니다

스마트 포인터 패턴에서 중요한 두번째 트레잇은 `Drop`인데, 이는 값이 스코프
밖으로 벗어나려고 할때 어떤 일이 발생될지를 커스터마이징하게끔 해줍니다. 우리는
어떠한 타입이든간에 `Drop` 트레잇을 위한 구현을 제공할 수 있고, 우리가 특정한
코드는 파일이나 네트워크 연결 같은 자원을 해제하는 데에 사용될 수 있습니다.
우리는 스마트 포인터의 맥락 안에서 `Drop`을 소개하고 있는데 그 이유는 `Drop`
트레잇의 기능이 언제나 대부분 스마트 포인터를 구현할 때에 사용되기 때문입니다.
예를 들면, `Box<T>`는 박스가 가리키고 있는 힙 상의 공간을 할당 해제하기 위해
`Drop`을 커스터마이징 합니다.

몇몇 언어들에서, 프로그래머는 스마트 포인터의 인스턴스 사용을 종료하는 매번마다
메모리 혹은 자원을 해제하기 위해 코드를 호출해야 합니다. 만일 이를 잊어먹으면,
그 시스템은 과부하가 걸리거나 멈출지도 모릅니다. 러스트에서는 값이 스코프 밖으로
벗어날 때마다 실행되어야 하는 특정한 코드 조각을 특정할 수 있고, 컴파일러는
이 코드를 자동으로 삽입해줄 것입니다. 결과적으로, 우리는 프로그램 내에서 특정한
타입의 인스턴스가 종료되는 곳마다 정리 코드를 집어넣는 것에 관한 걱정을 할
필요가 없지만, 여전히 자원 누수는 발생하지 않을 것입니다!

`Drop` 트레잇을 구현함으로서 값이 스코프 밖으로 벗어났을 때 실행될 코드를 특정합니다.
`Drop` 트레잇은 `self`에 대한 가변 참조자를 파라미터로 갖는 `drop` 이라는
이름의 하나의 메소드를 구현하도록 우리에게 요구합니다. 러스트가 언제 `drop`을
호출하는지 보기 위해서, 지금은 `println!` 구문과 함께 `drop`을 구현해봅시다.

Listing 15-4는 인스턴스가 스코프 밖으로 벗어낫을 때 `Dropping
CustomSmartPointer!`를 출력하는 커스텀 기능만을 갖춘 `CustomSmartPointer`
구조체를 보여주고 있습니다. 이 예제는 러스트가 `drop` 함수를 실행시키는
때를 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

<span class="caption">Listing 15-14: 우리의 정리 코드를 넣을 수 있는
`Drop` 트레잇을 구현한 `CustomSmartPointer` 구조체</span>

`Drop` 트레잇은 프렐루드에 포함되어 있으므로, 이를 가져오지 않아도 됩니다. 우리는
`CustomSmartPointer` 상에 `Drop` 트레잇을 구현하였고, `println!`을
호출하는 `drop` 메소드 구현을 제공했습니다. `drop` 함수의 본체는 여러분이 만든
타입의 인스턴스가 스코프 밖으로 벗어났을 때 실행시키고자 하는 어떠한 로직이라도
위치시킬수 있는 곳입니다. 우리는 여기서 러스트가 `drop`을 호출하게될 때를 보여주기
위해서 어떤 텍스트를 출력하는 중입니다.

`main`에서는 두 개의 `CustomSmartPointer` 인스턴스를 만든 다음
`CustomSmartPointers created.`를 출력합니다. `main`의 끝에서, 우리의 
`CustomSmartPointer` 인스턴스는 스코프 밖으로 벗어날 것이고, 러스트는 우리가
`drop` 메소드 내에 집어넣은 코드, 즉 우리의 마지막 메세지를 출력하는 코드를 호출할
것입니다. 우리가 `drop` 메소드를 명시적으로 호출할 필요가 없다는 점을 주의하세요.

이 프로그램을 실행시켰을 때, 다음과 같은 출력을 보게될 것입니다:

```text
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

러스트는 우리의 인스턴스가 스코프 밖으로 벗어났을 때 우리를 위하여 `drop`를
호출했고, 우리가 특정한 그 코드를 호출하게 됩니다. 변수들은 만들어진 순서의
역순으로 버려지므로, `d`는 `c` 전에 버려집니다. 이 예제는 여러분에게 `drop`
메소드가 어떻게 동작하는지에 대한 시각적인 가이드만을 제공하지만, 여러분은
보통 메세지 출력보다는 여러분의 타입이 실행할 필요가 있는 정리 코드를 특정할
것입니다.

### `std::mem::drop`을 이용하여 값을 일찍 버리기

불행하게도, 자동적인 `drop` 기능을 비활성화하는 것은 직관적이지 않습니다.
`drop` 비활성화는 보통 필요가 없습니다; `Drop` 트레잇의 전체적 관점은
자동적으로 다루어진다는 것입니다. 가끔, 여러분은 값을 일찍 정리하기를 원할
지도 모릅니다. 한가지 예는 락을 관리하는 스마트 포인터를 이용할 때입니다:
여러분은 실행할 락을 해제하는 `drop` 메소드를 강제로 실행시켜서 같은 스코프
내의 다른 코드가 락을 얻을 수 있길 원할지도 모릅니다. 러스트는 우리가 수동으로
`Drop` 트레잇의 `drop` 메소드를 호출하도록 해주지 않습니다; 대신 우리가
스코프 밖으로 벗어나기 전에 값이 강제로 버려질 원한다면 표준 라이브러리에서
제공하는 `std::mem::drop` 함수를 호출해야 합니다.

Listing 15-14의 `main` 함수를 Listing 15-15 처럼 수정하여
`Drop` 트레잇의 `drop` 메소드를 호출하려고 하면 어떤 일이 벌어지는지
봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Listing 15-15: 메모리 정리를 일찍하기 위해
`Drop` 트레잇으로부터 `drop` 메소드를 호출 시도하기</span>

이 코드의 컴파일을 시도하면, 다음과 같은 에러를 얻게됩니다:

```text
error[E0040]: explicit use of destructor method
  --> src/main.rs:14:7
   |
14 |     c.drop();
   |       ^^^^ explicit destructor calls not allowed
```

이 에러 메세지는 우리가 `drop`를 명시적으로 호출하는 것이 허용되지 않음을
기술하고 있습니다. 에러 메세지는 *소멸자 (destructor)* 라는 용어를 사용하는데,
이는 인스턴스를 정리하는 함수에 대한 일반적인 프로그래밍 용어입니다. *소멸자*는
인스턴스를 생성하는 *생성자 (constructor)* 와 비슷합니다. 러스트 내의 `drop`
함수는 특정한 형태의 소멸자입니다.

러스트는 우리가 `drop`을 명시적으로 호출하도록 해주지 않는데 이는 러스트가 `main`의
끝에서 값에 대한 `drop` 호출을 여전히 자동적으로 할 것이기 때문입니다. 이는 러스트가
동일한 값을 두번 메모리 정리를 시도할 수 있기 때문에 *중복 해제 (double free)* 에러가
될 수 있습니다.

우리는 값이 스코프 밖으로 벗어났을 때 자동적인 `drop` 추가를 비활성화 할 수
없고, `drop` 메소드를 명시적으로 호출할 수도 없습니다. 따라서, 값이 일찍 메모리
정리되도록 강제하길 원한다면, `std::mem::drop` 함수를 이용할 수 있습니다.


`std::mem::drop` 함수는 `Drop` 트레잇 내에 있는 `drop` 메소드와 다릅니다.
우리가 일찍 버리도록 강제하길 원하는 값을 인자로 넘김으로써 이를 호출할 수 있습니다.
이 함수는 프렐루드에 포함되어 있으므로, 우리는 Listing 15-14의 `main`을
Listing 15-16에서 보는 것처럼 수정할 수 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# struct CustomSmartPointer {
#     data: String,
# }
#
# impl Drop for CustomSmartPointer {
#     fn drop(&mut self) {
#         println!("Dropping CustomSmartPointer!");
#     }
# }
#
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

<span class="caption">Listing 15-16: 값이 스코프 밖으로 벗어나기 전에
명시적으로 버리기 위한 `std::mem::drop` 호출하기</span>

이 코드의 실행은 다음을 출력할 것입니다:

```text
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

```Dropping CustomSmartPointer with data `some data`!```라는 텍스트가
`CustomSmartPointer created.`와 `CustomSmartPointer dropped
before the end of main.` 사이에 출력되는데, 이는 `c`를 그 지점에서 버리기
위해 `drop` 메소드 코드가 호출되었음을 보여줍니다.

우리는 메모리 정리를 편리하고 안전하게 하기 위하여 `Drop` 트레잇 구현체 내에 특정된 코드를
다양한 방식으로 이용할 수 있습니다: 예를 들면, 이것을 우리만의 고유한 메모리 할당자를
만들기 위해 사용할 수도 있습니다! `Drop` 트레잇과 러스트의 소유권 시스템을 이용하면,
러스트가 메모리 정리를 자동적으로 수행하기 때문에 메모리 정리를 기억하지 않아도 됩니다.

우리는 또한 계속 사용중인 값이 뜻하지 않게 정리되는 것을 걱정하지 않아도 되는데,
그런 것은 컴파일 에러를 야기할 것이기 떄문입니다: 참조자가 항상 유효하도록 확실히
해주는 소유권 시스템은 또한 값이 더이상 사용되지 않을 때 `drop`이 오직 한번만
호출될 것을 보장합니다.

지금까지 `Box<T>`와 스마트 포인터의 몇가지 특성을 시험해 보았으니,
표준 라이브러리에 정의되어 있는 다른 몇 가지의 스마트 포인터를 살펴
봅시다.
