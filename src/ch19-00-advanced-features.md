# Advanced Features

We've come a long way! By now, we've learned 99% of the things you'll need to
know when writing Rust. We'll wrap the book up by doing one more project, but
before we get to that, let's talk about a few things that you may run into that
last 1% of the time. Feel free to skip this chapter and come back to it once
you run into these things in the wild; the tools we'll learn to use here are
useful in very specific situations. We don't want to leave them out, but you
won't find yourself reaching for them often.

Here's a quick summary:

* Unsafe Rust: for when you need to tell Rust "just trust me, promise!"
* Advanced Lifetimes: Additional lifetime syntax for complex situations.
* Advanced Traits: Associated Types, coherence, and disambiguation.

## Unsafe Rust

Things you may do in an unsafe block that you may not in safe rust

- deref a raw pointer
- call an unsafe fn
- access or modify a static variable
- impl an unsafe trait

Go see other stuff

Here's the syntax tho

You know unsafe blocks are the cause of any crashes

wrap all the unsafe, make it as small as possible, present a safe public API

### Raw Pointers

### Unsafe Functions

#### `transmute`

never ever. don't. stop.

#### `extern fn`

You have to write unsafe code to FFI

### `static`

### Unsafe Traits

## More Lifetimes

### Lifetimes that depend on other lifetimes

'a: 'b stuff: subtyping

### Higher ranked trait bounds

for<'a>

Needed for closures

## Associated Types

More common than the other things, less common than the rest of the book

why this is a thing instead of a generic

## The Thing Formerly Known as UFCS

Only needed when implementing super generic code

Lots of things are syntax sugar for this

Two traits that impl the same method - how to disambiguate

## Coherence

Show examples of when you control traits and types or not

Ex: Cannot impl Debug on someone else's type

Solution: newtype
