# Generics

One of the core tools a programming language gives you is the ability to deal
effectively with duplication of code. It's important to minimize the amount of
code that is duplicated throughout a program to make maintenance easier and
minimize logic errors. Maintenance will be easier if there's only one place
that you need to change the code if you change your mind about how the program
should work, rather than multiple places in the code. If your program's logic
is duplicated in different places and those places don't match, you'll get
errors or unexpected and undesired behavior from your program that could be
hard to track down. Rust has the concept of *generics* as one way to eliminate
duplicate code. Generics come in the form of generic types, traits that those
generic types have, and generic lifetimes. We'll cover how to use all of these
in this chapter.

## Removing Duplication by Extracting a Function

Let's first go through a technique for dealing with duplication that you're
probably familiar with: extracting a function. Consider a small program that
finds the largest number in a list, shown in Listing 10-1:

<figure>
<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

<figcaption>

Listing 10-1: Code to find the largest number in a list of numbers

</figcaption>
</figure>

If we needed to find the largest number in two different lists of numbers, we
could duplicate the code in Listing 10-1 and have the same logic exist in two
places in the program:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

Copying code is tedious and error-prone, plus now we have two places to update
the logic if we need it to change. Rust, like many languages, gives us a way to
deal with this duplication by creating an abstraction, and in this case the
abstraction we'll use is a function. Here's a program where we've extracted the
code in Listing 10-1 that finds the largest number into a function named
`largest`. This program can find the largest number in two different lists of
numbers, but the code from Listing 10-1 only exists in one spot:

<span class="filename">Filename: src/main.rs</span>

```rust
fn largest(numbers: Vec<i32>) {
    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    largest(numbers);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    largest(numbers);
}
```

The function defines a parameter, `numbers`, which represents any concrete
`Vec<i32>` that we might pass into the function. The code in the function
definition operates on the `numbers` representation of any `Vec<i32>`. When
we call the `largest` function, the code actually runs on the specific values
that we pass in.

Functions aren't the only way to eliminate duplication. For example, our
`largest` function only works for vectors of `i32`. What if we wanted to find
the largest number in a list of floats? Or the largest value in some sort of
custom `struct` or `enum`? We can't solve those kinds of duplication with
regular functions.

To solve these kinds of problems, Rust provides a feature called *generics*. In
the same way that functions allow us to abstract over common code, generics
allow us to abstract over types. This ability gives us tremendous power to
write code that works in a large number of situations. First, we'll examine the
syntax of generics. Then, we'll talk about another feature that's used to
augment generics: traits. Finally, we'll discuss one of Rust's most unique uses
of generics: lifetimes.
