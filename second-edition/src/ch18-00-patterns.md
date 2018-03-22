# Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with `match`
expressions and other constructs gives you more control over the control flow
of a program. A pattern is made up of some combination of:

- literals
- destructured arrays, enums, structs, or tuples
- variables
- wildcards
- placeholders

These pieces describe the shape of the data we’re working with, which we then
match against values to determine whether our program has the correct data to
continue running a particular bit of code.

To use a pattern we compare it to some value. If the pattern matches our value,
we use the value parts in our code. Recall our `match` expressions from Chapter
6 that used patterns like a coin sorting machine. If the value fits the shape
of the pattern, we can use the named pieces. If it doesn’t, the code associated
with the pattern won’t run.

This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between *refutable* and
*irrefutable* patterns, and the different kinds of pattern syntax that you
might see. By the end, you’ll see how to use patterns to create powerful and
clear code.
