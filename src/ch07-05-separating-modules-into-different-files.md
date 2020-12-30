## Separating Modules into Different Files

So far, all the examples in this chapter defined multiple modules in one file.
When modules get large, you might want to move their definitions to a separate
file to make the code easier to navigate.

For example, let’s start from the code in Listing 7-17 and move the
`front_of_house` module to its own file *src/front_of_house.rs* by changing the
crate root file so it contains the code shown in Listing 7-21. In this case,
the crate root file is *src/lib.rs*, but this procedure also works with binary
crates whose crate root file is *src/main.rs*.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listing 7-21: Declaring the `front_of_house` module whose
body will be in *src/front_of_house.rs*</span>

And *src/front_of_house.rs* gets the definitions from the body of the
`front_of_house` module, as shown in Listing 7-22.

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Listing 7-22: Definitions inside the `front_of_house`
module in *src/front_of_house.rs*</span>

Using a semicolon after `mod front_of_house` rather than using a block tells
Rust to load the contents of the module from another file with the same name as
the module. To continue with our example and extract the `hosting` module to
its own file as well, we change *src/front_of_house.rs* to contain only the
declaration of the `hosting` module:

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

Then we create a *src/front_of_house* directory and a file
*src/front_of_house/hosting.rs* to contain the definitions made in the
`hosting` module:

<span class="filename">Filename: src/front_of_house/hosting.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

The module tree remains the same, and the function calls in `eat_at_restaurant`
will work without any modification, even though the definitions live in
different files. This technique lets you move modules to new files as they grow
in size.

Note that the `pub use crate::front_of_house::hosting` statement in
*src/lib.rs* also hasn’t changed, nor does `use` have any impact on what files
are compiled as part of the crate. The `mod` keyword declares modules, and Rust
looks in a file with the same name as the module for the code that goes into
that module.

## Summary

Rust lets you split a package into multiple crates and a crate into modules
so you can refer to items defined in one module from another module. You can do
this by specifying absolute or relative paths. These paths can be brought into
scope with a `use` statement so you can use a shorter path for multiple uses of
the item in that scope. Module code is private by default, but you can make
definitions public by adding the `pub` keyword.

In the next chapter, we’ll look at some collection data structures in the
standard library that you can use in your neatly organized code.
