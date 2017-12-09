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

```rust
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
outside the `match` construct---as is the case with all variables. In [Listing 18-11][Listing-18-11],
we declare a variable named `x` with the value `Some(5)` and a variable
`y` with the value `10`. We then create a `match` expression on the value `x`.
Take a look at the patterns in the match arms and `println!` at the end, and
try to figure out what will be printed before running this code or reading
further:

<span class="filename">Filename: src/main.rs</span>

[Listing-18-11]: #Listing-18-11
<a name="Listing-18-11"></a>

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

<span class="caption">Listing 18-11: A `match` statement with an arm that
introduces a shadowed variable `y`</span>

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

```rust
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

```rust
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

```rust
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

[Listing 18-12][Listing-18-12] shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement:

<span class="filename">Filename: src/main.rs

[Listing-18-12]: #Listing-18-12
<a name="Listing-18-12"></a>

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

<span class="caption">Listing 18-12: Destructuring a struct’s fields into
separate variables</span>

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
created from the pattern will have the same names. [Listing 18-13][Listing-18-13] shows code
that behaves in the same way as the code in [Listing 18-12][Listing-18-12], but the variables
created in the `let` pattern are `x` and `y` instead of `a` and `b`:

<span class="filename">Filename: src/main.rs</span>

[Listing-18-13]: #Listing-18-13
<a name="Listing-18-13"></a>

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

<span class="caption">Listing 18-13: Destructuring struct fields using struct
field shorthand</span>

This code creates the variables `x` and `y` that match the `x` and `y` of the
`p` variable. The outcome is that the variables `x` and `y` contain the values
from the `p` struct.

We can also destructure with literal values as part of the struct pattern
rather than creating variables for all of the fields. This allows us to test
some of the fields for particular values while creating variables to
destructure the other fields.

[Listing 18-14][Listing-18-14] shows a `match` statement that separates `Point` values into
three cases: points that lie directly on the `x` axis (which is true when `y =
0`), on the `y` axis (`x = 0`), or neither:

<!-- I'm not sure what you mean by "inner parts of a value" -- that we aren't
matching a whole value but part of it? -->
<!-- I've reworded, is this version clearer? /Carol -->

<span class="filename">Filename: src/main.rs</span>

[Listing-18-14]: #Listing-18-14
<a name="Listing-18-14"></a>

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

<span class="caption">Listing 18-14: Destructuring and matching literal values
in one pattern</span>

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

We’ve destructured enums before in this book, like in [Listing 6-5][Listing-6-5] in Chapter 6
when we destructured an `Option<i32>`. One detail we haven’t mentioned
explicitly is that the pattern to destructure an enum should correspond to the
way the data stored within the enum is defined. For example, let’s take the
`Message` enum from [Listing 6-2][Listing-6-2] and write a `match` with patterns that will
destructure each inner value in [Listing 18-15][Listing-18-15]:

<span class="filename">Filename: src/main.rs</span>

[Listing-18-15]: #Listing-18-15
<a name="Listing-18-15"></a>

```rust
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

<span class="caption">Listing 18-15: Destructuring enum variants that hold
different kinds of values</span>

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

The example in [Listing 18-16][Listing-18-16] iterates over references to `Point` instances in a
vector, and destructures both the reference and the struct so we can perform
calculations on the `x` and `y` values easily:

[Listing-18-16]: #Listing-18-16
<a name="Listing-18-16"></a>

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
    .map(|&Point { x, y }| x * x + y * y)
    .sum();
```

<span class="caption">Listing 18-16: Destructuring a reference to a struct into
the struct field values</span>

<!-- and what do we actually get, instead of the error? -->
<!-- Added explanation text below /Carol -->

This code results in the value 135 in the variable `sum_of_squares`, which is
the result from squaring the `x` value and the `y` value, adding those
together, and then adding the result for each `Point` in the `points` vector to
get one number.

If we had not included the `&` in `&Point { x, y }` we’d get a type mismatch
error, because `iter` would then iterate over references to the items in the
vector rather than the values themselves. The error would look like this:

```text
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

```rust
# struct Point {
#     x: i32,
#     y: i32,
# }
#
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
function parameters, as shown in [Listing 18-17][Listing-18-17]:

<span class="filename">Filename: src/main.rs</span>

[Listing-18-17]: #Listing-18-17
<a name="Listing-18-17"></a>

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

<span class="caption">Listing 18-17: Using `_` in a function signature</span>

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
parts in the corresponding code we want to run. [Listing 18-18][Listing-18-18] shows code
responsible for giving a setting a value. The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting, but can unset the setting and can give the setting a value if it is
currently unset.

[Listing-18-18]: #Listing-18-18
<a name="Listing-18-18"></a>

```rust
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

<span class="caption">Listing 18-18: Using an underscore within patterns that
match `Some` variants when we don’t need to use the value inside the
`Some`</span>

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
particular values, as shown in [Listing 18-19][Listing-18-19] where we’re ignoring the second
and fourth values in a tuple of five items:

[Listing-18-19]: #Listing-18-19
<a name="Listing-18-19"></a>

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

<span class="caption">Listing 18-19: Ignoring multiple parts of a tuple</span>

This will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be
ignored.

#### Ignoring an Unused Variable by Starting its Name with an Underscore

If you create a variable but don’t use it anywhere, Rust will usually issue a
warning, since that could be a bug. Sometimes, though, it’s useful to create a
variable you won’t use yet, like if you’re prototyping or just starting a
project. In this situation you’ll want to tell Rust not to warn you about the
unused variable, which you can do by starting the name of the variable with an
underscore. In [Listing 18-20][Listing-18-20] we create two unused variables, but when we run
this code we should only get a warning about one of them.

<span class="filename">Filename: src/main.rs</span>

[Listing-18-20]: #Listing-18-20
<a name="Listing-18-20"></a>

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

<span class="caption">Listing 18-20: Starting a variable name with an
underscore in order to not get unused variable warnings</span>

Here we get a warning about not using the variable `y`, but not about not using
the variable preceded by the underscore.

Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. Something like `_x` still binds the value to
the variable, whereas `_` doesn’t bind at all. To show a case where this
distinction matters, [Listing 18-21][Listing-18-21] will provide us with an error.

[Listing-18-21]: #Listing-18-21
<a name="Listing-18-21"></a>

```rust,ignore
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">Listing 18-21: An unused variable starting with an
underscore still binds the value, which may take ownership of the value</span>

We’ll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. Using the underscore by itself,
however, doesn’t ever bind to the value. [Listing 18-22][Listing-18-22] will compile without any
errors since `s` does not get moved into `_`:

[Listing-18-22]: #Listing-18-22
<a name="Listing-18-22"></a>

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

<span class="caption">Listing 18-22: Using underscore does not bind the
value</span>

This works just fine; because we never bind `s` to anything, it isn’t moved.

#### Ignoring Remaining Parts of a Value with `..`

With values that have many parts, we can use the `..` syntax to use only a few
parts and ignore the rest, while avoiding having to list underscores for each
ignored value. The `..` pattern will ignore any parts of a value that we
haven’t explicitly matched in the rest of the pattern. In [Listing 18-23][Listing-18-23], we
have a `Point` struct that holds a coordinate in three dimensional space. In
the `match` expression, we want to operate only on the `x` coordinate and
ignore the values in the `y` and `z` fields:

[Listing-18-23]: #Listing-18-23
<a name="Listing-18-23"></a>

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

<span class="caption">Listing 18-23: Ignoring all fields of a `Point` except
for `x` by using `..`</span>

We list the `x` value, and then just include the `..` pattern. This is quicker
than having to list out `y: _` and `z: _`, particularly when working with
structs that have lots of fields, in situations where only one or two fields
are relevant.

`..` will expand to as many values as it needs to be. [Listing 18-24][Listing-18-24] shows a use
of `..` with a tuple:

<span class="filename">Filename: src/main.rs</span>

[Listing-18-24]: #Listing-18-24
<a name="Listing-18-24"></a>

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

<span class="caption">Listing 18-24: Matching only the first and last values in
a tuple and ignoring all other values</span>

Here, we have the first and last value matched with `first` and `last`. The
`..` will match and ignore everything in the middle.

