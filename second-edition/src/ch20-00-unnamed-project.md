# Un-named project

It's been a long journey, but here we are! It's the end of the book. Parting is
such sweet sorrow. But before we go, let's build one more project together, to
show off some of the things we learned in these final chapters, as well as
re-cap some of the earlier ones.

Here's what we're going to make: a web server that says hello:

![hello from rust](hello.png)

Before we get started, however, there's one thing we should mention: if you were
writing this code in production, there are a lot of better ways to write it.
Specifically, there are a number of robust crates on crates.io that would make
writing this easier. However, for this chapter, our intention is to learn, not
to take the easy route. So we'll be writing a basic implementation ourselves.

# Accepting a TCP connection

The HTTP protocol is built on top of the TCP protocol. So the first thing we need
to build our webserver is to be able to listen to a TCP connection. The standard
library has a `std::net` module that lets us do this. Let's make a new project:

```bash
$ cargo new hello --bin
     Created binary (application) `hello` project
$ cd hello
```

We'll put this in `src/main.rs`:

```rust,no_run
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

Let's talk about each part in turn:

```rust,ignore
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

```

A `TcpListener` allows us to listen for TCP connections. We've chosen to listen
to the address `127.0.0.1:8008`. The first four digits are an IP address
representing our own computer, and `8080` is the port. We've chosen this port
becuase HTTP is normally accepted on port 80, but connecting to port 80 requires
administrator privledges. Regular users can listen on ports higher than 1024;
8080 is easy to remember since it's port 80, but twice.

The `bind` method is sort of like `new`, but with a more descriptive name. In
networking, people will often talk about "binding to a port", and so the
function is called `bind`. Finally, it returns a `Result<T, E>`; binding may
fail. For example, if we had tried to connect to port 80 without being an
administrator. Since we're writing a basic client here, we're not going to worry
about handling these kinds of errors, and so `unwrap` lets us ignore them.

```rust,ignore
for stream in listener.incoming() {
```

The `incoming` method on `TcpListener` gives us an iterator that gives us a
sequence of streams, more specifically, `TcpStream`s. This struct represents an
open connection, and will let us read from and write to it. So this `for` loop
will process each connection in turn, and produce a series of streams. We can
then handle each one in turn.

```rust,ignore
let stream = stream.unwrap();

println!("Connection established!");
```

Right now, "handling" a stream means `unwrap`ping it to ignore any futher
errors, and then printing a message. Let's try this code out! First invoke
`cargo run`:

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
warning: unused variable: `stream`, #[warn(unused_variables)] on by default
 --> src\main.rs:8:13
  |
8 |         let stream = stream.unwrap();
  |             ^^^^^^

     Running `target\debug\hello.exe`
```

And then load up `127.0.0.1:8080` in your web browser. Your browser will
show an error message, something like "Connection reset", but if you look
at your terminal...

```bash
     Running `target\debug\hello.exe`
Connection established!
Connection established!
Connection established!
```

A bunch of messages! Why did we get multiple ones? Well, our browser is
expecting to speak HTTP, but we aren't replying with anything, just closing the
connection. This might be the browser making a request for the page and a
request for a `favicon.ico`, it might be retrying on its own... the important
thing is that we've successfully gotten a handle on a TCP connection!

In order to keep things clean, let's move our processing of the connection out
to a function. Modify your code to look like this:

```rust
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    println!("Connection established!");
}
```

Now we can worry about handling the `TcpStream` in `handle_connection` only, and
not worry about all of the connection processing stuff.

# Reading the request

Let's read in the request from our browser! Modify our code like this:

```rust
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // no changes in here!
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

We've added one new `use` declaration, importing the `std::io` module's
`prelude`. This will bring important traits into scope that let us read from and
write to the stream.

In `handle_connection`, we had to make `stream` mutable with the `mut` keyword.
We're going to be reading data from the stream, so it's going to get modified.
Next, we need to actually read from the stream; we do this in two steps:

```rust,ignore
let mut buffer = [0; 512];

stream.read(&mut buffer).unwrap();
```

First, we declare a `buffer` on the stack; we've made it 512 bytes. Why 512?
It's big enough to get a basic request, but not super huge. If we wanted to
handle requests of an arbitrary size, this would need to be more
complicated, but we're keeping it simple for now! We then pass that buffer
to `stream.read`. This will read bytes from the `TcpStream` and put them in
the buffer.

Next, we print that stream out:

```rust
println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
````

The `String::from_utf8_lossy` function will take a `&[u8]` and produce a `String`. The
'lossy' part of its name comes from its behavior when it sees invalid UTF-8 sequences;
it replaces them with �, `U+FFFD REPLACEMENT CHARACTER`.

Let's give this a try!

```bash
$ cargo run
   Compiling hello v0.1.0 (file:///C:/Users/steve/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
     Running `target\debug\hello.exe`
Request: GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:52.0) Gecko/20100101 Firefox/52.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
������������������������������������
```

You'll probably get slightly different output depending on your browser! You
also might see this request repeated; now we can tell that the reason we
have multiple connections is because the browser is trying to fetch `/`
repeatedly. Let's break this request down. HTTP is a text-based protocol, a
request looks like this:

```text
Request-Line headers CRLF message-body
```

First there's a 'request line'. Then, any headers. Next, a CRLF sequence, and then, the body
of the message. A request line looks like this:

```text
Request-Line = Method Request-URI HTTP-Version CRLF
```

First, we have a method, like `GET` or `POST`. Then, the request's URI, which is
often a URL. Next, we have the HTTP version, and then a CRLF sequence. That's
`\r\n` is the CRLF sequence; `\r` is a "carriage return" and `\n` is a "line
feed"; these terms come from the typewriter days!

If we apply this to our request:

```text
GET / HTTP/1.1
Host: 127.0.0.1:8080
<more headers>
```

`GET` is our method, `/` is our Request URI, and `HTTP/1.1` is our version.
All the stuff from `Host` and after are headers. `GET` requests have no body.
Neat!

# Writing a response

 Let's respond to our browser with a response. Responses look like this:

```text
Status-Line headers CRLF message-body
```

First, we need a status line. Then, any headers. Next, a CRLF sequence, and then, the body
of the message. What's a status line? Here's an example of one:

```text
HTTP/1.1 200 OK\r\n\r\n
```

Status lines look like this:

```text
Status-Line = HTTP-Version Status-Code Reason-Phrase CRLF
```

We're using version 1.1 of the protocol, and `200` is the status code. `OK` is
the "reason phrase", it's like a text description of the stauts code. Finally,
`\r\n` is the CRLF sequence; `\r` is a "carriage return" and `\n` is a "line
feed"; these terms come from the typewriter days!

We don't have any headers, so there's nothing to put there. Next, another CRLF
to separate the headers from the body, which is empty. Whew! With this text,
we've got a successful, tiny, HTTP response. We have to write it to the stream
though! Let's modify our code to write out a response. Remove the `println!`
line, and add these below:

```rust,ignore
let response = "HTTP/1.1 200 OK\r\n\r\n";

stream.write(header.as_bytes()).unwrap();
stream.flush().unwrap();
```

The first line defines our response. Then, we call `as_bytes` on our `response`,
as the `write` method on `stream` takes a `&[u8]`, and writes those bytes
directly down the connection. This could fail, so `write` returns a `Result<T,
E>`; we continue to use `unwrap` to make progress here. Finally, `flush()` will
wait until all of the underlying bytes are written to the connection;
`TcpStream` contains an internal buffer to minimize calls into the underlying
operating system.

With these changes, let's run our code!

```bash
> cargo run
   Compiling hello v0.1.0 (file:///C:/Users/steve/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39 secs
     Running `target\debug\hello.exe`
```

Once we've loaded `127.0.0.1:8080` in our web browser... we get a blank page!
How exciting! You've just hand-coded an HTTP request and response. From here on
out, it's all just details.

## Returning real HTML

Let's return more than a blank page. Create a new file, `hello.html`, in the
root of the project; that is, not in the `src` directory. You can put any
HTML you want in it, here's what I put in mine:

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

This is a minimal HTML 5 document, with a header and a little paragraph. Let's
modify `handle_connection` to read that file, append it to our header, and send
it as the response:

```rust,ignore
// add this import at the top
use std::fs::File;

// our new handle_connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let header = "HTTP/1.1 200 OK\r\n\r\n";
    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Here's opening and reading the file:

```rust,ignore
let mut file = File::open("hello.html").unwrap();
let mut contents = String::new();

file.read_to_string(&mut contents).unwrap();
```

We talked about this in the I/O project chapter, so this should
look fairly familliar. We open the file with `File::open`, and
the read it into a `String` with `file.read_to_string`.

Next, we write our response out:

```rust,ignore
let header = "HTTP/1.1 200 OK\r\n\r\n";
let response = format!("{}{}", header, contents);

stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

We use `format!` to concatenate our header onto the body,
and then change `write` to write `response`. Easy! Run it with
`cargo run`, load up `127.0.0.1:8080` in your browser, and you
should see your HTML rendered!

# Validating the request

Right now, our web server will return this HTML no matter what the request.
Let's check that the browser is requesting `/`, and then return an error if
it's not. First, modify `handle_connection` to look like this:

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let start = &buffer[..get.len()];

    if start == get {
        // success!
    } else {
        // error :(
    };
```

Here, we defined the HTTP request we're looking for with `get`. Because we are
reading raw bytes into the buffer, we use a byte string, with `b"`, to make this
a byte string too. Then, we take a slice of the `buffer` that's the same length
as `get`, and compare them. If they're identical, we've gotten a good request.
If not, we've gotten a bad request.

Let's add in the code to handle each side:

```rust,ignore
if start == get {
    let header = "HTTP/1.1 200 OK\r\n\r\n";
    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
} else {
    let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut file = File::open("404.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
};
```

The interesting bit is in the else case:

```rust
let header = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
let mut file = File::open("404.html").unwrap();
```

`404 NOT FOUND` is the proper error code here. And we need to make a new file,
`404.html`, to go along with `hello.html`. Here's its contents:

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

With these changes, try running your server again. Requesting `127.0.0.1:8080` should
return our `hello.html`, and any other request, like `127.0.0.1:8080/foo`, should return
our error!

There's a lot of repetition in this function; let's pull it out:

```rust
   let (header, filename) = if start == get {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Here, the only thing in our `if` is the header and the filename; we then use
destructuring to assign these two bits to `filename` and `header`. We have to
change the call to `File::open` to use this new variable.

Awesome! We have a simple little web server in ~40 lines of Rust code. So far,
this project has been relatively straightforward as far as Rust code goes; we
haven't done much of the more advanced things yet. Let's kick it up a notch
and add a feature to our web server: a threadpool.

# Threadpools