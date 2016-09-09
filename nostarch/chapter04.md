
[TOC]

# Understanding Ownership

Ownership is Rust's most unique feature, and enables Rust to make memory safety
guarantees without needing a garbage collector. It's therefore important to
understand how ownership works in Rust. In this chapter we'll talk about
ownership as well as several related features: borrowing, slices, and how Rust
lays things out in memory.

## Ownership

Rust’s central feature is *ownership*. It is a feature that is straightforward
to explain, but has deep implications for the rest of the language.

All programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection, while in others, the programmer has to
explicitly allocate and free the memory. Rust takes a third approach: memory is
managed through a system of ownership with a set of rules that the compiler
checks at compile-time. You do not pay any run-time cost for any of these
features.

Since ownership is a new concept for many programmers, it does take
some time to get used to. There is good news, though: the more experienced you
become with Rust and the rules of the ownership system, the more you'll be
able to naturally develop code that is both safe and efficient. Keep at it!

Once you understand ownership, you have a good foundation for understanding the
features that make Rust unique. In this chapter, we'll learn ownership by going
through some examples, focusing on a very common data structure: strings.

PROD: START BOX

###### The Stack and the Heap

In many programming languages, we don't have to think about the stack and the
heap very often. But in a systems programming language like Rust, whether a
value is on the stack or the heap has more of an effect on how the language
behaves and why we have to make certain decisions. We're going to be describing
parts of ownership in relation to the stack and the heap, so here is a brief
explanation.

Both the stack and the heap are parts of memory that is available to your code
to use at runtime, but they are structured in different ways. The stack stores
values in the order it gets them and removes the values in the opposite order.
This is referred to as *last in, first out*. Think of a stack of plates: when
you add more plates, you put them on top of the pile, and when you need a
plate, you take one off the top. Adding or removing plates from the middle or
bottom wouldn't work as well! Adding data is called *pushing onto the stack*
and removing data is called *popping off the stack*.

The stack is fast because of the way it accesses the data: it never has to look
around for a place to put new data or a place to get data from; that place is
always the top. Another property that makes the stack fast is that all data on
the stack must take up a known, fixed size.

For data with a size unknown to us at compile time, or a size that might
change, we can store data on the heap instead. The heap is less organized: when
we put data on the heap, we ask for some amount of space. The operating system
finds an empty spot somewhere in the heap that is big enough, marks it as being
in use, and returns to us a pointer to that location. This process is called
*allocating on the heap*, and sometimes we just say "allocating" for short.
Pushing values onto the stack is not considered allocating. Since the pointer
is a known, fixed size, we can store the pointer on the stack, but when we want
the actual data, we have to follow the pointer.

Think of being seated at a restaurant. When you enter, you say how many people
are in your group, and the staff finds an empty table that would fit everyone
and leads you there. If someone in your group comes late, they can ask where
you have been seated to find you.

Accessing data in the heap is slower because we have to follow a pointer to
get there. Allocating a large amount of space can also take time.

When our code calls a function, the values passed into the function (including,
potentially, pointers to data on the heap) and the function's local variables
get pushed onto the stack. When the function is over, those values get popped
off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing
the amount of duplicate data on the heap, and cleaning up unused data on the
heap so that we don't run out of space are all problems that ownership
addresses. Once you understand ownership, you won't need to think about the
stack and the heap very often, but knowing that managing heap data is why
ownership exists can help explain why it works the way it does.

PROD: END BOX

### Ownership Rules

First, let's take a look at the rules. Keep these in mind as we go through the
examples that will illustrate the rules:

> 1. Each value in Rust has a variable binding that’s called its *owner*.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be dropped.

### Variable Binding Scope

We've walked through an example of a Rust program already in the tutorial
chapter. Now that we’re past basic syntax, we won’t include all of the `fn
main() {` stuff in examples, so if you’re following along, you will have to put
the following examples inside of a `main` function yourself. This lets our
examples be a bit more concise, letting us focus on the actual details rather
than boilerplate.

