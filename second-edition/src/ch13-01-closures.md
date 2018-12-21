## 클로저: 환경을 캡처할 수 있는 익명 함수

러스트의 *클로저*는 변수에 저장하거나 다른 함수에 인자로 넘길 수 있는 익명 함수입니다.
한 곳에서 클로저를 만들고 다른 문맥에서 그것을 평가하기 위해 호출할 수
있습니다. 함수와 다르게 클로저는 그들이 호출되는 스코프로부터 변수들을 캡처할
수 있습니다. 이 클로저 특성이 코드 재사용과 동작 사용자 정의를 어떤 식으로
허용하는지 예를 들어 보여줄 것입니다.

### 클로저로 행위를 추상화 하기

클로저를 나중에 실행하기 위해 저장하는 것이 유용한 상황에 대한 예제로 작업해
봅시다. 따라가다 보면, 클로저 문법과 타입 추론, 트레잇에 대해 이야기할 것입니다.

이런 가상의 상황을 생각해 봅시다: 우리는 맞춤 운동 계획을 생성하는 앱을
만드는 스타트업에서 일합니다. 백엔드는 러스트로 작성되어 있고, 운동 계획을 생성하는
알고리즘은 앱 사용자의 나이, 체질량 지소, 선호도, 최근 운동들과 그들이
지정한 강도 숫자와 같은 많은 다른 요소들을 고려합니다. 이 예제에서 사용되는
실제 알고리즘은 중요하지 않습니다; 중요한 것은 이 알고리즘이 몇 초가 걸린다는
것입니다. 이 알고리즘을 우리가 필요할 때 한 번만 호출하기를 원하고, 그래서 사용
자가 필요 이상으로 기다리지 않게 만들고 싶습니다.

우리는 리스트 13-1 에 보이는 `simulated_expensive_calculation` 함수를 사용해서
이 가상의 알고리즘 호출을 실험할 것입니다. 이 함수는 `calculating slowly...` 을
출력하고, 2초를 기다린 다음, 인자로 넘어온 어떤 값이든 돌려줍니다:

<span class="filename">파일명: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

<span class="caption">리스트 13-1: 실행시간이 2초 걸리는 가상의 계산을 대신하는
함수</span>

다음은 이 예제에서 중요한 운동 앱의 일부를 담고 있는 `main` 함수 입니다.
이 함수는 사용자가 운동 계획을 물어볼 때 앱이 호출 할 코드를 나타냅니다.
앱의 프론트엔드와의 상호작용은 클로저를 사용하기에 적합하지 않기 때문에, 우리
프로 그램에 대한 입력을 나타내는 값을 코드상에 넣어두고 결과를 출력 할 것
입니다.

필요한 입력들은:

* *사용자로 부터의 강도 숫자*, 이것은 그들이 운동을 요청할 때 지정되며, 낮은
  강도 운동을 원하는지 혹은 고강도 운동을 원하는지를 나타냅니다.
* *임의의 숫자*는 몇 가지 다양한 운동 계획들을 생성할 것입니다.

결과는 추천 운동 계획이 될 것입니다. 리스트 13-2 에 우리가 사용할 `main` 함수
가 있습니다:

