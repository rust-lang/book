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

The reference count of the `Rc<List>` instances in both `a` and `b` are 2
after we change the list in `a` to point to `b`. At the end of `main`, Rust
will try to drop `b` first, which will decrease the count in each of the
`Rc<List>` instances in `a` and `b` by 1.

However, because `a` is still referencing the `Rc<List>` that was in `b`, that
`Rc<List>` has a count of 1 rather than 0, so the memory the `Rc<List>` has on
the heap won’t be dropped. The memory will just sit there with a count of 1,
forever. To visualize this reference cycle, we’ve created a diagram in Figure
15-4.

<img alt="Reference cycle of lists" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figure 15-4: A reference cycle of lists `a` and `b`
pointing to each other</span>

If you uncomment the last `println!` and run the program, Rust will try to
print this cycle with `a` pointing to `b` pointing to `a` and so forth until it
overflows the stack.

In this case, right after we create the reference cycle, the program ends. The
consequences of this cycle aren’t very dire. However, if a more complex program
allocated lots of memory in a cycle and held onto it for a long time, the
program would use more memory than it needed and might overwhelm the system,
causing it to run out of available memory.

Creating reference cycles is not easily done, but it’s not impossible either.
If you have `RefCell<T>` values that contain `Rc<T>` values or similar nested
combinations of types with interior mutability and reference counting, you must
ensure that you don’t create cycles; you can’t rely on Rust to catch them.
Creating a reference cycle would be a logic bug in your program that you should
use automated tests, code reviews, and other software development practices to
minimize.

Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don’t.
As a result, you can have cycles made up of some ownership relationships and
some non-ownership relationships, and only the ownership relationships affect
whether or not a value can be dropped. In Listing 15-25, we always want `Cons`
variants to own their list, so reorganizing the data structure isn’t possible.
Let’s look at an example using graphs made up of parent nodes and child nodes
to see when non-ownership relationships are an appropriate way to prevent
reference cycles.

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`

So far, we’ve demonstrated that calling `Rc::clone` increases the
`strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned
up if its `strong_count` is 0. You can also create a *weak reference* to the
value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a
reference to the `Rc<T>`. When you call `Rc::downgrade`, you get a smart
pointer of type `Weak<T>`. Instead of increasing the `strong_count` in the
`Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1.
The `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>`
references exist, similar to `strong_count`. The difference is the `weak_count`
doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.

Strong references are how you can share ownership of an `Rc<T>` instance. Weak
references don’t express an ownership relationship. They won’t cause a
reference cycle because any cycle involving some weak references will be broken
once the strong reference count of values involved is 0.

Because the value that `Weak<T>` references might have been dropped, to do
anything with the value that a `Weak<T>` is pointing to, you must make sure the
value still exists. Do this by calling the `upgrade` method on a `Weak<T>`
instance, which will return an `Option<Rc<T>>`. You’ll get a result of `Some`
if the `Rc<T>` value has not been dropped yet and a result of `None` if the
`Rc<T>` value has been dropped. Because `upgrade` returns an `Option<T>`, Rust
will ensure that the `Some` case and the `None` case are handled, and there
won’t be an invalid pointer.

As an example, rather than using a list whose items know only about the next
item, we’ll create a tree whose items know about their children items *and*
their parent items.

#### Creating a Tree Data Structure: a `Node` with Child Nodes

To start, we’ll build a tree with nodes that know about their child nodes.
We’ll create a struct named `Node` that holds its own `i32` value as well as
references to its children `Node` values:

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

We want a `Node` to own its children, and we want to share that ownership with
variables so we can access each `Node` in the tree directly. To do this, we
define the `Vec<T>` items to be values of type `Rc<Node>`. We also want to
modify which nodes are children of another node, so we have a `RefCell<T>` in
`children` around the `Vec<Rc<Node>>`.

Next, we’ll use our struct definition and create one `Node` instance named
`leaf` with the value 3 and no children, and another instance named `branch`
with the value 5 and `leaf` as one of its children, as shown in Listing 15-27:

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

<span class="caption">Listing 15-27: Creating a `leaf` node with no children
and a `branch` node with `leaf` as one of its children</span>

We clone the `Rc<Node>` in `leaf` and store that in `branch`, meaning the
`Node` in `leaf` now has two owners: `leaf` and `branch`. We can get from
`branch` to `leaf` through `branch.children`, but there’s no way to get from
`leaf` to `branch`. The reason is that `leaf` has no reference to `branch` and
doesn’t know they’re related. We want `leaf` to know that `branch` is its
parent. We’ll do that next.

#### Adding a Reference from a Child to Its Parent

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
