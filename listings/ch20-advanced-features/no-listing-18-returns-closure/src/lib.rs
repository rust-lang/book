fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
