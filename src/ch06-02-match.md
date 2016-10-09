## Match

Rust has an extremely powerful control-flow operator, `match`, that allows us to
compare a value against a series of patterns and then execute code based on
which pattern matches.

Think of a `match` expression kind of like a coin sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls
through the first hole it encounters that it fits into. In the same way, values
go through each pattern in a `match`, and at the first pattern the value
"fits", the value will fall into the associated code block to be used during
execution.

Since we're already talking about coins, let's use them for an example using
`match`! We can write a function that can take an unknown American coin and, in
a similar way as the counting machine, determine which coin it is and
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

Let's break down `match`. At a high-level, using `match` looks like this:

```text
match expression {
    pattern => code,
    pattern => code,
}
```
<!--- Flagging as a place to possibly put wingding numbers -- would it work to
put two arms in this example? I think that would illustrate the control flow
well -->

First, we list the `match` keyword followed by an expression. This feels
very similar to an expression used with `if`, but there's a big difference:
with `if`, the expression needs to return a boolean value. Here, it can be any
type.

Next, we have the *match arms*. An arm has two parts: a pattern and some code.
When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn't match the
value, execution continues to the next arm, much like a coin sorting machine.
We can have as many arms as we need: our `match` above has four arms.

The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets
returned for the entire `match` expression.

Curly braces typically aren't used if the match arm code is short, as it is in
the above example where each arm just returns a value. If we wanted to run
multiple lines of code in a match arm, we can use curly braces. This code would
print out "Lucky penny!" every time the method was called with a `Coin::Penny`,
but would still return the last value of the block, `1`:

```rust
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
### Matching for Attributes

Another useful feature of match arms is that they can create bindings to parts
of the values that match the pattern. This is useful for

<!--- Above, maybe give an explicit example of what we'd use this for -->

From 1999 through 2008, the U.S. printed quarters with different designs for
each of the 50 states on one side. No other coins got state designs, so only
quarters have this extra attribute. We can add this information to our `enum`
by changing the `Quarter` variant to include a `State` value as an argument:

```rust
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
While we sort our loose change by coin type, we're also going to call out the
name of the state associated with each quarter so that if it's one our friend
doesn't have they can add it to their collection.

In the match statement for this, we add a binding, `state`, to the quarter
variant that contains the value of that quarter's state. The binding will only
be created if the coin matches the `Quarter` pattern. Then we can use the
binding in the code for that arm like so:

```rust
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

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
will be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.

### Matching with Option<T>

In the previous section we wanted to get the inner `T` value out of the `Some`
case when using Option<T>; we can so this in a very similar way! Instead of
comparing coins we will be comparing patterns, but the way that the `match`
expression works remains the same.

Let's say we want to write a function that takes an `Option<i32>`, and if
there's a value inside, adds one to that value. If there isn't a value inside,
it should return the `None` value and not attempt to perform any operations.

This function is very easy to write, thanks to `match`, and will look like this:

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
<!-- Flagging for wingding numbers -->

#### A Some Match

Let's examine the first execution of `plus_one()` in more detail. In the above
example, `x` will be `Some(5)`. We compare that against each arm:

```text
None => None,
```

The `Some(5)` pattern doesn't match the variant `None`, so we continue.

```text
Some(i) => Some(i + 1),
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. In
that case, the `i` binds to the value contained in `Some`, so `i` takes the
value `5`. The code in the match arm is then executed, so we add one to the new
value of `i` and create a new `Some` value with our total `6` inside.

#### A None Match

Now let's consider the second call of `plus_one()` where `x` is
`None`. We enter the `match`, and compare to the first arm:

```text
None => None,
```

It matches! There's no value to add to, so the program stops and
returns the `None` value on the right side of `=>`. Since
the first arm matched, no other arms are compared.

Combining `match` and enums together is extremely powerful. You'll see this
pattern a lot in Rust code: `match` against an enum, bind to the data
inside, and then execute code based on it. It's a bit tricky at first, but
once you get used to it, you'll wish you had it in all languages.
It's consistently a user favorite.

### Matches are Exhaustive

There's one other aspect of `match` we haven't discussed. Consider this version
of our `plus_one()` program:

```rust,ignore
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn't handle the `None` case, so this will cause a bug. Luckily, it's a bug
Rust knows how to catch. If we try to compile this code, we'll get this error:

```bash
error: non-exhaustive patterns: `None` not covered [E0004]
match x {
    Some(i) => Some(i + 1),
}
```

Rust knows that we did not cover every possible option, and even knows which
pattern we forgot! Enums in Rust are *exhaustive*: we must exhaust
every last option possible in order to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null and thus making the billion-dollar mistake discussed earlier.

### The _ Placeholder

Rust also has an enum tool for dealing with situations when we don't want to
list all possible values. When there are a lot of possible values for a
type---for example, a `u8` can have valid values of zero through 255---we don't
want to list out 0, 2, 4, 6, 8, 9 all the way up to 255 if we only care about
1, 3, 5, and 7. We can use the special pattern `_` instead:

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

The `_` pattern is a placeholder for any value, so by putting it after our
other arms it will match all the possible cases that aren't specified before
it. The `()` syntax is the unit value and will do nothing. This way, we can say
that we want to do nothing for all of the possible values that we don't list
before the `_` placeholder.
