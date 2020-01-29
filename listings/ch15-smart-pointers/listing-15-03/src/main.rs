enum List {
    Cons(i32, List),
    Nil,
}

// ANCHOR: here
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
// ANCHOR_END: here
