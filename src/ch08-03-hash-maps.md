## Storing Keys with Associated Values in Hash Maps

The last of our common collections is the *hash map*. The type `HashMap<K, V>`
stores a mapping of keys of type `K` to values of type `V`. It does this via a
*hashing function*, which determines how it places these keys and values into
memory. Many programming languages support this kind of data structure, but
they often use a different name, such as hash, map, object, hash table,
dictionary, or associative array, just to name a few.

Hash maps are useful when you want to look up data not by using an index, as
you can with vectors, but by using a key that can be of any type. For example,
in a game, you could keep track of each team’s score in a hash map in which
each key is a team’s name and the values are each team’s score. Given a team
name, you can retrieve its score.

We’ll go over the basic API of hash maps in this section, but many more goodies
are hiding in the functions defined on `HashMap<K, V>` by the standard library.
As always, check the standard library documentation for more information.

### Creating a New Hash Map

You can create an empty hash map with `new` and add elements with `insert`. In
Listing 8-20, we’re keeping track of the scores of two teams whose names are
Blue and Yellow. The Blue team starts with 10 points, and the Yellow team
starts with 50.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

<span class="caption">Listing 8-20: Creating a new hash map and inserting some
keys and values</span>

Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values
must have the same type.

Another way of constructing a hash map is by using iterators and the `collect`
method on a vector of tuples, where each tuple consists of a key and its value.
We’ll be going into more detail about iterators and their associated methods in
the [”Processing a Series of Items with Iterators” section of Chapter
13][iterators]<!-- ignore -->. The `collect` method gathers data into a number
of collection types, including `HashMap`. For example, if we had the team names
and initial scores in two separate vectors, we could use the `zip` method to
create a vector of tuples where “Blue” is paired with 10, and so forth. Then we
could use the `collect` method to turn that vector of tuples into a hash map,
as shown in Listing 8-21.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

<span class="caption">Listing 8-21: Creating a hash map from a list of teams
and a list of scores</span>

The type annotation `HashMap<_, _>` is needed here because it’s possible to
`collect` into many different data structures and Rust doesn’t know which you
want unless you specify. For the parameters for the key and value types,
however, we use underscores, and Rust can infer the types that the hash map
contains based on the types of the data in the vectors. In Listing 8-21, the
key type will be `String` and the value type will be `i32`, just as the types
were in Listing 8-20.

### Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>

We aren’t able to use the variables `field_name` and `field_value` after
they’ve been moved into the hash map with the call to `insert`.

If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We’ll talk more about these issues in
the [“Validating References with
Lifetimes”][validating-references-with-lifetimes]<!-- ignore --> section in
Chapter 10.

### Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-23.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

<span class="caption">Listing 8-23: Accessing the score for the Blue team
stored in the hash map</span>

Here, `score` will have the value that’s associated with the Blue team, and the
result will be `Some(&10)`. The result is wrapped in `Some` because `get`
returns an `Option<&V>`; if there’s no value for that key in the hash map,
`get` will return `None`. The program will need to handle the `Option` in one
of the ways that we covered in Chapter 6.

We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

This code will print each pair in an arbitrary order:

```text
Yellow: 50
Blue: 10
```

### Updating a Hash Map

Although the number of keys and values is growable, each key can only have one
value associated with it at a time. When you want to change the data in a hash
map, you have to decide how to handle the case when a key already has a value
assigned. You could replace the old value with the new value, completely
disregarding the old value. You could keep the old value and ignore the new
value, only adding the new value if the key *doesn’t* already have a value. Or
you could combine the old value and the new value. Let’s look at how to do each
of these!

#### Overwriting a Value

If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-24 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

<span class="caption">Listing 8-24: Replacing a value stored with a particular
key</span>

This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.

#### Only Inserting a Value If the Key Has No Value

It’s common to check whether a particular key has a value and, if it doesn’t,
insert a value for it. Hash maps have a special API for this called `entry`
that takes the key you want to check as a parameter. The return value of the
`entry` method is an enum called `Entry` that represents a value that might or
might not exist. Let’s say we want to check whether the key for the Yellow team
has a value associated with it. If it doesn’t, we want to insert the value 50,
and the same for the Blue team. Using the `entry` API, the code looks like
Listing 8-25.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

<span class="caption">Listing 8-25: Using the `entry` method to only insert if
the key does not already have a value</span>

The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-25 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.

#### Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-26 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-26/src/main.rs:here}}
```

<span class="caption">Listing 8-26: Counting occurrences of words using a hash
map that stores words and counts</span>

This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. The
`or_insert` method actually returns a mutable reference (`&mut V`) to the value
for this key. Here we store that mutable reference in the `count` variable, so
in order to assign to that value, we must first dereference `count` using the
asterisk (`*`). The mutable reference goes out of scope at the end of the `for`
loop, so all of these changes are safe and allowed by the borrowing rules.

### Hashing Functions

By default, `HashMap` uses a “cryptographically strong”[^siphash] hashing
function that can provide resistance to Denial of Service (DoS) attacks. This
is not the fastest hashing algorithm available, but the trade-off for better
security that comes with the drop in performance is worth it. If you profile
your code and find that the default hash function is too slow for your
purposes, you can switch to another function by specifying a different
*hasher*. A hasher is a type that implements the `BuildHasher` trait. We’ll
talk about traits and how to implement them in Chapter 10. You don’t
necessarily have to implement your own hasher from scratch;
[crates.io](https://crates.io/) has libraries shared by other Rust users that
provide hashers implementing many common hashing algorithms.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Summary

Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:

* Given a list of integers, use a vector and return the mean (the average
  value), median (when sorted, the value in the middle position), and mode (the
  value that occurs most often; a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to
  the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
  that start with a vowel have “hay” added to the end instead (“apple” becomes
  “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in a company. For example, “Add Sally to
  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.

The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!

[iterators]: ch13-02-iterators.html
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
