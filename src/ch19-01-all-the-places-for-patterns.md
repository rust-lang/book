## All the Places Patterns Can Be Used

Patterns pop up in a number of places in Rust, and you’ve been using them a lot
without realizing it! This section discusses all the places where patterns are
valid.

### `match` Arms

As discussed in Chapter 6, we use patterns in the arms of `match` expressions.
Formally, `match` expressions are defined as the keyword `match`, a value to
match on, and one or more match arms that consist of a pattern and an
expression to run if the value matches that arm’s pattern, like this:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre><code>match <em>VALUE</em> {
    <em>PATTERN</em> => <em>EXPRESSION</em>,
    <em>PATTERN</em> => <em>EXPRESSION</em>,
    <em>PATTERN</em> => <em>EXPRESSION</em>,
}</code></pre>

For example, here’s the `match` expression from Listing 6-5 that matches on an
`Option<i32>` value in the variable `x`:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

The patterns in this `match` expression are the `None` and `Some(i)` on the
left of each arrow.

One requirement for `match` expressions is that they need to be _exhaustive_ in
the sense that all possibilities for the value in the `match` expression must
be accounted for. One way to ensure you’ve covered every possibility is to have
a catch-all pattern for the last arm: for example, a variable name matching any
value can never fail and thus covers every remaining case.

The particular pattern `_` will match anything, but it never binds to a
variable, so it’s often used in the last match arm. The `_` pattern can be
useful when you want to ignore any value not specified, for example. We’ll cover
the `_` pattern in more detail in [“Ignoring Values in a
Pattern”][ignoring-values-in-a-pattern]<!-- ignore --> later in this chapter.

### let Statements

Prior to this chapter, we had only explicitly discussed using patterns with
`match` and `if let`, but in fact, we’ve used patterns in other places as well,
including in `let` statements. For example, consider this straightforward
variable assignment with `let`:

```rust
let x = 5;
```

Every time you’ve used a `let` statement like this you’ve been using patterns,
although you might not have realized it! More formally, a `let` statement looks
like this:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre>
<code>let <em>PATTERN</em> = <em>EXPRESSION</em>;</code>
</pre>

In statements like `let x = 5;` with a variable name in the PATTERN slot, the
variable name is just a particularly simple form of a pattern. Rust compares
the expression against the pattern and assigns any names it finds. So, in the
`let x = 5;` example, `x` is a pattern that means “bind what matches here to
the variable `x`.” Because the name `x` is the whole pattern, this pattern
effectively means “bind everything to the variable `x`, whatever the value is.”

To see the pattern-matching aspect of `let` more clearly, consider Listing
19-1, which uses a pattern with `let` to destructure a tuple.


<Listing number="19-1" caption="Using a pattern to destructure a tuple and create three variables at once">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Here, we match a tuple against a pattern. Rust compares the value `(1, 2, 3)`
to the pattern `(x, y, z)` and sees that the value matches the pattern, in that
it sees that the number of elements is the same in both, so Rust binds `1` to
`x`, `2` to `y`, and `3` to `z`. You can think of this tuple pattern as nesting
three individual variable patterns inside it.

If the number of elements in the pattern doesn’t match the number of elements
in the tuple, the overall type won’t match and we’ll get a compiler error. For
example, Listing 19-2 shows an attempt to destructure a tuple with three
elements into two variables, which won’t work.

<Listing number="19-2" caption="Incorrectly constructing a pattern whose variables don’t match the number of elements in the tuple">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Attempting to compile this code results in this type error:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

To fix the error, we could ignore one or more of the values in the tuple using
`_` or `..`, as you’ll see in the [“Ignoring Values in a
Pattern”][ignoring-values-in-a-pattern]<!-- ignore --> section. If the problem
is that we have too many variables in the pattern, the solution is to make the
types match by removing variables so the number of variables equals the number
of elements in the tuple.

### Conditional if let Expressions

In Chapter 6, we discussed how to use `if let` expressions mainly as a shorter
way to write the equivalent of a `match` that only matches one case.
Optionally, `if let` can have a corresponding `else` containing code to run if
the pattern in the `if let` doesn’t match.

