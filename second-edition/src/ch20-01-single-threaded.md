## 싱글스레드 기반 웹 서버 만들기

싱글 스레드 기반의 웹 서버가 작동하는것을 알아보는것 부터 시작하겠습니다.
시작하기 전에, 웹서버를 구성하는 프로토콜들에 대해 빠르게 훑어봅시다.
프로토콜들에 대한 자세한 설명은 이 책의 범주를 넘어가지만,
간단한 설명은 여러분에게 도움이 될 것입니다.

웹서버의 두 주요 프로토콜은 *HTTP(Hyper Transfer Protocol)* 과
*TCP(Transmission Control Protocol)* 입니다.
이 두 프로토콜은 *요청-응답 (request-response)* 프로토콜입니다.
요청-응답은 클라이언트가 요청을 생성하면, 서버는 요청을 받고
클라이언트에게 응답하는 과정을 뜻합니다.
요청과 응답의 내용은 각 프로토콜에 의해 정의됩니다.

TCP는 저레벨 프로토콜로, 한 서버에서 다른 서버로 정보를 요청할때 사용하지만,
해당 정보가 무엇인지는 특정하지 않습니다. HTTP는 TCP 상위에서 만들어졌으며,
요청과 응답의 내용을 정의하고 있습니다.
HTTP가 TCP 이외의 프로토콜을 사용하는것은 기술적으론 불가능하지 않지만,
일반적으로 HTTP통신은 TCP프로토콜 위에서 이루어집니다.
이번 장에선 TCP를 이용한 바이트통신과 HTTP를 이용한 요청과 응답을 실습해 볼 것입니다.

### TCP 연결에 대한 처리

우리가 만들 웹 서버는 TCP 연결 요청에 대한 처리를 해야하기 떄문에, TCP 연결 요청을 수신하는 것 부터 작업하도록 하겠습니다.
이 작업은 표준 라이브러리에서 제공하는 `std::net` 모듈을 이용해 진행할 수 있습니다.
하던대로 새 프로젝트를 만들어 봅시다.

```text
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

이제 20-1번 예제의 _src/main.rs_ 코드를 입력합시다.
이 코드는 `127.0.0.1:7878` 주소로 TCP 연결 요청에 대해 수신 대기할 것입니다.
만약 요청이 들어온다면, `Connection established!` 가 출력될 것입니다.

<span class="filename">파일명: src/main.rs</span>

```rust,no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

<span class="caption">예제 20-1: 수신 스트림 대기와 수신시 메시지 출력</span>

우린 `TcpListener` 를 사용하여 `127.0.0.1:7878` 주소로 TCP연결을 수신할 수 있습니다.
이 주소의 `:` 앞부분은 여러분의 컴퓨터의 IP주소를 뜻합니다.
(`127.0.0.1` 은 loopback IP로, 현재 컴퓨터를 가리키는 IP 주소입니다)
그리고 `7878` 은 포트를 뜻합니다.
여기서 이 포트를 사용한 이유는 두가지입니다.
HTTP는 일반적으로 이 포트에서 요청되며, 7878은 "rust"를 전화기에서 입력했을때의 숫자이기 떄문입니다.

Note: 80포트를 이용한 연결은 관리자 권한이 필요하다는 점을 유의하세요;
비 관리자는 1024 이상의 포트번호만 사용 가능합니다.

위 코드에서 `bind` 함수는 `new` 함수처럼 동작하며
`TcpListner` 의 새 인스턴스를 반환합니다.
이 함수가 `bind` 라는 이름을 가진 이유는 네트워크 관련에서
포트를 수신대기하는 과정을 "포트를 binding한다" 라고 부르기 떄문입니다.

`bind` 함수는 바인딩의 성공여부를 나타내는 `Result<T, E>` 를 반환합니다.
예를들어, 우리가 만약 80포트를 관리자가 아닌 상태에서
연결하려고 시도할 경우나 같은 포트를 사용하는 프로그램을 여러개 실행할 경우에
바인딩은 실패하게 됩니다. 우리는 학습을 목적으로 서버를 작성하고 있기 때문에,
이러한 에러에 대한 처리를 해줄 필요가 없습니다.
따라서, 우리는 `unwrap` 을 이용해 에러가 생길 경우 프로그램을 멈출것입니다.

