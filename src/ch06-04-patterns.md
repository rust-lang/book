# Patterns

We've mentioned 'patterns' a few times so far: they're used in `let` bindings,
in function arguments, and in `match` expressions. Patterns have a lot of
abilities, so in this section, we'll cover some of the most commonly used ones.
Any of these abilities work in any place where a pattern is used.

Let's start with an example that is similar to the last example in the previous
section:

```rust
let x = 1;

match x {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => println!("anything else"),
}
```

This prints `one`. If we change `x` to have the value 4, this would print
`anything else`.

# Multiple patterns

What if we wanted to print the same thing for 1, 3, and 5? We could do:

```rust
let x = 1;

match x {
    1 => println!("an odd number less than six"),
    3 => println!("an odd number less than six"),
    5 => println!("an odd number less than six"),
    7 => println!("seven"),
    _ => println!("anything else"),
}
```

But that repeats the string "an odd number less than six" multiple times. If we
had to change that string, it would be annoying to have to change it in three
places to make 1, 3, and 5 still have the same behavior.

Instead, we could match multiple patterns with `|`:

```rust
let x = 1;

match x {
    1 | 3 | 5 => println!("an odd number less than six"),
    7 => println!("seven"),
    _ => println!("anything else"),
}
```

This match statement has the same functionality as the previous one, but we only
had to write the common println! statement once!

## Ranges

Another way to have multiple values match the same arm is using a range. If,
instead of the above where we treated 1, 3, and 5 the same, we wanted to treat
any number from 1 to 5 the same, we could do:

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything else"),
}
```

This prints `one through five`: 5 is included in the range.

Ranges are usually used with integers or `char`s:

```rust
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

This prints `early ASCII letter`.

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
