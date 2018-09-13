## 서버를 싱글 스레드에서 멀티 스레드로 바꾸기

현재, 서버는 한번에 하나의 요청만 처리할 것입니다.
즉 첫번째 요청에 대한 작업이 끝나기 전에 두번째 요청이 들어온다면
앞선 작업이 끝날때까지 대기하게 됩니다. 만약 서버가 훨씬 더 많은
요청을 받게 된다면, 처리는 점점 더 늦어지게 됩니다.
나중에 들어온 요청은 앞선 요청보다 더 빠르게 처리 될 수 있더라도
긴 시간을 기다려야 할 것입니다.
우린 이 문제를 해결해야 합니다만, 먼저 현재 우리의 문제를 살펴보도록 하죠.

### 현재 서버에서 느린 요청을 시뮬레이팅하기

우린 현재의 우리가 만든 서버에서 느린 요청이 어떻게
다른 요청들에게 영향을 미칠 수 있는지 살펴 볼 것입니다.
Listing 20-10은 `/sleep` 요청을 처리할때 응답하기 전에
5초간 서버를 멈추도록 하여 느린 요청을 시뮬레이션 합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
use std::thread;
use std::time::Duration;
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --생략--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
    // --생략--

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // --생략--
}
```

<span class="caption">Listing 20-10: `/sleep` 요청을 인식할시 5초간
멈춤으로써 느린 요청을 시뮬레이션 하기</span>

이 코드는 좀 지저분하지만 시뮬레이션 용도로는 충분합니다.
우리는 우리 서버가 인식할 두번째 요청인 `sleep` 을 생성하고.
`/sleep` 으로의 요청을 처리할 `else if` 를
`if` 블록 뒤에 추가했습니다. 만약 요청이 들어오면,
서버는 HTML 페이지를 렌더링 하기 전에 5초간 대기할 것입니다.

여러분은 우리 서버가 얼마나 부족한지 알 수 있습니다:
실제 라이브러리들은 훨씬 간단한 방법으로 여러개의 요청을 구분할 것입니다!

`cargo run` 를 이용해 서버를 실행시키고, 두 브라우저 창을 엽니다:
하나는 `http://localhost:7878/` 로 접속하고 다른 하나는 `http://localhost:7878/sleep` 으로 접속합니다.
만약 여러분이 `/` URI로 몇번 접속하시면 기존 처럼
빠른 응답을 보실 수 있으실 테지만, `/sleep` 으로 접속하고 `/` 로 접속한다면
`sleep` 이 5초동안의 로딩을 끝내고 나서야 `/` 에 대한 응답을 보실 수 있을겁니다.

우리 웹 서버가 모든 요청들을 느린 요청 뒤에 처리하도록 하는것을
피하는 방법은 여러가지가 있지만, 그중 우리가 사용할 방법은
스레드 풀 (thread pool) 입니다.

### 스레드 풀을 이용한 처리량 증진

*스레드 풀* 은 대기중이거나 작업을 처리할 준비가 되어 있는
스레드들의 그룹입니다. 프로그램이 새 작업을 받았을때,
스레드 풀은 작업을 풀(pool) 안에 있는 스레드중 하나에게 맡기고
해당 스레드가 작업을 처리하도록 합니다. 남은 스레드들은
첫번째 스레드가 처리중인 동안 들어온 작업을 언제든지 처리할 수
있도록 합니다. 첫번째 스레드가 작업을 끝마치면 풀로 돌아와
작업 대기상태가 됩니다. 스레드 풀은 우리가 여러 커넥션들을
동시에 처리할 수 있게 해주고 우리 서버의 처리량을 증가시킵니다.

우린 DoS (Denial of Service) 공격을 막기 위해 풀 안의
스레드 개수에 대한 제한을 작게 둘 것입니다;
만약 우리 프로그램이 각각의 요청이 들어올때마다 새 스레드를 생성한다면
누군가 우리 서버에 10만개의 요청을 보냈을때 우리 서버는
서버의 모든 리소스를 사용하고 모든 요청이 끝날때까지 처리가 계속될 것입니다.

우린 스레드를 제한없이 생성하는것이 아닌 풀 안에서 대기할
고정된 개수의 스레드를 가질 것입니다. 요청이 들어온다면,
요청들은 처리를 위해 풀로 보내지고, 풀에선 들어오는 요청들에
대한 큐(queue) 를 유지할 것입니다. 풀 내의 각 스레드들은 이 큐에서
요청을 꺼내서 처리하고 또 다른 요청이 있는지 큐에 물어봅니다.
우린 이 형태를 이용해 동시에 `N` 개의 요청을 처리할 수 있습니다. 여기서 `N` 은 스레드의 개수입니다.
만약 각각의 스레드가 응답하는데 오래 걸리는 요청을 처리하게되면
그 다음의 요청들은 여전히 큐에 남아있게 됩니다만,
이전보다 처리할 수 있는 요청은 늘어났습니다

이 기술은 우리 웹서버의 처리량을 증가시킬 수많은 방법중 하나일 뿐입니다.
여러분이 찾으실 다른 방법들은 fork/join 모델과 싱글 스레드 기반
비동기 I/O 모델 등일 것입니다. 만약 여러분이 이러한 내용에 관심이
있으시다면, 다른 해결책들에 대해 좀 더 자세히 찾아보시고 Rust로 구현해 보세요;
Rust같은 저레벨 언어로는 이와 같은 방법들이 전부 가능합니다.

