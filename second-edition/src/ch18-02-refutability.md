## Refutability: Whether a Pattern Might Fail to Match

Patterns come in two forms: refutable and irrefutable. Patterns which cannot
fail to match for any possible value are said to be *irrefutable*, and patterns
which can fail to match for some possible value are said to be *refutable*.
`let` statements, function parameters, and `for` loops are restricted to only
accept irrefutable patterns, since there’s nothing correct the program could do
if the pattern fails to match. `if let`, and `while let` expressions are
restricted to only accept refutable patterns, since they’re made to handle
possible failure and we wouldn’t need their functionality if the pattern could
never fail.

In general, you shouldn’t have to worry about the distinction between refutable
and irrefutable patterns; just be familiar with the concept of refutability
when you see it mentioned in an error message. When you get an error message
involving refutability, you’ll need to change either the pattern or the
construct you’re using the pattern with, depending on your intentions for the
behavior of the code.

Let’s look at some examples. Earlier in this chapter, we had `let x = 5;`. `x`
is indeed an irrefutable pattern we’re allowed to use: since it matches
anything, it can’t fail to match. In contrast, consider trying to match one
variant of an enum with `let`, such as matching only a `Some<T>` value from the
`Option<T>` enum as shown in Listing 18-7:

```rust,ignore
let Some(x) = some_option_value;
```

<span class="caption">Listing 18-7: Attempting to use a refutable pattern with
`let`</span>

If `some_option_value` was a `None` value, `some_option_value` would not match
the pattern `Some(x)`. The pattern `Some(x)` is refutable since there exists a
case in which it would fail to match a value. There’s nothing valid that our
code could do with this `let` statement if `some_option_value` was the `None`
value. Therefore, Rust will complain at compile time that we’ve tried to use a
refutable pattern where an irrefutable pattern is required:

```text
error[E0005]: refutable pattern in local binding: `None` not covered
 --> <anon>:3:5
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

We didn’t cover (and couldn’t cover!) every valid value with the pattern
`Some(x)`, so Rust will rightfully complain.

If we have a refutable pattern, instead of using `let`, we can use `if let`.
That way, if the pattern doesn’t match, the code inside the curly brackets
won’t execute. That code will only make sense and run if the value matches the
pattern. Listing 18-8 shows how to fix the code in Listing 18-7 with `Some(x)`
matching `some_option_value`. Using the refutable pattern `Some(x)` is allowed,
since this example uses `if let`:

```rust
# let some_option_value: Option<i32> = None;
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

<span class="caption">Listing 18-8: Using `if let` and a block with refutable
patterns instead of `let`</span>

Consequently, if we give `if let` an irrefutable pattern that will always match,
such as `x` as shown in Listing 18-9:

```rust,ignore
if let x = 5 {
    println!("{}", x);
};
```

<span class="caption">Listing 18-9: Attempting to use an irrefutable pattern
with `if let`</span>

Rust will complain that it doesn’t make sense to use `if let` with an
irrefutable pattern:

```text
error[E0162]: irrefutable if-let pattern
 --> <anon>:2:8
  |
2 | if let x = 5 {
  |        ^ irrefutable pattern
```

Generally, match arms use refutable patterns, except for the last arm that
might match any remaining values with an irrefutable pattern. A `match` with
only one arm whose pattern is irrefutable is allowed, but it’s not particularly
useful and could be replaced with a simpler `let` statement. Both the expressions
associated with a `let` statement and a single arm irrefutable match will
unconditionally be run, so the end result is the same if their expressions are.

Now that we’ve discussed all the places that patterns can be used and the
difference between refutable and irrefutable patterns, let’s go over all the
syntax we can use to create patterns.
