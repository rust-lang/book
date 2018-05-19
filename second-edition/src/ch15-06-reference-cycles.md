## 순환 참조는 메모리 릭을 발생시킬 수 있습니다

러스트의 메모리 안정성 보장은 (*메모리 릭 (memory leak)* 이라고도 알려져 있는) 뜻하지
않게 해제되지 않는 메모리를 생성하기 힘들게 하지만, 그게 불가능한 것은 아닙니다.
메모리 릭을 완전히 방지하는 것은 컴파일 타임에 데이터 레이스를 허용하지 않는 것과 마찬가지로
러스트가 보장하는 것들 중 하나가 아닌데, 이는 메모리 릭도 러스트에서는 메모리 안정성에 포함됨을
의미합니다. 러스트가 `Rc<T>` 및 `RefCell<T>`를 사용하여 메모리 릭을 허용하는 것을 우리는
알 수 있습니다: 즉 아이템들이 서로를 순환 참조하는 참조자를 만드는 것이 가능합니다. 이는
메모리 릭을 발생시키는데, 그 이유는 순환 고리 안의 각 아이템들의 참조 카운트는 결코 0이
되지 않을 것이고, 그러므로 값들은 버려지지 않을 것이기 때문입니다.

### 순환 참조 만들기

Listing 15-25의 `List` 열거형과 `tail` 메소드 정의를 가지고서
어떻게 순환 참조가 생길 수 있고, 이를 어떻게 방지하는지
알아봅시다:

<span class="filename">Filename: src/main.rs</span>

<!-- Hidden fn main is here to disable the automatic wrapping in fn main that
doc tests do; the `use List` fails if this listing is put within a main -->

```rust
# fn main() {}
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}
```

<span class="caption">Listing 15-25: `RefCell<T>`를 가지고 있어서
`Cons` variant가 참조하는 것을 변경할 수 있는 cons 리스트 정의</span>

우리는 Listing 15-5의 `List` 정의의 또다른 변형을 이용하고 있습니다.
이제 `Cons` variant 내의 두번째 요소는 `RefCell<Rc<List>>`인데,
이는 Listing 15-24에서 우리가 했던 것처럼 `i32` 값을 변경하는 능력을
갖는 대신, `Cons` variant가 가리키고 있는 `List` 값을 변경하길 원한다는
의미입니다. 또한 `Cons` variant를 갖고 있다면 두번째 아이템에 접근하기
편하도록 `tail` 메소드를 추가하고 있습니다.

Listing 15-26에서 우리는 Listing 15-25의 정의를 사용하는 `main` 함수를
추가하고 있습니다. 이 코드는 `a`에 리스트를 만들고 `b`에는 `a`의 리스트를 가리키고
있는 리스트를 만들어 넣었습니다. 그 다음 `a`의 리스트가 `b`를 가리키도록 수정하는데,
이것이 순환 참조를 생성합니다. 이 과정 내의 다양한 지점에서 참조 카운트가 얼마인지를
보기 위해 곳곳에 `println!` 구문들이 있습니다.

<span class="filename">Filename: src/main.rs</span>

```rust
# use List::{Cons, Nil};
# use std::rc::Rc;
# use std::cell::RefCell;
# #[derive(Debug)]
# enum List {
#     Cons(i32, RefCell<Rc<List>>),
#     Nil,
# }
#
# impl List {
#     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
#         match *self {
#             Cons(_, ref item) => Some(item),
#             Nil => None,
#         }
#     }
# }
#
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

<span class="caption">Listing 15-26: 두 개의 `List` 값이 서로를 가리키는
순환 참조 생성하기</span>

우리는 초기값 리스트 `5, Nil`를 가진 `List` 값을 갖는 `Rc<List>`
인스턴스를 만들어 `a` 변수에 넣었습니다. 그런 다음 값 10과 `a`의 리스트를
가리키는 또다른 `List` 값을 갖는 `Rc<List>` 인스턴스를 만들어서 `b`
변수에 넣었습니다.

우리는 `a`를 수정하여 이것이 `Nil` 대신 `b`를 가리키도록 하였습니다. `a` 내의
`RefCell<Rc<List>>`에 대한 참조자를 얻어오기 위해 `tail` 메소드를 사용했는데,
이 참조자는 `link` 변수에 집어넣습니다. 그런 다음 `RefCell<Rc<List>>`의
`borrow_mut` 메소드를 사용하여 `Nil` 값을 가지고 있는`Rc<List>` 내부의 값을
`b`의 `Rc<List>`로 바꾸었습니다.

지금 잠깐동안 마지막 `println!` 문이 들어가지 않도록 주석처리하고 이 코드를 실행시킬
때, 아래와 같은 출력을 얻을 것입니다:

```text
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

