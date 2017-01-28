# Enums

In this chapter we'll look at *enumerations*, also referred to as *enums*.
Enums allow you to define a type by enumerating its possible values. First
we'll define and use an enum to show how an enum can encode meaning along with
data. Then we'll explore a particularly useful enum, `Option`, which expresses
that a value can be either something or nothing. Next we'll look at how pattern
matching in the `match` statement makes it easy to run different code for
different values of an enum. Finally, we'll cover how the `if let` construct is
another convenient and concise idiom you have available to handle enums in your
code.

Enums are a feature in many languages, but their capabilities differ
per-language. Rust’s enums are most similar to "algebraic data types" in
functional languages like F#, OCaml, or Haskell.
