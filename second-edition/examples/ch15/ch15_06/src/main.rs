enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let _b = Cons(3, Box::new(a));
    let _c = Cons(4, Box::new(a));
}