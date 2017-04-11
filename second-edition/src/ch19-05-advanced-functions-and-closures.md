# Advanced Functions & Closures

We've talked a lot about functions in this book, and a little bit about a
related feature, closures. There's a few bits we haven't covered yet, so let's
go over those now.

## Function pointers

We've talked about how to pass closures to functions, but you can pass regular
functions to functions too! Functions have the type `fn()`, with a lower case 'f'.
Don't confuse it with the `Fn()` closure trait! The syntax is similar:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

This prints `The answer is: 12`. This `f(i32) -> i32` syntax is called
a 'function pointer', and unlike closures, you don't use it as a trait,
you use it directly, as you can see in the signature of `do_twice`.

### Point-free style

Function pointers implement all three of the closure traits: `Fn`, `FnMut`, and
`FnOnce`. So you can always pass a pointer to a function that expects a closure:

```rust
// fold takes a FnMut closure... but we can use this function too!
fn add(acc: i32, x: &i32) -> i32 {
    acc + *x
}

let v = vec![1, 2, 3];

let six = v.iter().fold(0, |acc, &x| acc + x);
let six = v.iter().fold(0, add);
```

This is sometimes called 'point-free style', for fairly obscure reasons
that don't matter. This can work for anything where the types line up.
For example:

```rust
let v = vec![1, 2, 3];

let strings: Vec<String> = v.iter().map(|s| s.to_string()).collect();

// to_string is provided by the ToString trait
let strings: Vec<String> = v.iter().map(ToString::to_string).collect();
```

Some people prefer this style, some people prefer the closure. They end up
with the same code, so use whichever feels more clear to you.

## Diverging functions

In the previous section, we talked about the never type, `!`. Functions
that return never are called "diverging functions":

```rust
fn never_returns() -> ! {
    panic!("oh no!");
}
```

For more details, see the previous section.

## Returning closures

As we discussed before, closures are represented by traits: `Fn`, `FnMut`, and `FnOnce`.
This means that returning them is a little tricky; you can't do it directly. This will
give a compiler error:

```rust,ignore
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```

It looks like this:

```text
error[E0277]: the trait bound `std::ops::Fn(i32) -> i32 + 'static: std::marker::Sized` is not satisfied
 --> <anon>:2:25
  |
2 | fn returns_closure() -> Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^ the trait `std::marker::Sized` is not implemented for `std::ops::Fn(i32) -> i32 + 'static`
  |
  = note: `std::ops::Fn(i32) -> i32 + 'static` does not have a constant size known at compile-time
  = note: the return type of a function must have a statically known size
```

What to do? With most things that implement traits, we could return them by naming
the type, but we can't do that with closures. Instead, we need to use a trait object:

```rust
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

For more about trait objects, see Chapter 18.