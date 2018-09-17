## 우아한 종료와 정리

Listing 20-21 의 코드는 우리가 의도한대로 스레드 풀을 이용해
비동기적으로 요청에 응답합니다.
다만 우린 `workers`, `id`, `thread` 필드를 직접적으로 사용하지 않고 있다는 경고를
받는데, 이는 우리가 아무것도 정리하질 않았다는 것을 상기시킵니다. 예를 들어
우리가 <span class="keystroke">ctrl-c</span> 처럼 우아하지 않은 방식으로
메인 스레드를 정지 시킬 경우 모든 스레드는 즉시 정지됩니다.
만약 그 스레드가 요청을 처리하는 도중 이더라도요.

이제 우린 풀 안의 각 스레드 상에서 join 을 호출하여 스레드가 종료되기 전에
그들이 처리하던 요청을 마저 처리할 수 있도록 하기 위하여 Drop 트레잇을 구현할 겁니다. 그런 다음
스레드들에게 더 이상 새로운 요청을 받지 말고 종료하라고 알려주는 방법을 구현할
것입니다. 이 코드가 작동하는지 확인하기 위해, 정상적으로 스레드 풀을 종료하기 전에
오직 두개의 요청만 수락하도록 우리 서버를 수정합시다.

### `ThreadPool` 에 `Drop` 트레잇 구현하기

우리가 만든 스레드 풀에 `Drop` 을 구현하는 것 부터 시작해봅시다.
풀이 드롭(dropped) 되었을 때, 스레드들은 모두 `join` 되어 자신의 작업을 마쳐야 합니다.
Listing 20-23 은 `Drop` 을 구현한 첫 시도의 모습입니다;
이 코드는 아직 작동하지 않습니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

<span class="caption">Listing 20-23: 스레드 풀이 스코프를 벗어날때
각 스레드 종료</span>

먼저, 스레드 풀의 `workers` 각 요소에 대한 반복문을 정의합니다.
`self` 가 가변 참조자이고, 우리가 `worker` 를 변경할 수 있도록 해야 하므로 `&mut`
를 사용했습니다. 각각의 worker에 대해서는 이 worker가 종료된다는 메시지를 출력하고
해당 worker의 스레드에 `join` 을 호출합니다.
만약 `join` 을 호출하는데 실패하면, `unwrap` 을 이용해 Rust 패닉을 일으키고
강제 종료합니다.

이 코드를 컴파일 했을 때 나오는 에러는 다음과 같습니다

```text
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:65:13
   |
65 |             worker.thread.join().unwrap();
   |             ^^^^^^ cannot move out of borrowed content
```

이 에러는 우리가 `worker` 의 가변 형태로 빌리기만 했기 때문에 인수의 소유권을
필요로 하는 `join` 을 호출할 수 없다는 걸 알려줍니다. 이 이슈를 해결하기 위해,
`join` 이 스레드를 사용할 수 있도록 `thread` 의 소유권을 `Worker` 인스턴스로부터
빼내야 합니다. 이전에 Listing 17-15 에서 한번 해봤었죠: `Worker` 가
`Option<thread::>JoinHandle<()>` 를 대신 갖도록 하면, `Option` 의 `take`
메소드를 사용하여 `Some` variant에서 값을 빼내고 `None` 으로 대체할 수 있습니다.
즉, 실행중인 `Worker` 는 `thread` 에 `Some` variant 를 갖게 되고,
우린 worker 를 종료하고자 할때 `Some` 을 `None` 으로 대체하여 worker 가 실행할
스레드를 없앨 수 있습니다.

그러니 `Worker` 의 정의를 다음과 같이 변경합시다:

<span class="filename">파일명: src/lib.rs</span>

```rust
# use std::thread;
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

변경해야 하는 나머지 부분은 컴파일러에 의지해서 찾아보도록 합시다.
코드를 `check` 해보니 두 에러가 나오네요:

```text
error[E0599]: no method named `join` found for type
`std::option::Option<std::thread::JoinHandle<()>>` in the current scope
  --> src/lib.rs:65:27
   |
65 |             worker.thread.join().unwrap();
   |                           ^^^^

error[E0308]: mismatched types
  --> src/lib.rs:89:13
   |
89 |             thread,
   |             ^^^^^^
   |             |
   |             expected enum `std::option::Option`, found struct
   `std::thread::JoinHandle`
   |             help: try using a variant of the expected type: `Some(thread)`
   |
   = note: expected type `std::option::Option<std::thread::JoinHandle<()>>`
              found type `std::thread::JoinHandle<_>`