`a`의 리스트가 `b`를 가리키도록 변경한 이후 `a`와 `b`의 `Rc<List>`
인스턴스의 참조 카운트는 둘 다 2입니다. `main`의 끝에서, 러스트는
`b`를 먼저 버리는 시도를 할 것인데, 이는 `a`와 `b`의 각각의 `Rc<List>`
인스턴스 내의 카운트를 1로 줄일 것입니다.

하지만 `a`가 여전히 `b` 내에 있는 `Rc<List>`를 참조하는 상태기 때문에, 이
`Rc<List>`는 0이 아니라 1의 카운트를 갖게 되고, 따라서 `Rc<List>`가 힙에
가지고 있는 메모리는 버려지지 않을 것입니다. 그 메모리는 참조 카운트 1을 가진 채로
영원히 그 자리에 그냥 있을 것입니다. 이러한 순환 참조를 시각화하기 위해 Figure
15-4의 다이어그램을 만들었습니다.

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figure 15-4: 리스트 `a`와 `b`가 서로를 가리키고
있는 순환 참조</span>

만일 여러분이 마지막 `println!`의 주석을 해제하고 프로그램을 실행해보면, 러스트는
`a`를 가리키고 있는 `b`를 가리키고 있는 `a`를 가리키고 있는... 과 같은 식으로
스택 오버플로우가 날 때까지 이 순환을 출력하려 할 것입니다.

이 경우, 우리가 순환 참조를 만든 직후, 프로그램은 종료됩니다. 위의 순환의
결과는 그렇게까지 심각하지는 않습니다. 하지만, 만일 좀더 복잡한 프로그램이
많은 매모리를 순환 형태로 할당했고 오랫동안 이를 유지했더라면, 프로그램은 
필요한 것보다 더 많은 메모리를 사용하게 되고, 사용 가능한 메모리를 동나게
하여 시스템을 멈추게 했을런지도 모릅니다.

순환 참조를 만드는 것은 쉽게 이루어지지는 않지만, 불가능한 것도 아닙니다.
만일 여러분이 `Rc<T>` 값을 가지고 있는 `RefCell<T>` 혹은 내부 가변성
및 참조 카운팅 기능이 있는 타입들로 유사한 조합을 사용한다면, 여러분은 순환을
만들지 않음을 보장해야 합니다; 이 순환들을 찾아내는 것을 러스트에 의지할 수는
없습니다. 순환 참조를 만드는 것은 여러분이 자동화된 테스트, 코드 리뷰, 그 외
소프트웨어 개발 연습 등을 이용하여 최소화해야 할 프로그램 내의 논리적
버그입니다.

순환 참조를 피하는 또다른 해결책은 여러분의 데이터 구조를 재구성하여
어떤 참조자는 소유권을 갖고 어떤 참조자는 그렇지 않도록 하는 것입니다.
결과적으로 여러분은 몇 개의 소유권 관계와 몇 개의 소유권 없는 관계로
이루어진 순환을 가질 수 있으며, 소유권 관계들만이 값을 버릴지 말지에
관해 영향을 주게 됩니다. Listing 15-25에서 우리는 `Cons` variant가
언제나 리스트를 소유하기를 원하므로, 데이터 구조를 재구성하는 것은 불가능합니다.
언제 소유권 없는 관계가 순환 참조를 방지하는 적절한 방법이 되는 때인지를
알기 위해서 부모 노드와 자식 노드로 구성된 그래프를 이용하는 예제를
살펴봅시다.

### 참조 순환 방지하기: `Rc<T>`를 `Weak<T>`로 바꾸기

이제까지 우리는 `Rc::clone`을 호출하는 것이 `Rc<T>` 인스턴스의 `strong_count`를
증가시키고, `Rc<T>` 인스턴스는 이것의 `strong_count`가 0이 된 경우에만 제거되는
것을 보았습니다. 여러분은 또한 `Rc::downgrade`를 호출하고 여기에 `Rc<T>`에 대한
참조자를 넘김겨서 `Rc<T>` 인스턴스 내의 값을 가리키는 *약한 참조 (weak reference)*
를 만들 수 있습니다. 여러분이 `Rc::downgrade`를 호출하면, 여러분은 `Weak<T>` 타입의
스마트 포인터를 얻게 됩니다. `Rc<T>` 인스턴스의 `strong_count`를 1 증가시키는 대신,
`Rc::downgrade`는 `weak_count`를 1 증가시킵니다. `Rc<T>` 타입은 몇 개의
`Weak<T>` 참조가 있는지 추적하기 위해서 `strong_count`와 유사한 방식으로
`weak_count`를 사용합니다. 차이점은 `Rc<T>`인스턴스가 제거되기 위해서 `weak_count`가 
0일 필요는 없다는 것입니다.

