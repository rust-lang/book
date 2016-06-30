# Patterns

We've mentioned 'patterns' a few times so far: they're used in `let` bindings,
in function arguments, and in `match` expressions. Patterns have a lot of
abilities, so in this section, we'll cover some of the most commonly used ones.
Any of these abilities work in any place where a pattern is used.

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

Usually, when you match against a pattern, bindings are bound by value.
This means you'll end up moving the value out:

```rust,ignore
let name = Some(String::from("Bors"));

match name {
    Some(name) => println!("Found a name: {}", name),
    None => (),
}

// name is moved here. This line will fail to compile:
println!("name is: {:?}", name);
```

If you'd prefer to bind `name` by reference, use the `ref` keyword:

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
```

## Ignoring bindings

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
fn main() {
    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
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

Whew! That’s a lot of different ways to match things. Let's cover one more place
you can use your newfound knowledge of patterns: `if let`.