As a first example of ownership, we'll look at the *scope* of some variable
bindings. A scope is the range within a program for which an item is valid.
Let's say we have a variable binding that looks like this:

```rust
let s = "hello";
```

The variable binding `s` refers to a string literal, where the value of the
string is hard coded into the text of our program. The binding is valid from
the point at which it’s declared until the end of the current _scope_. That is:

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

In other words, there are two important points in time here:

- When `s` comes *into scope*, it is valid.
- It remains so until it *goes out of scope*.

At this point, things are similar to other programming languages. Now let’s
build on top of this understanding by introducing the `String` type.

### The `String` Type

In order to illustrate the rules of ownership, we need a data type that is more
complex than the ones we covered in Chapter 3. All of the data types we've
looked at previously are stored on the stack and popped off the stack when
their scope is over, but we want to look at data that is stored on the heap and
explore how Rust knows when to clean that data up.

We're going to use `String` as the example here and concentrate on the parts of
`String` that relate to ownership. These aspects also apply to other complex
data types provided by the standard library and that you create. We'll go into
more depth about `String` specifically in Chapter XX.

We've already seen string literals, where a string value is hard-coded into our
program. String literals are convenient, but they aren’t always suitable for
every situation you want to use text. For one thing, they’re immutable. For
another, not every string value can be known when we write our code: what if we
want to take user input and store it?

For things like this, Rust has a second string type, `String`. This type is
allocated on the heap, and as such, is able to store an amount of text that is
unknown to us at compile time. You can create a `String` from a string literal
using the `from` function, like so:

```rust
let s = String::from("hello");
```

The double colon (`::`) is an operator that allows us to namespace this
particular `from` function under the `String` type itself, rather than using
some sort of name like `string_from`. We’ll discuss this syntax more in the
“Method Syntax” and “Modules” chapters.

This kind of string *can* be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```
So, what’s the difference here? Why can `String` be mutated, but literals
cannot? The difference comes down to how these two types deal with memory.

### Memory and Allocation

In the case of a string literal, because we know the contents at compile time,
the text is hard-coded directly into the final executable. This makes string
literals quite fast and efficient. But these properties only come from its
immutability. Unfortunately, we can’t put a blob of memory into the binary for
each piece of text whose size is unknown at compile time and whose size might
change over the course of running the program.

With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means two things:

1. The memory must be requested from the operating system at runtime.
2. We need a way of giving this memory back to the operating system when we’re
   done with our `String`.

That first part is done by us: when we call `String::from`, its
implementation requests the memory it needs. This is pretty much universal in
programming languages.

The second case, however, is different. In languages with a *garbage collector*
(GC), the GC will keep track and clean up memory that isn't being used anymore,
and we, as the programmer, don’t need to think about it. Without GC, it’s the
programmer's responsibility to identify when memory is no longer being used and
call code to explicitly return it, just as we did to request it. Doing this
correctly has historically been a difficult problem in programming. If we
forget, we will waste memory. If we do it too early, we will have an invalid
variable. If we do it twice, that’s a bug too. We need to pair exactly one
`allocate` with exactly one `free`.

Rust takes a different path: the memory is automatically returned once the
binding to it goes out of scope. Here’s a version of our scope example from
earlier using `String`:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid
```

There is a natural point at which we can return the memory our `String` needs
back to the operating system: when `s` goes out of scope. When a variable
binding goes out of scope, Rust calls a special function for us. This function
is called `drop`, and it is where the author of `String` can put the code to
return the memory. Rust calls `drop` automatically at the closing `}`.

> Note: This pattern is sometimes called *Resource Acquisition Is
> Initialization* in C++, or RAII for short. While they are very similar,
> Rust’s take on this concept has a number of differences, so we don’t tend
> to use the same term. If you’re familiar with this idea, keep in mind that it
> is _roughly_ similar in Rust, but not identical.

