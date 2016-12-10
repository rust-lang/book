# An I/O Project

`cargo new --bin greprs`

Project requirements:

- Take a search string and a filename as command line arguments
- Read the file
- Find lines in the file that contain the search string
- Print out those lines

In order to do a good job, we will:

- Organize code (using what we learned in modules, ch 7)
- Use strings (collections, ch 8)
- Handle errors (ch 9)
- Have tests (ch 11)

Generics/traits/lifetimes?

## Command line arguments

* Use `std::env::args()`

Filename: src/main.rs

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

