# Match

Rust has an extremely powerful control-flow operator: `match`. It allows us to
compare a value against a series of patterns, and then execute code based on
how they compare. Remember the `Option<T>` type from the previous section?
Let's say that we want to write a function that takes an `Option<i32>`, and
if there's a value inside, add one to it.

This function is very easy to write, thanks to `match`. It looks like this:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Let's break down the `match`! At a high-level, the `match` expression looks
like this:

```text
match condition {
    pattern => code,
}
```

First, we have the `match` keyword. Next, we have a condition. This feels very
similar to an `if` expression, but there's a big difference: with `if`, the
condition needs to be a boolean. Here, it can be any type.

Next, we have a "match arm". That's the part that looks like `pattern =>
code,`.  We can have as many arms as we need to: our `match` above has two
arms. An arm has two parts: a pattern, and some code. When the `match`
expression executes, it compares the condition against the pattern of each arm,
in turn. If the pattern matches the condition, the associated code is executed,
and the rest of the patterns are not checked. If it doesn't match, execution
continues to the next arm.

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. Let's compare that against each arm:

```text
None => None,
```

Does `Some(5)` match `None`? No, it's the wrong variant. So let's continue.

```text
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. But
what about `i`? In a pattern like this, we can declare new bindings, similarly
to what we did with `let`. So in this case, the code part of the match arm will
have a binding, `i`, which corresponds to the `5`.

With this arm, the code portion is `Some(i + 1)`. So we do exactly that: we
take `i`, which is `5`, add one to it, and create a new `Some` value with our
sum inside.

Because `match` is an expression, the value of the overall expression becomes
the value of the arm that executed. So the value of this `match` expression
will be `Some(6)`. And since our `match` is the only expression in the
function, the value of the `match` will be the value of the function, and so
`Some(6)` is our return value as well, which is exactly what we were shooting
for.

Now let's consider the second call. In this case, `x` is `None`. We enter the
`match`, and compare to the first arm:

```text
None => None,
```

Does `None` match `None`? Yup! And so we return `None`. There's no value to add
to.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, binding to the data
inside, and then executing code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in languages that don't support
it. It's consistently a user favorite.

## Matches are exhaustive

There's one other aspect of `match` we didn't talk about. Consider this version
of `plus_one()`:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

A bug! We didn't handle the `None` case. Luckily, it's a bug Rust knows how to catch.
If we try to compile this code, we'll get an error:

```text
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => Some(i + 1),
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! This is referred to as being "exhaustive", we must exhaust
every last option possible in order to be valid!

This analysis isn't perfect, however. This will also error:

```rust,ignore
# let some_u8_value = 0u8;
match some_u8_value {
    0 => println!("zero"),
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    6 => println!("six"),
    7 => println!("seven"),
    // We won't write out all of the arms here, but imagine that there are more
    // arms corresponding to the rest of the numbers.
    254 => println!("two-hundred and fifty-four"),
    255 => println!("two-hundred and fifty-five"),
}
```

Even though a `u8` can only have valid values of zero through 255, Rust isn't
quite smart enough to understand we've covered all the cases. In order to fix
this, we can use a special pattern, `_`:

```rust
# let some_u8_value = 0u8;
match some_u8_value {
    0 => println!("zero"),
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    6 => println!("six"),
    7 => println!("seven"),
    // ...
    254 => println!("two-hundred and fifty-four"),
    255 => println!("two-hundred and fifty-five"),
    _ => panic!("can't ever happen"),
}
```

The `_` pattern matches anything at all, and so with it as the final pattern,
Rust can understand that we have all our bases covered. It's not only used for
this sort of exhaustiveness issue, though. It's useful any time we don't want to
deal with a number of cases. Consider this scenario: if we wanted to print out
something one one, three, five, and seven:

```rust
# let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The `_` pattern will match all the other cases, and `()` will do nothing, it's
the unit value.

## More about patterns

As we've just seen, patterns are powerful, yet complex. Let's take a whole
section to cover all of the things that they can do.
