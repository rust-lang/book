## Match

Rust has an extremely powerful control-flow operator: `match`. It allows us to
compare a value against a series of patterns and then execute code based on
how they compare.

Think of a `match` expression kind of like a coin sorting machine. Coins slide
down a track that has variously sized holes along it, and each coin falls
through the first hole it encounters that it fits into. In the same way, values
go through each pattern in a `match`, and for the first pattern that the value
"fits", the value will fall into the associated code block to be used during
execution.

Since we're already talking about coins, let's use them for an example using
`match`. We can write a function that can take an unknown American coin and, in
a similar way as the coin counting machine, determine which coin it is and
return its value in cents:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
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
code,`.  We can have as many arms as we need to: our `match` above has four
arms. An arm has two parts: a pattern and some code. When the `match`
expression executes, it compares the resulting value against the pattern of
each arm, in order. If a pattern matches the value, the code associated
with that pattern is executed. If that pattern doesn't match the value,
execution continues to the next arm, much like a coin sorting machine.

The code associated with each arm is an expression, and the resulting value of
the code with the matching arm that gets executed is the value that gets
returned for the entire `match` expression.

Curly braces typically aren't used if the match arm code is short, as it is in
the above example where each arm just returns a value. If we wanted to run
multiple lines of code in a match arm, we can use curly braces. This code would
print out "Lucky penny!" every time the method was called with a `Coin::Penny`,
but would still return the last value of the block, `1`:

```rust
# enum Coin {
#    Penny,
#    Nickel,
#    Dime,
#    Quarter,
# }
#
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Another useful feature of match arms is that they can create variables for parts
of the values that match the pattern. From 1999 through 2008, the U.S. printed
quarters with different designs for each of the 50 states on one side. The other
coins did not get state designs, so only quarters have this extra attribute. We
can add this information to our `enum` by changing the `Quarter` variant to have
a `State` value:

```rust
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Let's imagine that a friend of ours is trying to collect all 50 state quarters.
While we sort our loose change by coin type in order to count it, we're going
to call out the name of the state so that if it's one our friend doesn't have
yet, they can add it to their collection.

In the match statement to do this, the quarter case now has a variable, `state`,
that contains the value of the state of that quarter. The variable will only get
created if the coin matches the `Quarter` pattern. Then we can use the variable
in the code for that arm:

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
#
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` will
be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each of the
match arms, none of them match until we reach `Coin::Quarter(state)`. At that
point, the variable `state` will be bound to the value `UsState::Alaska`. We can then
use that variable in the `println!`, thus getting the inner state value out of
the `Coin` enum variant for `Quarter` and enabling us to print "State quarter
from Alaska!".

Remember the `Option<T>` type from the previous section, and that we wanted to
be able to get the inner `T` value out of the `Some` case? This will be very
similar! Instead of coins, we will be comparing to other patterns, but the way
that the `match` expression works remains the same as a coin sorting machine in
the way that we look for the first pattern that fits the value.

Let's say that we want to write a function that takes an `Option<i32>`, and
if there's a value inside, add one to it. If there isn't a value inside, we
want to return the `None` value and not attempt to add.

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

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. Let's compare that against each arm:

```text
None => None,
```

Does `Some(5)` match `None`? No, it's the wrong variant. So let's continue.

```text
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value inside of the `Some`, so `i` has the value `5`. Then we
execute the code in that match arm: take `i`, which is `5`, add one to it, and
create a new `Some` value with our total inside.

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

### Matches are exhaustive

There's one other aspect of `match` we didn't talk about. Consider this version
of `plus_one()`:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

A bug! We didn't handle the `None` case. Luckily, it's a bug Rust knows how to
catch. If we try to compile this code, we'll get an error:

```bash
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => Some(i + 1),
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! This is referred to as being "exhaustive": we must exhaust
every last option possible in order to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null and thus making the billion-dollar mistake we discussed in the
previous section.

### The _ placeholder

What if we don't care about all of the possible values, though? Especially when
there are a lot of possible values for a type: a `u8` can have valid values of
zero through 255-- if we only care about 1, 3, 5, and 7, does this mean we must
list out 0, 2, 4, 6, 8, 9, all the way up through 255? Thankfully, no! We can
use a special pattern, `_`:

```rust
let some_u8_value = 0u8;
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
