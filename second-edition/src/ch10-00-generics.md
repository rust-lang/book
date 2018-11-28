# 제네릭 타입, 트레잇, 그리고 라이프타임

모든 프로그래밍 언어는 컨셉의 복제를 효율적으로 다루기 위한 도구를 가지고 있습니다; 러스트에서,
그러한 도구 중 하나가 바로 *제네릭(generic)* 입니다. 제네릭은 구체화된 타입이나 다른 속성들에
대하여 추상화된 대리인입니다. 코드를 작성하고 컴파일할 때, 우리는 제네릭들이 실제로 어떻게 완성되는지
알 필요 없이, 제네릭의 동작 혹은 다른 제네릭과 어떻게 연관되는지와 같은 제네릭에 대한 속성을 표현할
수 있습니다.

여러 개의 구체화된 값들에 대해 실행될 코드를 작성하기 위해서 함수가 어떤 값을 담을지 알 수 없는 파라미터를
갖는 것과 동일한 방식으로, `i32`나 `String`과 같은 구체화된 타입 대신 몇몇 제네릭 타입의 파라미터를
갖는 함수를 작성할 수 있습니다. 우리는 6장의 `Option<T>`, 8장의 `Vec<T>`와 `HashMap<K, V>`,
그리고 9장의 `Result<T, E>`에서 이미 제네릭을 사용해 보았습니다. 이 장에서는, 어떤 식으로
우리만의 타입, 함수, 그리고 메소드를 제네릭으로 정의하는지 탐험해 볼 것입니다!

우선, 우리는 코드 중복을 제거하는 함수의 추출하는 원리에 대해 돌아볼 것입니다. 그러고 나서 두 함수가
오직 파라미터의 타입만 다른 경우에 대하여 이들을 하나의 제네릭 함수로 만들기 위해 동일한 원리를 사용할
것입니다. 또한 제네릭 타입을 구조체와 열거형의 정의에 사용하는 것을 살펴볼 것입니다.

그리고 난 후 *트레잇(trait)* 에 대하여 논의할 것인데, 이는 동작을 제네릭 한 방식으로 정의하는
방법을 말합니다. 트레잇은 제네릭 타입과 결합되어 제네릭 타입에 대해 아무 타입이나 허용하지 않고,
특정 동작을 하는 타입으로 제한할 수 있습니다.

마지막으로, 우리는 *라이프타임(lifetime)* 에 대해 다룰 것인데, 이는 제네릭의 일종으로서 우리가
컴파일러에게 참조자들이 서로에게 어떤 연관이 있는지에 대한 정보를 줄 수 있도록 해줍니다. 라이프타임은
수많은 상황에서 값을 빌릴 수 있도록 허용해 주고도 여전히 참조자들이 유효할지를 컴파일러가 검증하도록
해주는 러스트의 지능입니다.

## 함수를 추출하여 중복 없애기

제네릭 문법을 들어가기 전에, 먼저 제네릭 타입을 이용하지 않는 중복 코드 다루기 기술을 훑어봅시다: 바로
함수 추출하기죠. 이를 한번 우리 마음속에서 생생하게 상기시키고 나면, 우리는 제네릭 함수를 추출하기 위해
제네릭을 가지고 똑같은 수법을 이용할 것입니다! 여러분이 함수로 추출할 중복된 코드를 인식하는 것과 똑같은
방식으로, 여러분은 제네릭을 이용할 수 있는 중복된 코드를 인식하기 시작할 것입니다.