This pattern has a profound impact on the way that Rust code is written. It may
seem simple right now, but things can get tricky in more advanced situations
when we want to have multiple variable bindings use the data that we have
allocated on the heap. Let’s go over some of those situations now.

#### Ways Bindings and Data Interact: Move

There are different ways that multiple bindings can interact with the same data
in Rust. Let's take an example using an integer:

```rust
let x = 5;
let y = x;
```

We can probably guess what this is doing based on our experience with other
languages: “Bind the value `5` to `x`, then make a copy of the value in `x` and
bind it to `y`.” We now have two bindings, `x` and `y`, and both equal `5`.
This is indeed what is happening since integers are simple values with a known,
fixed size, and these two `5` values are pushed onto the stack.

Now let’s look at the `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar to the previous code, so we might assume that the way
it works would be the same: that the second line would make a copy of the value
in `s1` and bind it to `s2`. This isn't quite what happens.

To explain this more thoroughly, let’s take a look at what `String` looks like
under the covers in Figure 4-1. A `String` is made up of three parts, shown on
the left: a pointer to the memory that holds the contents of the string, a
length, and a capacity. This group of data is stored on the stack. On the right
is the memory that holds the contents, and this is on the heap.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

Figure 4-1: Representation in memory of a `String` holding the value "hello"
bound to `s1`

The length is how much memory, in bytes, the contents of the `String` is
currently using. The capacity is the total amount of memory, in bytes, that the
`String` has gotten from the operating system. The difference between length
and capacity matters but not in this context, so for now, it's fine to ignore
the capacity.

When we assign `s1` to `s2`, the `String` data itself is copied, meaning we
copy the pointer, the length, and the capacity that are on the stack. We do not
copy the data on the heap that the `String`'s pointer refers to. In other
words, it looks like figure 4-2.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

Figure 4-2: Representation in memory of the binding `s2` that has a copy of
`s1`'s pointer, length and capacity

And _not_ Figure 4-3, which is what memory would look like if Rust instead
copied the heap data as well. If Rust did this, the operation `s2 = s1` could
potentially be very expensive if the data on the heap was large.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

Figure 4-3: Another possibility for what `s2 = s1` might do, if Rust chose to
copy heap data as well.

Earlier, we said that when a binding goes out of scope, Rust will automatically
call the `drop` function and clean up the heap memory for that binding. But
in figure 4-2, we see both data pointers pointing to the same location. This is
a problem: when `s2` and `s1` go out of scope, they will both try to free the
same memory. This is known as a *double free* error and is one of the memory
safety bugs we mentioned before. Freeing memory twice can lead to memory
corruption, which can potentially lead to security vulnerabilities.

In order to ensure memory safety, there's one more detail to what happens in
this situation in Rust. Instead of trying to copy the allocated memory, Rust
says that `s1` is no longer valid and, therefore, doesn’t need to free anything
when it goes out of scope. Check out what happens when you try to use `s1`
after `s2` is created:

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
without copying the data probably sounds like a shallow copy. But because Rust
also invalidates the first binding, instead of calling this a shallow copy,
it's known as a _move_. Here we would read this by saying that `s1` was _moved_
into `s2`. So what actually happens looks like Figure 4-4.

<img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

Figure 4-4: Representation in memory after `s1` has been invalidated

That solves our problem! With only `s2` valid, when it goes out of scope, it
alone will free the memory, and we’re done.

Furthermore, there’s a design choice that’s implied by this: Rust will never
automatically create "deep" copies of your data. Therefore, any _automatic_
copying can be assumed to be inexpensive.

#### Ways Bindings and Data Interact: Clone

If we _do_ want to deeply copy the `String`’s data and not just the `String`
itself, there’s a common method for that: `clone`. We will discuss methods in
the section on `structs` in Chapter XX, but they’re a
common enough feature in many programming languages that you have probably seen
them before.

Here’s an example of the `clone` method in action:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This will work just fine, and this is how you can explicitly get the behavior
we showed in Figure 4-3, where the heap data *does* get copied.

When you see a call to `clone`, you know that some arbitrary code is being
executed, and that code may be expensive. It’s a visual indicator that something
different is going on here.

#### Stack-only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code, that we showed
earlier, works and is valid:

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

This seems to contradict what we just learned: we don't have a call to
`clone`, but `x` is still valid, and wasn't moved into `y`.

This is because types like integers that have a known size at compile time are
stored entirely on the stack, so copies of the actual values are quick to make.
That means there's no reason we would want to prevent `x` from being valid
after we create the binding `y`. In other words, there’s no difference between
deep and shallow copying here, so calling `clone` wouldn’t do anything
differently from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on
types like these (we'll talk more about traits in Chapter XX). If a type has
the `Copy` trait, an older binding is still usable after assignment. Rust will
not let us annotate a type with the `Copy` trait if the type, or any of its
parts, has implemented `drop`. If the type needs something special to happen
when the value goes out of scope and we add the `Copy` annotation to that type,
we will get a compile-time error.

So what types are `Copy`? You can check the documentation for the given type to
be sure, but as a rule of thumb, any group of simple scalar values can be Copy,
and nothing that requires allocation or is some form of resource is `Copy`.
Here’s some of the types that are `Copy`:

* All of the integer types, like `u32`.
* The booleans, `true` and `false`.
* All of the floating point types, like `f64`.
* Tuples, but only if they contain types which are also `Copy`. `(i32, i32)`
  is `Copy`, but `(i32, String)` is not.

### Ownership and Functions

The semantics for passing a value to a function are similar to assigning a
value to a binding. Passing a binding to a function will move or copy, just
like assignment. Here’s an example, with some annotations showing where bindings
go into and out of scope:

Filename: src/main.rs

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use `s` after the call to `takes_ownership`, Rust
would throw a compile-time error. These static checks protect us from mistakes.
Try adding code to `main` that uses `s` and `x` to see where you can use them
and where the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Here's an example with similar annotations:

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

It’s the same pattern, every time: assigning a value to another binding moves
it, and when heap data values' bindings go out of scope, if the data hasn’t
been moved to be owned by another binding, the value will be cleaned up by
`drop`.

Taking ownership then returning ownership with every function is a bit tedious.
What if we want to let a function use a value but not take ownership? It’s
quite annoying that anything we pass in also needs to be passed back if we want
to use it again, in addition to any data resulting from the body of the
function that we might want to return as well.

It is possible to return multiple values using a tuple, like this:

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
common. Luckily for us, Rust has a feature for this concept: references.

## References and Borrowing

The issue with the tuple code at the end of the last section is that we have to
return the `String` back to the calling function so that we can still use the
`String` after the call to `calculate_length`, since the `String` was moved
into `calculate_length`.

Here is how you would use a function without taking ownership of it using
*references:*

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
`calculate_length`, and in its definition, we take `&String` rather than
`String`.

These `&`s are *references*, and they allow you to refer to some value
without taking ownership of it. Figure 4-5 shows a diagram of this.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

Figure 4-5: `&String s` pointing at `String s1`

Let’s take a closer look at the function call here:

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

The `&s1` syntax lets us create a reference which _refers_ to the value of `s1`
but does not own it. Because it does not own it, the value it points to will
not be dropped when the reference goes out of scope.

Likewise, the signature of the function uses `&` to indicate that it takes
a reference as an argument. Let’s add some explanatory annotations:

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    let length = s.len();

    length
} // Here, s goes out of scope. But since it does not have ownership of what
  // it refers to, nothing happens.
```

