# Patterns Match the Structure of Values

Patterns are a special syntax within Rust for matching against the structure of
our types, complex or simple. We take some value and compare it against the
pattern. If the pattern matches our value, then we do something with the value
parts. Recall chapter 6 when we discussed the `match` expression that uses
patterns like a coin sorting machine: patterns describe the "shape" of the data
we're working with. We can name pieces within the shape, like we named the
state that appeared on quarters in Chapter 6, and if the data fits the shape,
we can use the named pieces.

This chapter is a reference on all things related to patterns. We'll cover the
difference between *refutable* and *irrefutable* patterns, the valid places to
use patterns, and the different kinds of pattern syntax that you might see and
what it means.

## Refutability: Whether a Pattern Might Fail to Match

Patterns come in two forms: refutable and irrefutable. Patterns which cannot
fail to match for any possible value are *irrefutable*, and patterns which can
fail to match for some possible value are said to be *refutable*. Some places
where you can use patterns, like `let` statements (which we'll get into more in
the next section in this chapter), are restricted to only accept irrefutable
patterns, since there's nothing correct the program could do if the pattern
fails to match. Other places you can use patterns, like `if let` expressions,
are restricted to only accept refutable patterns, since they're made to handle
possible failure and we wouldn't need their functionality if the pattern could
never fail.

In general, you shouldn't have to worry about the distinction between refutable
and irrefutable patterns; just be familiar with the concept of refutability
when you see it mentioned in an error message. When you get an error message
involving refutability, you'll need to change either the pattern or the
construct you're using the pattern with, depending on your intentions for the
behavior of the code.

Let's now go through all the different places we can use patterns, and we'll
mention whether each place takes a refutable or irrefutable pattern as we do so.

## All the Places Patterns May be Used

Patterns pop up in a number of places in Rust. You've been using them a lot
without realizing it! This section is a reference to all the places where
patterns are valid.

### `match` Arms

As we discussed in Chapter 6, a common place patterns are used is in the arms
of `match` expressions. Formally, `match` expressions are defined as the
keyword `match`, a value, and one or more arms that consist of a pattern and an
expression to run if the value matches that arm's pattern:

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Generally, match arms use refutable patterns. A `match` with only one arm whose
pattern is irrefutable is allowed, but it's not particularly useful and could
be replaced with a simpler `let` statement. There's one caveat with patterns
and `match` expressions, however: the set of all patterns in a `match` must be
*exhaustive* and cover all possible cases.

#### Exhaustiveness and `_`

`match` expressions are required to be exhaustive. When we put all of the
patterns in the arms together, all possibilities for the value in the `match`
expression must be accounted for. One way to ensure you have every possibility
covered is to have an irrefutable pattern for the last arm, like a variable
name. Since a name matching any value is an irrefutable pattern, it can never
fail and thus covers every case remaining after the previous arms' patterns by
definition.

There's an additional irrefutable pattern that's often used in the last match
arm: `_`. It matches anything, but it never binds any variables. This can be
useful when you only want to do things for some patterns, but ignore the rest,
for example.

#### Shadowing in patterns

As with all variables, those declared by a pattern will shadow variables
outside of the `match` construct since a `match` starts a new scope. In the
next example, we declare a variable named `x` with the value `Some(5)` and a
variable `y` with the value `10`. Then we have a `match` expression on the
value `x`. Take a look at the patterns in the match arms and the `println!` at
the end, and make a guess about what will be printed before running this code
or reading further:

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

The first match arm has the pattern `Some(50)`, and the value in `x`
(`Some(5)`) does not match `Some(50)`, so we continue. In the second match arm,
the pattern `Some(y)` introduces a new variable name `y` that will match any
value inside a `Some` value. Because we're in a new scope inside the `match`
expression, this is a new variable, not the `y` we declared at the beginning
that has the value 10. So the new `y` binding will match any value inside a
`Some`, which is what we have in `x`, so we execute the expression for that arm
and print `Matched, y = 5` since this `y` binds to the inner value of the
`Some` in `x`, which is 5.

If `x` had been a `None` value instead of `Some(5)`, we would have matched the
underscore since the other two arms' patterns would not have matched. In the
expression for that match arm, since we did not introduce an `x` variable in
the pattern of the arm, this `x` is still the outer `x` that has not been
shadowed. In this hypothetical case, the `match` would print `Default case, x =
None`.

Once the `match` expression is over, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.

### `let` Statements

The `match` statement isn't the only place we can use patterns, though. For
example, consider this straightforward variable assignment with `let`:

```rust
let x = 5;
```

We've done this hundreds of times throughout this book. You may not have
realized it, but you were using patterns! A `let` statement looks like this,
more formally:

```text
let PATTERN = EXPRESSION;
```

We've seen statements like `let x = 5;` with a variable name in the `PATTERN`
slot; a variable name is just a particularly humble form of pattern.

With `let`, we compare the expression against the pattern, and assign any names
we find. So for example, in our `let x = 5;` case, `x` is a pattern that says
"bind what matches here to the variable `x`. And since the name `x` is the
whole pattern, this pattern effectively means "bind everything to the variable
`x`, whatever the value is."

To see the pattern matching aspect of `let` a bit more clearly, consider this
code:

```rust
let (x, y, z) = (1, 2, 3);
```

Here, we have a tuple that we're matching against a pattern. Rust will compare
the value `(1, 2, 3)` to the pattern `(x, y, z)`, and see that the value
matches the pattern. In this case, it will bind `1` to `x`, `2` to `y`, and `3`
to `z`.

We can mix, match, and nest patterns: you can think of this tuple pattern as
nesting three individual variable patterns inside of it. We can also do
something more complicated like this example where we nest tuples inside of
tuples:

