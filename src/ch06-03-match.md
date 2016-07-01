# Match

Rust has an extremely powerful control-flow operator: `match`. It allows us to
compare a value against a series of patterns and then execute code based on
how they compare. Remember the `Option<T>` type from the previous section?
Let's say that we want to write a function that takes an `Option<i32>`, and
if there's a value inside, add one to it. If there isn't a value inside, we
want to return the `None` value and not attempt to add.

This function is very easy to write, thanks to `match`. It looks like this:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            let h = i + 1;
            Some(h)
        },
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Let's break down the `match`! At a high-level, using `match` looks like this:

```text
match expression {
    pattern => code,
}
```

First, we have the `match` keyword. Next, we have an expression. This feels
very similar to an expression used with `if`, but there's a big difference:
with `if`, the condition needs to return a boolean value. Here, it can be any
type.

Next, we have a "match arm". That's the part that looks like `pattern =>
code,`.  We can have as many arms as we need to: our `match` above has two
arms. An arm has two parts: a pattern and some code. When the `match`
expression executes, it compares the resulting value against the pattern of
each arm, in order. If a pattern matches the value, the code associated
with that pattern is executed. If that pattern doesn't match the value,
execution continues to the next arm.

The code associated with each arm is an expression, and the resulting value of
the code with the matching arm that gets executed is the value that gets
returned for the entire `match` expression. If the match arm code is short, as
in the `None` case above, curly braces typically aren't used. If you want to
have multiple lines of code within a `match` arm, you can use curly braces as
in the `Some` case.

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. Let's compare that against each arm:

```text
None => None,
```

Does `Some(5)` match `None`? No, it's the wrong variant. So let's continue.

```text
Some(i) => {
    let h = i + 1;
    Some(h)
},
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. But
what about `i`? In a pattern like this, we can declare new bindings, similarly
to what we did with `let`. So in this case, the code part of the match arm will
have a binding, `i`, which corresponds to the `5`.

With this arm, the code portion is:

```text
let h = i + 1;
Some(h)
```

So we do exactly that: we take `i`, which is `5`, add one to it and bind that
to `h`, then create a new `Some` value with the value of `h` inside.

Because `match` is an expression, the value of the overall expression becomes
the value of the arm that executed. So the value of this `match` expression
will be `Some(6)`, and since our `match` is the only expression in the
function, the value of the `match` will be the value of the function. So
`Some(6)` is our return value as well, which is exactly what we were trying
to accomplish.

Now let's consider the second call of `plus_one()`. In this case, `x` is
`None`. We enter the `match`, and compare to the first arm:

```text
None => None,
```

Does `None` match `None`? Yup! There's no value to add to. So we stop and
return the `None` value that is on the right side of the `=>`. We don't
check any other arms since we found one that matched.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, bind to the data
inside, and then execute code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in languages that don't support
it. It's consistently a user favorite.

## Matches are exhaustive

There's one other aspect of `match` we didn't talk about. Consider this version
of `plus_one()`:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => {
            let h = i + 1;
            Some(h)
        },
    }
}
```

A bug! We didn't handle the `None` case. Luckily, it's a bug Rust knows how to
catch. If we try to compile this code, we'll get an error:

```text
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => {
        let h = i + 1;
        Some(h)
    },
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! This is referred to as being "exhaustive": we must exhaust
every last option possible in order to be valid!

## The _ placeholder

What if we don't care about all of the possible values, though? Especially when
there are a lot of possible values for a type: a `u8` can have valid values of
zero through 255-- if we only care about 1, 3, 5, and 7, does this mean we must
list out 0, 2, 4, 6, 8, 9, all the way up through 255? Thankfully, no! We can
use a special pattern, `_`:

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
the unit value. This way, we don't have to list individual match arms for all
the other possible values in order to say that we want to do nothing for all of
those-- the `_` is a placeholder for any value.

## More about patterns

As we've just seen, patterns are powerful. They can also get complex, so let's
take a whole section to cover all of the things that they can do.
