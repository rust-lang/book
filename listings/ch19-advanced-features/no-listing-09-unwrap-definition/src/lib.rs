enum Option<T> {
    Some(T),
    None,
}

use crate::Option::*;

// ANCHOR: here
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
// ANCHOR_END: here
