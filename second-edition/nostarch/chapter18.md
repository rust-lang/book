
[TOC]

# Patterns Match the Structure of Values

Patterns are a special syntax within Rust for matching against the structure of
our types, complex or simple. A pattern is made up of some combination of
literals; destructured arrays, enums, structs, or tuples; variables, wildcards,
and placeholders. These pieces describe the “shape” of the data we’re working
with.

We use a pattern by taking some value and comparing it against the pattern. If
the pattern matches our value, we do something with the value parts. Recall in
Chapter 6 when we discussed the `match` expression that uses patterns like a
coin sorting machine. We can name pieces within the shape, like we named the
state that appeared on quarters in Chapter 6, and if the data fits the shape,
we can use the named pieces.

This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between *refutable* and
*irrefutable* patterns, and the different kinds of pattern syntax that you
might see.

## All the Places Patterns May be Used

Patterns pop up in a number of places in Rust. You’ve been using them a lot
without realizing it! This section is a reference to all the places where
patterns are valid.

### `match` Arms

As we discussed in Chapter 6, a common place patterns are used is in the arms
of `match` expressions. Formally, `match` expressions are defined as the
keyword `match`, a value to match on, and one or more match arms that consist
of a pattern and an expression to run if the value matches that arm’s pattern:

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

#### Exhaustiveness and the Default Pattern `_`

`match` expressions are required to be exhaustive. When we put all of the
patterns in the arms together, all possibilities for the value in the `match`
expression must be accounted for. One way to ensure you have every possibility
covered is to have a catch-all pattern for the last arm, like a variable name.
A name matching any value can never fail and thus covers every case remaining
after the previous arms’ patterns.

There’s an additional pattern that’s often used in the last match arm: `_`. It
matches anything, but it never binds any variables. This can be useful when you
only want to run code for some patterns but ignore any other value, for example.

### `if let` Expressions

We discussed `if let` expressions in Chapter 6, and how they’re mostly a
shorter way to write the equivalent of a `match` that only cares about matching
one case. `if let` can optionally have a corresponding `else` with code to run
if the pattern in the `if let` doesn’t match.

Listing 18-1 shows that it’s even possible to mix and match `if let`, `else
if`, and `else if let`. This code shows a series of checks of a bunch of
different conditions to decide what the background color should be. For the
purposes of the example, we’ve created variables with hardcoded values that a
real program might get by asking the user. If the user has specified a favorite
color, we’ll use that as the background color. If today is Tuesday, the
background color will be green. If the user has specified their age as a string
and we can parse it as a number successfully, we’ll use either purple or orange
depending on the value of the parsed number. Finally, if none of these
conditions apply, the background color will be blue:

Filename: src/main.rs

```
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

Listing 18-1: Mixing `if let`, `else if`, `else if let`, and `else`

This conditional structure lets us support complex requirements. With the
hardcoded values we have here, this example will print `Using purple as the
background color`.

Note that `if let` can also introduce shadowed variables like `match` arms can:
`if let Ok(age) = age` introduces a new shadowed `age` variable that contains
the value inside the `Ok` variant. This also means the `if age > 30` condition
needs to go within the block; we aren’t able to combine these two conditions
into `if let Ok(age) = age && age > 30` since the shadowed `age` that we want
to compare to 30 isn’t valid until the new scope starts with the curly brace.

Also note that conditionals with many cases like these are not as powerful as
`match` expression since exhaustiveness is not checked by the compiler. If we
leave off the last `else` block and miss handling some cases, the compiler will
not error. This example might be too complex to rewrite as a readable `match`,
so we should take extra care to check that we’re handling all the cases since
the compiler is not checking exhaustiveness for us.

### `while let`

A similar construction to `if let` is `while let`: this allows you to do a
`while` loop as long as a pattern continues to match. Listing 18-2 shows an
example using a `while let` loop to use a vector as a stack and print out the
values in the vector in the opposite order that we pushed the values in:

```
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

Listing 18-2: Using a `while let` loop to print out values as long as
`stack.pop()` returns `Some`

