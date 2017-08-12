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
example, we can declare a particular user as shown in Listing 5-2:

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

<span class="caption">Listing 5-2: Creating an instance of the `User`
struct</span>

To get a specific value from a struct, we can use dot notation. If we wanted
just this user’s email address, we can use `user1.email` wherever we want to
use this value. To change a value in a struct, if the instance is mutable, we
can use the dot notation and assign into a particular field. Listing 5-3 shows
how to change the value in the `email` field of a mutable `User` instance:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

<span class="caption">Listing 5-3: Changing the value in the `email` field of a
`User` instance</span>

Like any expression, we can implicitly return a new instance of a struct from a
function by constructing the new instance as the last expression in the
function body. Listing 5-4 shows a `build_user` function that returns a `User`
instance with the given `email` and `username`. The `active` field gets the
value of `true`, and the `sign_in_count` gets a value of `1`.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Listing 5-4: A `build_user` function that takes an email
and username and returns a `User` instance</span>

Repeating the `email` field name and `email` variable, and the same for
`username`, is a bit tedious, though. It makes sense to name the function
arguments with the same name as the struct fields, but if the struct had more
fields, repeating each name would get even more annoying. Luckily, there’s a
convenient shorthand!

### Field Init Shorthand when Variables Have the Same Name as Fields

If you have variables with the same names as struct fields, you can use *field
init shorthand*. This can make functions that create new instances of structs
more concise.

In Listing 5-4, the parameter names `email` and `username` are the same as the
`User` struct’s field names `email` and `username`. Because the names are
exactly the same, we can write `build_user` without the repetition of `email`
and `username` as shown in Listing 5-5. This version of `build_user` behaves
the same way as the one in Listing 5-4. The field init syntax can make cases
like this shorter to write, especially when structs have many fields.

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

<span class="caption">Listing 5-5: A `build_user` function that uses field init
syntax since the `email` and `username` parameters have the same name as struct
fields</span>

### Creating Instances From Other Instances With Struct Update Syntax

It’s often useful to create a new instance from an old instance, using most of
the old instance’s values but changing some. Listing 5-6 shows an example of
creating a new `User` instance in `user2` by setting the values of `email` and
`username` but using the same values for the rest of the fields from the
`user1` instance we created in Listing 5-2:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

<span class="caption">Listing 5-6: Creating a new `User` instance, `user2`, and
setting some fields to the values of the same fields from `user1`</span>

The *struct update syntax* achieves the same effect as the code in Listing 5-6
using less code. The struct update syntax uses `..` to specify that the
remaining fields not set explicitly should have the same value as the fields in
the given instance. The code in Listing 5-7 also creates an instance in `user2`
that has a different value for `email` and `username` but has the same values
for the `active` and `sign_in_count` fields that `user1` has:

```rust
# struct User {
#     username: String,
#     email: String,
#     sign_in_count: u64,
#     active: bool,
# }
#
# let user1 = User {
#     email: String::from("someone@example.com"),
#     username: String::from("someusername123"),
#     active: true,
#     sign_in_count: 1,
# };
#
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

<span class="caption">Listing 5-7: Using struct update syntax to set a new
`email` and `username` values for a `User` instance but use the rest of the
values from the fields of the instance in the `user1` variable</span>

### Tuple Structs without Named Fields to Create Different Types

We can also define structs that look similar to tuples, called *tuple structs*,
that have the added meaning the struct name provides, but don’t have names
associated with their fields, just the types of the fields. The definition of a
tuple struct still starts with the `struct` keyword and the struct name, which
are followed by the types in the tuple. For example, here are definitions and
usages of tuple structs named `Color` and `Point`:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Note that the `black` and `origin` values are different types, since they’re
instances of different tuple structs. Each struct we define is its own type,
even though the fields within the struct have the same types. Otherwise, tuple
struct instances behave like tuples, which we covered in Chapter 3.

### Unit-Like Structs without Any Fields

We can also define structs that don’t have any fields! These are called
*unit-like structs* since they behave similarly to `()`, the unit type.
Unit-like structs can be useful in situations such as when you need to
implement a trait on some type, but you don’t have any data that you want to
store in the type itself. We’ll be discussing traits in Chapter 10.

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