스레드 풀을 구현하기 전에, 풀이 어떻게 쓰여야 할지 이야기 해 봅시다.
여러분이 코드를 디자인할때, 클라이언트 인터페이스를 먼저 작성해 보는건
여러분의 디자인에 도움이 될 수 있습니다.
코드의 API 를 작성하여 원하는 방식으로 구성한 다음
기능을 구현하고 공개 API를 디자인하는 대신
해당 구조 내에서 기능을 구현하세요.

12장의 프로젝트에서 테스트 주도 개발을 할때와 흡사하게,
우린 여기서 컴파일러 주도 개발을 할 것입니다. 이는 우리가 원하는대로
기능을 호출하는 코드를 작성하고, 컴파일러로부터의 에러를 조사하여
어떻게 코드를 변화시켜야 작동시킬 수 있을지 알아내는 과정을 말합니다.

#### 요청마다 스레드를 생성할 수 있는 코드 구조

먼저, 모든 연결에 대해 스레드를 새로 생성했을때의 코드는 어떤 모습이 될지 알아봅시다.
물론 앞에서 말했듯이, 이는 스레드들을 무한대로 만들어낼 수 있기 때문에
문제를 해결하기 위한 최종적인 대책은 될 수 없습니다만,
그에 대한 출발점 정도로는 볼 수 있습니다.
Listing 20-11는 `main` 함수의 `for` 반복문을 모든 요청에 대해 새 스레드를 생성하도록 변경한 모습을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">Listing 20-11: 매 요청마다
새 스레드 생성</span>

여러분이 16장에서 배우신대로, `thread::spawn` 은 새 스레드를 생성하고,
내부에 있는 클로저의 코드를 실행합니다.
만약 여러분이 이 코드를 실행하고 브라우저로 `/sleep` 으로 접속하신 후,
둘 이상의 브라우저 탭으로 `/` 에 접속하신다면, `/` 로의 요청이
`/sleep` 이 끝나길 기다리지 않고 완료 되는 것을 보실 수 있을 것입니다.
하지만 말했듯이, 스레드를 무한정 생성하는 것은 결국 시스템의 과부화를 일으킬 것입니다.

#### 유한 스레드 수를 위한 인터페이스 만들기

우린 스레드 풀을 비슷하고 익숙하게 작동하도록 만들어서
스레드 풀 방식으로 변경할때 우리 API를 사용하는
코드를 크게 변경할 필요가 없도록 하고자 합니다.
Listing 20-12는 `thread::spawn` 대신 이용하고자 하는 `ThreadPool` 이라는 가상의 인터페이스를 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust,no_run
# use std::thread;
# use std::io::prelude::*;
# use std::net::TcpListener;
# use std::net::TcpStream;
# struct ThreadPool;
# impl ThreadPool {
#    fn new(size: u32) -> ThreadPool { ThreadPool }
#    fn execute<F>(&self, f: F)
#        where F: FnOnce() + Send + 'static {}
# }
#
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
# fn handle_connection(mut stream: TcpStream) {}
```

<span class="caption">Listing 20-12: 우리의 이상적인 `ThreadPool` 인터페이스</span>

우린 새로운 스레드 풀을 만들때 `ThreadPool::new` 를
설정할 스레드의 개수를 나타내는 수(이 경우는 4)와 함께 사용했습니다.
그 후 `for` 반복문에선 `thread::spawn` 과 비슷한 인터페이스를 가진 `pool.execute` 에
풀이 각각의 스트림에 대해 실행해야 할 클로저를 넘겨줍니다.
우린 이제  `pool.execute` 를 클로저를 받고 풀 안의 스레드에게 넘겨주어서 실행하도록 구현해야 합니다.
이 코드는 아직 컴파일 되지 않지만 컴파일러가 문제를 해결하는 방법을 안내 할 수 있도록 노력할 것입니다.

#### `ThreadPool` 구조체를 컴파일러 주도 개발을 이용해 제작

*src/main.rs* 를 Listing 20-12와 같이 변경하고, `cargo check` 로 얻은
컴파일러 에러를 이용해 개발을 진행해 봅시다.
여기 우리가 얻은 첫번째 에러가 있습니다.

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve. Use of undeclared type or module `ThreadPool`
  --> src\main.rs:10:16
   |
10 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ Use of undeclared type or module
   `ThreadPool`

error: aborting due to previous error
```

훌륭합니다. 이 에러는 우리가 `ThreadPool` 타입이나 모듈이 필요하다고 알려주고 있으니 지금 하나 만들어 봅시다.
우리가 만들 `ThreadPool` 은 우리의 웹 서버가 하는 일의 성향과는
독립되어 있어야 합니다.
그러니 `hello` 크레이트를 바이너리 크레이트에서
라이브러리 크레이트로 변경하여 `ThreadPool` 구현을 유지합시다.
라이브러리 크레이트로 변경한 뒤에는, 우린 분리된 스레드 풀 라이브러리를 웹 요청을 처리하는것 만이 아닌
우리가 스레드 풀을 사용하길 원하는 어떤 작업에서든 사용할 수 있습니다.

가장 간단한 `ThreadPool` 구조체 정의가 포함된
*src/lib.rs* 를 생성합니다.

<span class="filename">Filename: src/lib.rs</span>

```rust
pub struct ThreadPool;
```

그 후 *src/bin* 이라는 새 디렉토리를 생성하고
*src/main.rs* 바이너리 크레이트를 *src/bin/main.rs* 의 위치로 이동시킵니다.
이로써 *hello* 디렉토리 안의 라이브러리 크레이트가 주요 크레이트가 될 것입니다;
우린 여전히 *src/bin/main.rs* 바이너리 크레이트를 `cargo run` 명령어를 이용해 실행시킬 수 있습니다.
*main.rs* 파일을 이동시킨 후 라이브러리 크레이트를 가져와서
*src/bin/main.rs* 상단에 다음 코드를 추가하여 `ThreadPool` 을 스코프 내로 가져옵니다:

