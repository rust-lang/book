
[TOC]

# Understanding Ownership

Ownership is important to understand: it's Rust's most unique feature, and
enables Rust to make memory safety guarantees without needing a garbage
collector. We’ll also talk about several related features: borrowing, slices,
and how Rust lays things out in memory.

## Ownership

Rust’s central feature is called *ownership*. It is a feature that is
straightforward to explain, but has deep implications for the rest of the
language.

All programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection, while in others, the programmer has to
explicitly allocate and free the memory. Rust takes a third approach: memory is
managed through a system of ownership with a set of rules that the compiler
checks at compile-time. You do not pay any run-time cost for any of these
features.

However, because ownership is a new concept for many programmers, it does take
some time to get used to. There is good news, though: the more experienced you
become with Rust and the rules of the ownership system, the more you'll be
able to naturally develop code that is both safe and efficient. Keep at it!

Once you understand ownership, you have a good foundation for understanding the
features that make Rust unique. In this chapter, we'll learn ownership by going
through some examples, focusing on a very common data structure: strings.

### Variable binding scope

We've walked through an example of a Rust program already in the tutorial
chapter. Now that we’re past basic syntax, we won’t include all of the `fn
main() {` stuff in examples, so if you’re following along, you will have to put
them inside of a `main()` function. This lets our examples be a bit more
concise, letting us focus on the actual details rather than boilerplate.

Anyway, here is our first example:

```rust
let s = "hello";
```

This variable binding refers to a string literal, where the value of the string
is hard coded into the text of our program. The binding is valid from the point
at which it’s declared until the end of the current _scope_. That is:

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

In other words, there are two important points in time here:

- When `s` comes "into scope", it is valid.
- It remains so until it "goes out of scope".

At this point, things are similar to other programming languages. Now let’s
build on top of this understanding by introducing the `String` type.

### Strings

String literals are convenient, but they aren’t the only way that you use
strings. For one thing, they’re immutable. For another, not every string is
literal: what about taking user input and storing it in a string?

For this, Rust has a second string type, `String`. You can create a `String`
from a string literal using the `from` function:

```rust
let s = String::from("hello");
```

We haven’t seen the double colon (`::`) syntax yet. It is an operator that
allows us to namespace this particular `from()` function under the `String`
type itself, rather than using some sort of name like `string_from()`. We’ll
discuss this syntax more in the “Method Syntax” and “Modules” chapters.

This kind of string can be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```

### Memory and allocation

So, what’s the difference here? Why can `String` be mutated, but literals
cannot? The difference comes down to how these two types deal with memory.

In the case of a string literal, because we know the contents of the string at
compile time, we can hard-code the text of the string directly into the final
executable. This means that string literals are quite fast and efficient. But
these properties only come from its immutability. Unfortunately, we can’t put a
blob of memory into the binary for each string whose size is unknown at compile
time and whose size might change over the course of running the program.

With `String`, in order to support a mutable, growable string, we need to
allocate an unknown amount of memory to hold the contents. This means two
things:

1. The memory must be requested from the operating system at runtime.
2. We need a way of giving this memory back to the operating system when we’re
   done with our `String`.

That first part is done by us: when we call `String::from()`, its
implementation requests the memory it needs. This is pretty much universal in
programming languages.

The second case, however, is different. In languages with a *garbage collector*
(*GC*), the GC will keep track and clean up memory that isn't being used
anymore, and we, as the programmer, don’t need to think about it. Without GC,
it’s our responsibility to identify when memory is no longer being used and
call code to explicitly return it, just as we did to request it. Doing this
correctly has historically been a difficult problem. If we forget, we will
waste memory. If we do it too early, we will have an invalid variable. If we do
it twice, that’s a bug too. We need to pair exactly one `allocate()` with
exactly one `free()`.

Rust takes a different path. Remember our example? Here’s a version with
`String`:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid
```

We have a natural point at which we can return the memory our `String` needs
back to the operating system: when it goes out of scope. When a variable goes
out of scope, Rust calls a special function for us. This function is called
`drop()`, and it is where the author of `String` can put the code to return the
memory.

> Aside: This pattern is sometimes called “Resource Acquisition Is
> Initialization” in C++, or “RAII” for short. While they are very similar,
> Rust’s take on this concept has a number of differences, and so we don’t tend
> to use the same term. If you’re familiar with this idea, keep in mind that it
> is _roughly_ similar in Rust, but not identical.

