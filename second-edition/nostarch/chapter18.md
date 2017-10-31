
[TOC]

# Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of
types, both complex and simple. Matching patterns gives you more control over
the execution of a program. The pattern itself will be made up of some
combination of types, whether literals; destructured arrays, enums, structs, or
tuples; variables, wildcards, and placeholders. These pieces describe the shape
of the data we're working with, which we then match against the values to
determine whether our program has the correct data to continue.

<!-- I think we need a concise description of what we use patterns for here,
what they provide the programmer. Hopefully you can see what I've trying to do,
above! But I think you'll agree it's not quite right, can you have a whack, try
to give the reader that explanation? -->

To use a pattern we compare it to some value. If the pattern matches our value,
we use the value parts in our code. Recall our `match` expressions from Chapter
6 that used patterns like a coin sorting machine. If the value fits the shape
of the pattern, we can use the named pieces. If it doesn't, our program might
not run or might have some other defined reaction.

This chapter is a reference on all things related to patterns. We'll cover the
valid places to use patterns, the difference between *refutable* and
*irrefutable* patterns, and the different kinds of pattern syntax that you
might see. By the end, you'll see how to use patterns to create powerful and
efficient code.

## All the Places Patterns May be Used

Patterns pop up in a number of places in Rust, and you've been using them a lot
without realizing it! This section is a reference to everywhere you can validly
use patterns.

### `match` Arms

As we discussed in Chapter 6, patterns are used in the arms of `match`
expressions. Formally, `match` expressions are defined as the keyword `match`,
a value to match on, and one or more match arms that consist of a pattern and
an expression to run if the value matches that arm's pattern:

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

There's a particular pattern `_` that will match anything, but never binds to a
variable, and so is often used in the last match arm `_`. This can be useful
when you want to ignore any value no specified, for example. We'll cover this
in more detail later in this chapter.

### Conditional`if let` Expressions

In Chapter 6 we discussed how `if let` expressions are used mainly as a shorter
way to write the equivalent of a `match` that only cares about matching one
case. Optionally,`if let` can have a corresponding `else` with code to run if
the pattern in the `if let` doesn't match. We might use use this method rather
than a `match` when we have a series of complex conditionals, which would be
difficult to read in a `match` expression.

<!-- Can you say up front why we'd use this, and not just a match? I've just
added something here, not sure if it's right -->