Listing 10-1과 같이 리스트에서 가장 큰 숫자를 찾아내는 작은 프로그램이 있다고 칩시다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
#  assert_eq!(largest, 100);
}
```

<span class="caption">Listing 10-1: 숫자 리스트 중에서 가장 큰 수를 찾는 코드</span>

이 코드는 정수의 리스트를 얻는데, 여기서는 변수 `numbers`에 저장되어 있습니다. 리스트의 첫 번째
아이템을 `largest`라는 이름의 변수에 우선 집어넣습니다. 그러고 나서 리스트 내의 모든 숫자들에 대해
반복 접근을 하는데, 만일 현재 숫자가 `largest` 내에 저장된 숫자보다 더 크다면, 이 숫자로
`largest` 내의 값을 변경합니다. 만일 현재 숫자가 여태까지 본 가장 큰 값보다 작다면, `largest`는
바뀌지 않습니다. 리스트 내의 모든 아이템을 다 처리했을 때, `largest`는 가장 큰 값을 가지고 있을
것인데, 위 코드의 경우에는 100이 될 것입니다.

만일 두 개의 서로 다른 숫자 리스트로부터 가장 큰 숫자를 찾기를 원한다면, Listing 10-1의 코드를
복사하여, Listing 10-2에서처럼 한 프로그램 내에 동일한 로직이 두 군데 있게 할 수도 있습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

<span class="caption">Listing 10-2: *두* 개의 숫자 리스트에서 가장 큰 숫자를
찾는 코드</span>

이 코드는 잘 동작하지만, 코드를 중복 적용하는 일은 지루하고 오류가 발생하기도 쉬우며, 또한 로직을
바꾸고 싶다면 이 로직을 갱신할 곳이 여러 군데가 된다는 의미이기도 합니다.

<!-- Are we safe assuming the reader will be familiar with the term
"abstraction" in this context, or do we want to give a brief definition? -->
<!-- Yes, our audience will be familiar with this term. /Carol -->

이러한 중복을 제거하기 위해서 우리는 추상화를 쓸 수 있는데, 이 경우에는 어떠한 정수 리스트가 함수의
파라미터로 주어졌을 때 동작하는 함수의 형태가 될 것입니다. 이는 우리 코드의 명료성을 증가시켜주고
리스트 내에서 가장 큰 수를 찾는 컨셉을 사용하는 특정한 위치와 상관없이 이러한 컨셉을 전달하고
추론하도록 해줍니다.

Listing 10-3의 프로그램에서는 가장 큰 수를 찾는 코드를 `largest`라는 이름의 함수로 추출했습니다.
이 프로그램은 두 개의 서로 다른 숫자 리스트에서 가장 큰 수를 찾을 수 있지만, Listing 10-1에서의
코드는 한 군데에서만 나타납니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
#    assert_eq!(result, 100);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
#    assert_eq!(result, 6000);
}
```

<span class="caption">Listing 10-3: 두 리스트에서 가장 큰 수를 찾는 추상화된 코드</span>

이 함수는 `list`라는 파라미터를 갖고 있는데, 이것이 함수로 넘겨질 구체적인 임의 `i32` 값들의
슬라이스를 나타냅니다. 함수 정의 내의 코드는 임의의 `&[i32]`의 `list` 표현에 대해 동작합니다.
`largest` 함수를 호출할 때, 이 코드는 실제로 우리가 넘겨준 구체적인 값에 대해 실행됩니다.

Listing 10-2에서부터 Listing 10-3까지 우리가 살펴본 원리는 아래와 같은 단계로 진행되었습니다:

1. 중복된 코드가 있음을 알아챘습니다.
2. 중복된 코드를 함수의 본체로 추출하고, 함수의 시그니처 내에 해당 코드의 입력값 및 반환 값을 명시했습니다.
3. 두 군데의 코드가 중복되었던 구체적인 지점에 함수 호출을 대신 집어넣었습니다.

우리는 다른 시나리오 상에서 다른 방식으로 제네릭을 가지고 중복된 코드를 제거하기 위해 같은 단계를 밟을
수 있습니다. 함수의 본체가 현재 구체적인 값 대신 추상화된 `list`에 대해 동작하고 있는 것과 같이,
제네릭을 이용한 코드는 추상화된 타입에 대해 작동할 것입니다. 제네릭으로 강화되는 컨셉은 여러분이 이미
알고 있는 함수로 강화되는 컨셉과 동일하며, 다만 다른 방식으로 적용될 뿐입니다.

만일 우리가 두 개의 함수를 가지고 있는데, 하나는 `i32`의 슬라이스에서 최댓값을 찾는 것이고 다른
하나는 `char` 값의 슬라이스에서 최댓값을 찾는 것이라면 어떨까요? 어떻게 하면 이런 중복을 제거할
수 있을까요? 한번 알아봅시다!