<span class="filename">파일이름: src/main.rs</span>

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
# fn generate_workout(intensity: u32, random_number: u32) {}
```

<span class="caption">리스트 13-2:사용자 입력과 임의의 숫자 생성을 시뮬레이션
하기 위한  `main` 함수와 하드코딩된 값</span>

단순함을 위해서 `simulated_user_specified_value` 변수의 값을 10 으로하고
`simulated_random_number` 변수의 값을 7로 하드코딩 했습니다; 실제 프로그램에서,
강도 숫자를 앱 프론트엔드에서 얻고 2장의 추리게임에서 그랬던 것 처럼, 임의의
숫자 생성을 위해 `rand` 크레이트를 사용합니다. `main` 함수는 `generate_workout`
함수를 모의의 입력값으로 호출 합니다.

이제 상황이 만들어 졌으니, 알고리즘으로 넘어가겠습니다. 리스트 13-3 에 있는
`generate_workout` 함수는 이 예제에서 가장 신경써야 할 앱의 비즈니스 로직을
포함하고 있습니다. 이 예제에서 나머지 코드를 변경 사항은 이 함수에 적용 됩니다:

<span class="filename">파일이름: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: u32) -> u32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

<span class="caption">리스트 13-3: 입력값과 `simulated_expensive_calculation` 함
수 호출에 근거해서 운동 계획을 출력하는 비즈니스 로직</span>

리스트 13-3 의 코드는 느린 계산 함수에 대해 여려번 호출을 합니다.
첫번째 `if` 블럭은 `simulated_expensive_calculation` 함수를 두번 호출하고,
바깥 `else` 의 안쪽에 있는 `if` 문에서는 전혀 호출하지 않으며, 두번째 `else` 문
의 경우는 한번 호출 합니다.

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

`generate_workout` 함수의 바람직한 행위는 먼저 사용자가 저강도 운동(25보다 작은
수로 표시) 혹은 고강도 운동(25 혹은 더 큰수)을 원하는지 체크하는 것입니다.

저강도 운동 계획은 우리가 시뮬레이션 하는 복잡한 알고리즘에 근거에서 푸쉬업과
싯업의 수를 추천 할 것입니다.

사용자가 고강도 운동을 원한다면, 약간의 추가 로직이 있습니다: 앱에 의해 생성된
임의의 숫자가 3이면, 앱은 휴식과 수분 섭취를 추천합니다. 그렇지 않다면, 사용자는
복잡한 알고리즘을 기반으로 몇 분의 달리기를 안내 받을 것입니다.

데이터 과학팀은 앞으로 알고리즘 호출 방식을 일부 변경해야 한다고 알렸습니다.
이러한 변경이 발생 했을 때 업데이트를 단순화 하기 위해서, 이 코드를 리팩토링
하여 `simulated_expensive_calculation` 함수를 단지 한 번만 호출 하도록 하려고
합니다. 또한 현재 프로세스에서 해당 함수에 대한 다른 호출을 추가하지 않고
불필요하게 함수를 두 번 호출하는 위치 없애고 싶습니다. 즉, 결과가 필요없다면
함수를 호출하고 싶지 않고, 여전히 그것을 한 번만 호출하고 싶습니다.

#### 함수를 사용해서 리팩토링 하기

우리는 여러 방향으로 운동 프로그램을 다시 구조화 할 수 있습니다. 우선, 리스트
13-4 에 보이는 것처럼, 중복된 `expensive_calculation` 함수 호출을 하나의 
변수로 추출 해볼 것입니다:

<span class="filename">파일이름: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# fn simulated_expensive_calculation(num: u32) -> u32 {
#     println!("calculating slowly...");
#     thread::sleep(Duration::from_secs(2));
#     num
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

<span class="caption">리스트 13-4: `simulated_expensive_calculation` 에 대한
호출들을 한 곳으로 추출하고 결과를 `expensive_result` 변수에 저장하기.</span>

이 변경은 `simulated_expensive_calculation` 에 대한 모든 호출들을 하나로 합치고
첫번째 `if` 문에서 불필요하게 이 함수를 여러번 호출하던 문제를 해결 합니다.
불행하게도, 이제 모든 경우에 대해서 이 함수를 호출하고 결과를 기다리며,
이 결과를 전혀 사용하지 않는 안쪽 `if` 블럭도 해당됩니다.

우리는 프로그램에서 한곳에서 코드를 정의하고, 실제로 결과가 필요한 곳에서만
그 코드를 *실행하고* 싶습니다. 이것이 클로저의 유스 케이스 입니다.

#### 코드를 저장하기 위해 클로저를 사용해서 리팩토링 하기.

`if` 블럭 전에 항상 `simulated_expensive_calculation` 함수를 호출하는 대신,
리스트 13-5에 보이는 것 처럼, 클로저를 정의하고 변수에 결과를 저장하기 보단
*클로저*를 변수에 저장 할 수 있습니다. 여기서 소개하는 것처럼 실제로 클로저 안에
`simulated_expensive_calculation` 의 전체 내용을 옮길 수 있습니다.

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
# expensive_closure(5);
```

