# Recoverable errors with `Result<T, E>`

The vast majority of errors in Rust are able to be recovered from. For this
case, Rust has a special type, `Result<T, E>`, to signal that a function might
succeed or fail.

Let's take a look at our example function from the last section:

```rust
fn check_guess(number: u32) -> bool {
    if number > 100 {
        panic!("Guess was too big: {}", number);
    }

    number == 34
}
```

We don't want the entire program to end if our `number` was incorrect. That's a
bit harsh! Instead, we want to signal that an error occurred. Here's a version
of `check_guess` that uses `Result<T, E>`:

```rust
fn check_guess(number: u32) -> Result<bool, &'static str> {
    if number > 100 {
        return Err("Number was out of range");
    }

    Ok(number == 34)
}
```

There are three big changes here: to the return type, to the error case, and to
the non-error case. Let's look at each in turn.

```rust
fn check_guess(number: u32) -> Result<bool, &'static str> {
#     if number > 100 {
#         return Err("Number was out of range");
#     }
# 
#     Ok(number == 34)
# }
```

Originally, we returned a `bool`, but now we return a
`Result<bool, &'static str>`. This is a type [provided by the standard library]
specifically for indicating that a function might have an error. More
specifically, it's an [`enum`] that looks like this:

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

[provided by the standard library]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
[`enum]`: ch06-01-enums.html

`Result<T, E>` is generic over two types: `T`, which is the successful case, and
`E`, which is the error case. It has two variants, `Ok` and `Err`, which also
correspond to these cases, respectively. So the type `Result<bool, &'static
str>` means that in the successful, `Ok` case, we will be returning a `bool`.
But in the failure, `Err` case, we will be returning a string literal.

```rust
# fn check_guess(number: u32) -> Result<bool, &'static str> {
#     if number > 100 {
        return Err("Number was out of range");
#     }
# 
#     Ok(number == 34)
# }
```

The second change we need to make is to our error case. Instead of causing a
`panic!`, we now `return` an `Err`, with a string literal inside. Remember,
`Result<T, E>` is an enum: `Err` is one of its variants.

```rust
# fn check_guess(number: u32) -> Result<bool, &'static str> {
#     if number > 100 {
#         return Err("Number was out of range");
#     }
# 
     Ok(number == 34)
# }
```

We also need to handle the successful case as well, and we make a change
similarly to the error case: we wrap our return value in `Ok`, which gives it
the correct type.

## Handling an error

Let's examine how to use this function:

```rust
fn check_guess(number: u32) -> Result<bool, &'static str> {
    if number > 100 {
        return Err("Number was out of range");
    }

    Ok(number == 34)
}

fn main() {
    let answer = check_guess(5);

    match answer {
        Ok(b) => println!("answer: {}", b),
        Err(e) => println!("There was an error: {}, e"),
    };
}
```