강한 참조는 여러분이 `Rc<T>` 인스턴스의 소유권을 공유할 수 있는 방법입니다. 약한
참조는 소유권 관계를 표현하지 않습니다. 이것은 순환 참조를 야기하지 않는데 그 이유는
몇몇의 약한 참조를 포함하는 순환이라도 강한 참조의 카운트가 0이 되고 나면 깨지게
될 것이기 때문입니다.

`Weak<T>`가 참조하고 있는 값이 이미 버려졌을지도 모르기 때문에, `Weak<T>`가
가리키고 있는 값을 가지고 어떤 일을 하기 위해서는 그 값이 여전히 존재하는지를 반드시
확인해야 합니다. 이를 위해 `Weak<T>`의 `upgrade` 메소드를 호출하는데, 이 메소드는
`Option<Rc<T>>`를 반환할 것입니다. 만일 `Rc<T>` 값이 아직 버려지지 않았다면
여러분은 `Some` 결과를 얻게 될 것이고 `Rc<T>` 값이 버려졌다면 `None` 결과값을
얻게 될 것입니다. `upgrade`가 `Option<T>`를 반환하기 때문에, 러스트는 `Some`의
경우와 `None`의 경우가 반드시 처리되도록 할 것이고, 따라서 유효하지 않은 포인터는
없을 것입니다.

예제로서 어떤 아이템이 오직 다음 아이템에 대해서만 알고 있는 리스트를 이용하는
것보다는 자식 아이템 *그리고* 부모 아이템에 대해 모두 알고 있는 아이템을 갖는
트리를 만들어 보겠습니다.

#### 트리 데이터 구조 만들기: 자식 노드를 가진 `Node`

자신의 자식 노드에 대해 알고 있는 노드를 갖는 트리를 만드는 것으로 시작해 보겠습니다.
우리는 `i32`값은 물론 자식 `Node`들의 참조자들 또한 가지고 있는 `Node`라는 이름의
구조체를 만들 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

우리는 `Node`가 자신의 자식들을 소유하기를 원하고, 이 소유권을 공유하여 트리의 각
`Node`에 직접 접근할 수 있도록 하기를 원합니다. 이를 하기 위해서 `Vec<T>` 아이템이
`Rc<Node>` 타입의 값이 되도록 정의하였습니다. 또한 우리는 어떤 노드가 다른 노드의
자식이 되도록 수정하기를 원하므로, `Vec<Rc<Node>>`를 `RefCell<T>`로 감싼
`children`을 갖도록 하였습니다.

그 다음, Listing 15-27에서 보시는 것처럼 이 구조체 정의를 이용하여 3의 값과
자식 노드가 없는 `leaf`라는 이름의 `Node` 인스턴스, 그리고 5의 값과 `leaf`를
자식으로 갖는 `branch`라는 이름의 인스턴스를 만들도록 하겠습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::rc::Rc;
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#    children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

<span class="caption">Listing 15-27: 자식이 없는 `leaf` 노드와
이 `leaf`를 자식 중 하나로 갖는 `branch` 노드 만들기</span>

`leaf` 내의 `Rc<Node>`를 클론하여 이를 `branch` 내에 저장했는데, 이는 `leaf` 내의
`Node`가 이제 두 소유권자를 가지게 되었다는 의미입니다. 우리는 `branch.children`를
통하여 `branch`에서부터 `leaf`까지 접근할 수 있게 되었지만, `leaf`에서부터 `branch`로
접근할 방법은 없습니다. 그 이유는 `leaf`가 `branch`에 대한 참조자를 가지고 있지 않아서
이들간의 연관성을 알지 못하기 때문입니다. 우리는 `leaf`로 하여금 `branch`가 그의
부모임을 알도록 하기를 원합니다. 이걸 다음에 해보겠습니다.

#### 자식으로부터 부모로 가는 참조자 추가하기

