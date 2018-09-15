## 공유 상태 동시성

메세지 패싱은 동시성을 다루는 좋은 방법이지만, 유일한 수단은
아닙니다. Go 언어 문서로부터 나온 슬로건의 일부를 다시한번 고려해보죠:
“메모리를 공유함으로써 소통하세요.”

메모리를 공유하는 통신은 어떤 형태로 보일까요? 더불어서 메세지 패싱의
열광적인 지지자들은 왜 이걸 안쓰고 대신 반대편의 것을 쓸까요?

어떤 면에서, 프로그래밍 언어의 채널들은 단일 소유권과 유사한데,
이는 여러분이 채널로 값을 송신하면, 그 값을 더이상 쓸 수 없게되기
때문입니다. 공유 메모리 동시성은 복수 소유권과 유사합니다: 복수개의
스레드들이 동시에 동일한 메모리 위치를 접근할 수 있지요. 스마트 포인터들이
복수 소유권을 가능하게 만드는 내용을 담은 15장에서 보셨듯이, 복수 소유권은
이 서로 다른 소유자들의 관리가 필요하기 때문에 복잡성을 더할 수 있습니다.
러스트의 타입 시스템과 소유권 규칙은 이러한 관리를 올바르도록 훌륭히 유도합니다.
예를 들면, 공유 메모리를 위한 더 일반적인 동시성의 기초 재료 중 하나인
뮤텍스 (mutex)를 살펴 봅시다.

### 뮤텍스를 사용하여 한번에 한 스레드에서의 데이터 접근을 허용하기

*뮤텍스*는 *상호 배제 (mutual exclusion)* 의 줄임말로서, 내부에서 뮤텍스는
주어진 시간에 오직 하나의 스레드만 데이터 접근을 허용합니다. 뮤텍스 내부의 데이터에
접근하기 위해서 스레드는 먼저 뮤텍스의 *락 (lock)* 을 얻기를 요청함으로써 접근을
윈한다는 신호를 보내야 합니다. 락은 누가 배타적으로 데이터에 접근하는지를 추적하는
뮤텍스의 부분인 데이터 구조입니다. 그러므로, 뮤텍스는 잠금 시스템을 통해 가지고 있는
데이터를 *보호하는* 것으로 묘사됩니다.

뮤텍스는 사용하기 어렵다는 평판을 가지고 있는데 이는 여러분이 다음 두 가지 규칙을
기억해야 하기 때문입니다:

* 여러분은 데이터를 사용하기 전에 반드시 락을 얻는 시도를 해야 합니다.
* 만일 뮤텍스가 보호하는 데이터의 사용이 끝났다면, 다른 스레드들이 락을
  얻을 수 있도록 반드시 언락해야 합니다.

뮤텍스에 대한 실세계 은유를 위해서, 마이크가 딱 하나만 있는 컨퍼런스 패널
토의를 상상해보세요. 패널 참가자들이 말하기 전, 그들은 마이크 사용을
원한다고 요청하거나 신호를 줘야 합니다. 마이크를 얻었을 때는
원하는 만큼 길게 말을 한 다음 말하기를 원하는 다음 매널 참가자에게
마이크를 건네줍니다. 만일 패널 참여자가 마이크 사용을 끝냈을 때
이를 건네주는 것을 잊어먹는다면, 그 외 아무도 말할 수 없게 됩니다.
공유된 마이크의 관리가 잘못되면, 패널은 계획된데로 되지
않을겁니다!

뮤텍스의 관리는 바로잡기 위해 믿을 수 없으리만치 교묘해질 수 있는데, 이것이 바로
많은 사람들이 체널의 열성 지지자가 되는 이유입니다. 하지만, 러스트의 타입 시스템과
소유권 규칙에 감사하게도, 여러분은 잘못 락을 얻거나 언락 할 수가 없습니다.

#### `Mutex<T>`의 API

어떻게 뮤텍스를 이용하는지에 대한 예제로서, Listing 16-12와 같이 단일
스레드 맥락 내에서 뮤텍스를 사용하는 것으로 시작해봅시다:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

