## Hash Maps

The last of our fundamental collections is the *hash map*. The type `HashMap<K,
V>` stores a mapping of keys of type `K` to values of type `V`. It does this
via a *hashing function*, which determines how it places these keys and values
into memory. Many different programming languges support this kind of data
structure, but often with a different name: hash, map, object, hash table, or
associative array, just to name a few.

We'll go over the basic API in this chapter, but there are many more goodies
hiding in the functions defined on `HashMap` by the standard library. As always,
check the standard library documentation for more information.

### Creating a New Hash Map

We can create an empty `HashMap` with `new`, and add elements with `insert`:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");
```

Note that we need to `use` the `HashMap` from the collections portion of the
standard library. Of our three fundamental collections, this one is the least
often used, so it has a bit less support from the language. There's no built-in
macro to construct them, for example, and they're not in the prelude, so we
need to add a `use` statement for them.

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `i32` and values of type `&str`. Like vectors, hash maps are
homogenous: all of the keys must have the same type, and all of the values must
have the same type.

If we have a vector of tuples, we can convert it into a hash map with the
`collect` method. The first element in each tuple will be the key, and the
second element will be the value:

```rust
use std::collections::HashMap;

let data = vec![(1, "hello"), (2, "world")];

let map: HashMap<_, _> = data.into_iter().collect();
```

The type annotation `HashMap<_, _>` is needed here because it's possible to
`collect` into many different data structures, so Rust doesn't know which we
want. For the type parameters for the key and value types, however, we can use
underscores and Rust can infer the types that the hash map contains based on the
types of the data in our vector.

For types that implement the `Copy` trait like `i32` does, the values are
copied into the hash map. If we insert owned values like `String`, the values
will be moved and the hash map will be the owner of those values:

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point
```

We would not be able to use the bindings `field_name` and `field_value` after
they have been moved into the hash map with the call to `insert`.

If we insert references to values, the values themselves will not be moved into
the hash map. The values that the references point to must be valid for at least
as long as the hash map is valid, though. We will talk more about these issues
in the Lifetimes section of Chapter 10.

### Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the `get` method:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");

let value = map.get(&2);
```

Here, `value` will have the value `Some("world")`, since that's the value
associated with the `2` key. "world" is wrapped in `Some` because `get` returns
an `Option<V>`. If there's no value for that key in the hash map, `get` will
return `None`.

We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(2, "world");

for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

This will print:

```bash
1: hello
2: world
```

### Updating a Hash Map

Since each key can only have one value, when we want to change the data in a
hash map, we have to decide how to handle the case when a key already has a
value assigned. We could choose to replace the old value with the new value. We
could choose to keep the old value and ignore the new value, and only add the
new value if the key *doesn't* already have a value. Or we could change the
existing value. Let's look at how to do each of these!

#### Overwriting a Value

If we insert a key and a value, then insert that key with a different value,
the value associated with that key will be replaced. Even though this code
calls `insert` twice, the hash map will only contain one key/value pair, since
we're inserting with the key `1` both times:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");
map.insert(1, "Hi There");

println!("{:?}", map);
```

This will print `{1: "Hi There"}`.

#### Only Insert If the Key Has No Value

It's common to want to see if there's some sort of value already stored in the
hash map for a particular key, and if not, insert a value. hash maps have a
special API for this, called `entry`, that takes the key we want to check as an
argument:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(1, "hello");

let e = map.entry(2);
```

Here, the value bound to `e` is a special enum, `Entry`. An `Entry` represents a
value that might or might not exist. Let's say that we want to see if the key
`2` has a value associated with it. If it doesn't, we want to insert the value
"world". In both cases, we want to return the resulting value that now goes
with `2`. With the entry API, it looks like this:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

map.insert(1, "hello");

map.entry(2).or_insert("world");
map.entry(1).or_insert("Hi There");

println!("{:?}", map);
```

The `or_insert` method on `Entry` does exactly this: returns the value for the
`Entry`'s key if it exists, and if not, inserts its argument as the new value
for the `Entry`'s key and returns that. This is much cleaner than writing the
logic ourselves, and in addition, plays more nicely with the borrow checker.

This code will print `{1: "hello", 2: "world"}`. The first call to `entry` will
insert the key `2` with the value "world", since `2` doesn't have a value
already. The second call to `entry` will not change the hash map since `1`
already has the value "hello".

#### Update a Value Based on the Old Value

Another common use case for hash maps is to look up a key's value then update
it, using the old value. For instance, if we wanted to count how many times
each word appeared in some text, we could use a hash map with the words as keys
and increment the value to keep track of how many times we've seen that word.
If this is the first time we've seen a word, we'll first insert the value `0`.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

This will print `{"world": 2, "hello": 1, "wonderful": 1}`. The `or_insert`
method actually returns a mutable reference (`&mut V`) to the value in the
hash map for this key. Here we store that mutable reference in the `count`
variable binding, so in order to assign to that value we must first dereference
`count` using the asterisk (`*`). The mutable reference goes out of scope at
the end of the `for` loop, so all of these changes are safe and allowed by the
borrowing rules.

### Hashing Function

By default, `HashMap` uses a cryptographically secure hashing function that can
provide resistance to Denial of Service (DoS) attacks. This is not the fastest
hashing algorithm out there, but the tradeoff for better security that comes
with the drop in performance is a good default tradeoff to make. If you profile
your code and find that the default hash function is too slow for your
purposes, you can switch to another function by specifying a different
*hasher*. A hasher is an object that implements the `BuildHasher` trait. We'll
be talking about traits and how to implement them in Chapter 10.

## Summary

Vectors, strings, and hash maps will take you far in programs where you need to
store, access, and modify data. Some programs you are now equipped to write and
might want to try include:

* Given a list of integers, use a vector and return their mean (average),
  median (when sorted, the value in the middle position), and mode (the value
  that occurs most often; a hash map will be helpful here).
* Convert strings to Pig Latin, where the first consonant of each word gets
  moved to the end with an added "ay", so "first" becomes "irst-fay". Words that
  start with a vowel get an h instead ("apple" becomes "apple-hay"). Remember
  about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in the company. For example, "Add Sally to
  Engineering" or "Add Ron to Sales". Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.

The standard library API documentation describes methods these types have that
will be helpful for these exercises!

We're getting into more complex programs where operations can fail, which means
it's a perfect time to go over error handling next!
