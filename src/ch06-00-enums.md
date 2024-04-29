# Enums and Pattern Matching

In this chapter, we’ll look at *enumerations*, also referred to as *enums*.
Enums allow you to define a type by enumerating its possible *variants*. First
we’ll define and use an enum to show how an enum can encode meaning along with
data. Next, we’ll explore a particularly useful enum, called `Option`, which
expresses that a value can be either something or nothing. Then we’ll look at
how pattern matching in the `match` expression makes it easy to run different
code for different values of an enum. Finally, we’ll cover how the `if let`
construct is another convenient and concise idiom available to handle enums in
your code.
