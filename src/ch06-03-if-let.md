## if let With match

<!-- Do we use if let with match, is that why we include it in this chapter?
That would be worth specifying -->

There's one more advanced control flow structure we haven't discussed that's
often used with the `match` enum: `if let`.

Take the following program:

```rust
match some_option {
    Some(x) => {
        // do something with x
    },
    None => {},
}
```

We want to do something with the `Some` match, but do nothing with the `None`
case. We can do this with an `Option`, but with a more complex enum,
adding `_ => {}` after processing just one variant doesn't feel great.
<!-- Could you be more specific about why that's bad, say it explicitly? -->

We have this boilerplate arm and an extra level of indentation for the code
that does something with `x`. We really want a construct that says "Do
something with this one case; do nothing with anything else."

<!-- I'm not totally clear how this is different to the last case--I liked this
line below from the original documentation and added it in, what do you think?
-->

The `if let` syntax lets you combine `if` and `let` to reduce the overhead for
certain kinds of pattern matching, so rather than using `match` we can do the
following:

```rust
if let Some(x) = some_option {
    // do something with x
}
```

<!--- So would we only use this if let pattern when searching for one case and
discarding all others, rather than searching for multiple cases? If so, can you
make that clear early on. What is the advantage of using if let over match,
here, it's more efficient for this one-case situation? -->

`if let` takes a pattern and an expression separated by an `=`. It works
exactly like a `match`, where the expression is given to the `match` and the
pattern is its first arm.

In other words, you can think of `if let` as syntax
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

<!-- Can you elaborate on this? -->

If we include an `else` and it becomes the body of the `_`
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

<!--- Can you talk this through a little, and perhaps add comments to label one
as mathc and one as if let, just to make it clear you're comparing the two? Why
is the if let else better than match here? -->

In other words, it's the high-level construct we were originally looking for:
do something special with only one pattern, and treat all others the same way.

<!-- Could you add a chapter summary? -->
