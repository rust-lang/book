## Writing a Response

Let's respond to our browser with a response. Responses look like this:

```text
Status-Line headers CRLF message-body
```

First, we need a status line. Then, any headers. Next, a CRLF sequence, and
then, the body of the message. What's a status line? Here's an example of one:

```text
HTTP/1.1 200 OK\r\n\r\n
```

Status lines look like this:

```text
Status-Line = HTTP-Version Status-Code Reason-Phrase CRLF
```

We're using version 1.1 of the protocol, and `200` is the status code. `OK` is
the "reason phrase", it's like a text description of the status code. Finally,
`\r\n` is the CRLF sequence; `\r` is a "carriage return" and `\n` is a "line
feed"; these terms come from the typewriter days!

We don't have any headers, so there's nothing to put there. Next, another CRLF
to separate the headers from the body, which is empty. Whew! With this text,
we've got a successful, tiny, HTTP response. We have to write it to the stream
though! Let's modify our code to write out a response. Remove the `println!`
line, and add these below:

```rust,ignore
let response = "HTTP/1.1 200 OK\r\n\r\n";

stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

The first line defines our response. Then, we call `as_bytes` on our
`response`, as the `write` method on `stream` takes a `&[u8]`, and writes those
bytes directly down the connection. This could fail, so `write` returns a
`Result<T, E>`; we continue to use `unwrap` to make progress here. Finally,
`flush()` will wait until all of the underlying bytes are written to the
connection; `TcpStream` contains an internal buffer to minimize calls into the
underlying operating system.

With these changes, let's run our code!

```text
> cargo run
   Compiling hello v0.1.0 (file:///projects/hello/src/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39 secs
     Running `target\debug\hello.exe`
```

Once we've loaded `127.0.0.1:8080` in our web browser... we get a blank page!
How exciting! You've just hand-coded an HTTP request and response. From here on
out, it's all just details.

### Returning Real HTML

Let's return more than a blank page. Create a new file, `hello.html`, in the
root of the project; that is, not in the `src` directory. You can put any HTML
you want in it, here's what the authors used for theirs:

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

This is a minimal HTML 5 document, with a heading and a little paragraph. Let's
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

We talked about this in the I/O project chapter, so this should look fairly
familiar. We open the file with `File::open`, and the read it into a `String`
with `file.read_to_string`.

Next, we write our response out:

```rust,ignore
let header = "HTTP/1.1 200 OK\r\n\r\n";
let response = format!("{}{}", header, contents);

stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

We use `format!` to concatenate our header onto the body, and then change
`write` to write `response`. Easy! Run it with `cargo run`, load up
`127.0.0.1:8080` in your browser, and you should see your HTML rendered!