Listing 19-3 shows that it’s also possible to mix and match `if let`, `else
if`, and `else if let` expressions. Doing so gives us more flexibility than a
`match` expression in which we can express only one value to compare with the
patterns. Also, Rust doesn’t require that the conditions in a series of `if
let`, `else if`, and `else if let` arms relate to each other.

The code in Listing 19-3 determines what color to make your background based on
a series of checks for several conditions. For this example, we’ve created
variables with hardcoded values that a real program might receive from user
input.

<Listing number="19-3" file-name="src/main.rs" caption="Mixing `if let`, `else if`, `else if let`, and `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

If the user specifies a favorite color, that color is used as the background.
If no favorite color is specified and today is Tuesday, the background color is
green. Otherwise, if the user specifies their age as a string and we can parse
it as a number successfully, the color is either purple or orange depending on
the value of the number. If none of these conditions apply, the background
color is blue.

This conditional structure lets us support complex requirements. With the
hardcoded values we have here, this example will print `Using purple as the
background color`.

You can see that `if let` can also introduce new variables that shadow existing
variables in the same way that `match` arms can: the line `if let Ok(age) = age`
introduces a new `age` variable that contains the value inside the `Ok` variant,
shadowing the existing `age` variable. This means we need to place the `if age >
30` condition within that block: we can’t combine these two conditions into `if
let Ok(age) = age && age > 30`. The new `age` we want to compare to 30 isn’t
valid until the new scope starts with the curly bracket.

The downside of using `if let` expressions is that the compiler doesn’t check
for exhaustiveness, whereas with `match` expressions it does. If we omitted the
last `else` block and therefore missed handling some cases, the compiler would
not alert us to the possible logic bug.

### `while let` Conditional Loops

Similar in construction to `if let`, the `while let` conditional loop allows a
`while` loop to run for as long as a pattern continues to match. In Listing
19-4 we show a `while let` loop that waits on messages sent between threads,
but in this case checking a `Result` instead of an `Option`.

<Listing number="19-4" caption="Using a `while let` loop to print values for as long as `rx.recv()` returns `Ok`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

This example prints `1`, `2`, and then `3`. The `recv` method takes the first
message out of the receiver side of the channel and returns an `Ok(value)`. When
we first saw `recv` back in Chapter 16, we unwrapped the error directly, or
interacted with it as an iterator using a `for` loop. As Listing 19-4 shows,
though, we can also use while let, because the `recv` method returns an `Ok`
each time a message arrives, as long as the sender exists, and then produces an
`Err `once the sender side disconnects.

### `for` Loops

In a `for` loop, the value that directly follows the keyword `for` is a
pattern. For example, in `for x in y`, the `x` is the pattern. Listing 19-5
demonstrates how to use a pattern in a `for` loop to *destructure*, or break
apart, a tuple as part of the `for` loop.


<Listing number="19-5" caption="Using a pattern in a `for` loop to destructure a tuple">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

The code in Listing 19-5 will print the following:


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

We adapt an iterator using the `enumerate` method so it produces a value and
the index for that value, placed into a tuple. The first value produced is the
tuple `(0, 'a')`. When this value is matched to the pattern `(index, value)`,
`index` will be `0` and `value` will be `'a'`, printing the first line of the
output.

### Function Parameters

Function parameters can also be patterns. The code in Listing 19-6, which
declares a function named `foo` that takes one parameter named `x` of type
`i32`, should by now look familiar.

<Listing number="19-6" caption="A function signature uses patterns in the parameters">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

The `x` part is a pattern! As we did with `let`, we could match a tuple in a
function’s arguments to the pattern. Listing 19-7 splits the values in a tuple
as we pass it to a function.

<Listing number="19-7" file-name="src/main.rs" caption="A function with parameters that destructure a tuple">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

This code prints `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` is the value `3` and `y` is the value `5`.

We can also use patterns in closure parameter lists in the same way as in
function parameter lists because closures are similar to functions, as
discussed in Chapter 13.

At this point, you’ve seen several ways to use patterns, but patterns don’t
work the same in every place we can use them. In some places, the patterns must
be irrefutable; in other circumstances, they can be refutable. We’ll discuss
these two concepts next.

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern
