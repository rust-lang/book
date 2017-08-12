## Advanced Functions & Closures

Finally, let’s discuss some advanced features having to do with functions and
closures: function pointers, diverging functions, and returning closures.

### Function pointers

We’ve talked about how to pass closures to functions, but you can pass regular
functions to functions too! Functions coerce to the type `fn`, with a lower
case ‘f’ not to be confused with the `Fn` closure trait. `fn` is called a
*function pointer*. The syntax for specifying that a parameter is a function
pointer is similar to that of closures, as shown in Listing 19-34:

<span class="filename">Filename: src/main.rs</span>

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

<span class="caption">Listing 19-34: Using the `fn` type to accept a function
pointer as an argument</span>

This prints `The answer is: 12`. We specify that the parameter `f` in
`do_twice` is an `fn` that takes one parameter of type `i32` and returns an
`i32`. We can then call `f` in the body of `do_twice`. In `main`, we can pass
the function name `add_one` as the first argument to `do_twice`.

Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound.

Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so we can always pass a function pointer as an argument when calling
a function that expects a closure. Prefer to write functions using a generic
type and one of the closure traits, so that your functions can accept either
functions or closures. An example of a case where you’d only want to accept
`fn` is when interfacing with external code that doesn’t have closures: C
functions can accept functions as arguments, but C doesn’t have closures.

For example, if we wanted to use the `map` function to turn a vector of numbers
into a vector of strings, we could use a closure:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```

Or we could name a function as the argument to `map` instead of the closure:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

Note that we do have to use the fully qualified syntax that we talked about in
the “Advanced Traits” section because there are multiple functions available
named `to_string`; here, we’re using the `to_string` function defined in the
`ToString` trait, which the standard library has implemented for any type that
implements `Display`.

Some people prefer this style, some people prefer the closure. They end up
with the same code, so use whichever feels more clear to you.

### Returning Closures

Because closures are represented by traits, returning closures is a little
tricky; we can’t do it directly. In most cases where we may want to return a
trait, we can instead use the concrete type that implements the trait of what
we’re returning as the return value of the function. We can’t do that with
closures, though. They don’t have a concrete type that’s returnable; we’re not
allowed to use the function pointer `fn` as a return type, for example.

This code that tries to return a closure directly won’t compile:

```rust,ignore
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```

The compiler error is:

```text
error[E0277]: the trait bound `std::ops::Fn(i32) -> i32 + 'static:
std::marker::Sized` is not satisfied
 --> <anon>:2:25
  |
2 | fn returns_closure() -> Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^ the trait `std::marker::Sized` is
  not implemented for `std::ops::Fn(i32) -> i32 + 'static`
  |
  = note: `std::ops::Fn(i32) -> i32 + 'static` does not have a constant size
  known at compile-time
  = note: the return type of a function must have a statically known size
```

The `Sized` trait again! Rust doesn’t know how much space it’ll need to store the
closure. We saw a solution to this in the previous section, though: we can use
a trait object:

```rust
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

For more about trait objects, refer back to Chapter 18.

## Summary

Whew! Now we’ve gone over features of Rust that aren’t used very often, but are
available if you need them. We’ve introduced a lot of complex topics so that
when you encounter them in error message suggestions or when reading others’
code, you’ll at least have seen these concepts and syntax once before.

Now, let’s put everything we’ve learned throughout the book into practice with
one more project!
