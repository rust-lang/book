# Smart Pointers

By smart pointers we mean a reference with more characteristics.

Example of something that doesn't work

Surprise! Vec and String are technically smart pointers too!

This chapter is not a comprehensive list, but will give some examples of the
ones in the standard library.


## `Box<T>`

Don't use very often in your own code
Heap allocated
Express Ownership of a heap allocated thing

The three situations to use Box

1. Trait objects
2. Recursive data structures
3. Extend the lifetime of something

How this interacts with the Drop trait

## `Rc<T>`

Reference counted. Rc is for *multiple ownership* - this thing should get
deallocated when all of the owners go out of scope.

Show the data structure:

```rust
struct Rc<T> {
    data: Box<T>,
    strong_reference_count: usize,
    weak_reference_count: usize,
}
```

Talk through this.

This only works if the data is immutable.

What happens when you clone an Rc: data isn't cloned, increase the strong count.
When an Rc clone goes out of scope, the count goes down.

### Rc Cycles

This is how you leak memory in rust, which btw is totally safe.

Is this garbage collecting? Well it's not tracing GC...  if you use Rc and had
a cycle detector, it would be functionally equivalent to a tracing GC. Different
runtime characteristics tho.


#### Solution: turn an Rc into a `Weak<T>`

Same as Rc, but doesn't count towards the strong ref count. When you do this, the
strong ref count goes down and the weak count goes up.

Data gets cleaned up when the strong count is 0, no matter what the weak count is.
However, Rc structure is kept until weak reference count also goes to zero, so weak pointers do not become dangling pointers.
At this point, attempt to upgrade Weak pointer will result into None.
Only when weak reference counter also reduces to zero, Rc structure is freed.

## `RefCell<T>`

Single owner of mutable data

The ownership rules checked at runtime instead of compile time.

Only single threaded. See next chapter.

### `borrow` and `borrow_mut` methods

Checks all the rules and panics at runtime if the code violates them.

1. The borrow checker is conservative and people can know more things. (no you
don't, but if you really want to go back to debugging segfaults, feel free)

2. For when you're only allowed to have an immutable thing (which could be `Rc`)
but you need to be able to mutate the underlying data.

## `Cell<T>`

Same thing as RefCell but for types that are Copy. No borrow checking rules here
anyway. So just reason #2 above.

## Is this really safe? Yes!

RefCell is still doing the checks, just at runtime
Cell is safe bc Copy types don't need the ownership rules anyway

### The Interior Mutability Pattern

The Interior Mutability Pattern is super unsafe internally but safe to use
from the outside and is totally safe, totally, trust us, seriously, it's safe.

Allude to `UnsafeCell<T>` maybe. Affects optimizations since &mut T is unique.
UnsafeCell turns off those optimizations so that everything doesn't break.

This is how you can opt-out of the default of Rust's ownership rules and opt
in to different guarantees.

## Summary

If you want to implement your own smart pointer, go read the Nomicon.

Now let's talk about concurrency, and some smart pointers that can be used
with multiple threads.
