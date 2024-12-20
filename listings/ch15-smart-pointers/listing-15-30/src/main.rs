#![feature(arbitrary_self_types)]
// TODO remove before we land this

use std::ops::Deref;

struct CustomSmartPointer<T>(T);

impl<T> Deref for CustomSmartPointer<T> {
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
    let ptr = CustomSmartPointer(Pointee);
    ptr.hello();
}