<span class="filename">파일명: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;
```

이 코드는 여전히 작동하지 않지만,
다음 오류를 확인하기 위해 다시 확인해 보겠습니다.

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0599]: no function or associated item named `new` found for type
`hello::ThreadPool` in the current scope
 --> src/bin/main.rs:13:16
   |
13 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^^^^^^ function or associated item not found in
   `hello::ThreadPool`
```

이 에러는 우리가 `ThreadPool` 의 `new` 함수를 생성해야 한다는 것을 나타냅니다.
우리는 `new` 가 `4` 를 인수로 받을 수 있도록 하나의 인자를 가져야 하고
`ThreadPool` 객체를 반환해야 한다는 것을 알고 있으니
해당하는 특성을 가진 가장 간단한 `new` 함수를
구현해 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

스레드의 개수가 음수라는 것은 말이 안되기 때문에
`size` 인자의 타입을 `usize` 로 정했습니다.
3장의 "정수 타입" 절에서 설명했듯이 이 4 라는 숫자를
`usize` 타입의 용도에 걸맞게 스레드 컬렉션 요소의 개수로 사용합니다.

코드를 다시한번 체크해 봅시다:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_size` instead

error[E0599]: no method named `execute` found for type `hello::ThreadPool` in the current scope
  --> src/bin/main.rs:18:14
   |
18 |         pool.execute(|| {
   |              ^^^^^^^
```

이제 경고와 에러가 발생합니다. 경고는 잠시 무시하고,
에러는 `ThreadPool` 에 `execute` 메소드가 없기 때문에 발생한 것을 볼 수 있습니다.
"유한 스레드 수를 위한 인터페이스 만들기" 절에서 우리가 만들 스레드 풀이
`thread::spawn` 과 비슷한 인터페이스를 가져야 한다고 결정했던걸 기억하세요.
또한 `execute` 함수를 구현하여 전달된 클로저를
풀의 유휴 스레드로 전달할 것입니다.

`ThreadPool` 에 `execute` 메소드를 매개변수로 클로저를 전달받도록 정의합시다.
13장의 "제네릭 파라미터와 `Fn` 트레잇을 사용하여 클로저 저장하기" 절에서
클로저를 매개변수로 받을때 `Fn` , `FnMut` , `FnOnce` 3가지의 트레잇이
있다고 했던걸 상기하세요.
우린 여기서 어떤 종류의 클로저를 사용할지 결정해야 합니다.
우린 표준 라이브러리인 `thread::spawn` 구현체와 비슷하게 만들것이기 때문에
`thread::spawn` 의 매개변수가 어떻게 되어 있는지 참고할 수 있습니다.
문서는 다음과 같은 내용입니다.

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`F` 타입 인자가 바로 우리가 찾는 녀석입니다.
`T` 타입 인자는 반환값과 연관된 인자니 관심을 가지지 않아도 됩니다.
우린 `spawn` 이 `F` 의 트레잇으로 `FnOnce` 를 사용하는 것을 알 수 있는데,
이게 바로 우리가 찾는 내용입니다.
왜냐하면 우린 결국 `spawn` 에 `execute` 인수를 전달해야하니까요.
또한 스레드가 요청을 처리할때 요청 클로저를 한번만 실행할 것이기 때문에
`Once` 에 매치되는 `FnOnce` 가 우리가 원하던 트레잇이라고 확신할 수 있습니다.

`F` 타입 인자는 `Send` 트레잇과 `'static` 생명주기가 바인딩되어 있습니다.
한 스레드에서 다른 스레드로 클로저를 전달해야하기에 `Send` 가 필요하고 스레드가
언제 파괴될지 모르기 때문에 `'static'` 이 필요합니다.
`execute` 메소드를 `ThreadPool` 에 생성하고 이들이 바인딩된 `F` 타입
제네릭 인자를 받도록 합시다.

<span class="filename">파일명: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    // --snip--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}
```

우린 클로저가 인자를 받지 않고 반환값도 없기 때문에
`FnOnce` 뒤에 `()` 를 사용합니다.
이처럼 함수의 반환값은 생략될 수 있습니다.
하지만 인자가 없더라도 괄호는 필요합니다.

이는 `execute` 메소드의 가장 간단한 구현입니다. 이 코드는 아무것도 하지 않지만,
우리 코드를 컴파일 시도해 볼 순 있습니다. 다시 한번 체크해봅시다.

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
warning: unused variable: `size`
 --> src/lib.rs:4:16
  |
4 |     pub fn new(size: usize) -> ThreadPool {
  |                ^^^^
  |
  = note: #[warn(unused_variables)] on by default
  = note: to avoid this warning, consider using `_size` instead

warning: unused variable: `f`
 --> src/lib.rs:8:30
  |
8 |     pub fn execute<F>(&self, f: F)
  |                              ^
  |
  = note: to avoid this warning, consider using `_f` instead
```

이제 경고만 받았으니, 컴파일에 성공했다는 뜻입니다!
하지만 여러분이 만약 `cargo run` 을 실행하고 브라우저로 요청을 보내보시면,
이 장의 초반에서 본 에러를 받게되실겁니다. 우리 라이브러리는 `execute` 로
전달된 클로저를 실행하지 않기 때문입니다.

