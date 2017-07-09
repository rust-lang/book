## 소유권이 뭔가요?

러스트의 핵심 기능은 바로 소유권입니다. 이 기능은 직관적으로 설명할 수 있지만, 언어의 나머지 부분에
깊은 영향을 끼칩니다.

모든 프로그램은 실행하는 동안 컴퓨터의 메모리를 사용하는 방법을 관리해야 합니다. 몇몇 언어들은 프로그램이
실행될 때 더이상 사용하지 않는 메모리를 끊임없이 찾는 가비지 콜렉션을 갖고 있습니다; 다른 언어들에서는
프로그래머가 직접 명시적으로 메모리를 할당하고 해제해야 합니다. 러스트는 제 3의 접근법을 이용합니다:
메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 소유권 시스템을 통해 관리됩니다. 소유권 기능들의
어떤 것도 런타임 비용이 발생하지 않습니다.

소유권이란 개념이 많은 프로그래머들에게 새로운 것이기 때문에, 이해하고 사용하는 데에는 약간의 시간이
걸립니다만, 좋은 소식은 여러분이 러스트와 소유권 시스템의 규칙에 더 많은 경험을 할수록, 여러분은 더
안전하고 더 효율적인 코드를 자연스럽게 개발할 수 있게될 것이라는 거죠. 견뎌내세요!

여러분이 소유권을 이해했을 때, 여러분은 러스트를 유니크하게 만드는 기능들을 이해하기 위한 견고한 기초를
가지게 될 것입니다. 이 장에서, 여러분은 매우 흔한 데이터 구조인 문자열에 집중된 몇가지 예제를 통해
소유권에 대해 배우게 될 것입니다.


<!-- PROD: START BOX -->