<span class="caption">리스트 13-5: 클로저를 정의하고 `expensive_closure` 변수에
저장하기</span>

클로저 정의는 변수 `expensive_closure` 에 그것을 할당하기 위해 `=` 다음에
옵니다. 클로저를 정의하기 위해, 수직의 파이프 (`|`) 한쌍으로 시작하며, 그 사이에
클로저에 대한 파라미터를 기술합니다; 이 문법은 스몰토크와 루비에서 클로저
정의와의 유사성 때문에 선택 되었습니다. 이 클로저는 `num` 이라는 하나의
파라미터를 갖습니다: 하나 이상의 파라미터를 갖는다면, `|param1, param2|` 와 같이
콤마로 구분합니다.

파라미터들 다음에, 클로저의 바디를 포함하는 중괄호를 넣습니다—클로저 바디가
하나의 표현식이라면 이것은 선택적 입니다. 중괄호 다음에 클로저의 끝에는 `let`
문을 완성하기 위해 세미콜론이 필요합니다. 클로저 바디에서 마지막 줄로부터
반환되는 값인 (`num`) 은 그것이 호출되었을 때 클로저로 부터 반환되는 값이 될
것입니다, 왜냐하면 그 줄은 함수 본문 처럼 세미콜론으로 끝나지 않기 때문 입니다.

`let` 문은 `expensive_closure` 가 익명함수의 *정의*를 포함하며, 익명함수를
호출한 *결과 값*을 포함하지 않는다는 것에 유의 하세요. 우리가 클로저를 사용하는
이유는 호출할 코드를 한 곳에서 정의하고, 그 코드를 저장하며, 이후 다른 곳에서
그것을 호출하길 원하기 때문이라는 것을 상기하세요; 우리가 호출하고자 하는 코드가
이제 `expensive_closure` 에 저장되었습니다.