This example will print 3, 2, then 1. The `pop` method takes the last element
out of the vector and returns `Some(value)`. If the vector is empty, it returns
`None`. The `while` loop will continue running the code in its block as long as
`pop` is returning `Some`. Once it returns `None`, the `while` loop stops. We
can use `while let` to pop every element off our stack.

### `for` loops

Looping with `for`, as we discussed in Chapter 3, is the most common loop
construction in Rust code. What we didn’t talk about in that chapter was that
`for` takes a pattern. In Listing 18-3, we’re demonstrating how we can use a
pattern in a `for` loop to destructure a tuple. The `enumerate` method adapts
an iterator to produce a value and the index of the value in the iterator in a
tuple:

```
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

Listing 18-3: Using a pattern in a `for` loop to destructure the tuple returned
from `enumerate` into its pieces

This will print:

```
1 is at index 0
2 is at index 1
3 is at index 2
```

The first call to `enumerate` produces the tuple `(0, 1)`. When this value is
matched to the pattern `(index, value)`, `index` will be 0 and `value` will
be 1.

### `let` Statements

`match` and `if let` are the places we’ve explicitly discussed using patterns
earlier in the book, but they aren’t the only places we’ve *used* patterns. For
example, consider this straightforward variable assignment with `let`:

```
let x = 5;
```

We’ve done this hundreds of times throughout this book. You may not have
realized it, but you were using patterns! A `let` statement looks like this,
more formally:

```
let PATTERN = EXPRESSION;
```

We’ve seen statements like `let x = 5;` with a variable name in the `PATTERN`
slot; a variable name is just a particularly humble form of pattern.

With `let`, we compare the expression against the pattern, and assign any names
we find. So for example, in our `let x = 5;` case, `x` is a pattern that says
“bind what matches here to the variable `x`.” And since the name `x` is the
whole pattern, this pattern effectively means “bind everything to the variable
`x`, whatever the value is.”

To see the pattern matching aspect of `let` a bit more clearly, consider
Listing 18-4 where we’re using a pattern with `let` to destructuring a tuple:

```
let (x, y, z) = (1, 2, 3);
```

Listing 18-4: Using a pattern to destructure a tuple and create 3 variables at
once

Here, we have a tuple that we’re matching against a pattern. Rust will compare
the value `(1, 2, 3)` to the pattern `(x, y, z)` and see that the value matches
the pattern. In this case, it will bind `1` to `x`, `2` to `y`, and `3` to `z`.
You can think of this tuple pattern as nesting three individual variable
patterns inside of it.

We saw another example of destructuring a tuple in Chapter 16, Listing 16-6,
where we destructured the return value of `mpsc::channel()` into the `tx`
(transmitter) and `rx` (receiver) parts.

### Function Parameters

Similarly to `let`, function parameters can also be patterns. The code in
Listing 18-5 declaring a function named `foo` that takes one parameter named
`x` of type `i32` should look familiar:

```
fn foo(x: i32) {
    // code goes here
}
```

Listing 18-5: A function signature uses patterns in the parameters

The `x` part is a pattern! In a similar way as we did with `let`, we could
match a tuple in a function’s arguments. Listing 18-6 shows how we could split
apart the values in a tuple as part of passing the tuple to a function:

Filename: src/main.rs

```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

Listing 18-6: A function with parameters that destructure a tuple

This will print `Current location: (3, 5)`. When we pass the value `&(3, 5)` to
`print_coordinates`, the values match the pattern `&(x, y)`. `x` gets the value
3, and `y` gets the value 5.

Because closures are similar to functions, as we discussed in Chapter 13, we
can use patterns in closure parameter lists as well.

One difference between the places we can use patterns is that with `for` loops,
`let`, and in function parameters, the patterns must be *irrefutable*. Let’s
discuss that next.

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

```
let Some(x) = some_option_value;
```

Listing 18-7: Attempting to use a refutable pattern with `let`

If `some_option_value` was a `None` value, `some_option_value` would not match
the pattern `Some(x)`. The pattern `Some(x)` is refutable since there exists a
case in which it would fail to match a value. There’s nothing valid that our
code could do with this `let` statement if `some_option_value` was the `None`
value. Therefore, Rust will complain at compile time that we’ve tried to use a
refutable pattern where an irrefutable pattern is required:

