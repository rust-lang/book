## Const Generics

Since rustc (rust compiler) version 1.51, which was released on March 25th, 2021, the core of Const Generics has become a stable part of the Rust Language.
Const generics are an incredibly useful feature in Rust and by taking advantage of the increased power const generics gives us we can write much simpler and clearer code.
For example the Rust standard library uses const generics to improve the ergonomics of arrays and their diagnostics.


### What are Const Generics?

Const generics are simply generic arguments that range over constant values, rather than types or lifetimes.
For instance, types can be parameterized by integers or booleans.
In fact, you have already met one example of a type utilizing const generics: the array type `[T; N]`,  are generic over some type `T` and some value `N` of type `usize`.
So how can we use const generic with our own types?

Outside the array types you have already met which can be consider are special case in regards to their syntax, const generics have an already familiar syntax.
Generic constants are placed in the same angled brackets as other generics but with the keyword `const` preceding their identifier then a colon followed by its type e.g `const FOO : bool`.
Also the generic parameters must currently come in a specific order: lifetimes, types, constants.
Below is several syntactic examples of const generics.

<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-34/const_gen_syntax/src/main.rs:here}}
```
<span class="caption">Listing 19-34: Possible const generics syntax</span>

To use functions, traits and types with a specific constant value we replace the generic with a constant of the correct type in the appropriate spot in its angled brackets:

<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-35/const_gen_syntax/src/main.rs:here}}
```
<span class="caption">Listing 19-35: Using const generics</span>


As you can also see above when implementing the `Bar` trait we can specify some constant values to replace some generics while remaining generic over other arguments. This is just like what we can do with type and lifetimes generics.


If the constant values given result in inconsistent types then a compiler error is given. For instance if we change the `foo` function call to have the generic vales of `<3,true>` we get the following error:


```console
{{#include ../listings/ch19-advanced-features/output-only-02/const_gen_syntax/output.txt}}
```

### Current restrictions

Currently const generics have been deliberately constrained: this version is the MVP (minimal viable product) for const generics.
The decision to stabilize just part of const generics is because the totality of const generics is very complicated and a large amount of work still remains however this restricted version const generics, that has been stabilized, is still a powerful tool for programmers to use.
As such it was decided to stabilize this limited version of const generics to give programmers access to these tools now while the work continues behind the scenes on the more complicated parts of const generics.
As such we will hopefully see these restrictions removed in future versions of Rust.
So what are these restrictions:

- Only integral types are permitted for const generics

    For now, the only types that may be used as the type of a const generic argument are the following types:

    * Integers (i.e. signed and unsigned integers, including isize and usize)
    *  Char
    *  Bool

    In the future, this restriction will be lifted to allow more complex types, such as &str and user-defined types.

- No complex generic expressions in const arguments

    Currently, const parameters may only be instantiated by const arguments of the following forms:

    * A standalone const parameter.
    * A literal (i.e. an integer, bool, or character).
    * A concrete constant expression (enclosed by {}), involving no generic parameters.

  For example:

<span class="filename">Filename: src/main.rs</span>

```rust, ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-36/const_gen_use/src/main.rs:here}}
```
<span class="caption">Listing 19-36: Possible initialization options of const generics</span>

For an explanation of why these restriction exist check out this [blog post](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html) by the Rust team.

### Arrays

Arrays are the major use case when it comes to const generics. Without const generics an array would need to specify its length when it is defined. For example consider:

```rust
struct Foo {
    arr: [i32; 8]
}
```

This produces a struct `Foo` containing an array of `i32`s with a length `8`. This length is fixed and must be defined when we define `Foo`.
This limits users of the `Foo` struct as every user of `Foo` must have an array of length `8` which can be an issue in several use cases.

Const Generics resolves this issue by allowing us to make our data types, traits and functions generic over the length of the array.

```rust
struct Foo<const N: usize> {
    arr: [i32; N]
}

fn bar<const N: usize>(inp: Foo<N>) -> [i32; N] {
    inp.arr
}
```

### Compile Time Inputs

The other major use case for const generics is as a compile time argument. As const generics must be defined at compile time, programmers and the compiler can use them to make various designs possible.

For example Rust's SIMD intrinsic use const generics to ensure that the arguments that need to be given at compile time are.

Another example is in which you're programming code that will have data and code running of various different processing units.
Such a case is in machine learning systems like Tensorflow which may have data on GPUs or CPUs.
When manipulating multiple pieces of data in these system, a common requirement is that all the data must be on the same hardware as the processor processing them.
Without Const Generics this requirement is either checked at runtime or determined at compile time by taking advantage of standard generic types, an approach which does not scale effectively.
Const generics however allow us to create a solution which is checked at compile time and is scalable.

Lets implement a very simplified code for this scenario:

