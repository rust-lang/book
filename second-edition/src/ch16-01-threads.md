## 스레드를 이용하여 코드를 동시에 실행하기

대부분의 요즘 운영 체제에서, 실행되는 프로그램의 코드는 *프로세스* 내에서 실행되고,
운영 체제는 한번에 여러 개의 프로세스들을 관리합니다. 여러분의 프로그램 내에서도
동시에 실행되는 독립적인 부분들을 가질 수 있습니다. 이러한 독립적인 부분들을 실행하는
기능을 *스레드*라고 부릅니다.

여러분의 프로그램 내에 계산 부분을 여러 개의 스레드로 쪼개는 것은 프로그램이
동시에 여러 개의 일을 할 수 있기 때문에 성능을 향상시킬 수 있지만, 프로그램을
복잡하게 만들기도 합니다. 스레드가 동시에 실행될 수 있기 때문에, 다른 스레드
상에서 실행될 여러분의 코드 조각들의 실행 순서에 대한 내재적인 보장이 없습니다.
이는 다음과 같은 문제들을 야기할 수 있습니다:

* 여러 스레드들이 일관성 없는 순서로 데이터 혹은 리소스에 접근하게 되는, 경쟁 조건
  (race condition)
* 두 스레드가 서로 상대방 스레드가 가지고 있는 리소스의 사용을 끝내길 기다려서
  양쪽 스레드 모두 계속 실행되는 것을 막아버리는, 데드록 (deadlock)
* 특정한 상황에서만 발생되어 재현하기와 안정적으로 수정하기가 힘든
  버그들

러스트는 스레드 사용의 부정적인 효과를 줄이려는 시도를 하지만, 다중
스레드 콘텍스트 내에서의 프로그래밍은 여전히 신중하게 생각해야 하고
단일 스레드 내에서 실행되는 프로그램의 것과는 다른 코드 구조가
필요합니다.

프로그래밍 언어들은 몇가지 다른 방식으로 스레드를 구현합니다. 많은 운영 체제들이
새로운 스레드를 만들기 위한 API를 제공합니다. 언어가 운영 체제의 API를 호출하여
스레드를 만드는 이러한 구조는 때때로 *1:1*이라 불리는데, 이는 하나의 운영 체제
스레드가 하나의 언어 스레드에 대응된다는 의미입니다.

많은 프로그래밍 언어들은 그들만의 특별한 스레드 구현을 제공합니다. 프로그래밍
언어가 제공하는 스레드는 *그린 (green)* 스레드라고 알려져 있으며, 이러한
그린 스레드를 사용하는 언어들은 다른 숫자의 운영 체제 스레드로 구성된 콘텍스트
내에서 그린 스레드들을 실행할 것입니다. 이러한 이유로 인하여 그린 스레드 구조는
*M:N*이라고 불립니다: `M` 개의 그린 스레드가 `N` 개의 시스템 스레드에
대응되는데, 여기서 `M`과 `N`은 굳이 동일한 숫자가 아니어도
됩니다.

각각의 구조는 고유한 장점과 트레이드 오프를 가지고 있으며, 러스트에게 있어
가장 중요한 트레이드 오프는 런타임 지원입니다. *런타임*은 혼동하기 쉬운 용어이고
맥락에 따라 다른 의미를 가질 수 있습니다.

이 글의 맥락에서 *런타임*이라 하는 것은 언어에 의해 모든 바이너리 내에
포함되는 코드를 의미합니다. 이 코드는 언어에 따라 크거나 작을 수 있지만,
모든 어셈블리 아닌 언어들은 어느 정도 크기의 런타임 코드를 가지게 될 것입니다.
이러한 이유로 인하여, 흔히 사람들이 “런타임이 없다”라고 말할 때는, 종종
“런타임이 작다”는 것을 의미하는 것입니다. 런타임이 작을 수록 더 적은
기능을 갖지만 더 작아진 바이너리로 인해 얻어지는 장점을 갖는데, 이는
더 큰 콘텍스트 내에서 다른 언어들과 조합하기 쉬워진다는 점입니다. 비록
많은 언어들이 더 많은 기능을 위하여 런타임 크기를 늘리는 거래를 수락하더라도,
러스트는 거의 런타임이 없을 필요가 있고 성능을 관리하기 위해 C를 호출하는
것에 대해 타협할 수 없습니다.

그린 스레드 M:N 구조는 스레드들을 관리하기 위해 더 큰 언어 런타임이
필요합니다. 그런 이유로 러스트 표준 라이브러리는 오직 1:1 스레드 구현만
제공합니다. 러스트가 이러한 저수준 언어이기 때문에, 여러분이 예를 들어 어떤
스레드를 언제 실행시킬지에 대한 더 많은 제어권과 콘텍스트 교환(context
switching)의 더 저렴한 비용 같은 관점을 위해 오버헤드와 맞바꾸겠다면
M:N 스레드를 구현한 크레이트도 있습니다.

이제 러스트에서의 스레드를 정의했으니, 표준 라이브러리가 제공하는 스레드
관련 API를 어떻게 사용하는지를 탐구해봅시다.

### `spawn`으로 새로운 스레드 생성하기

새로운 스레드를 생성하기 위해서는 `thread::spawn` 함수를 호출하고 여기에
우리가 새로운 스레드 내에서 실행하기를 원하는 코드가 담겨 있는 클로저를 넘깁니다
(클로저에 대해서는 13장에서 다뤘습니다). Listing 16-1의 예제는 메인 스레드에서
어떤 텍스트를 출력하고 새로운 스레드에서는 다른 텍스트를 출력합니다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

