## Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a *path*, in the
same way that, when navigating a file system, we use a path. If we want to call
a function, we need to know its path.

A *path* can take two forms:

* An *absolute path* starts from a crate root by using a crate name or a
  literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or
  an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).

Let’s return to our example in Listing 7-1. How do we call the
`add_to_waitlist` function? This is the same thing as asking, what’s the path
of the `add_to_waitlist` function? In Listing 7-3, we simplified our code a bit
by removing some of the modules and functions. We’ll show two ways to call the
`add_to_waitlist` function from a new function `eat_at_restaurant` defined in
the crate root. Note that this example won’t compile just yet, we’ll explain
why in a bit.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

<span class="caption">Listing 7-3: Calling the `add_to_waitlist` function using
absolute and relative paths</span>

The first time we call the `add_to_waitlist` function in `eat_at_restaurant`,
we use an absolute path. The `add_to_waitlist` function is defined in the same
crate as `eat_at_restaurant`, which means we can use the `crate` keyword to
start an absolute path.

After `crate`, we include each of the successive modules until we make our way
to `add_to_waitlist`. You could imagine a file system with the same structure,
and we’d specify the path `/front_of_house/hosting/add_to_waitlist` to run the
`add_to_waitlist` program; using the `crate` name to start from the crate root
is like using `/` to start from the filesystem root in your shell.

The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a
relative path. The path starts with `front_of_house`, the name of the module
defined at the same level of the module tree as `eat_at_restaurant`. Here the
filesystem equivalent would be using the path
`front_of_house/hosting/add_to_waitlist`. Starting with a name means that the
path is relative.

<!--- Can you outline briefly what effect the path being relative or absolute
has on the code? I'm not totally clear why or when you'd use either --->
<!-- Done /Carol -->

Choosing whether to use a relative or absolute paths is always a decision
you’ll have to make based on your specific project. It depends on whether
you’re more likely to move code that defines items together or separately from
the code that uses the items. For example, if we move the `front_of_house`
module and the `eat_at_restaurant` function together into a module named
`customer_experience`, we would need to update the absolute path to
`add_to_waitlist` but the relative path would still be valid. However, if we
moved the `eat_at_restaurant` function separately into a module named `dining`,
the absolute path to the `add_to_waitlist` call would stay the same but the
relative path would need to be updated. We tend to specify absolute paths as
we’ve found that it’s more likely to move code definitions and item calls
independently of each other.

We mentioned that Listing 7-3 won’t compile yet, let’s try to compile it and
find out why not! The error we get is shown in Listing 7-4.

```text
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^
```

<span class="caption">Listing 7-4: Compiler errors from building the code in
Listing 7-3</span>

The error messages say that module `hosting` is private. In other words, we
have the correct paths for the `hosting` module and the `add_to_waitlist`
function, but Rust won’t let us use them because it doesn’t have access to the
private sections.

<!-- Below: Can you define privacy boundary and say what it's useful for
briefly here? What does it mean for an item to be private? -->
<!-- Done /Carol -->

Modules are not only useful for organizing your code, they also define Rust’s
*privacy boundary*, the line that encapsulates implementation details that
external code isn’t allowed to know about, call, or rely on. Thus if you want
to make an item like a function or struct private, you put it in a module.

<!--- I didn't think we needed to have this as bullets, and likewise found the
up/down metaphor confusing. What do you think of this rewrite? --->
<!-- Pretty good, I took out the reference to "first" because there wasn't a
"second", etc anymore /Carol -->

The way privacy works in Rust is that all items (functions, methods, structs,
enums, modules, and constants) are private by default. Items in a parent module
can’t use the private items inside child modules, while items in child modules
can use the items in their ancestor modules. This is because child modules wrap
and hide their implementation details, but the child modules can see the
context in which they’re defined. To continue with the restaurant metaphor,
think of the privacy rules like the back office of a restaurant: what goes on
in there is private to customers of the restaurant, but managers in the office
can see and do everything in the restaurant in which they operate.

<!-- above: Is this line I added true? Can you quickly say why the child can
access the parent but not the other way round? -->
<!-- I've tried to explain and use the metaphor, how's this? /Carol -->

Rust chose to have the module system function this way so that hiding inner
implementation details is the default. That way, you know which parts of the
inner code you can change without the possibility of breaking outer code.
However, you can consciously choose to expose inner parts of child modules code
to outer ancestor modules by making an item public with the `pub` keyword.

### Exposing Paths with the `pub` Keyword

<!-- I'm not sure this is an accurate heading, can you fix it? I wanted to
expand a little to show its relation to the path topics that surround it-->
<!-- It's less about who on the outside gains access to it, and more about who
on the inside chooses to expose it. How's this heading, in light of that?
/Carol -->

With that understanding, let’s look back at the error in Listing 7-4, that told
us the `hosting` module is private. We want the `eat_at_restaurant` function in
the parent module to have access to the `add_to_waitlist` function in the child
module, so we mark the `hosting` module with the `pub` keyword, shown in
Listing 7-5.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

<span class="caption">Listing 7-5: Declaring the `hosting` module as `pub` so
that we’re allowed to use it from `eat_at_restaurant`</span>

Unfortunately, as you may have just found out, the code in Listing 7-5 still
results in an error (Listing 7-6):

```text
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^
```

<span class="caption">Listing 7-6: Compiler errors from building the code in
Listing 7-5</span>