클로저를 정의하면서, 저장된 코드를 실행하고 결과값을 얻기 위하여 `if` 블록 안의
코드를 클로저 호출 방식으로 변경할 수 있습니다. 우리는 함수를 호출하는 것 처럼
클로저를 호출 합니다: 리스트 13-6 에 보이는 것처럼, 클로저 정의를 갖고 있는
변수명을 쓰고 다음엔 사용할 인자값을 포함하는 괄호가 따라 옵니다:

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```

<span class="caption">리스트 13-6: 우리가 정의한 `expensive_closure` 호출하기
</span>

이제 비용이 큰 계산은 단 한곳에서만 호출 되고, 우리가 결과가 필요한 곳에서만
그 코드를 실행 합니다.

그러나, 리스트 13-3 에 있는 문제중 하나를 다시 소개합니다: 우리는 여전히 첫번째
`if` 블럭에서 클로저를 두번 호출 하는데, 이는 비용이 큰 코드를 두번 호출하고
사용자가 실행시간 만큼 긴시간을 두번 기다리게 합니다. 우리는 그 `if` 블럭안에
클로저 호출의 결과를 저장하는 로컬 변수를 만들어서 그 문제를 해결할 수 있지만,
클로저는 다른 해결책을 제공합니다. 우리는 그 해결책에 대해 조금 이야기할
것입니다. 그러나 우선 클로저 정의에 타입 어노테이션이 없는 이유와 클로저와
연관된 트레잇에 대해 이야기 합시다.

### 클로저 타입 추론과 어노테이션

클로저는 `fn` 함수처럼 파라미터나 반환값의 타입을 명시할 것을 요구하지 않습니다.
타입 어노테이션은 사용자에게 노출되는 명시적인 인터페이스의 일부이기 때문에
함수에 필요 합니다. 이 인터페이스를 엄격하게 정의하는 것은 함수가 어떤 타입의
값을 사용하고 반환하는지에 대해 모두가 합의 한다는 것을 보장하는데 중요 합니다.
그러나 클로저는 이와 같이 노출된 인터페이스에 사용되지 않습니다: 변수에 저장되고
이름없이 우리의 라이브러리 사용자들에게 노출되지 않고 사용 됩니다.

추가적으로, 클로저는 보통 짧고 임의의 시나리오 보다 좁은 문맥 안에서만 관련이
있습니다. 이런 제한된 문맥 안에서만, 컴파일러는 안정적으로 파라미터와 리턴타입을
추론할 수 있으며, 이는 대부분의 변수 타입을 추론 할 수 있는 방법과 비슷 합니다.

프로그래머들에게 이런 작고 익명의 함수들에 타입을 달도록하는 것은 짜증나고
컴파일러가 이미 사용할수 있는 정보와 대게는 중복 됩니다.

변수처럼, 엄밀하게 필요한 것 이상으로 자세히 표현하는 비용을 지불하고서라도
명확성과 명료성을 높이고 싶다면 타입 어노테이션(혹은 타입 명시)를 추가할 수
있습니다;
리스트 13-4 에 정의한 클로저에 타입을 명시하는 것은 리스트 13-7 에 보이는 것과
같을 것입니다:

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

<span class="caption">리스트 13-7: 클로저에 파라미터와 반환값 타입에 대한 선택적
인 타입 어노테이션 추가하기</span>

타입 어노테이션이 있으면 클로저와 함수의 문법은 더 비슷해 보입니다.
다음은 파라미터에 1을 더하는 함수 정의와 동일한 행위를 하는 클로저를 수직으로
비교한 것입니다. 관련 있는 부분들을 정렬하기 이해 약간의 공백을 추가했습니다.
이것은 파이프를 사용하는 것과 선택적인 문법의 양을 제외하고 클로저 문법과 함수
문법이 얼마나 비슷한지 보여줍니다:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

첫번째 줄은 함수 정의를 보여주고, 두번째 줄은 타입을 모두 명기한 클로저 정의를
보여 줍니다. 세번째 줄은 클로저 정의에서 타입 어노테이션을 지웠고, 네번째 줄은
선택적인 중괄호를 지웠는데, 클로저 보디가 단 하나의 표현식을 갖기 때문 입니다.
이것은 모두 호출 했을 때 동일한 행위를 수행하는 유효한 정의들 입니다.

클로저 정의는 각 파리미터들과 그들의 반환값에 대해 단 하나의 추론된 구체적인
타입을 갖을 것입니다. 예를 들면, 리스트 13-8 은 파리미터로 받은 값을 그대로
반환하는 짧은 클로저의 정의를 보여줍니다. 이 클로저는 이 예제의 목적 이에외는
유용하지 않습니다. 정의에 타입 어노테이션을 추가하지 않았다는 것에 유의하세요:
클로저를 두번 호출하는데, 첫번째는 `String` 을 인자로 사용하고 두번째는 `u32` 을
사용한다면 에러가 발생합니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

<span class="caption">리스트 13-8: 두개의 다른 타입으로 추론된 타입을 갖는
클로저 호출 해보기</span>

컴파일러는 이런 에러를 줍니다:

```text
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integral variable
  |
  = note: expected type `std::string::String`
             found type `{integer}`
