## Storing Lists of Values with Vectors

The first collection type we’ll look at is `Vec<T>`, also known as a *vector*.
Vectors allow you to store more than one value in a single data structure that
puts all the values next to each other in memory. Vectors can only store values
of the same type. They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.

### Creating a New Vector

To create a new, empty vector, we can call the `Vec::new` function, as shown in
Listing 8-1.

```rust
let v: Vec<i32> = Vec::new();
```

<span class="caption">Listing 8-1: Creating a new, empty vector to hold values
of type `i32`</span>

Note that we added a type annotation here. Because we aren’t inserting any
values into this vector, Rust doesn’t know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we’ll cover how to use generics with your own types in Chapter 10. For now,
know that the `Vec<T>` type provided by the standard library can hold any type,
and when a specific vector holds a specific type, the type is specified within
angle brackets. In Listing 8-1, we’ve told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.

In more realistic code, Rust can often infer the type of value you want to
store once you insert values, so you rarely need to do this type annotation.
It’s more common to create a `Vec<T>` that has initial values, and Rust
provides the `vec!` macro for convenience. The macro will create a new vector
that holds the values you give it. Listing 8-2 creates a new `Vec<i32>` that
holds the values `1`, `2`, and `3`.

```rust
let v = vec![1, 2, 3];
```

<span class="caption">Listing 8-2: Creating a new vector containing
values</span>

Because we’ve given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn’t necessary. Next, we’ll look at how
to modify a vector.

### Updating a Vector

To create a vector and then add elements to it, we can use the `push` method,
as shown in Listing 8-3.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

<span class="caption">Listing 8-3: Using the `push` method to add values to a
vector</span>

As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don’t need the `Vec<i32>` annotation.

### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-4.

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```

<span class="caption">Listing 8-4: Showing where the vector and its elements
are dropped</span>

When the vector gets dropped, all of its contents are also dropped, meaning
those integers it holds will be cleaned up. This may seem like a
straightforward point but can get a bit more complicated when you start to
introduce references to the elements of the vector. Let’s tackle that next!

### Reading Elements of Vectors

Now that you know how to create, update, and destroy vectors, knowing how to
read their contents is a good next step. There are two ways to reference a
value stored in a vector. In the examples, we’ve annotated the types of the
values that are returned from these functions for extra clarity.

Listing 8-5 shows both methods of accessing a value in a vector, either with
indexing syntax or the `get` method.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

<span class="caption">Listing 8-5: Using indexing syntax or the `get` method to
access an item in a vector</span>

Note two details here. First, we use the index value of `2` to get the third
element: vectors are indexed by number, starting at zero. Second, the two ways
to get the third element are by using `&` and `[]`, which gives us a reference,
or by using the `get` method with the index passed as an argument, which gives
us an `Option<&T>`.

Rust has two ways to reference an element so you can choose how the program
behaves when you try to use an index value that the vector doesn’t have an
element for. As an example, let’s see what a program will do if it has a vector
that holds five elements and then tries to access an element at index 100, as
shown in Listing 8-6.

```rust,should_panic,panics
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

<span class="caption">Listing 8-6: Attempting to access the element at index
100 in a vector containing five elements</span>

When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there’s an attempt to access an element past the
end of the vector.

When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector happens occasionally under normal circumstances.
Your code will then have logic to handle having either `Some(&element)` or
`None`, as discussed in Chapter 6. For example, the index could be coming from
a person entering a number. If they accidentally enter a number that’s too
large and the program gets a `None` value, you could tell the user how many
items are in the current vector and give them another chance to enter a valid
value. That would be more user-friendly than crashing the program due to a typo!

When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure this reference
and any other references to the contents of the vector remain valid. Recall the
rule that states you can’t have mutable and immutable references in the same
scope. That rule applies in Listing 8-7, where we hold an immutable reference to
the first element in a vector and try to add an element to the end, which won’t
work.

```rust,ignore,does_not_compile
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

<span class="caption">Listing 8-7: Attempting to add an element to a vector
while holding a reference to an item</span>

Compiling this code will result in this error:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/main.rs:10:5
   |
8  |     let first = &v[0];
   |                  - immutable borrow occurs here
9  |
10 |     v.push(6);
   |     ^^^^^^^^^ mutable borrow occurs here
11 |
12 |     println!("The first element is: {}", first);
   |                                          ----- borrow later used here
```

The code in Listing 8-7 might look like it should work: why should a reference
to the first element care about what changes at the end of the vector? This
error is due to the way vectors work: adding a new element onto the end of the
vector might require allocating new memory and copying the old elements to the
new space, if there isn’t enough room to put all the elements next to each
other where the vector currently is. In that case, the reference to the first
element would be pointing to deallocated memory. The borrowing rules prevent
programs from ending up in that situation.

> Note: For more on the implementation details of the `Vec<T>` type, see “The
> Rustonomicon” at https://doc.rust-lang.org/stable/nomicon/vec.html.

### Iterating over the Values in a Vector

If we want to access each element in a vector in turn, we can iterate through
all of the elements rather than use indexes to access one at a time. Listing
8-8 shows how to use a `for` loop to get immutable references to each element
in a vector of `i32` values and print them.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

<span class="caption">Listing 8-8: Printing each element in a vector by
iterating over the elements using a `for` loop</span>

We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-9
will add `50` to each element.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

<span class="caption">Listing 8-9: Iterating over mutable references to
elements in a vector</span>

To change the value that the mutable reference refers to, we have to use the
dereference operator (`*`) to get to the value in `i` before we can use the
`+=` operator. We’ll talk more about `*` in Chapter 15.

### Using an Enum to Store Multiple Types

At the beginning of this chapter, we said that vectors can only store values
that are the same type. This can be inconvenient; there are definitely use
cases for needing to store a list of items of different types. Fortunately, the
variants of an enum are defined under the same enum type, so when we need to
store elements of a different type in a vector, we can define and use an enum!

For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and then all the enum variants will be considered the same type:
that of the enum. Then we can create a vector that holds that enum and so,
ultimately, holds different types. We’ve demonstrated this in Listing 8-10.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

<span class="caption">Listing 8-10: Defining an `enum` to store values of
different types in one vector</span>

Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. A
secondary advantage is that we can be explicit about what types are allowed in
this vector. If Rust allowed a vector to hold any type, there would be a chance
that one or more of the types would cause errors with the operations performed
on the elements of the vector. Using an enum plus a `match` expression means
that Rust will ensure at compile time that every possible case is handled, as
discussed in Chapter 6.

When you’re writing a program, if you don’t know the exhaustive set of types
the program will get at runtime to store in a vector, the enum technique won’t
work. Instead, you can use a trait object, which we’ll cover in Chapter 17.

Now that we’ve discussed some of the most common ways to use vectors, be sure
to review the API documentation for all the many useful methods defined on
`Vec<T>` by the standard library. For example, in addition to `push`, a `pop`
method removes and returns the last element. Let’s move on to the next
collection type: `String`!