> Note: 여러분이 만약 하스켈이나 러스트같이 엄격한 컴파일러를 사용하는 언어를 사용하신다면,
> "코드가 컴파일이 되면, 작동한단 뜻입니다." 라는 말이 통용됩니다. 하지만 이게 항상
> 적용되는게 아닌것이, 우리 프로젝트는 컴파일 되었지만 아무것도 하지 않습니다. 만약
> 우리가 실제 완성을 목표로 프로젝트를 제작중이었다면 이 상황은 코드가 컴파일되는지
> 체크하는것에 *더해서* 우리가 원하는 기능이 구현됐는지 확인하기 위해 유닛테스트를
> 진행하기 시작할 좋은 기회가 될 것입니다.

#### `new` 의 스레드 개수에 대한 유효성 검사

`new` 와 `execute` 의 파라미터로 아무것도 하지 않기 때문에 여전히 경고가 나타납니다.
이제 우리가 원하는 기능을 이 함수의 몸체부분에 구현해봅시다. 시작하기 전에,
`new` 에 대해서 생각해보죠. 이전에 우리가 스레드풀의 스레드 개수가 음수라는건
말이 안되기 때문에 `size` 인자를 양수형 타입으로 정한것을 기억하시나요?
어쨋든, 스레드가 하나도 없는것도 말이 안되는건 마찬가지입니다.
`usize` 타입엔 0이 들어갈 수 있으므로, 우린 예시 20-13 처럼
`ThreadPool` 인스턴스를 반환하기 이전에 `size` 가 0보다 큰지 검사하고,
0일 경우 `assert!` 매크로를 이용해
프로그램 패닉을 일으키는 코드를 추가할 것입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust
# pub struct ThreadPool;
impl ThreadPool {
    /// 새 스레드풀 생성
    ///
    /// size 는 풀 안의 스레드 개수입니다.
    ///
    /// # Panics
    ///
    /// `new` 함수는 size 가 0일때 패닉을 일으킵니다
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // --생략--
}
```

<span class="caption">Listing 20-13: `ThreadPool::new` 를 `size` 가 0일 경우 패닉을
일으키도록 구현</span>

`ThreadPool` 에 문서 주석(doc comments)을 좀 추가해 봤습니다.
14장에서 논의했듯이 우리 함수가 패닉을 일으킬 수 있는 상황을
설명하는 절을 추가함으로써 좋은 문서화방법을 따랐습니다.
`cargo doc --open` 을 입력하고 `ThreadPool` 구조체를 클릭한 뒤
`new` 에 대한 문서가 어떻게 만들어 졌는지 확인해보세요!

위에서 한 것처럼 `assert!` 매크로를 추가하는 대신에, `new` 를
예제 12-9 의 I/O 프로젝트의 `Config::new` 처럼 `Result` 를 반환하도록 바꿔봅시다.
하지만 이처럼 스레드풀을 스레드 없이 생성하려 하는것은 회복할 수 없는(unrecoverable)
에러가 되어야 합니다. 만약 여러분이 오기가 생기신다면,
`new` 를 다음과 같이 시그니처를 만든 새 버전을 작성해 보시고,
두 버전을 비교해보세요.

```rust,ignore
pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### 스레드를 보관하기 위한 공간 생성하기

이제 우린 스레드풀에 보관할 스레드의 개수가 유효하단 것을 확인했으니,
반환하기 전에 `ThreadPool` 구조체에 스레드들을 생성하고 보관해 놓을 수 있습니다.
그런데, 어떻게 스레드를 "보관" 할까요?
`thread::spawn` 의 시그니처를 다시 살펴봅시다.

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
```

`spawn` 함수는 `JoinHandle<T>` 를 반환합니다. 여기서 `T` 는 클로저가
반환할 타입입니다. `JoinHandle` 을 사용해보고 무슨 일이 일어나는지 살펴봅시다.
우리의 경우, 스레드풀로 전달된 클로저는 커넥션을 다루고
아무것도 반환하지 않을테니 `T` 는 `()` 가 되겠네요.

Listing 20-14의 코드는 컴파일엔 문제가 없지만 아직 아무 스레드도 만들지 않습니다.
`thread::JoinHandle<()>` 객체를 담는 벡터를 취급하도록
`ThreadPool` 의 정의를 변경했습니다.
벡터의 크기를 `size` 로 초기화하고 `for` 반목문에서 스레드들을 생성한뒤,
스레드들을 가진 `ThreadPool` 객체를 반환할 것입니다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --생략--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // 스레드들을 생성하고 벡터 내에 보관합니다
        }

        ThreadPool {
            threads
        }
    }

    // --생략--
}
```

<span class="caption">Listing 20-14: `ThreadPool` 에 스레드들을 보관하기 위한
벡터 만들기</span>

`std::thread` 를 라이브러리 크레이트의 스코프 내로 가져왔습니다.
우리가 `thread::JoinHandle` 를 `ThreadPool` 내 벡터 요소의 타입으로
사용하고 있으니까요.

`ThreadPool` 은 유효한 숫자를 전달받을 경우 `size` 크기대로 새로운 벡터를 생성합니다.
이 책에선 아직 `Vec::new` 와 비슷한 기능을 하는 `with_capacity` 함수를 사용하진
않았습니다:
다만 이 두 함수는 중요한 차이가 있는데, `with_capacity` 함수는 벡터의
공간을 미리 할당합니다. 우린 벡터 안에 들어갈 요소의 개수를 알고 있기 때문에 사전에
공간을 할당 함으로써 요소의 삽입마다 재할당이 일어나는 `Vec::new` 를
사용할 때 보다 효율을 높일 수 있습니다.

`cargo check` 를 재실행 하시면 몇개의 경고는 발생하겠지만
문제 없이 성공하실겁니다.

#### `ThreadPool` 에서 스레드로 코드를 보내는 `Worker` 구조체

