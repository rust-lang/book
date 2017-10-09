# 제네릭 타입, 트레잇, 그리고 라이프타임

Every programming language has tools to deal effectively with duplication of
concepts; in Rust, one of those tools is *generics*. Generics are abstract
stand-ins for concrete types or other properties. When we're writing and
compiling the code we can express properties of generics, such as their
behavior or how they relate to other generics, without needing to know what
will actually be in their place.
모든 프로그래밍 언어는 컨셉의 복제를 효율적으로 다루기 위한 도구를 가지고 있습니다; 러스트에서,
그러한 도구중 하나가 바로 *제네릭*입니다. 제네릭은 구체화된 타입이나 다른 속성들을 위한 추상화된
대역입니다. 코드를 작성하고 컴파일할 때, 우리는 제네릭들이 실제로 어떻게 완성되는지 알 필요없이,
제네릭의 동작 혹은 다른 제네릭과 어떻게 연관되는지와 같은 제네릭에 대한 속성을 표현할 수 있습니다.

In the same way that a function takes parameters whose value we don't know in
order to write code once that will be run on multiple concrete values, we can
write functions that take parameters of some generic type instead of a concrete
type like `i32` or `String`. We've already used generics in Chapter 6 with
`Option<T>`, Chapter 8 with `Vec<T>` and `HashMap<K, V>`, and Chapter 9 with
`Result<T, E>`. In this chapter, we'll explore how to define our own types,
functions, and methods with generics!
여러개의 구체화된 값들에 대해 실행될 코드를 작성하기 위하여 함수가 어떤 값을 담을지 알수 없는 파라미터를
가지는 것과 같은 방식으로, `i32`나 `String`과 같은 구체화된 타입 대신 몇몇 제네릭 타입의 파라미터를
갖는 함수를 작성할 수 있습니다. 우리는 6장의 `Option<T>`, 8장의 `Vec<T>`와 `HashMap<K, V>`,
그리고 9장의 `Result<T, E>`에서 이미 제네릭을 사용해 보았습니다. 이 장에서는, 어떤 식으로
우리만의 타입, 함수, 그리고 메소드를 제네릭으로 정의하는지 탐험해 볼 것입니다!

First, we're going to review the mechanics of extracting a function that
reduces code duplication. Then we'll use the same mechanics to make a generic
function out of two functions that only differ in the types of their
parameters. We'll go over using generic types in struct and enum definitions
too.
우선, 우리는 코드 중복을 제거하는 함수의 추출하는 원리에 대해 돌아볼 것입니다. 그리고나서 두 함수가
오직 파라미터의 타입만 다른 경우에 대하여 이들을 하나의 제네릭 함수로 만들기 위해 동일한 원리을 사용할
것입니다. 또한 제네릭 타입을 구조체와 열거형의 정의에 사용하는 것을 살펴볼 것입니다.

After that, we'll discuss *traits*, which are a way to define behavior in a
generic way. Traits can be combined with generic types in order to constrain a
generic type to those types that have a particular behavior, rather than any
type at all.
그리고 난 후 *트레잇(trait)* 에 대하여 논의할 것인데, 이는 동작을 제네릭한 방식으로 정의하는
방법을 말합니다. 트레잇은 

Finally, we'll discuss *lifetimes*, which are a kind of generic that let us
give the compiler information about how references are related to each other.
Lifetimes are the feature in Rust that allow us to borrow values in many
situations and still have the compiler check that references will be valid.

## Removing Duplication by Extracting a Function

Before getting into generics syntax, let's first review a technique for dealing
with duplication that doesn't use generic types: extracting a function. Once
that's fresh in our minds, we'll use the same mechanics with generics to
extract a generic function! In the same way that you recognize duplicated code
to extract into a function, you'll start to recognize duplicated code that can
use generics.

Consider a small program that finds the largest number in a list, shown in
Listing 10-1:

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
#  assert_eq!(largest, 100);
}
```

<span class="caption">Listing 10-1: Code to find the largest number in a list
of numbers</span>

This code takes a list of integers, stored here in the variable `numbers`. It
puts the first item in the list in a variable named `largest`. Then it iterates
through all the numbers in the list, and if the current value is greater than
the number stored in `largest`, it replaces the value in `largest`. If the
current value is smaller than the largest value seen so far, `largest` is not
changed. When all the items in the list have been considered, `largest` will
hold the largest value, which in this case is 100.

If we needed to find the largest number in two different lists of numbers, we
could duplicate the code in Listing 10-1 and have the same logic exist in two
places in the program, as in Listing 10-2:

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

<span class="caption">Listing 10-2: Code to find the largest number in *two*
lists of numbers</span>

While this code works, duplicating code is tedious and error-prone, and means
we have multiple places to update the logic if we need to change it.

<!-- Are we safe assuming the reader will be familiar with the term
"abstraction" in this context, or do we want to give a brief definition? -->
<!-- Yes, our audience will be familiar with this term. /Carol -->

To eliminate this duplication, we can create an abstraction, which in this case
will be in the form of a function that operates on any list of integers given
to the function in a parameter. This will increase the clarity of our code and
let us communicate and reason about the concept of finding the largest number
in a list independently of the specific places this concept is used.

In the program in Listing 10-3, we've extracted the code that finds the largest
number into a function named `largest`. This program can find the largest
number in two different lists of numbers, but the code from Listing 10-1 only
exists in one spot:

<span class="filename">Filename: src/main.rs</span>

```rust
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
#    assert_eq!(result, 100);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("The largest number is {}", result);
#    assert_eq!(result, 6000);
}
```

<span class="caption">Listing 10-3: Abstracted code to find the largest number
in two lists</span>

The function has a parameter, `list`, which represents any concrete slice of
`i32` values that we might pass into the function. The code in the function
definition operates on the `list` representation of any `&[i32]`. When we call
the `largest` function, the code actually runs on the specific values that we
pass in.

The mechanics we went through to get from Listing 10-2 to Listing 10-3 were
these steps:

1. We noticed there was duplicate code.
2. We extracted the duplicate code into the body of the function, and specified
   the inputs and return values of that code in the function signature.
3. We replaced the two concrete places that had the duplicated code to call the
   function instead.

We can use these same steps with generics to reduce code duplication in
different ways in different scenarios. In the same way that the function body
is now operating on an abstract `list` instead of concrete values, code using
generics will operate on abstract types. The concepts powering generics are the
same concepts you already know that power functions, just applied in different
ways.

What if we had two functions, one that found the largest item in a slice of
`i32` values and one that found the largest item in a slice of `char` values?
How would we get rid of that duplication? Let's find out!