> ### 스택과 힙
>
> 많은 프로그래밍 언어들 안에서, 우리는 그렇게 자주 스택과 힙에 대한 생각을 할 필요가 없습니다. 그렇지만
> 러스트와 같은 시스템 프로그래밍 언어에서는, 값이 스택에 있는지 힙에 있는지의 여부가 언어의 동작 방식과
> 우리의 결단에 더 큰 영향을 줍니다. 우리는 이 장의 뒤쪽에서 스택과 힙에 관계된 소유권의 일부분을 기술할
> 것이기에, 여기서는 준비 삼아 간략한 설명만 하겠습니다.
>
> 스택과 힙 둘다 여러분의 코드상에서 런타임에 사용할 수 있는 메모리의 부분입니다만, 이들은 각기 다른
> 방식으로 구조화 되어 있습니다. 스택은 값을 받아들인 순서대로 값을 저장하고 반대 방향으로 값을 지웁니다.
> 이것을 *last in, first out*이라고 하죠. 쌓여있는 접시를 생각해보세요; 여러분이 접시를 더 추가하려면
> 접시더미의 꼭대기에 쌓아올리고, 여러분이 접시가 필요해지면 꼭대기에서부터 한장 꺼내게 됩니다. 중간이나
> 밑에서부터 접시를 추가하거나 제거하는 건 잘 안될겁니다! 데이터를 추가하는 것을 *스택에 푸시하기
> (pushing on the stack)*라고 부르고, 데이터를 제거하는 것을 *스택을 팝하기 (popping off the
> stack)*라고 부릅니다.
>
> 스택은 데이터에 접근하는 방식 덕택에 빠릅니다: 이 방식은 새로운 데이터를 넣어두기 위한 공간 혹은 데이터를
> 가져올 공간을 검색할 필요가 전혀 없는데, 바로 그 공간이 항상 스택의 꼭대기(top)이기 때문입니다. 스택을
> 빠르게 해주는 또다른 특성은 스택에 담긴 모든 데이터가 결정되어 있는 고정된 크기를 갖고 있어야 한다는
> 점입니다.
>
> 컴파일 타임에 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 위해서는, 힙에 데이터를 저장할
> 수 있습니다. 힙은 조금 더 복잡합니다: 데이터를 힙에 넣을때, 먼저 저장할 공간이 있는지 물어봅니다. 그러면
> 운영체제가 충분히 커다란 힙 안의 빈 어떤 지점을 찾아서 이 곳을 사용중이라고 표시하고, 해당 지점의 포인터를
> 우리에게 돌려주죠. 이 절차를 *힙 공간 할당하기(allocating on the heap)*이라 부르고, 종종 그냥
> "할당(allocating)"으로 줄여 부릅니다. 스택에 포인터를 푸싱하는 것은 할당에 해당되지 않습니다. 포인터는
> 결정되어 있는 고정된 크기의 값이므로, 우리는 스택에 포인터를 저장할 수 있지만, 실제 데이터를 사용하고자
> 할 때는 포인터를 따라가야 합니다.
>
> 힙에 저장된 데이터에 접근하는 것은 스택에 저장된 데이터에 접근하는 것보다 느린데, 그 이유는 포인터가
> 가리킨 곳을 따라가야 하기 때문입니다. 현대 프로세서들은 메모리 내부를 덜 뛰어다닐 때 더 빨라집니다. 유사한
> 예로, 여러 테이블로부터 주문을 받는 레스토랑의 웨이터를 생각해보세요. 다음 테이블로 움직이기 전에 지금
> 테이블에서 모든 주문을 다 받는 것이 가장 효율적이겠죠. A 테이블에서 하나 주문 받고, 다시 B 테이블로
> 가서 하나 주문 받고, 다시 A로, 다시 B로 가며 하나씩 주문을 받으면 훨씬 느려질 겁니다. 이와 마찬가지로,
> 프로세서는 (힙에 있는 데이터와 같이) 멀리 떨어져 있는 데이터들 보다는 (스택에 있는 것과 같이) 붙어있는
> 데이터들에 대한 작업을 하면 더 빨라집니다. 힙으로부터 큰 공간을 할당받는것 또한 시간이 걸릴 수 있습니다.
>
> 코드의 어느 부분이 힙의 어떤 데이터를 사용하는지 추적하는 것, 힙의 중복된 데이터의 양을 최소화하는 것,
> 그리고 힙 내에 사용하지 않는 데이터를 제거하여 공간이 모자라지 않게 하는 것은 모두 소유권과 관계된
> 문제들입니다. 여러분이 소유권을 이해하고 나면, 여러분은 더이상 스택과 힙에 대한 생각이 자주 필요치 않게
> 될겁니다만, 힙 데이터를 관리하는 것이 곧 소유권의 존재 이유임을 알게 되는 것은 이것이 어떤 방식으로
> 작동하는지 설명하는데 도움을 줄 수 있습니다.
>
<!-- PROD: END BOX -->


### 소유권 규칙

먼저, 소유권 규칙을 알아봅시다. 이것들을 설명할 예제들을 보는 내내 다음의 소유권 규칙들을 명심하세요:

> 1. 러스트의 각각의 값은 해당값의 *오너(owner)*라고 불리우는 변수를 갖고 있다.
> 2. 한번에 딱 하나의 오너만 존재할 수 있다.
> 3. 오너가 스코프 밖으로 벗어나는 때, 값은 내려간다(dropped).


### 변수의 스코프

우리는 이미 2장에서 완성된 형태의 러스트 프로그램 예제를 살펴봤습니다. 이제 과거의 기초 문법 형태로
돌아가서, `fn main() {` 코드를 예제에 붙이지 않을테니, 여러분들이 코드를 따라하려면 `main` 함수에
직접 예제들을 넣어야 할 겁니다. 결과적으로, 우리의 예제들은 좀더 간략해저셔 보일러 플레이트 코드에 비해
실제 디테일에 초점을 맞출 수 있도록 해줄 것입니다.

소유권에 대한 첫 예제로서, 변수들의 스코프를 보겠습니다. 스코프란 프로그램 내에서 아이템이 유효함을
표시하기 위한 범위입니다. 아래처럼 생긴 변수가 있다고 해봅시다:

