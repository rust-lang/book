Please insert this text after the paragraph ending in “ergonomic way to write
it” on page 160.

---

Speaking of different ways to write this function, Listing 9-9 shows that
there’s a way to make this even shorter.

Filename: src/main.rs

```
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Listing 9-9: Using `fs::read_to_string` instead of opening and then reading the
file

Reading a file into a string is a fairly common operation, so Rust provides the
convenient `fs::read_to_string` function that opens the file, creates a new
`String`, reads the contents of the file, puts the contents into that `String`,
and returns it. Of course, using `fs::read_to_string` doesn’t give us the
opportunity to explain all the error handling, so we did it the longer way
first.