```

`Worker::new` 의 끝에 위치한 두번째 에러부터 해결해 봅시다;
`Worker` 를 생성할 때 `thread` 를 `Some` 으로 감싸줘야 한다네요.
다음과 같이 변경해줍시다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --생략--

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

첫번째 에러는 우리의 `Drop` 구현에서 발생했네요.
이전에 우리가 `worker` 로 부터 `thread` 를 빼내기 위해선 `Option` 에서
`take` 를 호출해야 한다고 언급했습니다. 이는 다음과 같이 변경해줍시다:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

17장에서 의논한 대로, `Option` 의 `take` 메소드는 `Some` variant 를 빼내고 `None`
으로 대체합니다. `Some` 을 파괴하고 스레드를 얻기 위해 `if let` 를 사용합니다;
그리고 나서 스레드의 `join` 을 호출 합니다. 만약 이때 worker 의 스레드가 이미
`None` 일 경우, worker 가 자신의 스레드를 이미 정리했다는 뜻이므로 아무 일도 하지
않습니다.

### 스레드가 작업 리스닝을 중지하도록 신호하기

모두 수정하고 나면 경고 없이 컴파일이 잘 될 겁니다. 하지만 안좋은 소식이 있는데,
이 코드는 아직 우리가 원하는 대로 작동하지 않는다는 겁니다.
이에 대한 핵심은 `Worker` 인스턴스의 스레드에 의해 실행되는 클로저에 있습니다:
우리가 `join` 을 호출해도 스레드는 영원히 새 작업을 찾는 일을 반복할 것이기에
스레드는 종료되지 않습니다. 만약 우리가 현재 `drop` 의 구현대로 `ThreadPool` 을
drop 한다면, 메인스레드는 첫번째 스레드가 끝나기만을 기다리는 상태로 영원히
멈춰있을 겁니다.

이를 해결하기 위해, 실행할 `Job` 이나 리스닝을 멈추고
무한 반복문을 탈출하라는 신호를 기다리도록 스레드를 수정해야 합니다.
우리 채널은 `Job` 인스턴스 대신에 두 variant 를 가진 열거형을
전달할 겁니다.

<span class="filename">파일명: src/lib.rs</span>

```rust
# struct Job;
enum Message {
    NewJob(Job),
    Terminate,
}
```

이 `Message` 열거형은 스레드가 실행해야할
`Job` 을 담고있는 `NewJob` variant 가 되거나,
혹은 스레드를 중지시킬 `Terminate` variant 가 될 겁니다.

우린 Listing 20-24 처럼 `Job` 대신 `Message` 타입을 이용하도록
채널을 조정해야합니다.

<span class="filename">파일명: src/lib.rs</span>

```rust,ignore
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// --생략--

