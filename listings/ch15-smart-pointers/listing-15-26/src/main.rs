use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Since this list is now circular,
    // we'll see the same value if we call `.tail()` 2 times
    println!(
        "a next next item's value = {:?} should equal {}",
        match **a.tail().unwrap().borrow().tail().unwrap().borrow() {
            Cons(i, _) => i,
            Nil => {
                panic!()
            }
        },
        match *a {
            Cons(i, _) => i,
            Nil => {
                panic!()
            }
        }
    );

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack while formatting the string
    // because the node will recursively have next items
    // RefCell { value: Cons(5, RefCell { value: Cons(10, ... ) }) }
    // println!("a next item = {:?}", a.tail());
}
// ANCHOR_END: here