Using `..` must be unambiguous, however. If it is not clear which values are
intended for matching, and which to be ignored, Rust will error. [Listing 18-25][Listing-18-25]
shows an example of using `..` ambiguously that will not compile due to this
ambiguity:

<span class="filename">Filename: src/main.rs</span>

[Listing-18-25]: #Listing-18-25
<a name="Listing-18-25"></a>

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

<span class="caption">Listing 18-25: An attempt to use `..` in a way that is
ambiguous</span>

If we compile this example, we get this error:

```text
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
you’re using the pattern. [Listing 18-26][Listing-18-26] shows an example of a `match` that has
a pattern with a variable, and then another usage of the entire value after the
`match`. This will fail to compile because ownership of part of the
`robot_name` value is transferred to the `name` variable in the pattern of the
first `match` arm:

<!-- Can you lay out what is supposed to happen with this code, that doesn't
work? -->
<!-- Done /Carol -->

[Listing-18-26]: #Listing-18-26
<a name="Listing-18-26"></a>

```rust,ignore
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-26: Creating a variable in a match arm pattern
takes ownership of the value</span>

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
keyword before the new variable, as shown in [Listing 18-27][Listing-18-27]:

[Listing-18-27]: #Listing-18-27
<a name="Listing-18-27"></a>

```rust
let robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref name) => println!("Found a name: {}", name),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-27: Creating a reference so that a pattern
variable does not take ownership of a value</span>

This example will compile because the value in the `Some` variant in
`robot_name` is not moved into the `match`; the `match` only took a reference
to the data in `robot_name` rather than moving it.

To create a mutable reference in order to be able to mutate a value matched in
a pattern, use `ref mut` instead of `&mut` for the same reason that we use
`ref` instead of `&`: `&mut` in patterns is for matching existing mutable
references, not creating new ones. [Listing 18-28][Listing-18-28] shows an example of a pattern
creating a mutable reference:

[Listing-18-28]: #Listing-18-28
<a name="Listing-18-28"></a>

```rust
let mut robot_name = Some(String::from("Bors"));

match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
}

println!("robot_name is: {:?}", robot_name);
```

<span class="caption">Listing 18-28: Creating a mutable reference to a value as
part of a pattern using `ref mut`</span>

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

The condition can use variables created in the pattern. [Listing 18-29][Listing-18-29] shows a
`match` where the first arm has the pattern `Some(x)` and then also has a match
guard of `if x < 5`:

[Listing-18-29]: #Listing-18-29
<a name="Listing-18-29"></a>

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

<span class="caption">Listing 18-29: Adding a match guard to a pattern</span>

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

In [Listing 18-11][Listing-18-11], we mentioned that we could use match guards to solve our
pattern shadowing problem, where a new variable was created inside the pattern
in the `match` expression instead of using the variable outside the `match`.
That new variable meant we couldn’t test against the value that the outer
variable had. [Listing 18-30][Listing-18-30] shows how we can use a match guard to fix this:

<!-- Can you check this above -- I've tried to paraphrase the final paragraph
from that section. -->
<!-- Checked and reworded a bit /Carol -->

<span class="filename">Filename: src/main.rs</span>

[Listing-18-30]: #Listing-18-30
<a name="Listing-18-30"></a>

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

<span class="caption">Listing 18-30: Using a match guard to test for equality
with an outer variable</span>

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
[Listing 18-31][Listing-18-31] shows the precedence of combining a match guard with a pattern
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

[Listing-18-31]: #Listing-18-31
<a name="Listing-18-31"></a>

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

<span class="caption">Listing 18-31: Combining multiple patterns with a match
guard</span>

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

```text
(4 | 5 | 6) if y => ...
```

rather than this:

```text
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
time we’re testing that value to see if it matches a pattern. [Listing 18-32][Listing-18-32]
shows an example where we want to test that a `Message::Hello` `id` field is
within the range `3...7` but also be able to bind the value to the variable
`id_variable` so that we can use it in the code associated with the arm. We
could have named `id_variable` `id`, the same as the field, but for the
purposes of this example we’ve chosen to give it a different name:

[Listing-18-32]: #Listing-18-32
<a name="Listing-18-32"></a>

```rust
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

<span class="caption">Listing 18-32: Using `@` to bind to a value in a pattern
while also testing it</span>

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
