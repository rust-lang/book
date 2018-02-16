% Loops

<small>There is a new edition of the book and this is an old link.</small>

> Rust has three kinds of loops: `loop`, `while`, and `for`.
> The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
> `while` loops evaluate a block of code until a condition ceases to be true.
> A `for` loop executes some code for each item in a collection.

```rust,no_run
loop {
    println!("again!");
}

let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number = number - 1;
}

let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}
```

---

Here are the relevant sections in the new and old books:

* **[In the second edition: Ch 3.05 — Control flow][2]**
* <small>[In the first edition: Ch 3.6 — Loops][1]</small>


[1]: first-edition/loops.html
[2]: second-edition/ch03-05-control-flow.html#repetition-with-loops
