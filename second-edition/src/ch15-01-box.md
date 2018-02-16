## `Box<T>` Points to Data on the Heap and Has a Known Size
## `Box<T>`는 힙에 있는 데이터를 가리키고 알려진 크기를 갖습니다

가장 직관적인 스마트 포인터는 *박스 (box)* 인데, 이 타입은 `Box<T>`
라고 쓰여집니다. 박스는 여러분이 데이터를 스택이 아니라 힙에 저장할 수 있도록
해줍니다. 스택에 남는 것은 힙 데이터를 가리키는 포인터입니다. 스택과 힙의
차이를 상기하려면 4장을 참조하세요.

박스는 스택 대신 힙에 데이터를 저장한다는 점 외에는, 성능적인 오버헤드가
없습니다. 하지만 여러가지의 추가 기능 또한 가지고 있지 않습니다. 여러분은
이를 아래와 같은 상황에서 가장 자주 쓰게 될 것입니다:

* 컴파일 타임에 크기를 알 수 없는 타입을 갖고 있고, 정확한 하이즈를 알 필요가
  있는 맥락 안에서 해당 타입의 값을 이용하고 싶을 때
* 커다란 데이터를 가지고 있고 소유권을 옮기고 싶지만 그렇게 했을 때 데이터가
  복사되지 않을 것이라고 보장하기를 원할 때
* 어떤 값을 소유하고 이 값의 구체화된 타입을 알고 있기 보다는 특정 트레잇을
  구현한 타입이라는 점만 신경쓰고 싶을 때

이 장에서는 첫번째 상황을 보여줄 것입니다. 그러나 보여주기 전에, 나머지 두
상황에 대해 약간 더 자세히 말하겠습니다: 두번째 경우, 방대한 양의 데이터의
소유권 옮기기는 긴 시간이 소요될 수 있는데 이는 그 데이터가 스택 상에서
복사되기 때문입니다. 이러한 상황에서 성능을 향상시키기 위해서, 박스 안의
힙에 그 방대한 양의 데이터를 저장할 수 있습니다. 그러면, 작은 양의 포인터
데이터만 스택 상에서 복사되고, 데이터는 힙 상에서 한 곳에 머물게 됩니다.
세번째 경우는 *트레잇 객체 (trait object)* 라고 알려진 것이고, 17장이
이 주제만으로 전체를 쏟아부었습니다. 그러니 여러분이 여기서 배운 것을 17장에서
다시 적용하게 될 것입니다!


### `Box<T>`을 사용하여 힙에 데이터를 저장하기

`Box<T>`에 대한 사용례를 논의하기 전에, 먼저 문법 및 `Box<T>` 내에 저장된
값과 어떻게 상호작용 하는지 다루겠습니다.

Listing 15-1은 힙에 `i32` 값을 저장하기 위해 박스를 사용하는 법을 보여줍니다:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

<span class="caption">Listing 15-1: 박스를 사용하여 `i32` 값을
힙에 저장하기</span>

`5`라는 값을 가리키는 `Box`의 값을 갖는 변수 `b`를 선언했는데, 여기서
`5`는 힙에 할당됩니다. 이 프로그램은 `b = 5`를 출력할 것입니다; 이 경우,
우리는 마치 이 데이터가 스택에 있었던 것과 유사한 방식으로 박스 내의 데이터에
접근할 수 있습니다. 다른 어떤 소유한 값과 마찬가지로, `b`가 `main`의 끝에
도달하는 것처럼 어떤 박스가 스코프를 벗어날때, 할당은 해제될 것입니다. 할당
해제는 (스택에 저장된) 박스와 이것이 가리키고 있는 (힙에 저장된) 데이터
모두에게 일어납니다.

단일 값을 힙에 집어넣는 것은 그다지 유용하지는 않으므로, 이 방식처럼 박스를
이용하는 것은 자주 쓰지 않을 것입니다. 단일한 `i32` 같은 값을 스택에 갖는
것은, 스택이 해당 값이 기본적으로 저장되는 곳이기도 하고, 대부분의 경우에서
더 적절합니다. 만일 우리가 박스를 쓰지 않는다면 허용되지 않았을 타입을
정의하도록 해주는 경우를 살펴봅시다.

### 박스는 재귀적 타입을 가능케합니다

