Please add this text at the end of the Function Pointers section, just before
the Returning Closures section starts, so at the end of page 447 and before
page 448.

---

Another useful pattern exploits an implementation detail of tuple structs and
tuple-struct enum variants. These items use `()` as initializer syntax, which
looks like a function call. The initializers are actually implemented as
functions returning an instance constructed from their arguments. These
initializer functions can also be used as a function pointer that implements
the closure traits, so they can also be specified as arguments for methods that
take closures:

```
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> =
    (0u32..20)
    .map(Status::Value)
    .collect();
```

This code creates `Status::Value` instances using each `u32` value in the range
that `map` is called on by using the initializer function of `Status::Value`.
Some people prefer this style, and some people prefer to use closures. They end
up compiling to the same code, so use whichever style is clearer to you.
