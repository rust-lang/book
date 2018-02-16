
[TOC]

# Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with `match`
expressions and other constructs gives you more control over the control flow
of a program. A pattern is made up of some combination of:

- literals
- destructured arrays, enums, structs, or tuples
- variables
- wildcards
- placeholders

These pieces describe the shape of the data we’re working with, which we then
match against values to determine whether our program has the correct data to
continue running a particular bit of code.

<!-- I think we need a concise description of what we use patterns for here,
what they provide the programmer. Hopefully you can see what I've trying to do,
above! But I think you'll agree it's not quite right, can you have a whack, try
to give the reader that explanation? -->
<!-- We tweaked the wording a bit, how's this? /Carol -->

To use a pattern we compare it to some value. If the pattern matches our value,
we use the value parts in our code. Recall our `match` expressions from Chapter
6 that used patterns like a coin sorting machine. If the value fits the shape
of the pattern, we can use the named pieces. If it doesn’t, the code associated
with the pattern won’t run.

This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between *refutable* and
*irrefutable* patterns, and the different kinds of pattern syntax that you
might see. By the end, you’ll see how to use patterns to create powerful and
clear code.

## All the Places Patterns May be Used

Patterns pop up in a number of places in Rust, and you’ve been using them a lot
without realizing it! This section is a reference to all the places where
patterns are valid.

### `match` Arms

As we discussed in Chapter 6, patterns are used in the arms of `match`
expressions. Formally, `match` expressions are defined as the keyword `match`,
a value to match on, and one or more match arms that consist of a pattern and
an expression to run if the value matches that arm’s pattern:

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

`match` expressions are required to be *exhaustive*, in the sense that all
possibilities for the value in the `match` expression must be accounted for.
One way to ensure you have every possibility covered is to have a catch-all
pattern for the last arm---for example, a variable name matching any value can
never fail and thus covers every case remaining.

There’s a particular pattern `_` that will match anything, but never binds to a
variable, and so is often used in the last match arm. This can be useful when
you want to ignore any value not specified, for example. We’ll cover this in
more detail later in this chapter.

### Conditional `if let` Expressions

In Chapter 6 we discussed how `if let` expressions are used mainly as a shorter
way to write the equivalent of a `match` that only cares about matching one
case. Optionally,`if let` can have a corresponding `else` with code to run if
the pattern in the `if let` doesn’t match.

<!-- Can you say up front why we'd use this, and not just a match? I've just
added something here, not sure if it's right -->
<!-- The first sentence says why-- it's a shorter way to write a `match` when
there's only one case we care about. Can you elaborate on why that's not clear
or up front? /Carol -->

Listing 18-1 shows that it’s also possible to mix and match `if let`, `else
if`, and `else if let` expressions. This gives us more flexibility than a
`match` expression where we can only express one value to compare with the
patterns; the conditions in a series of `if let`/`else if`/`else if let` arms
aren’t required to have any relation to each other.

The code in Listing 18-1 shows a series of checks for a bunch of different
conditions that decide what the background color should be. For the purposes of
the example, we’ve created variables with hardcoded values that a real program
might get by asking the user.

If the user has specified a favorite color, that is used as the background
color. If today is Tuesday, the background color will be green. If the user has
specified their age as a string and we can parse it as a number successfully,
we’ll use either purple or orange depending on the value of the parsed number.
Finally, if none of these conditions apply, the background color will be blue:

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

We can see that `if let` can also introduce shadowed variables, in the same way
that `match` arms can: `if let Ok(age) = age` introduces a new shadowed `age`
variable that contains the value inside the `Ok` variant. This means we need to
place the `if age > 30` condition within that block; we can’t combine these two
conditions into `if let Ok(age) = age && age > 30` because the shadowed `age`
we want to compare to 30 isn’t valid until the new scope starts with the curly
brace.

The downside of using `if let` expressions in this way is that exhaustiveness
is not checked by the compiler, whereas with `match` expressions it is. If we
left off the last `else` block and so missed handling some cases, the compiler
would not alert us of the possible logic bug.

<!-- So what would happen, we'd just end up with a program that wasn't correct,
in the Rust sense? -->
<!-- Yes, we would have a logic bug. /Carol -->

### `while let` Conditional Loops

Similar in construction to `if let`, the `while let` conditional loop allows
your `while` loop to run for as long as a pattern continues to match. The
example in Listing 18-2 shows a `while let` loop that uses a vector as a stack
and prints out the values in the vector in the opposite order they were pushed
in:

