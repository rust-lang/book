# Creating your own Error types

This pattern of "return an error" is so common, many libraries create their
own error type, and use it for all of their functions. We can re-write the
previous example to use `std::io::Result` rathern than a regular `Result`:

```rust
#![feature(question_mark)]

use std::fs::File;
use std::io;

fn main() {
}

pub fn process_file() -> io::Result<()> {
    let f = File::open("hello.txt")?;

    // do some stuff with f

    Ok(())
}
```

`io::Result` looks like this:

```rust
# use std::io;
type Result<T> = Result<T, std::io::Error>;
```

It's a type alias for a regular-old `Result<T, E>`, with the `E` set up to be a
`std::io::Error`. This means we don't need to worry about the error type in our
function signatures, which is nice.
