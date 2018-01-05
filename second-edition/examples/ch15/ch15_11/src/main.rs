use std::cell::RefCell;

fn main() {
    let s = RefCell::new(String::from("hello"));

    let _r1 = s.borrow_mut();
    let _r2 = s.borrow_mut();
}