```
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

Listing 18-2: Using a `while let` loop to print out values for as long as
`stack.pop()` returns `Some`

<!-- Some lovely simple, but edifying, examples in this chapter!-->

This example will print 3, 2, then 1. The `pop` method takes the last element
out of the vector and returns `Some(value)`. If the vector is empty, it returns
`None`. The `while` loop will continue running the code in its block as long as
`pop` is returning `Some`. Once it returns `None`, the loop stops. We can use
`while let` to pop every element off our stack.

### `for` Loops

In Chapter 3 we mentioned that the `for` loop is the most common loop
construction in Rust code, but we haven’t yet discussed the pattern that `for`
takes. In a `for` loop, the pattern is the value that directly follows the
keyword `for`, so the `x` in `for x in y`.

<!-- Can you check the line I added above? I think it'd help to point out the
pattern section of a for loop straight away -->
<!-- Yep, looks good! /Carol -->

Listing 18-3 demonstrates how to use a pattern in a `for` loop to destructure,
or break apart, a tuple as part of the `for` loop:

<!-- Liz: We've been using the word "destructure" throughout the book in
chapters 3, 4, 5, and 16. In chapter 3, in the "Grouping Values into Tuples"
section, we said "This is called *destructuring*, because it breaks the single
tuple into three parts.". So I don't think we need to define destructure again
in this chapter, but I've added a small parenthetical here in case the reader
forgets. /Carol -->

```
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

Listing 18-3: Using a pattern in a `for` loop to destructure a tuple

This will print:

```
1 is at index 0
2 is at index 1
3 is at index 2
```

We use the `enumerate` method to adapt an iterator to produce a value and that
value’s index in the iterator, placed into a tuple. The first call to
`enumerate` produces the tuple `(0, 1)`. When this value is matched to the
pattern `(index, value)`, `index` will be 0 and `value` will be 1, printing our
first line of output.

### `let` Statements

Before this chapter, we’d only explicitly discussed using patterns with `match`
and `if let`, but in fact we’ve used patterns in other places too, including
`let` statements. For example, consider this straightforward variable
assignment with `let`:

```
let x = 5;
```

We’ve done this hundreds of times throughout this book, and though you may not
have realized it, you were using patterns! A `let` statement looks like this,
more formally:

```
let PATTERN = EXPRESSION;
```

In statements like `let x = 5;` with a variable name in the `PATTERN` slot, the
variable name is just a particularly humble form of pattern. We compare the
expression against the pattern, and assign any names we find. So for our `let x
= 5;` example, `x` is a pattern that says “bind what matches here to the
variable `x`.” And since the name `x` is the whole pattern, this pattern
effectively means “bind everything to the variable `x`, whatever the value is.”

To see the pattern matching aspect of `let` a bit more clearly, consider
Listing 18-4 where we’re using a pattern with `let` to destructure a tuple:

```
let (x, y, z) = (1, 2, 3);
```

Listing 18-4: Using a pattern to destructure a tuple and create three variables
at once

Here, we match a tuple against a pattern. Rust compares the value `(1, 2, 3)`
to the pattern `(x, y, z)` and sees that the value matches the pattern, so will
bind `1` to `x`, `2` to `y`, and `3` to `z`. You can think of this tuple
pattern as nesting three individual variable patterns inside of it.

<!-- so if we have a pattern of four elements, say (w, x, y, z), but only three
values, would the values would not bind at all? -->
<!-- Either too many or too few elements in the pattern is a type error. I've
added a small example below to illustrate. /Carol -->

If the number of elements in the pattern don’t match the number of elements in
the tuple, the overall type won’t match and we’ll get a compiler error. For
example, Listing 18-5 shows an attempt to destructure into two variables a
tuple with three elements that won’t work:

```
let (x, y) = (1, 2, 3);
```

Listing 18-5: Incorrectly constructing a pattern whose variables don’t match
the number of elements in the tuple

Attempting to compile this code gives us this type error:

```
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^ expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected type `({integer}, {integer}, {integer})`
             found type `(_, _)`
```

If we wanted to ignore one or more of the values in the tuple, we could use `_`
or `..` as we’ll see in the “Ignoring Values in a Pattern” section. If the
problem was that we had too many variables in the pattern, the solution would
be to make the types match by removing variables so that the number of
variables is equal to the number of elements in the tuple.

### Function Parameters

Function parameters can also be patterns. The code in Listing 18-6, declaring a
function named `foo` that takes one parameter named `x` of type `i32`, should
by now look familiar:

```
fn foo(x: i32) {
    // code goes here
}
```

Listing 18-6: A function signature uses patterns in the parameters

The `x` part is a pattern! Like we did with `let`, we could match a tuple in a
function’s arguments to the pattern. Listing 18-7 splits apart the values in a
tuple as we pass it to a function:

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

Listing 18-7: A function with parameters that destructure a tuple

This will print `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` gets the value 3, and `y` gets the value 5.

We can use patterns in closure parameter lists in the same way, too, because
closures are similar to functions, as we discussed in Chapter 13.

We’ve seen several ways of using patterns now, but patterns do not work the
same in every place we can use them; in some places, the patterns must be
*irrefutable*, meaning they must match any value provided. In other
circumstances, they may be refutable. Let’s discuss that next.

## Refutability: Whether a Pattern Might Fail to Match

Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are said to be *irrefutable*. An example would be
`x` in the statement `let x = 5;` because `x` matches anything and so cannot
fail to match. Patterns that may fail to match for some possible value are said
to be *refutable*. An example of this would be `Some(x)` in the expression `if
let Some(x) = a_value`; if the value in the `a_value` variable is `None` rather
than `Some`, then the `Some(x)` pattern would not match.