<span class="caption">Listing 16-12: 단순함을 위해 단일 스레드 맥락 내에서
`Mutex<T>`의 API 탐색하기</span>

많은 타입들처럼 `Mutex<T>`는 연관함수 `new`를 사용하여 만들어집니다.
뮤텍스 내의 데이터에 접근하기 위해서는 `lock` 메소드를 사용하여 락을
엇습니다. 이 호출은 현재의 스레드를 막아설 것이므로, 락을 얻는 차례가
될 때까지 아무런 작업도 할 수 없습니다.

`lock`의 호출은 다른 스레드가 패닉 상태의 락을 가지고 있을 경우 실패할 수 있습니다.
그런 경우 아무도 락을 얻을 수 없게 되므로, `unwrap`을 택하여 그런 상황일
경우 이 스레드에 패닉을 일으킵니다.

락을 얻고난 다음에는 그 반환값 (위의 경우에는 `num`이라는 이름의 값) 을 내부의 데이터에
대한 가변 참조자처럼 다룰 수 있습니다. 타입 시스템은 `m` 내부의 값을 사용하기 전에 우리가
락을 얻는 것을 확실히 해줍니다: `Mutex<i32>`는 `i32`가 아니므로 우리는 *반드시*
`i32` 값을 사용하기 위해 락을 얻어야 합니다. 우리는 이를 잊어버릴 수 없습니다;
잊어버린다면 타입 시스템이 내부의 `i32`에 접근할 수 없게 할 것입니다.

여러분이 의심한 것처럼, `Mutex<T>`는 스마트 포인터입니다. 더 정확하게는, `lock`의
호출은 `MutexGuard`라고 불리우는 스마트 포인터를 *반환합니다.* 이 스마트 포인터는
우리의 내부 데이터를 가리키도록 `Deref`가 구현되어 있습니다; 이 스마트 포인터는 또한
`MutexGuard`가 스코프 밖으로 벗어났을 때 자동으로 락을 해제하는 `Drop` 구현체를
가지고 있는데, 이는 Listing 16-12의 내부 스코프의 끝에서 일어나는 일입니다.
결과적으로 락이 자동으로 해제되기 때문에, 우리는 락을 해제하는 것을 잊어버리고
다른 스레드에 의해 뮤텍스가 사용되는 것을 막는 위험을 짊어지지 않아도
됩니다.

락이 버려진 후, 뮤텍스 값을 출력하여 내부의 `i32`를 6으로 바꿀 수 있음을
확인할 수 있습니다.

#### 여러 스레드들 사이에서 `Mutex<T>` 공유하기

이제 `Mutex<T>`를 사용하여 여러 스레드들 사이에서 값을 공유하는 시도를 해봅시다.
우리는 10개의 스레드를 돌리고 이들이 카운터 값을 1만큼씩 증가 시켜서,
카운터가 0에서 10으로 가도록 할 것입니다. 다음 몇 개의 예제가 컴파일
에러가 날 것이고, 우리가 이 에러를 사용하여 `Mutex<T>`를 사용하는 방법과
러스트가 이를 고치는 것을 어떻게 돕는지에 대해 학습할 것임을 주의하세요.
Listing 16-13이 시작 예제입니다:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-13: `Mutex<T>`에 의해 보소되는 카운터를
각자 증가시키는 10개의 스레드</span>

We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we
did in Listing 16-12. Next, we create 10 threads by iterating over a range
of numbers. We use `thread::spawn` and give all the threads the same closure,
one that moves the counter into the thread, acquires a lock on the `Mutex<T>`
by calling the `lock` method, and then adds 1 to the value in the mutex. When a
thread finishes running its closure, `num` will go out of scope and release the
lock so another thread can acquire it.

In the main thread, we collect all the join handles. Then, as we did in Listing
16-2, we call `join` on each handle to make sure all the threads finish. At
that point, the main thread will acquire the lock and print the result of this
program.

We hinted that this example wouldn’t compile. Now let’s find out why!

