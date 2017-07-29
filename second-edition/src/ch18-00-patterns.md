# Patterns Match the Structure of Values

Patterns are a special syntax within Rust for matching against the structure of
our types, complex or simple. A pattern is made up of some combination of
literals; destructured arrays, enums, structs, or tuples; variables, wildcards,
and placeholders. These pieces describe the “shape” of the data we’re working
with.

We use a pattern by taking some value and comparing it against the pattern. If
the pattern matches our value, we do something with the value parts. Recall in
Chapter 6 when we discussed the `match` expression that uses patterns like a
coin sorting machine. We can name pieces within the shape, like we named the
state that appeared on quarters in Chapter 6, and if the data fits the shape,
we can use the named pieces.

This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between *refutable* and
*irrefutable* patterns, and the different kinds of pattern syntax that you
might see.
