# `HashMap<K, V>`

The last of our essential collections is the hashmap. A `HashMap<K, V>`
stores a mapping of keys to values. It does this via a "hashing function",
which determines how it places these keys and values into memory. Many
different programming languges support this kind of data structure, but
often with a different name: a "hash", a "map", an "object", a "hash table",
an "associative array"... so many names.

We'll go over the basic API in this chapter, but there's many more goodies
hiding in there. As always, check the standard library docs for more fun
things to do with hashmaps.

## creating

We can create an empty hashmap with `new`, and add things with `insert`:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");
```

You'll note that we need to `use` the `HashMap` from the collections portion of
the standard library. Of our three essential collections, this one is the least
often used, and so has a bit less support from the language. There's no built-in
macro to construct them, for example, and they're not in the prelude, and so need
to be `use`d directly.

## reading

We can get a value out of the hashmap by providing the proper key to the `get`
method:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");

let value = map.get(&2);
```

Here, `value` will have the value `Some("world")`, since that's the value
associated with the `2` key. It's wrapped in `Some` because `get` returns an
`Option<V>`, that is, if there's no value for that key, it will return `None`.

## updating

It's common to want to see if there's some sort of value already stored in the hashmap,
and if not, insert something. Hashmaps have a special API for this, called `entry`:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");

let e = map.entry(2);
```

Here, `e` is a special enum, `Entry`. An entry represents a value that might
exist, or might not. Let's say that we want to see if the key `2` has a value
associated with it.  If it doesn't, we want to insert `"world"`, and then in
both cases, return the value. With the entry API, it looks like this:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");

let e = map.entry(2).or_insert("world");
```

The `or_insert` method on entry does exactly this: returns the value, and if
not, inserts its argument and returns that. This is much cleaner than writing
the logic yourself, and in addition, plays more nicely with the borrow checker.