It’s the same process as before, but we don’t drop what the reference points to
when it goes out of scope because we don't have ownership. This lets us write
functions which take references as arguments instead of the values themselves,
so that we won’t need to return them to give back ownership.

We call this process *borrowing*. Just like with real life, if a person owns
something, you can borrow it from them, and when you’re done, you have to give
it back.

So what happens if we try to modify something we're borrowing? Try this code
out. Spoiler alert: it doesn’t work!

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

Just as bindings are immutable by default, so are references. We’re not allowed
to modify something we have a reference to.

### Mutable References

We can fix this error with just a small tweak:

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

Mutable references have one big restriction, though: you can only have one
mutable reference to a particular piece of data in a particular scope. This
code will fail:

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

This restriction allows for mutation but in a very controlled fashion. It is
something that new Rustaceans struggle with, because most languages let you
mutate whenever you’d like. The benefit of having this restriction is that Rust
can prevent data races at compile time. A *data race* is a particular type of
race condition where two or more pointers access the same data at the same
time, at least one of the pointers is being used to write to the data, and
there's no mechanism being used to synchronize access to the data. Data races
cause undefined behavior and can be difficult to diagnose and fix when trying
to track them down at runtime; Rust prevents this problem from happening since
it won't even compile code with data races!

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
from under them! Multiple immutable references are okay, however, since no one
who is just reading the data has the ability to affect anyone else's reading of
the data.

