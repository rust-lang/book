# Essential Collections

Rust's standard library includes a number of really useful data structures
called *collections*. Most other types represent one specific value, but
collections can contain multiple values inside of them. Each collection has
different capabilities and costs, and choosing an appropriate one for the
situation you're in is a skill you'll develop over time. In this chapter, we'll
go over three collections which are used very often in Rust programs:

* A *Vector* allows you to store a variable number of values next to each other.
* A *String* is a collection of characters. We've seen `String` before, but
  we'll talk about it in depth now.
* A *HashMap* allows you to associate a value with a particular key.

There are more specialized variants of each of these data structures for
particular situations, but these are the most fundamental and common.