This pattern has a profound impact on the way that Rust code is written. It may
seem obvious right now, but things can get tricky in more advanced situations.
Let’s go over the first one of those right now.

### Move

What would you expect this code to do?

```rust
let x = 5;
let y = x;
```

You might say “Make a copy of `5`”, and that would be correct. We now have two
bindings, `x` and `y`, and both equal `5`.

Now let’s look at `String`. What would you expect this code to do?

```rust
let s1 = String::from("hello");
let s2 = s1;
```

You might say “copy the `String`!” This is both correct and incorrect at the
same time. It does a _shallow_ copy of the `String`. What’s that mean? Well,
let’s take a look at what `String` looks like under the covers:

<img alt="string" src="img/foo1.png" class="center" style="width: 50%;" />

A `String` is made up of three parts: a pointer to the memory that holds the
contents of the string, a length, and a capacity. The length is how much memory
the `String` is currently using. The capacity is the total amount of memory the
`String` has gotten from the operating system. The difference between length
and capacity matters but not in this context, so don’t worry about it too much.
For right now, it's fine to ignore the capacity.

When we assign `s1` to `s2`, the `String` itself is copied, meaning we copy the
pointer, the length, and the capacity. We do not copy the data that the
`String`'s pointer refers to. In other words, it looks like this:

<img alt="s1 and s2" src="img/foo2.png" class="center" style="width: 50%;" />

_Not_ this:

<img alt="s1 and s2 to two places" src="img/foo4.png" class="center" style="width: 50%;" />

There’s a problem here. Both data pointers are pointing to the same place. Why
is this a problem? Well, when `s2` goes out of scope, it will free the memory
that the pointer points to. And then `s1` goes out of scope, and it will _also_
try to free the memory that the pointer points to. That’s bad, and is known as
a "double free" error.

So what’s the solution? Here, we stand at a crossroads with a few options.

One way would be to change assignment so that it will also copy out any data.
This works, but is inefficient: what if our `String` contained a novel?
Also, that solution would only work for memory. What if, instead of a `String`,
we had a `TcpConnection`? Opening and closing a network connection is very
similar to allocating and freeing memory, so it would be nice to be able to use
the same mechanism. We wouldn't be able to, though, because creating a new
connection requires more than just copying memory: we have to request a new
connection from the operating system. We could then extend our solution to
allow the programmer to hook into the assignment, similar to `drop()`, and
write code to fix things up. That would work, but if we did that, an `=` could
run arbitrary code. That’s also not good, and it doesn’t solve our efficiency
concerns either.

Let’s take a step back: the root of the problem is that `s1` and `s2` both
think that they have control of the memory and therefore need to free it.
Instead of trying to copy the allocated memory, we could say that `s1` is no
longer valid and, therefore, doesn’t need to free anything. This is in fact the
choice that Rust makes. Check out what happens when you try to use `s1` after
`s2` is created:

```rust,ignore
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);
```

You’ll get an error like this:

```bash
5:22 error: use of moved value: `s1` [E0382]
println!("{}", s1);
               ^~
5:24 note: in this expansion of println! (defined in <std macros>)
3:11 note: `s1` moved here because it has type `collections::string::String`, which is moved by default
 let s2 = s1;
     ^~
```

If you have heard the terms "shallow copy" and "deep copy" while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data probably sounded like a shallow copy. Because Rust
also invalidates the first binding, instead of calling this a shallow copy,
it's called a _move_. Here we would read this by saying that `s1` was _moved_
into `s2`. So what actually happens looks like this:

<img alt="s1 and s2 to the same place" src="img/foo3.png" class="center" style="width: 50%;" />

That solves our problem! With only `s2` valid, when it goes out of scope, it
alone will free the memory, and we’re done.

### Ownership Rules

This leads us to the Ownership Rules:

> 1. Each value in Rust has a variable binding that’s called its *owner*.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be `drop()`ped.

Furthermore, there’s a design choice that’s implied by this: Rust will never
automatically create "deep" copies of your data. Therefore, any _automatic_
copying can be assumed to be inexpensive.

### Clone

