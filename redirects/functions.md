% Functions

<small>There is a new edition of the book and this is an old link.</small>

> Function definitions in Rust start with `fn` and have a set of parentheses after the function name.
> The curly brackets tell the compiler where the function body begins and ends.
> We can call any function we’ve defined by entering its name followed by a set of parentheses. 

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

---

Here are the relevant sections in the new and old books:

* **[In the first edition: Ch 3.2 — Functions][1]**
* <small>[In the second edition: Ch 3.03 — Functions][2]</small>


[1]: first-edition/functions.html
[2]: second-edition/ch03-03-how-functions-work.html