자식 노드가 그의 부모를 알도록 만들기 위하여, `parent` 필드를 우리의 `Node` 구조체
정의에 추가할 필요가 있습니다. 문제는 `parent`의 타입이 어떤 것이 되어야 하는지를 결정하는
중에 발생합니다. 이것이 `Rc<T>`를 담을 수 없음을 우리는 알고 있는데, 그렇게 하게 되면
`branch`를 가리키고 있는 `leaf.parent`와 `leaf`를 가리키고 있는 `branch.children`을
가지고 있는 순환 참조를 만들게 되며, 이것들의 `strong_count`값을 결코 0이 안되도록 하는
일을 야기하기 때문입니다.

이 관계들을 다른 방식으로 생각해보면, 부모 노드는 그의 자식들을 소유해야 합니다:
만일 부모 노드가 버려지게 되면, 그의 자식 노드들도 또한 버려져야 합니다. 하지만,
자식은 그의 부모를 소유해서는 안됩니다: 만일 우리가 자식 노드를 버리면, 그 부모는
여전히 존재해야 합니다. 이것이 바로 약한 참조를 위한 경우에 해당됩니다!

따라서 `Rc<T>` 대신 `Weak<T>`를 이용하여, 특별히 `RefCell<Weak<Node>>`를
이용하여 `parent`의 타입을 만들겠습니다. 이제 우리의 `Node` 구조체 정의는 아래와
같이 생기게 되었습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

노드는 그의 부모 노드를 참조할 수 있게 되겠지만 그 부모를 소유하지는 않습니다.
Listing 15-28에서, 우리는 이 새로운 정의를 사용하도록 `main`을 업데이트하여
`leaf` 노드가 그의 부모인 `branch`를 참조할 수 있는 방법을 갖도록 할 것입니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

<span class="caption">Listing 15-28: 부모 노드 `branch`의 약한 참조를
갖는 `leaf` 노드</span>

`leaf` 노드를 만드는 것은 `parent` 필드를 제외하고는 Listing 15-27에서 `leaf`
노드를 만드는 방법과 비슷해 보입니다: `leaf`는 부모없이 시작되어서, 새로운 비어있는
`Weak<Node>` 참조자 인스턴스를 생성하였습니다.

이 시점에서, 우리가 `upgrade` 메소드를 사용하여 `leaf`의 부모에 대한 참조자를
얻는 시도를 했을 때, 우리는 `None` 값을 얻습니다. 첫번째 `println!` 구문에서는
아래와 같은 출력을 보게됩니다:

```text
leaf parent = None
```

`branch` 노드를 생성할 때, 이 또한 `parent` 필드에 새로운 `Weak<Node>`
참조자를 가지도록 하는데, 이는 `branch`가 부모 노드를 가지지 않기 때문입니다.
우리는 여전히 `leaf`를 `branch`의 자식 중 하나로서 가지게 됩니다. 일단
`branch` 내의 `Node` 인스턴스를 가지게 되면, `leaf`에게 그의 부모에 대한
`Weak<Node>` 참조자를 가지도록 수정할 수 있습니다. 우리는 `leaf`의 `parent`
필드 내의 `RefCell<Weak<Node>>` 상의 `borrow_mut` 메소드를 사용하고,
그런 다음 `Rc::downgrade` 함수를 이용하여 `branch`의 `Rc<Node>`로부터
`branch`에 대한 `Weoak<Node>` 참조자를 생성하였습니다.

`leaf`의 부모를 다시한번 출력할 때, 이번에는 `branch`를 가지고 있는 `Some` variant를
얻게될 것입니다: 이제 `leaf`는 그의 부모에 접근할 수 있습니다! `leaf`를 출력할 때, 우리는
또한 Listing 15-26에서 발생했던 것과 같이 궁극적으로 스택 오버플로우로 끝나버리는 순환을
피하게 되었습니다; `Weak<Node>` 참조자는 `(Weak)`로 출력됩니다:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

무한 출력이 없다는 것은 이 코드가 순환 참조를 생성하지 않는 것을 나타냅니다.
이것은 또한 `Rc::strong_count`와 `Rc::weak_count`를 호출함으로써
얻은 값을 살펴보는 것으로도 알 수 있습니다.

#### `strong_count`와 `weak_count`의 변화를 시각화하기

새로운 내부 스코프를 만들고 `branch`의 생성을 이 스코프로 옮기는 것으로
`Rc<Node>` 인스턴스의 `strong_count`와 `weak_count` 값이 어떻게
변하는지 살펴보기로 합시다. 그렇게 함으로써, 우리는 `branch`가 만들어질 때와
그 다음 스코프 밖으로 벗어났을 때 어떤일이 생기는지 알 수 있습니다. 수정본은
Listing 15-29와 같습니다:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::rc::{Rc, Weak};
# use std::cell::RefCell;
#
# #[derive(Debug)]
# struct Node {
#     value: i32,
#     parent: RefCell<Weak<Node>>,
#     children: RefCell<Vec<Rc<Node>>>,
# }
#
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

