# Common Collections

Rust’s standard library includes a number of very useful data structures called
*collections*. Most other data types represent one specific value, but
collections can contain multiple values. Unlike the built-in array and tuple
types, the data these collections point to is stored on the heap, which means
the amount of data does not need to be known at compile time and can grow or
shrink as the program runs. Each kind of collection has different capabilities
and costs, and choosing an appropriate one for your current situation is a
skill you’ll develop over time. In this chapter, we’ll discuss three
collections that are used very often in Rust programs:

* A *vector* allows you to store a variable number of values next to each other.
* A *string* is a collection of characters. We’ve mentioned the `String` type
  previously, but in this chapter we’ll talk about it in depth.
* A *hash map* allows you to associate a value with a particular key. It’s a
  particular implementation of the more general data structure called a *map*.

To learn about the other kinds of collections provided by the standard library,
see [the documentation][collections].

[collections]: ../../std/collections/index.html

We’ll discuss how to create and update vectors, strings, and hash maps, as well
as what makes each special.