```

처음 `String` 값으로 `example_closure` 을 호출하면, 컴파일러는 `x` 의 타입과
클로저의 반환 타입을 `String` 으로 추론합니다. 이 타입들은 그다음에는
`example_closure` 에 있는 클로저에 고정되고, 같은 클로저를 다른 타입으로 
사용하려고 할 때 타입 에러를 얻게 됩니다.

### 제너릭 파라미터와 `Fn` 트레잇을 사용하여 클로저 저장하기

운동 생성 앱으로 돌아갑시다. 리스트 13-6 에서, 우리의 코드는 아직도 비용이 큰
계산을 하는 클로저를 필요한 것 보다 더 많이  호출 합니다. 이 문제를 풀기위한
한가지 옵션은 비싼 비용의 클로저 결과를 재활용을 위해 변수에 저장하고 결과가
필요한 부분에서 클로저를 다시 호출하는 대신 그 변수를 사용하는 것입니다.
그러나, 이 방법은 많은 반복된 코드를 만들 수 있습니다.

운 좋게도, 다른 해결책이 있습니다. 우리는 클로저와 클로저를 호출한 결과값을
갖고 있는 구조체를 만들 수 있습니다. 그 구조체는 결과값을 필요로 할 때만
클로저를 호출 할 것이며, 결과값을 캐시에 저장해 두어 우리의 나머지 코드에서
결과를 저장하고 재사용 하지 않아도 되도록 할 것입니다. 이 패턴을
*메모이제이션(memoization)* 혹은 *지연 평가(lazy evaluation)*로 알고 있을 것
입니다.

구조체에서 클로저를 갖고 있도록 하기 위해, 클로저 타입을 기술 할 필요가 있는데,
구조체 정의는 각 필드의 타입을 알 필요가 있기 때문 입니다. 각 클로저 인스턴스는
자신의 유일한 익명 타입을 갖습니다: 즉, 두 클로저가 동일한 타입 서명을 갖더라도
그들의 타입은 여전히 다른 것으로 간주 됩니다. 클로저를 사용하는 구조체, 열거형,
함수 파라미터를 정의하기 위해, 10장에서 설명한 것 처럼 제네릭과 트레잇 바운드를
사용합니다.

`Fn` 트레잇은 표준 라이브러리에서 제공 합니다. 모든 클로저들은 다음 트레잇 중
하나를 구현 합니다: `Fn`, `FnMut`, 혹은 `FnOnce`. 환경을 캡처하는 것에 대한 다음
절에서 이 트레잇들의 차이점들에 대해 설명할 것입니다; 이 예제에서, `Fn` 트레잇
을 사용할 수 있습니다.

클로저가 이 트레잇 바운드에 맞춰야 하는 파라미터와 반환값의 타입을 표현하기 위해
`Fn` 트레잇 바운드에 타입을 추가 합니다. 이 경우, 클로저는 파라미터 타입이 `u32`
이고 `u32` 타입을 번환하므로, 명시하는 트레잇 바운드는 `Fn(u32) -> u32` 입니다.

리스트 13-9 는 `Cacher` 구조체의 정의를 보여주는데 클로저와 선택적인 반환값을
갖고 있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

<span class="caption">리스트 13-9: `calculation` 에 클로저를 담고, 선택적인 결과
를 `value` 에 담는 `Cacher` 구조체 정의하기</span>

`Cacher` 구조체는 제너릭 타입 `T` 의 `calculation` 필드를 갖습니다.
`T` 에 대한 트레잇 바운드는 `Fn` 트레잇을 사용하여 그것이 클로저라는 것을 기술
합니다. `calculation` 필드에 저장하고자 하는 클로저는 하나의 `u32` 타입 파라미터
(`Fn` 다음에 괄호안에 명시됨)를 갖고 `u32` (`->` 다음에 명시됨) 타입의 값을
반환해야 합니다.

> 노트: 함수는 세개의 `Fn` 트레잇도 모두 구현 합니다. 환경에서 값을 캡처할 필요
> 가 없다면, `Fn` 트레잇을 구현한 어떤것을 필요로 하는 곳에 클로저 대신 함수를
> 사용할 수 있습니다.

`value` 필드는 `Option<u32>` 타입 입니다. 클로저를 실행하기 전에는 `value` 는
`None` 일 것입니다. `Cacher` 를 사용하는 코드에서 클로저의 *결과* 를 요청할 경
우, `Cacher` 는 그 때 클로저를 실행하고 결과를 `Some` variant 에 넣어서 `value`
필드에 저장 할 것입니다. 그 다음에는 코드에서 클로저의 결과를 다시 요청하면
클로저를 다시 실행하는 대신, `Cacher` 는 `Some` variant 안에 있는 결과를 돌려줄
것입니다.

방금 설명한 `value` 필드에 대한 로직은 리스트 13-10 에 정의되어 있습니다: 

<span class="filename">파일명: src/main.rs</span>

```rust
# struct Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     calculation: T,
#     value: Option<u32>,
# }
#
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

