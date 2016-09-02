## Ownership

Rust’s central feature is *ownership*. It is a feature that is
straightforward to explain, but has deep implications for the rest of the
language.

All programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection, while in others, the programmer has to
explicitly allocate and free the memory. Rust takes a third approach: memory is
managed through a system of ownership with a set of rules that the compiler
checks at compile-time. You do not pay any run-time cost for any of these
features.

<!-- I think it would be helpful to have the ownership rules up here, before we look at examples, to give the reader an idea of what they should be focusing on as they go through the examples. I won't make the move now, but wanted to float the idea -->

However, because ownership is a new concept for many programmers, it does take
some time to get used to. There is good news, though: the more experienced you
become with Rust and the rules of the ownership system, the more you'll be
able to naturally develop code that is both safe and efficient. Keep at it!

Once you understand ownership, you have a good foundation for understanding the
features that make Rust unique. In this chapter, we'll learn ownership by going
through some examples, focusing on a very common data structure: strings.

### Variable Binding Scope

We've walked through an example of a Rust program already in the tutorial
chapter. Now that we’re past basic syntax, we won’t include all of the `fn
main() {` stuff in examples, so if you’re following along, you will have to put
the folowing examples inside of a `main()` function yourself. This lets our examples be a bit more
concise, letting us focus on the actual details rather than boilerplate.

As a first example of ownership, we'll look at the scope---the range within a program for which an object is valid---inside a short variable binding program:

<!-- Is it worth giving a quick definition of scope here? I've suggested a placeholder definition above as an example -->

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

- When `s` comes *into scope*, it is valid.
- It remains so until it *goes out of scope*.

At this point, things are similar to other programming languages. Now let’s
build on top of this understanding by introducing the `String` type.

### The String Type

String literals are convenient, but they aren’t always suitable for every situation you want to use
strings. For one thing, they’re immutable. For another, not every string is
literal: what if you want to take user input and store it in a string?

For things like this, Rust has a second string type, `String`. You can create a `String`
from a string literal using the `from` function, like so:

<!-- So is the difference that a string literal is hard coded but a `String` is a string object? It'd help to make that explicit -->

```rust
let s = String::from("hello");
```

This double colon (`::`) syntax is new. It is an operator that
allows us to namespace this particular `from()` function under the `String`
type itself, rather than using some sort of name like `string_from()`. We’ll
discuss this syntax more in the “Method Syntax” and “Modules” chapters.

This kind of string *can* be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```
So, what’s the difference here? Why can `String` be mutated, but literals
cannot? The difference comes down to how these two types deal with memory.

### Memory and Allocation

In the case of a string literal, because we know the contents of the string at
compile time, we can hard-code the text of the string directly into the final
executable. This means that string literals are quite fast and efficient. But
these properties only come from its immutability. Unfortunately, we can’t put a
blob of memory into the binary for each string whose size is unknown at compile
time and whose size might change over the course of running the program.

<!--It would be good to be able to differentiate between a string literal and a `String` more clearly in print, the literal style that will be applied may not be obvious enough, could confuse the discussion here --- would is be accurate to call `String` a `String` object? -->

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
(GC), the GC will keep track and clean up memory that isn't being used
anymore, and we, as the programmer, don’t need to think about it. Without GC,
it’s the programmer's responsibility to identify when memory is no longer being used and
call code to explicitly return it, just as we did to request it. Doing this
correctly has historically been a difficult problem in programming languages. If we forget, we will
waste memory. If we do it too early, we will have an invalid variable. If we do
it twice, that’s a bug too. We need to pair exactly one `allocate()` with
exactly one `free()`.

Rust takes a different path; the memory is automatically returned once the object using it goes out of scope.  Here’s a version of our scope example from earlier using
`String`:

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid
```

There is a natural point at which we can return the memory our `String` needs
back to the operating system: when it goes out of scope. When a variable goes
out of scope, Rust calls a special function for us. This function is called
`drop()`, and it is where the author of `String` can put the code to return the
memory.

<!-- So, just to be certain, we don't see drop(), it happens behind the scenes at the closing } that takes the String out of scope, is that right? -->

