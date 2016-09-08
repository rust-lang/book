# Generics

One of the core tools a programming language gives you is the ability to deal
effectively with duplication. Different kinds of duplication are dealt with in
different ways. Consider a small program that finds the largest number in a
list:

```rust
let numbers = vec![34, 50, 25, 100, 65];

let mut largest = numbers[0];

for number in numbers {
    if largest > number {
        largest = number;
    }
}

println!("The largest number is {}", largest);
```

If we needed to find the largest number twice, we could duplicate our code:

```rust
let numbers = vec![34, 50, 25, 100, 65];

let mut largest = numbers[0];

for number in numbers {
    if largest > number {
        largest = number;
    }
}

println!("The largest number is {}", largest);

let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

let mut largest = numbers[0];

for number in numbers {
    if largest > number {
        largest = number;
    }
}

println!("The largest number is {}", largest);
```

However, this is tedious and error-prone. Rust, like many languages, gives us a
way to deal with this duplication by creating an abstraction. In this case, the
answer is functions:

```rust
fn largest(numbers: Vec<i32>) {
    let mut largest = numbers[0];

    for number in numbers {
        if largest > number {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

let numbers = vec![34, 50, 25, 100, 65];

largest(numbers);

let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

largest(numbers);
```

But functions aren't the only way to abstract away different kinds of code. For
example, our `largest` function only works for vectors of `i32`. What if we wanted
to find the largest number in a list of floats? Or the largest element of some sort
of custom `struct` or `enum`? We can't solve this duplication with regular functions.

To solve these kinds of problems, Rust provides a feature called *generics*. In the
same way that functions allow us to abstract over common code, generics allow us to
abstract over types. This ability gives us tremendous power to write code that works
in a large number of situations. First, we'll examine the syntax of generics. Then,
we'll talk about another feature that's used to augment generics: traits. Finally,
we'll discuss one of Rust's most unique uses of generics: lifetimes.