`let` statements, function parameters, and `for` loops can only accept
irrefutable patterns, because the program cannot continue do anything
meaningful with values that don’t match. The `if let` and `while let`
expressions are restricted to only accept refutable patterns, because by
definition they’re intended to handle possible failure---the functionality of a
conditional is in its ability to perform differently upon success and failure.

In general, you shouldn’t have to worry about the distinction between refutable
and irrefutable patterns, but you do need to be familiar with the concept of
refutability so you can respond when you see it in an error message. In those
cases, you’ll need to change either the pattern or the construct you’re using
the pattern with, depending on your intentions for the behavior of the code.

Let’s look at an example of what happens if we try to use a refutable pattern
where Rust requires an irrefutable pattern and vice versa. In Listing 18-8, we
have a `let` statement, but for the pattern we’ve specified `Some(x)`, a
refutable pattern. As you might expect, this will error:

```
let Some(x) = some_option_value;
```

Listing 18-8: Attempting to use a refutable pattern with `let`

If `some_option_value` was a `None` value, it would fail to match the pattern
`Some(x)`, meaning the pattern is refutable. The `let` statement, however, can
only accept an irrefutable patterns because there’s nothing valid the code
could do with a `None` value. At compile time, Rust will complain that we’ve
tried to use a refutable pattern where an irrefutable pattern is required:

```
error[E0005]: refutable pattern in local binding: `None` not covered
 --> <anon>:3:5
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

We didn’t cover (and couldn’t cover!) every valid value with the pattern
`Some(x)`, so Rust will rightfully complain.

To fix the case where we have a refutable pattern in a place where an
irrefutable pattern is needed, we can change the code that uses the pattern:
instead of using `let`, we can use `if let`. That way, if the pattern doesn’t
match, the code will just skip the code in the curly brackets, giving it a way
to continue validly. Listing 18-9 shows how to fix the code in Listing 18-8.

```
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

Listing 18-9: Using `if let` and a block with refutable patterns instead of
`let`

<!-- Whats the first commented out line here, I had though this was copied from
8-7 but it isn't quite the same -->
<!-- Sorry, that line has to do with the way we test our code examples and I
missed removing it before sending this chapter to you. Sorry about that! /Carol
-->

We’ve given the code an out! This code is perfectly valid, though does now of
course mean we cannot use an irrefutable pattern without receiving an error. If
we give `if let` a pattern that will always match, such as `x` as shown in
Listing 18-10, it will error:

```
if let x = 5 {
    println!("{}", x);
};
```

Listing 18-10: Attempting to use an irrefutable pattern with `if let`

Rust complains that it doesn’t make sense to use `if let` with an irrefutable
pattern:

```
error[E0162]: irrefutable if-let pattern
 --> <anon>:2:8
  |
2 | if let x = 5 {
  |        ^ irrefutable pattern
```

For this reason, match arms must use refutable patterns, except for the last
arm that should match any remaining values with an irrefutable pattern. Using
an irrefutable pattern in a `match` with only one arm is allowed, but isn’t
particularly useful and could be replaced with a simpler `let` statement.

Now that we’ve discussed where patterns can be used and the difference between
refutable and irrefutable patterns, let’s go over all the syntax we can use to
create patterns.

## All the Pattern Syntax

We’ve seen examples of many different kinds of patterns throughout the book, so
we’ll gather all the syntax valid in patterns in one place here, and why you
might want to use each of them.

<!-- We don't always go over why we might want to use them for each section
here, presumably because it's clear why it's useful. I might recommend you do
just add a line to each, since we've promised it, and just to really hammer the
point home. Definitely keep it short and sweet though, where it's pretty clear.
-->

### Matching Literals

As we saw in Chapter 6, you can match patterns against literals directly. This
following code gives some examples:

```
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This prints `one` since the value in `x` is 1. This is useful when you want to
take some action if you get a concrete value in particular.

### Matching Named Variables

<!-- I found this next bit a little tougher to follow, I've tried to clarify in
this opening paragraph, connect it all up, can you please check it? -->
<!-- Yep! Looks good! /Carol -->

Named variables are irrefutable patterns that match any value, which we have
used many times before. There is a complication, however, when used in `match`
expressions. Because `match` starts a new scope, variables declared as part of
a pattern inside the `match` expression will shadow those with the same name
outside the `match` construct---as is the case with all variables. In Listing
18-11, we declare a variable named `x` with the value `Some(5)` and a variable
`y` with the value `10`. We then create a `match` expression on the value `x`.
Take a look at the patterns in the match arms and `println!` at the end, and
try to figure out what will be printed before running this code or reading
further:

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

Listing 18-11: A `match` statement with an arm that introduces a shadowed
variable `y`

Let’s walk through what happens when the `match` statement runs. The pattern in
the first match arm does not match the defined value of `x`, so we continue.

The pattern in the second match arm introduces a new variable name `y` that
will match any value inside a `Some` value. Because we’re in a new scope inside
the `match` expression, this is a new variable, and not the `y` we declared at
the beginning with the value 10. This new `y` binding will match any value
inside a `Some`, which is what we have in `x`. Therefore this `y` binds to the
inner value of the `Some` in `x`. That value is 5, and so the expression for
that arm executes and prints `Matched, y = 5`.

<!-- Below -- We haven't fully introduced the underscore yet, is there anything
else we could use for that final arm? -->
<!-- We have *used* the underscore briefly before, though-- we actually
introduced the underscore in chapter 6. There really isn't anything else that
we can put that will still have this example illustrating what we want to
illustrate. /Carol -->

If `x` had been a `None` value instead of `Some(5)`, the patterns in the first
two arms would not have matched, so we would have matched to the underscore. We
did not introduce the `x` variable in the pattern of that arm, so the `x` in
the expression is still the outer `x` that has not been shadowed. In this
hypothetical case, the `match` would print `Default case, x = None`.

Once the `match` expression is over, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.

To create a `match` expression that compares the values of the outer `x` and
`y`, rather than introducing a shadowed variable, we would need to use a match
guard conditional instead. We’ll be talking about match guards later in this
section.

### Multiple Patterns

In `match` expressions you can match multiple patterns using the `|` syntax,
which means *or*. For example, the following code matches the value of `x`
against the match arms, the first of which has an *or* option, meaning if the
value of `x` matches either of the values in that arm, it will run:

<!-- I've tried to flesh this out a bit, can you check? -->
<!-- Yep, it's fine! /Carol -->

```
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This code will print `one or two`.

