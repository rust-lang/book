## All the Pattern Syntax

We’ve seen some examples of different kinds of patterns throughout the book.
This section lists all the syntax valid in patterns and why you might want to
use each of them.

### Literals

As we saw in Chapter 6, you can match against literals directly:

```rust
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

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 18-10: A `match` statement with an arm that
introduces a shadowed variable `y`</span>

<!-- NEXT PARAGRAPH WRAPPED WEIRD INTENTIONALLY SEE #199 -->

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

```rust
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

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

If `x` is 1, 2, 3, 4, or 5, the first arm will match.

Ranges are only allowed with numeric values or `char` values. Here’s an example
using ranges of `char` values:

```rust
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

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 18-11: Destructuring using struct field
shorthand</span>

This creates the variables `x` and `y` that match the `x` and `y` of `p`. The
names of the variables must match the names of the fields to use this
shorthand. If we wanted to use names different than the variable names, we can
specify `field_name: variable_name` in the pattern. In Listing 18-12, `a` will
have the value in the `Point` instance’s `x` field and `b` will have the value
in the `y` field:

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 18-12: Destructuring struct fields into variables
with different names than the fields</span>

We can also use destructuring with literal values in order to test and use
inner parts of a value. Listing 18-13 shows a `match` statement that determines
whether a point lies directly on the `x` axis (which is true when `y = 0`), on
the `y` axis (`x = 0`), or neither:

```rust
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

<span class="caption">Listing 18-13: Destructuring and matching literal values
in one pattern</span>

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

```rust
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

<span class="caption">Listing 18-14: Destructuring a reference to a struct into
the struct field values</span>

Because `iter` iterates over references to the items in the vector, if we
forgot the `&` in the closure arguments in the `map`, we’d get a type mismatch
error like this:

```text
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
we can do something complicated like this example where we nest structs and 
tuples inside of a tuple and destructure all the primitive values out:

```rust
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

```rust
fn foo(_: i32) {
    // code goes here
}
```

<span class="caption">Listing 18-15: Using `_` in a function signature</span>

Normally, you would change the signature to not have the unused parameter. In
cases such as implementing a trait, where you need a certain type signature,
using an underscore lets you ignore a parameter, and the compiler won’t warn
about unused function parameters like it would if we had used a name instead.

#### Ignoring Parts of a Value with a Nested `_`

We can also use `_` inside of another pattern to ignore just part of a value.
In Listing 18-16, the first `match` arm’s pattern matches a `Some` value but
ignores the value inside of the `Some` variant as specified by the underscore:

```rust
let x = Some(5);

