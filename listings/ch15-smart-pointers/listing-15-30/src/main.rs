#![feature(arbitrary_self_types)]
// TODO remove before we land this

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox <T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Pointee;

impl Pointee {
    fn hello(self: &CustomSmartPointer<Self>) {
        println!("Hello!");
    }
}

fn main() {
    let point = MyBox::new(Point(1.2, -3.5, 1.4));
    point.describe();
}