```text
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:10:27
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:21:29
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

The error message states that the `counter` value is moved into the closure and
then captured when we call `lock`. That description sounds like what we wanted,
but it’s not allowed!

Let’s figure this out by simplifying the program. Instead of making 10 threads
in a `for` loop, let’s just make two threads without a loop and see what
happens. Replace the first `for` loop in Listing 16-13 with this code instead:

```rust,ignore
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

We make two threads and change the variable names used with the second thread
to `handle2` and `num2`. When we run the code this time, compiling gives us the
following:

```text
error[E0382]: capture of moved value: `counter`
  --> src/main.rs:16:24
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:26:29
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
```

Aha! The first error message indicates that `counter` is moved into the closure
for the thread associated with `handle`. That move is preventing us from
capturing `counter` when we try to call `lock` on it and store the result in
`num2` in the second thread! So Rust is telling us that we can’t move ownership
of `counter` into multiple threads. This was hard to see earlier because our
threads were in a loop, and Rust can’t point to different threads in different
iterations of the loop. Let’s fix the compiler error with a multiple-ownership
method we discussed in Chapter 15.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we gave a value multiple owners by using the smart pointer
`Rc<T>` to create a reference counted value. Let’s do the same here and see
what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing 16-14 and clone
the `Rc<T>` before moving ownership to the thread. Now that we’ve seen the
errors, we’ll also switch back to using the `for` loop, and we’ll keep the
`move` keyword with the closure.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-14: Attempting to use `Rc<T>` to allow
multiple threads to own the `Mutex<T>`</span>

Once again, we compile and get... different errors! The compiler is teaching us
a lot.

```text
error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied in `[closure@src/main.rs:11:36:
15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
  --> src/main.rs:11:22
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>`
cannot be sent between threads safely
   |
   = help: within `[closure@src/main.rs:11:36: 15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send` is
not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
   = note: required because it appears within the type
`[closure@src/main.rs:11:36: 15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
   = note: required by `std::thread::spawn`
```

Wow, that error message is very wordy! Here are some important parts to focus
on: the first inline error says `` `std::rc::Rc<std::sync::Mutex<i32>>` cannot
be sent between threads safely ``. The reason for this is in the next important
part to focus on, the error message. The distilled error message says `` the
trait bound `Send` is not satisfied ``. We’ll talk about `Send` in the next
section: it’s one of the traits that ensures the types we use with threads are
meant for use in concurrent situations.

Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped. But it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts—subtle bugs that
could in turn lead to memory leaks or a value being dropped before we’re done
with it. What we need is a type exactly like `Rc<T>` but one that makes changes
to the reference count in a thread-safe way.

#### Atomic Reference Counting with `Arc<T>`

Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in
concurrent situations. The *a* stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here: see the standard library
documentation for `std::sync::atomic` for more details. At this point, you just
need to know that atomics work like primitive types but are safe to share
across threads.

You might then wonder why all primitive types aren’t atomic and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re just performing operations on values within a
single thread, your code can run faster if it doesn’t have to enforce the
guarantees atomics provide.

Let’s return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code in Listing 16-15 will finally compile and run:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

<span class="caption">Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>`
to be able to share ownership across multiple threads</span>

This code will print the following:

```text
Result: 10
```

We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety. You could also use this
program’s structure to do more complicated operations than just incrementing a
counter. Using this strategy, you can divide a calculation into independent
parts, split those parts across threads, and then use a `Mutex<T>` to have each
thread update the final result with its part.

### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

You might have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in
Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>`
to mutate contents inside an `Arc<T>`.

Another detail to note is that Rust can’t protect you from all kinds of logic
errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of
creating *deadlocks*. These occur when an operation needs to lock two resources
and two threads have each acquired one of the locks, causing them to wait for
each other forever. If you’re interested in deadlocks, try creating a Rust
program that has a deadlock; then research deadlock mitigation strategies for
mutexes in any language and have a go at implementing them in Rust. The
standard library API documentation for `Mutex<T>` and `MutexGuard` offers
useful information.

We’ll round out this chapter by talking about the `Send` and `Sync` traits and
how we can use them with custom types.