Listing 20-14 의 `for` 반복문에 스레드 생성에 관해 주석을 남겨놨습니다.
여기서 우리가 스레드들을 실제로 만드는 방법을 알아볼 예정입니다.
표준 라이브러리는 `thread:::spawn` 를 이용해 스레드를 생성할 수 있도록 제공하며,
`thread::spawn` 은 스레드가 생성되는 즉시 스레드가 실행할 코드를 전달 받도록
되어있습니다. 하지만 우린 스레드를 생성하고 나중에 코드를 전달받을 때까지
`기다리도록` 해야 합니다.
안타깝게도 표준 라이브러리의 스레드 구현에는 이러한 방법을 지원하지 않아서
우리가 직접 구현해야합니다.

우린 이 기능을 `ThreadPool` 과 스레드들 사이에 새로운 데이터 구조를 도입하여 구현할 것입니다.
앞으로 이 데이터 구조를 `Worker` 라고 부르겠습니다.
이 용어는 풀링 구현에서 흔하게 사용됩니다.
한번 식당의 부엌에서 일하는 사람들을 예로 들어보죠.
이 `Worker` 들은 고객으로부터 주문을 받을 때까지 기다린 다음,
주문을 받고 일합니다. 뭐 대충 비슷하지 않나요?

스레드 풀 안에 `JoinHandle<()>` 인스턴스 벡터 대신,
`Worker` 구조체의 인스턴스들을 내장하도록 해 봅시다.
이때 각각의 `Worker` 는 단일 `JoinHandle<()>` 인스턴스를 내장하게 됩니다.
그리고 실행할 코드의 클로저를 전달받고 스레드에게 전달해 실행하도록 하는 함수를
`Worker` 에 구현할 것입니다. 또한 우린 각각의 워커에 `id` 를 부여해
로그를 남기거나 디버깅을 할때 서로 다른 워커들을 구별할 수 있게 할 것입니다.

`ThreadPool` 을 생성할때 일어나는 일을 다음과 같이 변경해 보겠습니다.
다음과 같은 방법으로 `Worker` 를 설정하고 스레드에 클로저를 전송하는 코드를
구현합니다.

1. `id` 와 `JoinHandle<()>` 를 갖는 `Worker` 구조체를 정의 합니다.
2. `ThreadPool` 을 `Worker` 인스턴스들의 벡터를 갖도록 변경합니다.
3. `id` 숫자를 받고 전달받은 `Worker` 인스턴스를 반환하는 `Worker::new` 함수를
   정의합니다. 반환된 `Worker` 인스턴스에는 `id` 와 빈 클로저로 생성된 스레드가
   포함되어 있습니다.
4. `ThreadPool::new` 안에서, `for` 루프 카운터를 이용해 `id` 를 생성하고 생성된 `id`
   를 이용해 새 `Worker` 를 생성한 뒤 해당 워커를 벡터안에 저장합니다.

도전해보실 분은 Listing 20-15 코드를 보기 전에
직접 구현해보시길 바랍니다.

준비 되셨나요? 여기 앞선 수정사항들을 구현한 방법중 하나로 Listing 20-15 를 가져와 보았습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --생략--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers
        }
    }
    // --생략--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-15: `ThreadPool` 을 스레드들을 직접 내장하는 대신
`Worker` 인스턴스들을 내장하게 변경</span>

이제 `JoinHandle<()>` 인스턴스들이 아닌 `Worker` 인스턴스들을 내장하기 때문에
`ThreadPool` 의 필드 이름을 `threads` 에서 `workers` 로 변경했습니다.
우린 `for` 반복문으로 `Worker::new` 에 전달된 인자만큼 카운트하고
각각의 새 `Worker` 를 `workers` 벡터에 저장합니다.

외부 코드 (우리 서버의 *src/bin/main.rs* 같은) 에선 `ThreadPool` 내 `Worker` 구조체의 상세한 구현을 알 필요가 없기에
`Worker` 구조체와 `new` 함수를 `private` 로 만듭니다.
`Worker::new` 함수는 우리가 넘겨준 `id` 를 사용하고
새로 생성된 `JoinHandle<()>` 객체를 저장합니다.
이때 `JoinHandle<()>` 객체를 생성한 주체는 빈 클로저를 이용해 생성된 새 스레드입니다.

이 코드는 컴파일 되고 `Thread::new` 의 인자로서 지정된 `Worker` 인스턴스의 개수를 저장합니다.
하지만 우린 *여전히* `execute` 에서 전달받은 클로저를 처리하지 않고 있습니다.
다음에 그 작업을 수행하는 방법을 살펴 보겠습니다.

#### 채널을 통해 스레드에 요청 보내기

이제 우린 `thread::spawn` 에 주어진 클로저가 아무것도 하지 않는다는 문제점을 해결할 겁니다.
현재 우린 실행하고 싶은 클로저를 `execute` 메소드로 받고 있습니다.
하지만 우리가 `thread::spawn` 에 전달할 클로저는 `ThreadPool` 의 생성중
각각의 `Worker` 가 생성될때 실행할 클로저여야 합니다.

우리는 방금 생성 한 `Worker` 구조체가 `ThreadPool` 에 들어있는 큐에서 실행될
코드를 가져오고 그 코드를 스레드로 보내 실행하도록 하고자 합니다.

16장에서, 간단히 두 스레드간에 통신하는 방법인 *channels* 에 대해 배웠습니다.
이거 지금 상황에 딱이네요.
우린 채널을 작업 대기열로 사용하고 `execute` 는 `ThreadPool` 에서 `Worker` 인스턴스로 작업을 보냅니다.
그러면 작업이 스레드로 전송되겠죠.
계획은 다음과 같습니다:

