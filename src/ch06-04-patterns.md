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
    // name is moved here because of the binding to the `Some` value.
    Some(inner_name) => println!("Found a name: {}", inner_name),
    None => (),
}

// This line will fail to compile:
println!("name is: {:?}", name);
```

If you'd prefer to bind `name` by reference, use the `ref` keyword in order to
borrow the value instead:

```rust
let name = Some(String::from("Bors"));

match name {
    // name is not moved here.
    Some(ref inner_name) => println!("Found a name: {}", inner_name),
    None => (),
}

// The match only took a reference to its data rather than moving it.
// This works:
println!("name is: {:?}", name);
```

And for a mutable reference, use `ref mut`:

```rust
let mut name = Some(String::from("Bors"));

match name {
    // name is not moved here.
    Some(ref mut inner_name) => *inner_name = String::from("Another name"),
    None => (),
}

// The match only took a reference to its data rather than moving it.
// This works and prints the new value we gave it in the match statement:
println!("name is: {:?}", name);
```

## Ignoring bindings

We discussed using `_` as a whole pattern to ignore any value, but you can
also use `_` inside of another pattern to ignore just part of a value. This
usage of `_` will ignore the inner value of any `Some` value that is not a
`Some` with a `6` inside:

```rust
let x = Some(5);

match x {
    Some(6) => println!("got a Some(6)"),
    Some(_) => println!("got a Some and I don't care what's inside"),
    None => (),
}
```

It’s worth noting that using `_` never binds to the value, which means that the
value will not be moved:

```rust
let name = Some(String::from("Bors"));

match name {
    // name is not moved here because the _ does not bind to the `Some` value.
    Some(_) => println!("Found a name!"),
    None => (),
}

// This works:
println!("name is: {:?}", name);
```

## Guards

You can introduce "match guards" with `if`. This adds an extra condition that
often uses a value that the pattern has bound to:

```rust
let x = Some(5);

match x {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

In this case, we bound the inner value of a `Some` to `x` and then "guarded" the
first match arm with an additional condition that `x` must be less than 5. In
this case, `Some(5)` does not have an inner value that is less than 5, so this
code will just print out `5`.

Whew! That’s a lot of different ways to match things. Let's cover one more place
you can use your newfound knowledge of patterns: `if let`.