<span class="caption">리스트 13-10: `Cacher` 의 캐싱 로직</span>

우리는 이 필드에 있는 값을 호출하는 코드에서 잠재적으로 변경하도록 두기 보다
`Cacher` 가 구조체 필드의 값을 관리하도록 하고 싶기 때문에, 이 필드는 비공개
(private) 입니다.

`Cacher::new` 함수는 제네릭 파라미터 `T` 를 받는데, `Cacher` 구조체와 동일한
트레잇 바운드를 갖도록 정의 되었습니다. 그 다음 `Cacher::new` 는 `calculation`
필드에 명시된 클로저를 포함하고 클로저를 아직 실행한적이 없기 때문에 `value`
필드가 `None` 값을 갖는 `Cacher` 인스턴스를 반환 합니다.

호출하는 코드에서 클로저를 평가한 결과값을 원할때, 클로저를 직접 호출하기 보다,
`value` 메서드를 호출 할 것입니다. 이 메서드는 이미 `self.value` 에 결과값을
`Some` 으로 갖고 있는지 체크 합니다; 만약 그렇다면 클로저를 다시 실행하는 대신
`Some` 안에 있는 값을 반환 합니다.

만약 `self.value` 라 `None` 이라면, `self.calculation` 에 저장된 클로저를 호출
하고, 나중에 재사용 하기 위해 결과를 `self.value` 저장한 다음 그 값을 반환
합니다.

리스트 13-11 는 리스트 13-6 에 있는 `generate_workout` 함수에서 이 `Cacher` 구조
체를 사용하는 방법을 보여줍니다:

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::thread;
# use std::time::Duration;
#
# struct Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     calculation: T,
#     value: Option<u32>,
# }
#
# impl<T> Cacher<T>
#     where T: Fn(u32) -> u32
# {
#     fn new(calculation: T) -> Cacher<T> {
#         Cacher {
#             calculation,
#             value: None,
#         }
#     }
#
#     fn value(&mut self, arg: u32) -> u32 {
#         match self.value {
#             Some(v) => v,
#             None => {
#                 let v = (self.calculation)(arg);
#                 self.value = Some(v);
#                 v
#             },
#         }
#     }
# }
#
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

<span class="caption">리스트 13-11: 캐싱 로직을 추상화 하기 위해
`generate_workout` 함수 안에서 `Cacher` 사용하기</span>

클로저를 변수에 직접 저장하는 대신, 클로저를 갖는 `Cacher` 의 새 인스턴스를
저장 했습니다. 그러고는, 결과가 필요한 각 위치에 `Cacher` 인스턴스의 `value`
메소드를 호출 했습니다. 우리는 `value` 메소드를 원하는 만큼 많이 호출할 수 있고,
전혀 호출하지 않을 수도 있으며, 비싼 비용의 게산은 최대 한 번만 수행 될 것입니다.

리스트 13-2 의 `main` 함수로 이 프로그램을 실행해 보세요. 다양한 `if` 와 `else`
블럭에 있는 모든 케이스들을 검증하기 위해 `simulated_user_specified_value` 와
`simulated_random_number` 변수들을 변경해 보면, `calculating slowly...` 메세지는
필요할 때 단지 한 번만 나타 납니다. `Cacher` 는 필요한것 보다 더 많이 비싼 비용의
계산을 호출하지 않도록 보장하는 필요한 로직을 처리해서, `generate_workout` 가
비즈니스 로직에 집중하도록 해줍니다.

### `Cacher` 구현의 제약사항

