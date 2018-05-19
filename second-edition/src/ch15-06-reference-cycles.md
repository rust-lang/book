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

To make the child node aware of its parent, we need to add a `parent` field to
our `Node` struct definition. The trouble is in deciding what the type of
`parent` should be. We know it can’t contain an `Rc<T>`, because that would
create a reference cycle with `leaf.parent` pointing to `branch` and
`branch.children` pointing to `leaf`, which would cause their `strong_count`
values to never be 0.

Thinking about the relationships another way, a parent node should own its
children: if a parent node is dropped, its child nodes should be dropped as
well. However, a child should not own its parent: if we drop a child node, the
parent should still exist. This is a case for weak references!

So instead of `Rc<T>`, we’ll make the type of `parent` use `Weak<T>`,
specifically a `RefCell<Weak<Node>>`. Now our `Node` struct definition looks
like this:

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

A node will be able to refer to its parent node but doesn’t own its parent.
In Listing 15-28, we update `main` to use this new definition so the `leaf`
node will have a way to refer to its parent, `branch`:

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

<span class="caption">Listing 15-28: A `leaf` node with a weak reference to its
parent node `branch`</span>

Creating the `leaf` node looks similar to how creating the `leaf` node looked
in Listing 15-27 with the exception of the `parent` field: `leaf` starts out
without a parent, so we create a new, empty `Weak<Node>` reference instance.

At this point, when we try to get a reference to the parent of `leaf` by using
the `upgrade` method, we get a `None` value. We see this in the output from the
first `println!` statement:

```text
leaf parent = None
```

When we create the `branch` node, it will also have a new `Weak<Node>`
reference in the `parent` field, because `branch` doesn’t have a parent node.
We still have `leaf` as one of the children of `branch`. Once we have the
`Node` instance in `branch`, we can modify `leaf` to give it a `Weak<Node>`
reference to its parent. We use the `borrow_mut` method on the
`RefCell<Weak<Node>>` in the `parent` field of `leaf`, and then we use the
`Rc::downgrade` function to create a `Weak<Node>` reference to `branch` from
the `Rc<Node>` in `branch.`

When we print the parent of `leaf` again, this time we’ll get a `Some` variant
holding `branch`: now `leaf` can access its parent! When we print `leaf`, we
also avoid the cycle that eventually ended in a stack overflow like we had in
Listing 15-26; the `Weak<Node>` references are printed as `(Weak)`:

```text
leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
children: RefCell { value: [] } }] } })
```

The lack of infinite output indicates that this code didn’t create a reference
cycle. We can also tell this by looking at the values we get from calling
`Rc::strong_count` and `Rc::weak_count`.

#### Visualizing Changes to `strong_count` and `weak_count`

Let’s look at how the `strong_count` and `weak_count` values of the `Rc<Node>`
instances change by creating a new inner scope and moving the creation of
`branch` into that scope. By doing so, we can see what happens when `branch` is
created and then dropped when it goes out of scope. The modifications are shown
in Listing 15-29:

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

<span class="caption">Listing 15-29: Creating `branch` in an inner scope and
examining strong and weak reference counts</span>

After `leaf` is created, its `Rc<Node>` has a strong count of 1 and a weak
count of 0. In the inner scope, we create `branch` and associate it with
`leaf`, at which point when we print the counts, the `Rc<Node>` in `branch`
will have a strong count of 1 and a weak count of 1 (for `leaf.parent` pointing
to `branch` with a `Weak<Node>`). When we print the counts in `leaf`, we’ll see
it will have a strong count of 2, because `branch` now has a clone of the
`Rc<Node>` of `leaf` stored in `branch.children`, but will still have a weak
count of 0.

When the inner scope ends, `branch` goes out of scope and the strong count of
the `Rc<Node>` decreases to 0, so its `Node` is dropped. The weak count of 1
from `leaf.parent` has no bearing on whether or not `Node` is dropped, so we
don’t get any memory leaks!

If we try to access the parent of `leaf` after the end of the scope, we’ll get
`None` again. At the end of the program, the `Rc<Node>` in `leaf` has a strong
count of 1 and a weak count of 0, because the variable `leaf` is now the only
reference to the `Rc<Node>` again.

All of the logic that manages the counts and value dropping is built into
`Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait. By
specifying that the relationship from a child to its parent should be a
`Weak<T>` reference in the definition of `Node`, you’re able to have parent
nodes point to child nodes and vice versa without creating a reference cycle
and memory leaks.

## Summary

This chapter covered how to use smart pointers to make different guarantees and
trade-offs than those Rust makes by default with regular references. The
`Box<T>` type has a known size and points to data allocated on the heap. The
`Rc<T>` type keeps track of the number of references to data on the heap so
that data can have multiple owners. The `RefCell<T>` type with its interior
mutability gives us a type that we can use when we need an immutable type but
need to change an inner value of that type; it also enforces the borrowing
rules at runtime instead of at compile time.

Also discussed were the `Deref` and `Drop` traits, which enable a lot of the
functionality of smart pointers. We explored reference cycles that can cause
memory leaks and how to prevent them using `Weak<T>`.

If this chapter has piqued your interest and you want to implement your own
smart pointers, check out [“The Rustonomicon”][nomicon] for more useful
information.

[nomicon]: https://doc.rust-lang.org/stable/nomicon/

Next, we’ll talk about concurrency in Rust. You’ll even learn about a few new
smart pointers.