```
error[E0005]: refutable pattern in local binding: `None` not covered
 --> <anon>:3:5
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

We didn’t cover (and couldn’t cover!) every valid value with the pattern
`Some(x)`, so Rust will rightfully complain.

If we have a refutable pattern, instead of using `let`, we can use `if let`.
That way, if the pattern doesn’t match, the code inside the curly braces won’t
execute. That code will only make sense and run if the value matches the
pattern. Listing 18-8 shows how to fix the code in Listing 18-7 with `Some(x)`
matching `some_option_value`. Using the refutable pattern `Some(x)` is allowed,
since this example uses `if let`:

```
# let some_option_value: Option<i32> = None;
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

Listing 18-8: Using `if let` and a block with refutable patterns instead of
`let`

Consequently, if we give `if let` an irrefutable pattern that will always match,
such as `x` as shown in Listing 18-9:

```
if let x = 5 {
    println!("{}", x);
};
```

Listing 18-9: Attempting to use an irrefutable pattern with `if let`

Rust will complain that it doesn’t make sense to use `if let` with an
irrefutable pattern:

```
error[E0162]: irrefutable if-let pattern
 --> <anon>:2:8
  |
2 | if let x = 5 {
  |        ^ irrefutable pattern
```

Generally, match arms use refutable patterns, except for the last arm that
might match any remaining values with an irrefutable pattern. A `match` with
only one arm whose pattern is irrefutable is allowed, but it’s not particularly
useful and could be replaced with a simpler `let` statement.

Now that we’ve discussed all the places that patterns can be used and the
difference between refutable and irrefutable patterns, let’s go over all the
syntax we can use to create patterns.

## All the Pattern Syntax

We’ve seen some examples of different kinds of patterns throughout the book.
This section lists all the syntax valid in patterns and why you might want to
use each of them.

### Literals

As we saw in Chapter 6, you can match against literals directly:

```
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one` since the value in `x` is 1.

### Named Variables

Named variables are irrefutable patterns that match any value.

As with all variables, variables declared as part of a pattern will shadow
variables with the same name outside of the `match` construct since a `match`
starts a new scope. In Listing 18-10, we declare a variable named `x` with the
value `Some(5)` and a variable `y` with the value `10`. Then we have a `match`
expression on the value `x`. Take a look at the patterns in the match arms and
the `println!` at the end, and make a guess about what will be printed before
running this code or reading further:

Filename: src/main.rs

```
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

Listing 18-10: A `match` statement with an arm that introduces a shadowed
variable `y`

Let’s walk through what happens when the `match` statement runs. The first
match arm has the pattern `Some(50)`, and the value in `x` (`Some(5)`) does not
match `Some(50)`, so we continue. In the second match arm, the pattern
`Some(y)` introduces a new variable name `y` that will match any value inside a
`Some` value. Because we’re in a new scope inside the `match` expression, this
is a new variable, not the `y` we declared at the beginning that has the
value 10. The new `y` binding will match any value inside a `Some`, which is
what we have in `x`, so we execute the expression for that arm and print
`Matched, y = 5` since this `y` binds to the inner value of the `Some` in `x`,
which is 5.

If `x` had been a `None` value instead of `Some(5)`, we would have matched the
underscore since the other two arms’ patterns would not have matched. In the
expression for that match arm, since we did not introduce an `x` variable in
the pattern of the arm, this `x` is still the outer `x` that has not been
shadowed. In this hypothetical case, the `match` would print `Default case, x =
None`.

Once the `match` expression is over, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.

In order to make a `match` expression that compares the values of the outer `x`
and `y` rather than introducing a shadowed variable, we would need to use a
match guard conditional instead. We’ll be talking about match guards later in
this section.

### Multiple patterns

In `match` expressions only, you can match multiple patterns with `|`, which
means *or*:

```
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one or two`.

### Matching Ranges of Values with `...`

You can match an inclusive range of values with `...`:

```
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

