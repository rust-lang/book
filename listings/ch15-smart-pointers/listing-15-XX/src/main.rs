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

struct Point(i32, i32, i32);

impl Point {
    fn describe(self: &MyBox<Self>) {
        println!("x: {} | y: {} | z : {}", self.0.0, self.0.1, self.0.2);
    }
}

fn main() {
    let point = MyBox::new(Point(1, -3, 1));
    point.describe();
}
