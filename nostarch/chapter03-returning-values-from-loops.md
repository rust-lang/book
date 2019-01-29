Please insert this new section after the "Repeating Code with loop" section ends and before the "Conditional Loops with while" section starts, on page 53.

#### Returning Values From Loops

One of the uses of a `loop` is to retry an operation you know can fail, such as
checking if a thread completed its job. However, you might need to pass the
result of that operation to the rest of your code. If you add the value you
want to return after the `break` expression you use to stop the loop, it will
be returned out of the loop so you can use the value, as shown here:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

Before the loop, we declare a variable named `counter` and initialize it to
zero. Then we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add one to the counter variable,
and then check if the counter is equal to ten. When it is, we use the `break`
keyword with the value `counter * 2`. After the loop, we place a semicolon to
end the statement assigning the value to `result`. Finally, we print out the
value in `result`, which in this case will be twenty.