If `x` is 1, 2, 3, 4, or 5, the first arm will match.

Ranges are only allowed with numeric values or `char` values. Here’s an example
using ranges of `char` values:

```
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

This will print `early ASCII letter`.

### Destructuring to Break Apart Values

Patterns can be used to *destructure* structs, enums, tuples, and references.
Destructuring means to break a value up into its component pieces. Listing
18-11 shows a `Point` struct with two fields, `x` and `y`, that we can break
apart by using a pattern with a `let` statement:

Filename: src/main.rs

```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

Listing 18-11: Destructuring using struct field shorthand

This creates the variables `x` and `y` that match the `x` and `y` of `p`. The
names of the variables must match the names of the fields to use this
shorthand. If we wanted to use names different than the variable names, we can
specify `field_name: variable_name` in the pattern. In Listing 18-12, `a` will
have the value in the `Point` instance’s `x` field and `b` will have the value
in the `y` field:

Filename: src/main.rs

```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

Listing 18-12: Destructuring struct fields into variables with different names
than the fields

We can also use destructuring with literal values in order to test and use
inner parts of a value. Listing 18-13 shows a `match` statement that determines
whether a point lies directly on the `x` axis (which is true when `y = 0`), on
the `y` axis (`x = 0`), or neither:

```
# struct Point {
#     x: i32,
#     y: i32,
# }
#
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

Listing 18-13: Destructuring and matching literal values in one pattern

This will print `On the y axis at 7` since the value `p` matches the second arm
by virtue of `x` having the value 0.

We used destructuring on enums in Chapter 6, such as in Listing 6-5 where we
destructured an `Option<i32>` using a `match` expression and added one to the
inner value of the `Some` variant.

When the value we’re matching against a pattern contains a reference, we can
specify a `&` in the pattern in order to separate the reference and the value.
This is especially useful in closures used with iterators that iterate over
references to values when we want to use the values in the closure rather than
the references. Listing 18-14 shows how to iterate over references to `Point`
instances in a vector, and destructure both the reference and the struct in
order to be able to perform calculations on the `x` and `y` values easily:

```
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];
let sum_of_squares: i32 = points
    .iter()
    .map(|&Point {x, y}| x * x + y * y)
    .sum();
```

Listing 18-14: Destructuring a reference to a struct into the struct field
values

Because `iter` iterates over references to the items in the vector, if we
forgot the `&` in the closure arguments in the `map`, we’d get a type mismatch
error like this:

```
error[E0308]: mismatched types
  -->
   |
14 |         .map(|Point {x, y}| x * x + y * y)
   |               ^^^^^^^^^^^^ expected &Point, found struct `Point`
   |
   = note: expected type `&Point`
              found type `Point`
```

This says Rust was expecting our closure to match `&Point`, but we tried to
match the value with a pattern that was a `Point` value, not a reference to a
`Point`.

We can mix, match, and nest destructuring patterns in even more complex ways:
we can do something complicated like this example where we nest structs and and
tuples inside of a tuple and destructure all the primitive values out:

```
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

This lets us break complex types into their component parts.

### Ignoring Values in a Pattern

There are a few ways to ignore entire values or parts of values: using the `_`
pattern, using the `_` pattern within another pattern, using a name that starts
with an underscore, or using `..` to ignore all remaining parts of a value.
Let’s explore how and why to do each of these.

#### Ignoring an Entire Value with `_`

We’ve seen the use of underscore as a wildcard pattern that will match any value
but not bind to the value. While the underscore pattern is especially useful as
the last arm in a `match` expression, we can use it in any pattern, such as
function arguments as shown in Listing 18-15:

```
fn foo(_: i32) {
    // code goes here
}
```

Listing 18-15: Using `_` in a function signature

Normally, you would change the signature to not have the unused parameter. In
cases such as implementing a trait, where you need a certain type signature,
using an underscore lets you ignore a parameter, and the compiler won’t warn
about unused function parameters like it would if we had used a name instead.

#### Ignoring Parts of a Value with a Nested `_`

We can also use `_` inside of another pattern to ignore just part of a value.
In Listing 18-16, the first `match` arm’s pattern matches a `Some` value but
ignores the value inside of the `Some` variant as specified by the underscore:

```
let x = Some(5);

