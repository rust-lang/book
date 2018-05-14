## 반복자로 일련의 항목들 처리하기

반복자 패턴은 일련의 항목들에 대해 순서대로 어떤 작업을 수행할 수 있도록 해줍
니다. 반복자는 각 항목들을 순회하고 언제 시퀀스가 종료될지 결정하는 로직을
담당 합니다. 반복자를 사용하면, 저런 로직을 다시 구현할 필요가 없습니다.

러스트에서, 반복자는 *게으른데*, 항목들을 사용하기위해 반복자를 소비하는
메서드를 호출하기 전까지 반복자는 아무런 동작을 하지 않습니다.
예를 들면, 리스트 13-13 의 코드는 `Vec` 에 정의된 `iter` 메서드를 호출함으로써,
벡터 `v1` 에 있는 항목들에 대한 반복자를 생성 합니다. 이 코드 자체로는 어떤
유용한 동작을 하진 않습니다.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

<span class="caption">리스트 13-13: 반복자 생성하기</span>

일단 반복자를 만들면, 다양한 방법으로 사용할 수 있습니다. 3장의 리스트 3-5 에서,
각 항목에 대해 어떤 코드를 수행하기 위해 `for` 루프에서 반복자를 사용
했습니다만, 지금까지 `iter` 에 대한 호출이 무엇을 했는지 대충 넘어 갔었습니다.

리스트 13-14 의 예제는 `for` 루프에서 반복자를 사용하는 부분에서 반복자 생성을
분리 했습니다. 반복자는 `v1_iter` 변수에 저장되고, 그 시점에 순회는 발생하지
않습니다. `v1_iter` 에 있는 반복자를 사용하는 `for` 루프가 호출되면,
루프 순회 마다 반복자의 각 요소가 사용되는데, 각각의 값을 출력 합니다.

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

<span class="caption">리스트 13-14: `for` 루프에서 반복자 사용하기</span>

표준 라이브러리에서 반복자를 제공하지 않는 언어에서는, 변수를 인덱스 0으로
시작해서, 그 변수로 벡터를 색인해서 값을 가져오는데 사용하며, 루프안에서
벡터에 있는 아이템의 총 갯수까지 그 변수를 증가시키는 방식으로 동일한 기능을
작성할 수 있습니다.

반복자는 모든 로직을 대신 처리 하며, 잠재적으로 엉망이 될 수 있는 반복적인
코드를 줄여 줍니다. 반복자는 벡터처럼 색인할 수 있는 자료구조 뿐만 아니라,
많은 다른 종류의 시퀀스에 대해 동일한 로직을 사용할 수 있도록 더 많은 유연성을
제공 합니다. 반복자가 어떻게 그렇게 하는지 살펴 봅시다.

### The `Iterator` Trait and the `next` Method

모든 반복자는 표준 라이브러리에 정의된 `Iterator` 라는 이름의 트레잇을 구현 합니
다. 트레잇의 정의는 아래와 같습니다:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

이 정의는 몇 개의 새로운 문법을 사용하는데 유의하세요: `type Item` 과
`Self::Item` 은 이 트레잇과 *연관 타입* 을 정의 합니다. 우리는 19장에서
연관 타입에 대해 자세히 이야기 할 것 입니다. 지금 당장 알아야 할 것은 이 코드가
`Iterator` 트레잇을 구현하는 것은 `Item` 타입을 정의하는 것 또한 요구하며, 이 
`Item` 타입이 `next` 메서드의 리턴 타입으로 사용된다는 것을 나타낸다는 것 입니
다. 다른 말로, `Item` 타입은 반복자로 부터 반환되는 타입이 될 것 입니다.

`Iterator` 트레잇은 단지 구현자가 하나의 메서드를 정의하도록 요구 합니다: 
`next` 메서드 입니다. 이 메서드는 반복자의 하나의 항목을 `Some` 에 넣어서 반환
하고, 반복자가 종료되면 `None` 을 반환 합니다.

반복자의 `next` 메서드를 ㅈ기접 호출할 수 있습니다; 리스트 13-15 는 벡터로 부터
생성된 반복자에 대해 반복된 `next` 호출이 어떤 값들을 반환하는지 보여줍니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

<span class="caption">리스트 13-15: 반복자의 `next` 메서드 호출하기</span>

`v1_iter` 가 변경 가능하도록 만들 필요가 있다는 것에 유의 하세요: 반복자에 대해
`next` 메서드를 호출하면 시퀀스의 어디에 있는지 추적하기 위해 반복자가 사용하는
내부 상태를 변경합니다. 다른 말로, 이 코드는 반복자를 *소비 합니다*, 혹은 사용
합니다. `next` 에 대한 각 호출은 반복자로 부터 하나의 항목을 소비 합니다. 
`for` 루프를 사용할 때는 `v1_iter` 를 변경할 수 있도록 만들 필요가 없는데, 
루프가 `v1_iter` 의 소유권을 갖고 보이진 않지만 변경 가능하도록 만들기 때문
입니다.

