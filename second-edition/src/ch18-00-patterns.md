# Patterns

We've actually used patterns a few times so far: they're used in `let`
statements, in function parameters, and in the `match` expression. Patterns have
a lot more abilities than we have demonstrated so far, so we'll cover some of
the most commonly used ones in this section. Any of these abilities work in any
place where a pattern is used.

## `let` statements

A basic `let` statement has this form:

```text
let PATTERN = EXPRESSION;
```

We've seen statements like `let x = 5;` with a variable name in the `PATTERN`
slot; a variable name is just a particularly humble form of pattern.

Let’s try a more complex pattern. Change our example program to this:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let (x, y) = (5, 6);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

And run it with `cargo run`:

```text
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
     Running `target/debug/patterns`
The value of x is: 5
The value of y is: 6
```

We’ve created two variables with one `let`! Here’s our pattern:

```text
(x, y)
```

And here’s the value:

```text
(5, 6)
```

As you can see, the two line up visually, and so `let` binds the variable `x`
to the value `5` and `y` to `6`. We could have used two `let` statements as
well:

```rust
fn main() {
    let x = 5;
    let y = 6;
}
```

In simple cases like this, two `let`s may be clearer, but in others, creating
multiple variables at once is nice. As we become more proficient in Rust, we’ll
figure out which style is better, but it’s mostly a judgment call.

## Type annotations

Most of the time, Rust uses *type inference*, meaning that it attempts to infer
the types of your variables rather than you having to declare them explicitly
even though Rust is a statically typed language. Occasionally, Rust won't have
enough information to infer the type of your value, and you will need to add a
type annotation in with the pattern.

Here’s what a `let` statement with a *type annotation* looks like:

```rust
let x: i32 = 5;
```

We can add a colon, followed by the type name. Here’s the structure of a `let`
statement with a type annotation:

```text
let PATTERN: TYPE = VALUE;
```

Note that the colon and the `TYPE` go *after* the `PATTERN`, not in the pattern
itself. As an example, here’s our more complex pattern with two variables:

```rust
let (x, y): (i32, i32) = (5, 6);
```

Just like we match up the `VALUE` with the `PATTERN`, we match up the `TYPE`
with the `PATTERN`.

## Literals & _

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

# Multiple patterns

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

## ref and ref mut

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

## Destructuring

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

## Shadowing

As with all variables, those declared by a pattern will shadow variables
outside of the `match` construct:

```rust
let x = Some(5);

match x {
    Some(x) => { }, // x is an i32 here, not an Option<i32>
    None => (),
}
```

## Ignoring values

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
    (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
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

## Ranges

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

## Guards

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

This prints `no`, because the if applies to the whole of `4 | 5`, and not to only
the `5`. In other words, the precedence of if behaves like this:

```text
(4 | 5) if y => ...
```

not this:

```text
4 | (5 if y) => ...
```

## Bindings

You can bind values to names with `@`:
