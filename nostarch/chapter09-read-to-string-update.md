Please insert this text after the paragraph ending in "ergonomic way to write it" on page 160.

---

Speaking of different ways to write this function, there’s a way to make this
even shorter, shown in Listing 9-9.

Filename: src/main.rs

```
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Listing 9-9: Using `fs::read_to_string` instead of opening then reading the file

Reading a file into a string is a fairly common operation, so Rust provides a
convenience function called `fs::read_to_string` that will open the file,
create a new `String`, read the contents of the file, put the contents into
that `String`, and return it. Of course, this doesn’t give us the opportunity
to explain all the error handling, so we did it the longer way first.