# Strings

We've already talked about strings a bunch in chapter four, but let's take a
more in-depth look at them now.

## Creating

You can create an empty string with `new`:

```rust
let s = String::new();
```

Often, you have some initial data that you'd like to start the string off with.
For that, there's the `.to_string()` method:

```rust
let data = "initial contents";

let s = data.to_string();
```

## Reading

slicing syntax, indexing

## Updating

push_str, concatenation