But what if we _do_ want to deeply copy the `String`’s data and not just the
`String` itself? There’s a common method for that: `clone()`. We will discuss
methods in the section on `structs` in Chapter XX, but they’re a
common enough feature in many programming languages that you have probably seen
them before.

Here’s an example of the `clone()` method in action:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("{}", s1);
```

This will work just fine. Remember our diagram from before? In this case,
it _is_ doing this:

<img alt="s1 and s2 to two places" src="img/foo4.png" class="center" style="width: 50%;" />

When you see a call to `clone()`, you know that some arbitrary code is being
executed, and that code may be expensive. It’s a visual indicator that something
different is going on here.

### Copy

There’s one last wrinkle that we haven’t talked about yet. This code works:

```rust
let x = 5;
let y = x;

println!("{}", x);
```

But why? We don’t have a call to `clone()`. Why didn’t `x` get moved into `y`?

Types like integers that have a known size at compile time do not ask for
memory from the operating system and therefore do not need to be `drop()`ped
when they go out of scope. That means there's no reason we would want to
prevent `x` from being valid after we create the binding `y`. In other words,
there’s no difference between deep and shallow copying here, so calling
`clone()` wouldn’t do anything differently from the usual shallow copying and
we can leave it out.

Rust has a special annotation that you can place on types like these, and that
annotation is the `Copy` trait. We'll talk more about traits in Chapter XX. If
a type has the `Copy` trait, an older binding is still usable after assignment.
Rust will not let you make something have the `Copy` trait if it has
implemented `drop()`. If you need to do something special when the value goes
out of scope, being `Copy` will be an error.

So what types are `Copy`? You can check the documentation for the given type to
be sure, but as a rule of thumb, any group of simple scalar values can be Copy,
but nothing that requires allocation or is some form of resource is `Copy`. Here’s some of the types that are `Copy`:

* All of the integer types, like `u32`.
* The booleans, `true` and `false`.
* All of the floating point types, like `f64`.
* Tuples, but only if they contain types which are also `Copy`. `(i32, i32)`
  is `Copy`, but `(i32, String)` is not.

### Ownership and functions

Passing a value to a function has similar semantics as assigning it:

Filename: src/main.rs

```rust
fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

Passing a binding to a function will move or copy, just like assignment. Here’s
the same example, but with some annotations showing where things go into and
out of scope:

Filename: src/main.rs

```rust
fn main() {
    let s = String::from("hello");  // s goes into scope.

    takes_ownership(s);             // s moves into the function...
                                    // ... and so is no longer valid here.
    let x = 5;                      // x goes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s was moved, nothing special
  // happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop()` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

Remember: If we tried to use `s` after the call to `takes_ownership()`, Rust
would throw a compile-time error. These static checks protect us from mistakes.
Try adding code to `main` that uses `s` and `x` to see where you can use them
and where the ownership rules prevent you from doing so.

Returning values can also transfer ownership:

Filename: src/main.rs

```rust
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}
```

With similiar annotations:

Filename: src/main.rs

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
} // Here, s3 goes out of scope, and is dropped. s2 goes out of scope, but was
  // moved, so nothing happens. s1 goes out of scope, and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned, and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will both take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned, and moves out to the calling function
}
```

It’s the same pattern, every time: assigning something moves it, and when an
owner goes out of scope, if it hasn’t been moved, it will `drop()`.

This might seem a bit tedious, and it is. What if we want to let a function use
a value but not take ownership? It’s quite annoying that anything we pass in
also needs to be passed back if we want to use it again, in addition to any
data resulting from the body of the function that we might want to return as
well. It's _possible_ to return multiple values, using a tuple, like this:

Filename: src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for this concept, and it’s what the
next section is about.

## References and Borrowing

At the end of the last section, we had some example Rust that wasn’t very
good. Here it is again:

Filename: src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

The issue here is that we have to return the `String` back to the calling
function so that we can still use it there, since it was moved when we called
`calculate_length()`.

There is a better way. It looks like this:

Filename: src/main.rs

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}
```

First, you’ll notice all of the tuple stuff in the binding declaration and the
function return value is gone. Next, note that we pass `&s1` into
`calculate_length()`, and in its definition, we take `&String` rather than
`String`.

These `&`s are called *references*, and they allow you to refer to some value
without taking ownership of it. Here’s a diagram:

DIAGRAM GOES HERE of a &String pointing at a String, with (ptr, len, capacity)

Let’s take a closer look at the function call here:

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

The `&s1` syntax lets us create a reference with `s1`. This reference _refers_
to the value of `s1` but does not own it. Because it does not own it, the
value it points to will not be dropped when the reference goes out of scope.

Likewise, the signature of the function uses `&` to indicate that it takes
a reference as an argument. Let’s add some explanatory annotations:

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    let length = s.len();

    length
} // Here, s goes out of scope. But since it does not have ownership of what
  // it refers to, nothing happens.
```

It’s the same process as before, except that because we don’t have ownership,
we don’t drop what a reference points to when the reference goes out of scope.
This lets us write functions which take references as arguments instead of the
values themselves, so that we won’t need to return them to give back ownership.

There’s another word for what references do, and that’s *borrowing*. Just like
with real life, if a person owns something, you can borrow it from them. When
you’re done, you have to give it back.

Speaking of which, what if you try to modify something you borrow from me? Try
this code out. Spoiler alert: it doesn’t work!

Filename: src/main.rs

```rust,ignore
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Here’s the error:

```bash
error: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^
```

Just like bindings are immutable by default, so are references. We’re not
allowed to modify something we have a reference to.

### Mutable references

We can fix this bug! Just a small tweak:

Filename: src/main.rs

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First, we had to change `s` to be `mut`. Then we had to create a mutable
reference with `&mut s` and accept a mutable reference with `some_string: &mut
String`.

Mutable references have one big restriction, though. This code fails:

Filename: src/main.rs

```rust,ignore
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```bash
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```

The error is what it says: you cannot borrow something mutably more than once
at a time. This restriction allows for mutation but in a very controlled
fashion. It is something that new Rustaceans struggle with, because most
languages let you mutate whenever you’d like.

As always, we can use `{}`s to create a new scope, allowing for multiple mutable
references, just not _simultaneous_ ones:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

There is a similar rule for combining mutable and immutable references. This
code errors:

```rust,ignore
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

Here’s the error:

```bash
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Whew! We _also_ cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! Multiple immutable references are okay, however.

### Dangling references

In languages with pointers, it’s easy to create a “dangling pointer” by freeing
some memory while keeping around a pointer to that memory. In Rust, by
contrast, the compiler guarantees that references will never be dangling: if we
have a reference to something, the compiler will ensure that it will not go
out of scope before the reference does.

Let’s try to create a dangling reference:

Filename: src/main.rs

```rust,ignore
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```bash
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^^^^^^^
  |
  = help: this function's return type contains a borrowed value, but there is no
    value for it to be borrowed from
  = help: consider giving it a 'static lifetime

error: aborting due to previous error
```

This error message refers to a feature we haven’t learned about yet,
*lifetimes*. The message does contain the key to why this code is a problem,
though:

```bash
this function’s return type contains a borrowed value, but there is no value
for it to be borrowed from
```

Let’s examine exactly what happens with `dangle()`:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside of `dangle()`, when the code of `dangle()` is
finished, it will be deallocated. But we tried to return a reference to it.
That means this reference would be pointing to an invalid `String`! That’s
no good. Rust won’t let us do this.

The correct code here is to return the `String` directly:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works, no problem. Ownership is moved out, nothing is deallocated.

### The Rules of References

Here’s a recap of what we’ve talked about:

1. At any given time, you may have _either_, but not both of:
    1. One mutable reference.
    2. Any number of immutable references.
2. References must always be valid.

Next, let's look at a different kind of reference: slices.

## Slices

So far, we’ve talked about types that have ownership, like `String`, and ones
that don’t, like `&String`. There is another kind of type which does not have
ownership: slices. Slices let you reference a contiguous sequence of elements
in a collection rather than the whole collection itself.

Here’s a small programming problem: write a function which takes a string
and returns the first word you find. If we don’t find a space in the string,
then the whole string is a word, so the whole thing should be returned.

Let’s think about the signature of this function:

```rust,ignore
fn first_word(s: &String) -> ?
```

This function, `first_word`, takes a `&String` as an argument. We don’t want
ownership, so this is fine. But what should we return? We don’t really have a
way to talk about _part_ of a string. We could return the index of the end of
the word, though. Let’s try that:

Filename: src/main.rs

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Let’s break that down a bit:

```rust,ignore
let bytes = s.as_bytes();
```