```rust
let s = "hello";
```

변수 `s`는 문자열 리터럴을 나타내는데, 문자열 리터럴의 값은 우리의 프로그램의 텍스트 내에 하드코딩되어
있습니다. 변수는 선언된 시점부터 현재의 *스코프*가 끝날 때까지 유효합니다. 아래 예제 Listing 4-1은
변수 `s`가 유효한 지점을 주석으로 표시했습니다:

```rust
{                      // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
    let s = "hello";   // s는 이 지점부터 유효합니다.

    // s를 가지고 뭔가 합니다.
}                      // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.
```

<span class="caption">Listing 4-1: 변수와 이 변수가 유효한 스코프</span>

바꿔 말하면, 두가지 중요한 지점이 있습니다:

1. 스코프 *안에서* `s`가 등장하면, 유효합니다.
1. 이 유효기간은 스코프 *밖으로* 벗어날 때까지 지속됩니다.

이 지점에서, 스코프와 변수가 유효한 시점 간의 관계는 다른 프로그래밍 언어와 비슷합니다. 이제 우리는
이에 대한 이해를 기초로 하여 `String` 타입을 소개함으로써 계속 쌓아나갈 것입니다.

### `String` 타입

소유권 규칙을 설명하기 위하여, 우리는 3장에서 다룬 바 있는 타입보다 더 복잡한 데이터 타입이
필요합니다. 우리가 이전에 봐온 모든 데이터 타입들은 스텍에 저장되었다가 스코프를 벗어날 때
스택으로부터 팝 됩니다만, 우리는 이제 힙에 저장되는 데이터를 관찰하고 러스트는 과연 어떻게 이
데이터를 비워내는지 설명할 필요가 있습니다.

우리는 여기서 `String`을 예제로 활용하되, 소유권과 관련된 `String` 내용의 일부분에 집중할
것입니다. 이러한 관점은 표준 라이브러리나 여러분들이 만들 다른 복잡한 데이터 타입에도 적용됩니다.
`String`에 대해서는 8장에서 더 자세히 다루겠습니다.

문자열 리터럴을 이미 봤는데, 이 값은 프로그램 안에 하드코딩 되어 있습니다. 문자열 값은 편리하지만,
여러분이 텍스트를 필요로 하는 모든 경우에 대해 항상 적절하진 않습니다. 그 중 한가지 이유로, 문자열
값은 불변입니다(immutable). 또다른 이유는 모든 문자열이 우리가 프로그래밍 하는 시점에서 다 알수
있는 것이 아니란 점입니다: 예를 들면, 사용자의 입력을 받아 저장하고 싶다면요? 이러한 경우들에 대해서,
러스트는 두번째 문자열 타입인 `String`을 제공합니다. 이 타입은 힙에 할당되고 그런고로 컴파일 타임에는
우리가 알 수 없는 양의 텍스트를 저장할 수 있습니다. 여러분은 문자열 리터럴로부터 `from`이라는 함수를
이용해서 `String`을 아래처럼 만들 수 있습니다:

```rust
let s = String::from("hello");
```

더블 콜론(`::`)은 우리가 `string_from`과 같은 이름을 쓰기 보다는 `String` 타입 아래의
`from` 함수를 특정지을 수 있도록 해주는 네임스페이스 연산자입니다. 우리는 이러한 문접에 대해 5장의
"메소드 문법" 부분에서 더 자세히 다룰 것이고, 모듈에서의 네임스페이스와 관련한 이야기는 7장에서 할
것입니다.

이러한 종류의 문자열은 변경 가능합니다:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여줍니다.