값을 캐싱하는 것은 일반적으로 유용한 동작이기 때문에 이와는 다른 클로저를 사용
해서 우리 코드의 다른 부분에서 적용하고 싶을 수도 있습니다. 그러나 현재 `Cacher`
구현은 다른 문맥에서 다르게 재사용 하기에는 두 가지 문제가 있습니다.

첫 번째 문제는 `Cacher` 인스턴스가 `value` 메소드의 `arg` 파라미터에 대해
항상 같은 값을 얻는다는 가정을 한다는 것입니다. 즉, 이 `Cacher` 테스트는 실패 할 것
입니다:

```rust,ignore
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

이 테스트는 인자로 받은 값을 그대로 돌려주는 클로저가 포함된 새로운 `Cacher`
인스턴스를 생성 합니다. `arg` 값을 1로 그리고 `arg` 값을 2로 해서 이 `Cacher`
인스턴스의 `value` 메소드를 호출하고, `arg` 값을 2로 `value` 를 호출 했을 때
2를 반환 할 것으로 기대 합니다.

리스트 13-9 와 13-10 에 있는 `Cacher` 구현에 대해 이 테스트를 돌리면, 테스트는
이 메세지와 함께 `assert_eq!` 에서 실패 할 것입니다:


```text
thread 'call_with_different_values' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs
```

문제는 처음 `c.value` 을 1로 호출 했을 때, `Cacher` 인스턴스는 `self.value` 에
`Some(1)` 을 저장 합니다. 그 후에, `value` 값으로 무엇을 넘기던, 항상 1을 반환
할 것입니다.

`Cacher` 이 하나의 값보다 해시맵을 사용하도록 수정해 봅시다. 해시맵의 키는
넘겨받은 `arg` 값이 될 것이고, 해시맵의 값은 그 키로 클로저를 호출한 결과가 될
것입니다. `self.value` 가 `Some` 혹은 `None` 값인지 직접 살펴보는 대신,
`value` 함수는 해시맵의 `arg` 값을 살펴보고 값이 있으면 반환 할 것입니다.
값이 없으면, `Cacher` 는 클로저를 호출해서 해당 `arg` 값과 연관된 해시맵에
결과값을 저장 할 것입니다.

현재 `Cacher` 구현의 두 번째 문제는 `u32` 타입 파라미터 한 개만 받고 하나의
`u32` 을 반환한다는 것입니다. 예를 들면, 문자열 슬라이스를 넘겨주고 `usize`
값을 반환하는 클로저의 결과를 캐시에 저장하고 싶을 수도 있습니다. 이 이슈를 수정
하기 위해, `Cacher` 기능에 유연성을 높여주도록 더 중립적인 파라미터를 사용해
봅시다.

### 클로저로 환경 캡처 하기

운동 생성 예제에서, 우리는 클로저를 단지 인라인 익명 함수로 사용 했습니다.
그러나 클로저는 함수에 없는 추가적인 능력을 갖고 있습니다: 환경을 캡처해서
클로저가 정의된 스코프의 변수들을 접근할 수 있습니다.

`equal_to_x` 변수에 저장된 클로저가 클로저를 둘러싼 환경에 있는 `x` 변수를
사용하는 예제가 리스트 13-12 에 있습니다:

<span class="filename">파일명: src/main.rs</span>

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

<span class="caption">리스트 13-12: 둘러싼 범위에 있는 변수를 참조하는 클로저의
예</span>

비록 `x` 가 `equal_to_x` 의 파라미터 중에 하나가 아니더라도, `equal_to_x` 는
`equal_to_x` 가 정의된 동일한 스코프에 정의된 `x` 변수를 사용하는 것이 허용
됩니다.

함수로는 이와 동일하게 할 수 없습니다; 다음 예제로 시도해 보면, 코드는 컴파일
되지 않습니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```

에러가 발생 합니다:

```text
error[E0434]: can't capture dynamic environment in a fn item; use the || { ...
} closure form instead
 --> src/main.rs
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^
```

컴파일러는 이것은 클로저에서만 동작한다고 상기시켜 주기까지 합니다!