`TcpListener` 의 `incoming` 메소드는 스트림의 차례에 대한 반복자를 반환합니다.
(보다 정확히는, 여러 스트림의 종류 중 `TcpStream` 에 해당합니다)
각각의 *stream* 은 클라이언트와 서버간의 열려있는 커넥션을 의미합니다.
*connection* 은 클라이언트가 서버와 연결하고, 서버가 응답을 생성하고,
서버가 연결을 끊는 요청과 응답 과정을 통틀어 의미합니다.
이와같이, `TcpStream` 은 클라이언트가 보낸 정보를 읽어들이고,
우리의 응답을 스트림에 작성할 수 있게 해줍니다.
전체적으로, 이 `for` 반복문은 각각의 연결을 처리하고
우리에게 일련의 스트림들을 다룰 수 있도록 해줍니다.

현재, 우리는 어떠한 오류가 있을경우 `unwrap` 을 호출하여
프로그램을 종료시키는 방식으로 스트림을 처리합니다.
만약 오류가 없을경우, 프로그램은 메시지를 출력합니다.
우린 다음 항목에서 오류가 없을 경우에 대한 기능을 더 추가할 것입니다.
우리가 `incoming` 메소드를 통해서 에러를 받을때의 이유는,
클라이언트가 서버로 연결할때 우리가 실제적인 연결을 반복하는것이 아닌,
연결 시도를 반복하기 때문입니다. 
연결은 몇가지 이유로 실패할 수 있는데, 대다수의 경우 운영체제의 특성 때문입니다.
예를들어, 대부분의 운영체제는 동시에 열어놓을 수 있는 연결 개수에 제한을 가지고 있는데,
제한 이상으로 연결을 시도할 경우 이미 열려있는 연결이 닫힐때까지 오류를 발생시킵니다.

한번 이 코드를 실행해 봅시다. `cargo run` 을 터미널에 입력하고,
브라우저에서 `127.0.0.1:7878` 로 접속해봅시다.
브라우저는 "연결 재시도"와 같은 에러를 보여줄 것입니다.
이 이유는 현재의 서버는 어떠한 데이터도 전송하지 않기 때문입니다.
하지만 터미널을 보면, 브라우저가 서버에 접속할때 출력된 메시지들을 볼 수 있습니다!

```text
     Running `target/debug/hello`
Connection established!
Connection established!
Connection established!
```

종종, 브라우저로 한번 요청했을때 여러 메시지가 출력되는걸 보실겁니다.
이유는 브라우저가 페이지뿐만 아니라 다른 여러 리소스를 요청하기 때문입니다.
요청되는 다른 리소스들중 대표적인 것은 브라우저 탭에 표시되는 아이콘인
`favicon.ico` 가 있습니다.

또한 서버가 어떠한 데이터도 보내주지 않기 때문에
브라우저가 여러번 연결을 시도했기 때문일 수도 있습니다.
`stream` 이 영역를 벗어날 경우와 반복이 끝날때, `drop` 이 실행되는것처럼 연결이 끊어집니다.
브라우저는 서버와의 연결 문제가 일시적일 수도 있다고 생각하여
끊어진 연결을 재시도하기도 합니다.
여기서 중요한건 우리가 성공적으로 TCP연결을 처리했다는 것입니다.

이전 버전의 코드가 실행되는 프로그램을 종료할때는
<span class="keystroke">ctrl-c</span>를 누르고,
여러분이 만든 새 버전의 코드를 실행하기 위해
`cargo run` 명령어를 입력하는것을 기억하세요

### 요청 데이터 읽기

브라우저로부터의 요청을 읽는 기능을 구현해봅시다!
부담갖지 말고 '연결하기', '연결을 이용해보기' 로 나눠서 진행해봅시다.
연결을 처리하기 위해 새 함수를 만들어 봅시다.
여기선 `handle_connection` 이라는 함수를 새로 만들었습니다.
TCP 스트림에서 데이터를 읽고 출력해보며 브라우저가 보낸 데이터를 직접 확인해봅시다.
코드를 예제 20-2와 같이 변경합니다.

<span class="filename">파일명: src/main.rs</span>

