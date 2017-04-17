## Writing a Response

Let's send data back to our browser in response to its request. Responses have
this format:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is called a *status line* and contains the HTTP version used in
the response, a numeric status code that summarizes the result of the request,
and a reason phrase that provides a text description of the status code. After
the CRLF sequence comes any headers, another CRLF sequence, and the body of the
response.

Here's an example response that uses version 1.1 of HTTP, has a status code of
`200`, a reason phrase of `OK`, no headers, and no body:

```text
HTTP/1.1 200 OK\r\n\r\n
```

This text is a tiny successful HTTP response. Let's write this to the stream!
Remove the `println!` that was printing the request data, and add the code in
Listing 20-3 in its place:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 20-3: Writing a tiny successful HTTP response to
the stream</span>

The first new line defines the `response` variable that holds the data of the
tiny success response we're sending back. Then, we call `as_bytes` on our
`response` because the `write` method on `stream` takes a `&[u8]` and sends
those bytes directly down the connection.

The `write` operation could fail, so `write` returns a `Result<T, E>`; we're
continuing to use `unwrap` to make progress on the core ideas in this chapter
rather than error handling. Finally, `flush` will wait until all of the bytes
are written to the connection; `TcpStream` contains an internal buffer to
minimize calls into the underlying operating system.

With these changes, let's run our code and make a request! We're no longer
printing any data to the terminal, so we won't see any output there other than
the output from Cargo. When we load `127.0.0.1:8080` in a web browser, though,
we get a blank page instead of an error. How exciting! You've just hand-coded
an HTTP request and response.

### Returning Real HTML

Let's return more than a blank page. Create a new file, *hello.html*, in the
root of your project directory, that is, not in the `src` directory. You can
put any HTML you want in it; Listing 20-4 shows what the authors used for
theirs:

<span class="filename">Filename: hello.html</span>

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

<span class="caption">Listing 20-4: A sample HTML file to return in a
response</span>

This is a minimal HTML 5 document with a heading and a little paragraph. Let's
modify `handle_connection` as shown in Listing 20-5 to read the HTML file, add
it to the response as a body, and send it:

<span class="filename">Filename: src/main.rs</span>

```rust
# use std::io::prelude::*;
# use std::net::TcpStream;
use std::fs::File;

// ...snip...

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

<span class="caption">Listing 20-5: Sending the contents of *hello.html* as the
body of the response</span>

We've added a line at the top to bring the standard library's `File` into
scope, and the file opening and reading code should look familiar since we had
similar code in Chapter 12 when we read the contents of a file for our I/O
project in Listing 12-4.

Next, we're using `format!` to add the file's contents as the body of the
success response that we write to the stream.

Run it with `cargo run`, load up `127.0.0.1:8080` in your browser, and you
should see your HTML rendered!

Note that we're currently ignoring the request data in `buffer` and sending
back the contents of the HTML file unconditionally. Try requesting
`127.0.0.1:8080/something-else` in your browser and you'll get back your HTML
for that request too. Sending back the same response for all requests is pretty
limited and not what most web servers do; let's examine the request and only
send back the HTML file for a well-formed request to `/`.
