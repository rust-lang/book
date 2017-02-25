# Unsafe Rust

Things you may do in an unsafe block that you may not in safe rust

- deref a raw pointer
- call an unsafe fn
- access or modify a static variable
- impl an unsafe trait

Go see other stuff

Here's the syntax tho

You know unsafe blocks are the cause of any crashes

wrap all the unsafe, make it as small as possible, present a safe public API

## Raw Pointers

## Unsafe Functions

### `transmute`

never ever. don't. stop.

### `extern fn`

You have to write unsafe code to FFI

## `static`

## Unsafe Traits
