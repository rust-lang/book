## Defining and Instantiating Structs

Structs are similar to tuples, which were discussed in Chapter 3. Like tuples,
the pieces of a struct can be different types. Unlike tuples, we name each
piece of data so it’s clear what the values mean. As a result of these names,
structs are more flexible than tuples: we don’t have to rely on the order of
the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A
struct’s name should describe the significance of the pieces of data being
grouped together. Then, inside curly braces, we define the names and types of
the pieces of data, which we call *fields*. For example, Listing 5-1 shows a
struct to store information about a user account:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

<span class="caption">Listing 5-1: A `User` struct definition</span>

To use a struct after we’ve defined it, we create an *instance* of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct, and then add curly braces containing `key:
value` pairs where the keys are the names of the fields and the values are the
data we want to store in those fields. We don’t have to specify the fields in
the same order in which we declared them in the struct. In other words, the
struct definition is like a general template for the type, and instances fill
in that template with particular data to create values of the type. For
example, we can declare a particular user like this:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we can use `user1.email` wherever we want to
use this value.

> ### Ownership of Struct Data
>
> In the `User` struct definition in Listing 5-1, we used the owned `String`
> type rather than the `&str` string slice type. This is a deliberate choice
> because we want instances of this struct to own all of its data and for that
> data to be valid for as long as the entire struct is valid.
>
> It’s possible for structs to store references to data owned by something else,
> but to do so requires the use of *lifetimes*, a Rust feature that is discussed
> in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid
> for as long as the struct is. Let’s say you try to store a reference in a
> struct without specifying lifetimes, like this:
>
> <span class="filename">Filename: src/main.rs</span>
>
> ```rust,ignore
> struct User {
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
>     active: bool,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> The compiler will complain that it needs lifetime specifiers:
>
> ```text
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 2 |     username: &str,
>   |               ^ expected lifetime parameter
>
> error[E0106]: missing lifetime specifier
>  -->
>   |
> 3 |     email: &str,
>   |            ^ expected lifetime parameter
> ```
>
> We’ll discuss how to fix these errors so you can store references in structs
> in Chapter 10, but for now, we’ll fix errors like these using owned types like
> `String` instead of references like `&str`.