<!-- Is there a corresponding "and" operator? Is that worth tacking on here? -->
<!-- No, there is not-- how could one value match, say, 1 AND 2? Does it make
sense why there isn't an "and" operator? /Carol -->

### Matching Ranges of Values with `...`

The `...` syntax allows you to match to an inclusive range of values. In the
following code, when a pattern matches any of the values within the range,
that arm will execute:

<!-- Above--this seems like it's true, that the range allows you to match to
just one of the values? If so, can you say how this differs to using the or
operator? -->
<!-- I'm not sure what you mean by "match to just one of the values". `...`
matches any value between the two specified endpoints, which I thought would be
clear by the text below the code, and I changed "just one of" to "any of the
values within" above, and mentioned what the equivalent "or" pattern would look
like below. Does that clear it up? /Carol -->

```
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

If `x` is 1, 2, 3, 4, or 5, the first arm will match. This is more convenient
than using the `|` operator to express the same idea; instead of `1 ... 5` we
would have to specify `1 | 2 | 3 | 4 | 5` using `|`. Specifying a range instead
is much shorter, especially if we wanted to match, say, any number between 1
and 1,000!

Ranges are only allowed with numeric values or `char` values, because the
compiler checks that the range isn’t empty at compile time. `char` and numeric
values are the only types that Rust knows how to tell if a range is empty or
not.

<!-- why, because they are the only types with inherent order? -->
<!-- Nope, I've added the explanation /Carol -->

Here’s an example using ranges of `char` values:

```
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

Rust can tell that `c` is within the first pattern’s range, and this will print
`early ASCII letter`.

### Destructuring to Break Apart Values

<!-- I moved the definition of destructure earlier in the chapter, to when we
first use it -->
<!-- See my comment there; we first use destructure in chapter 3 /Carol -->

We can also use patterns to destructure structs, enums, tuples, and references
in order to use different parts of these values. Let’s go through each of those!

<!-- Above -- I think that's what we say later, that, we're skipping enums, but
please delete if not! You'll see my note where it comes up later -->
<!-- We actually had someone point out a detail we could cover regarding enums,
so we've added an enums section. /Carol -->

#### Destructuring Structs

Listing 18-12 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement:

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

Listing 18-12: Destructuring a struct’s fields into separate variables

<!-- I'm not sure I follow which part of this is the shorthand, what is it
shorthand for, and which syntax here counts as the shorthand? Can you slow this
down, talk it through a little more. Is the point of this section that we have
a shorthand for destructuring, or that we are able to destructure these items
with patterns at all? -->
<!-- I've reorganized this section to start with the non-shorthand instead, is
this clearer? /Carol -->

This code creates the variables `a` and `b` that match the values of the `x`
and `y` fields of the `p` variable.

This example shows that the names of the variable names in the pattern don’t
have to match the field names of the struct, but it’s common to want the
variable names to match the field names to make it easier to remember which
variables came from which fields. Because having variable names match the
fields is common, and because writing `let Point { x: x, y: y } = p;` contains
a lot of duplication, there’s a shorthand for patterns that match struct
fields: you only need to list the name of the struct field, and the variables
created from the pattern will have the same names. Listing 18-13 shows code
that behaves in the same way as the code in Listing 18-12, but the variables
created in the `let` pattern are `x` and `y` instead of `a` and `b`:

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

Listing 18-13: Destructuring struct fields using struct field shorthand

This code creates the variables `x` and `y` that match the `x` and `y` of the
`p` variable. The outcome is that the variables `x` and `y` contain the values
from the `p` struct.

We can also destructure with literal values as part of the struct pattern
rather than creating variables for all of the fields. This allows us to test
some of the fields for particular values while creating variables to
destructure the other fields.

