# Functional Language features in Rust - Iterators and Closures

## Closures

### What is a closure

How are they diff from fns

### `Fn` traits

## Iterators

### Iterator & for loop

.into_iter()

### Iterators are Lazy

Difference between adapter and consumer - another iterator or consuming?

### Implementing the Iterator trait

Talk about using Associated Types here, foreshadow to advanced type systems
chapter about why this is a different thing than normal

## ??? How does this improve `greprs`

Does this get woven into the above sections?

## Summary: Performance

### Iterators compile down to ASM == for loop

Most complicated chain of iterator functions that compile down to the same ASM as a for loop

### Representation: Closures are a Struct

Closures don't have any further perf penalty over regular fn calls

