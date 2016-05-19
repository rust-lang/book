# if let

There's one more advanced control flow structure we haven't discussed: `if
let`. Imagine we're in a situation like this:

```rust
# let some_option = Some(5);
match some_option {
    Some(x) => {
        // do something with x
    },
    None => {},
}
```

We care about the `Some` case, but don't want to do anything with the `None`
case. With an `Option`, this isn't _too_ bad, but with a more complex enum,
adding `_ => {}` after processing just one variant doesn't feel great. We have
this boilerplate arm, and we have an extra level of indentation: the code that
does something with `x` is indented twice, rather than just once. We really want
a construct that says "Do something with this one case, I don't care about the
others."

Enter `if let`:

```rust
# let some_option = Some(5);
if let Some(x) = some_option {
    // do something with x
}
```

`if let` takes a pattern and an expression, separated by an `=`. It works
exactly like a `match`, where the expression is given to the `match`, and the
pattern is its first arm. In other words, you can think of `if let` as syntax
sugar:

```rust,ignore
if let pattern = expression {
    body
}

match expression {
   pattern => body,
   _ => {}
}
```

And in fact, we can include an `else`, too, and it becomes the body of the `_`
case:

```rust,ignore
if let pattern = expression {
    body
} else {
    else_body
}

match expression {
   pattern => body,
   _ => else_body,
}
```

In other words, it's the high-level construct we were originally looking for:
do something with a single pattern.