What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we’re allowed to access `front_of_house`,
we can access `hosting`. But the *contents* of `hosting` are still private;
making the module public does not make its contents public. The `pub` keyword
on a module only lets code in its ancestor modules refer to it.

The errors in Listing 7-6 now say that the `add_to_waitlist` function is
private. The privacy rules apply to structs, enums, functions, and methods as
well as modules.

Let’s also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7:

<span class="filename">Filename: src/lib.rs</span>

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
# fn main() {}
```

<span class="caption">Listing 7-7: Adding the `pub` keyword to both `mod
hosting` and `fn add_to_waitlist` lets us call the function from
`eat_at_restaurant`</span>

Now it’ll compile! Let’s look at both the absolute and the relative path and
double check why adding the `pub` keyword lets us use these paths in
`add_to_waitlist` with respect to the privacy rules.

<!---
is a crate the same as a "module tree"? what does it mean to be the root of the
crate. --->
<!-- A crate has a module tree; a module tree is the structure of the crate.
I'm not sure how to clarify that here? I thought crate/root/module tree was
explained sufficiently earlier /Carol -->

In the absolute path case, we start with `crate`, the root of our crate’s
module tree. From here, we have the `front_of_house` module defined in the
crate root. The `front_of_house` module isn’t public, but because the
`eat_at_restaurant` function is defined in the same module that
`front_of_house` is defined (that is, `eat_at_restaurant` and `front_of_house`
are siblings), we’re allowed to refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we’re allowed to access `hosting`.
Finally, the `add_to_waitlist` function is marked with `pub` and we can access
its parent module, so this function call works!

In the relative path case, the logic is the same as the absolute path except
for the first step: rather than starting from the crate root, the path starts
from `front_of_house`. The `front_of_house` module is defined within the same
module as `eat_at_restaurant`, so the relative path starting from the module in
which `eat_at_restaurant` is defined works. Then because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works and this
function call is valid!

### Starting Relative Paths with `super`

You can also construct relative paths that begin in the parent module by using
`super` at the start of the path. This is like starting a filesystem path with
`..`. Why would we want to do this?

Consider the situation in Listing 7-8 that models the case where a chef fixes
an incorrect order and personally brings it out to the customer. The function
`fix_incorrect_order` calls the function `serve_order` by specifying the path
to `serve_order` starting with `super`:

<span class="filename">Filename: src/lib.rs</span>

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
# fn main() {}
```

<span class="caption">Listing 7-8: Calling a function using a relative path
starting with `super`</span>

The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `serve_order` and find it.
Success! We think the `back_of_house` module and the `serve_order` function are
likely to stay in the same relationship to each other and get moved together
should we decide to reorganize the crate’s module tree. Therefore, we used
`super` so that we’ll have fewer places to update code in the future, should
this code get moved to a different module.

<!-- Can you broaden this out and let them know the general reason we would use
super over the other methods here? -->
<!-- The reason we have here *is* the general reason, I'm not sure what about
the text implies that there's a broader reason we aren't mentioning? /Carol -->

### Making Structs and Enums Public

You can also use `pub` to designate structs and enums as public, but there are
a few extra details. If you use `pub` before a struct definition, you make the
struct public, but the struct’s fields will still be private. You can choose to
make each field public or not on a case-by-case basis. In Listing 7-9, we’ve
defined a public `back_of_house::Breakfast` struct with a public `toast` field
but a private `seasonal_fruit` field. This models the case in a restaurant
where the customer can pick the type of bread that comes with a meal, but the
chef decides what fruit will come with a meal based on what’s in season and
what they have in stock. The fruit that’s available changes quickly so
customers aren’t allowed to choose the fruit or even see which fruit they’ll
get.

<span class="filename">Filename: src/lib.rs</span>

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

<span class="caption">Listing 7-9: A struct with some public fields and some
private fields</span>

Because the `toast` field of the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice, though, that we’re not allowed to use the `seasonal_fruit`
field in `eat_at_restaurant` because `seasonal_fruit` is private. Try
uncommenting the line modifying the `seasonal_fruit` field value to see what
error you get!

Also note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast` didn’t
have such a function, we wouldn’t be able to create an instance of `Breakfast`
in `eat_at_restaurant` because we’re not allowed to set the value of the
private `seasonal_fruit` field in `eat_at_restaurant`.

In contrast, if you make an enum public, all of its variants are then public.
You only need the `pub` before the `enum` keyword, as shown in Listing 7-10.

<span class="filename">Filename: src/lib.rs</span>

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

<span class="caption">Listing 7-10: Designating an enum as public makes all its
variants public</span>

<!-- Is there a reason rust developers chose to have enum arms public but the
content of structs not, using pub? -->
<!-- Added! /Carol -->

Because we made the `Appetizer` enum public, we’re able to use the `Soup` and
`Salad` variants in `eat_at_restaurant`. Enums aren’t very useful unless their
variants are public; it would be annoying to have to annotate all enum variants
with `pub` in every case, so the default for enum variants is to be public.
Structs are often useful without their fields being public, so struct fields
follow the general rule of everything being private by default unless annotated
with `pub`.

There’s one more situation involving `pub` that we haven’t covered, and that
concerns our last module system feature: the `use` keyword. Let’s cover `use`
by itself first, and then we’ll show how `pub` and `use` can be combined.