```rust,no_run
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

<span class="caption">예제 20-2: `TcpStream`으로부터 데이터를 읽고
출력</span>

우린 `std::io::prelude`를 가져와 스트림으로부터
읽고 쓰는것을 허용하는 특성에 접근할 수 있도록 합니다.
`main`함수 내부의 `for`반복문 안에서는, 연결에 성공했다는 메시지를 출력하는 대신,
새로 만든 `handle_connection` 함수를 `stream` 을 전달하여 호출합니다.

`handle_connection` 함수에선, `stream` 매개변수를 가변으로 만들어 줬습니다.
이유는 `TcpStream` 인스턴스가 내부에서 어떤 데이터가 우리에게 반환되는지 추적하기 떄문입니다.
우리가 요청하는것에 따라 더 많은 데이터를 읽거나,
다음 요청때까지 데이터를 저장할 수 있습니다.
이와 같이 내부의 상태가 변경될 수 있기에 `mut` 이 되어야 합니다.
보통 "읽기"는 변화와 관련이 없다고 생각하지만 이 경우 `mut` 키워드가 필요합니다.

다음으로, 실제로 스트림으로부터 데이터를 읽어봅시다. 이는 두가지 과정으로 나뉘는데:
먼저, 우리는 `buffer` (버퍼)를 읽을 데이터를 저장할 스택에 선언해야 합니다.
여기선 버퍼를 기본적인 요청을 저장하는것과 우리의 목적에 충분한 크기인
512바이트로 만들었습니다. 만약 임의의 크기를 가진 요청을 다룰땐
버퍼관리는 좀 더 복잡해져야 할 테지만, 지금은 단순하게 생각합시다.
우린 버퍼를 `stream.read` 로 전달했는데 이 함수는 `TcpStream`  으로부터
읽어들인 바이트를 버퍼로 집어넣는 역할을 합니다.

두번째로, 버퍼 안에있는 바이트들을 문자열로 변환하고 출력합니다.
`String::from_utf8_lossy` 함수는 `&[u8]` 을 전달받고 `String` 으로 바꿔서 제공해줍니다.
함수의 이름 중 "lossy"는 이 함수가 유효하지 않은 UTF-8 배열을 만났을때의
행동을 나타냅니다.
유효하지 않은 배열은 `U+FFFD REPLACEMENT CHARACTER` 라는 `�` 로 교체되는데,
여러분은 아마 이 문자를 버퍼중 요청 데이터로 채워지지 않은곳에서 볼겁니다.

한번 코드를 실행해 보죠. 프로그램을 시작하고 웹 브라우저로 요청을 다시 보내봅시다.
브라우저는 여전히 에러페이지를 띄우겠지만,
우리의 프로그램은 아래와 비슷한 내용을 터미널에 출력할겁니다.

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

여러분의 브라우저에 따라서 조금씩 다른 출력결과가 나올 수 있습니다.
이렇게 요청 데이터를 출력해보았습니다. 이제 여러분은 `Reguest: GET` 뒤의 경로를 보고
어째서 한 브라우저가 여러번 연결 요청을 보냈는지 알 수 있습니다.
만약 반복되는 요청들이 모두 `/` 를 요청하고 있다면, 알다시피 브라우저가
우리의 프로그램에게서 응답을 받지 못했기 대문에 `/` 를 가져오려고 하는 것입니다.

한번 이 요청 데이터를 분석하며 브라우저가
우리 프로그램에 뭘 물어보는지 이해해 봅시다.

### HTTP 요청을 자세히 살펴보기

HTTP는 텍스트 기반 프로토콜이고, HTTP 요청은 아래와 같은 양식을 따릅니다.

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

첫번째 줄은 *request_line* 이고 클라이언트가 무슨 요청을 하는지에 대한 정보를 담고 있습니다.
요청 라인의 첫번째 부분은 사용된 메소드를 나타냅니다. `GET` 이나 `POST` 등을 말하는데,
이는 클라이언트가 어떻게 이 요청을 만들었는지 나타냅니다.
우리 클라이언트는 `GET` 요청을 사용했습니다.

요청 라인의 다음 부분은 `/` 입니다. 클라이언트가 요청한
*URI(Uniform Resource Identifier)* 를 나타내는데,
이는 *URL(Uniform Resource Locator)* 과 완전히는 아니지만 거의 똑같습니다.
URI와 URL의 차이는 이번 장의 우리 의도와는 관련이 별로 없으므로
여기선 머릿속으로 URL로 URI를 대체하도록 합시다.

마지막 부분은 클라이언트가 사용하는 HTTP 버전입니다. 그 다음은
CRLF 시퀀스로 요청라인이 끝나게 됩니다. CRLF 시퀀스는 `\r\n` 으로도
쓰일 수 있습니다: 여기서 `\r` 부분은 *carriage return* 이고 `\n` 은
*line feed* 입니다. (이 표현은 타자기 시절부터 이어져 온 것입니다)
CRLF 시퀀스는 요청 라인을 나머지 요청 데이터로부터 분리시키는 역할을 합니다.
여기서 CRLF 시퀀스는 출력되었을때 `\r\n` 이 아닌 줄바꿈이 되는걸 기억하세요.

이제 우리 프로그램이 실행되는동안 받은 요청 라인 데이터를 한번에 살펴봅시다.
`GET` 이 요청 메소드, `/` 가 요청 URI,
`HTTP/1.1` 은 버전을 뜻합니다.

요청 라인 이후의 남은 라인들중 `Host:` 이후는 모두 헤더입니다.
(일반적으로 `GET` 메소드를 통한 요청은 body를 가지지 않습니다)

한번 다른 브라우저나 다른 주소( `127.0.0.1:7878/test` 등)으로 요청을 보내보고,
요청 데이터가 어떻게 변화하는지도 살펴보세요.

이제 브라우저가 요청하는 내용이 무슨 뜻인지 알았으니, 역으로 데이터를 보내봅시다!

### 응답 작성하기

이제 클라이언트의 요청에 대응하는 응답을 보내 봅시다.
HTTP 응답은 아래와 같은 양식을 가집니다.

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

첫번째 줄은 *status line* 입니다.
이곳엔 응답에 사용된 HTTP 버전,
요청에 대한 결과를 나타내는 상태 코드,
상태 코드에 대한 설명 구문(텍스트)이 들어가 있습니다.
CRLF 시퀀스 이후는 헤더, 또 다른 CRLF의 뒤는 응답의 body가 들어갑니다.

여기 HTTP 1.1버전을 이용한 응답 예시가 있습니다.
상태 코드는 200, 설명 문구는 OK, 헤더와 body는 없습니다.

```text
HTTP/1.1 200 OK\r\n\r\n
```

200 상태 코드는 응답 성공을 뜻하는 표준 응답 코드입니다.
이제 이것을 요청에 대한 응답으로 스트림에 작성해 봅시다.
요청 데이터를 출력하는데 사용했던
`handle_connection` 함수의 `println!` 을 지우고
아래 20-3 예제의 코드를 대신 써 넣으세요.

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="caption">예제 20-3: 스트림에 간단한 HTTP 응답 성공 메시지
작성하기</span>

첫번째 새 줄에선 응답 메시지를 저장할 `response` 변수를 선언했습니다.
그 뒤 `response` 의 `as_bytes`를 호출하여 문자열 데이터를
바이트 배열로 변환합니다. `stream` 의 `write` 메소드는 `&[u8]` 을
전달받고 커넥션에 바이트 배열을 전송합니다.

`write` 작업이 실패할 수 잇기 때문에, 우린 전처럼 `unwrap` 을 사용합니다.
다시 말하지만, 실제 어플리케이션에선 이런 경우에 에러 처리를 해야합니다.
마지막으로, `flush` 는 모든 바이트들이 커넥션으로 쓰여질때까지 프로그램을 대기시킵니다.
`TcpStream` 은 운영체제의 기능 호출을 최소화하기 위해 내부적으로 버퍼를 사용하기 때문에,
커넥션으로 전송시키기 위해선 `flush` 를 이용해 이 버퍼를 비워야 합니다.

코드를 실행시키고, 요청을 만들어 봅시다.
어떤 데이터도 출력되지 않지만,
웹 브라우저로 `127.0.0.1:7878` 로 접속했을때,
에러 대신 빈 페이지를 볼 수 있을 것입니다.
여러분은 HTTP 요청과 응답을 직접 코딩해 보셨습니다!

### 실제 HTML로 응답하기

빈 페이지보다 더 많은걸 응답하는 기능을 만들어 봅시다.
*src* 폴더가 아닌 여러분의 프로젝트 디렉토리의 루트 디렉토리에 *hello.html* 파일을 새로 만든 뒤,
예제 20-4처럼 HTML을 작성하세요
(여러분이 원하는대로 내용을 바꾸셔도 됩니다)

<span class="filename">파일명: hello.html</span>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

<span class="caption">예제 20-4: 응답할 HTML
파일 예시</span>

위는 간단한 내용을 가진 HTML5 문서입니다.
이를 서버에서 요청이 들어왔을때 반환하도록 바꾸기 위해서,
`handle_connection` 을 예제 20-5처럼 HTML파일을 읽고,
응답의 body에 추가하고, 전송하도록 수정합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
use std::fs::File;
// --생략--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

<span class="caption">예제 20-5: *hello.html* 의 내용을 응답의
body에 넣고 전송하기</span>

맨 윗줄에서 `File` 표준 라이브러리를 가져왔습니다.
파일을 열고 내용을 읽는 코드는
12장 (예제 12-4)의 I/O 프로젝트에서
파일 내용을 읽을때 작성해봤으니 익숙하실 겁니다.

다음으로, 우린 `format!` 을 이용해 응답 데이터의 body부분에
파일의 내용을 추가했습니다.

이 코드를 `cargo run` 을 이용해 실행하고 브라우저로 `127.0.0.1:7878` 로 접속해보세요,
여러분이 작성한 HTML이 화면에 나타날 것입니다!

하지만 우리는 현재 `buffer` 안에 있는 요청 데이터를 무시하고
무조건 HTML 파일의 내용을 전송합니다. 이 말은 여러분이
브라우저로 `127.0.0.1:7878/something-else` 에 접속해도 똑같은 HTML이 나타난다는 뜻입니다.
우리의 서버는 매우 제한적이고 일반적인 웹 서버와는 다릅니다.
우리는 요청에 따라서 응답을 지정하고자 하고,
오직 `/` 에 대한 정상적인 요청에 대해서만 HTML 파일을 전송하려 합니다.

### 요청을 확인하고 선택적으로 응답하기

현재 우리 웹 서버는 클라이언트가 무엇을 요청했는지에 관계없이
파일 안의 HTML을 반환합니다.
한번 HTML 파일을 반환하기 전에 브라우저가 `/` 를 요청하는지 확인하고,
만약 다른걸 요청한다면 에러를 반환하는 기능을 추가해봅시다.
이를 위해 우리는 `handle_connection` 을 예제 20-6과 같이 수정할 필요가 있습니다.
이 코드는 받은 요청 내용을 우리가 아는 `/` 로의 요청 내용과 비교하여
`if` 와 `else` 블록을 추가해 요청들을 다르게 처리하는 코드입니다.

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --생략--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 기타 다른 요청
    }
}
```

<span class="caption">예제 20-6: 요청을 비교하고 `/` 로의 요청을
다른 요청들과 다르게 처리하기</span>

먼저, 우리는 `/` 로의 요청에 해당하는 데이터를 `get` 변수에 하드코딩했습니다.
우리가 버퍼에서 읽어들이는 것은 원시 바이트이기 때문에
`get` 에 바이트 문자열 구문인 `b""` 를 추가해
바이트 문자열로 바꿔줍니다.
이후 `if` 블록에서 `buffer` 가 `get` 의 내용으로 시작하는지 체크합니다.
만약 그렇다면 우린 정상적인 `/` 로의 요청을 받았다는 뜻이니,
우리의 HTML 파일의 내용을 반환합니다.

만약 `buffer` 가 `get` 의 내용으로 시작하지 *않는다면* ,
다른 요청을 받았다는 뜻입니다.
이 요청들을 처리할 `else` 블록의 코드는 잠시 후에 작성할 예정입니다.

이 코드를 실행시키고 `127.0.0.1:7878` 로 요청을 보내봅시다.
여러분은 *hello.html* 의 HTML을 받았을겁니다.
만약 `127.0.0.1:7878/somthing-else` 등의 다른 요청을 보낸다면,
여러분이 예제 20-1과 예제 20-2를 실행했을 때처럼 연결 에러가 날 것입니다.

이제 예제 20-7의 코드를 `else` 블록에 추가하고 응답으로
`404` 상태코드를 보내봅시다.
여기서 `404` 상태 코드는 요청에 대한 내용을 찾을 수 없다는 뜻을 신호입니다.
또한 우린 최종 유저에게 브라우저에서 보여질 페이지를 위한 HTML도 반환해볼겁니다.

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
# fn handle_connection(mut stream: TcpStream) {
# if true {
// --생략--

} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
# }
```

<span class="caption">예제 20-7: `/` 로의 요청이 아닐 경우
`404` 상태 코드와 에러 페이지를 응답</span>

여기서, 우린 `404` 상태 코드와 `NOT FOUND` 상태 메시지를 가진
status line을 응답에 포함하고 있습니다.
헤더는 없고, body는 *404.html* 파일의 HTML 내용입니다.
여러분은 *hello.html* 옆에 에러 페이지에 사용할 *404.html* 을 만들고,
자유롭게 HTML 을 입력하거나 예제 20-8의 예시를 사용하시기 바랍니다.

<span class="filename">파일명: 404.html</span>

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

<span class="caption">예제 20-8: `404` 응답에 전달될
페이지의 내용 예시</span>

변경사항들을 포함하고 다시 서버를 실행시켜 보세요.
`127.0.0.1:7878` 로의 요청은 *hello.html* 의 내용을 반환할 것이고,
그 외의 요청 (`127.0.0.1:7878/foo` 등)은 *404.html* 의 내용을 반환할 것입니다.

### 리팩토링

`if` 와 `else` 블록에는 중복되는 부분이 많습니다.
둘다 파일을 읽고, 파일의 내용을 스트림에 작성합니다.
차이점은 오직 status line과 파일명뿐입니다.
코드를 좀더 간결하게하기 위해 이 차이점만 `if` 와 `else` 줄로 분리하고
status line과 파일명을 변수로 맡기도록 합시다.
그럼 우린 이 변수들을 파일을 읽고 응답을 작성하는데 쓰기만 하면 됩니다.
예제 20-9에서 `if` 와 `else` 블록의
코드 대부분을 변경한 결과를 보실 수 있습니다.

<span class="filename">파일명: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
# use std::fs::File;
// --생략--

fn handle_connection(mut stream: TcpStream) {
#     let mut buffer = [0; 512];
#     stream.read(&mut buffer).unwrap();
#
#     let get = b"GET / HTTP/1.1\r\n";
    // --생략--

    let (status_line, filename) = if buffer.starts_with(get) {
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

<span class="caption">예제 20-9: `if` 와 `else` 블록을 각각이 처리하는 내용 중
다른부분만 포함하도록 리팩토링</span>

이제 `if`와 `else` 블록은 오직 각각의 적절한 status line과
파일명을 튜플로 반환할 뿐입니다. 우린 이 두 값을 18장에 나온
`let` 의 표현 패턴을 이용해 분리하고
각각을 `status_line` 과 `filename` 변수로 대입합니다.

이전의 중복된 코드는 이제 `if`와 `else` 블록의 밖에 있습니다.
그리고 `status_line` 과 `filename` 변수를 사용함으로써,
두 경우의 차이를 쉽게 볼 수 있고, 만약 우리가 파일을 읽고
응답하는 과정을 개선하고 싶을때 한 곳만 수정해도 된다는 이점을 얻을 수 있습니다.
물론 예제 20-9에 나온 코드는
예제 20-8에 나온것과 똑같이 동작할것입니다.

훌륭합니다! 우린 이제 대략 40줄 정도의 러스트 코드로 한 요청에 대해선
내용 있는 페이지로 응답하고 그 외의 요청은 `404` 를 응답하는
간단한 웹 서버를 만들어 보았습니다.

현재, 우리가 만든 서버는 싱글 스레드로 동작합니다.
즉 한번에 하나의 요청밖에 대응하지 못합니다.
이게 왜 문제가 되는지 느린 요청들이 들어온 상황을 시뮬레이팅하고,
우리 서버가 한번에 여러 요청을 처리할 수 있도록 고칠것입니다.