컴파일 타임에서, 러스트는 어떤 타입이 얼마나 많은 공간을 차지하는지를 알 필요가 있습니다.
컴파일 타임에는 크기를 알 수 없는 한가지 타입이 바로 *재귀적 타입 (recursive type)*
인데, 이는 어떤 값이 그 일부로서 동일한 타입의 다른 값을 갖을 수 있는 것을 말합니다.
이러한 값의 내포가 이론적으로는 무한하게 계속 될 수 있으므로, 러스트는 재귀적 타입의
값이 얼만큼의 공간을 필요로 하는지 알지 못합니다. 하지만, 박스는 알려진 크기를 갖고
있으므로, 재귀적 타입 정의 내에 박스를 넣음으로써 이를 쓸 수 있습니다.

재귀적 타입의 예제로서, 함수형 프로그래밍 언어에서 일반적인 데이터 타입인
*cons list*를 탐험해 봅시다. 우리가 정의할 cons list 타입은 재귀를
제외하면 직관적입니다; 그러므로, 우리가 작업할 예제에서의 개념은 여러분이
재귀적 타입을 포함하는 더 복잡한 어떠한 경우에 처하더라도 유용할 것입니다.

#### Cons List에 대한 더 많은 정보

*cons list*는 Lisp 프로그래밍 언어 및 그의 파생 언어들로부터 유래된 데이터
구조입니다. Lisp에서, (“생성 함수 (construct function)”의 줄임말인) `cons`
함수는 두 개의 인자를 받아 새로운 한 쌍을 생성하는데, 이 인자는 보통 단일
값과 또다른 쌍입니다. 이러한 쌍들을 담고 있는 쌍들이 리스트를 형성합니다.

cons 함수 개념은 더 일반적인 함수형 프로그래밍 용어로 나아갑니다:
“to cons x onto y”는 약식으로 요소 x를 새로운 컨테이너에 집어넣고,
그 다음 컨테이너 y를 넣는 식으로새로운 컨테이너 인스턴스를 생성하는
것을 의미합니다.

cons list 내의 각 아이템은 두 개의 요소를 담고 있습니다: 현재 아이템의
값과 다음 아이템이지요. 리스트의 마지막 아이템은 다음 아이템 없이 `Nil`
이라 불리우는 값을 담고 있습니다. cons list는 `cons` 함수를 재귀적으로
호출함으로서 만들어집니다. 재귀의 기본 케이스를 의미하는 표준 이름이 바로
`Nil` 입니다. 유효하지 않은 값 혹은 값이 없는 것을 말하는 6장의 “null”
혹은 “nil” 개념과 동일하지 않다는 점을 주의하세요.

비록 함수형 프로그래밍 언어들이 cons list를 자주 사용할지라도, 러스트에서는
흔히 사용되는 데이터 구조가 아닙니다. 러스트에서 아이템의 리스트를 갖는
대부분의 경우에는, `Vec<T>`이 사용하기에 더 나은 선택입니다. 그와는 다른,
더 복잡한 재귀적 데이터 타입들은 다양한 상황들에서 유용*하기는* 하지만,
cons list를 가지고 시작함으로써, 박스가 어떻게 재귀적 데이터 타입을
정의하도록 해주는지 우리의 집중을 방해하는 것들 없이 탐구할 수 있습니다.

Listing 15-2는 cons list를 위한 열거형 정의를 담고 있습니다. 우리가 보여주고자
하는 것인데, `List` 타입이 알려진 크기를 가지고 있지 않고 있기 때문에 이 코드는
아직 컴파일이 안된다는 점을 유의하세요:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
enum List {
    Cons(i32, List),
    Nil,
}
```

<span class="caption">Listing 15-2: `i32` 값의 cons list 데이터 구조를
표현하는 열거형 정의에 대한 첫번째 시도</span>

> 노트: 이 예제의 목적을 위해 오직 `i32` 값만 담는 cons list를 구현하고
> 있습니다. 우리가 10장에서 논의한 것처럼, 임의의 타입 값을 저장할 수 있는
> cons list 타입을 정의하기 위해서는 제네릭을 이용해 이를 구현할 수도
> 있습니다.

`List` 타입을 이용하여 리스트 `1, 2, 3`을 저장하는 것은 Listing 15-3의
코드와 같이 보일 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

<span class="caption">Listing 15-3: `List` 열거형을 이용하여 리스트 `1, 2, 3`
저장하기</span>

첫번째 `Cons` 값은 `1`과 `List` 값을 갖습니다. 이 `List` 값은 `2`와 또다른
`List` 값을 갖는 `Cons` 값입니다. 그 안의 `List` 값은 `3`과 `List` 값을
갖는 추가적인 `Cons`인데, 여기서 마지막의 `List`은 `Nil`로서, 리스트의
끝을 알리는 비재귀적인 variant입니다.

만일 Listing 15-3의 코드를 컴파일하고자 시도하면, Listing 15-4에 보이는
에러를 얻습니다:

```text
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ----- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