`next` 호출로 얻어온 값들은 벡터 안에 있는 값들에 대한 불변 참조라는 점 역시
유이 하세요. `iter` 메서드는 불변 참조에 대한 반복자를 만듭니다. 만약 `v1` 의
소유권을 갖고 소유된 값들을 반환하도록 하고 싶다면, `iter` 대신 `into_iter` 를
호출해야 합니다. 비슷하게, 가변 참조에 대한 반복자를 원한다면, `iter` 대신
`iter_mut` 을 호출할 수 있습니다.

### 반복자를 소비하는 메서드들

`Iterator` 트레잇에는 표준 라이브러리에서 기본 구현을 제공하는 다수의 다른
메서드들이 있습니다; `Iterator` 트레잇에 대한 표준 라이브러리 API 문서를 살펴
보면, 이 메서드들을 찾을 수 있습니다. 이 메서드들 중 일부는 그들의 구현에서
`next` 메서드를 호출하는데, 이것이 `Iterator` 트레잇을 구현할 때 `next` 메서드를
구현해야만 하는 이유 입니다.

`next` 를 호출하는 메서드들을 *소비하는 어댑터들* 이라고 하는데, 거들을 호출하면
반복자를 써버리기 때문 입니다. `sum` 메서드가 하나의 예인데, 반복자의 소유권을
가져오고 반복적으로 `next` 를 호출해서 순회함으로써 반복자를 소비 합니다.
순회해 나가면서 누적합계에 각 아이템을 더하고 순회가 완료되면 합계를 반환
합니다. 리스트 13-16 은 `sum` 메서드의 사용을 보여주는 테스트 입니다:

<span class="filename">Filename: src/lib.rs</span>

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

<span class="caption">리스트 13-16: 반복자의 모든 항목에 대한 합계를 얻기 위해
`sum` 메서드 호출 하기</span>

`sum` 은 호출한 반복자의 소유권을 갖기 때문에, `sum` 을 호출한 후 `v1_iter` 은
사용할 수 없습니다.

### Methods that Produce Other Iterators

Other methods defined on the `Iterator` trait, known as *iterator adaptors*,
allow you to change iterators into different kinds of iterators. You can chain
multiple calls to iterator adaptors to perform complex actions in a readable
way. But because all iterators are lazy, you have to call one of the consuming
adaptor methods to get results from calls to iterator adaptors.

Listing 13-17 shows an example of calling the iterator adaptor method `map`,
which takes a closure to call on each item to produce a new iterator. The
closure here creates a new iterator in which each item from the vector has been
incremented by 1. However, this code produces a warning:

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

<span class="caption">Listing 13-17: Calling the iterator adaptor `map` to
create a new iterator</span>

The warning we get is this:

```text
warning: unused `std::iter::Map` which must be used: iterator adaptors are lazy
and do nothing unless consumed
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
```

The code in Listing 13-17 doesn’t do anything; the closure we’ve specified
never gets called. The warning reminds us why: iterator adaptors are lazy, and
we need to consume the iterator here.

To fix this and consume the iterator, we’ll use the `collect` method, which we
used in Chapter 12 with `env::args` in Listing 12-1. This method consumes the
iterator and collects the resulting values into a collection data type.

In Listing 13-18, we collect the results of iterating over the iterator that’s
returned from the call to `map` into a vector. This vector will end up
containing each item from the original vector incremented by 1.

<span class="filename">Filename: src/main.rs</span>

```rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

<span class="caption">Listing 13-18: Calling the `map` method to create a new
iterator and then calling the `collect` method to consume the new iterator and
create a vector</span>

Because `map` takes a closure, we can specify any operation we want to perform
on each item. This is a great example of how closures let you customize some
behavior while reusing the iteration behavior that the `Iterator` trait
provides.

### Using Closures that Capture Their Environment

Now that we’ve introduced iterators, we can demonstrate a common use of
closures that capture their environment by using the `filter` iterator adaptor.
The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a Boolean. If the closure returns `true`, the value
will be included in the iterator produced by `filter`. If the closure returns
`false`, the value won’t be included in the resulting iterator.

In Listing 13-19, we use `filter` with a closure that captures the `shoe_size`
variable from its environment to iterate over a collection of `Shoe` struct
instances. It will return only shoes that are the specified size.

<span class="filename">Filename: src/lib.rs</span>

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```

<span class="caption">Listing 13-19: Using the `filter` method with a closure
that captures `shoe_size`</span>