```rust
let ((one, two), name, (three, four), five) = ((1, 2), "hello", (3, 4), 5);
```

Usually, Rust will warn you if you create a variable but don't use it anywhere,
since that could be a bug. If you're prototyping or just starting a project,
though, you might create a variable that you'll use eventually, but temporarily
it will be unused. If you're in this situation and would like to tell Rust not
to warn you about the unused variable, you can start the name of the variable
with an underscore. This works just like a variable name in any pattern, only
Rust won't warn you if the variable goes unused. In the following example, we
do get a warning about not using the variable `y`, but we don't get a warning
about not using the variable `_x`:

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

`let` takes an irrefutable pattern.

One place where patterns are required to be irrefutable is in `let` statements.
Consider our `let x = 5;` example. `x` is indeed an irrefutable pattern we're
allowed to use: since it matches anything, it can't fail to match. In contrast,
consider trying to match one variant of an enum with `let`, such as matching
only a `Some<T>` value from the `Option<T>` enum:

```rust,ignore
let Some(x) = some_option_value;
```

If `some_option_value` was the `None` value, `some_option_value` would not
match the pattern `Some(x)`. The pattern `Some(x)` is refutable since there
exists a case in which it would fail to match a value. There's nothing valid
that our code could do with this `let` statement if `some_option_value` was the
`None` value. Therefore, Rust will complain at compile time that we've tried to
use a refutable pattern where an irrefutable pattern is required:

```text
error[E0005]: refutable pattern in local binding: `None` not covered
 --> <anon>:3:5
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

We didn't cover (and couldn't cover!) every valid option with the pattern
`Some(x)`, so Rust will rightfully complain.

### `if let` statements

If we have a refutable pattern, instead of using `let`, we use `if let`. That
way, if the pattern doesn't match, the code inside the curly braces won't
execute. That code will only make sense and run if the value matches the
pattern. Here's our example with `Some(x)` matching `some_option_value` that is
allowed, since it uses `if let`:

```rust
# let some_option_value: Option<i32> = None;
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

Consequently, if we give `if let` an irrefutable pattern that will always match,
such as `x`:

```rust,ignore
if let x = 5 {
    println!("{}", x);
};
```

Rust will complain that it doesn't make sense to use `if let` with an
irrefutable pattern:

```text
error[E0162]: irrefutable if-let pattern
 --> <anon>:2:8
  |
2 | if let x = 5 {
  |        ^ irrefutable pattern
```

### Function Parameters

Similarly to `let`, function parameters are also irrefutable patterns:

```rust
fn foo(x: i32) {
    // code goes here
}
```

The `x` part is a pattern! If we didn't want to use the argument in the body of
our function, we could use `_` for example:

```rust
fn foo(_: i32) {
    // code goes here
}
```

Normally, you just wouldn't declare an argument, but maybe you're implementing
a trait, or need a certain type signature for some other reason. This lets you
not have to use the argument, and the compiler won't warn about unused function
parameters like it would if we had given it a name .

### `while let`

TODO: add a `while let` example

### `for` loops

TODO: add a `for` loop example

## All the Pattern Syntax

Here's a list of all of the different types of patterns.

### Literals & _

You can match against literals directly, and `_` acts as an any case:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one`.

Literals are refutable patterns, but `_` is irrefutable.

### Multiple patterns

You can match multiple patterns with `|`:

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one or two`.

### `ref` and `ref mut`

Usually, when you match against a pattern, variables are bound to a value.
This means you'll end up moving the value into the `match`:

```rust,ignore
let name = Some(String::from("Bors"));

match name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

// name is moved here. This line will fail to compile:
println!("name is: {:?}", name);
```

If you'd prefer to bind `name` to a reference, use the `ref` keyword:

```rust
let name = Some(String::from("Bors"));

match name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

// name is not moved here; the match only took a reference to its data rather
// than moving it. This will work:
println!("name is: {:?}", name);
```

And for a mutable reference, `ref mut`:

```rust
let mut name = Some(String::from("Bors"));

match name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

// name is not moved here; the match only took a reference to its data rather
// than moving it
println!("name is: {:?}", name);
```

### Destructuring

Patterns can be used to destructure structs and enums:

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

let Point { x, y } = origin;
```

This brings `x` and `y` variables into scope, matching the `x` and `y` of
`origin`. While it can be unusual in `let`, this is the same principle of
patterns in `match`:

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x, y } => { }, // variables x and y are created here
}
```

### Ignoring values

We discussed using `_` as a whole pattern to ignore it above, but you can
also use `_` inside of another pattern to ignore just part of it:

```rust
let x = Some(5);

match x {
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

Or like this:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

If you want, you can use `..` to ignore all of the parts you haven't defined:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => { }, // y and z are ignored
}
```

### Ranges

You can match a range of values with `...`:

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

Ranges are usually used with integers or `char`s:

```rust
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

### Guards

You can introduce match guards with `if`:

```rust
let x = Some(5);

match x {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

If youre using if with multiple patterns, the if applies to both sides:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
}
```

This prints `no`, because the if applies to the whole of `4 | 5`, and not to
only the `5`. In other words, the precedence of if behaves like this:

```text
(4 | 5) if y => ...
```

not this:

```text
4 | (5 if y) => ...
```

### `@` Bindings

TODO: difference between this and variable names

You can bind values to names with `@`:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id @ 3...7 } => println!("{}", id),
    _ => (),
}
```

In this case, we want to compare `id` against the range `3...7`, but we also
want to save the actual value of `id`.

## Summary

TODO: summary
