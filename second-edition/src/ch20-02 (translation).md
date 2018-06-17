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
예제 20-10은 `/sleep` 요청을 처리할때 응답하기 전에
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

<span class="caption">예제 20-10: `/sleep` 요청을 인식할시 5초간
멈춤으로써 느린 요청을 시뮬레이션 하기</span>

이 코드는 좀 지저분하지만 시뮬레이션 용도로는 충분합니다.
우리는 우리 서버가 인식할 두번째 요청인 `sleep` 을 생성하고.
`/sleep` 으로의 요청을 처리할 `else if` 를
`if` 블록 뒤에 추가했습니다. 만약 요청이 들어오면,
서버는 HTML 페이지를 렌더링 하기 전에 5초간 대기할 것입니다.

여러분은 우리 서버가 얼마나 부족한지 알 수 있습니다:
실제 라이브러리들은 훨씬 간단한 방법으로 여러개의 요청을 구분할 것입니다

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
스레드 풀은 작업을 pool 안에 있는 스레드중 하나에게 맡기고
해당 스레드가 작업을 처리하도록 합니다. 남은 스레드들은
첫번째 스레드가 처리중인 동안 들어온 작업을 언제든지 처리할 수
있도록 합니다. 첫번째 스레드가 작업을 끝마치면 pool로 돌아와
작업 대기상태가 됩니다. 스레드 풀은 우리가 여러 커넥션들을
동시에 처리할 수 있게 해주고 우리 서버의 처리량을 증가시킵니다.

우린 Dos (Denial of Service) 공격을 막기 위해 pool 안의
스레드 개수에 대한 제한을 작게 둘 것입니다;
만약 우리 프로그램이 각각의 요청이 들어올때마다 새 스레드를 생성한다면
누군가 우리 서버에 10만개의 요청을 보냈을때 우리 서버는
서버의 모든 리소스를 사용하고 모든 요청이 끝날때까지 처리가 계속될 것입니다.

우린 스레드를 제한없이 생성하는것이 아닌 pool 안에서 대기할
고정된 개수의 스레드를 가질 것입니다. 요청이 들어온다면,
요청들은 처리를 위해 pool로 보내지고, pool에선 들어오는 요청들에
대한 queue 를 유지할 것입니다. pool 내의 각 스레드들은 이 queue에서
요청을 꺼내서 처리하고 또 다른 요청이 있는지 queue에 물어봅니다.
우린 이 형태를 이용해 동시에 `N` 개의 요청을 처리할 수 있습니다. 여기서 `N` 은 스레드의 개수입니다.
만약 각각의 스레드가 응답하는데 오래 걸리는 요청을 처리하게되면
그 다음의 요청들은 여전히 queue에 남아있게 됩니다만,
이전보다 처리할 수 있는 요청은 늘어났습니다

이 기술은 우리 웹서버의 처리량을 증가시킬 수많은 방법중 하나일 뿐입니다.
여러분이 찾으실 다른 방법들은 fork/jon 모델과 싱글 스레드 기반
비동기 I/O 모델 등일 것입니다. 만약 여러분이 이러한 내용에 관심이
있으시다면, 다른 해결책들에 대해 좀 더 자세히 찾아보시고 Rust로 구현해 보세요;
Rust같은 저레벨 언어로는 이와 같은 방법들이 전부 가능합니다.

스레드 풀을 구현하기 전에, pool 이 어떻게 쓰여야 할지 이야기 해 봅시다.
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
예제 20-11는 `main` 함수의 `for` 반복문을 모든 요청에 대해 새 스레드를 생성하도록 변경한 모습을 보여줍니다.

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

<span class="caption">예제 20-11: 매 요청마다
새 스레드 생성</span>

여러분이 16장에서 배우신대로, `thread::spawn` 은 새 스레드를 생성하고,
내부에 있는 클로저의 코드를 실행합니다.
만약 여러분이 이 코드를 실행하고 브라우저로 `/sleep` 으로 접속하신 후,
둘 이상의 브라우저 탭으로 `/` 에 접속하신다면, `/` 로의 요청이
`/sleep` 이 끝나길 기다리지 않고 완료 되는 것을 보실 수 있을 것입니다.
하지만 말했듯이, 스레드를 무한정 생성하는 것은 결국 시스템을 과부화시킬 것입니다.

#### 유한 스레드 수를 위한 인터페이스 만들기

우린 스레드 풀을 비슷하고 익숙하게 작동하도록 만들어서
스레드 풀 방식으로 변경할때 우리 API를 사용하는
코드를 크게 변경할 필요가 없도록 하고자 합니다.
예제 20-12는 `thread::spawn` 대신 이용하고자 하는 `ThreadPool` 이라는 가상의 인터페이스를 보여줍니다.

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

<span class="caption">예제 20-12: 우리의 이상적인 `ThreadPool` 인터페이스</span>

우린 새로운 스레드 풀을 만들때 `ThreadPool::new` 를
설정할 스레드의 개수를 나타내는 수(이 경우는 4)와 함께 사용했습니다.
그 후 `for` 반복문에선 `thread::spawn` 과 비슷한 인터페이스를 가진 `pool.execute` 에
pool이 각각의 스트림에 대해 실행해야 할 클로저를 넘겨줍니다.
우린 이제  `pool.execute` 를 클로저를 받고 pool 안의 스레드에게 넘겨주어서 실행하도록 구현해야 합니다.
이 코드는 아직 컴파일 되지 않지만 컴파일러가 문제를 해결하는 방법을 안내 할 수 있도록 노력할 것입니다.

#### `ThreadPool` 구조체를 컴파일러 주도 개발을 이용해 제작

*src/main.rs* 를 예제 20-12와 같이 변경하고, `cargo check` 로 얻은
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
pool의 유휴 스레드로 전달할 것입니다.

`ThreadPool` 에 `execute` 메소드를 매개변수로 클로저를 전달받도록 정의합시다.
13장의 "제네릭 파라미터와 `Fn` 트레잇을 사용하여 클로저 저장하기" 절에서
클로저를 매개변수로 받을때 `Fn` , `FnMut` , `FnOnce` 3가지의 특성이
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
우린 `spawn` 이 `F` 의 특성으로 `FnOnce` 를 사용하는 것을 알 수 있는데,
이게 바로 우리가 찾는 내용입니다.
왜냐하면 우린 결국 `spawn` 에 `execute` 인수를 전달해야하니까요.
또한 스레드가 요청을 처리할때 요청 클로저를 한번만 실행할 것이기 때문에
`Once` 에 매치되는 `FnOnce` 가 우리가 원하던 트레잇이라고 확신할 수 있습니다.

`F` 타입 인자는 `Send` 트레잇 또한 가지고 있으며 우리의 상황에서 유용한 `'static` 생명주기가 바인딩 되어 있습니다. 