Listing 18-1 shows that it's even possible to mix and match `if let`, `else
if`, and `else if let` expressions. This code shows a series of checks for a
bunch of different conditions that decide what the background color should be.
For the purposes of the example, we've created variables with hardcoded values
that a real program might get by asking the user.

If the user has specified a favorite color, that is used as the background
color. If today is Tuesday, the background color will be green. If the user has
specified their age as a string and we can parse it as a number successfully,
we'll use either purple or orange depending on the value of the parsed number.
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
place the `if age > 30` condition within that block; we can't combine these two
conditions into `if let Ok(age) = age && age > 30` because the shadowed `age`
we want to compare to 30 isn't valid until the new scope starts with the curly
brace.

The downside of using `if let` expressions in this way is that exhaustiveness
is not checked by the compiler, whereas with `match` expressions it is. If we
left off the last `else` block and so missed handling some cases, the compiler
would not error.

<!-- So what would happen, we'd just end up with a program that wasn't correct,
in the Rust sense? -->

This particular example is probably too complex to rewrite as a readable
`match`, but when writing it as conditionals we need to take extra care to
check that we're handling all the cases, since the compiler doesn't check it
for us.

### `while let` Conditional Loops

Similar in constructionion to `if let`, the `while let` conditional statement
allows your `while` loop to run for as long as a pattern continues to match.
The example in Listing 18-2 shows a `while let` loop that uses a vector as a
stack and prints out the values in the vector in the opposite order they were
pushed in:

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
construction in Rust code, but we haven't yet discussed the pattern that `for`
takes. In a `for` loop, the pattern is the value that directly follows the
keyword `for`, so the`x` in `for x in y`.

<!-- Can you check the line I added above? I think it'd help to point out the
pattern section of a for loop straight away -->

Listing 18-3 demonstrates how to use a pattern in a `for` loop to destructure a
tuple; to *destructure* means to break a value, like a tuple or struct, up into
its component pieces, and is a valuable technique to know.

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
value's index in the iterator, placed into a tuple. The first call to
`enumerate` produces the tuple `(0, 1)`. When this value is matched to the
pattern `(index, value)`, `index` will be 0 and `value` will be 1, printing our
first line of output.

### `let` Statements

Before this chapter, we'd only explicitly discussed using patterns with `match`
and `if let`, but in fact we've used patterns in other places too, including
`let` statements. For example, consider this straightforward variable
assignment with `let`:

```
let x = 5;
```

We've done this hundreds of times throughout this book and though you may not
have realized it, you were using patterns! A `let` statement looks like this,
more formally:

```
let PATTERN = EXPRESSION;
```

In statements like `let x = 5;` with a variable name in the `PATTERN` slot, the
variable name is just a particularly humble form of pattern. We compare the
expression against the pattern, and assign any names we find. So for our `let x
= 5;` example, `x` is a pattern that says "bind what matches here to the
variable `x`." And since the name `x` is the whole pattern, this pattern
effectively means "bind everything to the variable `x`, whatever the value is."

To see the pattern matching aspect of `let` a bit more clearly, consider
Listing 18-4 where we're using a pattern with `let` to destructure a tuple:

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

### Function Parameters

Function parameters can also be patterns. The code in Listing 18-5, declaring a
function named `foo` that takes one parameter named `x` of type `i32`, should
by now look familiar:

```
fn foo(x: i32) {
    // code goes here
}
```

Listing 18-5: A function signature uses patterns in the parameters

The `x` part is a pattern! Like we did with `let`, we could match a tuple in a
function's arguments to the pattern. Listing 18-6 splits apart the values in a
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

Listing 18-6: A function with parameters that destructure a tuple

This will print `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` gets the value 3, and `y` gets the value 5.

We can use patterns in closure parameter lists in the same way, too, because
closures are similar to functions, as we discussed in Chapter 13.

We've seen several ways of using patterns now, but patterns do not work the
same in every place we can use them; in some places, the patterns must be
*irrefutable*, meaning they must match any value provided. In other
circumstances, they may be refutable. Let's discuss that next.

## Refutability: Whether a Pattern Might Fail to Match

Patterns come in two forms: refutable and irrefutable. Patterns that will match
for any possible value passed are said to be *irrefutable*. An example would be
`x` in the statement `let x=5;` beause `x` matches anything and so cannot fail
to match. Patterns that may fail to match for some possible value are said to
be *refutable*. An example of this would be XXXXX

`let` statements, function parameters, and `for` loops can only accept
irrefutable patterns, because the program cannot continue do anything
meaningful with values that don't match. The `if let` and `while let`
expressions are restricted to only accept refutable patterns, because by
definition they're intended to handle possible failure--the functionality of a
conditional is in it's ability to perform differently upon success and failure.

In general, you shouldn't have to worry about the distinction between refutable
and irrefutable patterns, but you do need to be familiar with the concept of
refutability so you can respond when you see it in an error message. In those
cases, you'll need to change either the pattern or the construct you're using
the pattern with, depending on your intentions for the behavior of the code.

Let's look at some examples. An irrefutable pattern matches anything, and can't
fail to match, like `x` from our `let x = 5` example. Let's instead see what
happens when we try to match one variant of an enum, such as a `Some<T>` value
from the `Option<T>` enum with `let`, shown in Listing 18-7. As you might
expect, this will error.

```
let Some(x) = some_option_value;
```

Listing 18-7: Attempting to use a refutable pattern with `let`

If `some_option_value` was a `None` value, it would fail to match the pattern
`Some(x)`, meaning the pattern is refutable. The `let` statement, however, can
only accept an irrefutable patterns because there's nothing valid the code
could do with a `None` value. At compile time, Rust will complain that we've
tried to use a refutable pattern where an irrefutable pattern is required:

```
error[E0005]: refutable pattern in local binding: `None` not covered
 --> <anon>:3:5
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

We didn't cover (and couldn't cover!) every valid value with the pattern
`Some(x)`, so Rust will rightfully complain.

To fix the case where we have a refutable pattern in a place where an
irrefutable pattern is needed, we can change the code that uses the pattern:
instead of using `let`, we can use `if let`. That way, if the pattern doesn't
match, the code will just skip the code in the curly brackets, giving it a way
to continue validly. Listing 18-8 shows how to fix the code in Listing 18-7.

