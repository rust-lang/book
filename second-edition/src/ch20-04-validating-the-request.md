## Validating the Request

Right now, our web server will return this HTML no matter what the request.
Let's check that the browser is requesting `/`, and then return an error if
it's not. First, modify `handle_connection` to look like this:

```rust,ignore
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
reading raw bytes into the buffer, we use a byte string, with `b"`, to make
this a byte string too. Then, we take a slice of the `buffer` that's the same
length as `get`, and compare them. If they're identical, we've gotten a good
request. If not, we've gotten a bad request.

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

```rust,ignore
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

With these changes, try running your server again. Requesting `127.0.0.1:8080`
should return our `hello.html`, and any other request, like
`127.0.0.1:8080/foo`, should return our error!

There's a lot of repetition in this function; let's pull it out:

```rust,ignore
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

Here's our final code. Don't forget those two HTML files as well!

```rust,ignore
use std::fs::File;
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

// our new handle_connection
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let start = &buffer[..get.len()];


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

Awesome! We have a simple little web server in ~40 lines of Rust code. So far,
this project has been relatively straightforward as far as Rust code goes; we
haven't done much of the more advanced things yet. Let's kick it up a notch and
add a feature to our web server: a thread pool.