Listing 18-14 shows a `match` statement that separates `Point` values into
three cases: points that lie directly on the `x` axis (which is true when `y =
0`), on the `y` axis (`x = 0`), or neither:

<!-- I'm not sure what you mean by "inner parts of a value" -- that we aren't
matching a whole value but part of it? -->
<!-- I've reworded, is this version clearer? /Carol -->

Filename: src/main.rs

```
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

Listing 18-14: Destructuring and matching literal values in one pattern

The first arm will match any point that lies on the `x` axis by specifying that
the `y` field matches if its value matches the literal `0`. The pattern still
creates an `x` variable that we can use in the code for this arm. Similarly,
the second arm matches any point on the `y` axis by specifying that the `x`
field matches if its value is `0`, and creates a variable `y` for the value of
the `y` field. The third arm doesn’t specify any literals, so it matches any
other `Point` and creates variables for both the `x` and `y` fields.

In this example, the value `p` matches the second arm by virtue of `x`
containing a 0, so this will print `On the y axis at 7`.

#### Destructuring Enums

We’ve destructured enums before in this book, like in Listing 6-5 in Chapter 6
when we destructured an `Option<i32>`. One detail we haven’t mentioned
explicitly is that the pattern to destructure an enum should correspond to the
way the data stored within the enum is defined. For example, let’s take the
`Message` enum from Listing 6-2 and write a `match` with patterns that will
destructure each inner value in Listing 18-15:

Filename: src/main.rs

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x: x, y: y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}
```

Listing 18-15: Destructuring enum variants that hold different kinds of values

This code will print `Change the color to red 0, green 160, and blue 255`. Try
changing the value of `msg` to see the code from the other arms run.

For enum variants without any data like `Message::Quit`, we can’t destructure
the value any further. We can only match on the literal `Message::Quit` value,
and there are no variables in that pattern.

For struct-like enum variants such as `Message::Move`, we can use a pattern
similar to the pattern we specify to match structs. After the variant name, we
place curly brackets and then list the fields with variables so that we break
apart the pieces to use in the code for this arm.

For tuple-like enum variants like `Message::Write`, that holds a tuple with one
element, and `Message::ChangeColor` that holds a tuple with three elements, the
pattern is similar to the pattern we specify to match tuples. The number of
variables in the pattern must match the number of elements in the variant we’re
matching.

#### Destructuring References

When the value we’re matching to our pattern contains a reference, we need to
destructure the reference from the value, which we can do can by specifying a
`&` in the pattern. This lets us get a variable holding the value that the
reference points to rather than getting a variable that holds the reference.

<!-- What does it mean, to separate the reference and the value, precisely? So
that we specify Rust use the value in place of the reference? And what does &
here do, tell Rust to follow the reference to the value itself, rather than
work on the reference?-->
<!-- Yes, pretty much. I've tried rewording, is this clearer? /Carol -->

This is especially useful in closures where we have iterators that iterate over
references, but we want to use the values in the closure rather than the
references.

The example in Listing 18-16 iterates over references to `Point` instances in a
vector, and destructures both the reference and the struct so we can perform
calculations on the `x` and `y` values easily:

```
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];
let sum_of_squares: i32 = points
    .iter()
    .map(|&Point { x, y }| x * x + y * y)
    .sum();
```

Listing 18-16: Destructuring a reference to a struct into the struct field
values

<!-- and what do we actually get, instead of the error? -->
<!-- Added explanation text below /Carol -->

This code results in the value 135 in the variable `sum_of_squares`, which is
the result from squaring the `x` value and the `y` value, adding those
together, and then adding the result for each `Point` in the `points` vector to
get one number.

If we had not included the `&` in `&Point { x, y }` we’d get a type mismatch
error, because `iter` would then iterate over references to the items in the
vector rather than the values themselves. The error would look like this:

```
error[E0308]: mismatched types
  -->
   |
14 |         .map(|Point { x, y }| x * x + y * y)
   |               ^^^^^^^^^^^^ expected &Point, found struct `Point`
   |
   = note: expected type `&Point`
              found type `Point`
```

This tells us that Rust was expecting our closure to match `&Point`, but we
tried to match directly to a `Point` value, and not a reference to a `Point`.

#### Destructuring Structs and Tuples

We can mix, match, and nest destructuring patterns in even more complex way.
Here’s an example of a complicated destructure, where we nest structs and
tuples inside a tuple, and destructure all the primitive values out:

```
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

This lets us break complex types into their component parts so that we can use
the values we’re interested in separately.

<!-- Can you round up the destructuring section here before we move on. For
this bit, maybe say explicitly what this would be useful for -->
<!-- Done /Carol -->

Destructuring with patterns is a convenient way to use pieces of values, such
as the value from each field in a struct, separately from each other.

### Ignoring Values in a Pattern

We’ve seen that it’s sometimes useful to ignore values in a pattern, such as in
the last arm of a `match` to give us a catch-all that doesn’t actually do
anything, but does account for all remaining possible values. There are a few
ways to ignore entire values or parts of values in a pattern: using the `_`
pattern (which we’ve seen), using the `_` pattern within another pattern, using
a name that starts with an underscore, or using `..` to ignore remaining parts
of a value. Let’s explore how and why to do each of these.

#### Ignoring an Entire Value with `_`

We’ve used the underscore as a wildcard pattern that will match any value but
not bind to the value. While the underscore pattern is especially useful as the
last arm in a `match` expression, we can use it in any pattern, including
function parameters, as shown in Listing 18-17:

Filename: src/main.rs

```
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