println!("{}", s); // 이 부분이 `hello, world!`를 출력할 겁니다.
```

그러니까, 여기서 어떤게 달라졌나요?, 왜 `String`은 변할 수 있는데 스트링 리터럴은 안될까요?
차이점은 두 타입이 메모리를 쓰는 방식에 있습니다.

### Memory and Allocation

In the case of a string literal, we know the contents at compile time so the
text is hardcoded directly into the final executable, making string literals
fast and efficient. But these properties only come from its immutability.
Unfortunately, we can’t put a blob of memory into the binary for each piece of
text whose size is unknown at compile time and whose size might change while
running the program.

With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means:

1. The memory must be requested from the operating system at runtime.
2. We need a way of returning this memory to the operating system when we’re
done with our `String`.

That first part is done by us: when we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages.

However, the second part is different. In languages with a *garbage collector
(GC)*, the GC keeps track and cleans up memory that isn’t being used anymore,
and we, as the programmer, don’t need to think about it. Without a GC, it’s the
programmer’s responsibility to identify when memory is no longer being used and
call code to explicitly return it, just as we did to request it. Doing this
correctly has historically been a difficult programming problem. If we forget,
we’ll waste memory. If we do it too early, we’ll have an invalid variable. If
we do it twice, that’s a bug too. We need to pair exactly one `allocate` with
exactly one `free`.

Rust takes a different path: the memory is automatically returned once the
variable that owns it goes out of scope. Here’s a version of our scope example
from Listing 4-1 using a `String` instead of a string literal:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

There is a natural point at which we can return the memory our `String` needs
to the operating system: when `s` goes out of scope. When a variable goes out
of scope, Rust calls a special function for us. This function is called `drop`,
and it’s where the author of `String` can put the code to return the memory.
Rust calls `drop` automatically at the closing `}`.

> Note: In C++, this pattern of deallocating resources at the end of an item's
> lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*.
> The `drop` function in Rust will be familiar to you if you’ve used RAII
> patterns.

This pattern has a profound impact on the way Rust code is written. It may seem
simple right now, but the behavior of code can be unexpected in more
complicated situations when we want to have multiple variables use the data
we’ve allocated on the heap. Let’s explore some of those situations now.

#### Ways Variables and Data Interact: Move

Multiple variables can interact with the same data in different ways in Rust.
Let’s look at an example using an integer in Listing 4-2:

```rust
let x = 5;
let y = x;
```

<span class="caption">Listing 4-2: Assigning the integer value of variable `x`
to `y`</span>

We can probably guess what this is doing based on our experience with other
languages: “Bind the value `5` to `x`; then make a copy of the value in `x` and
bind it to `y`.” We now have two variables, `x` and `y`, and both equal `5`.
This is indeed what is happening because integers are simple values with a
known, fixed size, and these two `5` values are pushed onto the stack.

Now let’s look at the `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar to the previous code, so we might assume that the way
it works would be the same: that is, the second line would make a copy of the
value in `s1` and bind it to `s2`. But this isn’t quite what happens.

To explain this more thoroughly, let’s look at what `String` looks like under
the covers in Figure 4-3. A `String` is made up of three parts, shown on the
left: a pointer to the memory that holds the contents of the string, a length,
and a capacity. This group of data is stored on the stack. On the right is the
memory on the heap that holds the contents.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-3: Representation in memory of a `String`
holding the value `"hello"` bound to `s1`</span>

The length is how much memory, in bytes, the contents of the `String` is
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has received from the operating system. The difference between length
and capacity matters, but not in this context, so for now, it’s fine to ignore
the capacity.

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. In other words, the data
representation in memory looks like Figure 4-4.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-4: Representation in memory of the variable `s2`
that has a copy of the pointer, length, and capacity of `s1`</span>

The representation does *not* look like Figure 4-5, which is what memory would
look like if Rust instead copied the heap data as well. If Rust did this, the
operation `s2 = s1` could potentially be very expensive in terms of runtime
performance if the data on the heap was large.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-5: Another possibility of what `s2 = s1` might
do if Rust copied the heap data as well</span>

