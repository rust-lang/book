# Unrecoverable errors with panic!

You've already seen the way to signal an unrecoverable error: the `panic!`
macro. Here's an example of using `panic!`:

```rust
fn check_guess(number: u32) -> bool {
    if number > 100 {
        panic!("Guess was too big: {}", number);
    }

    number == 34
}
```

This function accepts a guess between zero and a hundred, and checks if it's
equivalent to the correct number, which is `34` in this case. It's kind of a
silly function, but it's similar to a real example you've already seen:
indexing vectors:

```rust,should_panic
let v = vec![1, 2, 3];

v[1000]; // this will panic
```

The implementation of indexing with `[]` looks similar to our `check_guess`
function above: check if the number is larger than the length of the vector,
and if it is, panic.

Why do we need to panic? There's no number type for "between zero and a
hundred" in Rust, so we are accepting a `u32`, and then checking in the
function's body to make sure the guess is in-bounds. If the number is too big,
there's been an error: someone made a mistake. We then invoke `panic!` to say
"something went wrong, we cannot continue to run this program."