Listing 18-17: Using `_` in a function signature

<!-- What is this doing exactly, can you help the reader out here? Are we
letting the function run without a parameter at all? I'm not sure the purpose
clear enough at the moment -->
<!-- Done /Carol -->

This code will completely ignore the value passed as the first argument, 3, and
will print out `This code only uses the y parameter: 4`. In most cases when you
no longer need a particular function parameter, you would change the signature
so it doesn’t include the unused parameter.

Ignoring a function parameter can be especially useful in some cases, such as
when implementing a trait, when you need a certain type signature but the
function body in your implementation doesn’t need one of the parameters. The
compiler will then not warn about unused function parameters, as it would if we
used a name instead.

#### Ignoring Parts of a Value with a Nested `_`

<!-- When would we want to do this? -->
<!-- Done, moved the explanation up and made the example have a bit more
motivation /Carol -->

We can also use `_` inside of another pattern to ignore just part of a value,
when we only want to test for part of a value but have no use for the other
parts in the corresponding code we want to run. Listing 18-18 shows code
responsible for giving a setting a value. The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting, but can unset the setting and can give the setting a value if it is
currently unset.

```
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
```

Listing 18-18: Using an underscore within patterns that match `Some` variants
when we don’t need to use the value inside the `Some`

This code will print `Can't overwrite an existing customized value` and then
`setting is Some(5)`. In the first match arm, we don’t need to match on or use
the values inside either `Some` variant; the important part we need to test for
is the case when both `setting_value` and `new_setting_value` are the `Some`
variant. In that case, we want to print out why we’re not changing
`setting_value`, and we don’t change it.

In all other cases (if either `setting_value` or `new_setting_value` are
`None`), which is expressed by the `_` pattern in the second arm, we do want to
allow `new_setting_value` to become `setting_value`.

<!-- So when we need to match but don't actually need the value, is that what
we're saying? -->
<!-- Yes /Carol -->

We can also use underscores in multiple places within one pattern to ignore
particular values, as shown in Listing 18-19 where we’re ignoring the second
and fourth values in a tuple of five items:

```
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

Listing 18-19: Ignoring multiple parts of a tuple

This will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be
ignored.

#### Ignoring an Unused Variable by Starting its Name with an Underscore

If you create a variable but don’t use it anywhere, Rust will usually issue a
warning, since that could be a bug. Sometimes, though, it’s useful to create a
variable you won’t use yet, like if you’re prototyping or just starting a
project. In this situation you’ll want to tell Rust not to warn you about the
unused variable, which you can do by starting the name of the variable with an
underscore. In Listing 18-20 we create two unused variables, but when we run
this code we should only get a warning about one of them.

```
fn main() {
    let _x = 5;
    let y = 10;
}
```

Listing 18-20: Starting a variable name with an underscore in order to not get
unused variable warnings

Here we get a warning about not using the variable `y`, but not about not using
the variable preceded by the underscore.

Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. Something like `_x` still binds the value to
the variable, whereas `_` doesn’t bind at all. To show a case where this
distinction matters, Listing 18-21 will provide us with an error.

```
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

Listing 18-21: An unused variable starting with an underscore still binds the
value, which may take ownership of the value

We’ll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. Using the underscore by itself,
however, doesn’t ever bind to the value. Listing 18-22 will compile without any
errors since `s` does not get moved into `_`:

```
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

Listing 18-22: Using underscore does not bind the value

This works just fine; because we never bind `s` to anything, it isn’t moved.

#### Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can use the `..` syntax to use only a few
parts and ignore the rest, while avoiding having to list underscores for each
ignored value. The `..` pattern will ignore any parts of a value that we
haven’t explicitly matched in the rest of the pattern. In Listing 18-23, we
have a `Point` struct that holds a coordinate in three dimensional space. In
the `match` expression, we want to operate only on the `x` coordinate and
ignore the values in the `y` and `z` fields:

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

Listing 18-23: Ignoring all fields of a `Point` except for `x` by using `..`

We list the `x` value, and then just include the `..` pattern. This is quicker
than having to list out `y: _` and `z: _`, particularly when working with
structs that have lots of fields, in situations where only one or two fields
are relevant.

`..` will expand to as many values as it needs to be. Listing 18-24 shows a use
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

Listing 18-24: Matching only the first and last values in a tuple and ignoring
all other values

Here, we have the first and last value matched with `first` and `last`. The
`..` will match and ignore everything in the middle.

Using `..` must be unambiguous, however. If it is not clear which values are
intended for matching, and which to be ignored, Rust will error. Listing 18-25
shows an example of using `..` ambiguously that will not compile due to this
ambiguity:

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

Listing 18-25: An attempt to use `..` in a way that is ambiguous

If we compile this example, we get this error:

```
error: `..` can only be used once per tuple or tuple struct pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |                      ^^
```

It’s not possible for Rust to determine how many values in the tuple to ignore
before matching a value with `second`, and then how many further values to
ignore after that. This code could mean that we intend to ignore 2, bind
`second` to 4, then ignore 8, 16, and 32; or we could mean that we want to
ignore 2 and 4, bind `second` to 8, then ignore 16 and 32, and so forth. The
variable name `second` doesn’t mean anything special to Rust, so we get a
compiler error since using `..` in two places like this is ambiguous.

### `ref` and `ref mut` to Create References in Patterns

Here we’ll look at using `ref` to make references so ownership of the values
isn’t moved to variables in the pattern. Usually, when you match against a
pattern, the variables introduced by the pattern are bound to a value. Rust’s
ownership rules mean the value will be moved into the `match`, or wherever
you’re using the pattern. Listing 18-26 shows an example of a `match` that has
a pattern with a variable, and then another usage of the entire value after the
`match`. This will fail to compile because ownership of part of the
`robot_name` value is transferred to the `name` variable in the pattern of the
first `match` arm:

<!-- Can you lay out what is supposed to happen with this code, that doesn't
work? -->
<!-- Done /Carol -->

```
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