impl ThreadPool {
    // --생략--

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// --생략--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

<span class="caption">Listing 20-24: `Message` 값을 전달하고 받으며
`Worker` 가 `Message::Terminate` 를 받을 경우 반복문 탈출</span>

`Meesage` 열거형을 통합하기 위해, `ThreadPool` 정의와 `Worker::new` 의
시그니처에서 `Job` 을 `Message` 로 변경해야 합니다:
`ThreadPool` 의 `execute` 메소드는 job 을 `Message::NewJob`
variant 로 감싸서 전달해야 합니다. 그리고
`Worker::new` 의 채널로부터 `Message` 를 받는 부분에선
전달 받은게 `NewJob` 일시 작업을 처리할 것이고,
`Terminate` 일 경우 스레드는 루프를 탈출 할 겁니다.

변경하고 나면, 이 코드는 컴파일 되고 Listing 20-21 과 똑같이 작동 할 겁니다.
하지만 우리가 `Terminate` 메시지를 아무것도 만들지 않았기 때문에
경고가 나타납니다.
우리 `Drop` 구현체를 Listing 20-25 와 같이 수정해서 경고를 고쳐봅시다.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

<span class="caption">Listing 20-25: 각 worker 스레드에 `join` 을 호출하기 전에
`Message::Terminate` 전달하기</span>

이제 우린 각 worker 들을 두번 순회합니다: 한번은 각 worker 에 `Terminate`
메시지를 보내기 위해서고 한번은 각 워커의 스레드에 `join` 을 호출하기 위해서
입니다. 만약 루프를 한번만 이용해서 메시지를 보내는 동시에 `join` 을 호출한다면
현재 반복되는 worker 가 채널에서 메시지를 새로 가져오려 하는 중이란 걸 보장할 수
없기에 별 효과를 볼 수 없습니다.

반복문을 두번으로 나눈 이유를 좀더 자세히 설명해 보겠습니다. 한번 두 worker 를
상상해 보세요. 만약 우리가 반복문을 한번만 사용한다면, 첫번째 반복자에서 종료
메시지가 채널로 전송되고 첫 worker 의 스레드에서 `join` 이 호출될 겁니다. 만약
첫번째 worker 가 요청을 처리하느라 바쁠 경우, 두번째 worker 가 채널에서 종료
메시지를 가져와 종료합니다. 우린 첫번째 worker 가 종료되길 기다리지만, 두번째
스레드가 이미 종료 메시지를 가져가는 바람에 첫번째 worker 는 영원히 종료되지
않습니다.
교착상태(Deadlock) 에 걸려버렸네요!

이 시나리오를 방지하기 위해서는 하나의 반복문으로 모든 `Terminate` 메시지를 채널에 넣어야 합니다;
그 뒤 다른 반복문으로 모든 스레드에 join 합니다.
각 worker 는 종료 메시지를 받으면 채널로부터의 요청 수신을 중지합니다.
따라서 우린 worker 의 수와 같은 수의 종료 메시지를 보내면 각 worker 는
자신의 스레드에 `join` 이 호출되기 전에 종료 메시지를 수신하게 될 거라고
확신할 수 있습니다.

이 코드가 작동하는 걸 보려면, `main` 을 Listing 20-26 에서 나오는 것 처럼
우아하게 종료 되기 전에 오직 두 요청만 받도록 변경해야 합니다.

<span class="filename">파일명: src/bin/main.rs</span>

```rust,ignore
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
```

<span class="caption">Listing 20-26: 두 요청을 처리하고서 반복문을
탈출하게 하여 서버를 종료</span>

여러분은 실제 웹 서버가 달랑 두개의 요청만 처리하고 종료되는걸 원하진 않을겁니다.
이 코드는 어디까지나 우아한 종료 및 정리 작업이 잘 작동하는지 보기위한
시연용 입니다.

`take` 메소드는 `Iterator` 트레잇에 정의되어 있으며 반복을 처음 두 항목으로
제한합니다. `ThreadPool` 은 `main` 의 끝에서 스코프를 벗어나게 될 것이고, `drop`
이 실행 될 것입니다.

`cargo run` 으로 서버를 실행시키고, 요청을 3개 생성해 보세요. 세번째 요청은
에러가 날 것이고, 여러분은 터미널에서 다음과 비슷한 내용의 출력을 보게 될 겁니다.

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0 secs
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

아마 여러분은 작업자와 메시지가 출력된 순서가 다르다는 걸 보셨을 겁니다. 우린 이
메시지로부터 이 코드가 어떻게 작동하는지 알 수 있습니다: worker 0 과 3 이 처음 두
요청을 받고, 그런 다음 3번째 요청에서 서버는 연결 수락(connection accept)을
중지했습니다. `ThreadPool` 이 `main` 의 끝에서 스코프를 벗어나게 되면 `Drop` 이
실행되고, 풀(pool)이 모든 worker 에게 종료 신호를 알립니다. 각 worker 는 자신이
종료 메시지를 받았을때 메시지를 출력하고, 스레드 풀은 각 worker 스레드에 `join`
을 호출합니다.

이 실행 결과의 한가지 흥미로운 점을 주목해보세요:
`ThreadPool` 은 종료 메시지들을 채널로 전송하고,
worker 가 메시지를 수신하기 전에 worker 0 에 join 을 시도합니다.
worker 0 이 아직 종료 메시지를 받지 못했기에
메인 스레드는 worker 0 이 종료될때까지 멈추게 됩니다.
그동안 각 worker 들은 종료 메시지를 수신합니다.
worker 0 이 종료되면, 메인 스레드는 나머지 worker 들이 종료될 때 까지 대기합니다.
이때 그들은 이미 종료 메시지를 받았으므로 종료될 수 있습니다.

축하드립니다! 드디어 우리 프로젝트를 완성했습니다;
우린 스레드 풀을 이용해 비동기적으로 응답하고, 종료될때 풀의 모든 스레드를
정리하는 우아한 종료를 가진 기초적인 웹 서버를 만들었습니다.

다음은 참고용 전체 코드입니다.

<span class="filename">파일명: src/bin/main.rs</span>

```rust,ignore
extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

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

     let mut file = File::open(filename).unwrap();
     let mut contents = String::new();

     file.read_to_string(&mut contents).unwrap();

     let response = format!("{}{}", status_line, contents);

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap();
}
```

<span class="filename">Filename: src/lib.rs</span>

```rust
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

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

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

여기서 더 많은걸 할 수도 있습니다! 만약 여러분이 이 프로젝트를 개선하고
싶으시다면, 여기 몇가지 아이디어를 참고하세요:

* `ThreadPool` 과 public 메소드에 문서 더 추가하기.
* 라이브러리의 기능 테스트 추가하기.
* `unwrap` 호출을 에러 처리가 더 뛰어난 에러 핸들링 호출로 변경하기.
* `ThreadPool` 을 웹 요청을 처리하는 것 외에 다른 작업을 수행하는데 사용해보기.
* *https://crates.io/* 에서 스레드 풀 크레이트를 찾아보고 그를 이용해
  유사한 웹 서버를 구현해보고 그것의 API랑 견고성을 우리가 구현한 스레드 풀과
  비교해 보기.

## 마치며

수고하셨습니다! 여러분은 이 책을 끝마치셨습니다! 이 러스트의 여정에 참여해주셔서
감사드립니다. 여러분은 이제 자신의 러스트 프로젝트를 구현하고 다른 사람들의
프로젝트를 도와줄 준비가 되셨습니다. 여러분이 앞으로 러스트를 사용하시면서 겪으실
어려움을 해결하는데 도움이 되길 원하는 다른 러스트 유저들이 모여있는 커뮤니티가
언제나 여러분을 환영한다는 걸 잊지 마세요.