<span class="caption">Listing 15-29: `branch`를 내부 스코프에서 만들고
강한 참조 및 약한 참조 카운트를 시험하기</span>

`leaf`가 생성된 다음, 이것의 `Rc<Node>`는 강한 참조 카운트 1개와 약한 참조 카운트
0개를 갖습니다. 내부 스코프에서 `branch`를 만들고 `leaf`와 연관짓게 되는데, 이때
우리가 카운트를 출력하면 `branch`의 `Rc<Node>`는 강한 참조 카운트 1개와
(`Weak<Node`를 가지고 `branch`를 가리키는 `leaf.parent`에 대한) 약한 참조 카운트
1개를 갖고 있을 것입니다. `leaf` 내의 카운트를 출력하면 강한 참조 카운트 2개를 갖고 있음을
보게될 것인제, 이는 `branch`가 이제 `branch.children`에 저장된 `leaf`의
`Rc<Node>`에 대한 클론을 가지게 되었지만, 여전히 약한 참조 0개를 갖게 될 것이기
때문입니다.

내부 스코프가 끝나게 되면, `branch`는 스포프 밖으로 벗어나게 되며 `Rc<Node>`의
강한 참조 카운트는 0으로 줄어들게 되므로, 이것의 `Node`는 버려지게 됩니다.
`leaf.parent`로부터 발생된 1개의 약한 참조 카운트는 `Node`가 버려질지 말지에
대한 어떠한 영향도 주지 않으므로, 아무런 메모리 릭도 발생하지 않았습니다!

만일 우리가 이 스코프의 끝 이후에 `leaf`의 부모에 접근하기를 시도한다면, 우리는
다시 `None`을 얻게 될 것입니다. 프로그램의 끝 부분에서, `leaf`의 `Rc<Node>`는
강한 참조 카운트 1개와 약한 참조 카운트 0개를 갖고 있는데, 그 이유는 `leaf` 변수가
이제 다시 `Rc<Node>`에 대한 유일한 참조자이기 때문입니다.

참조 카운트들과 버리는 값들을 관리하는 모든 로직은 `Rc<T>`와
`Weak<T>`, 그리고 이들의 `Drop` 트레잇에 대한 구현부에 만들어져
있습니다. 자식으로부터 부모로의 관계를 가 `Node`의 정의 내에서
`Weak<T>` 참조자로 되어야 함을 특정함으로서, 여러분은 순환 참조와
메모리 릭을 만들지 않고도 자식 노드를 가리키는 부모 노드 혹은 그 반대의
것을 가지게 될 수 있습니다.

## 정리

이번 장에서는 러스트가 일반적인 참조자를 가지고 기본적으로 보장하는 것들과는
다른 보장 및 트레이드 오프를 만들어내기 위해 스마트 포인터를 사용하는 방법을
다루었습니다. `Box<T>` 타입은 알려진 크기를 갖고 있고 힙에 할당된 데이터를
가리킵니다. `Rc<T>` 타입은 힙에 있는 데이터에 대한 참조자의 개수를 추적하여
그 데이터가 여러 개의 소유자들을 갖을 수 있도록 합니다. 내부 가변성을 갖춘
`RefCell<T>` 타입은 불변 타입을 원하지만 그 타입의 내부 값을 변경하기를
원할 때 사용할 수 있는 타입을 제공합니다; 이는 또한 컴파일 타임 대신 런타임에
빌림 규칙을 따르도록 강제합니다.

또한 `Deref` 및 `Drop` 트레잇을 다루었는데, 이는 스마트 포인터의 수많은
기능을 활성화해줍니다. 우리는 메모리 릭을 발생시킬 수 있는 순환 참조에 대한
것과 `Weak<T>`을 이용하여 이들을 방지하는 방법도 탐구하였습니다.

만일 이번 장이 여러분의 흥미를 언짢게 하고 여러분이 직접 여러분만의 스마트
포인터를 구현하기를 원한다면, [“러스토노미콘”][nomicon]에서 더 유용한
정보를 확인하세요.

[nomicon]: https://doc.rust-lang.org/stable/nomicon/

다음으로 우리는 러스트의 동시성에 대해 이야기해볼 것입니다. 여러분은 심지어 몇 개의
새로운 스마트 포인터에 대해서도 배우게 될 것입니다.