Listing 18-26: Creating a variable in a match arm pattern takes ownership of
the value

This example will fail because the value inside `Some` in `robot_name` is moved
to within the `match` when `name` binds to that value. Because ownership of
part of `robot_name` has been moved to `name`, we can no longer use
`robot_name` in the `println!` after the `match` because `robot_name` no longer
has ownership.

<!-- Above -- why will that make it fail, because the bind is then invalid? -->
<!-- Yes, I've clarified a bit /Carol -->

<!--Below -- Is this then the solution, introducing &? I assume so, because we
don’t have & in the example above, but the connection isn't clear -->
<!-- No, the solution is introducing `ref`. I've clarified /Carol -->

In order to fix this code, we want to have the `Some(name)` pattern borrow that
part of `robot_name` rather than taking ownership. Outside of patterns, we’ve
seen that the way to borrow a value is to create a reference using `&`, so you
may think the solution is changing `Some(name)` to `Some(&name)`.

However, we saw in the “Destructuring to Break Apart Values” section that `&`
in patterns does not *create* a reference, it *matches* an existing reference
in the value. Because `&` already has that meaning in patterns, we can’t use
`&` to create a reference in a pattern.

Instead, to create a reference in a pattern, we do this by using the `ref`
keyword before the new variable, as shown in Listing 18-27:

```
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

Listing 18-27: Creating a reference so that a pattern variable does not take
ownership of a value

This example will compile because the value in the `Some` variant in
`robot_name` is not moved into the `match`; the `match` only took a reference
to the data in `robot_name` rather than moving it.

To create a mutable reference in order to be able to mutate a value matched in
a pattern, use `ref mut` instead of `&mut` for the same reason that we use
`ref` instead of `&`: `&mut` in patterns is for matching existing mutable
references, not creating new ones. Listing 18-28 shows an example of a pattern
creating a mutable reference:

```
let mut robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

Listing 18-28: Creating a mutable reference to a value as part of a pattern
using `ref mut`

This example will compile and print `robot_name is: Some("Another name")`.
Because `name` is a mutable reference, we need to dereference within the match
arm code using the `*` operator in order to be able to mutate the value.

### Extra Conditionals with Match Guards

<!-- Can you give a full definition of a match guard here, and what we use it
for, before covering how to do it? -->

A *match guard* is an additional `if` condition specified after the pattern in
a `match` arm that also must match if the pattern matches in order for that arm
to be chosen. Match guards are useful for expressing more complex ideas than a
pattern alone allows.

The condition can use variables created in the pattern. Listing 18-29 shows a
`match` where the first arm has the pattern `Some(x)` and then also has a match
guard of `if x < 5`:

```
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

Listing 18-29: Adding a match guard to a pattern

This example will print `less than five: 4`. When `num` is compared to the
pattern in the first arm, it matches since `Some(4)` matches `Some(x)`. Then
the match guard checks to see if the value in `x` is less than 5, and because 4
is less than 5, the first arm is selected.

If `num` had been `Some(10)` instead, the match guard in the first arm would
have been false since 10 is not less than 5. Rust would then go to the second
arm, which would match because the second arm does not have a match guard and
therefore matches any `Some` variant.

There’s no way to express the `if x < 5` condition within a pattern, so the
match guard has given us the ability to express this logic.

<!-- I think we need this spelled out, can you say what it is the match guard
is doing here? I've had a guess above, but I think it needs your review! -->
<!-- Reviewed and tweaked a bit! /Carol -->

In Listing 18-11, we mentioned that we could use match guards to solve our
pattern shadowing problem, where a new variable was created inside the pattern
in the `match` expression instead of using the variable outside the `match`.
That new variable meant we couldn’t test against the value that the outer
variable had. Listing 18-30 shows how we can use a match guard to fix this:

<!-- Can you check this above -- I've tried to paraphrase the final paragraph
from that section. -->
<!-- Checked and reworded a bit /Carol -->

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

Listing 18-30: Using a match guard to test for equality with an outer variable

This will now print `Default case, x = Some(5)`. The pattern in the second
match arm is now not introducing a new variable `y` that would shadow the outer
`y`, meaning we can use the outer `y` in the match guard. Instead of specifying
the pattern as `Some(y)`, which would have shadowed the outer `y`, we specify
`Some(n)`. This creates a new variable `n` that does not shadow anything
because there is no `n` variable outside the `match`.

In the match guard, `if n == y`, this is not a pattern and therefore does not
introduce new variables. This `y` *is* the outer `y` rather than a new shadowed
`y`, and we can express the idea that we’re looking for a value that has the
same value as the outer `y` by comparing `n` to `y`.

<!-- Why is this one not introducing a new variable y but 18-10 was? Instead we
create a new variable n and then compare it to the outer y, is that it? In
which case, I'm not understanding how we get n from destructuring x, can you
lay this out?-->
<!-- I've elaborated a bit, does this clear it up? /Carol -->

You can also use the or operator `|` in a match guard to specify multiple
patterns, and the match guard condition will apply to all of the patterns.
Listing 18-31 shows the precedence of combining a match guard with a pattern
that uses `|`. The important part of this example is that the `if y` match
guard applies to 4, 5, *and* 6, even though it might look like `if y` only
applies to 6:

<!-- What's the match condition actually doing here, with y having a value of
`false`? Can you let us know how that's being applied to all the values in that
match arm? -->
<!-- The point of the example here is to illustrate operator precedence, that
this code might look like it's saying `4 | 5 | (6 if y)` but it's actually
saying `(4 | 5 | 6) if y`. I've tried to elaborate above and below, does that
make sense now? /Carol -->

```
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

Listing 18-31: Combining multiple patterns with a match guard

The match condition states that the arm only matches if the value of `x` is
equal to 4, 5, or 6 *and* if `y` is `true`. What happens when this code runs is
that the pattern of the first arm matches because `x` is 4, but the match guard
`if y` is false, so the first arm is not chosen. The code moves on to the
second arm, which does match, and this program prints `no`.

<!-- Is this what we mean, if 4 or 5 or 6 being equal to x is false, run the
first arm? And so, because it's applying that to all of the values (including
4), the second arm is run and not the first? -->
<!-- It seems like `if y` was confusing, I've tried to spell it out a bit more.
Does this make sense now? /Carol -->

This is because the `if` condition applies to the whole pattern `4 | 5 | 6`,
and not only to the last value `6`. In other words, the precedence of a match
guard in relation to a pattern behaves like this:

```
(4 | 5 | 6) if y => ...
```

rather than this:

```
4 | 5 | (6 if y) => ...
```

We can tell this from what happened when we ran the code: if the match guard
was only applied to the final value in the list of values specified using the
`|` operator, the arm would have matched and the program would have printed
`yes`.

### `@` Bindings

<!-- Below - use @ to what, can you say explicitly what it does. Also what the
name of the operator is? -->
<!-- I don't think it has a name other than "the at operator". And we tried to
say what it does-- it creates a variable at the same time as letting us test
it, I've tried rewording a bit but I'm not sure why that wasn't explicit
enough, can you clarify if this still doesn't make sense? /Carol -->

The at operator, `@`, lets us create a variable that holds a value at the same
time we’re testing that value to see if it matches a pattern. Listing 18-32
shows an example where we want to test that a `Message::Hello` `id` field is
within the range `3...7` but also be able to bind the value to the variable
`id_variable` so that we can use it in the code associated with the arm. We
could have named `id_variable` `id`, the same as the field, but for the
purposes of this example we’ve chosen to give it a different name:

```
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3...7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

Listing 18-32: Using `@` to bind to a value in a pattern while also testing it

This example will print `Found an id in range: 5`. By specifying `id_variable
@` before the range `3...7`, we’re capturing whatever value matched the range
while also testing that the value matched the range pattern.

In the second arm where we only have a range specified in the pattern, the code
associated with the arm doesn’t have a variable that contains the actual value
of the `id` field. The `id` field’s value could have been 10, 11, or 12 but the
code that goes with that pattern doesn’t know which one and isn’t able to use
the value from the `id` field, because we haven’t saved the `id` value in a
variable.

In the last arm where we’ve specified a variable without a range, we do have
the value available to use in the arm’s code in a variable named `id` because
we’ve used the struct field shorthand syntax. We haven’t applied any test to
the value in the `id` field in this arm, though, like the first two arms did:
any value would match this pattern.

Using `@` lets us test a value and save it in a variable within one pattern.

## Summary

Patterns are a useful feature of Rust that help distinguish between different
kinds of data. When used in `match` statements, Rust makes sure your patterns
cover every possible value, or your program will not compile. Patterns in `let`
statements and function parameters make those constructs more powerful,
enabling the destructuring of values into smaller parts at the same time as
assigning to variables. We can create simple or complex patterns to suit our
needs.

Now, for the penultimate chapter of the book, let’s take a look at some
advanced parts of a variety of Rust’s features.