클로저가 그것의 환경에서 값을 캡처할 때, 클로저 바디에서 사용하기 위해 그 값을
저장하기 위한 메모리를 사용 합니다. 이 메모리 사용은 환경을 캡처하지 않는 코드를
실행하길 원하는 더 흔한 상황에서는 지불하기 싶지 않은 오버헤드 입니다.
왜냐하면 함수는 그들의 환경을 캡처할 수 없기 때문에, 함수를 정의하고 사용하는데
결코 이런 오버헤드는 발생하지 않을 것이기 때문 입니다.

클로저는 세가지 방식으로 그들의 환경에서 값을 캡처 할 수 있는데, 함수가 파라미터
를 받는 세가지 방식과 직접 연결 됩니다: 소유권 받기, 불변으로 빌려오기, 가변으로
빌려오기. 이것들은 다음과 같이 세개의 `Fn` 트레잇으로 표현 합니다:

* `FnOnce` 는 클로저의 *환경*으로 알고 있는, 그것을 둘러싼 환경에서 캡처한 변수
  들을 소비합니다. 캡처한 변수를 소비하기 위해, 클로저는 이 변수의 소유권을 
  가져야 하고 그것이 정의될 때 클로저 안으로 그것들을 옮겨와야 합니다. 이름의
  일부인 `Once` 는 그 클로저가 동일한 변수들에 대해 한번이상 소유권을 얻을수
  없다는 사실을 의미하며, 그래서 한 번만 호출 될 수 있습니다.
* `Fn` 은 그 환경으로 부터 값들을 불변으로 빌려 옵니다.
* `FnMut` 값들을 가변으로 빌려오기 때문에 그 환경을 변경할 수 있습니다.

우리가 클로저를 만들때, 러스트는 클로저가 환경에 있는 값을 어떻게 사용하는지에
근거 해서 어떤 트레잇을 사용할지 추론 합니다. 리스트 13-12 에서, `equal_to_x`
클로저의 바디에서는 `x` 에 있는 값을 읽기만 하면 되기 때문에 클로저는 `x` 를 
불변으로 빌려 옵니다. (그래서 `equal_to_x` 은 `Fn` 트래잇 입니다) 

만약 클로저가 환경으로부터 사용하는 값에 대해 소유권을 갖도록 강제하고 싶다면,
파라미터 리스트 앞에 `move` 키워드를 사용할 수 있습니다. 이 기법은 클로저를
다른 쓰레드로 넘길때 데이터를 이동시켜 새로운 쓰레드가 소유하도록 할때 대부분
유용 합니다.

16장에 병렬성에 대해 이야기 하는 부분에서 더 많은 `move` 클로저에 대한 예제가
있습니다. 지금은 리스트 13-12 의 코드에서 클로저에 `move` 키워드를 추가하고
정수 대신 벡터를 사용하도록 했는데, 정수는 이동되지 않고 복사되기 때문 입니다;
이 코드는 아직 컴파일 되지 않습니다:

<span class="filename">파일명: src/main.rs</span>

```rust,ignore
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

아래와 같은 에러가 발생합니다:

```text
error[E0382]: use of moved value: `x`
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
  implement the `Copy` trait
```

`move` 키워드를 추가했기 때문에 클로저가 정의될 때 `x` 값은 클로저 안으로
이동됩니다. `x` 의 소유권은 클로저가 갖게 되었고, `main` 은 더 이상  `println!`
문에서 `x` 사용하도록 허용되지 않습니다. `println!` 를 삭제하면 이 예제는 수정
됩니다.

`Fn` 트래잇 바운드 중 하나를 기술할 때 대부분의 경우, `Fn` 으로 시작해보면
컴파일러는 클로저 바디에서 무슨일을 하는지에 근거해서 `FnMut` 혹은 `FnOnce` 이
필요한지 말해 줍니다.

클로저가 그들의 환경을 캡처할 수 있는 상황을 표현하는 것은 함수 파라미터로써
유용 합니다. 다음 주제로 넘어가 봅시다: 반복자.
