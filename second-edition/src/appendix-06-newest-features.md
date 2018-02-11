# Appendix F - Newest Features

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


## Returning from loops

One of the uses of a `loop` is to retry an operation you know can fail, such as
checking if a thread completed its job. However, you might need to pass the
result of that operation to the rest of your code. If you add it to the `break`
expression you use to stop the loop, it will be returned by the broken loop:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```