> Note: This pattern is sometimes called “Resource Acquisition Is
> Initialization” in C++, or “RAII” for short. While they are very similar,
> Rust’s take on this concept has a number of differences, and so we don’t tend
> to use the same term. If you’re familiar with this idea, keep in mind that it
> is _roughly_ similar in Rust, but not identical.

This pattern has a profound impact on the way that Rust code is written. It may
seem simple right now, but things can get tricky in more advanced situations.
Let’s go over the first one of those right now.

#### Types of Memory Allocation: Move
<!-- Could you add a small general intro before going into the example, let the reader know what this section is about? I've added an example line below -->

There are different ways to use memory in Rust. Let's take an example using an integer:
```rust
let x = 5;
let y = x;
```

You can probably see what this is saying: “Allocate `5` to `x` and make a copy of `5`”.
We now have two
bindings, `x` and `y`, and both equal `5`.

Now let’s look at `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

You might expect that this code will “copy the `String`!” This is both correct and incorrect at the
same time. It does a _shallow_ copy of the `String`, which means it copies some aspects of the `String` object, but not the data itself. To explain this more thoroughly,
let’s take a look at what `String` looks like under the covers in Figure 4-1.

<!-- A little later we say that it **doesn't** make a shallow copy of the string -- we should be consistent in how we describe this, do you want to alter the above? -->

<img alt="string" src="img/foo1.png" class="center" style="width: 50%;" />
Figure 4-1:

<!-- We'll number and caption each figure in the final book so we can call it our in the text -- could you suggest a caption for this and the following figures? -->

<!-- Could you also send me any images you're using for these first six chapters? -->

A `String` is made up of three parts: a pointer to the memory that holds the
contents of the string, a length, and a capacity. The length is how much memory
the `String` is currently using. The capacity is the total amount of memory the
`String` has gotten from the operating system. The difference between length
and capacity matters but not in this context, so
for now, it's fine to ignore the capacity.

<!-- are the length and capacity measured in bytes? -->

When we assign `s1` to `s2`, the `String` itself is copied, meaning we copy the
pointer, the length, and the capacity. We do not copy the data that the
`String`'s pointer refers to. In other words, it looks like figure 4-2.

<img alt="s1 and s2" src="img/foo2.png" class="center" style="width: 50%;" />
Figure 4-2:

And _not_ Figure 4-3.

<img alt="s1 and s2 to two places" src="img/foo4.png" class="center" style="width: 50%;" />
Figure 4-3:

There’s a problem here. Both data pointers are pointing to the same place. This means that when `s2` goes out of scope, it will free the memory
that the pointer points to, but then when `s1` goes out of scope, and it will _also_
try to free the memory that the pointer points to. That’s bad, and is known as
a "double free" error.

<!-- Will the program throw an error, or will the memory just be mismanaged? -->

So what’s the solution? Here, we stand at a crossroads with a few options.

<!-- I'm not sure we need this following paragraph with the 'wrong' options, it might just complicate the explanation --- is there a particular reason you want to include it, or would it work to delete it? -->

One way to fix this would be to change assignment so that it will also copy out any data.
This works, but is inefficient: what if our `String` contained a novel?
Also, that solution would only work for memory. What if, instead of a `String`,
we had a `TcpConnection`? Opening and closing a network connection is very
similar to allocating and freeing memory, so it would be nice to be able to use
the same mechanism. We wouldn't be able to, though, because creating a new
connection requires more than just copying memory: we have to request a new
connection from the operating system. We could instead extend our solution to
allow the programmer to hook into the assignment, similar to `drop()`, and
write code to fix things up. That would work, but if we did that, an `=` could
run arbitrary code. That’s also not good, and it doesn’t solve our efficiency
concerns either.

Let’s take a step back: the root of the problem is that `s1` and `s2` both
think that they have control of the memory and therefore need to free it.
Instead of trying to copy the allocated memory, Rust says that `s1` is no
longer valid and, therefore, doesn’t need to free anything.
Check out what happens when you try to use `s1` after
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
without copying the data probably sounds like a shallow copy. But because Rust
also invalidates the first binding, instead of calling this a shallow copy,
it's known as a _move_. Here we would read this by saying that `s1` was _moved_
into `s2`. So what actually happens looks like Figure 4-4.

<img alt="s1 and s2 to the same place" src="img/foo3.png" class="center" style="width: 50%;" />
Figure 4-4:

That solves our problem! With only `s2` valid, when it goes out of scope, it
alone will free the memory, and we’re done.

#### Ownership Rules
<!-- See my note above. I do think it'd be useful for the reader to see these before getting into the move example. Knowing that there can only be one owner at a time could help navigate them through that explanation -->

This leads us to Rust's Ownership Rules:

> 1. Each value in Rust has a variable binding that’s called its *owner*.
> 2. There can only be one owner at a time.
> 3. When the owner goes out of scope, the value will be `drop()`ped.

Furthermore, there’s a design choice that’s implied by this: Rust will never
automatically create "deep" copies of your data. Therefore, any _automatic_
copying can be assumed to be inexpensive.

#### Deep Copy with Clone

If we _do_ want to deeply copy the `String`’s data and not just the
`String` itself, there’s a common method for that: `clone()`. We will discuss
methods in the section on `structs` in Chapter XX, but they’re a
common enough feature in many programming languages that you have probably seen
them before.

Here’s an example of the `clone()` method in action:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("{}", s1);
```

