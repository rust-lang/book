## Concise control flow with `if let`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to
handle values that match one pattern and ignoring the rest. Take the following
program:

```rust
# let some_option = Some(5);
match some_option {
    Some(x) => {
        // do something with x
    },
    None => (),
}
```

We want to do something with the `Some` match, but do nothing with the `None`
case. We can do this with an `Option`, but with a more complex enum,
adding `_ => ()` after processing just one variant is a lot of boilerplate code
that we have to add to satisfy the `match` expression.

Instead, we could write this in a shorter way with `if let`. This code behaves
exactly the same as the `match` above:

```rust
# let some_option = Some(5);
if let Some(x) = some_option {
    // do something with x
}
```

`if let` takes a pattern and an expression separated by an `=`. It works
just like a `match`, where the expression is given to the `match` and the
pattern is its first arm.

Using `if let` means you have less to type, less indentation, and less
boilerplate. However, we’ve lost the exhaustiveness checking that `match`
enforces. Choosing between `match` and `if let` depends on what you’re doing in
your particular case, and if gaining conciseness is an appropriate tradeoff for
losing exhaustiveness checking.

In other words, you can think of `if let` as syntax sugar for a `match` that
runs code when the value matches one pattern and then ignores all other values.

We can include an `else` that goes with an `if let`. The block of code that
goes with the `else` is the same as the block of code that would go with the
`_` case in the `match` expression that is equivalent to the `if let` and
`else`. Recall the `Coin` enum definition in Listing 6-3, where the `Quarter`
variant also held a `UsState` value. If we wanted to count all non-quarter
coins we see while also announcing the state of the quarters, we could do that
with a `match` expression like this:

```rust
# #[derive(Debug)]
# enum UsState {
#    Alabama,
#    Alaska,
# }
#
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter(UsState),
# }
# let coin = Coin::Penny;
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Or we could choose to use an `if let` and `else` expression like this:

```rust
# #[derive(Debug)]
# enum UsState {
#    Alabama,
#    Alaska,
# }
#
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter(UsState),
# }
# let coin = Coin::Penny;
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

If you find yourself in a situation where your program has logic that is
verbose to express using a `match`, remember that `if let` is in your Rust
toolbox as well.

## Summary

We’ve now covered how to use enums to create custom types that can be one of a
set of enumerated values. We’ve shown how the standard library’s `Option<T>`
type helps you use the type system to prevent errors. When enum values have data
inside them, you can use `match` or `if let` to extract and use those values,
depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and
enums. Creating custom types to use in your API ensures type safety: the
compiler will make certain your functions only get values of the type each
function expects.

In order to provide a well-organized API to your users that is straightforward
to use and only exposes exactly what your users will need, let’s now turn to
Rust’s *modules*.