match x {
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

Listing 18-16: Ignoring the value inside of the `Some` variant by using a
nested underscore

This is useful when the code associated with the `match` arm doesn’t use the
nested part of the variable at all.

We can also use underscores in multiple places within one pattern, as shown in
Listing 18-17 where we’re ignoring the second and fourth values in a tuple of
five items:

```
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

Listing 18-17: Ignoring multiple parts of a tuple

This will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be
ignored.

#### Ignoring an Unused Variable by Starting its Name with an Underscore

Usually, Rust will warn you if you create a variable but don’t use it anywhere,
since that could be a bug. If you’re prototyping or just starting a project,
though, you might create a variable that you’ll use eventually, but temporarily
it will be unused. If you’re in this situation and would like to tell Rust not
to warn you about the unused variable, you can start the name of the variable
with an underscore. This works just like a variable name in any pattern, only
Rust won’t warn you if the variable goes unused. In Listing 18-18, we
do get a warning about not using the variable `y`, but we don’t get a warning
about not using the variable `_x`:

```
fn main() {
    let _x = 5;
    let y = 10;
}
```

Listing 18-18: Starting a variable name with an underscore in order to not get
unused variable warnings

Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore like `_x`: `_x` still binds the value to the
variable, but `_` doesn’t bind at all.

Listing 18-19 shows a case where this distinction matters: `s` will still be
moved into `_s`, which prevents us from using `s` again:

```
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

Listing 18-19: An unused variable starting with an underscore still binds the
value, which may take ownership of the value

Using underscore by itself, however, doesn’t ever bind to the value. Listing
18-20 will compile without any errors since `s` does not get moved into `_`:

```
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

Listing 18-20: Using underscore does not bind the value

This works just fine. Because we never bind `s` to anything, it’s not moved.

#### Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can extract only a few parts and avoid
having to list underscores for each remaining part by instead using `..`. The
`..` pattern will ignore any parts of a value that we haven’t explicitly
matched in the rest of the pattern. In Listing 18-21, we have a `Point` struct
that holds a coordinate in three dimensional space. In the `match` expression,
we only want to operate on the `x` coordinate and ignore the values in the `y`
and `z` fields:

```
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

Listing 18-21: Ignoring all fields of a `Point` except for `x` by using `..`

Using `..` is shorter to type than having to list out `y: _` and `z: _`. The
`..` pattern is especially useful when working with structs that have lots of
fields in situations where only one or two fields are relevant.

`..` will expand to as many values as it needs to be. Listing 18-22 shows a use
of `..` with a tuple:

```
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

Listing 18-22: Matching only the first and last values in a tuple and ignoring
all other values with `..`

Here, we have the first and last value matched, with `first` and `last`. The
`..` will match and ignore all of the things in the middle.

Using `..` must be unambiguous, however. Listing 18-23 shows an example where
it’s not clear to Rust which values we want to match and which values we want
to ignore:

```
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

Listing 18-23: An attempt to use `..` in a way that is ambiguous

If we compile this example, we get this error:

```
error: `..` can only be used once per tuple or tuple struct pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |                      ^^
```

It’s not possible to determine how many values in the tuple should be ignored
before one value is matched with `second`, and then how many further values are
ignored after that. We could mean that we want to ignore 2, bind `second` to 4,
then ignore 8, 16, and 32, or we could mean that we want to ignore 2 and 4,
bind `second` to 8, then ignore 16 and 32, and so forth. The variable name
`second` doesn’t mean anything special to Rust, so we get a compiler error
since using `..` in two places like this is ambiguous.

### `ref` and `ref mut` to Create References in Patterns

Usually, when you match against a pattern, the variables that the pattern
introduces are bound to a value. This means you’ll end up moving the value into
the `match` (or wherever you’re using the pattern) since the ownership rules
apply. Listing 18-24 shows an example:

```
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

Listing 18-24: Creating a variable in a match arm pattern takes ownership of
the value

This example will fail to compile since the value inside the `Some` value in
`robot_name` is moved within the `match` when `name` binds to that value.

Using `&` in a pattern matches an existing reference in the value, as we saw in
the “Destructuring to Break Apart Values” section. If you want to create a
reference instead in order to borrow the value in a pattern variable, use the
`ref` keyword before the new variable, as shown in Listing 18-25:

```
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

Listing 18-25: Creating a reference so that a pattern variable does not take
ownership of a value

This example will compile because the value in the `Some` variant in
`robot_name` is not moved into the `Some(ref name)` arm of the match; the match
only took a reference to the data in `robot_name` rather than moving it.

To create a mutable reference, use `ref mut` for the same reason as shown in
Listing 18-26:

```
let mut robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

Listing 18-26: Creating a mutable reference to a value as part of a pattern
using `ref mut`

This example will compile and print `robot_name is: Some("Another name")`.
Since `name` is a mutable reference, within the match arm code, we need to
dereference using the `*` operator in order to be able to mutate the value.

### Extra Conditionals with Match Guards

You can introduce *match guards* as part of a match arm by specifying an
additional `if` conditional after the pattern. The conditional can use
variables created in the pattern. Listing 18-27 has a `match` expression with a
match guard in the first arm:

```
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

Listing 18-27: Adding a match guard to a pattern

This example will print `less than five: 4`. If `num` was instead `Some(7)`,
this example would print `7`. Match guards allow you to express more complexity
than patterns alone give you.

In Listing 18-10, we saw that since patterns shadow variables, we weren’t able
to specify a pattern to express the case when a value was equal to a variable
outside the `match`. Listing 18-28 shows how we can use a match guard to
accomplish this:

```
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

Listing 18-28: Using a match guard to test for equality with an outer variable

This will now print `Default case, x = Some(5)`. Because the second match arm
is not introducing a new variable `y` that shadows the outer `y` in the
pattern, we can use `y` in the match guard. We’re still destructuring `x` to
get the inner value `n`, and then we can compare `n` and `y` in the match guard.

If you’re using a match guard with multiple patterns specified by `|`, the
match guard condition applies to all of the patterns. Listing 18-29 shows a
match guard that applies to the value matched by all three patterns in the
first arm:

```
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

Listing 18-29: Combining multiple patterns with a match guard

This prints `no` since the `if` condition applies to the whole pattern `4 | 5 |
6`, not only to the last value `6`. In other words, the precedence of a match
guard in relation to a pattern behaves like this:

```
(4 | 5 | 6) if y => ...
```

rather than this:

```
4 | 5 | (6 if y) => ...
```

### `@` Bindings

In order to test a value in a pattern but also be able to create a variable
bound to the value, we can use `@`. Listing 18-30 shows an example where we
want to test that a `Message::Hello` `id` field is within the range `3...7` but
also be able to bind to the value so that we can use it in the code associated
with the arm:

```
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id @ 3...7 } => {
        println!("Found an id in range: {}", id)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

Listing 18-30: Using `@` to bind to a value in a pattern while also testing it

This example will print `Found an id in range: 5`. By specifying `id @` before
the range, we’re capturing whatever value matched the range while also testing
it. In the second arm where we only have a range specified in the pattern, the
code associated with the arm doesn’t know if `id` is 10, 11, or 12, since we
haven’t saved the `id` value in a variable: we only know that the value matched
something in that range if that arm’s code is executed. In the last arm where
we’ve specified a variable without a range, we do have the value available to
use in the arm’s code, but we haven’t applied any other test to the value.
Using `@` lets us test a value and save it in a variable within one pattern.

## Summary

Patterns are a useful feature of Rust that help to distinguish between
different kinds of data. When used in `match` statements, Rust makes sure that
your patterns cover every possible value. Patterns in `let` statements and
function parameters make those constructs more powerful, enabling the
destructuring of values into smaller parts at the same time as assigning to
variables.

Now, for the penultimate chapter of the book, let’s take a look at some
advanced parts of a variety of Rust’s features.
