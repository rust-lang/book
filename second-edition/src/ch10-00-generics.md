# 제네릭 타입, 트레잇, 그리고 라이프타임

모든 프로그래밍 언어는 컨셉의 복제를 효율적으로 다루기 위한 도구를 가지고 있습니다; 러스트에서,
그러한 도구중 하나가 바로 *제네릭(generic)* 입니다. 제네릭은 구체화된 타입이나 다른 속성들에
대하여 추상화된 대리인입니다. 코드를 작성하고 컴파일할 때, 우리는 제네릭들이 실제로 어떻게 완성되는지
알 필요없이, 제네릭의 동작 혹은 다른 제네릭과 어떻게 연관되는지와 같은 제네릭에 대한 속성을 표현할
수 있습니다.

여러 개의 구체화된 값들에 대해 실행될 코드를 작성하기 위해서 함수가 어떤 값을 담을지 알 수 없는 파라미터를
갖는 것과 동일한 방식으로, `i32`나 `String`과 같은 구체화된 타입 대신 몇몇 제네릭 타입의 파라미터를
갖는 함수를 작성할 수 있습니다. 우리는 6장의 `Option<T>`, 8장의 `Vec<T>`와 `HashMap<K, V>`,
그리고 9장의 `Result<T, E>`에서 이미 제네릭을 사용해 보았습니다. 이 장에서는, 어떤 식으로
우리만의 타입, 함수, 그리고 메소드를 제네릭으로 정의하는지 탐험해 볼 것입니다!

우선, 우리는 코드 중복을 제거하는 함수의 추출하는 원리에 대해 돌아볼 것입니다. 그리고나서 두 함수가
오직 파라미터의 타입만 다른 경우에 대하여 이들을 하나의 제네릭 함수로 만들기 위해 동일한 원리를 사용할
것입니다. 또한 제네릭 타입을 구조체와 열거형의 정의에 사용하는 것을 살펴볼 것입니다.

그리고 난 후 *트레잇(trait)* 에 대하여 논의할 것인데, 이는 동작을 제네릭한 방식으로 정의하는
방법을 말합니다. 트레잇은 제네릭 타입과 결합되어 제네릭 타입에 대해 아무 타입이나 허옹하지 않고,
특정 동작을 하는 타입으로 제한할 수 있습니다.

마지막으로, 우리는 *라이프타임(lifetime)* 에 대해 다룰 것인데, 이는 제네릭의 일종으로서 우리가
컴파일러에게 참조자들이 서로에게 어떤 연관이 있는지에 대한 정보를 줄 수 있도록 해줍니다. 라이프타임은
수많은 상황에서 값을 빌릴 수 있도록 허용해 주고도 여전히 참조자들이 유효할지를 컴파일러가 검증하도록
해주는 러스트의 지능입니다.

## 함수를 추출하여 중복 없애기

제네릭 문법을 들어가기 전에, 먼저 제네릭 타입을 이용하지 않는 중복 코드 다루기 기술을 훑어봅시다: 바로
함수 추출하기죠. 이를 한번 우리 마음 속에서 생생하게 상기시키고 나면, 우리는 제네릭 함수를 추출하기 위해
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

이 코드는 정수의 리스트를 얻는데, 여기서는 변수 `numbers`에 저장되어 있습니다. 리스트의 첫번째
아이템을 `largest`라는 이름의 변수에 우선 집어넣습니다. 그리고나서 리스트 내의 모든 숫자들에 대해
반복 접근을 하는데, 만일 현재 숫자가 `largest` 내에 저장된 숫자보다 더 크다면, 이 숫자로
`largest` 내의 값을 갱신합니다. 만일 현재 숫자가 여태까지 본 가장 큰값보다 작다면, `largest`는
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

이 코드는 잘 동작하지만, 코드를 복제하는 일은 지루하고 오류가 발생하기도 쉬우며, 또한 로직을 바꾸고
싶다면 이 로직을 갱신할 곳이 여러 군데가 된다는 의미이기도 합니다.

<!-- Are we safe assuming the reader will be familiar with the term
"abstraction" in this context, or do we want to give a brief definition? -->
<!-- Yes, our audience will be familiar with this term. /Carol -->

To eliminate this duplication, we can create an abstraction, which in this case
will be in the form of a function that operates on any list of integers given
to the function in a parameter. This will increase the clarity of our code and
let us communicate and reason about the concept of finding the largest number
in a list independently of the specific places this concept is used.

In the program in Listing 10-3, we've extracted the code that finds the largest
number into a function named `largest`. This program can find the largest
number in two different lists of numbers, but the code from Listing 10-1 only
exists in one spot:

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

<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>

The function has a parameter, `list`, which represents any concrete slice of
`i32` values that we might pass into the function. The code in the function
definition operates on the `list` representation of any `&[i32]`. When we call
the `largest` function, the code actually runs on the specific values that we
pass in.

The mechanics we went through to get from Listing 10-2 to Listing 10-3 were
these steps:

1. We noticed there was duplicate code.
2. We extracted the duplicate code into the body of the function, and specified
   the inputs and return values of that code in the function signature.
3. We replaced the two concrete places that had the duplicated code to call the
   function instead.

We can use these same steps with generics to reduce code duplication in
different ways in different scenarios. In the same way that the function body
is now operating on an abstract `list` instead of concrete values, code using
generics will operate on abstract types. The concepts powering generics are the
same concepts you already know that power functions, just applied in different
ways.

What if we had two functions, one that found the largest item in a slice of
`i32` values and one that found the largest item in a slice of `char` values?
How would we get rid of that duplication? Let's find out!