1. `ThreadPool` 은 채널을 생성하고
   채널의 송신단을 유지합니다.
2. 각 `Worker` 는 채널의 수신단을 유지합니다.
3. 우린 채널로 전송하려는 클로저를 저장할
   새로운 `Job` 구조체를 생성할 겁니다.
4. `execute` 메소드는 채널의 송신단으로 실행하려는
   작업을 전송합니다.
5. 스레드에선 `Worker` 가 채널의 수신단에서 반복되며
   수신되는 모든 작업의 클로저를 실행합니다.

`ThreadPool::new` 안에 채널을 생성하고 `ThreadPool` 객체에 송신단을
내장하도록 하는것부터 시작해봅시다.
Listing 20-16에 보이는 것처럼 `Job` 구조체는 아직은 아무것도 들어있지 않지만,
우리가 채널에 보낼 요소의 타입이 될 것입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust
# use std::thread;
// --생략--
use std::sync::mpsc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    // --생략--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // --생략--
}
#
# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
# impl Worker {
#     fn new(id: usize) -> Worker {
#         let thread = thread::spawn(|| {});
#
#         Worker {
#             id,
#             thread,
#         }
#     }
# }
```

<span class="caption">Listing 20-16: `ThreadPool` 이 `Job` 객제를 전송하는
채널의 송신 측을 저장하도록 변경</span>

`ThreadPool::new` 에서 우린 새 채널을 만들고 풀에 송신단을 저장합니다.
이 소스는 경고 몇개와 함께 성공적으로 컴파일됩니다.

한번 스레드풀이 생성한 각각의 *worker* 에 채널의 수신단을 넘겨봅시다.
우린 *worker* 가 생성한 스레드에서 수신단을 사용할 수 있게 만들기 위해,
클로저의 `receiver` 매개변수를 참조 할 것입니다.
Listing 20-17의 코드는 아직 컴파일 되지 않습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
impl ThreadPool {
    // --생략--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
    // --생략--
}

// --생략--

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-17: *worker* 에
채널의 송신단을 전달</span>

작고 간단한 변경사항을 만들었습니다: `Worker::new` 에 채널의 수신단을
전달하고, 클로저 안에서 사용합니다.

이 코드를 `check` 해보면, 다음과 같은 에러가 나타날 겁니다:

```text
$ cargo check
   Compiling hello v0.1.0 (file:///projects/hello)
error[E0382]: use of moved value: `receiver`
  --> src/lib.rs:27:42
   |
27 |             workers.push(Worker::new(id, receiver));
   |                                          ^^^^^^^^ value moved here in
   previous iteration of loop
   |
   = note: move occurs because `receiver` has type
   `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
```

이 코드는 `receiver` 을 여러개의 `Worker` 객체에 넘기는 시도를 하는데, 이는 작동하지 않습니다.
16장에서 배운걸 떠올려보세요: 러스트가 제공하는 채널 구현은
여러 *producer* , 하나의 *consumer* 를 제공합니다.
즉 이 코드를 수정하기 위해 채널의 소비측 끝을 복제할 수는 없습니다.
만약 가능하더라도 우리가 원하는 기법은 아닙니다;
우린 대신에 하나의 `receiver` 을 모든 `worker` 들이 공유하도록
만들어 스레드간 작업을 분산하고자 합니다.

또한, 채널 큐에서 작업을 가져오는 작업은 `receiver` 을 이용하는데, 이 과정에서 `receiver` 이 변화할 수도 있습니다.
따라서 스레드들은 `receiver` 을 공유하고 수정하기 위한 안전한 방법이 필요합니다.
그렇지 않다면, 경쟁 조건 (관련 내용은 16장에서 다뤘습니다) 이 발생하게 될 것 입니다.

16장에서 설명한 스레드-안전 스마트 포인터를 생각해 보세요:
이는 여러 스레드 간에 소유권을 공유하고 스레드가 값을 변경하도록 허용합니다.
우린 `Arc<Mutex<T>>` 를 사용해야 하는데, 이 `Arc` 타입은 여러 *worker* 들이 *receiver* 를 소유하는 걸 허용해줍니다.
그리고 `Mutex` 는 한번에 하나의 *worker* 만이 *receiver* 로부터 작업을 가져가도록 보장합니다.
Listing 20-18 은 위 내용대로 수정한 모습입니다.

<span class="filename">파일명: src/lib.rs</span>

```rust
# use std::thread;
# use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
// --생략--

# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# struct Job;
#
impl ThreadPool {
    // --생략--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // --생략--
}

# struct Worker {
#     id: usize,
#     thread: thread::JoinHandle<()>,
# }
#
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --생략--
#         let thread = thread::spawn(|| {
#            receiver;
#         });
#
#         Worker {
#             id,
#             thread,
#         }
    }
}
```

<span class="caption">Listing 20-18: `Arc` 와 `Mutex` 를 이용해 채널의
를 *worker* 들이 공유하도록 변경</span>

`ThreadPool::new` 에서, `Arc` 와 `Mutex` 를 이용해 채널의 수신단을 감싸고,
새로운 *worker* 각각에 `Arc` 를 복제해 참조 카운트를 늘려서 *worker* 들이
소유권을 공유할 수 있도록 합니다.

이렇게 변경하고 나면 컴파일이 될 겁니다! 우리가 해냈내요!

#### `execute` 메소드 구현

이제 마지막으로 `ThreadPool` 의 `execute` 메소드를 구현해 봅시다.
우린 `execute` 가 받는 클로저 타입을 포함할 트레잇 오브젝트를 위해 `Job` 을 구조체에서 타입 별칭으로 변경할 겁니다.
19장의 “Type Aliases Create Type Synonyms” 부문에서 이야기한 대로,
타입 별칭은 긴 이름을 가진 타입을 짧게 만들 수 있게 해줍니다.
Listing 20-19에서 확인해 보세요.

<span class="filename">파일명: src/lib.rs</span>

```rust
// --생략--
# pub struct ThreadPool {
#     workers: Vec<Worker>,
#     sender: mpsc::Sender<Job>,
# }
# use std::sync::mpsc;
# struct Worker {}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
    // --생략--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

// --생략--
```

<span class="caption">예제 20-19: 각 클로저를 내장한 `Box` 에 대한 타입 별칭인
`Job` 을 만들고 채널에 작업 전송하기</span>

`execute` 로 얻은 클로저를 사용하여 새 `Job` 객체를 생성하고 나면,
생성된 작업을 채널의 송신 측으로 보냅니다. `send` 가 실패할 경우엔 `unwrap` 을 호출합니다.
예를 들어 실행중인 모든 스레드가 중지되면 수신단이
새로운 메시지를 수신하지 못하게 될 수 있습니다.
현재, 우린 실행중인 스레드들을 멈출 수 없습니다:
스레드들은 풀이 존재하는 한 계속 실행 될 겁니다.
`unwrap` 을 사용하는 이유는 실패 사례가 발생하지 않을 것이란걸
우린 알고 있지만 컴파일러는 모르기 때문입니다.

아직 끝나지 않았습니다! *worker* 에서,
우리 클로저는 `thread::spawn` 에 전달되어 여전히 채널의 수신단만 *참조* 합니다.
대신에 클로저가 계속 반복되며 채널의 수신단에 작업을 요청하고
받은 작업을 실행해야 합니다.
Listin 20-20에서 `Worker::new` 를 봅시다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
// --생략--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                (*job)();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-20: *worker* 스레드에서 작업을 수신하고
실행하기</span>

맨 처음, 뮤텍스를 얻기 위해 `receiver` 의 `lock` 을 호출 하였고,
그 뒤 `unwrap` 을 이용해 어떤 에러든 패닉을 일으키도록 하였습니다.
만약 어떤 스레드에서 잠금을 걸고 나서 해제하기 전에 패닉 상태가 되어 뮤텍스가
*poisoned* 상태가 되었을 경우 뮤텍스를 얻는데 실패할 수 있기 때문에,
이 경우 `unwrap` 을 호출해 스레드 패닉을 발생시키는 것이 취해야 할 올바른 행동 입니다.
원하실 경우 `unwrap` 을 여러분에게 의미있는 에러 메세지와 함께
`expect` 로 바꾸어 보세요.

만약 우리가 뮤텍스의 잠금을 얻게 된다면,
채널로부터 `Job` 을 얻기 위해 `recv` 를 호출합니다.
마지막 `unwrap` 은 송신단을 유지하는 스레드가 종료 되었을 경우 발생할 수 있는 에러를 지나쳐서 이동합니다.
`send` 메소드가 수신단이 종료되면 `Err` 을 리턴하는 것과 비슷합니다.

아직 아무 작업도 없어서 `recv` 에 대한 호출이 막힌다면,
현재 스레드는 작업이 가능해질 때까지 대기합니다. `Mutex<T>` 가 한번에
오직 하나의 `Worker` 스레드가 작업을 요청할 수 있도록 보장합니다.

이론적으론 이 코드는 컴파일 되어야 합니다.
하지만 불행하게도 러스트 컴파일러는 아직 완벽하지 않습니다. 나타나는 에러는 다음과 같습니다:

```text
error[E0161]: cannot move a value of type std::ops::FnOnce() +
std::marker::Send: the size of std::ops::FnOnce() + std::marker::Send cannot be
statically determined
  --> src/lib.rs:63:17
   |
63 |                 (*job)();
   |                 ^^^^^^
```

문제가 상당히 난해하기 때문에 이 오류는 상당히 수수께끼스럽습니다. `Box<T>` (`Job` 가 가르키는 그것)
안에 저장되어 있는 `FnOnce` 클로저를 호출하기 위해선,
클로저는 `Box<T>` 에서 스스로 벗어나야 합니다.
우리가 호출할때 클로저는 `self` 의 소유권을 가지기 때문이죠.
보편적으로, 러스트는 `Box<T>` 의 값을 옯기는 것을 허용하지 않습니다.
러스트에서 `Box<T>` 안에 얼마나 큰 값이 들어갈지 알 수 없기 때문입니다:
15장에서 우리가 박스에 저장하고자 하는 알수없는 크기의 무언가를
알고있는 크기의 값으로 얻어내기 위해 `Box<T>` 를 사용했던 걸 떠올려 보세요.

여러분이 Listing 17-15에서 보신 것처럼 우린 `self: Box<Self>`
구문을 이용하는 메소드를 작성할 수 있습니다.
`Box<T>` 에 저장된 `Self` 값의 소유권을 다룰 수 있도록 허용된 메소드 말이죠.
우리가 지금 하고 싶은 것 그 자체네요.
그런데 불행히도 러스트가 우릴 놓아주지 않네요: 러스트의 구현체중 클로저가
호출될 때의 구현체 부분은 `self: Box<Self>` 방식을 사용하지 않았습니다.
따라서 러스트는 이 상황에서 `self: Box<Self>` 를 사용할 수 있다는 것을 아직 이해하지 못합니다.

러스트의 컴파일러는 여전히 개선중입니다.
따라서 언젠가 Listing 20-20 의 코드는 정상적으로 작동할 거예요.
여러분 같은 사람들이 이런 문제를 해결하기 위해 노력중입니다!
여러분이 이 책을 끝내고 나서 참여하신다면 우린 환영할 겁니다.

하지만 지금 당장은, 편리한 트릭을 이용해 이 문제를 해결하도록 하겠습니다.
우린 러스트에게 명시적으로 이러한 경우에 우린 `self: Box<Self>` 를 이용해
`Box<T>` 내부의 값에 대한 소유권을 가질 수 있다고 말할 수 있습니다;
클로저에 대한 소유권을 가진 뒤에는 호출할 수 있습니다.
이는 `call_box` 메소드로 새로운 트레잇인 `FnBox` 를 정의하는 것입니다.
`call_box` 는 시그니처로 `self: Box<Self>` 를 사용하고 `FnOnce()` 를 구현하는 모든 타입에 `FnBox` 를 정의하고
타입 별명을 새 트레잇으로 변경하고 `Worker`를 `call_box` 메소드를 사용하도록 변경할 것 입니다.
이 내용들을 Listing 20-21 에서 보실 수 있습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-21: `Box<FnOnce()>` 의 한계를 해결하기 위한
새 트레잇 `FnBox` 추가</span>

먼저, `FnBox` 라는 이름의 새 트레잇을 생성합니다.
이 트레잇은 `call_box` 메소드를 하나 가집니다.
이 메소드는 `self` 의 소유권을 다루고 `Box<T>` 에서 값을 제외하기 위해
`self: Box<Self>` 를 다룬다는 점 외에는 다른 `Fn*` 트레잇들의 `call` 메소드와 흡사합니다.

다음으로, `FnOnce()` 트레잇을 구현하는 `F` 타입에 대한 `FnBox` 트레잇을
구현합니다. 효과적으로, 이는 `FnOnce()` 클로저가 우리의 `call_box` 메소드를
사용할 수 있다는 뜻입니다. `call_box` 의 구현은 `(*self)()` 를 사용하여
클로저를 `Box<T>` 밖으로 빼내고 호출합니다.

우리는 이제 `Job` 타입 별명이 새로운 트레잇 인 `FnBox` 를 구현하는 `Box` 가 될
필요가 있습니다. 이는 클로저를 직접 호출하는 대신 `Job` 값을 얻을 때 `Worker`
에서 `call_box` 를 사용할 수 있게 해줍니다. 어떤 `FnOnce()` 클로저에 대한
`FnBox` 트레잇을 구현한다는 것은 채널을 보내고있는 실제 값에 대해서는 아무것도
변경할 필요가 없다는 것을 의미합니다. 이제 러스트는 우리가 하고자 하는 일이
문제 없단걸 정상적으로 인식할 수 있습니다.

이 트릭은 매우 복잡하고 교활합니다. 정확히 이해가 되지 않아도 걱정하지 마세요;
언젠가 완전히 필요 없어질 겁니다.

이 트릭을 구현함으로써, 우리의 스레드 풀은 작동합니다.
`cargo run` 을 실행하고, 몇가지 요청을 해보세요.

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never used: `workers`
 --> src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: field is never used: `id`
  --> src/lib.rs:61:5
   |
61 |     id: usize,
   |     ^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

warning: field is never used: `thread`
  --> src/lib.rs:62:5
   |
62 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.99 secs
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

성공했습니다! 우린 이제 비동기적으로 커넥션을 실행하는 스레드 풀을 가지게
되었습니다. 스레드는 4개 이상 만들어지지 않을 겁니다. 따라서 우리 시스템은
서버가 수많은 요청을 받더라도 과부하 될 일이 없습니다.
만약 우리가 `/sleep` 요청을 하더라도,
다른 스레드가 작동함으로써 다른 요청에 문제 없이 작동합니다.

18장에서 `while let` 반복문을 배우셨다면, 아마 제가 왜 Listing 20-22와 같이
*worker* 스레드의 코드를 작성하지 않았는지 궁금해 하실 겁니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
// --생략--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}
```

<span class="caption">Listing 20-22: `while let` 을 이용한
`Worker::new` 의 대안 구현체</span>

이 코드는 컴파일 될 것이나 원하던 스레딩 동작은 하지 않습니다.
느린 요청은 여전히 다른 요청들이 처리되길 기다립니다.
이유는 다소 미묘합니다:
`Mutex` 구조체는 공개된(public) `unlock` 메소드를 가지고 있지 않습니다.
lock 의 소유권은 `LockResult<MutexGuard<T>>` 에 있는 `MutexGuard<T>` 의
라이프타임에 기반을 두고 있기 때문입니다.
컴파일시, 빌림 검사기 (borrow checker) 는 잠금을 유지하지 않으면 `Mutex` 에
의해 보호받는 리소스에 접근할 수 없다는 규칙을 적용할 수 있습니다.
그러나 이러한 구현은 `MutexGuard<T>` 의 라이프타임을 주의 깊게 생각하지 않았을
경우 잠금이 의도보다 오래 지속되는 결과를 초래할 수 있습니다.
`while` 식 안의 값은 블록의 지속 시간동안 남아있기 때문에,
잠금은 `job.call_box()` 호출 기간동안 유지되어 다른 *worker* 들이 작업을 수신할 수 없음을 뜻합니다.

대신에 `loop` 를 사용하고 잠금과 작업을 얻음으로써
외부가 아닌 블록 내에서 얻음으로써,
`lock` 메소드에서 반환 된 `MutexGuard` 는 `let job` 문이 끝나자 마자 사라지게됩니다.
이렇게하면 잠금이 `recv`에 대한 호출 중에 해제되지만,
`job.call_box()` 호출 이전에 해제되어 여러 요청을 동시에 처리 할 수 있습니다.