Even though these errors may be frustrating at times, remember that it's the
Rust compiler pointing out a potential bug earlier (at compile time rather than
at runtime) and showing you exactly where the problem is instead of you having
to track down why sometimes your data isn't what you thought it should be.

### Dangling References

In languages with pointers, it's easy to make the error of creating a *dangling
pointer*, a pointer referencing a location in memory that may have been given
to someone else, by freeing some memory while keeping around a pointer to that
memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling: if we have a reference to some data, the compiler will
ensure that the data will not go out of scope before the reference to the data
does.

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

This error message refers to a feature we haven’t learned about yet:
*lifetimes*. We'll discuss lifetimes in detail in Chapter XX, but, disregarding
the parts about lifetimes, the message does contain the key to why this code is
a problem: `this function’s return type contains a borrowed value, but there is
no value for it to be borrowed from`.

Let’s have a closer look at exactly what's happenening at each stage of our
`dangle` code:

```rust,ignore
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside of `dangle`, when the code of `dangle` is
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

There is another data type which does not have ownership: slices. Slices let
you reference a contiguous sequence of elements in a collection rather than the
whole collection itself.

Here’s a small programming problem: write a function which takes a string and
returns the first word it finds in that string. If it doesn’t find a space in
the string, it means the whole string is one word, so the whole thing should be
returned.

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
array of bytes using the `as_bytes` method.

```rust,ignore
for (i, &item) in bytes.iter().enumerate() {
```

We will be discussing iterators in more detail in Chapter XX, but for now, know
that `iter` is a method that returns each element in a collection, and
`enumerate` modifies the result of `iter` and returns each element as part
of a tuple instead, where the first element of the tuple is the index, and the
second element is a reference to the element itself. This is a bit nicer than
calculating the index ourselves.

Since it’s a tuple, we can use patterns, just like elsewhere in Rust. So we
match against the tuple with `i` for the index and `&item` for a single byte.
Since we get a reference from `.iter().enumerate()`, we use `&` in the pattern.

```rust,ignore
    if item == b' ' {
        return i;
    }
}
s.len()
```

We search for the byte that represents the space, using the byte literal
syntax. If we find a space, we return the position. Otherwise, we return the
length of the string, using `s.len()`.

We now have a way to find out the index of the end of the first word in the
string, but there’s a problem. We’re returning a `usize` on its own, but it’s
only a meaningful number in the context of the `&String`. In other words,
because it’s a separate value from the `String`, there’s no guarantee that it
will still be valid in the future. Consider this program that uses this
`first_word` function:

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

This program compiles without any errors, and also would if we used `word`
after calling `s.clear()`. `word` isn't connected to the state of `s` at all,
so `word` still contains the value `5`. We could use that `5` with `s` to try
to extract the first word out, but this would be a bug since the contents of
`s` have changed since we saved `5` in `word`.

This is bad! It’s even worse if we wanted to write a `second_word`
function. Its signature would have to look like this:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Now we’re tracking both a start _and_ an ending index, and we have even more
values that were calculated from data in a particular state but aren't tied to
that state at all. We now have three unrelated variable bindings floating
around which need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

### String Slices

A string slice is a reference to part of a `String`, and looks like this:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This is similar to taking a reference to the whole `String`, but with the
extra `[0..5]` bit. Rather than a reference to the entire `String`, it’s a
reference to an internal position in the `String` and the number of elements
that it refers to.

We create slices with a range of `[starting_index..ending_index]`, but the
slice data structure actually stores the starting position and the length of the
slice. So in the case of `let world = &s[6..11];`, `world` would be a slice that
contains a pointer to the 6th byte of `s` and a length value of 5.

Figure 4-6 shows this in a diagram:

<img alt="world containing a pointer to the 6th byte of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

Figure 4-6: Two string slices referring to parts of a `String`

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

With this in mind, let’s re-write `first_word` to return a slice. The type
that signifies "string slice" is written as `&str`:

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

We get the index for the end of the word in the same way as before, by looking
for the first occurrence of a space. When we find a space, we return a string
slice using the start of the string and the index of the space as the starting
and ending indices.

Now when we call `first_word`, we get back a single value that is tied to the
underlying data. The value is made up of a reference to the starting point of
the slice and the number of elements in the slice.

Returning a slice would also work for a `second_word` function:

```rust,ignore
fn second_word(s: &String) -> &str {
```

We now have a straightforward API that’s much harder to mess up. Remember our
bug from before, when we got the first word but then cleared the string so that
our first word was invalid? That code was logically incorrect but didn't show
any immediate errors. The problems would show up later, if we kept trying to
use the first word index with an emptied string. Slices make this bug
impossible, and let us know we have a problem with our code much sooner. Using
the slice version of `first_word` will throw a compile time error:

Filename: src/main.rs

```rust,ignore
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```

Here’s the compiler error:

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

Remember from the borrowing rules that if we have an immutable reference to
something, we cannot also take a mutable reference. Since `clear` needs to
truncate the `String`, it tries to take a mutable reference, which fails. Not
only has Rust made our API easier to use, but it’s also eliminated an entire
class of errors at compile time!

#### String Literals are Slices

Remember how we talked about string literals being stored inside of the binary
itself? Now that we know about slices, we can now properly understand string
literals.

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: It’s a slice, pointing to that specific point
of the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

#### String Slices as Arguments

Knowing that you can take slices of both literals and `String`s leads us to
one more improvement on `first_word`, and that’s its signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write this one instead because it allows us
to use the same function on both `String`s and `&str`s:

```rust,ignore
fn first_word(s: &str) -> &str {
```

If we have a string slice, we can pass that as the argument directly. If we
have a `String`, we can pass a slice of the entire `String`. This makes our API
more general and useful without losing any functionality:

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

### Other Slices

String slices, as you might imagine, are specific to strings. But there’s a more
general slice type, too. Consider this array:

```rust
let a = [1, 2, 3, 4, 5];
```

Just like we may want to refer to a part of a string, we may want to refer to
part of an array, and would do so like this:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

This slice has the type `&[i32]`. It works the exact same way as string slices
do, by storing a reference to the first element and a length. You’ll use this
kind of slice for all sorts of other collections. We’ll discuss these in detail
when we talk about vectors in Chapter XX.

## Summary

The concepts of ownership, borrowing, and slices are what ensure memory safety
in Rust programs at compile time. Rust is a language that gives you control
over your memory usage like other systems programming languages, but having the
owner of data automatically clean up that data when the owner goes out of scope
means you don't have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we will be talking
about these concepts further throughout the rest of the book. Let's move on to
the next chapter where we'll look at grouping pieces of data together in a
`struct`.
