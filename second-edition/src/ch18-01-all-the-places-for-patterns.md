## All the Places Patterns May be Used

Patterns pop up in a number of places in Rust. You’ve been using them a lot
without realizing it! This section is a reference to all the places where
patterns are valid.

### `match` Arms

As we discussed in Chapter 6, a common place patterns are used is in the arms
of `match` expressions. Formally, `match` expressions are defined as the
keyword `match`, a value to match on, and one or more match arms that consist
of a pattern and an expression to run if the value matches that arm’s pattern:

```text
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

Note that `if let` can also introduce shadowed variables like `match` arms can:
`if let Ok(age) = age` introduces a new shadowed `age` variable that contains
the value inside the `Ok` variant. This also means the `if age > 30` condition
needs to go within the block; we aren’t able to combine these two conditions
into `if let Ok(age) = age && age > 30` since the shadowed `age` that we want
to compare to 30 isn’t valid until the new scope starts with the curly bracket.

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

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

<span class="caption">Listing 18-2: Using a `while let` loop to print out values
as long as `stack.pop()` returns `Some`</span>

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

```rust
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

<span class="caption">Listing 18-3: Using a pattern in a `for` loop to
destructure the tuple returned from `enumerate` into its pieces</span>

This will print:

```text
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

```rust
let x = 5;
```

We’ve done this hundreds of times throughout this book. You may not have
realized it, but you were using patterns! A `let` statement looks like this,
more formally:

```text
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

```rust
let (x, y, z) = (1, 2, 3);
```

<span class="caption">Listing 18-4: Using a pattern to destructure a tuple and
create 3 variables at once</span>

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

```rust
fn foo(x: i32) {
    // code goes here
}
```

<span class="caption">Listing 18-5: A function signature uses patterns in the
parameters</span>

The `x` part is a pattern! In a similar way as we did with `let`, we could
match a tuple in a function’s arguments. Listing 18-6 shows how we could split
apart the values in a tuple as part of passing the tuple to a function:

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

<span class="caption">Listing 18-6: A function with parameters that destructure
a tuple</span>

This will print `Current location: (3, 5)`. When we pass the value `&(3, 5)` to
`print_coordinates`, the values match the pattern `&(x, y)`. `x` gets the value
3, and `y` gets the value 5.

Because closures are similar to functions, as we discussed in Chapter 13, we
can use patterns in closure parameter lists as well.

One difference between the places we can use patterns is that with `for` loops,
`let`, and in function parameters, the patterns must be *irrefutable*. Let’s
discuss that next.
