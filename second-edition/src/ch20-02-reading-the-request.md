## Reading the Request

Let's read in the request from our browser! Since we're adding more
functionality that has the purpose of handling the connection, let's start a
new function to have a nice separation of the concerns around setting up the
server and connections versus processing each connection. In this new
`handle_connection` function, we'll read data from the `stream` and print it
out in order to see the data that the browser is sending us. Change the code to
look like Listing 20-2:

<span class="filename">Filename: src/main.rs</span>

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

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

<span class="caption">Listing 20-2: Reading from the `TcpStream` and printing
out the data</span>

We added `std::io::prelude` to the beginning in order to bring traits into
scope that let us read from and write to the stream. Instead of printing a
message that we got a connection in the `for` loop in `main`, we're calling the
new `handle_connection` function and passing the `stream` to it.

In `handle_connection`, we made the `stream` parameter mutable with the `mut`
keyword. We're going to be reading data from the stream, so it's going to get
modified.

Next, we need to actually read from the stream; we do this in two steps. First,
we declare a `buffer` on the stack; we've made it 512 bytes. Why 512? It's big
enough to get a basic request, but not super huge. If we wanted to handle
requests of an arbitrary size, this would need to be more complicated, but
we're keeping it simple for now! We then pass that buffer to `stream.read`.
This will read bytes from the `TcpStream` and put them in the buffer.

Next, we print that stream out. The `String::from_utf8_lossy` function takes a
`&[u8]` and produce a `String`. The 'lossy' part of its name comes from its
behavior when it sees invalid UTF-8 sequences; it replaces them with �, `U+FFFD
REPLACEMENT CHARACTER`.

Let's give this a try!

```text
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target/debug/hello`
Request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101
Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

You'll probably get slightly different output depending on your browser! You
also might see this request repeated; if so, we can definitively tell that the
reason we have multiple connections is because the browser is trying to fetch
`/` repeatedly.

Let's break this request data down. HTTP is a text-based protocol, and a
request looks like this:

```text
Request-Line headers CRLF message-body
```

First there's a 'request line'. Then, any headers. Next, a CRLF sequence, and
then, the body of the message. A request line looks like this:

```text
Request-Line = Method Request-URI HTTP-Version CRLF
```

First, we have a method, like `GET` or `POST`. Then, the request's URI, which
is a term the HTTP spec uses. You have probably heard of a 'URL'. All URLs are
URIs, but not all URIs are URLs. Since this isn't a book about the HTTP
specification, given this fact, we can just think "URL" when we see "URI" and
move on. Next, we have the HTTP version, and then a CRLF sequence. That's
`\r\n` is the CRLF sequence; `\r` is a "carriage return" and `\n` is a "line
feed"; these terms come from the typewriter days!

If we apply this to our request:

```text
GET / HTTP/1.1
Host: 127.0.0.1:8080
<more headers>
```

`GET` is our method, `/` is our Request URI, and `HTTP/1.1` is our version. All
the stuff from `Host` and after are headers. `GET` requests have no body. Neat!