<span class="caption">Listing 15-4: 재귀적 열거형을 정의하고자 시도했을때
얻게되는 에러</span>

이 에러는 이 타입이 “무한한 크기를 갖는다”고 말해줍니다. 그 원인은 우리가 재귀적인
variant를 이용하여 `List`를 정의했기 때문입니다: 즉 이것은 또다른 자신을 직접
값으로 갖습니다. 결과적으로, 러스트는 `List` 값을 저장하는데 필요한 크기가 얼마나
되는지 알아낼 수 없습니다. 왜 우리가 이런 에러를 얻게 되는지 좀더 쪼개어 봅시다:
먼저, 러스트가 비재귀적인 타입의 값을 저장하는데 필요한 용량이 얼마나 되는지
결정하는 방법을 살펴봅시다.

#### Computing the Size of a Non-Recursive Type

Recall the `Message` enum we defined in Listing 6-2 when we discussed enum
definitions in Chapter 6:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a `Message` value, Rust goes
through each of the variants to see which variant needs the most space. Rust
sees that `Message::Quit` doesn’t need any space, `Message::Move` needs enough
space to store two `i32` values, and so forth. Because only one variant will be
used, the most space a `Message` value will need is the space it would take to
store the largest of its variants.

Contrast this to what happens when Rust tries to determine how much space a
recursive type like the `List` enum in Listing 15-2 needs. The compiler starts
by looking at the `Cons` variant, which holds a value of type `i32` and a value
of type `List`. Therefore, `Cons` needs an amount of space equal to the size of
an `i32` plus the size of a `List`. To figure out how much memory the `List`
type needs, the compiler looks at the variants, starting with the `Cons`
variant. The `Cons` variant holds a value of type `i32` and a value of type
`List`, and this process continues infinitely, as shown in Figure 15-1:

<img alt="An infinite Cons list" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-1: An infinite `List` consisting of infinite
`Cons` variants</span>

#### Using `Box<T>` to Get a Recursive Type with a Known Size

Rust can’t figure out how much space to allocate for recursively defined types,
so the compiler gives the error in Listing 15-4. But the error does include
this helpful suggestion:

```text
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
```

In this suggestion, “indirection” means that instead of storing a value
directly, we’ll change the data structure to store the value indirectly by
storing a pointer to the value instead.

Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>`
needs: a pointer’s size doesn’t change based on the amount of data it’s
pointing to. This means we can put a `Box<T>` inside the `Cons` variant instead
of another `List` value directly. The `Box<T>` will point to the next `List`
value that will be on the heap rather than inside the `Cons` variant.
Conceptually, we still have a list, created with lists “holding” other lists,
but this implementation is now more like the items being next to one another
rather than inside one another.

We can change the definition of the `List` enum in Listing 15-2 and the usage
of the `List` in Listing 15-3 to the code in Listing 15-5, which will compile:

<span class="filename">Filename: src/main.rs</span>

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

<span class="caption">Listing 15-5: Definition of `List` that uses `Box<T>` in
order to have a known size</span>

The `Cons` variant will need the size of an `i32` plus the space to store the
box’s pointer data. The `Nil` variant stores no values, so it needs less space
than the `Cons` variant. We now know that any `List` value will take up the
size of an `i32` plus the size of a box’s pointer data. By using a box, we’ve
broken the infinite, recursive chain, so the compiler can figure out the size
it needs to store a `List` value. Figure 15-2 shows what the `Cons` variant
looks like now:

<img alt="A finite Cons list" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figure 15-2: A `List` that is not infinitely sized
because `Cons` holds a `Box`</span>

Boxes only provide the indirection and heap allocation; they don’t have any
other special capabilities, like those we’ll see with the other smart pointer
types. They also don’t have any performance overhead that these special
capabilities incur, so they can be useful in cases like the cons list where the
indirection is the only feature we need. We’ll look at more use cases for boxes
in Chapter 17, too.

The `Box<T>` type is a smart pointer because it implements the `Deref` trait,
which allows `Box<T>` values to be treated like references. When a `Box<T>`
value goes out of scope, the heap data that the box is pointing to is cleaned
up as well because of the `Drop` trait implementation. Let’s explore these two
traits in more detail. These two traits will be even more important to the
functionality provided by the other smart pointer types we’ll discuss in the
rest of this chapter.
