# Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handling the duplication
of concepts. In Rust, one such tool is *generics*. Generics are abstract
stand-ins for concrete types or other properties. When we’re writing code, we
can express the behavior of generics or how they relate to other generics
without knowing what will be in their place when compiling and running the code.

Similar to the way a function takes parameters with unknown values to run the
same code on multiple concrete values, functions can take parameters of some
generic type instead of a concrete type, like `i32` or `String`. In fact, we’ve
already used generics in Chapter 6 with `Option<T>`, Chapter 8 with `Vec<T>`
and `HashMap<K, V>`, and Chapter 9 with `Result<T, E>`. In this chapter, you’ll
explore how to define your own types, functions, and methods with generics!

First, we’ll review how to extract a function to reduce code duplication. Next,
we’ll use the same technique to make a generic function from two functions that
differ only in the types of their parameters. We’ll also explain how to use
generic types in struct and enum definitions.

Then you’ll learn how to use *traits* to define behavior in a generic way. You
can combine traits with generic types to constrain a generic type to only
those types that have a particular behavior, as opposed to just any type.

Finally, we’ll discuss *lifetimes*, a variety of generics that give the
compiler information about how references relate to each other. Lifetimes allow
us to borrow values in many situations while still enabling the compiler to
check that the references are valid.

## Removing Duplication by Extracting a Function

Before diving into generics syntax, let’s first look at how to remove
duplication that doesn’t involve generic types by extracting a function. Then
we’ll apply this technique to extract a generic function! In the same way that
you recognize duplicated code to extract into a function, you’ll start to
recognize duplicated code that can use generics.

Consider a short program that finds the largest number in a list, as shown in
Listing 10-1.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

<span class="caption">Listing 10-1: Code to find the largest number in a list
of numbers</span>

This code stores a list of integers in the variable `number_list` and places
the first number in the list in a variable named `largest`. Then it iterates
through all the numbers in the list, and if the current number is greater than
the number stored in `largest`, it replaces the number in that variable.
However, if the current number is less than or equal to the largest number seen
so far, the variable doesn’t change, and the code moves on to the next number
in the list. After considering all the numbers in the list, `largest` should
hold the largest number, which in this case is 100.

To find the largest number in two different lists of numbers, we can duplicate
the code in Listing 10-1 and use the same logic at two different places in the
program, as shown in Listing 10-2.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

<span class="caption">Listing 10-2: Code to find the largest number in *two*
lists of numbers</span>

Although this code works, duplicating code is tedious and error prone. We also
have to update the code in multiple places when we want to change it.

To eliminate this duplication, we can create an abstraction by defining a
function that operates on any list of integers given to it in a parameter. This
solution makes our code clearer and lets us express the concept of finding the
largest number in a list abstractly.

In Listing 10-3, we extracted the code that finds the largest number into a
function named `largest`. Unlike the code in Listing 10-1, which can find the
largest number in only one particular list, this program can find the largest
number in two different lists.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>

The `largest` function has a parameter called `list`, which represents any
concrete slice of `i32` values that we might pass into the function. As a
result, when we call the function, the code runs on the specific values that we
pass in. Don't worry about the syntax of the `for` loop for now. We aren't
referencing a reference to an `i32` here; we're pattern matching and
destructuring each `&i32` that the `for` loop gets so that `item` will be an
`i32` inside the loop body. We'll cover pattern matching in detail in [Chapter
18][ch18]<!-- ignore -->.

In sum, here are the steps we took to change the code from Listing 10-2 to
Listing 10-3:

1. Identify duplicate code.
2. Extract the duplicate code into the body of the function and specify the
   inputs and return values of that code in the function signature.
3. Update the two instances of duplicated code to call the function instead.

Next, we’ll use these same steps with generics to reduce code duplication in
different ways. In the same way that the function body can operate on an
abstract `list` instead of specific values, generics allow code to operate on
abstract types.

For example, say we had two functions: one that finds the largest item in a
slice of `i32` values and one that finds the largest item in a slice of `char`
values. How would we eliminate that duplication? Let’s find out!

[ch18]: ch18-00-patterns.html