The `shoes_in_my_size` function takes ownership of a vector of shoes and a shoe
size as parameters. It returns a vector containing only shoes of the specified
size.

In the body of `shoes_in_my_size`, we call `into_iter` to create an iterator
that takes ownership of the vector. Then we call `filter` to adapt that
iterator into a new iterator that only contains elements for which the closure
returns `true`.

The closure captures the `shoe_size` parameter from the environment and
compares the value with each shoe’s size, keeping only shoes of the size
specified. Finally, calling `collect` gathers the values returned by the
adapted iterator into a vector that’s returned by the function.

The test shows that when we call `shoes_in_my_size`, we get back only shoes
that have the same size as the value we specified.

### Creating Our Own Iterators with the `Iterator` Trait

We’ve shown that you can create an iterator by calling `iter`, `into_iter`, or
`iter_mut` on a vector. You can create iterators from the other collection
types in the standard library, such as hash map. You can also create iterators
that do anything you want by implementing the `Iterator` trait on your own
types. As previously mentioned, the only method you’re required to provide a
definition for is the `next` method. Once you’ve done that, you can use all
other methods that have default implementations provided by the `Iterator`
trait!

To demonstrate, let’s create an iterator that will only ever count from 1 to 5.
First, we’ll create a struct to hold some values. Then we’ll make this struct
into an iterator by implementing the `Iterator` trait and using the values in
that implementation.

Listing 13-20 has the definition of the `Counter` struct and an associated
`new` function to create instances of `Counter`:

<span class="filename">Filename: src/lib.rs</span>

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

<span class="caption">Listing 13-20: Defining the `Counter` struct and a `new`
function that creates instances of `Counter` with an initial value of 0 for
`count`</span>

The `Counter` struct has one field named `count`. This field holds a `u32`
value that will keep track of where we are in the process of iterating from 1
to 5. The `count` field is private because we want the implementation of
`Counter` to manage its value. The `new` function enforces the behavior of
always starting new instances with a value of 0 in the `count` field.

Next, we’ll implement the `Iterator` trait for our `Counter` type by defining
the body of the `next` method to specify what we want to happen when this
iterator is used, as shown in Listing 13-21:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

<span class="caption">Listing 13-21: Implementing the `Iterator` trait on our
`Counter` struct</span>

We set the associated `Item` type for our iterator to `u32`, meaning the
iterator will return `u32` values. Again, don’t worry about associated types
yet, we’ll cover them in Chapter 19.

We want our iterator to add 1 to the current state, so we initialized `count`
to 0 so it would return 1 first. If the value of `count` is less than 6, `next`
will return the current value wrapped in `Some`, but if `count` is 6 or higher,
our iterator will return `None`.

#### Using Our `Counter` Iterator’s `next` Method

Once we’ve implemented the `Iterator` trait, we have an iterator! Listing 13-22
shows a test demonstrating that we can use the iterator functionality of our
`Counter` struct by calling the `next` method on it directly, just as we did
with the iterator created from a vector in Listing 13-15.

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Iterator for Counter {
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         self.count += 1;
#
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

<span class="caption">Listing 13-22: Testing the functionality of the `next`
method implementation</span>

This test creates a new `Counter` instance in the `counter` variable and then
calls `next` repeatedly, verifying that we have implemented the behavior we
want this iterator to have: returning the values from 1 to 5.

#### Using Other `Iterator` Trait Methods

We implemented the `Iterator` trait by defining the `next` method, so we
can now use any `Iterator` trait method’s default implementations as defined in
the standard library, because they all use the `next` method’s functionality.

For example, if for some reason we wanted to take the values produced by an
instance of `Counter`, pair them with values produced by another `Counter`
instance after skipping the first value, multiply each pair together, keep only
those results that are divisible by 3, and add all the resulting values
together, we could do so, as shown in the test in Listing 13-23:

<span class="filename">Filename: src/lib.rs</span>

```rust
# struct Counter {
#     count: u32,
# }
#
# impl Counter {
#     fn new() -> Counter {
#         Counter { count: 0 }
#     }
# }
#
# impl Iterator for Counter {
#     // Our iterator will produce u32s
#     type Item = u32;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         // increment our count. This is why we started at zero.
#         self.count += 1;
#
#         // check to see if we've finished counting or not.
#         if self.count < 6 {
#             Some(self.count)
#         } else {
#             None
#         }
#     }
# }
#
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```

<span class="caption">Listing 13-23: Using a variety of `Iterator` trait
methods on our `Counter` iterator</span>

Note that `zip` produces only four pairs; the theoretical fifth pair `(5,
None)` is never produced because `zip` returns `None` when either of its input
iterators return `None`.

All of these method calls are possible because we specified how the `next`
method works, and the standard library provides default implementations for
other methods that call `next`.