<span class="caption">Listing 16-1: 메인 스레드에서 무언가를 출력하는 동안
다른 것을 출력하는 새로운 스레드 생성하기</span>

이 함수를 가지고, 새로운 스레드는 실행이 종료되었든 혹은 그렇지 않든
메인 스레드가 종료될 때 멈추게 될 것이라는 점을 주의하세요. 이
프로그램의 출력은 매번 약간씩 다를지도 모르겠으나, 아래와 비슷하게 보일
것입니다:

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep`의 호출은 강제로 스레드가 잠깐 동안 실행을 멈추게 하는데,
다른 스레드가 실행되는 것을 허용합니다. 스레드들은 아마도 교대로 실행될 것이지만,
보장되지는 않습니다: 여러분의 운영 체제가 어떻게 스레드를 스케줄링 하는지에 따라
달린 문제입니다. 위의 실행 예에서는 생성된 스레드로부터의 출력 구문이 코드의
첫번째에 나타나 있음에도 불구하고 메인 스레드가 먼저 출력하였습니다. 그리고
생성된 스레드에게 `i`가 9일때까지 출력하라고 했음에도 불구하고, 메인 스레드가
멈추기 전까지 고작 5에 도달했습니다.

만일 여러분이 이 코드를 실행하고 메인 스레드로부터의 출력만 보았다면, 혹은 어떠한
오버랩도 보지 못했다면, 숫자 범위를 늘려서 운영 체제로 하여금 스레드간의 전환에
더 많은 기회를 주는 시도를 해보세요.

### Waiting for All Threads to Finish Using `join` Handles

The code in Listing 16-1 not only stops the spawned thread prematurely most of
the time due to the main thread ending, but also can't guarantee that the
spawned thread will get to run at all. The reason is that there is no guarantee
on the order in which threads run!

We can fix the problem of the spawned thread not getting to run, or not getting
to run completely, by saving the return value of `thread::spawn` in a variable.
The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned
value that, when we call the `join` method on it, will wait for its thread to
finish. Listing 16-2 shows how to use the `JoinHandle` of the thread we created
in Listing 16-1 and call `join` to make sure the spawned thread finishes before
`main` exits:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-2: Saving a `JoinHandle` from `thread::spawn`
to guarantee the thread is run to completion</span>

Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. *Blocking* a thread means that
thread is prevented from performing work or exiting. Because we’ve put the call
to `join` after the main thread’s `for` loop, running Listing 16-2 should
produce output similar to this:

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

The two threads continue alternating, but the main thread waits because of the
call to `handle.join()` and does not end until the spawned thread is finished.

But let’s see what happens when we instead move `handle.join()` before the
`for` loop in `main`, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

The main thread will wait for the spawned thread to finish and then run its
`for` loop, so the output won’t be interleaved anymore, as shown here:

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

Small details, such as where `join` is called, can affect whether or not your
threads run at the same time.

### Using `move` Closures with Threads

The `move` closure is often used alongside `thread::spawn` because it allows
you to use data from one thread in another thread.

In Chapter 13, we mentioned we can use the `move` keyword before the parameter
list of a closure to force the closure to take ownership of the values it uses
in the environment. This technique is especially useful when creating new
threads in order to transfer ownership of values from one thread to another.

Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. To use data from the main thread in the spawned thread, the
spawned thread’s closure must capture the values it needs. Listing 16-3 shows
an attempt to create a vector in the main thread and use it in the spawned
thread. However, this won’t yet work, as you’ll see in a moment.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-3: Attempting to use a vector created by the
main thread in another thread</span>

The closure uses `v`, so it will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread. But when we compile this
example, we get the following error:

```text
error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

Rust *infers* how to capture `v`, and because `println!` only needs a reference
to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t
tell how long the spawned thread will run, so it doesn’t know if the reference
to `v` will always be valid.

Listing 16-4 provides a scenario that’s more likely to have a reference to `v`
that won’t be valid:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-4: A thread with a closure that attempts to
capture a reference to `v` from a main thread that drops `v`</span>

If we were allowed to run this code, there’s a possibility the spawned thread
would be immediately put in the background without running at all. The spawned
thread has a reference to `v` inside, but the main thread immediately drops
`v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!

To fix the compiler error in Listing 16-3, we can use the error message’s
advice:

```text
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```

By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using rather than allowing Rust to infer that it
should borrow the values. The modification to Listing 16-3 shown in Listing
16-5 will compile and run as we intend:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

<span class="caption">Listing 16-5: Using the `move` keyword to force a closure
to take ownership of the values it uses</span>

What would happen to the code in Listing 16-4 where the main thread called
`drop` if we use a `move` closure? Would `move` fix that case? Unfortunately,
no; we would get a different error because what Listing 16-4 is trying to do
isn’t allowed for a different reason. If we added `move` to the closure, we
would move `v` into the closure’s environment, and we could no longer call
`drop` on it in the main thread. We would get this compiler error instead:

```text
error[E0382]: use of moved value: `v`
  --> src/main.rs:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
```

Rust’s ownership rules have saved us again! We got an error from the code in
Listing 16-3 because Rust was being conservative and only borrowing `v` for the
thread, which meant the main thread could theoretically invalidate the spawned
thread’s reference. By telling Rust to move ownership of `v` to the spawned
thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If
we change Listing 16-4 in the same way, we’re then violating the ownership
rules when we try to use `v` in the main thread. The `move` keyword overrides
Rust’s conservative default of borrowing; it doesn’t let us violate the
ownership rules.

With a basic understanding of threads and the thread API, let’s look at what we
can *do* with threads.