```
# let some_option_value: Option<i32> = None;
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

Listing 18-8: Using `if let` and a block with refutable patterns instead of
`let`

<!-- Whats the first commented out line here, I had though this was copied from
8-7 but it isn't quite the same -->

We've given the code an out! This code is perfectly valid, though does now of
course mean we cannot use an irrefutable pattern without receiving an error. If
we give `if let` a pattern that will always match, such as `x` as shown in
Listing 18-9, it will error:

```
if let x = 5 {
    println!("{}", x);
};
```

Listing 18-9: Attempting to use an irrefutable pattern with `if let`

Rust complains that it doesn't make sense to use `if let` with an irrefutable
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
an irrefutable pattern in a `match` with only one arm is allowed, but isn't
particularly useful and could be replaced with a simpler `let` statement.

Now that we've discussed where patterns can be used and the difference between
refutable and irrefutable patterns, let's go over all the syntax we can use to
create patterns.

## All the Pattern Syntax

We've seen examples of many different kinds of patterns throughout the book, so
we'll gather all the syntax valid in patterns in one place here, and why you
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

This prints `one` since the value in `x` is 1. ==This is useful in many
applications, XXXXXXX==.

### Matching Named Variables

<!-- I found this next bit a little tougher to follow, I've tried to clarify in
this opening paragraph, connect it all up, can you please check it? -->

Named variables are irrefutable patterns that match any value. There is a
complication, however, when used in `match` expressions. Because `match` starts
a new scope, variables declared as part of a pattern inside the `match`
expression will shadow those with the same name outside the `match`
construct---as is the case with all variables. In Listing 18-10, we declare a
variable named `x` with the value `Some(5)` and a variable `y` with the value
`10`. We then create a `match` expression on the value `x`. Take a look at the
patterns in the match arms and `println!` at the end, and try to figure out
what will be printed before running this code or reading further:

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

Let's walk through what happens when the `match` statement runs. The pattern in
the first match arm does not match the defined value of `x`, so we continue.

The pattern in the second match arm introduces a new variable name `y` that
will match any value inside a `Some` value. Because we're in a new scope inside
the `match` expression, this is a new variable, and not the `y` we declared at
the beginning with the value 10. This new `y` binding will match any value
inside a `Some`, which is what we have in `x`. Therefore this `y` binds to the
inner value of the `Some` in `x`. That value is 5, and so the expression for
that arm executes and prints `Matched, y = 5`.

<!-- Below -- We haven't fully introduced the underscore yet, is there anything
else we could use for that final arm? -->

If `x` had been a `None` value instead of `Some(5)`, the patterns in the first
two arms would not have matched, so we would have matched to the underscore. We
did not introduce the `x` variable in the pattern of that arm, so the `x` in
the expression is still the outer `x` that has not been shadowed. In this
hypothetical case, the `match` would print `Default case, x = None`.

Once the `match` expression is over, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.

To create a `match` expression that compares the values of the outer `x` and
`y`, rather than introducing a shadowed variable, we would need to use a match
guard conditional instead. We'll be talking about match guards later in this
section.

### Multiple Patterns

In `match` expressions you can match multiple patterns using the `|` syntax,
standing in for *or*. For example, the following code matches the value of `x`
against the match arms, the first of which has an *or* option, meaning if the
value of `x` matches either of the values in that arm, it will run:

<!-- I've tried to flesh this out a bit, can you check? -->

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

### Matching Ranges of Values with `...`

The `...` syntax allows you to match to an inclusive range of values. In the
following code, when a pattern matches just one of the values in the range,
that arm will execute:

<!-- Above--this seems like it's true, that the range allows you to match to
just one of the values? If so, can you say how this differs to using the or
operator? -->

```
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

If `x` is 1, 2, 3, 4, or 5, the first arm will match. Ranges are only allowed
with numeric values or `char` values, because the values need an inherent order
for Rust to know which values are included in the range.

<!-- why, because they are the only types with inherent order? -->

Here's an example using ranges of `char` values:

```
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

Rust knows what to substitute in place of `...` because chars have an inherent
order, and so can tell that `c` is within that range. This will print `early
ASCII letter`.

### Destructuring to Break Apart Values

<!-- I moved the definition of destructure earlier in the chapter, to when we
first use it -->

We can also use patterns to destructure structs, enums, tuples, and references.
We've destructured enums before in this book, like in Listing 6-5 in Chapter 6
when we destructured an `Option<i32>` using a `match` expression and added one
to the inner value of the `Some` variant, so won't cover it again here . If you
want a refresher, flip back to Chapter 6. Here we'll go straight to structs.

<!-- Above -- I think that's what we say later, that, we're skipping enums, but
please delete if not! You'll see my note where it comes up later -->

#### Destructuring Structs

Listing 18-11 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement:

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

<!-- I'm not sure I follow which part of this is the shorthand, what is it
shorthand for, and which syntax here counts as the shorthand? Can you slow this
down, talk it through a little more. Is the point of this section that we have
a shorthand for destructuring, or that we are able to destructure these items
with patterns at all? -->

This code creates the variables `x` and `y` that match the `x` and `y` of the
`p` variable. The outcome is that the variables `x` and `y` contain the values
from the `p` struct. To use this shorthand the variables you destructure into
must have the same names as the fields in the struct.

If we do want to use different names, however, we can specify `field_name:
variable_name` in the pattern. In Listing 18-12, we destructure the values in
the `x` and `y` fields of the tuple into the variables `a` and `b`.

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

`a` will have the value in the `Point` instance's `x` field and `b` will have
the value in the `y` field.

We can also destructure with literal values. This allows us to use inner parts
of a value rather than the whole value. For example, we can match and find just
one point in a `Point` struct, ignoring the other. Listing 18-13 shows a
`match` statement that determines whether a point lies directly on the `x` axis
(which is true when `y = 0`), on the `y` axis (`x = 0`), or neither:

<!-- I'm not sure what you mean by "inner parts of a value" -- that we aren't
matching a whole value but part of it? -->

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

Here we match only one of the two values contained in `p`. The value `p`
matches the second arm by virtue of `x` containing a 0, so this will print `On
the y axis at 7`.

<!-- Does "use destructuring on enums" mean the same thing as "destructure
enums"? If so, can we use the latter? It seems easier to follow if it's
consistently phrased. -->

<!-- We've destructured enums before in this book, like in Listing 6-5 in
Chapter 6, when we destructured an `Option<i32>` using a `match` expression and
added one to the inner value of the `Some` variant.-->

<!-- I can't tell if this, above, is connected to the previous and next
paragraphs, or if we are saying "we won't go into it here since we've done it
before" -- I've assumed the former, earlier in the chapter, but please correct
if I've misunderstood-->

#### Destructuring References

When the value we're matching to our pattern contains a reference, we need to
separate the reference and the value, which we can do can by specifying a `&`
in the pattern.

<!-- What does it mean, to separate the reference and the value, precisely? So
that we specify Rust use the value in place of the reference? And what does &
here do, tell Rust to follow the reference to the value itself, rather than
work on the reference?-->

This is especially useful in closures where we have iterators that iterate over
references, but we want to use the values in the closure rather than the
references.

The example in Listing 18-14 iterates over references to `Point` instances in a
vector, and destructures both the reference and the struct so we can perform
calculations on the `x` and `y` values easily:

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

<!-- and what do we actually get, instead of the error? -->

If we had not included the `&` we'd get a type mismatch error, because `iter`
would then iterate over references to the items in the vector rather than the
values themselves. The error would look like this:

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

This tells us that Rust was expecting our closure to match `&Point`, but we
tried to match directly to a `Point` value, and not a reference to a `Point`.

#### Destructuring Structs and Tuples

We can mix, match, and nest destructuring patterns in even more complex way.
Here's an example of a complicated destructure, where we nest structs and
tuples inside a tuple, and destructure all the primitive values out:

```
# struct Point {
#     x: i32,
#     y: i32,
# }
#
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

This lets us break complex types into their component parts.

<!-- Can you round up the destructuring section here before we move on. For
this bit, maybe say explicitly what this would be useful for -->

### Ignoring Values in a Pattern

