## All the Places Patterns May be Used

Patterns pop up in a number of places in Rust, and you’ve been using them a lot
without realizing it! This section is a reference to everywhere you can validly
use patterns.

### `match` Arms

As we discussed in Chapter 6, patterns are used in the arms of `match`
expressions. Formally, `match` expressions are defined as the keyword `match`,
a value to match on, and one or more match arms that consist of a pattern and
an expression to run if the value matches that arm’s pattern:

```text
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

### Conditional`if let` Expressions

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

<span class="filename">Filename: src/main.rs</span>

```rust
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

<span class="caption">Listing 18-1: Mixing `if let`, `else if`, `else if let`,
and `else`</span>

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

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

<span class="caption">Listing 18-2: Using a `while let` loop to print out
values for as long as `stack.pop()` returns `Some`</span>

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

```rust
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

<span class="caption">Listing 18-3: Using a pattern in a `for` loop to
destructure a tuple</span>

This will print:

```text
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

```rust
let x = 5;
```

We’ve done this hundreds of times throughout this book, and though you may not
have realized it, you were using patterns! A `let` statement looks like this,
more formally:

```text
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

```rust
let (x, y, z) = (1, 2, 3);
```

<span class="caption">Listing 18-4: Using a pattern to destructure a tuple and
create three variables at once</span>

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

```rust,ignore
let (x, y) = (1, 2, 3);
```

<span class="caption">Listing 18-5: Incorrectly constructing a pattern whose
variables don’t match the number of elements in the tuple</span>

Attempting to compile this code gives us this type error:

```text
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

```rust
fn foo(x: i32) {
    // code goes here
}
```

<span class="caption">Listing 18-6: A function signature uses patterns in the
parameters</span>

The `x` part is a pattern! Like we did with `let`, we could match a tuple in a
function’s arguments to the pattern. Listing 18-7 splits apart the values in a
tuple as we pass it to a function:

<span class="filename">Filename: src/main.rs</span>

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

<span class="caption">Listing 18-7: A function with parameters that destructure
a tuple</span>

This will print `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` gets the value 3, and `y` gets the value 5.

We can use patterns in closure parameter lists in the same way, too, because
closures are similar to functions, as we discussed in Chapter 13.

We’ve seen several ways of using patterns now, but patterns do not work the
same in every place we can use them; in some places, the patterns must be
*irrefutable*, meaning they must match any value provided. In other
circumstances, they may be refutable. Let’s discuss that next.
