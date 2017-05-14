# Appendix G - Newest Features

This appendix documents features that have been added to stable Rust since the
main part of the book was completed.


## Field init shorthand

We can initialize a data structure (struct, enum, union) with named
fields, by writing `fieldname` as a shorthand for `fieldname: fieldname`.
This allows a compact syntax for initialization, with less duplication:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;

    // Using full syntax:
    let peter = Person { name: name, age: age };

    let name = String::from("Portia");
    let age = 27;

    // Using field init shorthand:
    let portia = Person { name, age };

    println!("{:?}", portia);
}
```
