## Reading the Request

Let's read in the request from our browser! Since we're adding more
functionality that has the purpose of handling the connection, let's start a
new function to have a nice separation of the concerns around setting up the
server and connections versus processing each connection. In this new
`handle_connection` function, we'll read data from the `stream` and print it
out in order to see the data that the browser is sending us. Change the code to
look like Listing 20-2:

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
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

Next, we need to actually read from the stream. We do this in two steps: first,
we declare a `buffer` on the stack to hold the data that we read in. We've made
the buffer 512 bytes in size, which is big enough to hold the data of a basic
request. That's sufficient for our purposes in this chapter. If we wanted to
handle requests of an arbitrary size, managing the buffer would need to be more
complicated, but we're keeping it simple for now. We then pass the buffer to
`stream.read`, which will read bytes from the `TcpStream` and put them in the
buffer.

Next, we convert the bytes in the buffer to a string and print out that string.
The `String::from_utf8_lossy` function takes a `&[u8]` and produces a `String`.
The 'lossy' part of the name comes from the behavior when this function sees
invalid UTF-8 sequences: it replaces the invalid sequences with �, `U+FFFD
REPLACEMENT CHARACTER`. You might see the replacement characters for remaining
characters in the buffer that aren't filled by request data.

Let's give this a try! Start up the program and make a request in a web browser
again. Note that we'll still get an error page in the browser, but the output
of our program in the terminal will now look similar to this:

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

You'll probably get slightly different output depending on your browser. You
also might see this request repeated again. Now that we're printing out the
request data, we can see why we're getting multiple connections from one
browser request by looking at the path after `Request: GET`. If the repeated
connections are all requesting `/`, we know the browser is trying to fetch `/`
repeatedly since it's not getting a response from us.

Let's break down this request data to understand what the browser is asking of
us. HTTP is a text-based protocol, and a request takes this format:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is called the *request line*, and it holds information about
what the client is requesting. The first part of the request line is a
*method*, like `GET` or `POST`, that describes how the client is making this
request.

Then comes the request's *URI*, which stands for *Uniform Resource Identifier*.
URIs are almost, but not quite the same as URLs (*Uniform Resource Locators*),
which is what we typically call the addresses that we enter into a web browser.
The HTTP spec uses the term URI, and the difference between URIs and URLs isn't
important for our purposes of this chapter, so we can just mentally substitute
URL for URI here.

Next, we have the HTTP version that the client used, and then the request line
ends in a CRLF sequence. The CRLF sequence can also be written as `\r\n`: `\r`
is a *carriage return* and `\n` is a *line feed*. These terms come from the
typewriter days! The CRLF sequence separates the request line from the rest of
the request data.

Taking a look at the request line data we saw printed out by our code:

```text
GET / HTTP/1.1
```

`GET` is the method, `/` is the Request URI, and `HTTP/1.1` is the version.

The remaining lines starting from `Host:` onward are headers; `GET` requests
have no body.

Try making a request from a different browser, or asking for a different
address like `127.0.0.1:8080/test` to see how the request data changes, if
you'd like.

Now that we know what the browser is asking for, let's send some data back!
