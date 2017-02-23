# Patterns

One of Rust's more subtle features is the simple *pattern*. Patterns let us
pick apart complex strucutres and do all kinds of fun things with them. In this
chapter, we'll start off by showing you some very basic patterns, and then all
of the places that you can use patterns. Finally, we'll have a reference-like
elaboration of all of the different kinds of patterns, with examples to show
you how to use them.

## What are patterns?

Patterns are a special syntax within Rust for matching against the structure of
your types, complex or simple. We take some value, compare it against the
pattern, and then do something with it. For example, consider this simple
variable assignment with `let`:

```rust
let x = 5;
```

We've done this hundreds of times throughout this book. Well, you didn't
realize it, but you were using patterns! A `let` statement looks like this,
gramatically:

```text
let PATTERN = EXPRESSION;
```

We've seen statements like `let x = 5;` with a variable name in the `PATTERN`
slot; a variable name is just a particularly humble form of pattern.

With `let`, we compare the expression against the pattern, and assign any
names we find. So for example, in our `let x = 5;` case, `x` is a pattern
that says "bind what matches here to the variable `x`. And since that's the
whole pattern, it effecitvely means "bind everything to the variable `x`."

To see this a bit more clearly, consider this code:

```rust
let (x, y, z) = (1, 2, 3);
```

Here, we have a tuple that we're matching against a pattern. Rust will compare
the value `(1, 2, 3)` to the pattern `(x, y, z)`, and see that it's valid. In
this case, it will bind `x` to `1`, `y` to `2`, and `z` to `3`.

We can mix and match and nest patterns: you can think of this tuple pattern as
nesting three individual variable patterns inside of it. Or something like:

```rust
let ((one, two), name, (three, four), five) = ((1, 2), "hello", (3, 4), 5);
```

Where we nest tons of things inside of each other. 

## Refutability

Patterns come in two forms: refutable, and irrefutable. Patterns which cannot
fail to match are "irrefutable", and patterns which can fail to match are said
to be "refutable".

Consider our `let x = 5;` example. `let` takes an irrefutable pattern, and this
is true for `x`: since it matches anything, it can't fail to match. Consider
trying to match an enum with `let`, something like

```rust,ignore
let Some(x) = some_option_value;
```

This can't work, and Rust will complain:

```text
error[E0005]: refutable pattern in local binding: `None` not covered
 --> <anon>:3:5
  |
3 | let Some(x) = some_option_value;
  |     ^^^^^^^ pattern `None` not covered
```

We didn't cover (and couldn't cover!) every valid option here, and so Rust will
rightfully complain. What should it have done with the `None` case?

On the other hand, `if let` takes a refutable pattern:

```rust,ignore
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

And if you give it an irrefutable one...

```rust
if let x = 5 {
    println!("{}", x);
};
```

...it will complain:

```text
error[E0162]: irrefutable if-let pattern
 --> <anon>:2:8
  |
2 | if let x = 5 {
  |        ^ irrefutable pattern
```

In general, you shouldn't have to worry about this distinction; just be
familliar with the word when you see it, and realize that you need to change
either the pattern, or the construct you're using the pattern with.

## Where can I use patterns?

Patterns pop up in a number of places in Rust. You've been using them a lot
without realizing it!

### `let` statements

We talked about this above, the basic grammar of a `let` statement is:

```text
let PATTERN = EXPRESSION;
```

Many people don't realize that you can use any patterns on the left hand side
of a `let` statement, not just binding patterns. You'll see more examples of
this later in this chapter.

`let`, as mentioned above as well, takes an irrefutable pattern.

### `match` arms

Patterns are used very heavily by `match` expressions, particularly in match
arms.

```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

These patterns are refuatble. However...

#### Exhaustiveness and `_`

`match` expressions are required to be exhaustive. So when we put all of the
patterns together, they must cover all of our bases. One way to ensure you have
every possibility covered is to introduce an irrefutable pattern, like a
binding. Since it is irrefutable, it can never fail, and so covers every case
by definition.

There's an additional irrefutable pattern that's often used in this case
though: `_`. It matches anything, but it never binds any variables. This can be
useful when you only want to do things for some patterns, but ignore the rest,
for example.

#### Shadowing in patterns

As with all variables, those declared by a pattern will shadow variables
outside of the `match` construct:

```rust
let x = Some(5);

match x {
    Some(x) => { }, // x is an i32 here, not an Option<i32>
    None => (),
}
```

### Function Arguments

Similarly to `let`, function arguments are also irrefutable patterns:

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
not have to use the argument.

## Kinds of pattern

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

### ref and ref mut

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

This prints `no`, because the if applies to the whole of `4 | 5`, and not to only
the `5`. In other words, the precedence of if behaves like this:

```text
(4 | 5) if y => ...
```

not this:

```text
4 | (5 if y) => ...
```

### Bindings

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