Earlier, we said that when a variable goes out of scope, Rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
Figure 4-4 shows both data pointers pointing to the same location. This is a
problem: when `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a *double free* error and is one of the memory
safety bugs we mentioned previously. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, there’s one more detail to what happens in this
situation in Rust. Instead of trying to copy the allocated memory, Rust
considers `s1` to no longer be valid and therefore, Rust doesn’t need to free
anything when `s1` goes out of scope. Check out what happens when you try to
use `s1` after `s2` is created:

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);
```

You’ll get an error like this because Rust prevents you from using the
invalidated reference:

```text
error[E0382]: use of moved value: `s1`
 --> src/main.rs:4:27
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
which does not implement the `Copy` trait
```

If you’ve heard the terms “shallow copy” and “deep copy” while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounds like a shallow copy. But because Rust
also invalidates the first variable, instead of calling this a shallow copy,
it’s known as a *move*. Here we would read this by saying that `s1` was *moved*
into `s2`. So what actually happens is shown in Figure 4-6.

<img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-6: Representation in memory after `s1` has been
invalidated</span>

That solves our problem! With only `s2` valid, when it goes out of scope, it
alone will free the memory, and we’re done.

In addition, there’s a design choice that’s implied by this: Rust will never
automatically create “deep” copies of your data. Therefore, any *automatic*
copying can be assumed to be inexpensive in terms of runtime performance.

#### Ways Variables and Data Interact: Clone

If we *do* want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`. We’ll discuss method
syntax in Chapter 5, but because methods are a common feature in many
programming languages, you’ve probably seen them before.

Here’s an example of the `clone` method in action:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This works just fine and is how you can explicitly produce the behavior shown
in Figure 4-5, where the heap data *does* get copied.

When you see a call to `clone`, you know that some arbitrary code is being
executed and that code may be expensive. It’s a visual indicator that something
different is going on.

#### Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code using integers,
part of which was shown earlier in Listing 4-2, works and is valid:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

But this code seems to contradict what we just learned: we don’t have a call to
`clone`, but `x` is still valid and wasn’t moved into `y`.

The reason is that types like integers that have a known size at compile time
are stored entirely on the stack, so copies of the actual values are quick to
make. That means there’s no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there’s no difference
between deep and shallow copying here, so calling `clone` wouldn’t do anything
differently from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on
types like integers that are stored on the stack (we’ll talk more about traits
in Chapter 10). If a type has the `Copy` trait, an older variable is still
usable after assignment. Rust won’t let us annotate a type with the `Copy`
trait if the type, or any of its parts, has implemented the `Drop` trait. If
the type needs something special to happen when the value goes out of scope and
we add the `Copy` annotation to that type, we’ll get a compile time error. To
learn about how to add the `Copy` annotation to your type, see Appendix C on
Derivable Traits.

So what types are `Copy`? You can check the documentation for the given type to
be sure, but as a general rule, any group of simple scalar values can be
`Copy`, and nothing that requires allocation or is some form of resource is
`Copy`. Here are some of the types that are `Copy`:

* All the integer types, like `u32`.
* The boolean type, `bool`, with values `true` and `false`.
* All the floating point types, like `f64`.
* Tuples, but only if they contain types that are also `Copy`. `(i32, i32)` is
`Copy`, but `(i32, String)` is not.

### Ownership and Functions

The semantics for passing a value to a function are similar to assigning a
value to a variable. Passing a variable to a function will move or copy, just
like assignment. Listing 4-7 has an example with some annotations showing where
variables go into and out of scope:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

<span class="caption">Listing 4-7: Functions with ownership and scope
annotated</span>

If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile time error. These static checks protect us from mistakes. Try adding
code to `main` that uses `s` and `x` to see where you can use them and where
the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Here’s an example with similar
annotations to those in Listing 4-7:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope.

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}
```

The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless the data
has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit
tedious. What if we want to let a function use a value but not take ownership?
It’s quite annoying that anything we pass in also needs to be passed back if we
want to use it again, in addition to any data resulting from the body of the
function that we might want to return as well.

It’s possible to return multiple values using a tuple, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for this concept, and it’s called
*references*.