<!-- I got an unused variable message -- do you want to mention that? -->

This will work just fine. Remember Figure XX? In this case,
it _is_ doing what's shown in Figure 4-5.

<!-- If this is just a repeat of an earlier figure, we might just refer back to that figure rather than showing it again-->

<img alt="s1 and s2 to two places" src="img/foo4.png" class="center" style="width: 50%;" />

Figure 4-5:

When you see a call to `clone()`, you know that some arbitrary code is being
executed, and that code may be expensive. It’s a visual indicator that something
different is going on here.

#### Deep and Shallow Copy with Copy

There’s one last wrinkle that we haven’t talked about yet. This code, that we showed earlier, works and is valid:

```rust
let x = 5;
let y = x;

println!("{}", x);
```

This seems to contradict what we learned so far: we don't have a call to `clone()`, but `x` is still valid, and wasn't moved into `y`.

This is because types like integers that have a known size at compile time do not ask for
memory from the operating system and therefore do not need to be `drop()`ped
when they go out of scope. That means there's no reason we would want to
prevent `x` from being valid after we create the binding `y`. In other words,
there’s no difference between deep and shallow copying here, so calling
`clone()` wouldn’t do anything differently from the usual shallow copying and
we can leave it out.

Rust has a special annotation that you can place on types like these called the `Copy` trait (we'll talk more about traits in Chapter XX). If
a type has the `Copy` trait, an older binding is still usable after assignment.
Rust will not let you give something the `Copy` trait if it has
implemented `drop()`. If you need to do something special when the value goes
out of scope, being `Copy` will be an error.

<!-- So Copy is allocated by the user, and not by rust, is that right? I'm not sure whether the programmer is doing this explicitly or not? -->

So what types are `Copy`? You can check the documentation for the given type to
be sure, but as a rule of thumb, any group of simple scalar values can be Copy,
and nothing that requires allocation or is some form of resource is `Copy`. Here’s some of the types that are `Copy`:

* All of the integer types, like `u32`.
* The booleans, `true` and `false`.
* All of the floating point types, like `f64`.
* Tuples, but only if they contain types which are also `Copy`. `(i32, i32)`
  is `Copy`, but `(i32, String)` is not.

### Ownership and Functions

The semantics for passing a value to a function are similar to assigning it.

<!-- Filename: src/main.rs

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
``` -->

Passing a binding to a function will move or copy, just like assignment. Here’s
an example, with some annotations showing where things go into and
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

If we tried to use `s` after the call to `takes_ownership()`, Rust
would throw a compile-time error. These static checks protect us from mistakes.
Try adding code to `main` that uses `s` and `x` to see where you can use them
and where the ownership rules prevent you from doing so.

### Return Values and Scope

Returning values can also transfer ownership. Here's an example with similar annotations:

<!-- Filename: src/main.rs

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

With similiar annotations: -->

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

This might seem a bit tedious. What if we want to let a function use
a value but not take ownership? It’s quite annoying that anything we pass in
also needs to be passed back if we want to use it again, in addition to any
data resulting from the body of the function that we might want to return as
well.

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