match x {
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

<span class="caption">Listing 18-16: Ignoring the value inside of the `Some`
variant by using a nested underscore</span>

This is useful when the code associated with the `match` arm doesn’t use the
nested part of the variable at all.

We can also use underscores in multiple places within one pattern, as shown in
Listing 18-17 where we’re ignoring the second and fourth values in a tuple of
five items:

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

<span class="caption">Listing 18-17: Ignoring multiple parts of a tuple</span>

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

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

<span class="caption">Listing 18-18: Starting a variable name with an underscore
in order to not get unused variable warnings</span>

Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore like `_x`: `_x` still binds the value to the
variable, but `_` doesn’t bind at all.

Listing 18-19 shows a case where this distinction matters: `s` will still be
moved into `_s`, which prevents us from using `s` again:

```rust,ignore
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">Listing 18-19: An unused variable starting with an
underscore still binds the value, which may take ownership of the value</span>

Using underscore by itself, however, doesn’t ever bind to the value. Listing
18-20 will compile without any errors since `s` does not get moved into `_`:

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">Listing 18-20: Using underscore does not bind the
value</span>

This works just fine. Because we never bind `s` to anything, it’s not moved.

#### Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can extract only a few parts and avoid
having to list underscores for each remaining part by instead using `..`. The
`..` pattern will ignore any parts of a value that we haven’t explicitly
matched in the rest of the pattern. In Listing 18-21, we have a `Point` struct
that holds a coordinate in three dimensional space. In the `match` expression,
we only want to operate on the `x` coordinate and ignore the values in the `y`
and `z` fields:

```rust
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

<span class="caption">Listing 18-21: Ignoring all fields of a `Point` except
for `x` by using `..`</span>

Using `..` is shorter to type than having to list out `y: _` and `z: _`. The
`..` pattern is especially useful when working with structs that have lots of
fields in situations where only one or two fields are relevant.

`..` will expand to as many values as it needs to be. Listing 18-22 shows a use
of `..` with a tuple:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

<span class="caption">Listing 18-22: Matching only the first and last values in
a tuple and ignoring all other values with `..`</span>

Here, we have the first and last value matched, with `first` and `last`. The
`..` will match and ignore all of the things in the middle.

Using `..` must be unambiguous, however. Listing 18-23 shows an example where
it’s not clear to Rust which values we want to match and which values we want
to ignore:

```rust,ignore
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

<span class="caption">Listing 18-23: An attempt to use `..` in a way that is
ambiguous</span>

If we compile this example, we get this error:

```text
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

```rust,ignore
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-24: Creating a variable in a match arm pattern
takes ownership of the value</span>

This example will fail to compile since the value inside the `Some` value in
`robot_name` is moved within the `match` when `name` binds to that value.

Using `&` in a pattern matches an existing reference in the value, as we saw in
the “Destructuring to Break Apart Values” section. If you want to create a
reference instead in order to borrow the value in a pattern variable, use the
`ref` keyword before the new variable, as shown in Listing 18-25:

```rust
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-25: Creating a reference so that a pattern
variable does not take ownership of a value</span>

This example will compile because the value in the `Some` variant in
`robot_name` is not moved into the `Some(ref name)` arm of the match; the match
only took a reference to the data in `robot_name` rather than moving it.

To create a mutable reference, use `ref mut` for the same reason as shown in
Listing 18-26:

```rust
let mut robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-26: Creating a mutable reference to a value as
part of a pattern using `ref mut`</span>

This example will compile and print `robot_name is: Some("Another name")`.
Since `name` is a mutable reference, within the match arm code, we need to
dereference using the `*` operator in order to be able to mutate the value.

### Extra Conditionals with Match Guards

You can introduce *match guards* as part of a match arm by specifying an
additional `if` conditional after the pattern. The conditional can use
variables created in the pattern. Listing 18-27 has a `match` expression with a
match guard in the first arm:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

<span class="caption">Listing 18-27: Adding a match guard to a pattern</span>

This example will print `less than five: 4`. If `num` was instead `Some(7)`,
this example would print `7`. Match guards allow you to express more complexity
than patterns alone give you.

In Listing 18-10, we saw that since patterns shadow variables, we weren’t able
to specify a pattern to express the case when a value was equal to a variable
outside the `match`. Listing 18-28 shows how we can use a match guard to
accomplish this:

```rust
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

<span class="caption">Listing 18-28: Using a match guard to test for equality
with an outer variable</span>

This will now print `Default case, x = Some(5)`. Because the second match arm
is not introducing a new variable `y` that shadows the outer `y` in the
pattern, we can use `y` in the match guard. We’re still destructuring `x` to
get the inner value `n`, and then we can compare `n` and `y` in the match guard.

If you’re using a match guard with multiple patterns specified by `|`, the
match guard condition applies to all of the patterns. Listing 18-29 shows a
match guard that applies to the value matched by all three patterns in the
first arm:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

<span class="caption">Listing 18-29: Combining multiple patterns with a match
guard</span>

This prints `no` since the `if` condition applies to the whole pattern `4 | 5 |
6`, not only to the last value `6`. In other words, the precedence of a match
guard in relation to a pattern behaves like this:

```text
(4 | 5 | 6) if y => ...
```

rather than this:

```text
4 | 5 | (6 if y) => ...
```

### `@` Bindings

In order to test a value in a pattern but also be able to create a variable
bound to the value, we can use `@`. Listing 18-30 shows an example where we
want to test that a `Message::Hello` `id` field is within the range `3...7` but
also be able to bind to the value so that we can use it in the code associated
with the arm:

```rust
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

<span class="caption">Listing 18-30: Using `@` to bind to a value in a pattern
while also testing it</span>

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