Since we need to go through the String element by element and
check if a value is a space, we will convert our String to an
array of bytes using the `.as_bytes()` method.

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

We will be discussing iterators in more detail in Chapter XX, but for
now, know that `iter()` is a method that returns each element in a
collection, and `enumerate()` modifies the result of `iter()` and returns
a tuple instead. The first element of the tuple is the index, and the
second element is a reference to the element itself. This is a bit
nicer than calculating the index ourselves.

Since it’s a tuple, we can use patterns, just like elsewhere in Rust. So we
match against the tuple with i for the index and &item for a single byte. Since
we get a reference from `.iter().enumerate()`, we use `&` in the pattern.

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

We search for the byte that represents the space, using the byte literal
syntax. If we find one, we return the position. Otherwise, we return the length
of the string, using `s.len()`.

This works, but there’s a problem. We’re returning a `usize` on its own, but
it’s only a meaningful number in the context of the `&String`. In other
words, because it’s a separate value from the `String`, there’s no guarantee
that it will still be valid in the future. Consider this:

Filename: src/main.rs

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5.

    s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

This is bad! It’s even worse if we wanted to write a `second_word()`
function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking both a start _and_ an ending index. Even more chances for
things to go wrong. We now have three unrelated variable bindings floating
around which need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

## String slices

A string slice looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This looks just like taking a reference to the whole `String`, but with the
extra `[0..5]` bit. Instead of being a reference to the entire `String`, it’s a
reference to an internal position in the `String` and the number of elements
that it refers to.

We can create slices with a range of `[starting_index..ending_index]`, but the
slice data structure actually stores the starting position and the length of the
slice. So in the case of `let world = &s[6..11];`, `world` would be a slice that
contains a pointer to the 6th byte of `s` and a length value of 5.

In other words, it looks like this:

DIAGRAM GOES HERE of s, hello, and world

With Rust’s `..` range syntax, if you want to start at the first index (zero),
you can drop the value before the `..`. In other words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice should include the last byte of the
`String`, you can drop the trailing number. That means these are
equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these
are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

With this in mind, let’s re-write `first_word()` to return a slice:

Filename: src/main.rs

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Now we have a single value, the `&str`, pronounced "string slice". It stores
both elements that we care about: a reference to the starting point of the
slice and the number of elements in the slice.

This would also work for a `second_word()`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up.

But what about our error condition from before? Slices also fix that. Using
the slice version of `first_word()` will throw an error:

Filename: src/main.rs

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the error:

```bash
17:6 error: cannot borrow `s` as mutable because it is also borrowed as
            immutable [E0502]
    s.clear(); // Error!
    ^
15:29 note: previous borrow of `s` occurs here; the immutable borrow prevents
            subsequent moves or mutable borrows of `s` until the borrow ends
    let word = first_word(&s);
                           ^
18:2 note: previous borrow ends here
fn main() {

}
^
```

Remember the borrowing rules? If we have an immutable reference to something,
we cannot also take a mutable reference. Since `clear()` needs to truncate the
`String`, it tries to take a mutable reference, which fails. Not only has Rust
made our API easier to use, but it’s also eliminated an entire class of errors
at compile time!

### String literals are slices

Remember how we talked about string literals being stored inside of the binary
itself? Now that we know about slices, we can now properly understand string
literals.

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: It’s a slice, pointing to that specific point
of the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

### String slices as arguments

Knowing that you can take slices of both literals and `String`s leads us to
one more improvement on `first_word()`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write this one instead:

```rust,ignore
fn first_word(s: &str) -> &str {
```

Why is this? Well, we aren’t trying to modify `s` at all. And we can take
a string slice that’s the full length of a `String`, so we haven’t lost
the ability to talk about full `String`s. And additionally, we can take
string slices of string literals too, so this function is more useful, but
with no loss of functionality:

Filename: src/main.rs

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

## Other slices

String slices, as you might imagine, are specific to strings. But there’s a more
general slice type, too. Consider arrays:

```rust
let a = [1, 2, 3, 4, 5];
```

Just like we may want to refer to a part of a string, we may want to refer to
part of an array:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the exact same way as string slices
do, by storing a reference to the first element and a length. You’ll use this
kind of slice for all sorts of other collections. We’ll discuss these in detail
when we talk about vectors in Chapter XX.
