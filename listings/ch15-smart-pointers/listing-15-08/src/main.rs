// ANCHOR: here
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// ANCHOR_END: here

fn main() {}