<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-37/const_gen_use/src/main.rs}}
```
<span class="caption">Listing 19-37: A simplified solution to ensure data is on the same device</span>

> Note: If we try and create the `NumberWrapper<T>` without a field containing some form of type `T` then the compiler throws an error.
> We can solve this by using the standard library's [Phantom Data](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) type.
> `PhantomData<T>` is a marker type which contains a type `T`. This means it is a Zero Sized Type (ZST) so will take up no space in the compiled binary but will tell the compiler our type contains a field containing a type of some form of `T`.
> For more information on this topic see the standard library [documentation](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) or the section on `PhantomData` in the [nomicon](https://doc.rust-lang.org/nomicon/phantom-data.html).

This code ensures at compile time all the data that we are manipulating is on the same device. Therefore the programmer won't have to wait until they try running the code to discover the error.

For instance if we were to instead use a `CPUId<1>` or `GPUId<0>` for `y` we get a compiler error:

```console
{{#include ../listings/ch19-advanced-features/listing-19-37/const_gen_use/output_error_cpu.txt}}
```
<span class="caption">compiler error given when `y` is on `CPUId<1>`</span>


```console
{{#include ../listings/ch19-advanced-features/listing-19-37/const_gen_use/output_error_gpu.txt}}
```
<span class="caption">compiler error given when `y` is on `GPUId<0>`</span>


### Associated Constants

Const Generics can also be used when setting the values of associated constants

<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-38/const_gen_const/src/main.rs}}
```
<span class="caption">Listing 19-38: Using const generics to set an associated constant</span>

### Tradeoffs

So what is the downside of const generics? So far they sound great!
They do however come with a tradeoff which is the same as the generic types you have already been introduced to and that is that generics if heavily used can lead to a larger binary size and possibly longer build times.

To understand how this happens consider the following code:

<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-39/const_gen_use/src/main.rs}}
```
<span class="caption">Listing 19-39: A simple case with const generics</span>

The compiler will initially monomorphize the `foo` function to create the two distinct functions`foo::<true>` and `foo::<false>` in the compiler's representation of the program.
From this point the compiler performs all the necessary checks and compiles the two functions into machine code.

Basically the compiler must compile down a version of the appropriate code into machine code for each unique set of const generics.

The compiler *may* be able to optimize most of these functions away meaning that the resultant binary remains roughly the same size of the binary generated without const generics.
But if the compiler does not or cannot optimize, the binary will contain a version of the code for ach unique set of const generics.

So in the case of listing 19-39 both the functions `foo::<true>` and `foo::<false>` may both be present in the final binary but given the code in this example it is likely that it is optimized down to just the `println`.

Regardless of optimization the compiler build times will very likely grow as the compiler now effectively has to check, optimize and build multiple versions of the code instead of just one.

So when deciding how to use const generics keep in mind whether or not the compile time or binary size matters to those using your code.
Also be aware that if users of your code often use a large number of different combinations of const generic arguments and they're concerned about binary size then not using const generics or at least supplying an option without const generics may be the preferred option.

Now while these disadvantages are worth keeping in mind when you are using const generics, the advantages of using const generics often are much greater and outside of cases where you're limited in regards to binary size these downsides will likely be unnoticeable.



We have now covered the core functionality of Const Generics.
So now lets consider actually using const generics.
And to do so we will examine the typestate design pattern when combined with const generics.

### Typestate Pattern

The typestate pattern is a design pattern which has grown out of various functional programming languages.
The core idea in the design pattern is to encode the possible states of the program into the type system itself.

For example consider modelling a traffic light which has three lights (red, amber, green) and can be in four different unique states (red, red and amber, amber, green).
The traffic light will transition from red to red and amber to green to amber then back to red.
To ensure these transitions are respected we can create distinct types for each state and then carefully defining a transition function from each state to the next state (in more complex systems with multiple possible states from a state you could have multiple transition functions for each of these transitions).
Let's see this implemented in code without const generics:


<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-40/const_gen_use/src/main.rs}}
```
<span class="caption">Listing 19-40: Typestate pattern implemented without Const Generics</span>


This code ensures that at compile time the states and their transitions are satisfied.
It does this by ensuring each state is a different type.

However this code is already beginning to become unwieldy as in order for this to work we need to create a new struct for each possible state.
Imagine now if instead the system we are modeling was a road junction controlled with traffic lights and has multiple entrances and exits.
We would need to define likely dozens of types to describe each state but in doing so the code itself would likely become more opaque and unclear.
Now while various tricks with traits and macros can help to reduce these problems, const generics can make this pattern much cleaner.


<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-41/const_gen_use/src/main.rs}}
```
<span class="caption">Listing 19-41: Typestate pattern implemented with Const Generics</span>


While this particular code may not be much shorter it is arguably clearer. Also as the model we wish to encode grows more complex, this version may scale much more effectively. The pattern will also become much clearer, simpler and scalable as const generics with user-defined enums and struct is stabilized.


Another great example of the typestate pattern and how const generics can make it even better is the builder pattern that you have previously met.
Recall the builder pattern, where you create a type `BazBuilder` that collects all values necessary for creating the type `Baz`.

Now lets say your building a struct `Foo` with three fields: `a:u32`,`b:i64`,`c:Option<u8>`. Const generics can be used in `FooBuilder` so that at compile time we can ensure the builder has values for fields `a` and `b` but may not have one for `c`.

<span class="filename">Filename: src/main.rs</span>
```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-42/const_gen_use/src/main.rs}}
```
<span class="caption">Listing 19-42: Builder pattern using Const Generics </span>


This code tells the compiler to only call the `build` function if const generic arguments `ASET` and `BSET` are set to`true`. Therefore the code compiles happily as it is but creates an error at compile time if `foo3` is uncommented.
Also while this may seem extensive to write by hand, a custom derive macro is well suited to the task.


Rust's already rich type system enables us to effectively utilise the typestate pattern.
Const generics however can offer an improvement to the clarity and ease in the use of the typestate pattern in Rust.
It also stands to improve further as support for user-defined enums and structs reaches stable rust.

## Summary

Whew! Now you have some Rust features in your toolbox that you won’t use often,
but you’ll know they’re available in very particular circumstances. We’ve
introduced several complex topics so that when you encounter them in error
message suggestions or in other people’s code, you’ll be able to recognize
these concepts and syntax. Use this chapter as a reference to guide you to
solutions.

Next, we’ll put everything we’ve discussed throughout the book into practice
and do one more project!