We've seen that it's sometimes useful to ignore values in a pattern, such as in
the last arm of a `match` to give us a catch-all that doesn't actually do
anything, but does account for all remaining possible valiues. There are a few
ways to ignore entire values or parts of values in a pattern: using the `_`
pattern (which we've seen), using the `_` pattern within another pattern, using
a name that starts with an underscore, or using `..` to ignore remaining parts
of a value. Let's explore how and why to do each of these.

#### Ignoring an Entire Value with `_`

We've used the underscore as a wildcard pattern that will match any value but
not bind to the value. While the underscore pattern is especially useful as the
last arm in a `match` expression, we can use it in any pattern, including
function arguments, as shown in Listing 18-15:

```
fn foo(_: i32) {
    // code goes here
}
```

Listing 18-15: Using `_` in a function signature

<!-- What is this doing exactly, can you help the reader out here? Are we
letting the function run without a parameter at all? I'm not sure the purpose
clear enough at the moment -->

Normally, to achieve XXX, you would change the signature so it doesn't include
the unused parameter. In some cases though, such as when implementing a trait,
you need a certain type signature; here, using an underscore lets you ignore a
parameter. The compiler will then not warn about unused function parameters, as
it would if we used a name instead.

#### Ignoring Parts of a Value with a Nested `_`

<!-- When would we want to do this? -->

We can also use `_` inside of another pattern to ignore just part of a value.
In Listing 18-16, the pattern in the first `match` arm matches a `Some` value
but ignores the value inside the `Some` variant, as specified by the underscore.

```
let x = Some(5);

match x {
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

Listing 18-16: Ignoring the value inside of the `Some` variant by using a
nested underscore

This is useful when the code associated with the `match` arm doesn't use the
nested part of the variable at all.

<!-- So when we need to match but don't actually need the value, is that what
we're saying? -->

We can also use underscores in multiple places within one pattern to ignore
particular values, as shown in Listing 18-17 where we're ignoring the second
and fourth values in a tuple of five items:

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

If you create a variable but don't use it anywhere, Rust will usually issue a
warning, since that could be a bug. Sometimes, though, it's useful to create a
variable you won't use yet, like if you're prototyping or just starting a
project. In this situation you'll want to tell Rust not to warn you about the
unused variable, which you can do by starting the name of the variable with an
underscore. In Listing 18-18 we create two unused variables, but when we run
this code we should only get a warning about one of them.

```
fn main() {
    let _x = 5;
    let y = 10;
}
```

Listing 18-18: Starting a variable name with an underscore in order to not get
unused variable warnings

Here we get a warning about not using the variable `y`, but not about not using
the variable preceded by the underscore.

Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. Something like `_x` still binds the value to
the variable, whereas `_` doesn't bind at all. To show a case where this
distinction matters, Listing 18-19 will provide us with an error.

```
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

Listing 18-19: An unused variable starting with an underscore still binds the
value, which may take ownership of the value

We'll receive an error beause the `s` value will still be moved into `_s`,
which prevents us from using `s` again. Using the underscore by itself,
however, doesn't ever bind to the value. Listing 18-20 will compile without any
errors since `s` does not get moved into `_`:

```
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

Listing 18-20: Using underscore does not bind the value

This works just fine; because we never bind `s` to anything, it isn't moved.

#### Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can use the `..` syntax to use only a few
parts and ignore the rest, while avoiding having to list underscores for each
ignored value. The `..` pattern will ignore any parts of a value that we
haven't explicitly matched in the rest of the pattern. In Listing 18-21, we
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

Listing 18-21: Ignoring all fields of a `Point` except for `x` by using `..`

We list the `x` value, and then just include the `..` pattern. This is quicker
than having to list out `y: _` and `z: _`, particularly when working with
structs that have lots of fields, in situations where only one or two fields
are relevant.

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
all other values

Here, we have the first and last value matched with `first` and `last`. The
`..` will match and ignore everything in the middle.

Using `..` must be unambiguous, however. If it is not clear which values are
intended for matching, and which to be ignored, Rust will error. Listing 18-23
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

Listing 18-23: An attempt to use `..` in a way that is ambiguous

If we compile this example, we get this error:

```
error: `..` can only be used once per tuple or tuple struct pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |                      ^^
```

It's not possible for Rust to determine how many values in the tuple to ignore
before matching a value with `second`, and then how many further values to
ignore after that. This code could mean that we intend to ignore 2, bind
`second` to 4, then ignore 8, 16, and 32; or we could mean that we want to
ignore 2 and 4, bind `second` to 8, then ignore 16 and 32, and so forth. The
variable name `second` doesn't mean anything special to Rust, so we get a
compiler error since using `..` in two places like this is ambiguous.

### `ref` and `ref mut` to Create References in Patterns

Here we'll look at using `ref` to make references so ownership of the values
isn't moved to variables in the pattern. Usually, when you match against a
pattern, the variables introduced by the pattern are bound to a value. Rust's
ownership rules mean the value will be moved into the `match`, or wherever
you're using the pattern. Listing 18-24 shows an example of XXXXX that will
fail to compile:

<!-- Can you lay out what is supposed to happen with this code, that doesn't
work? -->

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

This example will fail because the value inside `Some` in `robot_name` is moved
to within the `match` when `name` binds to that value.

<!-- Above -- why will that make it fail, because the bind is then invalid? -->
<!--Below -- Is this then the solution, introducing &? I assume so, becuase we
dont have & in the example above, but the connection isn't clear -->

We saw in the =="Destructuring to Break Apart Values"== section that we can use
`&` in a pattern to match an existing reference in the value . To fix this,
therefore, we can *create* a reference to borrow the value in a pattern
variable. We do this by using the `ref` keyword before the new variable, as
shown in Listing 18-25:

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
`robot_name` is not moved into the match; the match only took a reference to
the data in `robot_name` rather than moving it.

To create a mutable reference, use `ref mut` for the same reason, as shown in
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
Because `name` is a mutable reference, we need to dereference within the match
arm code using the `*` operator in order to be able to mutate the value.

### Extra Conditionals with Match Guards

<!-- Can you give a full definition of a match guard here, and what we use it
for, before covering how to do it? -->

To introduce *match guards* as part of a match arm, you simply specify an
additional `if` conditional after the pattern. This will XXXXXXXXXXX. The
conditional can use variables created in the pattern. Listing 18-27 compared
the `num` variable pattern to three arms and executes the matching one. The
first arm has a match guard to ensure that it only executes under certain
circumstances:

```
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

Listing 18-27: Adding a match guard to a pattern

This example will print `less than five: 4`, because the match guard in the
first arm checks whether the value being matched is less than 5 and, if it is,
runs that first arm. The value in this case is of course 4.

<!-- I think we need this spelled out, can you say what it is the match guard
is doing here? I've had a guess above, but I think it needs your review! -->

If `num` was instead `Some(7)`, this example would print `7`. Match guards
allow you to express more complexity than patterns alone give you.

In Listing 18-10, we mentioned that we could use match guards to solve our
pattern shadowing problem, where a new variable was created inside the pattern
in the `match` expression, meaning we couldn't match to values outside the
expression. Listing 18-28 shows how we can use a match guard to fix this:

<!-- Can you check this above -- I've tried to paraphrase the final paragraph
from that section. -->

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

This will now print `Default case, x = Some(5)`. The pattern in the second
match arm is now not introducing a new variable `y` that would shadow the outer
`y`, meaning we can use the outer `y` in the match guard. We're still
destructuring `x` to get the inner value `n`, and then we can compare `n` and
`y` in the match guard.

<!-- Why is this one not introducing a new variable y but 18-10 was? Instead we
create a new variable n and then compare it to the outer y, is that it? In
which case, I'm not understanding how we get n from destructuring x, can you
lay this out?-->

You can also use the or operator `|` in a match guard to specify multiple
patterns, and the match guard condition will apply to all of the patterns.
Listing 18-29 shows a match guard that applies to all three values in the first
arm:

<!-- What's the match condition actually doing here, with y having a value of
`false`? Can you let us know how that's being applied to all the values in that
match arm? -->

```
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

Listing 18-29: Combining multiple patterns with a match guard

The match condition states that the arm only matches if the value of `x` is
equal to `y`; that is, if the value of `x` is not in that arm. That match
condition is being applied to all three values, and must be true of all three.
The first arm does indeed include the value of `x`, meaning the second variant
is executed, and the program prints `no`.

<!-- Is this what we mean, if 4 or 5 or 6 being equal to x is false, run the
first arm? And so, because it's applying that to all of the values (including
4), the second arm is run and not the first? -->

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

If it were only applied to the final value, the arm would have matched, of
course.

### `@` Bindings

<!-- Below - use @ to what, can you say explicitly what it does. Also what the
name of the operator is? -->

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
the range, we're capturing whatever value matched the range while also testing
it. In the second arm where we only have a range specified in the pattern, the
code associated with the arm doesn't know if `id` is 10, 11, or 12, since we
haven't saved the `id` value in a variable: we only know that the value matched
something in that range if that arm's code is executed. In the last arm where
we've specified a variable without a range, we do have the value available to
use in the arm's code, but we haven't applied any other test to the value.
Using `@` lets us test a value and save it in a variable within one pattern.

## Summary

Patterns are a useful feature of Rust that help distinguish between different
kinds of data. When used in `match` statements, Rust makes sure your patterns
cover every possible value, or your program will not compile. Patterns in `let`
statements and function parameters make those constructs more powerful,
enabling the destructuring of values into smaller parts at the same time as
assigning to variables. We can create simple or complex patterns to suit our
needs.

Now, for the penultimate chapter of the book, let's take a look at some
advanced parts of a variety of Rust's features.
