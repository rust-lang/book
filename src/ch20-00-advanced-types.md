# Advanced Type System Features

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
